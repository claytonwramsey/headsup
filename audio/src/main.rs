#![warn(clippy::pedantic)]

use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};

use gpio_cdev::{EventRequestFlags, LineRequestFlags};

use audio::localization::{source_of_shot, Event, Point};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// The GPIO pin IDs which are associated with microphone input.
    const MIC_INPUT_PINS: [u32; 8] = [2, 3, 4, 9, 15, 18, 17, 27];
    const MIC_POINTS: [Point<3>; MIC_INPUT_PINS.len()] = [
        Point([-0.09, 00.095, 00.00]),  // M0 - front left
        Point([-0.14, 00.0015, 00.00]), // M1 - left
        Point([-0.105, -0.06, 00.00]),  // M2- back left
        Point([00.01, -0.115, 00.00]),  // M3 - back
        Point([00.09, -0.085, 00.00]),  // M4 - back right
        Point([00.12, 00.00, 00.00]),   // M5 - right
        Point([00.085, 00.105, 00.00]), // M6 - front right
        Point([00.00, 00.16, 00.00]),   // M7 - front
    ];

    let event_start_time = Mutex::new(None);
    let event_start_ref = &event_start_time;
    // Bitmap.
    // seen_status & 1 << i corresponds to whether the i-th mic has already seen a rising edge in
    // this impulse event.
    let seen_status = AtomicU8::new(0);
    let seen_status_ref = &seen_status;
    let event_duration = Duration::from_millis(100);

    // List of events observed in this impulse observation
    let events = Mutex::new(Vec::new());
    let events_ref = &events;

    let mut chip = gpio_cdev::Chip::new("/dev/gpiochip0")?;
    let event_sources = MIC_INPUT_PINS.iter().map(|&pin| {
        chip.get_line(pin)
            .unwrap()
            .events(
                LineRequestFlags::INPUT,
                EventRequestFlags::RISING_EDGE,
                "headsup-audio",
            )
            .unwrap()
    });

    let start_time = Instant::now();

    std::thread::scope(|s| {
        let mut handles = Vec::new();
        for (mic_id, event_source) in event_sources.enumerate() {
            let point = MIC_POINTS[mic_id];
            handles.push(s.spawn(move || {
                for event in event_source {
                    if event.is_err() {
                        continue;
                    }
                    let now = Instant::now();
                    let mut event_start_guard = event_start_ref.lock().unwrap();

                    // if it's been a while since the last rising edge seen by anyone,
                    // or nobody's seeon one at all,
                    // it's a new rising edge event.
                    if event_start_guard
                        .map(|start_time| now.duration_since(start_time) > event_duration)
                        .unwrap_or(true)
                    {
                        *event_start_guard = Some(now);
                        // make sure to release the mutex as early as possible!
                        drop(event_start_guard);
                        seen_status_ref.store(1 << mic_id, Ordering::Relaxed);
                        println!(
                            "Microphone {mic_id} saw a rising edge at {:?} and started the event",
                            now.duration_since(start_time)
                        );

                        events_ref.lock().unwrap().push(Event {
                            location: point,
                            time: now.duration_since(start_time).as_secs_f32(),
                        });
                        continue;
                    }
                    // make sure to release the mutex as early as possible!
                    drop(event_start_guard);
                    // if this thread hasn't seen a rising edge in this event, mark it as seeing one
                    // and update to match
                    if seen_status_ref.load(Ordering::Relaxed) & 1 << mic_id == 0 {
                        seen_status_ref.fetch_or(1 << mic_id, Ordering::Relaxed);
                        println!(
                            "Microphone {mic_id} saw a rising edge at {:?}",
                            now.duration_since(start_time)
                        );

                        let mut events_guard = events_ref.lock().unwrap();

                        events_guard.push(Event {
                            location: point,
                            time: now.duration_since(start_time).as_secs_f32(),
                        });

                        if events_guard.len() == MIC_INPUT_PINS.len() {
                            // we are the last event - initiate a localization process!
                            let localization_result =
                                source_of_shot(&events_guard, 10_000, 5e-9, 0.05);

                            if let Ok((shot_evt, n_iters, train_err)) = localization_result {
                                println!("Localized shot to {shot_evt:?} in {n_iters} iterations (MSE {train_err})");
                            } else {
                                println!("Failed to find solution for gunshot source");
                            }

                            events_guard.clear();
                        }
                    }
                }
            }));
        }

        for handle in handles {
            // wait for all threads to die (this will never happen)
            handle.join().unwrap();
        }
    });

    Ok(())
}
