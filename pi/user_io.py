"""
Module for user input/output to Raspberry Pi.

@author Team Headsup
@date: 2/15/2023
"""
from typing import List
import RPi.GPIO as GPIO


class UserInputOutput:
    """
	User input and output class. Supports three buttons and single LED.
	"""

    def __init__(self, B1_pin: int, B2_pin: int, B3_pin: int, L0_pin: int):
        self.B1_pin = B1_pin
        self.B2_pin = B2_pin
        self.B3_pin = B3_pin
        self.L0_pin = L0_pin

        self.led_on = False

        GPIO.setwarnings(False)
        GPIO.setmode(GPIO.BCM)

        GPIO.setup(self.B1_pin, GPIO.IN)
        GPIO.setup(self.B2_pin, GPIO.IN)
        GPIO.setup(self.B3_pin, GPIO.IN)
        GPIO.setup(self.L0_pin, GPIO.OUT)

    def toggle_led(self):
        self.led_on = not self.led_on
        if self.led_on:
            GPIO.output(self.L0_pin, GPIO.HIGH)
        else:
            GPIO.output(self.L0_pin, GPIO.LOW)

    def led_on(self):
        self.led_on = True
        GPIO.output(self.L0_pin, GPIO.HIGH)

    def led_off(self):
        self.led_off = True
        GPIO.output(self.L0_pin, GPIO.LOW)

    def poll(self) -> List[bool]:
        """
		Poll the buttons.

		:returns: a list of booleans for buttons 1, 2, and 3.
		"""
        return [
            GPIO.input(self.B1_pin) == GPIO.HIGH,
            GPIO.input(self.B2_pin) == GPIO.HIGH,
            GPIO.input(self.B3_pin) == GPIO.HIGH
        ]

    def cleanup(self):
        GPIO.cleanup()
