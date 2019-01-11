use std::collections::HashMap;

pub enum RawType {
    U32,
    I32,
}

impl RawType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            RawType::U32 => "u32",
            RawType::I32 => "i32",
        }
    }
}

pub struct Extra {
    pub raw_type: RawType,
    pub vari_prefix: &'static str,
    pub vari_blacklist: Vec<&'static str>,
}

pub fn create_extra() -> HashMap<&'static str, Extra> {
    let mut map = HashMap::new();

    map.insert(
        "EVREye",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVREye_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ETextureType",
        Extra {
            raw_type: RawType::I32,
            vari_prefix: "ETextureType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EColorSpace",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EColorSpace_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ETrackingResult",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "ETrackingResult_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ETrackedDeviceClass",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "ETrackedDeviceClass_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ETrackedControllerRole",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "ETrackedControllerRole_",
            vari_blacklist: vec!["TrackedControllerRole_Max"],
        },
    );
    map.insert(
        "ETrackingUniverseOrigin",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "ETrackingUniverseOrigin_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ETrackedDeviceProperty",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "ETrackedDeviceProperty_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ETrackedPropertyError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "ETrackedPropertyError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRSubmitFlags",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRSubmitFlags_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRState",
        Extra {
            raw_type: RawType::I32,
            vari_prefix: "EVRState_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVREventType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVREventType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EDeviceActivityLevel",
        Extra {
            raw_type: RawType::I32,
            vari_prefix: "EDeviceActivityLevel_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRButtonId",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRButtonId_",
            vari_blacklist: vec![
                "k_EButton_SteamVR_Touchpad",
                "k_EButton_SteamVR_Trigger",
                "k_EButton_Dashboard_Back",
                "k_EButton_Knuckles_A",
                "k_EButton_Knuckles_B",
                "k_EButton_Knuckles_JoyStick",
            ],
        },
    );
    map.insert(
        "EVRMouseButton",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRMouseButton_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EDualAnalogWhich",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EDualAnalogWhich_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EShowUIType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EShowUIType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRInputError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRInputError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRSpatialAnchorError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRSpatialAnchorError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EHiddenAreaMeshType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EHiddenAreaMeshType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRControllerAxisType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRControllerAxisType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRControllerEventOutputType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRControllerEventOutputType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ECollisionBoundsStyle",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "ECollisionBoundsStyle_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVROverlayError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVROverlayError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRApplicationType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRApplicationType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRFirmwareError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRFirmwareError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRNotificationError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRNotificationError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRSkeletalMotionRange",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRSkeletalMotionRange_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRSkeletalTrackingLevel",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRSkeletalTrackingLevel_",
            vari_blacklist: vec!["VRSkeletalTrackingLevel_Max"],
        },
    );
    map.insert(
        "EVRInitError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRInitError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRScreenshotType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRScreenshotType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRScreenshotPropertyFilenames",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRScreenshotPropertyFilenames_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRTrackedCameraError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRTrackedCameraError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRTrackedCameraFrameLayout",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRTrackedCameraFrameType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRTrackedCameraFrameType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRDistortionFunctionType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRDistortionFunctionType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVSync",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVSync_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRMuraCorrectionMode",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "Imu_OffScaleFlags",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "Imu_OffScaleFlags_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRApplicationError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRApplicationError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRApplicationProperty",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRApplicationProperty_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRApplicationTransitionState",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRApplicationTransitionState_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "ChaperoneCalibrationState",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EChaperoneConfigFile",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EChaperoneImportFlags",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EChaperoneImportFlags_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRCompositorError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRCompositorError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRCompositorTimingMode",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRCompositorTimingMode_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "VROverlayInputMethod",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "VROverlayTransformType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "VROverlayTransformType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "VROverlayFlags",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "VRMessageOverlayResponse",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EGamepadTextInputMode",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EGamepadTextInputMode_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EGamepadTextInputLineMode",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EGamepadTextInputLineMode_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EOverlayDirection",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EOverlayDirection_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVROverlayIntersectionMaskPrimitiveType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVROverlayIntersectionMaskPrimitiveType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRRenderModelError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRRenderModelError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRComponentProperty",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRComponentProperty_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRNotificationType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRNotificationStyle",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRSettingsError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRSettingsError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRScreenshotError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRScreenshotError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRSkeletalTransformSpace",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRSkeletalTransformSpace_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRSkeletalReferencePose",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRSkeletalReferencePose_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRFinger",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRFinger_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRFingerSplay",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRFingerSplay_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRInputFilterCancelType",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EVRInputFilterCancelType_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EVRInputStringBits",
        Extra {
            raw_type: RawType::I32,
            vari_prefix: "EVRInputStringBits_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EIOBufferError",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EIOBufferError_",
            vari_blacklist: vec![],
        },
    );
    map.insert(
        "EIOBufferMode",
        Extra {
            raw_type: RawType::U32,
            vari_prefix: "EIOBufferMode_",
            vari_blacklist: vec![],
        },
    );

    map
}
