from imu import Orientation
from vision import VisionSystem

import depthai as dai

pipeline = dai.Pipeline()

orientation = Orientation(pipeline)
vision = VisionSystem(pipeline)

with dai.Device(pipeline) as device:
    vision.periodic()
    orientation.periodic()
    print(orientation.current_quaternion())
