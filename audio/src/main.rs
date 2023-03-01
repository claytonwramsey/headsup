use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};

use gpio_cdev::{EventRequestFlags, LineRequestFlags};

use headsup_audio::localization::compute_direction;
use nalgebra::{SMatrix, SVector};

const N_MICS: usize = 8;

fn main() {
    /// The GPIO pin IDs which are associated with microphone input.
    const MIC_INPUT_PINS: [u32; N_MICS] = [2, 3, 4, 9, 15, 18, 17, 27];
    let mic_points = SMatrix::<f64, 2, 8>::from_row_slice(&[
        -0.09, -0.14, -0.105, -0.01, 0.09, 0.12, 0.085, 0.0, // x positions
        0.095, 0.015, -0.06, -0.115, -0.085, 0.0, 0.105, 0.16, // y positions
    ]);
    let mic_points_ref = &mic_points;

    let mic_times = Mutex::new(SVector::<f64, 8>::zeros());
    let mic_times_ref = &mic_times;

    let event_start_time = Mutex::new(None);
    let event_start_ref = &event_start_time;

    let degrees = std::env::args().nth(2).unwrap();
    let range = std::env::args().nth(3).unwrap();

    println!(
        "Test on file {}: {degrees} degrees at range {range}",
        std::env::args().nth(1).unwrap()
    );
    // Bitmap.
    // seen_status & 1 << i corresponds to whether the i-th mic has already seen a rising edge in
    // this impulse event.
    let seen_status = AtomicU8::new(0);
    let seen_status_ref = &seen_status;
    let event_duration = Duration::from_millis(1000);

    let mut chip = gpio_cdev::Chip::new("/dev/gpiochip0").unwrap();
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
                    let last_event = if event_start_guard
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

                        mic_times_ref.lock().unwrap()[mic_id] =
                            now.duration_since(start_time).as_secs_f64();
                        false
                    } else {
                        // make sure to release the mutex as early as possible!
                        drop(event_start_guard);
                        // if this thread hasn't seen a rising edge in this event, mark it as seeing one
                        // and update to match
                        let prior_event_state =
                            seen_status_ref.fetch_or(1 << mic_id, Ordering::Relaxed);
                        if prior_event_state & 1 << mic_id == 0 {
                            println!(
                                "Microphone {mic_id} saw a rising edge at {:?}",
                                now.duration_since(start_time)
                            );

                            mic_times_ref.lock().unwrap()[mic_id] =
                                now.duration_since(start_time).as_secs_f64();
                            prior_event_state.count_ones() == N_MICS as u32 - 1
                        } else {
                            false
                        }
                    };

                    if last_event {
                        let direction =
                            compute_direction(mic_points_ref, &mic_times_ref.lock().unwrap());

                        println!("time vector: {:?}", mic_times_ref.lock().unwrap());
                        println!("pointing to source direction: {direction:?}");
                    }
                }
            }));
        }

        for handle in handles {
            // wait for all threads to die (this will never happen)
            handle.join().unwrap();
        }
    });
}
