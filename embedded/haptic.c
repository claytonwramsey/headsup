/**
 * Source file for controlling haptic motors in helmet.
 *
 * @date 2/02/2023
 */
#include "haptic.h"

/**
 * Prepares TimerA to generate PWM signal.
 */
void haptic_setup() {
    CCTL0 = CCIE; // Timer A interrupt enabled (CCTL0 is TACCTL0)
    CCR0 = 0x0329; // set Timer A interrupt to trigger every 50,000 CPU cycles
    TA0CTL = TASSEL_2 + MC_1 + ID_3; // set Timer A to up mode with SMCLK as a counting trigger and an /8 divider
    TA0CCTL1 |= OUTMOD_7;

    P2DIR |= BIT1 | BIT2 | BIT3 | BIT4 | BIT5; // set pin 2.1, 2.2, 2.3, 2.4 and 2.5 as outputs
    P2SEL |= BIT1; // pwm-enable pin 2.1
}

/**
 * Maps a microphone identifier in range [1, 8] to a 4-bit binary control signal.
 * The control signal is used to control a decoder that passes a PWM signal to eight
 * output microphones.
 *
 * @param mic_id microphone identifier.
 */
void assert_digital_control(int mic_id) {
    P2OUT &= ~(BIT2 | BIT3 | BIT4 | BIT5); // clear previous output control signal
    switch (mic_id) {
    case 0:
        P2OUT |= BIT2;
        break;
    case 1:
        P2OUT |= BIT3;
        break;
    case 2:
        P2OUT |= BIT2 | BIT3;
        break;
    case 3:
        P2OUT |= BIT4;
        break;
    case 4:
        P2OUT |= BIT4 | BIT2;
        break;
    case 5:
        P2OUT |= BIT4 | BIT3;
        break;
    case 6:
        P2OUT |= BIT4 | BIT3 | BIT2;
        break;
    case 7:
        P2OUT |= BIT5;
        break;
    }
}

/**
 * Start generating 170 Hz PWM signal to control haptic motors.
 */
void start_pwm() {
    P2OUT |= BIT1; // allow pin 2.1 to turn on
}

/**
 * Stop generating 170 Hz PWM signal to control haptic motors.
 */
void stop_pwm() {
    P2OUT &= ~BIT1; // turn pin 2.1 off
}
