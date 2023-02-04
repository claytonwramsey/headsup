from pathlib import Path
import depthai as dai


class VisionSystem:
    """
    A handler for the entire vision system on the Raspberry Pi for the HeadsUp project.
    """

    # number of columns in map
    NUM_COLUMNS = 10

    # map from IDs to labels
    LABEL_MAP = ["background", "aeroplane", "bicycle", "bird", "boat", "bottle", "bus", "car", "cat", "chair", "cow",
                 "diningtable", "dog", "horse", "motorbike", "person", "pottedplant", "sheep", "sofa", "train", "tvmonitor"]

    def __init__(self, pipeline: dai.Pipeline):
        """
        Set up the vision system.
        """
        self.has_device = False

        nnPath = str(
            (Path(__file__).parent / Path('mobilenet-ssd_openvino_2021.4_5shave.blob')).resolve().absolute())

        # Define sources and outputs
        monoLeft = pipeline.create(dai.node.MonoCamera)
        monoRight = pipeline.create(dai.node.MonoCamera)
        stereo = pipeline.create(dai.node.StereoDepth)
        spatialLocationCalculator = pipeline.create(
            dai.node.SpatialLocationCalculator)

        camRgb = pipeline.create(dai.node.ColorCamera)
        spatialDetectionNetwork = pipeline.create(
            dai.node.MobileNetSpatialDetectionNetwork)
        objectTracker = pipeline.create(dai.node.ObjectTracker)

        xoutDepth = pipeline.create(dai.node.XLinkOut)
        xoutSpatialData = pipeline.create(dai.node.XLinkOut)
        xinSpatialCalcConfig = pipeline.create(dai.node.XLinkIn)

        trackerOut = pipeline.create(dai.node.XLinkOut)

        xoutDepth.setStreamName("depth")
        xoutSpatialData.setStreamName("spatialData")
        xinSpatialCalcConfig.setStreamName("spatialCalcConfig")

        trackerOut.setStreamName("tracklets")

        # Properties
        monoLeft.setResolution(
            dai.MonoCameraProperties.SensorResolution.THE_400_P)
        monoLeft.setBoardSocket(dai.CameraBoardSocket.LEFT)
        monoRight.setResolution(
            dai.MonoCameraProperties.SensorResolution.THE_400_P)
        monoRight.setBoardSocket(dai.CameraBoardSocket.RIGHT)

        camRgb.setPreviewSize(300, 300)
        camRgb.setResolution(
            dai.ColorCameraProperties.SensorResolution.THE_1080_P)
        camRgb.setInterleaved(False)
        camRgb.setColorOrder(dai.ColorCameraProperties.ColorOrder.BGR)

        stereo.setDefaultProfilePreset(
            dai.node.StereoDepth.PresetMode.HIGH_DENSITY)
        stereo.setLeftRightCheck(True)
        stereo.setSubpixel(False)
        # Align depth map to the perspective of RGB camera, on which inference is done
        stereo.setDepthAlign(dai.CameraBoardSocket.RGB)
        stereo.setOutputSize(monoLeft.getResolutionWidth(),
                             monoLeft.getResolutionHeight())

        spatialLocationCalculator.inputConfig.setWaitForMessage(False)

        spatialDetectionNetwork.setBlobPath(nnPath)
        spatialDetectionNetwork.setConfidenceThreshold(0.5)
        spatialDetectionNetwork.input.setBlocking(False)
        spatialDetectionNetwork.setBoundingBoxScaleFactor(0.5)
        spatialDetectionNetwork.setDepthLowerThreshold(100)
        spatialDetectionNetwork.setDepthUpperThreshold(5000)

        objectTracker.setDetectionLabelsToTrack([15])  # track only person
        # possible tracking types: ZERO_TERM_COLOR_HISTOGRAM, ZERO_TERM_IMAGELESS, SHORT_TERM_IMAGELESS, SHORT_TERM_KCF
        objectTracker.setTrackerType(dai.TrackerType.ZERO_TERM_COLOR_HISTOGRAM)
        # take the smallest ID when new object is tracked, possible options: SMALLEST_ID, UNIQUE_ID
        objectTracker.setTrackerIdAssignmentPolicy(
            dai.TrackerIdAssignmentPolicy.SMALLEST_ID)

        # Linking
        monoLeft.out.link(stereo.left)
        monoRight.out.link(stereo.right)

        camRgb.preview.link(spatialDetectionNetwork.input)
        objectTracker.out.link(trackerOut.input)
        spatialDetectionNetwork.passthrough.link(
            objectTracker.inputTrackerFrame)

        spatialDetectionNetwork.passthrough.link(
            objectTracker.inputDetectionFrame)
        spatialDetectionNetwork.out.link(objectTracker.inputDetections)
        stereo.depth.link(spatialDetectionNetwork.inputDepth)

        spatialLocationCalculator.passthroughDepth.link(xoutDepth.input)
        stereo.depth.link(spatialLocationCalculator.inputDepth)

        spatialLocationCalculator.out.link(xoutSpatialData.input)
        xinSpatialCalcConfig.out.link(spatialLocationCalculator.inputConfig)

        for col in range(VisionSystem.NUM_COLUMNS):
            # Set up column configuration
            topLeft = dai.Point2f(col / VisionSystem.NUM_COLUMNS, 0.0)
            bottomRight = dai.Point2f(
                (col + 1) / (VisionSystem.NUM_COLUMNS), 1.0)

            cfgdata = dai.SpatialLocationCalculatorConfigData()
            cfgdata.depthThresholds.lowerThreshold = 100
            cfgdata.depthThresholds.upperThreshold = 10000
            cfgdata.roi = dai.Rect(topLeft, bottomRight)
            spatialLocationCalculator.initialConfig.addROI(cfgdata)

    def use_device(self, device: dai.Device):
        """
        After creating a device, initialize device-dependent parts of the vision system.
        """
        # Output queue will be used to get the depth frames from the outputs defined above
        self.depthQueue = device.getOutputQueue(
            name="depth", maxSize=4, blocking=False)
        self.spatialCalcQueue = device.getOutputQueue(
            name="spatialData", maxSize=4, blocking=False)
        self.spatialCalcConfigInQueue = device.getInputQueue(
            "spatialCalcConfig")
        # things that are being tracked
        self.tracklets = device.getOutputQueue("tracklets", 4, False)

        self.has_device = True

    def periodic(self):
        """
        Perform periodic steps associated with the vision system.
        """
        if not self.has_device:
            raise RuntimeError(
                "Vision system was not initialized with `use_device()` - cannot perform periodic()")

        # Blocking call, will wait until a new data has arrived
        self.depthQueue.get()

        spatialData = self.spatialCalcQueue.get().getSpatialLocations()
        trackletsData = self.tracklets.get().tracklets

        depths = [int(data.spatialCoordinates.z) for data in spatialData]
        print(depths)
        for t in trackletsData:
            tracklet_coordinates = np.array([data.spatialCoordinates.x, data.spatialCoordinates.y, data.spatialCoordinates.z])
            rho = np.sqrt(data.spatialCoordinates.x ** 2 + data.spatialCoordinates.z ** 2) * 0.00328084
            theta = np.arctan2(data.spatialCoordinates.x, data.spatialCoordinates.z) * 180/np.pi

            
            t.label = f"({rho}, {theta})"

            # label = VisionSystem.LABEL_MAP[t.label] if t.label < len(
            #     VisionSystem.LABEL_MAP) else str(t.label)
            # print(
            #     f"found tracklet {t.id}: {label} at ({t.spatialCoordinates.x:.4}, {t.spatialCoordinates.y:.4}, {t.spatialCoordinates.z:.4})")

