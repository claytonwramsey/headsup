"""
Module for interfacing with OAK-D Lite IMU.
Sections of code are provided by Luxonis in their IMU documentation.

Date: 3/11/2023
"""

from ahrs import Quaternion
import depthai as dai
import numpy as np


class Orientation:
    def __init__(self, pipeline: dai.Pipeline):
        """
        Initialize the orientation computation system.
        """
        self.has_device = False
        self.pipeline = pipeline  # dai pipeline

        # Define sources and outputs (setup from Luxonis example script)
        self.imu = pipeline.create(dai.node.IMU)
        xlinkOut = pipeline.create(dai.node.XLinkOut)

        xlinkOut.setStreamName("imu")

        # enable ACCELEROMETER_RAW at 500 hz rate
        self.imu.enableIMUSensor(dai.IMUSensor.ROTATION_VECTOR, 400)
        # it's recommended to set both setBatchReportThreshold and setMaxBatchReports to 20 when
        # integrating in a pipeline with a lot of input/output connections.
        # above this threshold packets will be sent in batch of X, if the host is not blocked and
        # USB bandwidth is available
        self.imu.setBatchReportThreshold(1)
        # maximum number of IMU packets in a batch, if it's reached device will block sending until
        # host can receive it.
        # if lower or equal to batchReportThreshold then the sending is always blocking on device
        # useful to reduce device's CPU load and number of lost packets, if CPU load is high on
        # device side due to multiple nodes
        self.imu.setMaxBatchReports(10)

        # Link plugins IMU -> XLINK
        self.imu.out.link(xlinkOut.input)

    def use_device(self, device: dai.Device):
        """
        Prepare the IMU.
        Must be called before calling `update()`.
        """

        # The queue will store several readings sent in bulk over a USB connection
        self.imuQueue = device.getOutputQueue(
            name="imu", maxSize=50, blocking=False)
        self.has_device = True

    def get_euler_angles(self) -> np.ndarray:
        """
        Get the current heading of the orientation as euler angles.
        """
        if not self.has_device:
            raise RuntimeError(
                "IMU was not initialized with `use_device()` - cannot perform periodic()")

        imu_data = self.imuQueue.get()
        imu_packets = imu_data.packets
        rv = imu_packets[-1].rotationVector

        orientation_quat = Quaternion([rv.real, rv.i, rv.j, rv.k])
        euler_angles = orientation_quat.to_angles()

        return euler_angles
