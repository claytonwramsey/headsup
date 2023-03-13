use std::{
    mem::{transmute, MaybeUninit},
    sync::Mutex,
    thread::sleep,
    time::Duration,
};

use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

use once_cell::sync::Lazy;

/// The number of microphone output pins.
const N_OUTPUT_PINS: usize = 8;
/// The pin IDs for the output microphones.
const MIC_OUTPUT_PINS: [u32; N_OUTPUT_PINS] = [17; N_OUTPUT_PINS]; // TODO: fix
/// One-half of the period of the buzzer's square waveform.
const BUZZ_HALF_PERIOD: Duration = Duration::from_micros(5880);
/// Number of full periods of the buzzer wavefrom to go through before stopping buzzing.
const N_BUZZ_PERIODS: usize = 100;

/// Microphone output handles will be lazily initialized on program start.
///
/// Question: should we wrap all line handles in one mutex (more memory efficient, but prevents
/// multiple buzzers from working at once) or have them each in their own mutex?
static MIC_OUTPUT_HANDLES: Lazy<Mutex<[LineHandle; N_OUTPUT_PINS]>> = Lazy::new(|| {
    // we need to use unsafe code here to allocate a fixed-sized array containing uninitialized
    // values
    Mutex::new(unsafe {
        // SAFETY: The uninitialized values from the lines below are totally overwritten by the end
        // of the block.

        let mut chip = Chip::new("/dev/gpiochip0").unwrap();
        let mut handles: [MaybeUninit<LineHandle>; N_OUTPUT_PINS] =
            MaybeUninit::uninit().assume_init();
        for (handle, &pin_id) in handles.iter_mut().zip(MIC_OUTPUT_PINS.iter()) {
            handle.write(
                chip.get_line(pin_id)
                    .unwrap()
                    .request(LineRequestFlags::OUTPUT, 0, "headsup-audio-mic-out")
                    .unwrap(),
            );
        }

        transmute(handles)
    })
});

/// Buzz one handle for a short period of time.
///
/// Blocks the calling thread (TODO: decide if that is acceptable?)
///
/// # Panics
///
/// This function will panic if we are unable to write to the mic output pin referred to by
/// `output_id`.
pub fn buzz(output_id: usize) {
    let handle = MIC_OUTPUT_HANDLES.lock().unwrap();
    for _ in 0..N_BUZZ_PERIODS {
        handle[output_id].set_value(1).unwrap();
        sleep(BUZZ_HALF_PERIOD);
        handle[output_id].set_value(0).unwrap();
        sleep(BUZZ_HALF_PERIOD);
    }
}
