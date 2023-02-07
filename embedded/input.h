/**
 * Module for interfacing with internal MSP430 ADC.
 *
 * @date 2/02/2023
 */

#include <msp430.h>
#include <stdlib.h>

#ifndef INPUT_H_
#define INPUT_H_

void input_setup();

void adc_setup();

void read_adc(unsigned int*);

#endif
