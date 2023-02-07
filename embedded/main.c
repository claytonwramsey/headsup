/**
 * Proof of concept program for impulse localization.
 * ADC sampling section of code uses code from https://youtu.be/Ev871bhGFt0 (drselim).
 * Note that eventually an external ADC will be used to simultaneously sample all
 * channels.
 *
 * @date 2/05/2023
 */

#include <msp430.h> 
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>
#include "input.h"
#include "haptic.h"

unsigned int adc[8];

int main() {
    WDTCTL = WDTPW | WDTHOLD;   // stop watchdog timer
    P1DIR = 0;
    P2DIR = 0;
    P1OUT = 0;
    P2OUT = 0;

    haptic_setup();
    // start_pwm();
    input_setup();
    // adc_setup();

    _enable_interrupts();

    while (1) {
        // main loop
    }
}
