from display import Display
import cv2
import numpy as np


def main():
    overlay_display = Display(motion_color=(255, 255, 0),
                              static_color=(0, 255, 255),
                              location=(200, 200), size=100,
                              icon_size=5)

    depth_ai_output = cv2.imread('example_img.jpg')
    example_rt_pairs = [(3 * 12 * 25.4, 20 * np.pi / 180), (5 * 12 * 25.4, -20 * np.pi / 180)]

    display_output = overlay_display.update_radar_screen(depth_ai_output, example_rt_pairs)
    cv2.imshow("Output Display", display_output)
    wait = cv2.waitKey(0)


if __name__ == '__main__':
    main()
