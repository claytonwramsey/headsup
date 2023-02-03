from imu import Orientation
from vision import VisionSystem

import depthai as dai


def main():
    pipeline = dai.Pipeline()

    orientation = Orientation(pipeline)
    vision = VisionSystem(pipeline)

    with dai.Device(pipeline) as device:
        vision.periodic()
        orientation.periodic()
        print(orientation.current_quaternion())


if __name__ == "__main__":
    main()
