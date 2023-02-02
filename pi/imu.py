"""
Module for interfacing with OAK-D Lite IMU.
Sections of code are provided by Luxonis in their IMU documentation.

Date: 2/02/2023
"""

from ahrs.filters import Madgwick
from ahrs.common.quaternion import Quaternion
import depthai as dai


class Orientation:
    def __init__(self, pipeline, device):
        self.madgwick_filter = Madgwick()
        self.orientation_q = np.array([1.0, 0.0, 0.0, 0.0])
        self.pipeline = pipeline  # dai pipeline
        self.device = device  # dai device

        # Define sources and outputs (setup from Luxonis example script)
        self.imu = pipeline.create(dai.node.IMU)
        xlinkOut = pipeline.create(dai.node.XLinkOut)

        xlinkOut.setStreamName("imu")

        # enable ACCELEROMETER_RAW at 500 hz rate
        imu.enableIMUSensor(dai.IMUSensor.ACCELEROMETER_CALIBRATED, 500)
        # enable GYROSCOPE_RAW at 400 hz rate
        imu.enableIMUSensor(dai.IMUSensor.GYROSCOPE_CALIBRATED, 400)
        # it's recommended to set both setBatchReportThreshold and setMaxBatchReports to 20 when integrating in a pipeline with a lot of input/output connections
# above this threshold packets will be sent in batch of X, if the host is not blocked and USB bandwidth is available
        imu.setBatchReportThreshold(1)
        # maximum number of IMU packets in a batch, if it's reached device will block sending until host can receive it
        # if lower or equal to batchReportThreshold then the sending is always blocking on device
        # useful to reduce device's CPU load  and number of lost packets, if CPU load is high on device side due to multiple nodes
        imu.setMaxBatchReports(10)

        # Link plugins IMU -> XLINK
        imu.out.link(xlinkOut.input)

    def create_queue(device: Device):
        """
        Prepare the IMU.
        Must be called before calling `update()`.
        """

        # The queue will store several readings sent in bulk over a USB connection
        self.imuQueue = device.getOutputQueue(name="imu", maxSize=50, blocking=False)

    def update(self) -> np.array:
        imu_packet = self.imuQueue.get()
        acc_data = imu_packet.acceleroMeter
        gyro_data = imu_packet.gyroscope
        self.madgwick_filter.updateIMU(self.orientation_q, gyr=gyro_data, acc=acc_data, dt=4e-3)

        return Quaternion.to_angles(self.orientation_q)

