/**
 * Module for interfacing with internal MSP430 ADC.
 *
 * @date 2/02/2023
 */

#include <msp430.h>
#include <stdlib.h>

#ifndef ADC_H_
#define ADC_H_

void adc_setup();

void read_adc(unsigned int*);

#endif
