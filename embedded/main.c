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

unsigned int high_threshold = 700;
unsigned int low_threshold = 300;
unsigned int q1, q2, q3, q4;
unsigned int adc[8];
unsigned int order[4];
unsigned int state_1 = 0, state_2 = 0, state_3 = 0, state_4 = 0;
unsigned int num_pulses_received = 0;

/**
 * Toggle LED corresponding to first quadrant.
 */
void toggle_quadrant1() {
    P2OUT ^= BIT4;
}

/**
 * Toggle LED corresponding to second quadrant.
 */
void toggle_quadrant2() {
    P2OUT ^= BIT3;
}

/**
 * Toggle LED corresponding to third quadrant.
 */
void toggle_quadrant3() {
    P2OUT ^= BIT5;
}

/**
 * Toggle LED corresponding to fourth quadrant.
 */
void toggle_quadrant4() {
    P2OUT ^= BIT0;
}


void main(void)
{
    WDTCTL = WDTPW | WDTHOLD;   // stop watchdog timer
    BCSCTL1= CALBC1_1MHZ;
    DCOCTL = CALDCO_1MHZ;
    P1SEL = BIT1|BIT2;
    P1SEL2 = BIT1|BIT2;
    UCA0CTL1 |= UCSWRST+UCSSEL_2;
    UCA0BR0 = 52;
    UCA0BR1 = 0;
    UCA0MCTL = UCBRS_0;
    UCA0CTL1 &= ~UCSWRST;

    CCTL0 = CCIE; // Timer A interrupt enabled (CCTL0 is TACCTL0)
    TACTL = TASSEL_2 + MC_2 + ID_3; // set Timer A to continuous mode with SMCLK as a counting trigger

    ADC10CTL1 = INCH_7 + ADC10DIV_0 + CONSEQ_3 + SHS_0;
    ADC10CTL0 = SREF_0 + ADC10SHT_2 + MSC + ADC10ON; //ADC10IE
    ADC10AE0 = BIT7 + BIT6 + BIT5 + BIT4 + BIT3 + BIT0;
    ADC10DTC1 = 8;

    P2DIR = BIT0 + BIT3 + BIT4 + BIT5;
    P2OUT = 0;

    while (1) {
        ADC10CTL0 &= ~ENC;
        while (ADC10CTL1 & BUSY);
        ADC10CTL0 |= ENC + ADC10SC;
        ADC10SA = (unsigned int)adc;

        q1 = adc[4];
        q2 = adc[3];
        q3 = adc[1];
        q4 = adc[2];

        // Detect a peak (rising or falling)
        if ((q1 > high_threshold || q1 < low_threshold) && state_1 == 0) {
            state_1++;
        }
        if ((q2 > high_threshold || q2 < low_threshold) && state_2 == 0) {
            state_2++;
        }
        if ((q3 > high_threshold || q3 < low_threshold) && state_3 == 0) {
            state_3++;
        }
        if ((q4 > high_threshold || q4 < low_threshold) && state_4 == 0) {
            state_4++;
        }

        // Detect a zero-crossing
        if ((q1 <= 515 && q1 >= 510) && state_1 == 1) {
            order[num_pulses_received] = 1;
            num_pulses_received++;
            state_1++;
        }
        if ((q2 <= 515 && q2 >= 510) && state_2 == 1) {
            order[num_pulses_received] = 2;
            num_pulses_received++;
            state_2++;
        }
        if ((q3<= 515 && q3>= 510) && state_3 == 1) {
            order[num_pulses_received] = 3;
            num_pulses_received++;
            state_3++;
        }
        if ((q4 <= 515 && q4 >= 510) && state_4 == 1) {
            order[num_pulses_received] = 4;
            num_pulses_received++;
            state_4++;
        }

        // Compute quadrant and reset finite state machine
        if (num_pulses_received == 4) {
            if ((order[0] == 1 && order[1] == 2) || (order[0] == 2 && order[1] == 1)) {
                toggle_quadrant1();
            } else if ((order[0] == 2 && order[1] == 3) || (order[0] == 3 && order[1] == 2)) {
                toggle_quadrant2();
            } else if ((order[0] == 3 && order[1] == 4) || (order[0] == 4 && order[1] == 3)) {
                toggle_quadrant3();
            } else if ((order[0] == 4 && order[1] == 1) || (order[0] == 1 && order[1] == 4)) {
                toggle_quadrant4();
            } else {
                toggle_quadrant1();
                toggle_quadrant2();
                toggle_quadrant3();
                toggle_quadrant4();
            }

            state_1 = 0;
            state_2 = 0;
            state_3 = 0;
            state_4 = 0;
            num_pulses_received = 0;
            __delay_cycles(100000);
            P2OUT = 0;
        }
    }
}
