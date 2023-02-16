#![warn(clippy::pedantic)]

use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};

use gpio_cdev::{EventRequestFlags, LineRequestFlags};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// The GPIO pin IDs which are associated with microphone input.
    const MIC_INPUT_PINS: [u32; 8] = [2, 3, 4, 9, 15, 18, 17, 27];

    let event_start_time = Mutex::new(None);
    let event_start_ref = &event_start_time;
    // Bitmap.
    // seen_status & 1 << i corresponds to whether the i-th mic has already seen a rising edge in
    // this impulse event.
    let seen_status = AtomicU8::new(0);
    let seen_status_ref = &seen_status;
    let event_duration = Duration::from_millis(100);

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
                        continue;
                    }
                    // make sure to release the mutex as early as possible!
                    drop(event_start_guard);
                    // if this thread hasn't seen a rising edge in this event, mark it as seeing one
                    // and update to match
                    if seen_status_ref.load(Ordering::Relaxed) & 1 << mic_id != 0 {
                        seen_status_ref.fetch_or(1 << mic_id, Ordering::Relaxed);
                        println!(
                            "Microphone {mic_id} saw a rising edge at {:?}",
                            now.duration_since(start_time)
                        );
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
