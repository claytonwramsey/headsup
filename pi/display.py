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
    RING_DELTA_FT = 5
    RING_DELTA = RING_DELTA_FT*12*25.4  # in mm
    FOV = 70  # degrees

    def __init__(self, motion_color: Tuple[float, float, float],
                 static_color: Tuple[float, float, float], size: float,
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
        self.size = size
        self.icon_size = icon_size
        self.font = cv2.FONT_HERSHEY_SIMPLEX
        self.font_scale = 1
        self.font_thickness = 1
        self.line_type = 2

    def update_radar_screen(self, img: np.array, rt_pairs: List[Tuple[float, float]]) -> np.array:
        """
        Given an input OpenCV image, overlay a radar screen on top

        @param img: OpenCV image with
        @param rt_pairs: a list of tuples containing (distance in mm, angle in degrees) to each tracklet

        @return an OpenCV image
        """
        radar_img = np.zeros((self.size, self.size, 3))

        # Draw distance circles
        for circle_ind in range(1, int(self.MAX_DEPTH / self.RING_DELTA)):
            radar_img = cv2.circle(radar_img, (self.size // 2, 0), int(self.RING_DELTA*float(circle_ind)*self.size/self.MAX_DEPTH), (255, 255, 255), 1)
            cv2.putText(radar_img, f"{circle_ind*self.RING_DELTA_FT}",
                        (self.size//2, int(self.RING_DELTA*float(circle_ind)*self.size/self.MAX_DEPTH)), self.font, self.font_scale, (255, 255, 255),
                        self.font_thickness, self.line_type)

        # Draw FOV lines
        radar_img = cv2.line(radar_img, (0, int(self.size/(2*np.tan(self.FOV//2)))), (self.size//2, 0), (255, 255, 255), 1)
        radar_img = cv2.line(radar_img, (self.size//2, 0), (self.size, int(self.size/(2*np.tan(self.FOV//2)))), (255, 255, 255), 1)

        # Add scale
        cv2.putText(radar_img, "ft.",
                    (0, self.size), self.font,
                    self.font_scale, (255, 255, 255),
                    self.font_thickness, self.line_type)

        # Draw user circle
        pt1 = (self.size//2, 0)
        pt2 = (self.size//2 - self.icon_size, self.icon_size*2)
        pt3 = (self.size//2 + self.icon_size, self.icon_size*2)
        triangle_points = np.array([pt1, pt2, pt3])
        radar_img = cv2.drawContours(radar_img, [triangle_points], 0, (0, 0, 255), -1)
        # radar_img = cv2.circle(radar_img, (self.size//2, 0), self.icon_size, (255, 0, 0), -1)

        for pair in rt_pairs:
            r, theta = pair
            # print(f"r, theta = {r}, {theta}")
            disp_r = r/self.MAX_DEPTH*self.size  # convert to pixels
            # print(f"Disp_r: {disp_r}")
            disp_x = int(np.sin(theta*np.pi/180)*disp_r) + self.size//2
            disp_y = int(np.cos(theta*np.pi/180)*disp_r)
            # print(f"(x, y) <- {disp_x}, {disp_y}")

            radar_img = cv2.circle(radar_img, (disp_x, disp_y), self.icon_size, self.static_color, -1)

        return radar_img
