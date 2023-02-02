#include <msp430.h>
#include <stdlib.h>

#ifndef HAPTIC_H_
#define HAPTIC_H_

/**
 * Prepares TimerA to generate PWM signal.
 */
void haptic_setup();

/**
 * Maps a microphone identifier in range [1, 8] to a 4-bit binary control signal.
 * The control signal is used to control a decoder that passes a PWM signal to eight
 * output microphones.
 *
 * @param mic_id microphone identifier.
 */
void assert_digital_control(int);

/**
 * Start generating 170 Hz PWM signal to control haptic motors.
 */
void start_pwm();

/**
 * Stop generating 170 Hz PWM signal to control haptic motors.
 */
void stop_pwm();

#endif
