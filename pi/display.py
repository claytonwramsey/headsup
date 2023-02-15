"""
Module for HUD display output.

@author Team Headsup
@date: 2/15/2023
"""

from typing import List, Tuple
import depthai as dai
import numpy as np
import cv2


class Display:
    MIN_DEPTH = 1*12*25.4  # in mm
    MAX_DEPTH = 13*12*25.4  # in mm

    def __init__(self, motion_color: Tuple[float, float, float],
                 static_color: Tuple[float, float, float],
                 location: Tuple[float, float], size: float,
                 icon_size=5):
        self.motion_color = motion_color
        self.static_color = static_color
        self.location = location
        self.size = size
        self.icon_size = icon_size

    def update_radar_screen(img: np.array, rt_pairs: List[Tuple[float, float]]) -> np.array:
        output_img = np.copy(img)

        if (size[0] != size[1]):
            raise RuntimeError(
                "Vision system does not currently support non-square simplified displays")

        for pair in rt_pairs:
            r, theta = pair
            disp_r = r/MAX_DEPTH*self.size  # convert to pixels
            disp_x = int(np.cos(theta)*disp_r)
            disp_y = int(np.sin(theta)*disp_r)

            disp_x += self.location[0]
            disp_y += self.location[1]  # add in offsets

            output_img = cv2.circle(output_img, (disp_x, disp_y), self.icon_size, self.static_color, -1)

        return output_img
