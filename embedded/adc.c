/**
 * Source code for ADC module.
 *
 * @date 2/02/2023
 */

#include "adc.h"

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
