/**
 * Proof of concept program for impulse localization.
 * ADC sampling section of code uses code from https://youtu.be/Ev871bhGFt0 (drselim).
 * Note that eventually an external ADC will be used to simultaneously sample all
 * channels.
 *
 * @date 1/27/2023
 */

#include <msp430.h> 
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <math.h>
#include "haptic.h"
#include "adc.h"

unsigned int adc[8];

int main() {
    P2DIR = 0;
    P2OUT = 0;
    WDTCTL = WDTPW | WDTHOLD;   // stop watchdog timer

    haptic_setup();
    start_pwm();
    adc_setup();

    while (1) {
        // main loop

    }
}
