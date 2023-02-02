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

int main() {
    P2DIR = 0;
    P2OUT = 0;

    haptic_setup();
    start_pwm();

    while (1) {
        // main loop

    }
}
