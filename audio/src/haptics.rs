use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        Mutex,
    },
    time::{Duration, Instant},
};

use gpio_cdev::{EventRequestFlags, LineRequestFlags};

const MIC_OUTPUT_PINS: [u32; 4] = [15, 15, 15, 15]; // TODO: fix
const PWM_PIN: u32 = 15; // TODO: fix

fn start_pwm -> Result<(), Box<dyn std::error::Error>> {
	
}

fn buzz_output(motor_id: u32) -> Result<(), Box<dyn std::error::Error>> {

}