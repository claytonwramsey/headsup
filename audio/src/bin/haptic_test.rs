/// Run a quick test of the haptic output buzzers.
fn main() {
    println!("Running haptics test! Remember to make sure the GPIO pin assignment in `haptics.rs` is correct!");
    loop {
        for id in 0..8 {
            println!("Vibrating buzzer {id}...");
            headsup_audio::haptics::buzz(id);
        }
    }
}
