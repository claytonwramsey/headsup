/**
 * Source code for ADC module.
 *
 * @date 2/02/2023
 */

#include "input.h"


void input_setup() {
    P1SEL &= ~BIT3; // all of port 1 is set in GPIO mode
    P1DIR = 0; // all of port 1 is inputs
    P1IES &= ~BIT3; // listen for low-to-high transitions
    P1IE |= BIT3; // pin 1.3 can trigger a hardware interrupt
    P1IFG = 0x00; // clear all interrupt flags upon setup
    P2DIR |= BIT0; // 2.0 output for debugging purposes
    // P2OUT |= BIT3;
}

void adc_setup() {
    ADC10CTL1 = INCH_7 + ADC10DIV_0 + CONSEQ_3 + SHS_0;
    ADC10CTL0 = SREF_0 + ADC10SHT_2 + MSC + ADC10ON; //ADC10IE
    ADC10AE0 = BIT7 + BIT6 + BIT5 + BIT4 + BIT3 + BIT2 + BIT1 + BIT0; // sampling all of port 1
    ADC10DTC1 = 8;
}

void read_adc(unsigned int* data_array) {
    ADC10CTL0 &= ~ENC;
    while (ADC10CTL1 & BUSY);
    ADC10CTL0 |= ENC + ADC10SC;
    ADC10SA = (unsigned int)data_array; // ADC data write address is now data_array first element address
}

/**
 * Rising edge on Port 1 pin detected
 */
#if defined(__TI_COMPILER_VERSION__) || defined(__IAR_SYSTEMS_ICC__)
#pragma vector=PORT1_VECTOR
__interrupt void button(void)
#elif defined(__GNUC__)
void __attribute__ ((interrupt(PORT1_VECTOR))) button (void)
#else
#error Compiler not supported!
#endif
{
    __delay_cycles(1000);
    if (P1IN & BIT3) {
        // rising edge on 1.3
        P2OUT |= BIT0;
        __delay_cycles(100000);
        P2OUT &= ~BIT0; // briefly light up LED on 2.0
    }

    P1IFG &= 0x00; // clear interrupt bits
}

