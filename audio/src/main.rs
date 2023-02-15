#![warn(clippy::pedantic)]

use std::time::Instant;

use gpio_cdev::{EventRequestFlags, LineRequestFlags};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// The GPIO pin IDs which are associated with microphone input.
    const MIC_INPUT_PINS: [u32; 8] = [2, 3, 4, 9, 15, 18, 17, 27];

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
                    match event {
                        Ok(_) => {
                            println!(
                                "rising edge on mic thread {mic_id} at time {:?}",
                                Instant::now().duration_since(start_time)
                            );
                        }
                        Err(_) => println!("error on mic thread {mic_id}"),
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
