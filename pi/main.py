import cv2
from imu import Orientation
from vision import VisionSystem
from display import Display
from user_io import UserInputOutput
import depthai as dai


def main():
    pipeline = dai.Pipeline()

    orientation = Orientation(pipeline)
    vision = VisionSystem(pipeline)
    overlay_display = Display(motion_color=(255, 255, 0),
                              static_color=(0, 255, 255),
                              size=200,
                              icon_size=5)

    io_manager = UserInputOutput(B1_pin=15, B2_pin=15, B3_pin=15, L0_pin=15)

    with dai.Device(pipeline) as device:
        vision.use_device(device)
        orientation.use_device(device)

        while True:
            current_frame, rho_theta_pairs = vision.periodic()
            current_orientation = orientation.get_euler_angles()
            print(current_orientation)

            radar_frame = overlay_display.update_radar_screen(current_frame, rho_theta_pairs)
            cv2.imshow("HEADSUP Application", current_frame)
            cv2.imshow("Radar Screen", radar_frame)
            cv2.waitKey(0)


if __name__ == "__main__":
    main()
