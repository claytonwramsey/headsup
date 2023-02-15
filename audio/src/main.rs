#![warn(clippy::pedantic)]

use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use gpio_cdev::{EventRequestFlags, LineRequestFlags};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// The GPIO pin IDs which are associated with microphone input.
    const MIC_INPUT_PINS: [u32; 8] = [2, 3, 4, 9, 15, 18, 17, 27];

    let event_start_time = Mutex::new(None);
    let seen_events = Mutex::new(vec![false; MIC_INPUT_PINS.len()]);

    let mutex_ref = &event_start_time;
    let seen_events_ref = &seen_events;
    let event_duration = Duration::from_millis(500);

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
                    let mut ev_start_guard = mutex_ref.lock().unwrap();
                    let now = Instant::now();
                    match *ev_start_guard {
                        Some(time) if now.duration_since(time) > event_duration => {
                            *ev_start_guard = Some(now);
                            let mut new_seen = vec![false; MIC_INPUT_PINS.len()];
                            new_seen[mic_id] = true;
                            *seen_events_ref.lock().unwrap() = new_seen;

                            println!(
                                "started mic event on thread {mic_id} at time {:?}",
                                now.duration_since(start_time)
                            );
                        }
                        None => {
                            *ev_start_guard = Some(now);
                            let mut new_seen = vec![false; MIC_INPUT_PINS.len()];
                            new_seen[mic_id] = true;
                            *seen_events_ref.lock().unwrap() = new_seen;
                            println!(
                                "started mic event on thread {mic_id} at time {:?}",
                                now.duration_since(start_time)
                            );
                        }
                        _ if !seen_events_ref.lock().unwrap()[mic_id] => {
                            seen_events_ref.lock().unwrap()[mic_id] = true;
                            println!("observed mic event on thread {mic_id}");
                        }
                        _ => (),
                    };
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
