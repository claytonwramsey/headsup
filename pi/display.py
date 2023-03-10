"""
Module for HUD display output

@author Team Headsup
@date: 2/15/2023
"""

from typing import List, Tuple
import numpy as np
import cv2


class Display:
    """
    Class for overlaying radar screens over rearview image.
    Run test_display.py to test this implementation
    """
    MIN_DEPTH = 1*12*25.4  # in mm
    MAX_DEPTH = 20*12*25.4  # in mm

    def __init__(self, motion_color: Tuple[float, float, float],
                 static_color: Tuple[float, float, float],
                 location: Tuple[float, float], size: float,
                 icon_size=5):
        """
        Initialize display map with provided parameters

        @param motion_color: RGB color of moving tracklets
        @param static_color: RGB color of static or slow-moving tracklets
        @param location: a tuple with the location of the top left corner of the map
        @param icon_size: an integer with the size (in pixels) circles representing tracklets
        """
        self.motion_color = motion_color
        self.static_color = static_color
        self.location = location
        self.size = size
        self.icon_size = icon_size

    def update_radar_screen(self, img: np.array, rt_pairs: List[Tuple[float, float]]) -> np.array:
        """
        Given an input OpenCV image, overlay a radar screen on top

        @param img: OpenCV image with
        @param rt_pairs: a list of tuples containing (distance in mm, angle in degrees) to each tracklet

        @return an OpenCV image
        """
        output_img = np.copy(img)
        output_img = cv2.flip(output_img, 1)  # flip horizontally
        output_img = cv2.rectangle(output_img, self.location, self.location + np.array([self.size, self.size]), (0, 0, 0), -1)

        for pair in rt_pairs:
            r, theta = pair
            # print(f"r, theta = {r}, {theta}")
            disp_r = r/self.MAX_DEPTH*self.size  # convert to pixels
            # print(f"Disp_r: {disp_r}")
            disp_x = int(np.sin(theta*np.pi/180)*disp_r) + self.location[0] + self.size//2
            disp_y = int(np.cos(theta*np.pi/180)*disp_r) + self.location[1]
            # print(f"(x, y) <- {disp_x}, {disp_y}")

            output_img = cv2.circle(output_img, (disp_x, disp_y), self.icon_size, self.static_color, -1)

        return output_img
