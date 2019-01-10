#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawEye(pub u32);

pub const Eye_Left: RawEye = RawEye(0);
pub const Eye_Right: RawEye = RawEye(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Eye {
    Left = 0,
    Right = 1,
}

impl Eye {
    #[inline]
    fn from_raw(val: RawEye) -> Option<Self> {
        match val {
            Eye_Left => Some(Eye::Left),
            Eye_Right => Some(Eye::Right),
            _ => None,
        }
    }
}

impl From<RawEye> for Eye {
    fn from(val: RawEye) -> Self {
        Eye::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for Eye.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTextureType(pub u32);

pub const TextureType_Invalid: RawTextureType = RawTextureType(::std::u32::MAX);
pub const TextureType_DirectX: RawTextureType = RawTextureType(0);
pub const TextureType_OpenGL: RawTextureType = RawTextureType(1);
pub const TextureType_Vulkan: RawTextureType = RawTextureType(2);
pub const TextureType_IOSurface: RawTextureType = RawTextureType(3);
pub const TextureType_DirectX12: RawTextureType = RawTextureType(4);
pub const TextureType_DXGISharedHandle: RawTextureType = RawTextureType(5);
pub const TextureType_Metal: RawTextureType = RawTextureType(6);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TextureType {
    Invalid = ::std::u32::MAX,
    DirectX = 0,
    OpenGL = 1,
    Vulkan = 2,
    IOSurface = 3,
    DirectX12 = 4,
    DXGISharedHandle = 5,
    Metal = 6,
}

impl TextureType {
    #[inline]
    fn from_raw(val: RawTextureType) -> Option<Self> {
        match val {
            TextureType_Invalid => Some(TextureType::Invalid),
            TextureType_DirectX => Some(TextureType::DirectX),
            TextureType_OpenGL => Some(TextureType::OpenGL),
            TextureType_Vulkan => Some(TextureType::Vulkan),
            TextureType_IOSurface => Some(TextureType::IOSurface),
            TextureType_DirectX12 => Some(TextureType::DirectX12),
            TextureType_DXGISharedHandle => Some(TextureType::DXGISharedHandle),
            TextureType_Metal => Some(TextureType::Metal),
            _ => None,
        }
    }
}

impl From<RawTextureType> for TextureType {
    fn from(val: RawTextureType) -> Self {
        TextureType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TextureType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawColorSpace(pub u32);

pub const ColorSpace_Auto: RawColorSpace = RawColorSpace(0);
pub const ColorSpace_Gamma: RawColorSpace = RawColorSpace(1);
pub const ColorSpace_Linear: RawColorSpace = RawColorSpace(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ColorSpace {
    Auto = 0,
    Gamma = 1,
    Linear = 2,
}

impl ColorSpace {
    #[inline]
    fn from_raw(val: RawColorSpace) -> Option<Self> {
        match val {
            ColorSpace_Auto => Some(ColorSpace::Auto),
            ColorSpace_Gamma => Some(ColorSpace::Gamma),
            ColorSpace_Linear => Some(ColorSpace::Linear),
            _ => None,
        }
    }
}

impl From<RawColorSpace> for ColorSpace {
    fn from(val: RawColorSpace) -> Self {
        ColorSpace::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ColorSpace.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackingResult(pub u32);

pub const TrackingResult_Uninitialized: RawTrackingResult = RawTrackingResult(1);
pub const TrackingResult_Calibrating_InProgress: RawTrackingResult = RawTrackingResult(100);
pub const TrackingResult_Calibrating_OutOfRange: RawTrackingResult = RawTrackingResult(101);
pub const TrackingResult_Running_OK: RawTrackingResult = RawTrackingResult(200);
pub const TrackingResult_Running_OutOfRange: RawTrackingResult = RawTrackingResult(201);
pub const TrackingResult_Fallback_RotationOnly: RawTrackingResult = RawTrackingResult(300);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackingResult {
    Uninitialized = 1,
    Calibrating_InProgress = 100,
    Calibrating_OutOfRange = 101,
    Running_OK = 200,
    Running_OutOfRange = 201,
    Fallback_RotationOnly = 300,
}

impl TrackingResult {
    #[inline]
    fn from_raw(val: RawTrackingResult) -> Option<Self> {
        match val {
            TrackingResult_Uninitialized => Some(TrackingResult::Uninitialized),
            TrackingResult_Calibrating_InProgress => Some(TrackingResult::Calibrating_InProgress),
            TrackingResult_Calibrating_OutOfRange => Some(TrackingResult::Calibrating_OutOfRange),
            TrackingResult_Running_OK => Some(TrackingResult::Running_OK),
            TrackingResult_Running_OutOfRange => Some(TrackingResult::Running_OutOfRange),
            TrackingResult_Fallback_RotationOnly => Some(TrackingResult::Fallback_RotationOnly),
            _ => None,
        }
    }
}

impl From<RawTrackingResult> for TrackingResult {
    fn from(val: RawTrackingResult) -> Self {
        TrackingResult::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackingResult.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackedDeviceClass(pub u32);

pub const TrackedDeviceClass_Invalid: RawTrackedDeviceClass = RawTrackedDeviceClass(0);
pub const TrackedDeviceClass_HMD: RawTrackedDeviceClass = RawTrackedDeviceClass(1);
pub const TrackedDeviceClass_Controller: RawTrackedDeviceClass = RawTrackedDeviceClass(2);
pub const TrackedDeviceClass_GenericTracker: RawTrackedDeviceClass = RawTrackedDeviceClass(3);
pub const TrackedDeviceClass_TrackingReference: RawTrackedDeviceClass = RawTrackedDeviceClass(4);
pub const TrackedDeviceClass_DisplayRedirect: RawTrackedDeviceClass = RawTrackedDeviceClass(5);
pub const TrackedDeviceClass_Max: RawTrackedDeviceClass = RawTrackedDeviceClass(6);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedDeviceClass {
    Invalid = 0,
    HMD = 1,
    Controller = 2,
    GenericTracker = 3,
    TrackingReference = 4,
    DisplayRedirect = 5,
    Max = 6,
}

impl TrackedDeviceClass {
    #[inline]
    fn from_raw(val: RawTrackedDeviceClass) -> Option<Self> {
        match val {
            TrackedDeviceClass_Invalid => Some(TrackedDeviceClass::Invalid),
            TrackedDeviceClass_HMD => Some(TrackedDeviceClass::HMD),
            TrackedDeviceClass_Controller => Some(TrackedDeviceClass::Controller),
            TrackedDeviceClass_GenericTracker => Some(TrackedDeviceClass::GenericTracker),
            TrackedDeviceClass_TrackingReference => Some(TrackedDeviceClass::TrackingReference),
            TrackedDeviceClass_DisplayRedirect => Some(TrackedDeviceClass::DisplayRedirect),
            TrackedDeviceClass_Max => Some(TrackedDeviceClass::Max),
            _ => None,
        }
    }
}

impl From<RawTrackedDeviceClass> for TrackedDeviceClass {
    fn from(val: RawTrackedDeviceClass) -> Self {
        TrackedDeviceClass::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackedDeviceClass.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackedControllerRole(pub u32);

pub const TrackedControllerRole_Invalid: RawTrackedControllerRole = RawTrackedControllerRole(0);
pub const TrackedControllerRole_LeftHand: RawTrackedControllerRole = RawTrackedControllerRole(1);
pub const TrackedControllerRole_RightHand: RawTrackedControllerRole = RawTrackedControllerRole(2);
pub const TrackedControllerRole_OptOut: RawTrackedControllerRole = RawTrackedControllerRole(3);
pub const TrackedControllerRole_Treadmill: RawTrackedControllerRole = RawTrackedControllerRole(4);
pub const TrackedControllerRole_Max: RawTrackedControllerRole = RawTrackedControllerRole(4);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedControllerRole {
    Invalid = 0,
    LeftHand = 1,
    RightHand = 2,
    OptOut = 3,
    Treadmill = 4,
    Max = 4,
}

impl TrackedControllerRole {
    #[inline]
    fn from_raw(val: RawTrackedControllerRole) -> Option<Self> {
        match val {
            TrackedControllerRole_Invalid => Some(TrackedControllerRole::Invalid),
            TrackedControllerRole_LeftHand => Some(TrackedControllerRole::LeftHand),
            TrackedControllerRole_RightHand => Some(TrackedControllerRole::RightHand),
            TrackedControllerRole_OptOut => Some(TrackedControllerRole::OptOut),
            TrackedControllerRole_Treadmill => Some(TrackedControllerRole::Treadmill),
            TrackedControllerRole_Max => Some(TrackedControllerRole::Max),
            _ => None,
        }
    }
}

impl From<RawTrackedControllerRole> for TrackedControllerRole {
    fn from(val: RawTrackedControllerRole) -> Self {
        TrackedControllerRole::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackedControllerRole.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackingUniverseOrigin(pub u32);

pub const TrackingUniverseOrigin_TrackingUniverseSeated: RawTrackingUniverseOrigin =
    RawTrackingUniverseOrigin(0);
pub const TrackingUniverseOrigin_TrackingUniverseStanding: RawTrackingUniverseOrigin =
    RawTrackingUniverseOrigin(1);
pub const TrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated: RawTrackingUniverseOrigin =
    RawTrackingUniverseOrigin(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackingUniverseOrigin {
    TrackingUniverseSeated = 0,
    TrackingUniverseStanding = 1,
    TrackingUniverseRawAndUncalibrated = 2,
}

impl TrackingUniverseOrigin {
    #[inline]
    fn from_raw(val: RawTrackingUniverseOrigin) -> Option<Self> {
        match val {
            TrackingUniverseOrigin_TrackingUniverseSeated => {
                Some(TrackingUniverseOrigin::TrackingUniverseSeated)
            }
            TrackingUniverseOrigin_TrackingUniverseStanding => {
                Some(TrackingUniverseOrigin::TrackingUniverseStanding)
            }
            TrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated => {
                Some(TrackingUniverseOrigin::TrackingUniverseRawAndUncalibrated)
            }
            _ => None,
        }
    }
}

impl From<RawTrackingUniverseOrigin> for TrackingUniverseOrigin {
    fn from(val: RawTrackingUniverseOrigin) -> Self {
        TrackingUniverseOrigin::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackingUniverseOrigin.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackedDeviceProperty(pub u32);

pub const TrackedDeviceProperty_Invalid: RawTrackedDeviceProperty = RawTrackedDeviceProperty(0);
pub const TrackedDeviceProperty_TrackingSystemName_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1000);
pub const TrackedDeviceProperty_ModelNumber_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1001);
pub const TrackedDeviceProperty_SerialNumber_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1002);
pub const TrackedDeviceProperty_RenderModelName_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1003);
pub const TrackedDeviceProperty_WillDriftInYaw_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1004);
pub const TrackedDeviceProperty_ManufacturerName_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1005);
pub const TrackedDeviceProperty_TrackingFirmwareVersion_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1006);
pub const TrackedDeviceProperty_HardwareRevision_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1007);
pub const TrackedDeviceProperty_AllWirelessDongleDescriptions_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1008);
pub const TrackedDeviceProperty_ConnectedWirelessDongle_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1009);
pub const TrackedDeviceProperty_DeviceIsWireless_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1010);
pub const TrackedDeviceProperty_DeviceIsCharging_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1011);
pub const TrackedDeviceProperty_DeviceBatteryPercentage_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1012);
pub const TrackedDeviceProperty_StatusDisplayTransform_Matrix34: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1013);
pub const TrackedDeviceProperty_Firmware_UpdateAvailable_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1014);
pub const TrackedDeviceProperty_Firmware_ManualUpdate_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1015);
pub const TrackedDeviceProperty_Firmware_ManualUpdateURL_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1016);
pub const TrackedDeviceProperty_HardwareRevision_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1017);
pub const TrackedDeviceProperty_FirmwareVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1018);
pub const TrackedDeviceProperty_FPGAVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1019);
pub const TrackedDeviceProperty_VRCVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1020);
pub const TrackedDeviceProperty_RadioVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1021);
pub const TrackedDeviceProperty_DongleVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1022);
pub const TrackedDeviceProperty_BlockServerShutdown_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1023);
pub const TrackedDeviceProperty_CanUnifyCoordinateSystemWithHmd_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1024);
pub const TrackedDeviceProperty_ContainsProximitySensor_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1025);
pub const TrackedDeviceProperty_DeviceProvidesBatteryStatus_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1026);
pub const TrackedDeviceProperty_DeviceCanPowerOff_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1027);
pub const TrackedDeviceProperty_Firmware_ProgrammingTarget_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1028);
pub const TrackedDeviceProperty_DeviceClass_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1029);
pub const TrackedDeviceProperty_HasCamera_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1030);
pub const TrackedDeviceProperty_DriverVersion_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1031);
pub const TrackedDeviceProperty_Firmware_ForceUpdateRequired_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1032);
pub const TrackedDeviceProperty_ViveSystemButtonFixRequired_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1033);
pub const TrackedDeviceProperty_ParentDriver_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1034);
pub const TrackedDeviceProperty_ResourceRoot_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1035);
pub const TrackedDeviceProperty_RegisteredDeviceType_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1036);
pub const TrackedDeviceProperty_InputProfilePath_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1037);
pub const TrackedDeviceProperty_NeverTracked_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1038);
pub const TrackedDeviceProperty_NumCameras_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1039);
pub const TrackedDeviceProperty_CameraFrameLayout_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1040);
pub const TrackedDeviceProperty_CameraStreamFormat_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1041);
pub const TrackedDeviceProperty_AdditionalDeviceSettingsPath_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1042);
pub const TrackedDeviceProperty_Identifiable_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1043);
pub const TrackedDeviceProperty_ReportsTimeSinceVSync_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2000);
pub const TrackedDeviceProperty_SecondsFromVsyncToPhotons_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2001);
pub const TrackedDeviceProperty_DisplayFrequency_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2002);
pub const TrackedDeviceProperty_UserIpdMeters_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2003);
pub const TrackedDeviceProperty_CurrentUniverseId_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2004);
pub const TrackedDeviceProperty_PreviousUniverseId_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2005);
pub const TrackedDeviceProperty_DisplayFirmwareVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2006);
pub const TrackedDeviceProperty_IsOnDesktop_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2007);
pub const TrackedDeviceProperty_DisplayMCType_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2008);
pub const TrackedDeviceProperty_DisplayMCOffset_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2009);
pub const TrackedDeviceProperty_DisplayMCScale_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2010);
pub const TrackedDeviceProperty_EdidVendorID_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2011);
pub const TrackedDeviceProperty_DisplayMCImageLeft_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2012);
pub const TrackedDeviceProperty_DisplayMCImageRight_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2013);
pub const TrackedDeviceProperty_DisplayGCBlackClamp_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2014);
pub const TrackedDeviceProperty_EdidProductID_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2015);
pub const TrackedDeviceProperty_CameraToHeadTransform_Matrix34: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2016);
pub const TrackedDeviceProperty_DisplayGCType_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2017);
pub const TrackedDeviceProperty_DisplayGCOffset_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2018);
pub const TrackedDeviceProperty_DisplayGCScale_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2019);
pub const TrackedDeviceProperty_DisplayGCPrescale_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2020);
pub const TrackedDeviceProperty_DisplayGCImage_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2021);
pub const TrackedDeviceProperty_LensCenterLeftU_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2022);
pub const TrackedDeviceProperty_LensCenterLeftV_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2023);
pub const TrackedDeviceProperty_LensCenterRightU_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2024);
pub const TrackedDeviceProperty_LensCenterRightV_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2025);
pub const TrackedDeviceProperty_UserHeadToEyeDepthMeters_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2026);
pub const TrackedDeviceProperty_CameraFirmwareVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2027);
pub const TrackedDeviceProperty_CameraFirmwareDescription_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2028);
pub const TrackedDeviceProperty_DisplayFPGAVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2029);
pub const TrackedDeviceProperty_DisplayBootloaderVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2030);
pub const TrackedDeviceProperty_DisplayHardwareVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2031);
pub const TrackedDeviceProperty_AudioFirmwareVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2032);
pub const TrackedDeviceProperty_CameraCompatibilityMode_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2033);
pub const TrackedDeviceProperty_ScreenshotHorizontalFieldOfViewDegrees_Float:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2034);
pub const TrackedDeviceProperty_ScreenshotVerticalFieldOfViewDegrees_Float:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2035);
pub const TrackedDeviceProperty_DisplaySuppressed_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2036);
pub const TrackedDeviceProperty_DisplayAllowNightMode_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2037);
pub const TrackedDeviceProperty_DisplayMCImageWidth_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2038);
pub const TrackedDeviceProperty_DisplayMCImageHeight_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2039);
pub const TrackedDeviceProperty_DisplayMCImageNumChannels_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2040);
pub const TrackedDeviceProperty_DisplayMCImageData_Binary: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2041);
pub const TrackedDeviceProperty_SecondsFromPhotonsToVblank_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2042);
pub const TrackedDeviceProperty_DriverDirectModeSendsVsyncEvents_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2043);
pub const TrackedDeviceProperty_DisplayDebugMode_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2044);
pub const TrackedDeviceProperty_GraphicsAdapterLuid_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2045);
pub const TrackedDeviceProperty_DriverProvidedChaperonePath_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2048);
pub const TrackedDeviceProperty_ExpectedTrackingReferenceCount_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2049);
pub const TrackedDeviceProperty_ExpectedControllerCount_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2050);
pub const TrackedDeviceProperty_NamedIconPathControllerLeftDeviceOff_String:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2051);
pub const TrackedDeviceProperty_NamedIconPathControllerRightDeviceOff_String:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2052);
pub const TrackedDeviceProperty_NamedIconPathTrackingReferenceDeviceOff_String:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2053);
pub const TrackedDeviceProperty_DoNotApplyPrediction_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2054);
pub const TrackedDeviceProperty_CameraToHeadTransforms_Matrix34_Array: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2055);
pub const TrackedDeviceProperty_DistortionMeshResolution_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2056);
pub const TrackedDeviceProperty_DriverIsDrawingControllers_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2057);
pub const TrackedDeviceProperty_DriverRequestsApplicationPause_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2058);
pub const TrackedDeviceProperty_DriverRequestsReducedRendering_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2059);
pub const TrackedDeviceProperty_MinimumIpdStepMeters_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2060);
pub const TrackedDeviceProperty_AudioBridgeFirmwareVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2061);
pub const TrackedDeviceProperty_ImageBridgeFirmwareVersion_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2062);
pub const TrackedDeviceProperty_ImuToHeadTransform_Matrix34: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2063);
pub const TrackedDeviceProperty_ImuFactoryGyroBias_Vector3: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2064);
pub const TrackedDeviceProperty_ImuFactoryGyroScale_Vector3: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2065);
pub const TrackedDeviceProperty_ImuFactoryAccelerometerBias_Vector3: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2066);
pub const TrackedDeviceProperty_ImuFactoryAccelerometerScale_Vector3: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2067);
pub const TrackedDeviceProperty_ConfigurationIncludesLighthouse20Features_Bool:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2069);
pub const TrackedDeviceProperty_DriverRequestedMuraCorrectionMode_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(2200);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_InnerLeft_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2201);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_InnerRight_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2202);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_InnerTop_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2203);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_InnerBottom_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2204);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_OuterLeft_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2205);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_OuterRight_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2206);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_OuterTop_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2207);
pub const TrackedDeviceProperty_DriverRequestedMuraFeather_OuterBottom_Int32:
    RawTrackedDeviceProperty = RawTrackedDeviceProperty(2208);
pub const TrackedDeviceProperty_AttachedDeviceId_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3000);
pub const TrackedDeviceProperty_SupportedButtons_Uint64: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3001);
pub const TrackedDeviceProperty_Axis0Type_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3002);
pub const TrackedDeviceProperty_Axis1Type_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3003);
pub const TrackedDeviceProperty_Axis2Type_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3004);
pub const TrackedDeviceProperty_Axis3Type_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3005);
pub const TrackedDeviceProperty_Axis4Type_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3006);
pub const TrackedDeviceProperty_ControllerRoleHint_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(3007);
pub const TrackedDeviceProperty_FieldOfViewLeftDegrees_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(4000);
pub const TrackedDeviceProperty_FieldOfViewRightDegrees_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(4001);
pub const TrackedDeviceProperty_FieldOfViewTopDegrees_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(4002);
pub const TrackedDeviceProperty_FieldOfViewBottomDegrees_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(4003);
pub const TrackedDeviceProperty_TrackingRangeMinimumMeters_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(4004);
pub const TrackedDeviceProperty_TrackingRangeMaximumMeters_Float: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(4005);
pub const TrackedDeviceProperty_ModeLabel_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(4006);
pub const TrackedDeviceProperty_IconPathName_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5000);
pub const TrackedDeviceProperty_NamedIconPathDeviceOff_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5001);
pub const TrackedDeviceProperty_NamedIconPathDeviceSearching_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5002);
pub const TrackedDeviceProperty_NamedIconPathDeviceSearchingAlert_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5003);
pub const TrackedDeviceProperty_NamedIconPathDeviceReady_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5004);
pub const TrackedDeviceProperty_NamedIconPathDeviceReadyAlert_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5005);
pub const TrackedDeviceProperty_NamedIconPathDeviceNotReady_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5006);
pub const TrackedDeviceProperty_NamedIconPathDeviceStandby_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5007);
pub const TrackedDeviceProperty_NamedIconPathDeviceAlertLow_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5008);
pub const TrackedDeviceProperty_DisplayHiddenArea_Binary_Start: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5100);
pub const TrackedDeviceProperty_DisplayHiddenArea_Binary_End: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5150);
pub const TrackedDeviceProperty_ParentContainer: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(5151);
pub const TrackedDeviceProperty_UserConfigPath_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6000);
pub const TrackedDeviceProperty_InstallPath_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6001);
pub const TrackedDeviceProperty_HasDisplayComponent_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6002);
pub const TrackedDeviceProperty_HasControllerComponent_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6003);
pub const TrackedDeviceProperty_HasCameraComponent_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6004);
pub const TrackedDeviceProperty_HasDriverDirectModeComponent_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6005);
pub const TrackedDeviceProperty_HasVirtualDisplayComponent_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6006);
pub const TrackedDeviceProperty_HasSpatialAnchorsSupport_Bool: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(6007);
pub const TrackedDeviceProperty_ControllerType_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(7000);
pub const TrackedDeviceProperty_LegacyInputProfile_String: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(7001);
pub const TrackedDeviceProperty_ControllerHandSelectionPriority_Int32: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(7002);
pub const TrackedDeviceProperty_VendorSpecific_Reserved_Start: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(10000);
pub const TrackedDeviceProperty_VendorSpecific_Reserved_End: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(10999);
pub const TrackedDeviceProperty_TrackedDeviceProperty_Max: RawTrackedDeviceProperty =
    RawTrackedDeviceProperty(1000000);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedDeviceProperty {
    Invalid = 0,
    TrackingSystemName_String = 1000,
    ModelNumber_String = 1001,
    SerialNumber_String = 1002,
    RenderModelName_String = 1003,
    WillDriftInYaw_Bool = 1004,
    ManufacturerName_String = 1005,
    TrackingFirmwareVersion_String = 1006,
    HardwareRevision_String = 1007,
    AllWirelessDongleDescriptions_String = 1008,
    ConnectedWirelessDongle_String = 1009,
    DeviceIsWireless_Bool = 1010,
    DeviceIsCharging_Bool = 1011,
    DeviceBatteryPercentage_Float = 1012,
    StatusDisplayTransform_Matrix34 = 1013,
    Firmware_UpdateAvailable_Bool = 1014,
    Firmware_ManualUpdate_Bool = 1015,
    Firmware_ManualUpdateURL_String = 1016,
    HardwareRevision_Uint64 = 1017,
    FirmwareVersion_Uint64 = 1018,
    FPGAVersion_Uint64 = 1019,
    VRCVersion_Uint64 = 1020,
    RadioVersion_Uint64 = 1021,
    DongleVersion_Uint64 = 1022,
    BlockServerShutdown_Bool = 1023,
    CanUnifyCoordinateSystemWithHmd_Bool = 1024,
    ContainsProximitySensor_Bool = 1025,
    DeviceProvidesBatteryStatus_Bool = 1026,
    DeviceCanPowerOff_Bool = 1027,
    Firmware_ProgrammingTarget_String = 1028,
    DeviceClass_Int32 = 1029,
    HasCamera_Bool = 1030,
    DriverVersion_String = 1031,
    Firmware_ForceUpdateRequired_Bool = 1032,
    ViveSystemButtonFixRequired_Bool = 1033,
    ParentDriver_Uint64 = 1034,
    ResourceRoot_String = 1035,
    RegisteredDeviceType_String = 1036,
    InputProfilePath_String = 1037,
    NeverTracked_Bool = 1038,
    NumCameras_Int32 = 1039,
    CameraFrameLayout_Int32 = 1040,
    CameraStreamFormat_Int32 = 1041,
    AdditionalDeviceSettingsPath_String = 1042,
    Identifiable_Bool = 1043,
    ReportsTimeSinceVSync_Bool = 2000,
    SecondsFromVsyncToPhotons_Float = 2001,
    DisplayFrequency_Float = 2002,
    UserIpdMeters_Float = 2003,
    CurrentUniverseId_Uint64 = 2004,
    PreviousUniverseId_Uint64 = 2005,
    DisplayFirmwareVersion_Uint64 = 2006,
    IsOnDesktop_Bool = 2007,
    DisplayMCType_Int32 = 2008,
    DisplayMCOffset_Float = 2009,
    DisplayMCScale_Float = 2010,
    EdidVendorID_Int32 = 2011,
    DisplayMCImageLeft_String = 2012,
    DisplayMCImageRight_String = 2013,
    DisplayGCBlackClamp_Float = 2014,
    EdidProductID_Int32 = 2015,
    CameraToHeadTransform_Matrix34 = 2016,
    DisplayGCType_Int32 = 2017,
    DisplayGCOffset_Float = 2018,
    DisplayGCScale_Float = 2019,
    DisplayGCPrescale_Float = 2020,
    DisplayGCImage_String = 2021,
    LensCenterLeftU_Float = 2022,
    LensCenterLeftV_Float = 2023,
    LensCenterRightU_Float = 2024,
    LensCenterRightV_Float = 2025,
    UserHeadToEyeDepthMeters_Float = 2026,
    CameraFirmwareVersion_Uint64 = 2027,
    CameraFirmwareDescription_String = 2028,
    DisplayFPGAVersion_Uint64 = 2029,
    DisplayBootloaderVersion_Uint64 = 2030,
    DisplayHardwareVersion_Uint64 = 2031,
    AudioFirmwareVersion_Uint64 = 2032,
    CameraCompatibilityMode_Int32 = 2033,
    ScreenshotHorizontalFieldOfViewDegrees_Float = 2034,
    ScreenshotVerticalFieldOfViewDegrees_Float = 2035,
    DisplaySuppressed_Bool = 2036,
    DisplayAllowNightMode_Bool = 2037,
    DisplayMCImageWidth_Int32 = 2038,
    DisplayMCImageHeight_Int32 = 2039,
    DisplayMCImageNumChannels_Int32 = 2040,
    DisplayMCImageData_Binary = 2041,
    SecondsFromPhotonsToVblank_Float = 2042,
    DriverDirectModeSendsVsyncEvents_Bool = 2043,
    DisplayDebugMode_Bool = 2044,
    GraphicsAdapterLuid_Uint64 = 2045,
    DriverProvidedChaperonePath_String = 2048,
    ExpectedTrackingReferenceCount_Int32 = 2049,
    ExpectedControllerCount_Int32 = 2050,
    NamedIconPathControllerLeftDeviceOff_String = 2051,
    NamedIconPathControllerRightDeviceOff_String = 2052,
    NamedIconPathTrackingReferenceDeviceOff_String = 2053,
    DoNotApplyPrediction_Bool = 2054,
    CameraToHeadTransforms_Matrix34_Array = 2055,
    DistortionMeshResolution_Int32 = 2056,
    DriverIsDrawingControllers_Bool = 2057,
    DriverRequestsApplicationPause_Bool = 2058,
    DriverRequestsReducedRendering_Bool = 2059,
    MinimumIpdStepMeters_Float = 2060,
    AudioBridgeFirmwareVersion_Uint64 = 2061,
    ImageBridgeFirmwareVersion_Uint64 = 2062,
    ImuToHeadTransform_Matrix34 = 2063,
    ImuFactoryGyroBias_Vector3 = 2064,
    ImuFactoryGyroScale_Vector3 = 2065,
    ImuFactoryAccelerometerBias_Vector3 = 2066,
    ImuFactoryAccelerometerScale_Vector3 = 2067,
    ConfigurationIncludesLighthouse20Features_Bool = 2069,
    DriverRequestedMuraCorrectionMode_Int32 = 2200,
    DriverRequestedMuraFeather_InnerLeft_Int32 = 2201,
    DriverRequestedMuraFeather_InnerRight_Int32 = 2202,
    DriverRequestedMuraFeather_InnerTop_Int32 = 2203,
    DriverRequestedMuraFeather_InnerBottom_Int32 = 2204,
    DriverRequestedMuraFeather_OuterLeft_Int32 = 2205,
    DriverRequestedMuraFeather_OuterRight_Int32 = 2206,
    DriverRequestedMuraFeather_OuterTop_Int32 = 2207,
    DriverRequestedMuraFeather_OuterBottom_Int32 = 2208,
    AttachedDeviceId_String = 3000,
    SupportedButtons_Uint64 = 3001,
    Axis0Type_Int32 = 3002,
    Axis1Type_Int32 = 3003,
    Axis2Type_Int32 = 3004,
    Axis3Type_Int32 = 3005,
    Axis4Type_Int32 = 3006,
    ControllerRoleHint_Int32 = 3007,
    FieldOfViewLeftDegrees_Float = 4000,
    FieldOfViewRightDegrees_Float = 4001,
    FieldOfViewTopDegrees_Float = 4002,
    FieldOfViewBottomDegrees_Float = 4003,
    TrackingRangeMinimumMeters_Float = 4004,
    TrackingRangeMaximumMeters_Float = 4005,
    ModeLabel_String = 4006,
    IconPathName_String = 5000,
    NamedIconPathDeviceOff_String = 5001,
    NamedIconPathDeviceSearching_String = 5002,
    NamedIconPathDeviceSearchingAlert_String = 5003,
    NamedIconPathDeviceReady_String = 5004,
    NamedIconPathDeviceReadyAlert_String = 5005,
    NamedIconPathDeviceNotReady_String = 5006,
    NamedIconPathDeviceStandby_String = 5007,
    NamedIconPathDeviceAlertLow_String = 5008,
    DisplayHiddenArea_Binary_Start = 5100,
    DisplayHiddenArea_Binary_End = 5150,
    ParentContainer = 5151,
    UserConfigPath_String = 6000,
    InstallPath_String = 6001,
    HasDisplayComponent_Bool = 6002,
    HasControllerComponent_Bool = 6003,
    HasCameraComponent_Bool = 6004,
    HasDriverDirectModeComponent_Bool = 6005,
    HasVirtualDisplayComponent_Bool = 6006,
    HasSpatialAnchorsSupport_Bool = 6007,
    ControllerType_String = 7000,
    LegacyInputProfile_String = 7001,
    ControllerHandSelectionPriority_Int32 = 7002,
    VendorSpecific_Reserved_Start = 10000,
    VendorSpecific_Reserved_End = 10999,
    TrackedDeviceProperty_Max = 1000000,
}

impl TrackedDeviceProperty {
    #[inline]
    fn from_raw(val: RawTrackedDeviceProperty) -> Option<Self> {
        match val {
            TrackedDeviceProperty_Invalid => Some(TrackedDeviceProperty::Invalid),
            TrackedDeviceProperty_TrackingSystemName_String => {
                Some(TrackedDeviceProperty::TrackingSystemName_String)
            }
            TrackedDeviceProperty_ModelNumber_String => {
                Some(TrackedDeviceProperty::ModelNumber_String)
            }
            TrackedDeviceProperty_SerialNumber_String => {
                Some(TrackedDeviceProperty::SerialNumber_String)
            }
            TrackedDeviceProperty_RenderModelName_String => {
                Some(TrackedDeviceProperty::RenderModelName_String)
            }
            TrackedDeviceProperty_WillDriftInYaw_Bool => {
                Some(TrackedDeviceProperty::WillDriftInYaw_Bool)
            }
            TrackedDeviceProperty_ManufacturerName_String => {
                Some(TrackedDeviceProperty::ManufacturerName_String)
            }
            TrackedDeviceProperty_TrackingFirmwareVersion_String => {
                Some(TrackedDeviceProperty::TrackingFirmwareVersion_String)
            }
            TrackedDeviceProperty_HardwareRevision_String => {
                Some(TrackedDeviceProperty::HardwareRevision_String)
            }
            TrackedDeviceProperty_AllWirelessDongleDescriptions_String => {
                Some(TrackedDeviceProperty::AllWirelessDongleDescriptions_String)
            }
            TrackedDeviceProperty_ConnectedWirelessDongle_String => {
                Some(TrackedDeviceProperty::ConnectedWirelessDongle_String)
            }
            TrackedDeviceProperty_DeviceIsWireless_Bool => {
                Some(TrackedDeviceProperty::DeviceIsWireless_Bool)
            }
            TrackedDeviceProperty_DeviceIsCharging_Bool => {
                Some(TrackedDeviceProperty::DeviceIsCharging_Bool)
            }
            TrackedDeviceProperty_DeviceBatteryPercentage_Float => {
                Some(TrackedDeviceProperty::DeviceBatteryPercentage_Float)
            }
            TrackedDeviceProperty_StatusDisplayTransform_Matrix34 => {
                Some(TrackedDeviceProperty::StatusDisplayTransform_Matrix34)
            }
            TrackedDeviceProperty_Firmware_UpdateAvailable_Bool => {
                Some(TrackedDeviceProperty::Firmware_UpdateAvailable_Bool)
            }
            TrackedDeviceProperty_Firmware_ManualUpdate_Bool => {
                Some(TrackedDeviceProperty::Firmware_ManualUpdate_Bool)
            }
            TrackedDeviceProperty_Firmware_ManualUpdateURL_String => {
                Some(TrackedDeviceProperty::Firmware_ManualUpdateURL_String)
            }
            TrackedDeviceProperty_HardwareRevision_Uint64 => {
                Some(TrackedDeviceProperty::HardwareRevision_Uint64)
            }
            TrackedDeviceProperty_FirmwareVersion_Uint64 => {
                Some(TrackedDeviceProperty::FirmwareVersion_Uint64)
            }
            TrackedDeviceProperty_FPGAVersion_Uint64 => {
                Some(TrackedDeviceProperty::FPGAVersion_Uint64)
            }
            TrackedDeviceProperty_VRCVersion_Uint64 => {
                Some(TrackedDeviceProperty::VRCVersion_Uint64)
            }
            TrackedDeviceProperty_RadioVersion_Uint64 => {
                Some(TrackedDeviceProperty::RadioVersion_Uint64)
            }
            TrackedDeviceProperty_DongleVersion_Uint64 => {
                Some(TrackedDeviceProperty::DongleVersion_Uint64)
            }
            TrackedDeviceProperty_BlockServerShutdown_Bool => {
                Some(TrackedDeviceProperty::BlockServerShutdown_Bool)
            }
            TrackedDeviceProperty_CanUnifyCoordinateSystemWithHmd_Bool => {
                Some(TrackedDeviceProperty::CanUnifyCoordinateSystemWithHmd_Bool)
            }
            TrackedDeviceProperty_ContainsProximitySensor_Bool => {
                Some(TrackedDeviceProperty::ContainsProximitySensor_Bool)
            }
            TrackedDeviceProperty_DeviceProvidesBatteryStatus_Bool => {
                Some(TrackedDeviceProperty::DeviceProvidesBatteryStatus_Bool)
            }
            TrackedDeviceProperty_DeviceCanPowerOff_Bool => {
                Some(TrackedDeviceProperty::DeviceCanPowerOff_Bool)
            }
            TrackedDeviceProperty_Firmware_ProgrammingTarget_String => {
                Some(TrackedDeviceProperty::Firmware_ProgrammingTarget_String)
            }
            TrackedDeviceProperty_DeviceClass_Int32 => {
                Some(TrackedDeviceProperty::DeviceClass_Int32)
            }
            TrackedDeviceProperty_HasCamera_Bool => Some(TrackedDeviceProperty::HasCamera_Bool),
            TrackedDeviceProperty_DriverVersion_String => {
                Some(TrackedDeviceProperty::DriverVersion_String)
            }
            TrackedDeviceProperty_Firmware_ForceUpdateRequired_Bool => {
                Some(TrackedDeviceProperty::Firmware_ForceUpdateRequired_Bool)
            }
            TrackedDeviceProperty_ViveSystemButtonFixRequired_Bool => {
                Some(TrackedDeviceProperty::ViveSystemButtonFixRequired_Bool)
            }
            TrackedDeviceProperty_ParentDriver_Uint64 => {
                Some(TrackedDeviceProperty::ParentDriver_Uint64)
            }
            TrackedDeviceProperty_ResourceRoot_String => {
                Some(TrackedDeviceProperty::ResourceRoot_String)
            }
            TrackedDeviceProperty_RegisteredDeviceType_String => {
                Some(TrackedDeviceProperty::RegisteredDeviceType_String)
            }
            TrackedDeviceProperty_InputProfilePath_String => {
                Some(TrackedDeviceProperty::InputProfilePath_String)
            }
            TrackedDeviceProperty_NeverTracked_Bool => {
                Some(TrackedDeviceProperty::NeverTracked_Bool)
            }
            TrackedDeviceProperty_NumCameras_Int32 => Some(TrackedDeviceProperty::NumCameras_Int32),
            TrackedDeviceProperty_CameraFrameLayout_Int32 => {
                Some(TrackedDeviceProperty::CameraFrameLayout_Int32)
            }
            TrackedDeviceProperty_CameraStreamFormat_Int32 => {
                Some(TrackedDeviceProperty::CameraStreamFormat_Int32)
            }
            TrackedDeviceProperty_AdditionalDeviceSettingsPath_String => {
                Some(TrackedDeviceProperty::AdditionalDeviceSettingsPath_String)
            }
            TrackedDeviceProperty_Identifiable_Bool => {
                Some(TrackedDeviceProperty::Identifiable_Bool)
            }
            TrackedDeviceProperty_ReportsTimeSinceVSync_Bool => {
                Some(TrackedDeviceProperty::ReportsTimeSinceVSync_Bool)
            }
            TrackedDeviceProperty_SecondsFromVsyncToPhotons_Float => {
                Some(TrackedDeviceProperty::SecondsFromVsyncToPhotons_Float)
            }
            TrackedDeviceProperty_DisplayFrequency_Float => {
                Some(TrackedDeviceProperty::DisplayFrequency_Float)
            }
            TrackedDeviceProperty_UserIpdMeters_Float => {
                Some(TrackedDeviceProperty::UserIpdMeters_Float)
            }
            TrackedDeviceProperty_CurrentUniverseId_Uint64 => {
                Some(TrackedDeviceProperty::CurrentUniverseId_Uint64)
            }
            TrackedDeviceProperty_PreviousUniverseId_Uint64 => {
                Some(TrackedDeviceProperty::PreviousUniverseId_Uint64)
            }
            TrackedDeviceProperty_DisplayFirmwareVersion_Uint64 => {
                Some(TrackedDeviceProperty::DisplayFirmwareVersion_Uint64)
            }
            TrackedDeviceProperty_IsOnDesktop_Bool => Some(TrackedDeviceProperty::IsOnDesktop_Bool),
            TrackedDeviceProperty_DisplayMCType_Int32 => {
                Some(TrackedDeviceProperty::DisplayMCType_Int32)
            }
            TrackedDeviceProperty_DisplayMCOffset_Float => {
                Some(TrackedDeviceProperty::DisplayMCOffset_Float)
            }
            TrackedDeviceProperty_DisplayMCScale_Float => {
                Some(TrackedDeviceProperty::DisplayMCScale_Float)
            }
            TrackedDeviceProperty_EdidVendorID_Int32 => {
                Some(TrackedDeviceProperty::EdidVendorID_Int32)
            }
            TrackedDeviceProperty_DisplayMCImageLeft_String => {
                Some(TrackedDeviceProperty::DisplayMCImageLeft_String)
            }
            TrackedDeviceProperty_DisplayMCImageRight_String => {
                Some(TrackedDeviceProperty::DisplayMCImageRight_String)
            }
            TrackedDeviceProperty_DisplayGCBlackClamp_Float => {
                Some(TrackedDeviceProperty::DisplayGCBlackClamp_Float)
            }
            TrackedDeviceProperty_EdidProductID_Int32 => {
                Some(TrackedDeviceProperty::EdidProductID_Int32)
            }
            TrackedDeviceProperty_CameraToHeadTransform_Matrix34 => {
                Some(TrackedDeviceProperty::CameraToHeadTransform_Matrix34)
            }
            TrackedDeviceProperty_DisplayGCType_Int32 => {
                Some(TrackedDeviceProperty::DisplayGCType_Int32)
            }
            TrackedDeviceProperty_DisplayGCOffset_Float => {
                Some(TrackedDeviceProperty::DisplayGCOffset_Float)
            }
            TrackedDeviceProperty_DisplayGCScale_Float => {
                Some(TrackedDeviceProperty::DisplayGCScale_Float)
            }
            TrackedDeviceProperty_DisplayGCPrescale_Float => {
                Some(TrackedDeviceProperty::DisplayGCPrescale_Float)
            }
            TrackedDeviceProperty_DisplayGCImage_String => {
                Some(TrackedDeviceProperty::DisplayGCImage_String)
            }
            TrackedDeviceProperty_LensCenterLeftU_Float => {
                Some(TrackedDeviceProperty::LensCenterLeftU_Float)
            }
            TrackedDeviceProperty_LensCenterLeftV_Float => {
                Some(TrackedDeviceProperty::LensCenterLeftV_Float)
            }
            TrackedDeviceProperty_LensCenterRightU_Float => {
                Some(TrackedDeviceProperty::LensCenterRightU_Float)
            }
            TrackedDeviceProperty_LensCenterRightV_Float => {
                Some(TrackedDeviceProperty::LensCenterRightV_Float)
            }
            TrackedDeviceProperty_UserHeadToEyeDepthMeters_Float => {
                Some(TrackedDeviceProperty::UserHeadToEyeDepthMeters_Float)
            }
            TrackedDeviceProperty_CameraFirmwareVersion_Uint64 => {
                Some(TrackedDeviceProperty::CameraFirmwareVersion_Uint64)
            }
            TrackedDeviceProperty_CameraFirmwareDescription_String => {
                Some(TrackedDeviceProperty::CameraFirmwareDescription_String)
            }
            TrackedDeviceProperty_DisplayFPGAVersion_Uint64 => {
                Some(TrackedDeviceProperty::DisplayFPGAVersion_Uint64)
            }
            TrackedDeviceProperty_DisplayBootloaderVersion_Uint64 => {
                Some(TrackedDeviceProperty::DisplayBootloaderVersion_Uint64)
            }
            TrackedDeviceProperty_DisplayHardwareVersion_Uint64 => {
                Some(TrackedDeviceProperty::DisplayHardwareVersion_Uint64)
            }
            TrackedDeviceProperty_AudioFirmwareVersion_Uint64 => {
                Some(TrackedDeviceProperty::AudioFirmwareVersion_Uint64)
            }
            TrackedDeviceProperty_CameraCompatibilityMode_Int32 => {
                Some(TrackedDeviceProperty::CameraCompatibilityMode_Int32)
            }
            TrackedDeviceProperty_ScreenshotHorizontalFieldOfViewDegrees_Float => {
                Some(TrackedDeviceProperty::ScreenshotHorizontalFieldOfViewDegrees_Float)
            }
            TrackedDeviceProperty_ScreenshotVerticalFieldOfViewDegrees_Float => {
                Some(TrackedDeviceProperty::ScreenshotVerticalFieldOfViewDegrees_Float)
            }
            TrackedDeviceProperty_DisplaySuppressed_Bool => {
                Some(TrackedDeviceProperty::DisplaySuppressed_Bool)
            }
            TrackedDeviceProperty_DisplayAllowNightMode_Bool => {
                Some(TrackedDeviceProperty::DisplayAllowNightMode_Bool)
            }
            TrackedDeviceProperty_DisplayMCImageWidth_Int32 => {
                Some(TrackedDeviceProperty::DisplayMCImageWidth_Int32)
            }
            TrackedDeviceProperty_DisplayMCImageHeight_Int32 => {
                Some(TrackedDeviceProperty::DisplayMCImageHeight_Int32)
            }
            TrackedDeviceProperty_DisplayMCImageNumChannels_Int32 => {
                Some(TrackedDeviceProperty::DisplayMCImageNumChannels_Int32)
            }
            TrackedDeviceProperty_DisplayMCImageData_Binary => {
                Some(TrackedDeviceProperty::DisplayMCImageData_Binary)
            }
            TrackedDeviceProperty_SecondsFromPhotonsToVblank_Float => {
                Some(TrackedDeviceProperty::SecondsFromPhotonsToVblank_Float)
            }
            TrackedDeviceProperty_DriverDirectModeSendsVsyncEvents_Bool => {
                Some(TrackedDeviceProperty::DriverDirectModeSendsVsyncEvents_Bool)
            }
            TrackedDeviceProperty_DisplayDebugMode_Bool => {
                Some(TrackedDeviceProperty::DisplayDebugMode_Bool)
            }
            TrackedDeviceProperty_GraphicsAdapterLuid_Uint64 => {
                Some(TrackedDeviceProperty::GraphicsAdapterLuid_Uint64)
            }
            TrackedDeviceProperty_DriverProvidedChaperonePath_String => {
                Some(TrackedDeviceProperty::DriverProvidedChaperonePath_String)
            }
            TrackedDeviceProperty_ExpectedTrackingReferenceCount_Int32 => {
                Some(TrackedDeviceProperty::ExpectedTrackingReferenceCount_Int32)
            }
            TrackedDeviceProperty_ExpectedControllerCount_Int32 => {
                Some(TrackedDeviceProperty::ExpectedControllerCount_Int32)
            }
            TrackedDeviceProperty_NamedIconPathControllerLeftDeviceOff_String => {
                Some(TrackedDeviceProperty::NamedIconPathControllerLeftDeviceOff_String)
            }
            TrackedDeviceProperty_NamedIconPathControllerRightDeviceOff_String => {
                Some(TrackedDeviceProperty::NamedIconPathControllerRightDeviceOff_String)
            }
            TrackedDeviceProperty_NamedIconPathTrackingReferenceDeviceOff_String => {
                Some(TrackedDeviceProperty::NamedIconPathTrackingReferenceDeviceOff_String)
            }
            TrackedDeviceProperty_DoNotApplyPrediction_Bool => {
                Some(TrackedDeviceProperty::DoNotApplyPrediction_Bool)
            }
            TrackedDeviceProperty_CameraToHeadTransforms_Matrix34_Array => {
                Some(TrackedDeviceProperty::CameraToHeadTransforms_Matrix34_Array)
            }
            TrackedDeviceProperty_DistortionMeshResolution_Int32 => {
                Some(TrackedDeviceProperty::DistortionMeshResolution_Int32)
            }
            TrackedDeviceProperty_DriverIsDrawingControllers_Bool => {
                Some(TrackedDeviceProperty::DriverIsDrawingControllers_Bool)
            }
            TrackedDeviceProperty_DriverRequestsApplicationPause_Bool => {
                Some(TrackedDeviceProperty::DriverRequestsApplicationPause_Bool)
            }
            TrackedDeviceProperty_DriverRequestsReducedRendering_Bool => {
                Some(TrackedDeviceProperty::DriverRequestsReducedRendering_Bool)
            }
            TrackedDeviceProperty_MinimumIpdStepMeters_Float => {
                Some(TrackedDeviceProperty::MinimumIpdStepMeters_Float)
            }
            TrackedDeviceProperty_AudioBridgeFirmwareVersion_Uint64 => {
                Some(TrackedDeviceProperty::AudioBridgeFirmwareVersion_Uint64)
            }
            TrackedDeviceProperty_ImageBridgeFirmwareVersion_Uint64 => {
                Some(TrackedDeviceProperty::ImageBridgeFirmwareVersion_Uint64)
            }
            TrackedDeviceProperty_ImuToHeadTransform_Matrix34 => {
                Some(TrackedDeviceProperty::ImuToHeadTransform_Matrix34)
            }
            TrackedDeviceProperty_ImuFactoryGyroBias_Vector3 => {
                Some(TrackedDeviceProperty::ImuFactoryGyroBias_Vector3)
            }
            TrackedDeviceProperty_ImuFactoryGyroScale_Vector3 => {
                Some(TrackedDeviceProperty::ImuFactoryGyroScale_Vector3)
            }
            TrackedDeviceProperty_ImuFactoryAccelerometerBias_Vector3 => {
                Some(TrackedDeviceProperty::ImuFactoryAccelerometerBias_Vector3)
            }
            TrackedDeviceProperty_ImuFactoryAccelerometerScale_Vector3 => {
                Some(TrackedDeviceProperty::ImuFactoryAccelerometerScale_Vector3)
            }
            TrackedDeviceProperty_ConfigurationIncludesLighthouse20Features_Bool => {
                Some(TrackedDeviceProperty::ConfigurationIncludesLighthouse20Features_Bool)
            }
            TrackedDeviceProperty_DriverRequestedMuraCorrectionMode_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraCorrectionMode_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_InnerLeft_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_InnerLeft_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_InnerRight_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_InnerRight_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_InnerTop_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_InnerTop_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_InnerBottom_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_InnerBottom_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_OuterLeft_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_OuterLeft_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_OuterRight_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_OuterRight_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_OuterTop_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_OuterTop_Int32)
            }
            TrackedDeviceProperty_DriverRequestedMuraFeather_OuterBottom_Int32 => {
                Some(TrackedDeviceProperty::DriverRequestedMuraFeather_OuterBottom_Int32)
            }
            TrackedDeviceProperty_AttachedDeviceId_String => {
                Some(TrackedDeviceProperty::AttachedDeviceId_String)
            }
            TrackedDeviceProperty_SupportedButtons_Uint64 => {
                Some(TrackedDeviceProperty::SupportedButtons_Uint64)
            }
            TrackedDeviceProperty_Axis0Type_Int32 => Some(TrackedDeviceProperty::Axis0Type_Int32),
            TrackedDeviceProperty_Axis1Type_Int32 => Some(TrackedDeviceProperty::Axis1Type_Int32),
            TrackedDeviceProperty_Axis2Type_Int32 => Some(TrackedDeviceProperty::Axis2Type_Int32),
            TrackedDeviceProperty_Axis3Type_Int32 => Some(TrackedDeviceProperty::Axis3Type_Int32),
            TrackedDeviceProperty_Axis4Type_Int32 => Some(TrackedDeviceProperty::Axis4Type_Int32),
            TrackedDeviceProperty_ControllerRoleHint_Int32 => {
                Some(TrackedDeviceProperty::ControllerRoleHint_Int32)
            }
            TrackedDeviceProperty_FieldOfViewLeftDegrees_Float => {
                Some(TrackedDeviceProperty::FieldOfViewLeftDegrees_Float)
            }
            TrackedDeviceProperty_FieldOfViewRightDegrees_Float => {
                Some(TrackedDeviceProperty::FieldOfViewRightDegrees_Float)
            }
            TrackedDeviceProperty_FieldOfViewTopDegrees_Float => {
                Some(TrackedDeviceProperty::FieldOfViewTopDegrees_Float)
            }
            TrackedDeviceProperty_FieldOfViewBottomDegrees_Float => {
                Some(TrackedDeviceProperty::FieldOfViewBottomDegrees_Float)
            }
            TrackedDeviceProperty_TrackingRangeMinimumMeters_Float => {
                Some(TrackedDeviceProperty::TrackingRangeMinimumMeters_Float)
            }
            TrackedDeviceProperty_TrackingRangeMaximumMeters_Float => {
                Some(TrackedDeviceProperty::TrackingRangeMaximumMeters_Float)
            }
            TrackedDeviceProperty_ModeLabel_String => Some(TrackedDeviceProperty::ModeLabel_String),
            TrackedDeviceProperty_IconPathName_String => {
                Some(TrackedDeviceProperty::IconPathName_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceOff_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceOff_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceSearching_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceSearching_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceSearchingAlert_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceSearchingAlert_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceReady_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceReady_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceReadyAlert_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceReadyAlert_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceNotReady_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceNotReady_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceStandby_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceStandby_String)
            }
            TrackedDeviceProperty_NamedIconPathDeviceAlertLow_String => {
                Some(TrackedDeviceProperty::NamedIconPathDeviceAlertLow_String)
            }
            TrackedDeviceProperty_DisplayHiddenArea_Binary_Start => {
                Some(TrackedDeviceProperty::DisplayHiddenArea_Binary_Start)
            }
            TrackedDeviceProperty_DisplayHiddenArea_Binary_End => {
                Some(TrackedDeviceProperty::DisplayHiddenArea_Binary_End)
            }
            TrackedDeviceProperty_ParentContainer => Some(TrackedDeviceProperty::ParentContainer),
            TrackedDeviceProperty_UserConfigPath_String => {
                Some(TrackedDeviceProperty::UserConfigPath_String)
            }
            TrackedDeviceProperty_InstallPath_String => {
                Some(TrackedDeviceProperty::InstallPath_String)
            }
            TrackedDeviceProperty_HasDisplayComponent_Bool => {
                Some(TrackedDeviceProperty::HasDisplayComponent_Bool)
            }
            TrackedDeviceProperty_HasControllerComponent_Bool => {
                Some(TrackedDeviceProperty::HasControllerComponent_Bool)
            }
            TrackedDeviceProperty_HasCameraComponent_Bool => {
                Some(TrackedDeviceProperty::HasCameraComponent_Bool)
            }
            TrackedDeviceProperty_HasDriverDirectModeComponent_Bool => {
                Some(TrackedDeviceProperty::HasDriverDirectModeComponent_Bool)
            }
            TrackedDeviceProperty_HasVirtualDisplayComponent_Bool => {
                Some(TrackedDeviceProperty::HasVirtualDisplayComponent_Bool)
            }
            TrackedDeviceProperty_HasSpatialAnchorsSupport_Bool => {
                Some(TrackedDeviceProperty::HasSpatialAnchorsSupport_Bool)
            }
            TrackedDeviceProperty_ControllerType_String => {
                Some(TrackedDeviceProperty::ControllerType_String)
            }
            TrackedDeviceProperty_LegacyInputProfile_String => {
                Some(TrackedDeviceProperty::LegacyInputProfile_String)
            }
            TrackedDeviceProperty_ControllerHandSelectionPriority_Int32 => {
                Some(TrackedDeviceProperty::ControllerHandSelectionPriority_Int32)
            }
            TrackedDeviceProperty_VendorSpecific_Reserved_Start => {
                Some(TrackedDeviceProperty::VendorSpecific_Reserved_Start)
            }
            TrackedDeviceProperty_VendorSpecific_Reserved_End => {
                Some(TrackedDeviceProperty::VendorSpecific_Reserved_End)
            }
            TrackedDeviceProperty_TrackedDeviceProperty_Max => {
                Some(TrackedDeviceProperty::TrackedDeviceProperty_Max)
            }
            _ => None,
        }
    }
}

impl From<RawTrackedDeviceProperty> for TrackedDeviceProperty {
    fn from(val: RawTrackedDeviceProperty) -> Self {
        TrackedDeviceProperty::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackedDeviceProperty.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackedPropertyError(pub u32);

pub const TrackedPropertyError_Success: RawTrackedPropertyError = RawTrackedPropertyError(0);
pub const TrackedPropertyError_WrongDataType: RawTrackedPropertyError = RawTrackedPropertyError(1);
pub const TrackedPropertyError_WrongDeviceClass: RawTrackedPropertyError =
    RawTrackedPropertyError(2);
pub const TrackedPropertyError_BufferTooSmall: RawTrackedPropertyError = RawTrackedPropertyError(3);
pub const TrackedPropertyError_UnknownProperty: RawTrackedPropertyError =
    RawTrackedPropertyError(4);
pub const TrackedPropertyError_InvalidDevice: RawTrackedPropertyError = RawTrackedPropertyError(5);
pub const TrackedPropertyError_CouldNotContactServer: RawTrackedPropertyError =
    RawTrackedPropertyError(6);
pub const TrackedPropertyError_ValueNotProvidedByDevice: RawTrackedPropertyError =
    RawTrackedPropertyError(7);
pub const TrackedPropertyError_StringExceedsMaximumLength: RawTrackedPropertyError =
    RawTrackedPropertyError(8);
pub const TrackedPropertyError_NotYetAvailable: RawTrackedPropertyError =
    RawTrackedPropertyError(9);
pub const TrackedPropertyError_PermissionDenied: RawTrackedPropertyError =
    RawTrackedPropertyError(10);
pub const TrackedPropertyError_InvalidOperation: RawTrackedPropertyError =
    RawTrackedPropertyError(11);
pub const TrackedPropertyError_CannotWriteToWildcards: RawTrackedPropertyError =
    RawTrackedPropertyError(12);
pub const TrackedPropertyError_IPCReadFailure: RawTrackedPropertyError =
    RawTrackedPropertyError(13);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedPropertyError {
    Success = 0,
    WrongDataType = 1,
    WrongDeviceClass = 2,
    BufferTooSmall = 3,
    UnknownProperty = 4,
    InvalidDevice = 5,
    CouldNotContactServer = 6,
    ValueNotProvidedByDevice = 7,
    StringExceedsMaximumLength = 8,
    NotYetAvailable = 9,
    PermissionDenied = 10,
    InvalidOperation = 11,
    CannotWriteToWildcards = 12,
    IPCReadFailure = 13,
}

impl TrackedPropertyError {
    #[inline]
    fn from_raw(val: RawTrackedPropertyError) -> Option<Self> {
        match val {
            TrackedPropertyError_Success => Some(TrackedPropertyError::Success),
            TrackedPropertyError_WrongDataType => Some(TrackedPropertyError::WrongDataType),
            TrackedPropertyError_WrongDeviceClass => Some(TrackedPropertyError::WrongDeviceClass),
            TrackedPropertyError_BufferTooSmall => Some(TrackedPropertyError::BufferTooSmall),
            TrackedPropertyError_UnknownProperty => Some(TrackedPropertyError::UnknownProperty),
            TrackedPropertyError_InvalidDevice => Some(TrackedPropertyError::InvalidDevice),
            TrackedPropertyError_CouldNotContactServer => {
                Some(TrackedPropertyError::CouldNotContactServer)
            }
            TrackedPropertyError_ValueNotProvidedByDevice => {
                Some(TrackedPropertyError::ValueNotProvidedByDevice)
            }
            TrackedPropertyError_StringExceedsMaximumLength => {
                Some(TrackedPropertyError::StringExceedsMaximumLength)
            }
            TrackedPropertyError_NotYetAvailable => Some(TrackedPropertyError::NotYetAvailable),
            TrackedPropertyError_PermissionDenied => Some(TrackedPropertyError::PermissionDenied),
            TrackedPropertyError_InvalidOperation => Some(TrackedPropertyError::InvalidOperation),
            TrackedPropertyError_CannotWriteToWildcards => {
                Some(TrackedPropertyError::CannotWriteToWildcards)
            }
            TrackedPropertyError_IPCReadFailure => Some(TrackedPropertyError::IPCReadFailure),
            _ => None,
        }
    }
}

impl From<RawTrackedPropertyError> for TrackedPropertyError {
    fn from(val: RawTrackedPropertyError) -> Self {
        TrackedPropertyError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackedPropertyError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawSubmitFlags(pub u32);

pub const SubmitFlags_Default: RawSubmitFlags = RawSubmitFlags(0);
pub const SubmitFlags_LensDistortionAlreadyApplied: RawSubmitFlags = RawSubmitFlags(1);
pub const SubmitFlags_GlRenderBuffer: RawSubmitFlags = RawSubmitFlags(2);
pub const SubmitFlags_Reserved: RawSubmitFlags = RawSubmitFlags(4);
pub const SubmitFlags_TextureWithPose: RawSubmitFlags = RawSubmitFlags(8);
pub const SubmitFlags_TextureWithDepth: RawSubmitFlags = RawSubmitFlags(16);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SubmitFlags {
    Default = 0,
    LensDistortionAlreadyApplied = 1,
    GlRenderBuffer = 2,
    Reserved = 4,
    TextureWithPose = 8,
    TextureWithDepth = 16,
}

impl SubmitFlags {
    #[inline]
    fn from_raw(val: RawSubmitFlags) -> Option<Self> {
        match val {
            SubmitFlags_Default => Some(SubmitFlags::Default),
            SubmitFlags_LensDistortionAlreadyApplied => {
                Some(SubmitFlags::LensDistortionAlreadyApplied)
            }
            SubmitFlags_GlRenderBuffer => Some(SubmitFlags::GlRenderBuffer),
            SubmitFlags_Reserved => Some(SubmitFlags::Reserved),
            SubmitFlags_TextureWithPose => Some(SubmitFlags::TextureWithPose),
            SubmitFlags_TextureWithDepth => Some(SubmitFlags::TextureWithDepth),
            _ => None,
        }
    }
}

impl From<RawSubmitFlags> for SubmitFlags {
    fn from(val: RawSubmitFlags) -> Self {
        SubmitFlags::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for SubmitFlags.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawState(pub u32);

pub const State_Undefined: RawState = RawState(::std::u32::MAX);
pub const State_Off: RawState = RawState(0);
pub const State_Searching: RawState = RawState(1);
pub const State_Searching_Alert: RawState = RawState(2);
pub const State_Ready: RawState = RawState(3);
pub const State_Ready_Alert: RawState = RawState(4);
pub const State_NotReady: RawState = RawState(5);
pub const State_Standby: RawState = RawState(6);
pub const State_Ready_Alert_Low: RawState = RawState(7);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum State {
    Undefined = ::std::u32::MAX,
    Off = 0,
    Searching = 1,
    Searching_Alert = 2,
    Ready = 3,
    Ready_Alert = 4,
    NotReady = 5,
    Standby = 6,
    Ready_Alert_Low = 7,
}

impl State {
    #[inline]
    fn from_raw(val: RawState) -> Option<Self> {
        match val {
            State_Undefined => Some(State::Undefined),
            State_Off => Some(State::Off),
            State_Searching => Some(State::Searching),
            State_Searching_Alert => Some(State::Searching_Alert),
            State_Ready => Some(State::Ready),
            State_Ready_Alert => Some(State::Ready_Alert),
            State_NotReady => Some(State::NotReady),
            State_Standby => Some(State::Standby),
            State_Ready_Alert_Low => Some(State::Ready_Alert_Low),
            _ => None,
        }
    }
}

impl From<RawState> for State {
    fn from(val: RawState) -> Self {
        State::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for State.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawEventType(pub u32);

pub const EventType_None: RawEventType = RawEventType(0);
pub const EventType_TrackedDeviceActivated: RawEventType = RawEventType(100);
pub const EventType_TrackedDeviceDeactivated: RawEventType = RawEventType(101);
pub const EventType_TrackedDeviceUpdated: RawEventType = RawEventType(102);
pub const EventType_TrackedDeviceUserInteractionStarted: RawEventType = RawEventType(103);
pub const EventType_TrackedDeviceUserInteractionEnded: RawEventType = RawEventType(104);
pub const EventType_IpdChanged: RawEventType = RawEventType(105);
pub const EventType_EnterStandbyMode: RawEventType = RawEventType(106);
pub const EventType_LeaveStandbyMode: RawEventType = RawEventType(107);
pub const EventType_TrackedDeviceRoleChanged: RawEventType = RawEventType(108);
pub const EventType_WatchdogWakeUpRequested: RawEventType = RawEventType(109);
pub const EventType_LensDistortionChanged: RawEventType = RawEventType(110);
pub const EventType_PropertyChanged: RawEventType = RawEventType(111);
pub const EventType_WirelessDisconnect: RawEventType = RawEventType(112);
pub const EventType_WirelessReconnect: RawEventType = RawEventType(113);
pub const EventType_ButtonPress: RawEventType = RawEventType(200);
pub const EventType_ButtonUnpress: RawEventType = RawEventType(201);
pub const EventType_ButtonTouch: RawEventType = RawEventType(202);
pub const EventType_ButtonUntouch: RawEventType = RawEventType(203);
pub const EventType_DualAnalog_Press: RawEventType = RawEventType(250);
pub const EventType_DualAnalog_Unpress: RawEventType = RawEventType(251);
pub const EventType_DualAnalog_Touch: RawEventType = RawEventType(252);
pub const EventType_DualAnalog_Untouch: RawEventType = RawEventType(253);
pub const EventType_DualAnalog_Move: RawEventType = RawEventType(254);
pub const EventType_DualAnalog_ModeSwitch1: RawEventType = RawEventType(255);
pub const EventType_DualAnalog_ModeSwitch2: RawEventType = RawEventType(256);
pub const EventType_DualAnalog_Cancel: RawEventType = RawEventType(257);
pub const EventType_MouseMove: RawEventType = RawEventType(300);
pub const EventType_MouseButtonDown: RawEventType = RawEventType(301);
pub const EventType_MouseButtonUp: RawEventType = RawEventType(302);
pub const EventType_FocusEnter: RawEventType = RawEventType(303);
pub const EventType_FocusLeave: RawEventType = RawEventType(304);
pub const EventType_Scroll: RawEventType = RawEventType(305);
pub const EventType_TouchPadMove: RawEventType = RawEventType(306);
pub const EventType_OverlayFocusChanged: RawEventType = RawEventType(307);
pub const EventType_ReloadOverlays: RawEventType = RawEventType(308);
pub const EventType_InputFocusCaptured: RawEventType = RawEventType(400);
pub const EventType_InputFocusReleased: RawEventType = RawEventType(401);
pub const EventType_SceneFocusLost: RawEventType = RawEventType(402);
pub const EventType_SceneFocusGained: RawEventType = RawEventType(403);
pub const EventType_SceneApplicationChanged: RawEventType = RawEventType(404);
pub const EventType_SceneFocusChanged: RawEventType = RawEventType(405);
pub const EventType_InputFocusChanged: RawEventType = RawEventType(406);
pub const EventType_SceneApplicationSecondaryRenderingStarted: RawEventType = RawEventType(407);
pub const EventType_SceneApplicationUsingWrongGraphicsAdapter: RawEventType = RawEventType(408);
pub const EventType_ActionBindingReloaded: RawEventType = RawEventType(409);
pub const EventType_HideRenderModels: RawEventType = RawEventType(410);
pub const EventType_ShowRenderModels: RawEventType = RawEventType(411);
pub const EventType_ConsoleOpened: RawEventType = RawEventType(420);
pub const EventType_ConsoleClosed: RawEventType = RawEventType(421);
pub const EventType_OverlayShown: RawEventType = RawEventType(500);
pub const EventType_OverlayHidden: RawEventType = RawEventType(501);
pub const EventType_DashboardActivated: RawEventType = RawEventType(502);
pub const EventType_DashboardDeactivated: RawEventType = RawEventType(503);
pub const EventType_DashboardThumbSelected: RawEventType = RawEventType(504);
pub const EventType_DashboardRequested: RawEventType = RawEventType(505);
pub const EventType_ResetDashboard: RawEventType = RawEventType(506);
pub const EventType_RenderToast: RawEventType = RawEventType(507);
pub const EventType_ImageLoaded: RawEventType = RawEventType(508);
pub const EventType_ShowKeyboard: RawEventType = RawEventType(509);
pub const EventType_HideKeyboard: RawEventType = RawEventType(510);
pub const EventType_OverlayGamepadFocusGained: RawEventType = RawEventType(511);
pub const EventType_OverlayGamepadFocusLost: RawEventType = RawEventType(512);
pub const EventType_OverlaySharedTextureChanged: RawEventType = RawEventType(513);
pub const EventType_ScreenshotTriggered: RawEventType = RawEventType(516);
pub const EventType_ImageFailed: RawEventType = RawEventType(517);
pub const EventType_DashboardOverlayCreated: RawEventType = RawEventType(518);
pub const EventType_SwitchGamepadFocus: RawEventType = RawEventType(519);
pub const EventType_RequestScreenshot: RawEventType = RawEventType(520);
pub const EventType_ScreenshotTaken: RawEventType = RawEventType(521);
pub const EventType_ScreenshotFailed: RawEventType = RawEventType(522);
pub const EventType_SubmitScreenshotToDashboard: RawEventType = RawEventType(523);
pub const EventType_ScreenshotProgressToDashboard: RawEventType = RawEventType(524);
pub const EventType_PrimaryDashboardDeviceChanged: RawEventType = RawEventType(525);
pub const EventType_RoomViewShown: RawEventType = RawEventType(526);
pub const EventType_RoomViewHidden: RawEventType = RawEventType(527);
pub const EventType_ShowUI: RawEventType = RawEventType(528);
pub const EventType_Notification_Shown: RawEventType = RawEventType(600);
pub const EventType_Notification_Hidden: RawEventType = RawEventType(601);
pub const EventType_Notification_BeginInteraction: RawEventType = RawEventType(602);
pub const EventType_Notification_Destroyed: RawEventType = RawEventType(603);
pub const EventType_Quit: RawEventType = RawEventType(700);
pub const EventType_ProcessQuit: RawEventType = RawEventType(701);
pub const EventType_QuitAborted_UserPrompt: RawEventType = RawEventType(702);
pub const EventType_QuitAcknowledged: RawEventType = RawEventType(703);
pub const EventType_DriverRequestedQuit: RawEventType = RawEventType(704);
pub const EventType_ChaperoneDataHasChanged: RawEventType = RawEventType(800);
pub const EventType_ChaperoneUniverseHasChanged: RawEventType = RawEventType(801);
pub const EventType_ChaperoneTempDataHasChanged: RawEventType = RawEventType(802);
pub const EventType_ChaperoneSettingsHaveChanged: RawEventType = RawEventType(803);
pub const EventType_SeatedZeroPoseReset: RawEventType = RawEventType(804);
pub const EventType_ChaperoneFlushCache: RawEventType = RawEventType(805);
pub const EventType_AudioSettingsHaveChanged: RawEventType = RawEventType(820);
pub const EventType_BackgroundSettingHasChanged: RawEventType = RawEventType(850);
pub const EventType_CameraSettingsHaveChanged: RawEventType = RawEventType(851);
pub const EventType_ReprojectionSettingHasChanged: RawEventType = RawEventType(852);
pub const EventType_ModelSkinSettingsHaveChanged: RawEventType = RawEventType(853);
pub const EventType_EnvironmentSettingsHaveChanged: RawEventType = RawEventType(854);
pub const EventType_PowerSettingsHaveChanged: RawEventType = RawEventType(855);
pub const EventType_EnableHomeAppSettingsHaveChanged: RawEventType = RawEventType(856);
pub const EventType_SteamVRSectionSettingChanged: RawEventType = RawEventType(857);
pub const EventType_LighthouseSectionSettingChanged: RawEventType = RawEventType(858);
pub const EventType_NullSectionSettingChanged: RawEventType = RawEventType(859);
pub const EventType_UserInterfaceSectionSettingChanged: RawEventType = RawEventType(860);
pub const EventType_NotificationsSectionSettingChanged: RawEventType = RawEventType(861);
pub const EventType_KeyboardSectionSettingChanged: RawEventType = RawEventType(862);
pub const EventType_PerfSectionSettingChanged: RawEventType = RawEventType(863);
pub const EventType_DashboardSectionSettingChanged: RawEventType = RawEventType(864);
pub const EventType_WebInterfaceSectionSettingChanged: RawEventType = RawEventType(865);
pub const EventType_TrackersSectionSettingChanged: RawEventType = RawEventType(866);
pub const EventType_LastKnownSectionSettingChanged: RawEventType = RawEventType(867);
pub const EventType_StatusUpdate: RawEventType = RawEventType(900);
pub const EventType_WebInterface_InstallDriverCompleted: RawEventType = RawEventType(950);
pub const EventType_MCImageUpdated: RawEventType = RawEventType(1000);
pub const EventType_FirmwareUpdateStarted: RawEventType = RawEventType(1100);
pub const EventType_FirmwareUpdateFinished: RawEventType = RawEventType(1101);
pub const EventType_KeyboardClosed: RawEventType = RawEventType(1200);
pub const EventType_KeyboardCharInput: RawEventType = RawEventType(1201);
pub const EventType_KeyboardDone: RawEventType = RawEventType(1202);
pub const EventType_ApplicationTransitionStarted: RawEventType = RawEventType(1300);
pub const EventType_ApplicationTransitionAborted: RawEventType = RawEventType(1301);
pub const EventType_ApplicationTransitionNewAppStarted: RawEventType = RawEventType(1302);
pub const EventType_ApplicationListUpdated: RawEventType = RawEventType(1303);
pub const EventType_ApplicationMimeTypeLoad: RawEventType = RawEventType(1304);
pub const EventType_ApplicationTransitionNewAppLaunchComplete: RawEventType = RawEventType(1305);
pub const EventType_ProcessConnected: RawEventType = RawEventType(1306);
pub const EventType_ProcessDisconnected: RawEventType = RawEventType(1307);
pub const EventType_Compositor_MirrorWindowShown: RawEventType = RawEventType(1400);
pub const EventType_Compositor_MirrorWindowHidden: RawEventType = RawEventType(1401);
pub const EventType_Compositor_ChaperoneBoundsShown: RawEventType = RawEventType(1410);
pub const EventType_Compositor_ChaperoneBoundsHidden: RawEventType = RawEventType(1411);
pub const EventType_TrackedCamera_StartVideoStream: RawEventType = RawEventType(1500);
pub const EventType_TrackedCamera_StopVideoStream: RawEventType = RawEventType(1501);
pub const EventType_TrackedCamera_PauseVideoStream: RawEventType = RawEventType(1502);
pub const EventType_TrackedCamera_ResumeVideoStream: RawEventType = RawEventType(1503);
pub const EventType_TrackedCamera_EditingSurface: RawEventType = RawEventType(1550);
pub const EventType_PerformanceTest_EnableCapture: RawEventType = RawEventType(1600);
pub const EventType_PerformanceTest_DisableCapture: RawEventType = RawEventType(1601);
pub const EventType_PerformanceTest_FidelityLevel: RawEventType = RawEventType(1602);
pub const EventType_MessageOverlay_Closed: RawEventType = RawEventType(1650);
pub const EventType_MessageOverlayCloseRequested: RawEventType = RawEventType(1651);
pub const EventType_Input_HapticVibration: RawEventType = RawEventType(1700);
pub const EventType_Input_BindingLoadFailed: RawEventType = RawEventType(1701);
pub const EventType_Input_BindingLoadSuccessful: RawEventType = RawEventType(1702);
pub const EventType_Input_ActionManifestReloaded: RawEventType = RawEventType(1703);
pub const EventType_Input_ActionManifestLoadFailed: RawEventType = RawEventType(1704);
pub const EventType_Input_ProgressUpdate: RawEventType = RawEventType(1705);
pub const EventType_Input_TrackerActivated: RawEventType = RawEventType(1706);
pub const EventType_SpatialAnchors_PoseUpdated: RawEventType = RawEventType(1800);
pub const EventType_SpatialAnchors_DescriptorUpdated: RawEventType = RawEventType(1801);
pub const EventType_SpatialAnchors_RequestPoseUpdate: RawEventType = RawEventType(1802);
pub const EventType_SpatialAnchors_RequestDescriptorUpdate: RawEventType = RawEventType(1803);
pub const EventType_VendorSpecific_Reserved_Start: RawEventType = RawEventType(10000);
pub const EventType_VendorSpecific_Reserved_End: RawEventType = RawEventType(19999);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EventType {
    None = 0,
    TrackedDeviceActivated = 100,
    TrackedDeviceDeactivated = 101,
    TrackedDeviceUpdated = 102,
    TrackedDeviceUserInteractionStarted = 103,
    TrackedDeviceUserInteractionEnded = 104,
    IpdChanged = 105,
    EnterStandbyMode = 106,
    LeaveStandbyMode = 107,
    TrackedDeviceRoleChanged = 108,
    WatchdogWakeUpRequested = 109,
    LensDistortionChanged = 110,
    PropertyChanged = 111,
    WirelessDisconnect = 112,
    WirelessReconnect = 113,
    ButtonPress = 200,
    ButtonUnpress = 201,
    ButtonTouch = 202,
    ButtonUntouch = 203,
    DualAnalog_Press = 250,
    DualAnalog_Unpress = 251,
    DualAnalog_Touch = 252,
    DualAnalog_Untouch = 253,
    DualAnalog_Move = 254,
    DualAnalog_ModeSwitch1 = 255,
    DualAnalog_ModeSwitch2 = 256,
    DualAnalog_Cancel = 257,
    MouseMove = 300,
    MouseButtonDown = 301,
    MouseButtonUp = 302,
    FocusEnter = 303,
    FocusLeave = 304,
    Scroll = 305,
    TouchPadMove = 306,
    OverlayFocusChanged = 307,
    ReloadOverlays = 308,
    InputFocusCaptured = 400,
    InputFocusReleased = 401,
    SceneFocusLost = 402,
    SceneFocusGained = 403,
    SceneApplicationChanged = 404,
    SceneFocusChanged = 405,
    InputFocusChanged = 406,
    SceneApplicationSecondaryRenderingStarted = 407,
    SceneApplicationUsingWrongGraphicsAdapter = 408,
    ActionBindingReloaded = 409,
    HideRenderModels = 410,
    ShowRenderModels = 411,
    ConsoleOpened = 420,
    ConsoleClosed = 421,
    OverlayShown = 500,
    OverlayHidden = 501,
    DashboardActivated = 502,
    DashboardDeactivated = 503,
    DashboardThumbSelected = 504,
    DashboardRequested = 505,
    ResetDashboard = 506,
    RenderToast = 507,
    ImageLoaded = 508,
    ShowKeyboard = 509,
    HideKeyboard = 510,
    OverlayGamepadFocusGained = 511,
    OverlayGamepadFocusLost = 512,
    OverlaySharedTextureChanged = 513,
    ScreenshotTriggered = 516,
    ImageFailed = 517,
    DashboardOverlayCreated = 518,
    SwitchGamepadFocus = 519,
    RequestScreenshot = 520,
    ScreenshotTaken = 521,
    ScreenshotFailed = 522,
    SubmitScreenshotToDashboard = 523,
    ScreenshotProgressToDashboard = 524,
    PrimaryDashboardDeviceChanged = 525,
    RoomViewShown = 526,
    RoomViewHidden = 527,
    ShowUI = 528,
    Notification_Shown = 600,
    Notification_Hidden = 601,
    Notification_BeginInteraction = 602,
    Notification_Destroyed = 603,
    Quit = 700,
    ProcessQuit = 701,
    QuitAborted_UserPrompt = 702,
    QuitAcknowledged = 703,
    DriverRequestedQuit = 704,
    ChaperoneDataHasChanged = 800,
    ChaperoneUniverseHasChanged = 801,
    ChaperoneTempDataHasChanged = 802,
    ChaperoneSettingsHaveChanged = 803,
    SeatedZeroPoseReset = 804,
    ChaperoneFlushCache = 805,
    AudioSettingsHaveChanged = 820,
    BackgroundSettingHasChanged = 850,
    CameraSettingsHaveChanged = 851,
    ReprojectionSettingHasChanged = 852,
    ModelSkinSettingsHaveChanged = 853,
    EnvironmentSettingsHaveChanged = 854,
    PowerSettingsHaveChanged = 855,
    EnableHomeAppSettingsHaveChanged = 856,
    SteamVRSectionSettingChanged = 857,
    LighthouseSectionSettingChanged = 858,
    NullSectionSettingChanged = 859,
    UserInterfaceSectionSettingChanged = 860,
    NotificationsSectionSettingChanged = 861,
    KeyboardSectionSettingChanged = 862,
    PerfSectionSettingChanged = 863,
    DashboardSectionSettingChanged = 864,
    WebInterfaceSectionSettingChanged = 865,
    TrackersSectionSettingChanged = 866,
    LastKnownSectionSettingChanged = 867,
    StatusUpdate = 900,
    WebInterface_InstallDriverCompleted = 950,
    MCImageUpdated = 1000,
    FirmwareUpdateStarted = 1100,
    FirmwareUpdateFinished = 1101,
    KeyboardClosed = 1200,
    KeyboardCharInput = 1201,
    KeyboardDone = 1202,
    ApplicationTransitionStarted = 1300,
    ApplicationTransitionAborted = 1301,
    ApplicationTransitionNewAppStarted = 1302,
    ApplicationListUpdated = 1303,
    ApplicationMimeTypeLoad = 1304,
    ApplicationTransitionNewAppLaunchComplete = 1305,
    ProcessConnected = 1306,
    ProcessDisconnected = 1307,
    Compositor_MirrorWindowShown = 1400,
    Compositor_MirrorWindowHidden = 1401,
    Compositor_ChaperoneBoundsShown = 1410,
    Compositor_ChaperoneBoundsHidden = 1411,
    TrackedCamera_StartVideoStream = 1500,
    TrackedCamera_StopVideoStream = 1501,
    TrackedCamera_PauseVideoStream = 1502,
    TrackedCamera_ResumeVideoStream = 1503,
    TrackedCamera_EditingSurface = 1550,
    PerformanceTest_EnableCapture = 1600,
    PerformanceTest_DisableCapture = 1601,
    PerformanceTest_FidelityLevel = 1602,
    MessageOverlay_Closed = 1650,
    MessageOverlayCloseRequested = 1651,
    Input_HapticVibration = 1700,
    Input_BindingLoadFailed = 1701,
    Input_BindingLoadSuccessful = 1702,
    Input_ActionManifestReloaded = 1703,
    Input_ActionManifestLoadFailed = 1704,
    Input_ProgressUpdate = 1705,
    Input_TrackerActivated = 1706,
    SpatialAnchors_PoseUpdated = 1800,
    SpatialAnchors_DescriptorUpdated = 1801,
    SpatialAnchors_RequestPoseUpdate = 1802,
    SpatialAnchors_RequestDescriptorUpdate = 1803,
    VendorSpecific_Reserved_Start = 10000,
    VendorSpecific_Reserved_End = 19999,
}

impl EventType {
    #[inline]
    fn from_raw(val: RawEventType) -> Option<Self> {
        match val {
            EventType_None => Some(EventType::None),
            EventType_TrackedDeviceActivated => Some(EventType::TrackedDeviceActivated),
            EventType_TrackedDeviceDeactivated => Some(EventType::TrackedDeviceDeactivated),
            EventType_TrackedDeviceUpdated => Some(EventType::TrackedDeviceUpdated),
            EventType_TrackedDeviceUserInteractionStarted => {
                Some(EventType::TrackedDeviceUserInteractionStarted)
            }
            EventType_TrackedDeviceUserInteractionEnded => {
                Some(EventType::TrackedDeviceUserInteractionEnded)
            }
            EventType_IpdChanged => Some(EventType::IpdChanged),
            EventType_EnterStandbyMode => Some(EventType::EnterStandbyMode),
            EventType_LeaveStandbyMode => Some(EventType::LeaveStandbyMode),
            EventType_TrackedDeviceRoleChanged => Some(EventType::TrackedDeviceRoleChanged),
            EventType_WatchdogWakeUpRequested => Some(EventType::WatchdogWakeUpRequested),
            EventType_LensDistortionChanged => Some(EventType::LensDistortionChanged),
            EventType_PropertyChanged => Some(EventType::PropertyChanged),
            EventType_WirelessDisconnect => Some(EventType::WirelessDisconnect),
            EventType_WirelessReconnect => Some(EventType::WirelessReconnect),
            EventType_ButtonPress => Some(EventType::ButtonPress),
            EventType_ButtonUnpress => Some(EventType::ButtonUnpress),
            EventType_ButtonTouch => Some(EventType::ButtonTouch),
            EventType_ButtonUntouch => Some(EventType::ButtonUntouch),
            EventType_DualAnalog_Press => Some(EventType::DualAnalog_Press),
            EventType_DualAnalog_Unpress => Some(EventType::DualAnalog_Unpress),
            EventType_DualAnalog_Touch => Some(EventType::DualAnalog_Touch),
            EventType_DualAnalog_Untouch => Some(EventType::DualAnalog_Untouch),
            EventType_DualAnalog_Move => Some(EventType::DualAnalog_Move),
            EventType_DualAnalog_ModeSwitch1 => Some(EventType::DualAnalog_ModeSwitch1),
            EventType_DualAnalog_ModeSwitch2 => Some(EventType::DualAnalog_ModeSwitch2),
            EventType_DualAnalog_Cancel => Some(EventType::DualAnalog_Cancel),
            EventType_MouseMove => Some(EventType::MouseMove),
            EventType_MouseButtonDown => Some(EventType::MouseButtonDown),
            EventType_MouseButtonUp => Some(EventType::MouseButtonUp),
            EventType_FocusEnter => Some(EventType::FocusEnter),
            EventType_FocusLeave => Some(EventType::FocusLeave),
            EventType_Scroll => Some(EventType::Scroll),
            EventType_TouchPadMove => Some(EventType::TouchPadMove),
            EventType_OverlayFocusChanged => Some(EventType::OverlayFocusChanged),
            EventType_ReloadOverlays => Some(EventType::ReloadOverlays),
            EventType_InputFocusCaptured => Some(EventType::InputFocusCaptured),
            EventType_InputFocusReleased => Some(EventType::InputFocusReleased),
            EventType_SceneFocusLost => Some(EventType::SceneFocusLost),
            EventType_SceneFocusGained => Some(EventType::SceneFocusGained),
            EventType_SceneApplicationChanged => Some(EventType::SceneApplicationChanged),
            EventType_SceneFocusChanged => Some(EventType::SceneFocusChanged),
            EventType_InputFocusChanged => Some(EventType::InputFocusChanged),
            EventType_SceneApplicationSecondaryRenderingStarted => {
                Some(EventType::SceneApplicationSecondaryRenderingStarted)
            }
            EventType_SceneApplicationUsingWrongGraphicsAdapter => {
                Some(EventType::SceneApplicationUsingWrongGraphicsAdapter)
            }
            EventType_ActionBindingReloaded => Some(EventType::ActionBindingReloaded),
            EventType_HideRenderModels => Some(EventType::HideRenderModels),
            EventType_ShowRenderModels => Some(EventType::ShowRenderModels),
            EventType_ConsoleOpened => Some(EventType::ConsoleOpened),
            EventType_ConsoleClosed => Some(EventType::ConsoleClosed),
            EventType_OverlayShown => Some(EventType::OverlayShown),
            EventType_OverlayHidden => Some(EventType::OverlayHidden),
            EventType_DashboardActivated => Some(EventType::DashboardActivated),
            EventType_DashboardDeactivated => Some(EventType::DashboardDeactivated),
            EventType_DashboardThumbSelected => Some(EventType::DashboardThumbSelected),
            EventType_DashboardRequested => Some(EventType::DashboardRequested),
            EventType_ResetDashboard => Some(EventType::ResetDashboard),
            EventType_RenderToast => Some(EventType::RenderToast),
            EventType_ImageLoaded => Some(EventType::ImageLoaded),
            EventType_ShowKeyboard => Some(EventType::ShowKeyboard),
            EventType_HideKeyboard => Some(EventType::HideKeyboard),
            EventType_OverlayGamepadFocusGained => Some(EventType::OverlayGamepadFocusGained),
            EventType_OverlayGamepadFocusLost => Some(EventType::OverlayGamepadFocusLost),
            EventType_OverlaySharedTextureChanged => Some(EventType::OverlaySharedTextureChanged),
            EventType_ScreenshotTriggered => Some(EventType::ScreenshotTriggered),
            EventType_ImageFailed => Some(EventType::ImageFailed),
            EventType_DashboardOverlayCreated => Some(EventType::DashboardOverlayCreated),
            EventType_SwitchGamepadFocus => Some(EventType::SwitchGamepadFocus),
            EventType_RequestScreenshot => Some(EventType::RequestScreenshot),
            EventType_ScreenshotTaken => Some(EventType::ScreenshotTaken),
            EventType_ScreenshotFailed => Some(EventType::ScreenshotFailed),
            EventType_SubmitScreenshotToDashboard => Some(EventType::SubmitScreenshotToDashboard),
            EventType_ScreenshotProgressToDashboard => {
                Some(EventType::ScreenshotProgressToDashboard)
            }
            EventType_PrimaryDashboardDeviceChanged => {
                Some(EventType::PrimaryDashboardDeviceChanged)
            }
            EventType_RoomViewShown => Some(EventType::RoomViewShown),
            EventType_RoomViewHidden => Some(EventType::RoomViewHidden),
            EventType_ShowUI => Some(EventType::ShowUI),
            EventType_Notification_Shown => Some(EventType::Notification_Shown),
            EventType_Notification_Hidden => Some(EventType::Notification_Hidden),
            EventType_Notification_BeginInteraction => {
                Some(EventType::Notification_BeginInteraction)
            }
            EventType_Notification_Destroyed => Some(EventType::Notification_Destroyed),
            EventType_Quit => Some(EventType::Quit),
            EventType_ProcessQuit => Some(EventType::ProcessQuit),
            EventType_QuitAborted_UserPrompt => Some(EventType::QuitAborted_UserPrompt),
            EventType_QuitAcknowledged => Some(EventType::QuitAcknowledged),
            EventType_DriverRequestedQuit => Some(EventType::DriverRequestedQuit),
            EventType_ChaperoneDataHasChanged => Some(EventType::ChaperoneDataHasChanged),
            EventType_ChaperoneUniverseHasChanged => Some(EventType::ChaperoneUniverseHasChanged),
            EventType_ChaperoneTempDataHasChanged => Some(EventType::ChaperoneTempDataHasChanged),
            EventType_ChaperoneSettingsHaveChanged => Some(EventType::ChaperoneSettingsHaveChanged),
            EventType_SeatedZeroPoseReset => Some(EventType::SeatedZeroPoseReset),
            EventType_ChaperoneFlushCache => Some(EventType::ChaperoneFlushCache),
            EventType_AudioSettingsHaveChanged => Some(EventType::AudioSettingsHaveChanged),
            EventType_BackgroundSettingHasChanged => Some(EventType::BackgroundSettingHasChanged),
            EventType_CameraSettingsHaveChanged => Some(EventType::CameraSettingsHaveChanged),
            EventType_ReprojectionSettingHasChanged => {
                Some(EventType::ReprojectionSettingHasChanged)
            }
            EventType_ModelSkinSettingsHaveChanged => Some(EventType::ModelSkinSettingsHaveChanged),
            EventType_EnvironmentSettingsHaveChanged => {
                Some(EventType::EnvironmentSettingsHaveChanged)
            }
            EventType_PowerSettingsHaveChanged => Some(EventType::PowerSettingsHaveChanged),
            EventType_EnableHomeAppSettingsHaveChanged => {
                Some(EventType::EnableHomeAppSettingsHaveChanged)
            }
            EventType_SteamVRSectionSettingChanged => Some(EventType::SteamVRSectionSettingChanged),
            EventType_LighthouseSectionSettingChanged => {
                Some(EventType::LighthouseSectionSettingChanged)
            }
            EventType_NullSectionSettingChanged => Some(EventType::NullSectionSettingChanged),
            EventType_UserInterfaceSectionSettingChanged => {
                Some(EventType::UserInterfaceSectionSettingChanged)
            }
            EventType_NotificationsSectionSettingChanged => {
                Some(EventType::NotificationsSectionSettingChanged)
            }
            EventType_KeyboardSectionSettingChanged => {
                Some(EventType::KeyboardSectionSettingChanged)
            }
            EventType_PerfSectionSettingChanged => Some(EventType::PerfSectionSettingChanged),
            EventType_DashboardSectionSettingChanged => {
                Some(EventType::DashboardSectionSettingChanged)
            }
            EventType_WebInterfaceSectionSettingChanged => {
                Some(EventType::WebInterfaceSectionSettingChanged)
            }
            EventType_TrackersSectionSettingChanged => {
                Some(EventType::TrackersSectionSettingChanged)
            }
            EventType_LastKnownSectionSettingChanged => {
                Some(EventType::LastKnownSectionSettingChanged)
            }
            EventType_StatusUpdate => Some(EventType::StatusUpdate),
            EventType_WebInterface_InstallDriverCompleted => {
                Some(EventType::WebInterface_InstallDriverCompleted)
            }
            EventType_MCImageUpdated => Some(EventType::MCImageUpdated),
            EventType_FirmwareUpdateStarted => Some(EventType::FirmwareUpdateStarted),
            EventType_FirmwareUpdateFinished => Some(EventType::FirmwareUpdateFinished),
            EventType_KeyboardClosed => Some(EventType::KeyboardClosed),
            EventType_KeyboardCharInput => Some(EventType::KeyboardCharInput),
            EventType_KeyboardDone => Some(EventType::KeyboardDone),
            EventType_ApplicationTransitionStarted => Some(EventType::ApplicationTransitionStarted),
            EventType_ApplicationTransitionAborted => Some(EventType::ApplicationTransitionAborted),
            EventType_ApplicationTransitionNewAppStarted => {
                Some(EventType::ApplicationTransitionNewAppStarted)
            }
            EventType_ApplicationListUpdated => Some(EventType::ApplicationListUpdated),
            EventType_ApplicationMimeTypeLoad => Some(EventType::ApplicationMimeTypeLoad),
            EventType_ApplicationTransitionNewAppLaunchComplete => {
                Some(EventType::ApplicationTransitionNewAppLaunchComplete)
            }
            EventType_ProcessConnected => Some(EventType::ProcessConnected),
            EventType_ProcessDisconnected => Some(EventType::ProcessDisconnected),
            EventType_Compositor_MirrorWindowShown => Some(EventType::Compositor_MirrorWindowShown),
            EventType_Compositor_MirrorWindowHidden => {
                Some(EventType::Compositor_MirrorWindowHidden)
            }
            EventType_Compositor_ChaperoneBoundsShown => {
                Some(EventType::Compositor_ChaperoneBoundsShown)
            }
            EventType_Compositor_ChaperoneBoundsHidden => {
                Some(EventType::Compositor_ChaperoneBoundsHidden)
            }
            EventType_TrackedCamera_StartVideoStream => {
                Some(EventType::TrackedCamera_StartVideoStream)
            }
            EventType_TrackedCamera_StopVideoStream => {
                Some(EventType::TrackedCamera_StopVideoStream)
            }
            EventType_TrackedCamera_PauseVideoStream => {
                Some(EventType::TrackedCamera_PauseVideoStream)
            }
            EventType_TrackedCamera_ResumeVideoStream => {
                Some(EventType::TrackedCamera_ResumeVideoStream)
            }
            EventType_TrackedCamera_EditingSurface => Some(EventType::TrackedCamera_EditingSurface),
            EventType_PerformanceTest_EnableCapture => {
                Some(EventType::PerformanceTest_EnableCapture)
            }
            EventType_PerformanceTest_DisableCapture => {
                Some(EventType::PerformanceTest_DisableCapture)
            }
            EventType_PerformanceTest_FidelityLevel => {
                Some(EventType::PerformanceTest_FidelityLevel)
            }
            EventType_MessageOverlay_Closed => Some(EventType::MessageOverlay_Closed),
            EventType_MessageOverlayCloseRequested => Some(EventType::MessageOverlayCloseRequested),
            EventType_Input_HapticVibration => Some(EventType::Input_HapticVibration),
            EventType_Input_BindingLoadFailed => Some(EventType::Input_BindingLoadFailed),
            EventType_Input_BindingLoadSuccessful => Some(EventType::Input_BindingLoadSuccessful),
            EventType_Input_ActionManifestReloaded => Some(EventType::Input_ActionManifestReloaded),
            EventType_Input_ActionManifestLoadFailed => {
                Some(EventType::Input_ActionManifestLoadFailed)
            }
            EventType_Input_ProgressUpdate => Some(EventType::Input_ProgressUpdate),
            EventType_Input_TrackerActivated => Some(EventType::Input_TrackerActivated),
            EventType_SpatialAnchors_PoseUpdated => Some(EventType::SpatialAnchors_PoseUpdated),
            EventType_SpatialAnchors_DescriptorUpdated => {
                Some(EventType::SpatialAnchors_DescriptorUpdated)
            }
            EventType_SpatialAnchors_RequestPoseUpdate => {
                Some(EventType::SpatialAnchors_RequestPoseUpdate)
            }
            EventType_SpatialAnchors_RequestDescriptorUpdate => {
                Some(EventType::SpatialAnchors_RequestDescriptorUpdate)
            }
            EventType_VendorSpecific_Reserved_Start => {
                Some(EventType::VendorSpecific_Reserved_Start)
            }
            EventType_VendorSpecific_Reserved_End => Some(EventType::VendorSpecific_Reserved_End),
            _ => None,
        }
    }
}

impl From<RawEventType> for EventType {
    fn from(val: RawEventType) -> Self {
        EventType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for EventType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawDeviceActivityLevel(pub u32);

pub const DeviceActivityLevel_EDeviceActivityLevel_Unknown: RawDeviceActivityLevel =
    RawDeviceActivityLevel(::std::u32::MAX);
pub const DeviceActivityLevel_EDeviceActivityLevel_Idle: RawDeviceActivityLevel =
    RawDeviceActivityLevel(0);
pub const DeviceActivityLevel_EDeviceActivityLevel_UserInteraction: RawDeviceActivityLevel =
    RawDeviceActivityLevel(1);
pub const DeviceActivityLevel_EDeviceActivityLevel_UserInteraction_Timeout: RawDeviceActivityLevel =
    RawDeviceActivityLevel(2);
pub const DeviceActivityLevel_EDeviceActivityLevel_Standby: RawDeviceActivityLevel =
    RawDeviceActivityLevel(3);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DeviceActivityLevel {
    EDeviceActivityLevel_Unknown = ::std::u32::MAX,
    EDeviceActivityLevel_Idle = 0,
    EDeviceActivityLevel_UserInteraction = 1,
    EDeviceActivityLevel_UserInteraction_Timeout = 2,
    EDeviceActivityLevel_Standby = 3,
}

impl DeviceActivityLevel {
    #[inline]
    fn from_raw(val: RawDeviceActivityLevel) -> Option<Self> {
        match val {
            DeviceActivityLevel_EDeviceActivityLevel_Unknown => {
                Some(DeviceActivityLevel::EDeviceActivityLevel_Unknown)
            }
            DeviceActivityLevel_EDeviceActivityLevel_Idle => {
                Some(DeviceActivityLevel::EDeviceActivityLevel_Idle)
            }
            DeviceActivityLevel_EDeviceActivityLevel_UserInteraction => {
                Some(DeviceActivityLevel::EDeviceActivityLevel_UserInteraction)
            }
            DeviceActivityLevel_EDeviceActivityLevel_UserInteraction_Timeout => {
                Some(DeviceActivityLevel::EDeviceActivityLevel_UserInteraction_Timeout)
            }
            DeviceActivityLevel_EDeviceActivityLevel_Standby => {
                Some(DeviceActivityLevel::EDeviceActivityLevel_Standby)
            }
            _ => None,
        }
    }
}

impl From<RawDeviceActivityLevel> for DeviceActivityLevel {
    fn from(val: RawDeviceActivityLevel) -> Self {
        DeviceActivityLevel::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for DeviceActivityLevel.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawButtonId(pub u32);

pub const ButtonId_EButton_System: RawButtonId = RawButtonId(0);
pub const ButtonId_EButton_ApplicationMenu: RawButtonId = RawButtonId(1);
pub const ButtonId_EButton_Grip: RawButtonId = RawButtonId(2);
pub const ButtonId_EButton_DPad_Left: RawButtonId = RawButtonId(3);
pub const ButtonId_EButton_DPad_Up: RawButtonId = RawButtonId(4);
pub const ButtonId_EButton_DPad_Right: RawButtonId = RawButtonId(5);
pub const ButtonId_EButton_DPad_Down: RawButtonId = RawButtonId(6);
pub const ButtonId_EButton_A: RawButtonId = RawButtonId(7);
pub const ButtonId_EButton_ProximitySensor: RawButtonId = RawButtonId(31);
pub const ButtonId_EButton_Axis0: RawButtonId = RawButtonId(32);
pub const ButtonId_EButton_Axis1: RawButtonId = RawButtonId(33);
pub const ButtonId_EButton_Axis2: RawButtonId = RawButtonId(34);
pub const ButtonId_EButton_Axis3: RawButtonId = RawButtonId(35);
pub const ButtonId_EButton_Axis4: RawButtonId = RawButtonId(36);
pub const ButtonId_EButton_SteamVR_Touchpad: RawButtonId = RawButtonId(32);
pub const ButtonId_EButton_SteamVR_Trigger: RawButtonId = RawButtonId(33);
pub const ButtonId_EButton_Dashboard_Back: RawButtonId = RawButtonId(2);
pub const ButtonId_EButton_Knuckles_A: RawButtonId = RawButtonId(2);
pub const ButtonId_EButton_Knuckles_B: RawButtonId = RawButtonId(1);
pub const ButtonId_EButton_Knuckles_JoyStick: RawButtonId = RawButtonId(35);
pub const ButtonId_EButton_Max: RawButtonId = RawButtonId(64);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ButtonId {
    EButton_System = 0,
    EButton_ApplicationMenu = 1,
    EButton_Grip = 2,
    EButton_DPad_Left = 3,
    EButton_DPad_Up = 4,
    EButton_DPad_Right = 5,
    EButton_DPad_Down = 6,
    EButton_A = 7,
    EButton_ProximitySensor = 31,
    EButton_Axis0 = 32,
    EButton_Axis1 = 33,
    EButton_Axis2 = 34,
    EButton_Axis3 = 35,
    EButton_Axis4 = 36,
    EButton_SteamVR_Touchpad = 32,
    EButton_SteamVR_Trigger = 33,
    EButton_Dashboard_Back = 2,
    EButton_Knuckles_A = 2,
    EButton_Knuckles_B = 1,
    EButton_Knuckles_JoyStick = 35,
    EButton_Max = 64,
}

impl ButtonId {
    #[inline]
    fn from_raw(val: RawButtonId) -> Option<Self> {
        match val {
            ButtonId_EButton_System => Some(ButtonId::EButton_System),
            ButtonId_EButton_ApplicationMenu => Some(ButtonId::EButton_ApplicationMenu),
            ButtonId_EButton_Grip => Some(ButtonId::EButton_Grip),
            ButtonId_EButton_DPad_Left => Some(ButtonId::EButton_DPad_Left),
            ButtonId_EButton_DPad_Up => Some(ButtonId::EButton_DPad_Up),
            ButtonId_EButton_DPad_Right => Some(ButtonId::EButton_DPad_Right),
            ButtonId_EButton_DPad_Down => Some(ButtonId::EButton_DPad_Down),
            ButtonId_EButton_A => Some(ButtonId::EButton_A),
            ButtonId_EButton_ProximitySensor => Some(ButtonId::EButton_ProximitySensor),
            ButtonId_EButton_Axis0 => Some(ButtonId::EButton_Axis0),
            ButtonId_EButton_Axis1 => Some(ButtonId::EButton_Axis1),
            ButtonId_EButton_Axis2 => Some(ButtonId::EButton_Axis2),
            ButtonId_EButton_Axis3 => Some(ButtonId::EButton_Axis3),
            ButtonId_EButton_Axis4 => Some(ButtonId::EButton_Axis4),
            ButtonId_EButton_SteamVR_Touchpad => Some(ButtonId::EButton_SteamVR_Touchpad),
            ButtonId_EButton_SteamVR_Trigger => Some(ButtonId::EButton_SteamVR_Trigger),
            ButtonId_EButton_Dashboard_Back => Some(ButtonId::EButton_Dashboard_Back),
            ButtonId_EButton_Knuckles_A => Some(ButtonId::EButton_Knuckles_A),
            ButtonId_EButton_Knuckles_B => Some(ButtonId::EButton_Knuckles_B),
            ButtonId_EButton_Knuckles_JoyStick => Some(ButtonId::EButton_Knuckles_JoyStick),
            ButtonId_EButton_Max => Some(ButtonId::EButton_Max),
            _ => None,
        }
    }
}

impl From<RawButtonId> for ButtonId {
    fn from(val: RawButtonId) -> Self {
        ButtonId::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ButtonId.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawMouseButton(pub u32);

pub const MouseButton_Left: RawMouseButton = RawMouseButton(1);
pub const MouseButton_Right: RawMouseButton = RawMouseButton(2);
pub const MouseButton_Middle: RawMouseButton = RawMouseButton(4);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MouseButton {
    Left = 1,
    Right = 2,
    Middle = 4,
}

impl MouseButton {
    #[inline]
    fn from_raw(val: RawMouseButton) -> Option<Self> {
        match val {
            MouseButton_Left => Some(MouseButton::Left),
            MouseButton_Right => Some(MouseButton::Right),
            MouseButton_Middle => Some(MouseButton::Middle),
            _ => None,
        }
    }
}

impl From<RawMouseButton> for MouseButton {
    fn from(val: RawMouseButton) -> Self {
        MouseButton::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for MouseButton.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawDualAnalogWhich(pub u32);

pub const DualAnalogWhich_EDualAnalog_Left: RawDualAnalogWhich = RawDualAnalogWhich(0);
pub const DualAnalogWhich_EDualAnalog_Right: RawDualAnalogWhich = RawDualAnalogWhich(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DualAnalogWhich {
    EDualAnalog_Left = 0,
    EDualAnalog_Right = 1,
}

impl DualAnalogWhich {
    #[inline]
    fn from_raw(val: RawDualAnalogWhich) -> Option<Self> {
        match val {
            DualAnalogWhich_EDualAnalog_Left => Some(DualAnalogWhich::EDualAnalog_Left),
            DualAnalogWhich_EDualAnalog_Right => Some(DualAnalogWhich::EDualAnalog_Right),
            _ => None,
        }
    }
}

impl From<RawDualAnalogWhich> for DualAnalogWhich {
    fn from(val: RawDualAnalogWhich) -> Self {
        DualAnalogWhich::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for DualAnalogWhich.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawShowUIType(pub u32);

pub const ShowUIType_ControllerBinding: RawShowUIType = RawShowUIType(0);
pub const ShowUIType_ManageTrackers: RawShowUIType = RawShowUIType(1);
pub const ShowUIType_QuickStart: RawShowUIType = RawShowUIType(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ShowUIType {
    ControllerBinding = 0,
    ManageTrackers = 1,
    QuickStart = 2,
}

impl ShowUIType {
    #[inline]
    fn from_raw(val: RawShowUIType) -> Option<Self> {
        match val {
            ShowUIType_ControllerBinding => Some(ShowUIType::ControllerBinding),
            ShowUIType_ManageTrackers => Some(ShowUIType::ManageTrackers),
            ShowUIType_QuickStart => Some(ShowUIType::QuickStart),
            _ => None,
        }
    }
}

impl From<RawShowUIType> for ShowUIType {
    fn from(val: RawShowUIType) -> Self {
        ShowUIType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ShowUIType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawInputError(pub u32);

pub const InputError_None: RawInputError = RawInputError(0);
pub const InputError_NameNotFound: RawInputError = RawInputError(1);
pub const InputError_WrongType: RawInputError = RawInputError(2);
pub const InputError_InvalidHandle: RawInputError = RawInputError(3);
pub const InputError_InvalidParam: RawInputError = RawInputError(4);
pub const InputError_NoSteam: RawInputError = RawInputError(5);
pub const InputError_MaxCapacityReached: RawInputError = RawInputError(6);
pub const InputError_IPCError: RawInputError = RawInputError(7);
pub const InputError_NoActiveActionSet: RawInputError = RawInputError(8);
pub const InputError_InvalidDevice: RawInputError = RawInputError(9);
pub const InputError_InvalidSkeleton: RawInputError = RawInputError(10);
pub const InputError_InvalidBoneCount: RawInputError = RawInputError(11);
pub const InputError_InvalidCompressedData: RawInputError = RawInputError(12);
pub const InputError_NoData: RawInputError = RawInputError(13);
pub const InputError_BufferTooSmall: RawInputError = RawInputError(14);
pub const InputError_MismatchedActionManifest: RawInputError = RawInputError(15);
pub const InputError_MissingSkeletonData: RawInputError = RawInputError(16);
pub const InputError_InvalidBoneIndex: RawInputError = RawInputError(17);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InputError {
    None = 0,
    NameNotFound = 1,
    WrongType = 2,
    InvalidHandle = 3,
    InvalidParam = 4,
    NoSteam = 5,
    MaxCapacityReached = 6,
    IPCError = 7,
    NoActiveActionSet = 8,
    InvalidDevice = 9,
    InvalidSkeleton = 10,
    InvalidBoneCount = 11,
    InvalidCompressedData = 12,
    NoData = 13,
    BufferTooSmall = 14,
    MismatchedActionManifest = 15,
    MissingSkeletonData = 16,
    InvalidBoneIndex = 17,
}

impl InputError {
    #[inline]
    fn from_raw(val: RawInputError) -> Option<Self> {
        match val {
            InputError_None => Some(InputError::None),
            InputError_NameNotFound => Some(InputError::NameNotFound),
            InputError_WrongType => Some(InputError::WrongType),
            InputError_InvalidHandle => Some(InputError::InvalidHandle),
            InputError_InvalidParam => Some(InputError::InvalidParam),
            InputError_NoSteam => Some(InputError::NoSteam),
            InputError_MaxCapacityReached => Some(InputError::MaxCapacityReached),
            InputError_IPCError => Some(InputError::IPCError),
            InputError_NoActiveActionSet => Some(InputError::NoActiveActionSet),
            InputError_InvalidDevice => Some(InputError::InvalidDevice),
            InputError_InvalidSkeleton => Some(InputError::InvalidSkeleton),
            InputError_InvalidBoneCount => Some(InputError::InvalidBoneCount),
            InputError_InvalidCompressedData => Some(InputError::InvalidCompressedData),
            InputError_NoData => Some(InputError::NoData),
            InputError_BufferTooSmall => Some(InputError::BufferTooSmall),
            InputError_MismatchedActionManifest => Some(InputError::MismatchedActionManifest),
            InputError_MissingSkeletonData => Some(InputError::MissingSkeletonData),
            InputError_InvalidBoneIndex => Some(InputError::InvalidBoneIndex),
            _ => None,
        }
    }
}

impl From<RawInputError> for InputError {
    fn from(val: RawInputError) -> Self {
        InputError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for InputError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawSpatialAnchorError(pub u32);

pub const SpatialAnchorError_Success: RawSpatialAnchorError = RawSpatialAnchorError(0);
pub const SpatialAnchorError_Internal: RawSpatialAnchorError = RawSpatialAnchorError(1);
pub const SpatialAnchorError_UnknownHandle: RawSpatialAnchorError = RawSpatialAnchorError(2);
pub const SpatialAnchorError_ArrayTooSmall: RawSpatialAnchorError = RawSpatialAnchorError(3);
pub const SpatialAnchorError_InvalidDescriptorChar: RawSpatialAnchorError =
    RawSpatialAnchorError(4);
pub const SpatialAnchorError_NotYetAvailable: RawSpatialAnchorError = RawSpatialAnchorError(5);
pub const SpatialAnchorError_NotAvailableInThisUniverse: RawSpatialAnchorError =
    RawSpatialAnchorError(6);
pub const SpatialAnchorError_PermanentlyUnavailable: RawSpatialAnchorError =
    RawSpatialAnchorError(7);
pub const SpatialAnchorError_WrongDriver: RawSpatialAnchorError = RawSpatialAnchorError(8);
pub const SpatialAnchorError_DescriptorTooLong: RawSpatialAnchorError = RawSpatialAnchorError(9);
pub const SpatialAnchorError_Unknown: RawSpatialAnchorError = RawSpatialAnchorError(10);
pub const SpatialAnchorError_NoRoomCalibration: RawSpatialAnchorError = RawSpatialAnchorError(11);
pub const SpatialAnchorError_InvalidArgument: RawSpatialAnchorError = RawSpatialAnchorError(12);
pub const SpatialAnchorError_UnknownDriver: RawSpatialAnchorError = RawSpatialAnchorError(13);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SpatialAnchorError {
    Success = 0,
    Internal = 1,
    UnknownHandle = 2,
    ArrayTooSmall = 3,
    InvalidDescriptorChar = 4,
    NotYetAvailable = 5,
    NotAvailableInThisUniverse = 6,
    PermanentlyUnavailable = 7,
    WrongDriver = 8,
    DescriptorTooLong = 9,
    Unknown = 10,
    NoRoomCalibration = 11,
    InvalidArgument = 12,
    UnknownDriver = 13,
}

impl SpatialAnchorError {
    #[inline]
    fn from_raw(val: RawSpatialAnchorError) -> Option<Self> {
        match val {
            SpatialAnchorError_Success => Some(SpatialAnchorError::Success),
            SpatialAnchorError_Internal => Some(SpatialAnchorError::Internal),
            SpatialAnchorError_UnknownHandle => Some(SpatialAnchorError::UnknownHandle),
            SpatialAnchorError_ArrayTooSmall => Some(SpatialAnchorError::ArrayTooSmall),
            SpatialAnchorError_InvalidDescriptorChar => {
                Some(SpatialAnchorError::InvalidDescriptorChar)
            }
            SpatialAnchorError_NotYetAvailable => Some(SpatialAnchorError::NotYetAvailable),
            SpatialAnchorError_NotAvailableInThisUniverse => {
                Some(SpatialAnchorError::NotAvailableInThisUniverse)
            }
            SpatialAnchorError_PermanentlyUnavailable => {
                Some(SpatialAnchorError::PermanentlyUnavailable)
            }
            SpatialAnchorError_WrongDriver => Some(SpatialAnchorError::WrongDriver),
            SpatialAnchorError_DescriptorTooLong => Some(SpatialAnchorError::DescriptorTooLong),
            SpatialAnchorError_Unknown => Some(SpatialAnchorError::Unknown),
            SpatialAnchorError_NoRoomCalibration => Some(SpatialAnchorError::NoRoomCalibration),
            SpatialAnchorError_InvalidArgument => Some(SpatialAnchorError::InvalidArgument),
            SpatialAnchorError_UnknownDriver => Some(SpatialAnchorError::UnknownDriver),
            _ => None,
        }
    }
}

impl From<RawSpatialAnchorError> for SpatialAnchorError {
    fn from(val: RawSpatialAnchorError) -> Self {
        SpatialAnchorError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for SpatialAnchorError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawHiddenAreaMeshType(pub u32);

pub const HiddenAreaMeshType_eHiddenAreaMesh_Standard: RawHiddenAreaMeshType =
    RawHiddenAreaMeshType(0);
pub const HiddenAreaMeshType_eHiddenAreaMesh_Inverse: RawHiddenAreaMeshType =
    RawHiddenAreaMeshType(1);
pub const HiddenAreaMeshType_eHiddenAreaMesh_LineLoop: RawHiddenAreaMeshType =
    RawHiddenAreaMeshType(2);
pub const HiddenAreaMeshType_eHiddenAreaMesh_Max: RawHiddenAreaMeshType = RawHiddenAreaMeshType(3);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum HiddenAreaMeshType {
    eHiddenAreaMesh_Standard = 0,
    eHiddenAreaMesh_Inverse = 1,
    eHiddenAreaMesh_LineLoop = 2,
    eHiddenAreaMesh_Max = 3,
}

impl HiddenAreaMeshType {
    #[inline]
    fn from_raw(val: RawHiddenAreaMeshType) -> Option<Self> {
        match val {
            HiddenAreaMeshType_eHiddenAreaMesh_Standard => {
                Some(HiddenAreaMeshType::eHiddenAreaMesh_Standard)
            }
            HiddenAreaMeshType_eHiddenAreaMesh_Inverse => {
                Some(HiddenAreaMeshType::eHiddenAreaMesh_Inverse)
            }
            HiddenAreaMeshType_eHiddenAreaMesh_LineLoop => {
                Some(HiddenAreaMeshType::eHiddenAreaMesh_LineLoop)
            }
            HiddenAreaMeshType_eHiddenAreaMesh_Max => Some(HiddenAreaMeshType::eHiddenAreaMesh_Max),
            _ => None,
        }
    }
}

impl From<RawHiddenAreaMeshType> for HiddenAreaMeshType {
    fn from(val: RawHiddenAreaMeshType) -> Self {
        HiddenAreaMeshType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for HiddenAreaMeshType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawControllerAxisType(pub u32);

pub const ControllerAxisType_eControllerAxis_None: RawControllerAxisType = RawControllerAxisType(0);
pub const ControllerAxisType_eControllerAxis_TrackPad: RawControllerAxisType =
    RawControllerAxisType(1);
pub const ControllerAxisType_eControllerAxis_Joystick: RawControllerAxisType =
    RawControllerAxisType(2);
pub const ControllerAxisType_eControllerAxis_Trigger: RawControllerAxisType =
    RawControllerAxisType(3);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ControllerAxisType {
    eControllerAxis_None = 0,
    eControllerAxis_TrackPad = 1,
    eControllerAxis_Joystick = 2,
    eControllerAxis_Trigger = 3,
}

impl ControllerAxisType {
    #[inline]
    fn from_raw(val: RawControllerAxisType) -> Option<Self> {
        match val {
            ControllerAxisType_eControllerAxis_None => {
                Some(ControllerAxisType::eControllerAxis_None)
            }
            ControllerAxisType_eControllerAxis_TrackPad => {
                Some(ControllerAxisType::eControllerAxis_TrackPad)
            }
            ControllerAxisType_eControllerAxis_Joystick => {
                Some(ControllerAxisType::eControllerAxis_Joystick)
            }
            ControllerAxisType_eControllerAxis_Trigger => {
                Some(ControllerAxisType::eControllerAxis_Trigger)
            }
            _ => None,
        }
    }
}

impl From<RawControllerAxisType> for ControllerAxisType {
    fn from(val: RawControllerAxisType) -> Self {
        ControllerAxisType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ControllerAxisType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawControllerEventOutputType(pub u32);

pub const ControllerEventOutputType_OSEvents: RawControllerEventOutputType =
    RawControllerEventOutputType(0);
pub const ControllerEventOutputType_VREvents: RawControllerEventOutputType =
    RawControllerEventOutputType(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ControllerEventOutputType {
    OSEvents = 0,
    VREvents = 1,
}

impl ControllerEventOutputType {
    #[inline]
    fn from_raw(val: RawControllerEventOutputType) -> Option<Self> {
        match val {
            ControllerEventOutputType_OSEvents => Some(ControllerEventOutputType::OSEvents),
            ControllerEventOutputType_VREvents => Some(ControllerEventOutputType::VREvents),
            _ => None,
        }
    }
}

impl From<RawControllerEventOutputType> for ControllerEventOutputType {
    fn from(val: RawControllerEventOutputType) -> Self {
        ControllerEventOutputType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ControllerEventOutputType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawCollisionBoundsStyle(pub u32);

pub const CollisionBoundsStyle_BOUNDS_STYLE_BEGINNER: RawCollisionBoundsStyle =
    RawCollisionBoundsStyle(0);
pub const CollisionBoundsStyle_BOUNDS_STYLE_INTERMEDIATE: RawCollisionBoundsStyle =
    RawCollisionBoundsStyle(1);
pub const CollisionBoundsStyle_BOUNDS_STYLE_SQUARES: RawCollisionBoundsStyle =
    RawCollisionBoundsStyle(2);
pub const CollisionBoundsStyle_BOUNDS_STYLE_ADVANCED: RawCollisionBoundsStyle =
    RawCollisionBoundsStyle(3);
pub const CollisionBoundsStyle_BOUNDS_STYLE_NONE: RawCollisionBoundsStyle =
    RawCollisionBoundsStyle(4);
pub const CollisionBoundsStyle_BOUNDS_STYLE_COUNT: RawCollisionBoundsStyle =
    RawCollisionBoundsStyle(5);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CollisionBoundsStyle {
    BOUNDS_STYLE_BEGINNER = 0,
    BOUNDS_STYLE_INTERMEDIATE = 1,
    BOUNDS_STYLE_SQUARES = 2,
    BOUNDS_STYLE_ADVANCED = 3,
    BOUNDS_STYLE_NONE = 4,
    BOUNDS_STYLE_COUNT = 5,
}

impl CollisionBoundsStyle {
    #[inline]
    fn from_raw(val: RawCollisionBoundsStyle) -> Option<Self> {
        match val {
            CollisionBoundsStyle_BOUNDS_STYLE_BEGINNER => {
                Some(CollisionBoundsStyle::BOUNDS_STYLE_BEGINNER)
            }
            CollisionBoundsStyle_BOUNDS_STYLE_INTERMEDIATE => {
                Some(CollisionBoundsStyle::BOUNDS_STYLE_INTERMEDIATE)
            }
            CollisionBoundsStyle_BOUNDS_STYLE_SQUARES => {
                Some(CollisionBoundsStyle::BOUNDS_STYLE_SQUARES)
            }
            CollisionBoundsStyle_BOUNDS_STYLE_ADVANCED => {
                Some(CollisionBoundsStyle::BOUNDS_STYLE_ADVANCED)
            }
            CollisionBoundsStyle_BOUNDS_STYLE_NONE => Some(CollisionBoundsStyle::BOUNDS_STYLE_NONE),
            CollisionBoundsStyle_BOUNDS_STYLE_COUNT => {
                Some(CollisionBoundsStyle::BOUNDS_STYLE_COUNT)
            }
            _ => None,
        }
    }
}

impl From<RawCollisionBoundsStyle> for CollisionBoundsStyle {
    fn from(val: RawCollisionBoundsStyle) -> Self {
        CollisionBoundsStyle::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for CollisionBoundsStyle.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawOverlayError(pub u32);

pub const OverlayError_None: RawOverlayError = RawOverlayError(0);
pub const OverlayError_UnknownOverlay: RawOverlayError = RawOverlayError(10);
pub const OverlayError_InvalidHandle: RawOverlayError = RawOverlayError(11);
pub const OverlayError_PermissionDenied: RawOverlayError = RawOverlayError(12);
pub const OverlayError_OverlayLimitExceeded: RawOverlayError = RawOverlayError(13);
pub const OverlayError_WrongVisibilityType: RawOverlayError = RawOverlayError(14);
pub const OverlayError_KeyTooLong: RawOverlayError = RawOverlayError(15);
pub const OverlayError_NameTooLong: RawOverlayError = RawOverlayError(16);
pub const OverlayError_KeyInUse: RawOverlayError = RawOverlayError(17);
pub const OverlayError_WrongTransformType: RawOverlayError = RawOverlayError(18);
pub const OverlayError_InvalidTrackedDevice: RawOverlayError = RawOverlayError(19);
pub const OverlayError_InvalidParameter: RawOverlayError = RawOverlayError(20);
pub const OverlayError_ThumbnailCantBeDestroyed: RawOverlayError = RawOverlayError(21);
pub const OverlayError_ArrayTooSmall: RawOverlayError = RawOverlayError(22);
pub const OverlayError_RequestFailed: RawOverlayError = RawOverlayError(23);
pub const OverlayError_InvalidTexture: RawOverlayError = RawOverlayError(24);
pub const OverlayError_UnableToLoadFile: RawOverlayError = RawOverlayError(25);
pub const OverlayError_KeyboardAlreadyInUse: RawOverlayError = RawOverlayError(26);
pub const OverlayError_NoNeighbor: RawOverlayError = RawOverlayError(27);
pub const OverlayError_TooManyMaskPrimitives: RawOverlayError = RawOverlayError(29);
pub const OverlayError_BadMaskPrimitive: RawOverlayError = RawOverlayError(30);
pub const OverlayError_TextureAlreadyLocked: RawOverlayError = RawOverlayError(31);
pub const OverlayError_TextureLockCapacityReached: RawOverlayError = RawOverlayError(32);
pub const OverlayError_TextureNotLocked: RawOverlayError = RawOverlayError(33);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayError {
    None = 0,
    UnknownOverlay = 10,
    InvalidHandle = 11,
    PermissionDenied = 12,
    OverlayLimitExceeded = 13,
    WrongVisibilityType = 14,
    KeyTooLong = 15,
    NameTooLong = 16,
    KeyInUse = 17,
    WrongTransformType = 18,
    InvalidTrackedDevice = 19,
    InvalidParameter = 20,
    ThumbnailCantBeDestroyed = 21,
    ArrayTooSmall = 22,
    RequestFailed = 23,
    InvalidTexture = 24,
    UnableToLoadFile = 25,
    KeyboardAlreadyInUse = 26,
    NoNeighbor = 27,
    TooManyMaskPrimitives = 29,
    BadMaskPrimitive = 30,
    TextureAlreadyLocked = 31,
    TextureLockCapacityReached = 32,
    TextureNotLocked = 33,
}

impl OverlayError {
    #[inline]
    fn from_raw(val: RawOverlayError) -> Option<Self> {
        match val {
            OverlayError_None => Some(OverlayError::None),
            OverlayError_UnknownOverlay => Some(OverlayError::UnknownOverlay),
            OverlayError_InvalidHandle => Some(OverlayError::InvalidHandle),
            OverlayError_PermissionDenied => Some(OverlayError::PermissionDenied),
            OverlayError_OverlayLimitExceeded => Some(OverlayError::OverlayLimitExceeded),
            OverlayError_WrongVisibilityType => Some(OverlayError::WrongVisibilityType),
            OverlayError_KeyTooLong => Some(OverlayError::KeyTooLong),
            OverlayError_NameTooLong => Some(OverlayError::NameTooLong),
            OverlayError_KeyInUse => Some(OverlayError::KeyInUse),
            OverlayError_WrongTransformType => Some(OverlayError::WrongTransformType),
            OverlayError_InvalidTrackedDevice => Some(OverlayError::InvalidTrackedDevice),
            OverlayError_InvalidParameter => Some(OverlayError::InvalidParameter),
            OverlayError_ThumbnailCantBeDestroyed => Some(OverlayError::ThumbnailCantBeDestroyed),
            OverlayError_ArrayTooSmall => Some(OverlayError::ArrayTooSmall),
            OverlayError_RequestFailed => Some(OverlayError::RequestFailed),
            OverlayError_InvalidTexture => Some(OverlayError::InvalidTexture),
            OverlayError_UnableToLoadFile => Some(OverlayError::UnableToLoadFile),
            OverlayError_KeyboardAlreadyInUse => Some(OverlayError::KeyboardAlreadyInUse),
            OverlayError_NoNeighbor => Some(OverlayError::NoNeighbor),
            OverlayError_TooManyMaskPrimitives => Some(OverlayError::TooManyMaskPrimitives),
            OverlayError_BadMaskPrimitive => Some(OverlayError::BadMaskPrimitive),
            OverlayError_TextureAlreadyLocked => Some(OverlayError::TextureAlreadyLocked),
            OverlayError_TextureLockCapacityReached => {
                Some(OverlayError::TextureLockCapacityReached)
            }
            OverlayError_TextureNotLocked => Some(OverlayError::TextureNotLocked),
            _ => None,
        }
    }
}

impl From<RawOverlayError> for OverlayError {
    fn from(val: RawOverlayError) -> Self {
        OverlayError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for OverlayError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawApplicationType(pub u32);

pub const ApplicationType_Other: RawApplicationType = RawApplicationType(0);
pub const ApplicationType_Scene: RawApplicationType = RawApplicationType(1);
pub const ApplicationType_Overlay: RawApplicationType = RawApplicationType(2);
pub const ApplicationType_Background: RawApplicationType = RawApplicationType(3);
pub const ApplicationType_Utility: RawApplicationType = RawApplicationType(4);
pub const ApplicationType_VRMonitor: RawApplicationType = RawApplicationType(5);
pub const ApplicationType_SteamWatchdog: RawApplicationType = RawApplicationType(6);
pub const ApplicationType_Bootstrapper: RawApplicationType = RawApplicationType(7);
pub const ApplicationType_WebHelper: RawApplicationType = RawApplicationType(8);
pub const ApplicationType_Max: RawApplicationType = RawApplicationType(9);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationType {
    Other = 0,
    Scene = 1,
    Overlay = 2,
    Background = 3,
    Utility = 4,
    VRMonitor = 5,
    SteamWatchdog = 6,
    Bootstrapper = 7,
    WebHelper = 8,
    Max = 9,
}

impl ApplicationType {
    #[inline]
    fn from_raw(val: RawApplicationType) -> Option<Self> {
        match val {
            ApplicationType_Other => Some(ApplicationType::Other),
            ApplicationType_Scene => Some(ApplicationType::Scene),
            ApplicationType_Overlay => Some(ApplicationType::Overlay),
            ApplicationType_Background => Some(ApplicationType::Background),
            ApplicationType_Utility => Some(ApplicationType::Utility),
            ApplicationType_VRMonitor => Some(ApplicationType::VRMonitor),
            ApplicationType_SteamWatchdog => Some(ApplicationType::SteamWatchdog),
            ApplicationType_Bootstrapper => Some(ApplicationType::Bootstrapper),
            ApplicationType_WebHelper => Some(ApplicationType::WebHelper),
            ApplicationType_Max => Some(ApplicationType::Max),
            _ => None,
        }
    }
}

impl From<RawApplicationType> for ApplicationType {
    fn from(val: RawApplicationType) -> Self {
        ApplicationType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ApplicationType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawFirmwareError(pub u32);

pub const FirmwareError_None: RawFirmwareError = RawFirmwareError(0);
pub const FirmwareError_Success: RawFirmwareError = RawFirmwareError(1);
pub const FirmwareError_Fail: RawFirmwareError = RawFirmwareError(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FirmwareError {
    None = 0,
    Success = 1,
    Fail = 2,
}

impl FirmwareError {
    #[inline]
    fn from_raw(val: RawFirmwareError) -> Option<Self> {
        match val {
            FirmwareError_None => Some(FirmwareError::None),
            FirmwareError_Success => Some(FirmwareError::Success),
            FirmwareError_Fail => Some(FirmwareError::Fail),
            _ => None,
        }
    }
}

impl From<RawFirmwareError> for FirmwareError {
    fn from(val: RawFirmwareError) -> Self {
        FirmwareError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for FirmwareError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawNotificationError(pub u32);

pub const NotificationError_OK: RawNotificationError = RawNotificationError(0);
pub const NotificationError_InvalidNotificationId: RawNotificationError = RawNotificationError(100);
pub const NotificationError_NotificationQueueFull: RawNotificationError = RawNotificationError(101);
pub const NotificationError_InvalidOverlayHandle: RawNotificationError = RawNotificationError(102);
pub const NotificationError_SystemWithUserValueAlreadyExists: RawNotificationError =
    RawNotificationError(103);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NotificationError {
    OK = 0,
    InvalidNotificationId = 100,
    NotificationQueueFull = 101,
    InvalidOverlayHandle = 102,
    SystemWithUserValueAlreadyExists = 103,
}

impl NotificationError {
    #[inline]
    fn from_raw(val: RawNotificationError) -> Option<Self> {
        match val {
            NotificationError_OK => Some(NotificationError::OK),
            NotificationError_InvalidNotificationId => {
                Some(NotificationError::InvalidNotificationId)
            }
            NotificationError_NotificationQueueFull => {
                Some(NotificationError::NotificationQueueFull)
            }
            NotificationError_InvalidOverlayHandle => Some(NotificationError::InvalidOverlayHandle),
            NotificationError_SystemWithUserValueAlreadyExists => {
                Some(NotificationError::SystemWithUserValueAlreadyExists)
            }
            _ => None,
        }
    }
}

impl From<RawNotificationError> for NotificationError {
    fn from(val: RawNotificationError) -> Self {
        NotificationError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for NotificationError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawSkeletalMotionRange(pub u32);

pub const SkeletalMotionRange_WithController: RawSkeletalMotionRange = RawSkeletalMotionRange(0);
pub const SkeletalMotionRange_WithoutController: RawSkeletalMotionRange = RawSkeletalMotionRange(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalMotionRange {
    WithController = 0,
    WithoutController = 1,
}

impl SkeletalMotionRange {
    #[inline]
    fn from_raw(val: RawSkeletalMotionRange) -> Option<Self> {
        match val {
            SkeletalMotionRange_WithController => Some(SkeletalMotionRange::WithController),
            SkeletalMotionRange_WithoutController => Some(SkeletalMotionRange::WithoutController),
            _ => None,
        }
    }
}

impl From<RawSkeletalMotionRange> for SkeletalMotionRange {
    fn from(val: RawSkeletalMotionRange) -> Self {
        SkeletalMotionRange::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for SkeletalMotionRange.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawSkeletalTrackingLevel(pub u32);

pub const SkeletalTrackingLevel_Estimated: RawSkeletalTrackingLevel = RawSkeletalTrackingLevel(0);
pub const SkeletalTrackingLevel_Partial: RawSkeletalTrackingLevel = RawSkeletalTrackingLevel(1);
pub const SkeletalTrackingLevel_Full: RawSkeletalTrackingLevel = RawSkeletalTrackingLevel(2);
pub const SkeletalTrackingLevel_Count: RawSkeletalTrackingLevel = RawSkeletalTrackingLevel(3);
pub const SkeletalTrackingLevel_Max: RawSkeletalTrackingLevel = RawSkeletalTrackingLevel(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalTrackingLevel {
    Estimated = 0,
    Partial = 1,
    Full = 2,
    Count = 3,
    Max = 2,
}

impl SkeletalTrackingLevel {
    #[inline]
    fn from_raw(val: RawSkeletalTrackingLevel) -> Option<Self> {
        match val {
            SkeletalTrackingLevel_Estimated => Some(SkeletalTrackingLevel::Estimated),
            SkeletalTrackingLevel_Partial => Some(SkeletalTrackingLevel::Partial),
            SkeletalTrackingLevel_Full => Some(SkeletalTrackingLevel::Full),
            SkeletalTrackingLevel_Count => Some(SkeletalTrackingLevel::Count),
            SkeletalTrackingLevel_Max => Some(SkeletalTrackingLevel::Max),
            _ => None,
        }
    }
}

impl From<RawSkeletalTrackingLevel> for SkeletalTrackingLevel {
    fn from(val: RawSkeletalTrackingLevel) -> Self {
        SkeletalTrackingLevel::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for SkeletalTrackingLevel.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawInitError(pub u32);

pub const InitError_None: RawInitError = RawInitError(0);
pub const InitError_Unknown: RawInitError = RawInitError(1);
pub const InitError_Init_InstallationNotFound: RawInitError = RawInitError(100);
pub const InitError_Init_InstallationCorrupt: RawInitError = RawInitError(101);
pub const InitError_Init_VRClientDLLNotFound: RawInitError = RawInitError(102);
pub const InitError_Init_FileNotFound: RawInitError = RawInitError(103);
pub const InitError_Init_FactoryNotFound: RawInitError = RawInitError(104);
pub const InitError_Init_InterfaceNotFound: RawInitError = RawInitError(105);
pub const InitError_Init_InvalidInterface: RawInitError = RawInitError(106);
pub const InitError_Init_UserConfigDirectoryInvalid: RawInitError = RawInitError(107);
pub const InitError_Init_HmdNotFound: RawInitError = RawInitError(108);
pub const InitError_Init_NotInitialized: RawInitError = RawInitError(109);
pub const InitError_Init_PathRegistryNotFound: RawInitError = RawInitError(110);
pub const InitError_Init_NoConfigPath: RawInitError = RawInitError(111);
pub const InitError_Init_NoLogPath: RawInitError = RawInitError(112);
pub const InitError_Init_PathRegistryNotWritable: RawInitError = RawInitError(113);
pub const InitError_Init_AppInfoInitFailed: RawInitError = RawInitError(114);
pub const InitError_Init_Retry: RawInitError = RawInitError(115);
pub const InitError_Init_InitCanceledByUser: RawInitError = RawInitError(116);
pub const InitError_Init_AnotherAppLaunching: RawInitError = RawInitError(117);
pub const InitError_Init_SettingsInitFailed: RawInitError = RawInitError(118);
pub const InitError_Init_ShuttingDown: RawInitError = RawInitError(119);
pub const InitError_Init_TooManyObjects: RawInitError = RawInitError(120);
pub const InitError_Init_NoServerForBackgroundApp: RawInitError = RawInitError(121);
pub const InitError_Init_NotSupportedWithCompositor: RawInitError = RawInitError(122);
pub const InitError_Init_NotAvailableToUtilityApps: RawInitError = RawInitError(123);
pub const InitError_Init_Internal: RawInitError = RawInitError(124);
pub const InitError_Init_HmdDriverIdIsNone: RawInitError = RawInitError(125);
pub const InitError_Init_HmdNotFoundPresenceFailed: RawInitError = RawInitError(126);
pub const InitError_Init_VRMonitorNotFound: RawInitError = RawInitError(127);
pub const InitError_Init_VRMonitorStartupFailed: RawInitError = RawInitError(128);
pub const InitError_Init_LowPowerWatchdogNotSupported: RawInitError = RawInitError(129);
pub const InitError_Init_InvalidApplicationType: RawInitError = RawInitError(130);
pub const InitError_Init_NotAvailableToWatchdogApps: RawInitError = RawInitError(131);
pub const InitError_Init_WatchdogDisabledInSettings: RawInitError = RawInitError(132);
pub const InitError_Init_VRDashboardNotFound: RawInitError = RawInitError(133);
pub const InitError_Init_VRDashboardStartupFailed: RawInitError = RawInitError(134);
pub const InitError_Init_VRHomeNotFound: RawInitError = RawInitError(135);
pub const InitError_Init_VRHomeStartupFailed: RawInitError = RawInitError(136);
pub const InitError_Init_RebootingBusy: RawInitError = RawInitError(137);
pub const InitError_Init_FirmwareUpdateBusy: RawInitError = RawInitError(138);
pub const InitError_Init_FirmwareRecoveryBusy: RawInitError = RawInitError(139);
pub const InitError_Init_USBServiceBusy: RawInitError = RawInitError(140);
pub const InitError_Init_VRWebHelperStartupFailed: RawInitError = RawInitError(141);
pub const InitError_Init_TrackerManagerInitFailed: RawInitError = RawInitError(142);
pub const InitError_Driver_Failed: RawInitError = RawInitError(200);
pub const InitError_Driver_Unknown: RawInitError = RawInitError(201);
pub const InitError_Driver_HmdUnknown: RawInitError = RawInitError(202);
pub const InitError_Driver_NotLoaded: RawInitError = RawInitError(203);
pub const InitError_Driver_RuntimeOutOfDate: RawInitError = RawInitError(204);
pub const InitError_Driver_HmdInUse: RawInitError = RawInitError(205);
pub const InitError_Driver_NotCalibrated: RawInitError = RawInitError(206);
pub const InitError_Driver_CalibrationInvalid: RawInitError = RawInitError(207);
pub const InitError_Driver_HmdDisplayNotFound: RawInitError = RawInitError(208);
pub const InitError_Driver_TrackedDeviceInterfaceUnknown: RawInitError = RawInitError(209);
pub const InitError_Driver_HmdDriverIdOutOfBounds: RawInitError = RawInitError(211);
pub const InitError_Driver_HmdDisplayMirrored: RawInitError = RawInitError(212);
pub const InitError_IPC_ServerInitFailed: RawInitError = RawInitError(300);
pub const InitError_IPC_ConnectFailed: RawInitError = RawInitError(301);
pub const InitError_IPC_SharedStateInitFailed: RawInitError = RawInitError(302);
pub const InitError_IPC_CompositorInitFailed: RawInitError = RawInitError(303);
pub const InitError_IPC_MutexInitFailed: RawInitError = RawInitError(304);
pub const InitError_IPC_Failed: RawInitError = RawInitError(305);
pub const InitError_IPC_CompositorConnectFailed: RawInitError = RawInitError(306);
pub const InitError_IPC_CompositorInvalidConnectResponse: RawInitError = RawInitError(307);
pub const InitError_IPC_ConnectFailedAfterMultipleAttempts: RawInitError = RawInitError(308);
pub const InitError_Compositor_Failed: RawInitError = RawInitError(400);
pub const InitError_Compositor_D3D11HardwareRequired: RawInitError = RawInitError(401);
pub const InitError_Compositor_FirmwareRequiresUpdate: RawInitError = RawInitError(402);
pub const InitError_Compositor_OverlayInitFailed: RawInitError = RawInitError(403);
pub const InitError_Compositor_ScreenshotsInitFailed: RawInitError = RawInitError(404);
pub const InitError_Compositor_UnableToCreateDevice: RawInitError = RawInitError(405);
pub const InitError_VendorSpecific_UnableToConnectToOculusRuntime: RawInitError =
    RawInitError(1000);
pub const InitError_VendorSpecific_WindowsNotInDevMode: RawInitError = RawInitError(1001);
pub const InitError_VendorSpecific_HmdFound_CantOpenDevice: RawInitError = RawInitError(1101);
pub const InitError_VendorSpecific_HmdFound_UnableToRequestConfigStart: RawInitError =
    RawInitError(1102);
pub const InitError_VendorSpecific_HmdFound_NoStoredConfig: RawInitError = RawInitError(1103);
pub const InitError_VendorSpecific_HmdFound_ConfigTooBig: RawInitError = RawInitError(1104);
pub const InitError_VendorSpecific_HmdFound_ConfigTooSmall: RawInitError = RawInitError(1105);
pub const InitError_VendorSpecific_HmdFound_UnableToInitZLib: RawInitError = RawInitError(1106);
pub const InitError_VendorSpecific_HmdFound_CantReadFirmwareVersion: RawInitError =
    RawInitError(1107);
pub const InitError_VendorSpecific_HmdFound_UnableToSendUserDataStart: RawInitError =
    RawInitError(1108);
pub const InitError_VendorSpecific_HmdFound_UnableToGetUserDataStart: RawInitError =
    RawInitError(1109);
pub const InitError_VendorSpecific_HmdFound_UnableToGetUserDataNext: RawInitError =
    RawInitError(1110);
pub const InitError_VendorSpecific_HmdFound_UserDataAddressRange: RawInitError = RawInitError(1111);
pub const InitError_VendorSpecific_HmdFound_UserDataError: RawInitError = RawInitError(1112);
pub const InitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck: RawInitError =
    RawInitError(1113);
pub const InitError_Steam_SteamInstallationNotFound: RawInitError = RawInitError(2000);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InitError {
    None = 0,
    Unknown = 1,
    Init_InstallationNotFound = 100,
    Init_InstallationCorrupt = 101,
    Init_VRClientDLLNotFound = 102,
    Init_FileNotFound = 103,
    Init_FactoryNotFound = 104,
    Init_InterfaceNotFound = 105,
    Init_InvalidInterface = 106,
    Init_UserConfigDirectoryInvalid = 107,
    Init_HmdNotFound = 108,
    Init_NotInitialized = 109,
    Init_PathRegistryNotFound = 110,
    Init_NoConfigPath = 111,
    Init_NoLogPath = 112,
    Init_PathRegistryNotWritable = 113,
    Init_AppInfoInitFailed = 114,
    Init_Retry = 115,
    Init_InitCanceledByUser = 116,
    Init_AnotherAppLaunching = 117,
    Init_SettingsInitFailed = 118,
    Init_ShuttingDown = 119,
    Init_TooManyObjects = 120,
    Init_NoServerForBackgroundApp = 121,
    Init_NotSupportedWithCompositor = 122,
    Init_NotAvailableToUtilityApps = 123,
    Init_Internal = 124,
    Init_HmdDriverIdIsNone = 125,
    Init_HmdNotFoundPresenceFailed = 126,
    Init_VRMonitorNotFound = 127,
    Init_VRMonitorStartupFailed = 128,
    Init_LowPowerWatchdogNotSupported = 129,
    Init_InvalidApplicationType = 130,
    Init_NotAvailableToWatchdogApps = 131,
    Init_WatchdogDisabledInSettings = 132,
    Init_VRDashboardNotFound = 133,
    Init_VRDashboardStartupFailed = 134,
    Init_VRHomeNotFound = 135,
    Init_VRHomeStartupFailed = 136,
    Init_RebootingBusy = 137,
    Init_FirmwareUpdateBusy = 138,
    Init_FirmwareRecoveryBusy = 139,
    Init_USBServiceBusy = 140,
    Init_VRWebHelperStartupFailed = 141,
    Init_TrackerManagerInitFailed = 142,
    Driver_Failed = 200,
    Driver_Unknown = 201,
    Driver_HmdUnknown = 202,
    Driver_NotLoaded = 203,
    Driver_RuntimeOutOfDate = 204,
    Driver_HmdInUse = 205,
    Driver_NotCalibrated = 206,
    Driver_CalibrationInvalid = 207,
    Driver_HmdDisplayNotFound = 208,
    Driver_TrackedDeviceInterfaceUnknown = 209,
    Driver_HmdDriverIdOutOfBounds = 211,
    Driver_HmdDisplayMirrored = 212,
    IPC_ServerInitFailed = 300,
    IPC_ConnectFailed = 301,
    IPC_SharedStateInitFailed = 302,
    IPC_CompositorInitFailed = 303,
    IPC_MutexInitFailed = 304,
    IPC_Failed = 305,
    IPC_CompositorConnectFailed = 306,
    IPC_CompositorInvalidConnectResponse = 307,
    IPC_ConnectFailedAfterMultipleAttempts = 308,
    Compositor_Failed = 400,
    Compositor_D3D11HardwareRequired = 401,
    Compositor_FirmwareRequiresUpdate = 402,
    Compositor_OverlayInitFailed = 403,
    Compositor_ScreenshotsInitFailed = 404,
    Compositor_UnableToCreateDevice = 405,
    VendorSpecific_UnableToConnectToOculusRuntime = 1000,
    VendorSpecific_WindowsNotInDevMode = 1001,
    VendorSpecific_HmdFound_CantOpenDevice = 1101,
    VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102,
    VendorSpecific_HmdFound_NoStoredConfig = 1103,
    VendorSpecific_HmdFound_ConfigTooBig = 1104,
    VendorSpecific_HmdFound_ConfigTooSmall = 1105,
    VendorSpecific_HmdFound_UnableToInitZLib = 1106,
    VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107,
    VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108,
    VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109,
    VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110,
    VendorSpecific_HmdFound_UserDataAddressRange = 1111,
    VendorSpecific_HmdFound_UserDataError = 1112,
    VendorSpecific_HmdFound_ConfigFailedSanityCheck = 1113,
    Steam_SteamInstallationNotFound = 2000,
}

impl InitError {
    #[inline]
    fn from_raw(val: RawInitError) -> Option<Self> {
        match val {
            InitError_None => Some(InitError::None),
            InitError_Unknown => Some(InitError::Unknown),
            InitError_Init_InstallationNotFound => Some(InitError::Init_InstallationNotFound),
            InitError_Init_InstallationCorrupt => Some(InitError::Init_InstallationCorrupt),
            InitError_Init_VRClientDLLNotFound => Some(InitError::Init_VRClientDLLNotFound),
            InitError_Init_FileNotFound => Some(InitError::Init_FileNotFound),
            InitError_Init_FactoryNotFound => Some(InitError::Init_FactoryNotFound),
            InitError_Init_InterfaceNotFound => Some(InitError::Init_InterfaceNotFound),
            InitError_Init_InvalidInterface => Some(InitError::Init_InvalidInterface),
            InitError_Init_UserConfigDirectoryInvalid => {
                Some(InitError::Init_UserConfigDirectoryInvalid)
            }
            InitError_Init_HmdNotFound => Some(InitError::Init_HmdNotFound),
            InitError_Init_NotInitialized => Some(InitError::Init_NotInitialized),
            InitError_Init_PathRegistryNotFound => Some(InitError::Init_PathRegistryNotFound),
            InitError_Init_NoConfigPath => Some(InitError::Init_NoConfigPath),
            InitError_Init_NoLogPath => Some(InitError::Init_NoLogPath),
            InitError_Init_PathRegistryNotWritable => Some(InitError::Init_PathRegistryNotWritable),
            InitError_Init_AppInfoInitFailed => Some(InitError::Init_AppInfoInitFailed),
            InitError_Init_Retry => Some(InitError::Init_Retry),
            InitError_Init_InitCanceledByUser => Some(InitError::Init_InitCanceledByUser),
            InitError_Init_AnotherAppLaunching => Some(InitError::Init_AnotherAppLaunching),
            InitError_Init_SettingsInitFailed => Some(InitError::Init_SettingsInitFailed),
            InitError_Init_ShuttingDown => Some(InitError::Init_ShuttingDown),
            InitError_Init_TooManyObjects => Some(InitError::Init_TooManyObjects),
            InitError_Init_NoServerForBackgroundApp => {
                Some(InitError::Init_NoServerForBackgroundApp)
            }
            InitError_Init_NotSupportedWithCompositor => {
                Some(InitError::Init_NotSupportedWithCompositor)
            }
            InitError_Init_NotAvailableToUtilityApps => {
                Some(InitError::Init_NotAvailableToUtilityApps)
            }
            InitError_Init_Internal => Some(InitError::Init_Internal),
            InitError_Init_HmdDriverIdIsNone => Some(InitError::Init_HmdDriverIdIsNone),
            InitError_Init_HmdNotFoundPresenceFailed => {
                Some(InitError::Init_HmdNotFoundPresenceFailed)
            }
            InitError_Init_VRMonitorNotFound => Some(InitError::Init_VRMonitorNotFound),
            InitError_Init_VRMonitorStartupFailed => Some(InitError::Init_VRMonitorStartupFailed),
            InitError_Init_LowPowerWatchdogNotSupported => {
                Some(InitError::Init_LowPowerWatchdogNotSupported)
            }
            InitError_Init_InvalidApplicationType => Some(InitError::Init_InvalidApplicationType),
            InitError_Init_NotAvailableToWatchdogApps => {
                Some(InitError::Init_NotAvailableToWatchdogApps)
            }
            InitError_Init_WatchdogDisabledInSettings => {
                Some(InitError::Init_WatchdogDisabledInSettings)
            }
            InitError_Init_VRDashboardNotFound => Some(InitError::Init_VRDashboardNotFound),
            InitError_Init_VRDashboardStartupFailed => {
                Some(InitError::Init_VRDashboardStartupFailed)
            }
            InitError_Init_VRHomeNotFound => Some(InitError::Init_VRHomeNotFound),
            InitError_Init_VRHomeStartupFailed => Some(InitError::Init_VRHomeStartupFailed),
            InitError_Init_RebootingBusy => Some(InitError::Init_RebootingBusy),
            InitError_Init_FirmwareUpdateBusy => Some(InitError::Init_FirmwareUpdateBusy),
            InitError_Init_FirmwareRecoveryBusy => Some(InitError::Init_FirmwareRecoveryBusy),
            InitError_Init_USBServiceBusy => Some(InitError::Init_USBServiceBusy),
            InitError_Init_VRWebHelperStartupFailed => {
                Some(InitError::Init_VRWebHelperStartupFailed)
            }
            InitError_Init_TrackerManagerInitFailed => {
                Some(InitError::Init_TrackerManagerInitFailed)
            }
            InitError_Driver_Failed => Some(InitError::Driver_Failed),
            InitError_Driver_Unknown => Some(InitError::Driver_Unknown),
            InitError_Driver_HmdUnknown => Some(InitError::Driver_HmdUnknown),
            InitError_Driver_NotLoaded => Some(InitError::Driver_NotLoaded),
            InitError_Driver_RuntimeOutOfDate => Some(InitError::Driver_RuntimeOutOfDate),
            InitError_Driver_HmdInUse => Some(InitError::Driver_HmdInUse),
            InitError_Driver_NotCalibrated => Some(InitError::Driver_NotCalibrated),
            InitError_Driver_CalibrationInvalid => Some(InitError::Driver_CalibrationInvalid),
            InitError_Driver_HmdDisplayNotFound => Some(InitError::Driver_HmdDisplayNotFound),
            InitError_Driver_TrackedDeviceInterfaceUnknown => {
                Some(InitError::Driver_TrackedDeviceInterfaceUnknown)
            }
            InitError_Driver_HmdDriverIdOutOfBounds => {
                Some(InitError::Driver_HmdDriverIdOutOfBounds)
            }
            InitError_Driver_HmdDisplayMirrored => Some(InitError::Driver_HmdDisplayMirrored),
            InitError_IPC_ServerInitFailed => Some(InitError::IPC_ServerInitFailed),
            InitError_IPC_ConnectFailed => Some(InitError::IPC_ConnectFailed),
            InitError_IPC_SharedStateInitFailed => Some(InitError::IPC_SharedStateInitFailed),
            InitError_IPC_CompositorInitFailed => Some(InitError::IPC_CompositorInitFailed),
            InitError_IPC_MutexInitFailed => Some(InitError::IPC_MutexInitFailed),
            InitError_IPC_Failed => Some(InitError::IPC_Failed),
            InitError_IPC_CompositorConnectFailed => Some(InitError::IPC_CompositorConnectFailed),
            InitError_IPC_CompositorInvalidConnectResponse => {
                Some(InitError::IPC_CompositorInvalidConnectResponse)
            }
            InitError_IPC_ConnectFailedAfterMultipleAttempts => {
                Some(InitError::IPC_ConnectFailedAfterMultipleAttempts)
            }
            InitError_Compositor_Failed => Some(InitError::Compositor_Failed),
            InitError_Compositor_D3D11HardwareRequired => {
                Some(InitError::Compositor_D3D11HardwareRequired)
            }
            InitError_Compositor_FirmwareRequiresUpdate => {
                Some(InitError::Compositor_FirmwareRequiresUpdate)
            }
            InitError_Compositor_OverlayInitFailed => Some(InitError::Compositor_OverlayInitFailed),
            InitError_Compositor_ScreenshotsInitFailed => {
                Some(InitError::Compositor_ScreenshotsInitFailed)
            }
            InitError_Compositor_UnableToCreateDevice => {
                Some(InitError::Compositor_UnableToCreateDevice)
            }
            InitError_VendorSpecific_UnableToConnectToOculusRuntime => {
                Some(InitError::VendorSpecific_UnableToConnectToOculusRuntime)
            }
            InitError_VendorSpecific_WindowsNotInDevMode => {
                Some(InitError::VendorSpecific_WindowsNotInDevMode)
            }
            InitError_VendorSpecific_HmdFound_CantOpenDevice => {
                Some(InitError::VendorSpecific_HmdFound_CantOpenDevice)
            }
            InitError_VendorSpecific_HmdFound_UnableToRequestConfigStart => {
                Some(InitError::VendorSpecific_HmdFound_UnableToRequestConfigStart)
            }
            InitError_VendorSpecific_HmdFound_NoStoredConfig => {
                Some(InitError::VendorSpecific_HmdFound_NoStoredConfig)
            }
            InitError_VendorSpecific_HmdFound_ConfigTooBig => {
                Some(InitError::VendorSpecific_HmdFound_ConfigTooBig)
            }
            InitError_VendorSpecific_HmdFound_ConfigTooSmall => {
                Some(InitError::VendorSpecific_HmdFound_ConfigTooSmall)
            }
            InitError_VendorSpecific_HmdFound_UnableToInitZLib => {
                Some(InitError::VendorSpecific_HmdFound_UnableToInitZLib)
            }
            InitError_VendorSpecific_HmdFound_CantReadFirmwareVersion => {
                Some(InitError::VendorSpecific_HmdFound_CantReadFirmwareVersion)
            }
            InitError_VendorSpecific_HmdFound_UnableToSendUserDataStart => {
                Some(InitError::VendorSpecific_HmdFound_UnableToSendUserDataStart)
            }
            InitError_VendorSpecific_HmdFound_UnableToGetUserDataStart => {
                Some(InitError::VendorSpecific_HmdFound_UnableToGetUserDataStart)
            }
            InitError_VendorSpecific_HmdFound_UnableToGetUserDataNext => {
                Some(InitError::VendorSpecific_HmdFound_UnableToGetUserDataNext)
            }
            InitError_VendorSpecific_HmdFound_UserDataAddressRange => {
                Some(InitError::VendorSpecific_HmdFound_UserDataAddressRange)
            }
            InitError_VendorSpecific_HmdFound_UserDataError => {
                Some(InitError::VendorSpecific_HmdFound_UserDataError)
            }
            InitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck => {
                Some(InitError::VendorSpecific_HmdFound_ConfigFailedSanityCheck)
            }
            InitError_Steam_SteamInstallationNotFound => {
                Some(InitError::Steam_SteamInstallationNotFound)
            }
            _ => None,
        }
    }
}

impl From<RawInitError> for InitError {
    fn from(val: RawInitError) -> Self {
        InitError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for InitError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawScreenshotType(pub u32);

pub const ScreenshotType_None: RawScreenshotType = RawScreenshotType(0);
pub const ScreenshotType_Mono: RawScreenshotType = RawScreenshotType(1);
pub const ScreenshotType_Stereo: RawScreenshotType = RawScreenshotType(2);
pub const ScreenshotType_Cubemap: RawScreenshotType = RawScreenshotType(3);
pub const ScreenshotType_MonoPanorama: RawScreenshotType = RawScreenshotType(4);
pub const ScreenshotType_StereoPanorama: RawScreenshotType = RawScreenshotType(5);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ScreenshotType {
    None = 0,
    Mono = 1,
    Stereo = 2,
    Cubemap = 3,
    MonoPanorama = 4,
    StereoPanorama = 5,
}

impl ScreenshotType {
    #[inline]
    fn from_raw(val: RawScreenshotType) -> Option<Self> {
        match val {
            ScreenshotType_None => Some(ScreenshotType::None),
            ScreenshotType_Mono => Some(ScreenshotType::Mono),
            ScreenshotType_Stereo => Some(ScreenshotType::Stereo),
            ScreenshotType_Cubemap => Some(ScreenshotType::Cubemap),
            ScreenshotType_MonoPanorama => Some(ScreenshotType::MonoPanorama),
            ScreenshotType_StereoPanorama => Some(ScreenshotType::StereoPanorama),
            _ => None,
        }
    }
}

impl From<RawScreenshotType> for ScreenshotType {
    fn from(val: RawScreenshotType) -> Self {
        ScreenshotType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ScreenshotType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawScreenshotPropertyFilenames(pub u32);

pub const ScreenshotPropertyFilenames_Preview: RawScreenshotPropertyFilenames =
    RawScreenshotPropertyFilenames(0);
pub const ScreenshotPropertyFilenames_VR: RawScreenshotPropertyFilenames =
    RawScreenshotPropertyFilenames(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ScreenshotPropertyFilenames {
    Preview = 0,
    VR = 1,
}

impl ScreenshotPropertyFilenames {
    #[inline]
    fn from_raw(val: RawScreenshotPropertyFilenames) -> Option<Self> {
        match val {
            ScreenshotPropertyFilenames_Preview => Some(ScreenshotPropertyFilenames::Preview),
            ScreenshotPropertyFilenames_VR => Some(ScreenshotPropertyFilenames::VR),
            _ => None,
        }
    }
}

impl From<RawScreenshotPropertyFilenames> for ScreenshotPropertyFilenames {
    fn from(val: RawScreenshotPropertyFilenames) -> Self {
        ScreenshotPropertyFilenames::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ScreenshotPropertyFilenames.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackedCameraError(pub u32);

pub const TrackedCameraError_None: RawTrackedCameraError = RawTrackedCameraError(0);
pub const TrackedCameraError_OperationFailed: RawTrackedCameraError = RawTrackedCameraError(100);
pub const TrackedCameraError_InvalidHandle: RawTrackedCameraError = RawTrackedCameraError(101);
pub const TrackedCameraError_InvalidFrameHeaderVersion: RawTrackedCameraError =
    RawTrackedCameraError(102);
pub const TrackedCameraError_OutOfHandles: RawTrackedCameraError = RawTrackedCameraError(103);
pub const TrackedCameraError_IPCFailure: RawTrackedCameraError = RawTrackedCameraError(104);
pub const TrackedCameraError_NotSupportedForThisDevice: RawTrackedCameraError =
    RawTrackedCameraError(105);
pub const TrackedCameraError_SharedMemoryFailure: RawTrackedCameraError =
    RawTrackedCameraError(106);
pub const TrackedCameraError_FrameBufferingFailure: RawTrackedCameraError =
    RawTrackedCameraError(107);
pub const TrackedCameraError_StreamSetupFailure: RawTrackedCameraError = RawTrackedCameraError(108);
pub const TrackedCameraError_InvalidGLTextureId: RawTrackedCameraError = RawTrackedCameraError(109);
pub const TrackedCameraError_InvalidSharedTextureHandle: RawTrackedCameraError =
    RawTrackedCameraError(110);
pub const TrackedCameraError_FailedToGetGLTextureId: RawTrackedCameraError =
    RawTrackedCameraError(111);
pub const TrackedCameraError_SharedTextureFailure: RawTrackedCameraError =
    RawTrackedCameraError(112);
pub const TrackedCameraError_NoFrameAvailable: RawTrackedCameraError = RawTrackedCameraError(113);
pub const TrackedCameraError_InvalidArgument: RawTrackedCameraError = RawTrackedCameraError(114);
pub const TrackedCameraError_InvalidFrameBufferSize: RawTrackedCameraError =
    RawTrackedCameraError(115);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedCameraError {
    None = 0,
    OperationFailed = 100,
    InvalidHandle = 101,
    InvalidFrameHeaderVersion = 102,
    OutOfHandles = 103,
    IPCFailure = 104,
    NotSupportedForThisDevice = 105,
    SharedMemoryFailure = 106,
    FrameBufferingFailure = 107,
    StreamSetupFailure = 108,
    InvalidGLTextureId = 109,
    InvalidSharedTextureHandle = 110,
    FailedToGetGLTextureId = 111,
    SharedTextureFailure = 112,
    NoFrameAvailable = 113,
    InvalidArgument = 114,
    InvalidFrameBufferSize = 115,
}

impl TrackedCameraError {
    #[inline]
    fn from_raw(val: RawTrackedCameraError) -> Option<Self> {
        match val {
            TrackedCameraError_None => Some(TrackedCameraError::None),
            TrackedCameraError_OperationFailed => Some(TrackedCameraError::OperationFailed),
            TrackedCameraError_InvalidHandle => Some(TrackedCameraError::InvalidHandle),
            TrackedCameraError_InvalidFrameHeaderVersion => {
                Some(TrackedCameraError::InvalidFrameHeaderVersion)
            }
            TrackedCameraError_OutOfHandles => Some(TrackedCameraError::OutOfHandles),
            TrackedCameraError_IPCFailure => Some(TrackedCameraError::IPCFailure),
            TrackedCameraError_NotSupportedForThisDevice => {
                Some(TrackedCameraError::NotSupportedForThisDevice)
            }
            TrackedCameraError_SharedMemoryFailure => Some(TrackedCameraError::SharedMemoryFailure),
            TrackedCameraError_FrameBufferingFailure => {
                Some(TrackedCameraError::FrameBufferingFailure)
            }
            TrackedCameraError_StreamSetupFailure => Some(TrackedCameraError::StreamSetupFailure),
            TrackedCameraError_InvalidGLTextureId => Some(TrackedCameraError::InvalidGLTextureId),
            TrackedCameraError_InvalidSharedTextureHandle => {
                Some(TrackedCameraError::InvalidSharedTextureHandle)
            }
            TrackedCameraError_FailedToGetGLTextureId => {
                Some(TrackedCameraError::FailedToGetGLTextureId)
            }
            TrackedCameraError_SharedTextureFailure => {
                Some(TrackedCameraError::SharedTextureFailure)
            }
            TrackedCameraError_NoFrameAvailable => Some(TrackedCameraError::NoFrameAvailable),
            TrackedCameraError_InvalidArgument => Some(TrackedCameraError::InvalidArgument),
            TrackedCameraError_InvalidFrameBufferSize => {
                Some(TrackedCameraError::InvalidFrameBufferSize)
            }
            _ => None,
        }
    }
}

impl From<RawTrackedCameraError> for TrackedCameraError {
    fn from(val: RawTrackedCameraError) -> Self {
        TrackedCameraError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackedCameraError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackedCameraFrameLayout(pub u32);

pub const TrackedCameraFrameLayout_Mono: RawTrackedCameraFrameLayout =
    RawTrackedCameraFrameLayout(1);
pub const TrackedCameraFrameLayout_Stereo: RawTrackedCameraFrameLayout =
    RawTrackedCameraFrameLayout(2);
pub const TrackedCameraFrameLayout_VerticalLayout: RawTrackedCameraFrameLayout =
    RawTrackedCameraFrameLayout(16);
pub const TrackedCameraFrameLayout_HorizontalLayout: RawTrackedCameraFrameLayout =
    RawTrackedCameraFrameLayout(32);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedCameraFrameLayout {
    Mono = 1,
    Stereo = 2,
    VerticalLayout = 16,
    HorizontalLayout = 32,
}

impl TrackedCameraFrameLayout {
    #[inline]
    fn from_raw(val: RawTrackedCameraFrameLayout) -> Option<Self> {
        match val {
            TrackedCameraFrameLayout_Mono => Some(TrackedCameraFrameLayout::Mono),
            TrackedCameraFrameLayout_Stereo => Some(TrackedCameraFrameLayout::Stereo),
            TrackedCameraFrameLayout_VerticalLayout => {
                Some(TrackedCameraFrameLayout::VerticalLayout)
            }
            TrackedCameraFrameLayout_HorizontalLayout => {
                Some(TrackedCameraFrameLayout::HorizontalLayout)
            }
            _ => None,
        }
    }
}

impl From<RawTrackedCameraFrameLayout> for TrackedCameraFrameLayout {
    fn from(val: RawTrackedCameraFrameLayout) -> Self {
        TrackedCameraFrameLayout::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackedCameraFrameLayout.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawTrackedCameraFrameType(pub u32);

pub const TrackedCameraFrameType_Distorted: RawTrackedCameraFrameType =
    RawTrackedCameraFrameType(0);
pub const TrackedCameraFrameType_Undistorted: RawTrackedCameraFrameType =
    RawTrackedCameraFrameType(1);
pub const TrackedCameraFrameType_MaximumUndistorted: RawTrackedCameraFrameType =
    RawTrackedCameraFrameType(2);
pub const TrackedCameraFrameType_CAMERA_FRAME_TYPES: RawTrackedCameraFrameType =
    RawTrackedCameraFrameType(3);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedCameraFrameType {
    Distorted = 0,
    Undistorted = 1,
    MaximumUndistorted = 2,
    CAMERA_FRAME_TYPES = 3,
}

impl TrackedCameraFrameType {
    #[inline]
    fn from_raw(val: RawTrackedCameraFrameType) -> Option<Self> {
        match val {
            TrackedCameraFrameType_Distorted => Some(TrackedCameraFrameType::Distorted),
            TrackedCameraFrameType_Undistorted => Some(TrackedCameraFrameType::Undistorted),
            TrackedCameraFrameType_MaximumUndistorted => {
                Some(TrackedCameraFrameType::MaximumUndistorted)
            }
            TrackedCameraFrameType_CAMERA_FRAME_TYPES => {
                Some(TrackedCameraFrameType::CAMERA_FRAME_TYPES)
            }
            _ => None,
        }
    }
}

impl From<RawTrackedCameraFrameType> for TrackedCameraFrameType {
    fn from(val: RawTrackedCameraFrameType) -> Self {
        TrackedCameraFrameType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for TrackedCameraFrameType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawDistortionFunctionType(pub u32);

pub const DistortionFunctionType_None: RawDistortionFunctionType = RawDistortionFunctionType(0);
pub const DistortionFunctionType_FTheta: RawDistortionFunctionType = RawDistortionFunctionType(1);
pub const DistortionFunctionType_Extended_FTheta: RawDistortionFunctionType =
    RawDistortionFunctionType(2);
pub const DistortionFunctionType_DISTORTION_FUNCTION_TYPES: RawDistortionFunctionType =
    RawDistortionFunctionType(3);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DistortionFunctionType {
    None = 0,
    FTheta = 1,
    Extended_FTheta = 2,
    DISTORTION_FUNCTION_TYPES = 3,
}

impl DistortionFunctionType {
    #[inline]
    fn from_raw(val: RawDistortionFunctionType) -> Option<Self> {
        match val {
            DistortionFunctionType_None => Some(DistortionFunctionType::None),
            DistortionFunctionType_FTheta => Some(DistortionFunctionType::FTheta),
            DistortionFunctionType_Extended_FTheta => Some(DistortionFunctionType::Extended_FTheta),
            DistortionFunctionType_DISTORTION_FUNCTION_TYPES => {
                Some(DistortionFunctionType::DISTORTION_FUNCTION_TYPES)
            }
            _ => None,
        }
    }
}

impl From<RawDistortionFunctionType> for DistortionFunctionType {
    fn from(val: RawDistortionFunctionType) -> Self {
        DistortionFunctionType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for DistortionFunctionType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawVSync(pub u32);

pub const VSync_None: RawVSync = RawVSync(0);
pub const VSync_WaitRender: RawVSync = RawVSync(1);
pub const VSync_NoWaitRender: RawVSync = RawVSync(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum VSync {
    None = 0,
    WaitRender = 1,
    NoWaitRender = 2,
}

impl VSync {
    #[inline]
    fn from_raw(val: RawVSync) -> Option<Self> {
        match val {
            VSync_None => Some(VSync::None),
            VSync_WaitRender => Some(VSync::WaitRender),
            VSync_NoWaitRender => Some(VSync::NoWaitRender),
            _ => None,
        }
    }
}

impl From<RawVSync> for VSync {
    fn from(val: RawVSync) -> Self {
        VSync::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for VSync.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawMuraCorrectionMode(pub u32);

pub const MuraCorrectionMode_Default: RawMuraCorrectionMode = RawMuraCorrectionMode(0);
pub const MuraCorrectionMode_NoCorrection: RawMuraCorrectionMode = RawMuraCorrectionMode(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MuraCorrectionMode {
    Default = 0,
    NoCorrection = 1,
}

impl MuraCorrectionMode {
    #[inline]
    fn from_raw(val: RawMuraCorrectionMode) -> Option<Self> {
        match val {
            MuraCorrectionMode_Default => Some(MuraCorrectionMode::Default),
            MuraCorrectionMode_NoCorrection => Some(MuraCorrectionMode::NoCorrection),
            _ => None,
        }
    }
}

impl From<RawMuraCorrectionMode> for MuraCorrectionMode {
    fn from(val: RawMuraCorrectionMode) -> Self {
        MuraCorrectionMode::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for MuraCorrectionMode.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawImu_OffScaleFlags(pub u32);

pub const Imu_OffScaleFlags_AccelX: RawImu_OffScaleFlags = RawImu_OffScaleFlags(1);
pub const Imu_OffScaleFlags_AccelY: RawImu_OffScaleFlags = RawImu_OffScaleFlags(2);
pub const Imu_OffScaleFlags_AccelZ: RawImu_OffScaleFlags = RawImu_OffScaleFlags(4);
pub const Imu_OffScaleFlags_GyroX: RawImu_OffScaleFlags = RawImu_OffScaleFlags(8);
pub const Imu_OffScaleFlags_GyroY: RawImu_OffScaleFlags = RawImu_OffScaleFlags(16);
pub const Imu_OffScaleFlags_GyroZ: RawImu_OffScaleFlags = RawImu_OffScaleFlags(32);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Imu_OffScaleFlags {
    AccelX = 1,
    AccelY = 2,
    AccelZ = 4,
    GyroX = 8,
    GyroY = 16,
    GyroZ = 32,
}

impl Imu_OffScaleFlags {
    #[inline]
    fn from_raw(val: RawImu_OffScaleFlags) -> Option<Self> {
        match val {
            Imu_OffScaleFlags_AccelX => Some(Imu_OffScaleFlags::AccelX),
            Imu_OffScaleFlags_AccelY => Some(Imu_OffScaleFlags::AccelY),
            Imu_OffScaleFlags_AccelZ => Some(Imu_OffScaleFlags::AccelZ),
            Imu_OffScaleFlags_GyroX => Some(Imu_OffScaleFlags::GyroX),
            Imu_OffScaleFlags_GyroY => Some(Imu_OffScaleFlags::GyroY),
            Imu_OffScaleFlags_GyroZ => Some(Imu_OffScaleFlags::GyroZ),
            _ => None,
        }
    }
}

impl From<RawImu_OffScaleFlags> for Imu_OffScaleFlags {
    fn from(val: RawImu_OffScaleFlags) -> Self {
        Imu_OffScaleFlags::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for Imu_OffScaleFlags.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawApplicationError(pub u32);

pub const ApplicationError_None: RawApplicationError = RawApplicationError(0);
pub const ApplicationError_AppKeyAlreadyExists: RawApplicationError = RawApplicationError(100);
pub const ApplicationError_NoManifest: RawApplicationError = RawApplicationError(101);
pub const ApplicationError_NoApplication: RawApplicationError = RawApplicationError(102);
pub const ApplicationError_InvalidIndex: RawApplicationError = RawApplicationError(103);
pub const ApplicationError_UnknownApplication: RawApplicationError = RawApplicationError(104);
pub const ApplicationError_IPCFailed: RawApplicationError = RawApplicationError(105);
pub const ApplicationError_ApplicationAlreadyRunning: RawApplicationError =
    RawApplicationError(106);
pub const ApplicationError_InvalidManifest: RawApplicationError = RawApplicationError(107);
pub const ApplicationError_InvalidApplication: RawApplicationError = RawApplicationError(108);
pub const ApplicationError_LaunchFailed: RawApplicationError = RawApplicationError(109);
pub const ApplicationError_ApplicationAlreadyStarting: RawApplicationError =
    RawApplicationError(110);
pub const ApplicationError_LaunchInProgress: RawApplicationError = RawApplicationError(111);
pub const ApplicationError_OldApplicationQuitting: RawApplicationError = RawApplicationError(112);
pub const ApplicationError_TransitionAborted: RawApplicationError = RawApplicationError(113);
pub const ApplicationError_IsTemplate: RawApplicationError = RawApplicationError(114);
pub const ApplicationError_SteamVRIsExiting: RawApplicationError = RawApplicationError(115);
pub const ApplicationError_BufferTooSmall: RawApplicationError = RawApplicationError(200);
pub const ApplicationError_PropertyNotSet: RawApplicationError = RawApplicationError(201);
pub const ApplicationError_UnknownProperty: RawApplicationError = RawApplicationError(202);
pub const ApplicationError_InvalidParameter: RawApplicationError = RawApplicationError(203);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationError {
    None = 0,
    AppKeyAlreadyExists = 100,
    NoManifest = 101,
    NoApplication = 102,
    InvalidIndex = 103,
    UnknownApplication = 104,
    IPCFailed = 105,
    ApplicationAlreadyRunning = 106,
    InvalidManifest = 107,
    InvalidApplication = 108,
    LaunchFailed = 109,
    ApplicationAlreadyStarting = 110,
    LaunchInProgress = 111,
    OldApplicationQuitting = 112,
    TransitionAborted = 113,
    IsTemplate = 114,
    SteamVRIsExiting = 115,
    BufferTooSmall = 200,
    PropertyNotSet = 201,
    UnknownProperty = 202,
    InvalidParameter = 203,
}

impl ApplicationError {
    #[inline]
    fn from_raw(val: RawApplicationError) -> Option<Self> {
        match val {
            ApplicationError_None => Some(ApplicationError::None),
            ApplicationError_AppKeyAlreadyExists => Some(ApplicationError::AppKeyAlreadyExists),
            ApplicationError_NoManifest => Some(ApplicationError::NoManifest),
            ApplicationError_NoApplication => Some(ApplicationError::NoApplication),
            ApplicationError_InvalidIndex => Some(ApplicationError::InvalidIndex),
            ApplicationError_UnknownApplication => Some(ApplicationError::UnknownApplication),
            ApplicationError_IPCFailed => Some(ApplicationError::IPCFailed),
            ApplicationError_ApplicationAlreadyRunning => {
                Some(ApplicationError::ApplicationAlreadyRunning)
            }
            ApplicationError_InvalidManifest => Some(ApplicationError::InvalidManifest),
            ApplicationError_InvalidApplication => Some(ApplicationError::InvalidApplication),
            ApplicationError_LaunchFailed => Some(ApplicationError::LaunchFailed),
            ApplicationError_ApplicationAlreadyStarting => {
                Some(ApplicationError::ApplicationAlreadyStarting)
            }
            ApplicationError_LaunchInProgress => Some(ApplicationError::LaunchInProgress),
            ApplicationError_OldApplicationQuitting => {
                Some(ApplicationError::OldApplicationQuitting)
            }
            ApplicationError_TransitionAborted => Some(ApplicationError::TransitionAborted),
            ApplicationError_IsTemplate => Some(ApplicationError::IsTemplate),
            ApplicationError_SteamVRIsExiting => Some(ApplicationError::SteamVRIsExiting),
            ApplicationError_BufferTooSmall => Some(ApplicationError::BufferTooSmall),
            ApplicationError_PropertyNotSet => Some(ApplicationError::PropertyNotSet),
            ApplicationError_UnknownProperty => Some(ApplicationError::UnknownProperty),
            ApplicationError_InvalidParameter => Some(ApplicationError::InvalidParameter),
            _ => None,
        }
    }
}

impl From<RawApplicationError> for ApplicationError {
    fn from(val: RawApplicationError) -> Self {
        ApplicationError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ApplicationError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawApplicationProperty(pub u32);

pub const ApplicationProperty_Name_String: RawApplicationProperty = RawApplicationProperty(0);
pub const ApplicationProperty_LaunchType_String: RawApplicationProperty =
    RawApplicationProperty(11);
pub const ApplicationProperty_WorkingDirectory_String: RawApplicationProperty =
    RawApplicationProperty(12);
pub const ApplicationProperty_BinaryPath_String: RawApplicationProperty =
    RawApplicationProperty(13);
pub const ApplicationProperty_Arguments_String: RawApplicationProperty = RawApplicationProperty(14);
pub const ApplicationProperty_URL_String: RawApplicationProperty = RawApplicationProperty(15);
pub const ApplicationProperty_Description_String: RawApplicationProperty =
    RawApplicationProperty(50);
pub const ApplicationProperty_NewsURL_String: RawApplicationProperty = RawApplicationProperty(51);
pub const ApplicationProperty_ImagePath_String: RawApplicationProperty = RawApplicationProperty(52);
pub const ApplicationProperty_Source_String: RawApplicationProperty = RawApplicationProperty(53);
pub const ApplicationProperty_ActionManifestURL_String: RawApplicationProperty =
    RawApplicationProperty(54);
pub const ApplicationProperty_IsDashboardOverlay_Bool: RawApplicationProperty =
    RawApplicationProperty(60);
pub const ApplicationProperty_IsTemplate_Bool: RawApplicationProperty = RawApplicationProperty(61);
pub const ApplicationProperty_IsInstanced_Bool: RawApplicationProperty = RawApplicationProperty(62);
pub const ApplicationProperty_IsInternal_Bool: RawApplicationProperty = RawApplicationProperty(63);
pub const ApplicationProperty_WantsCompositorPauseInStandby_Bool: RawApplicationProperty =
    RawApplicationProperty(64);
pub const ApplicationProperty_LastLaunchTime_Uint64: RawApplicationProperty =
    RawApplicationProperty(70);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationProperty {
    Name_String = 0,
    LaunchType_String = 11,
    WorkingDirectory_String = 12,
    BinaryPath_String = 13,
    Arguments_String = 14,
    URL_String = 15,
    Description_String = 50,
    NewsURL_String = 51,
    ImagePath_String = 52,
    Source_String = 53,
    ActionManifestURL_String = 54,
    IsDashboardOverlay_Bool = 60,
    IsTemplate_Bool = 61,
    IsInstanced_Bool = 62,
    IsInternal_Bool = 63,
    WantsCompositorPauseInStandby_Bool = 64,
    LastLaunchTime_Uint64 = 70,
}

impl ApplicationProperty {
    #[inline]
    fn from_raw(val: RawApplicationProperty) -> Option<Self> {
        match val {
            ApplicationProperty_Name_String => Some(ApplicationProperty::Name_String),
            ApplicationProperty_LaunchType_String => Some(ApplicationProperty::LaunchType_String),
            ApplicationProperty_WorkingDirectory_String => {
                Some(ApplicationProperty::WorkingDirectory_String)
            }
            ApplicationProperty_BinaryPath_String => Some(ApplicationProperty::BinaryPath_String),
            ApplicationProperty_Arguments_String => Some(ApplicationProperty::Arguments_String),
            ApplicationProperty_URL_String => Some(ApplicationProperty::URL_String),
            ApplicationProperty_Description_String => Some(ApplicationProperty::Description_String),
            ApplicationProperty_NewsURL_String => Some(ApplicationProperty::NewsURL_String),
            ApplicationProperty_ImagePath_String => Some(ApplicationProperty::ImagePath_String),
            ApplicationProperty_Source_String => Some(ApplicationProperty::Source_String),
            ApplicationProperty_ActionManifestURL_String => {
                Some(ApplicationProperty::ActionManifestURL_String)
            }
            ApplicationProperty_IsDashboardOverlay_Bool => {
                Some(ApplicationProperty::IsDashboardOverlay_Bool)
            }
            ApplicationProperty_IsTemplate_Bool => Some(ApplicationProperty::IsTemplate_Bool),
            ApplicationProperty_IsInstanced_Bool => Some(ApplicationProperty::IsInstanced_Bool),
            ApplicationProperty_IsInternal_Bool => Some(ApplicationProperty::IsInternal_Bool),
            ApplicationProperty_WantsCompositorPauseInStandby_Bool => {
                Some(ApplicationProperty::WantsCompositorPauseInStandby_Bool)
            }
            ApplicationProperty_LastLaunchTime_Uint64 => {
                Some(ApplicationProperty::LastLaunchTime_Uint64)
            }
            _ => None,
        }
    }
}

impl From<RawApplicationProperty> for ApplicationProperty {
    fn from(val: RawApplicationProperty) -> Self {
        ApplicationProperty::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ApplicationProperty.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawApplicationTransitionState(pub u32);

pub const ApplicationTransitionState_None: RawApplicationTransitionState =
    RawApplicationTransitionState(0);
pub const ApplicationTransitionState_OldAppQuitSent: RawApplicationTransitionState =
    RawApplicationTransitionState(10);
pub const ApplicationTransitionState_WaitingForExternalLaunch: RawApplicationTransitionState =
    RawApplicationTransitionState(11);
pub const ApplicationTransitionState_NewAppLaunched: RawApplicationTransitionState =
    RawApplicationTransitionState(20);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationTransitionState {
    None = 0,
    OldAppQuitSent = 10,
    WaitingForExternalLaunch = 11,
    NewAppLaunched = 20,
}

impl ApplicationTransitionState {
    #[inline]
    fn from_raw(val: RawApplicationTransitionState) -> Option<Self> {
        match val {
            ApplicationTransitionState_None => Some(ApplicationTransitionState::None),
            ApplicationTransitionState_OldAppQuitSent => {
                Some(ApplicationTransitionState::OldAppQuitSent)
            }
            ApplicationTransitionState_WaitingForExternalLaunch => {
                Some(ApplicationTransitionState::WaitingForExternalLaunch)
            }
            ApplicationTransitionState_NewAppLaunched => {
                Some(ApplicationTransitionState::NewAppLaunched)
            }
            _ => None,
        }
    }
}

impl From<RawApplicationTransitionState> for ApplicationTransitionState {
    fn from(val: RawApplicationTransitionState) -> Self {
        ApplicationTransitionState::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ApplicationTransitionState.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawChaperoneCalibrationState(pub u32);

pub const ChaperoneCalibrationState_OK: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(1);
pub const ChaperoneCalibrationState_Warning: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(100);
pub const ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(101);
pub const ChaperoneCalibrationState_Warning_BaseStationRemoved: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(102);
pub const ChaperoneCalibrationState_Warning_SeatedBoundsInvalid: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(103);
pub const ChaperoneCalibrationState_Error: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(200);
pub const ChaperoneCalibrationState_Error_BaseStationUninitialized: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(201);
pub const ChaperoneCalibrationState_Error_BaseStationConflict: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(202);
pub const ChaperoneCalibrationState_Error_PlayAreaInvalid: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(203);
pub const ChaperoneCalibrationState_Error_CollisionBoundsInvalid: RawChaperoneCalibrationState =
    RawChaperoneCalibrationState(204);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChaperoneCalibrationState {
    OK = 1,
    Warning = 100,
    Warning_BaseStationMayHaveMoved = 101,
    Warning_BaseStationRemoved = 102,
    Warning_SeatedBoundsInvalid = 103,
    Error = 200,
    Error_BaseStationUninitialized = 201,
    Error_BaseStationConflict = 202,
    Error_PlayAreaInvalid = 203,
    Error_CollisionBoundsInvalid = 204,
}

impl ChaperoneCalibrationState {
    #[inline]
    fn from_raw(val: RawChaperoneCalibrationState) -> Option<Self> {
        match val {
            ChaperoneCalibrationState_OK => Some(ChaperoneCalibrationState::OK),
            ChaperoneCalibrationState_Warning => Some(ChaperoneCalibrationState::Warning),
            ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved => {
                Some(ChaperoneCalibrationState::Warning_BaseStationMayHaveMoved)
            }
            ChaperoneCalibrationState_Warning_BaseStationRemoved => {
                Some(ChaperoneCalibrationState::Warning_BaseStationRemoved)
            }
            ChaperoneCalibrationState_Warning_SeatedBoundsInvalid => {
                Some(ChaperoneCalibrationState::Warning_SeatedBoundsInvalid)
            }
            ChaperoneCalibrationState_Error => Some(ChaperoneCalibrationState::Error),
            ChaperoneCalibrationState_Error_BaseStationUninitialized => {
                Some(ChaperoneCalibrationState::Error_BaseStationUninitialized)
            }
            ChaperoneCalibrationState_Error_BaseStationConflict => {
                Some(ChaperoneCalibrationState::Error_BaseStationConflict)
            }
            ChaperoneCalibrationState_Error_PlayAreaInvalid => {
                Some(ChaperoneCalibrationState::Error_PlayAreaInvalid)
            }
            ChaperoneCalibrationState_Error_CollisionBoundsInvalid => {
                Some(ChaperoneCalibrationState::Error_CollisionBoundsInvalid)
            }
            _ => None,
        }
    }
}

impl From<RawChaperoneCalibrationState> for ChaperoneCalibrationState {
    fn from(val: RawChaperoneCalibrationState) -> Self {
        ChaperoneCalibrationState::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ChaperoneCalibrationState.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawChaperoneConfigFile(pub u32);

pub const ChaperoneConfigFile_Live: RawChaperoneConfigFile = RawChaperoneConfigFile(1);
pub const ChaperoneConfigFile_Temp: RawChaperoneConfigFile = RawChaperoneConfigFile(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChaperoneConfigFile {
    Live = 1,
    Temp = 2,
}

impl ChaperoneConfigFile {
    #[inline]
    fn from_raw(val: RawChaperoneConfigFile) -> Option<Self> {
        match val {
            ChaperoneConfigFile_Live => Some(ChaperoneConfigFile::Live),
            ChaperoneConfigFile_Temp => Some(ChaperoneConfigFile::Temp),
            _ => None,
        }
    }
}

impl From<RawChaperoneConfigFile> for ChaperoneConfigFile {
    fn from(val: RawChaperoneConfigFile) -> Self {
        ChaperoneConfigFile::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ChaperoneConfigFile.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawChaperoneImportFlags(pub u32);

pub const ChaperoneImportFlags_BoundsOnly: RawChaperoneImportFlags = RawChaperoneImportFlags(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChaperoneImportFlags {
    BoundsOnly = 1,
}

impl ChaperoneImportFlags {
    #[inline]
    fn from_raw(val: RawChaperoneImportFlags) -> Option<Self> {
        match val {
            ChaperoneImportFlags_BoundsOnly => Some(ChaperoneImportFlags::BoundsOnly),
            _ => None,
        }
    }
}

impl From<RawChaperoneImportFlags> for ChaperoneImportFlags {
    fn from(val: RawChaperoneImportFlags) -> Self {
        ChaperoneImportFlags::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ChaperoneImportFlags.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawCompositorError(pub u32);

pub const CompositorError_None: RawCompositorError = RawCompositorError(0);
pub const CompositorError_RequestFailed: RawCompositorError = RawCompositorError(1);
pub const CompositorError_IncompatibleVersion: RawCompositorError = RawCompositorError(100);
pub const CompositorError_DoNotHaveFocus: RawCompositorError = RawCompositorError(101);
pub const CompositorError_InvalidTexture: RawCompositorError = RawCompositorError(102);
pub const CompositorError_IsNotSceneApplication: RawCompositorError = RawCompositorError(103);
pub const CompositorError_TextureIsOnWrongDevice: RawCompositorError = RawCompositorError(104);
pub const CompositorError_TextureUsesUnsupportedFormat: RawCompositorError =
    RawCompositorError(105);
pub const CompositorError_SharedTexturesNotSupported: RawCompositorError = RawCompositorError(106);
pub const CompositorError_IndexOutOfRange: RawCompositorError = RawCompositorError(107);
pub const CompositorError_AlreadySubmitted: RawCompositorError = RawCompositorError(108);
pub const CompositorError_InvalidBounds: RawCompositorError = RawCompositorError(109);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CompositorError {
    None = 0,
    RequestFailed = 1,
    IncompatibleVersion = 100,
    DoNotHaveFocus = 101,
    InvalidTexture = 102,
    IsNotSceneApplication = 103,
    TextureIsOnWrongDevice = 104,
    TextureUsesUnsupportedFormat = 105,
    SharedTexturesNotSupported = 106,
    IndexOutOfRange = 107,
    AlreadySubmitted = 108,
    InvalidBounds = 109,
}

impl CompositorError {
    #[inline]
    fn from_raw(val: RawCompositorError) -> Option<Self> {
        match val {
            CompositorError_None => Some(CompositorError::None),
            CompositorError_RequestFailed => Some(CompositorError::RequestFailed),
            CompositorError_IncompatibleVersion => Some(CompositorError::IncompatibleVersion),
            CompositorError_DoNotHaveFocus => Some(CompositorError::DoNotHaveFocus),
            CompositorError_InvalidTexture => Some(CompositorError::InvalidTexture),
            CompositorError_IsNotSceneApplication => Some(CompositorError::IsNotSceneApplication),
            CompositorError_TextureIsOnWrongDevice => Some(CompositorError::TextureIsOnWrongDevice),
            CompositorError_TextureUsesUnsupportedFormat => {
                Some(CompositorError::TextureUsesUnsupportedFormat)
            }
            CompositorError_SharedTexturesNotSupported => {
                Some(CompositorError::SharedTexturesNotSupported)
            }
            CompositorError_IndexOutOfRange => Some(CompositorError::IndexOutOfRange),
            CompositorError_AlreadySubmitted => Some(CompositorError::AlreadySubmitted),
            CompositorError_InvalidBounds => Some(CompositorError::InvalidBounds),
            _ => None,
        }
    }
}

impl From<RawCompositorError> for CompositorError {
    fn from(val: RawCompositorError) -> Self {
        CompositorError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for CompositorError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawCompositorTimingMode(pub u32);

pub const CompositorTimingMode_Implicit: RawCompositorTimingMode = RawCompositorTimingMode(0);
pub const CompositorTimingMode_Explicit_RuntimePerformsPostPresentHandoff: RawCompositorTimingMode =
    RawCompositorTimingMode(1);
pub const CompositorTimingMode_Explicit_ApplicationPerformsPostPresentHandoff:
    RawCompositorTimingMode = RawCompositorTimingMode(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CompositorTimingMode {
    Implicit = 0,
    Explicit_RuntimePerformsPostPresentHandoff = 1,
    Explicit_ApplicationPerformsPostPresentHandoff = 2,
}

impl CompositorTimingMode {
    #[inline]
    fn from_raw(val: RawCompositorTimingMode) -> Option<Self> {
        match val {
            CompositorTimingMode_Implicit => Some(CompositorTimingMode::Implicit),
            CompositorTimingMode_Explicit_RuntimePerformsPostPresentHandoff => {
                Some(CompositorTimingMode::Explicit_RuntimePerformsPostPresentHandoff)
            }
            CompositorTimingMode_Explicit_ApplicationPerformsPostPresentHandoff => {
                Some(CompositorTimingMode::Explicit_ApplicationPerformsPostPresentHandoff)
            }
            _ => None,
        }
    }
}

impl From<RawCompositorTimingMode> for CompositorTimingMode {
    fn from(val: RawCompositorTimingMode) -> Self {
        CompositorTimingMode::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for CompositorTimingMode.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawOverlayInputMethod(pub u32);

pub const OverlayInputMethod_None: RawOverlayInputMethod = RawOverlayInputMethod(0);
pub const OverlayInputMethod_Mouse: RawOverlayInputMethod = RawOverlayInputMethod(1);
pub const OverlayInputMethod_DualAnalog: RawOverlayInputMethod = RawOverlayInputMethod(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayInputMethod {
    None = 0,
    Mouse = 1,
    DualAnalog = 2,
}

impl OverlayInputMethod {
    #[inline]
    fn from_raw(val: RawOverlayInputMethod) -> Option<Self> {
        match val {
            OverlayInputMethod_None => Some(OverlayInputMethod::None),
            OverlayInputMethod_Mouse => Some(OverlayInputMethod::Mouse),
            OverlayInputMethod_DualAnalog => Some(OverlayInputMethod::DualAnalog),
            _ => None,
        }
    }
}

impl From<RawOverlayInputMethod> for OverlayInputMethod {
    fn from(val: RawOverlayInputMethod) -> Self {
        OverlayInputMethod::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for OverlayInputMethod.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawOverlayTransformType(pub u32);

pub const OverlayTransformType_Absolute: RawOverlayTransformType = RawOverlayTransformType(0);
pub const OverlayTransformType_TrackedDeviceRelative: RawOverlayTransformType =
    RawOverlayTransformType(1);
pub const OverlayTransformType_SystemOverlay: RawOverlayTransformType = RawOverlayTransformType(2);
pub const OverlayTransformType_TrackedComponent: RawOverlayTransformType =
    RawOverlayTransformType(3);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayTransformType {
    Absolute = 0,
    TrackedDeviceRelative = 1,
    SystemOverlay = 2,
    TrackedComponent = 3,
}

impl OverlayTransformType {
    #[inline]
    fn from_raw(val: RawOverlayTransformType) -> Option<Self> {
        match val {
            OverlayTransformType_Absolute => Some(OverlayTransformType::Absolute),
            OverlayTransformType_TrackedDeviceRelative => {
                Some(OverlayTransformType::TrackedDeviceRelative)
            }
            OverlayTransformType_SystemOverlay => Some(OverlayTransformType::SystemOverlay),
            OverlayTransformType_TrackedComponent => Some(OverlayTransformType::TrackedComponent),
            _ => None,
        }
    }
}

impl From<RawOverlayTransformType> for OverlayTransformType {
    fn from(val: RawOverlayTransformType) -> Self {
        OverlayTransformType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for OverlayTransformType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawOverlayFlags(pub u32);

pub const OverlayFlags_None: RawOverlayFlags = RawOverlayFlags(0);
pub const OverlayFlags_Curved: RawOverlayFlags = RawOverlayFlags(1);
pub const OverlayFlags_RGSS4X: RawOverlayFlags = RawOverlayFlags(2);
pub const OverlayFlags_NoDashboardTab: RawOverlayFlags = RawOverlayFlags(3);
pub const OverlayFlags_AcceptsGamepadEvents: RawOverlayFlags = RawOverlayFlags(4);
pub const OverlayFlags_ShowGamepadFocus: RawOverlayFlags = RawOverlayFlags(5);
pub const OverlayFlags_SendVRScrollEvents: RawOverlayFlags = RawOverlayFlags(6);
pub const OverlayFlags_SendVRTouchpadEvents: RawOverlayFlags = RawOverlayFlags(7);
pub const OverlayFlags_ShowTouchPadScrollWheel: RawOverlayFlags = RawOverlayFlags(8);
pub const OverlayFlags_TransferOwnershipToInternalProcess: RawOverlayFlags = RawOverlayFlags(9);
pub const OverlayFlags_SideBySide_Parallel: RawOverlayFlags = RawOverlayFlags(10);
pub const OverlayFlags_SideBySide_Crossed: RawOverlayFlags = RawOverlayFlags(11);
pub const OverlayFlags_Panorama: RawOverlayFlags = RawOverlayFlags(12);
pub const OverlayFlags_StereoPanorama: RawOverlayFlags = RawOverlayFlags(13);
pub const OverlayFlags_SortWithNonSceneOverlays: RawOverlayFlags = RawOverlayFlags(14);
pub const OverlayFlags_VisibleInDashboard: RawOverlayFlags = RawOverlayFlags(15);
pub const OverlayFlags_MakeOverlaysInteractiveIfVisible: RawOverlayFlags = RawOverlayFlags(16);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayFlags {
    None = 0,
    Curved = 1,
    RGSS4X = 2,
    NoDashboardTab = 3,
    AcceptsGamepadEvents = 4,
    ShowGamepadFocus = 5,
    SendVRScrollEvents = 6,
    SendVRTouchpadEvents = 7,
    ShowTouchPadScrollWheel = 8,
    TransferOwnershipToInternalProcess = 9,
    SideBySide_Parallel = 10,
    SideBySide_Crossed = 11,
    Panorama = 12,
    StereoPanorama = 13,
    SortWithNonSceneOverlays = 14,
    VisibleInDashboard = 15,
    MakeOverlaysInteractiveIfVisible = 16,
}

impl OverlayFlags {
    #[inline]
    fn from_raw(val: RawOverlayFlags) -> Option<Self> {
        match val {
            OverlayFlags_None => Some(OverlayFlags::None),
            OverlayFlags_Curved => Some(OverlayFlags::Curved),
            OverlayFlags_RGSS4X => Some(OverlayFlags::RGSS4X),
            OverlayFlags_NoDashboardTab => Some(OverlayFlags::NoDashboardTab),
            OverlayFlags_AcceptsGamepadEvents => Some(OverlayFlags::AcceptsGamepadEvents),
            OverlayFlags_ShowGamepadFocus => Some(OverlayFlags::ShowGamepadFocus),
            OverlayFlags_SendVRScrollEvents => Some(OverlayFlags::SendVRScrollEvents),
            OverlayFlags_SendVRTouchpadEvents => Some(OverlayFlags::SendVRTouchpadEvents),
            OverlayFlags_ShowTouchPadScrollWheel => Some(OverlayFlags::ShowTouchPadScrollWheel),
            OverlayFlags_TransferOwnershipToInternalProcess => {
                Some(OverlayFlags::TransferOwnershipToInternalProcess)
            }
            OverlayFlags_SideBySide_Parallel => Some(OverlayFlags::SideBySide_Parallel),
            OverlayFlags_SideBySide_Crossed => Some(OverlayFlags::SideBySide_Crossed),
            OverlayFlags_Panorama => Some(OverlayFlags::Panorama),
            OverlayFlags_StereoPanorama => Some(OverlayFlags::StereoPanorama),
            OverlayFlags_SortWithNonSceneOverlays => Some(OverlayFlags::SortWithNonSceneOverlays),
            OverlayFlags_VisibleInDashboard => Some(OverlayFlags::VisibleInDashboard),
            OverlayFlags_MakeOverlaysInteractiveIfVisible => {
                Some(OverlayFlags::MakeOverlaysInteractiveIfVisible)
            }
            _ => None,
        }
    }
}

impl From<RawOverlayFlags> for OverlayFlags {
    fn from(val: RawOverlayFlags) -> Self {
        OverlayFlags::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for OverlayFlags.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawMessageOverlayResponse(pub u32);

pub const MessageOverlayResponse_ButtonPress_0: RawMessageOverlayResponse =
    RawMessageOverlayResponse(0);
pub const MessageOverlayResponse_ButtonPress_1: RawMessageOverlayResponse =
    RawMessageOverlayResponse(1);
pub const MessageOverlayResponse_ButtonPress_2: RawMessageOverlayResponse =
    RawMessageOverlayResponse(2);
pub const MessageOverlayResponse_ButtonPress_3: RawMessageOverlayResponse =
    RawMessageOverlayResponse(3);
pub const MessageOverlayResponse_CouldntFindSystemOverlay: RawMessageOverlayResponse =
    RawMessageOverlayResponse(4);
pub const MessageOverlayResponse_CouldntFindOrCreateClientOverlay: RawMessageOverlayResponse =
    RawMessageOverlayResponse(5);
pub const MessageOverlayResponse_ApplicationQuit: RawMessageOverlayResponse =
    RawMessageOverlayResponse(6);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageOverlayResponse {
    ButtonPress_0 = 0,
    ButtonPress_1 = 1,
    ButtonPress_2 = 2,
    ButtonPress_3 = 3,
    CouldntFindSystemOverlay = 4,
    CouldntFindOrCreateClientOverlay = 5,
    ApplicationQuit = 6,
}

impl MessageOverlayResponse {
    #[inline]
    fn from_raw(val: RawMessageOverlayResponse) -> Option<Self> {
        match val {
            MessageOverlayResponse_ButtonPress_0 => Some(MessageOverlayResponse::ButtonPress_0),
            MessageOverlayResponse_ButtonPress_1 => Some(MessageOverlayResponse::ButtonPress_1),
            MessageOverlayResponse_ButtonPress_2 => Some(MessageOverlayResponse::ButtonPress_2),
            MessageOverlayResponse_ButtonPress_3 => Some(MessageOverlayResponse::ButtonPress_3),
            MessageOverlayResponse_CouldntFindSystemOverlay => {
                Some(MessageOverlayResponse::CouldntFindSystemOverlay)
            }
            MessageOverlayResponse_CouldntFindOrCreateClientOverlay => {
                Some(MessageOverlayResponse::CouldntFindOrCreateClientOverlay)
            }
            MessageOverlayResponse_ApplicationQuit => Some(MessageOverlayResponse::ApplicationQuit),
            _ => None,
        }
    }
}

impl From<RawMessageOverlayResponse> for MessageOverlayResponse {
    fn from(val: RawMessageOverlayResponse) -> Self {
        MessageOverlayResponse::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for MessageOverlayResponse.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawGamepadTextInputMode(pub u32);

pub const GamepadTextInputMode_EGamepadTextInputModeNormal: RawGamepadTextInputMode =
    RawGamepadTextInputMode(0);
pub const GamepadTextInputMode_EGamepadTextInputModePassword: RawGamepadTextInputMode =
    RawGamepadTextInputMode(1);
pub const GamepadTextInputMode_EGamepadTextInputModeSubmit: RawGamepadTextInputMode =
    RawGamepadTextInputMode(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GamepadTextInputMode {
    EGamepadTextInputModeNormal = 0,
    EGamepadTextInputModePassword = 1,
    EGamepadTextInputModeSubmit = 2,
}

impl GamepadTextInputMode {
    #[inline]
    fn from_raw(val: RawGamepadTextInputMode) -> Option<Self> {
        match val {
            GamepadTextInputMode_EGamepadTextInputModeNormal => {
                Some(GamepadTextInputMode::EGamepadTextInputModeNormal)
            }
            GamepadTextInputMode_EGamepadTextInputModePassword => {
                Some(GamepadTextInputMode::EGamepadTextInputModePassword)
            }
            GamepadTextInputMode_EGamepadTextInputModeSubmit => {
                Some(GamepadTextInputMode::EGamepadTextInputModeSubmit)
            }
            _ => None,
        }
    }
}

impl From<RawGamepadTextInputMode> for GamepadTextInputMode {
    fn from(val: RawGamepadTextInputMode) -> Self {
        GamepadTextInputMode::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for GamepadTextInputMode.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawGamepadTextInputLineMode(pub u32);

pub const GamepadTextInputLineMode_EGamepadTextInputLineModeSingleLine:
    RawGamepadTextInputLineMode = RawGamepadTextInputLineMode(0);
pub const GamepadTextInputLineMode_EGamepadTextInputLineModeMultipleLines:
    RawGamepadTextInputLineMode = RawGamepadTextInputLineMode(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GamepadTextInputLineMode {
    EGamepadTextInputLineModeSingleLine = 0,
    EGamepadTextInputLineModeMultipleLines = 1,
}

impl GamepadTextInputLineMode {
    #[inline]
    fn from_raw(val: RawGamepadTextInputLineMode) -> Option<Self> {
        match val {
            GamepadTextInputLineMode_EGamepadTextInputLineModeSingleLine => {
                Some(GamepadTextInputLineMode::EGamepadTextInputLineModeSingleLine)
            }
            GamepadTextInputLineMode_EGamepadTextInputLineModeMultipleLines => {
                Some(GamepadTextInputLineMode::EGamepadTextInputLineModeMultipleLines)
            }
            _ => None,
        }
    }
}

impl From<RawGamepadTextInputLineMode> for GamepadTextInputLineMode {
    fn from(val: RawGamepadTextInputLineMode) -> Self {
        GamepadTextInputLineMode::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for GamepadTextInputLineMode.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawOverlayDirection(pub u32);

pub const OverlayDirection_Up: RawOverlayDirection = RawOverlayDirection(0);
pub const OverlayDirection_Down: RawOverlayDirection = RawOverlayDirection(1);
pub const OverlayDirection_Left: RawOverlayDirection = RawOverlayDirection(2);
pub const OverlayDirection_Right: RawOverlayDirection = RawOverlayDirection(3);
pub const OverlayDirection_Count: RawOverlayDirection = RawOverlayDirection(4);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Count = 4,
}

impl OverlayDirection {
    #[inline]
    fn from_raw(val: RawOverlayDirection) -> Option<Self> {
        match val {
            OverlayDirection_Up => Some(OverlayDirection::Up),
            OverlayDirection_Down => Some(OverlayDirection::Down),
            OverlayDirection_Left => Some(OverlayDirection::Left),
            OverlayDirection_Right => Some(OverlayDirection::Right),
            OverlayDirection_Count => Some(OverlayDirection::Count),
            _ => None,
        }
    }
}

impl From<RawOverlayDirection> for OverlayDirection {
    fn from(val: RawOverlayDirection) -> Self {
        OverlayDirection::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for OverlayDirection.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawOverlayIntersectionMaskPrimitiveType(pub u32);

pub const OverlayIntersectionMaskPrimitiveType_Rectangle: RawOverlayIntersectionMaskPrimitiveType =
    RawOverlayIntersectionMaskPrimitiveType(0);
pub const OverlayIntersectionMaskPrimitiveType_Circle: RawOverlayIntersectionMaskPrimitiveType =
    RawOverlayIntersectionMaskPrimitiveType(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayIntersectionMaskPrimitiveType {
    Rectangle = 0,
    Circle = 1,
}

impl OverlayIntersectionMaskPrimitiveType {
    #[inline]
    fn from_raw(val: RawOverlayIntersectionMaskPrimitiveType) -> Option<Self> {
        match val {
            OverlayIntersectionMaskPrimitiveType_Rectangle => {
                Some(OverlayIntersectionMaskPrimitiveType::Rectangle)
            }
            OverlayIntersectionMaskPrimitiveType_Circle => {
                Some(OverlayIntersectionMaskPrimitiveType::Circle)
            }
            _ => None,
        }
    }
}

impl From<RawOverlayIntersectionMaskPrimitiveType> for OverlayIntersectionMaskPrimitiveType {
    fn from(val: RawOverlayIntersectionMaskPrimitiveType) -> Self {
        OverlayIntersectionMaskPrimitiveType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for OverlayIntersectionMaskPrimitiveType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawRenderModelError(pub u32);

pub const RenderModelError_None: RawRenderModelError = RawRenderModelError(0);
pub const RenderModelError_Loading: RawRenderModelError = RawRenderModelError(100);
pub const RenderModelError_NotSupported: RawRenderModelError = RawRenderModelError(200);
pub const RenderModelError_InvalidArg: RawRenderModelError = RawRenderModelError(300);
pub const RenderModelError_InvalidModel: RawRenderModelError = RawRenderModelError(301);
pub const RenderModelError_NoShapes: RawRenderModelError = RawRenderModelError(302);
pub const RenderModelError_MultipleShapes: RawRenderModelError = RawRenderModelError(303);
pub const RenderModelError_TooManyVertices: RawRenderModelError = RawRenderModelError(304);
pub const RenderModelError_MultipleTextures: RawRenderModelError = RawRenderModelError(305);
pub const RenderModelError_BufferTooSmall: RawRenderModelError = RawRenderModelError(306);
pub const RenderModelError_NotEnoughNormals: RawRenderModelError = RawRenderModelError(307);
pub const RenderModelError_NotEnoughTexCoords: RawRenderModelError = RawRenderModelError(308);
pub const RenderModelError_InvalidTexture: RawRenderModelError = RawRenderModelError(400);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RenderModelError {
    None = 0,
    Loading = 100,
    NotSupported = 200,
    InvalidArg = 300,
    InvalidModel = 301,
    NoShapes = 302,
    MultipleShapes = 303,
    TooManyVertices = 304,
    MultipleTextures = 305,
    BufferTooSmall = 306,
    NotEnoughNormals = 307,
    NotEnoughTexCoords = 308,
    InvalidTexture = 400,
}

impl RenderModelError {
    #[inline]
    fn from_raw(val: RawRenderModelError) -> Option<Self> {
        match val {
            RenderModelError_None => Some(RenderModelError::None),
            RenderModelError_Loading => Some(RenderModelError::Loading),
            RenderModelError_NotSupported => Some(RenderModelError::NotSupported),
            RenderModelError_InvalidArg => Some(RenderModelError::InvalidArg),
            RenderModelError_InvalidModel => Some(RenderModelError::InvalidModel),
            RenderModelError_NoShapes => Some(RenderModelError::NoShapes),
            RenderModelError_MultipleShapes => Some(RenderModelError::MultipleShapes),
            RenderModelError_TooManyVertices => Some(RenderModelError::TooManyVertices),
            RenderModelError_MultipleTextures => Some(RenderModelError::MultipleTextures),
            RenderModelError_BufferTooSmall => Some(RenderModelError::BufferTooSmall),
            RenderModelError_NotEnoughNormals => Some(RenderModelError::NotEnoughNormals),
            RenderModelError_NotEnoughTexCoords => Some(RenderModelError::NotEnoughTexCoords),
            RenderModelError_InvalidTexture => Some(RenderModelError::InvalidTexture),
            _ => None,
        }
    }
}

impl From<RawRenderModelError> for RenderModelError {
    fn from(val: RawRenderModelError) -> Self {
        RenderModelError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for RenderModelError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawComponentProperty(pub u32);

pub const ComponentProperty_IsStatic: RawComponentProperty = RawComponentProperty(1);
pub const ComponentProperty_IsVisible: RawComponentProperty = RawComponentProperty(2);
pub const ComponentProperty_IsTouched: RawComponentProperty = RawComponentProperty(4);
pub const ComponentProperty_IsPressed: RawComponentProperty = RawComponentProperty(8);
pub const ComponentProperty_IsScrolled: RawComponentProperty = RawComponentProperty(16);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ComponentProperty {
    IsStatic = 1,
    IsVisible = 2,
    IsTouched = 4,
    IsPressed = 8,
    IsScrolled = 16,
}

impl ComponentProperty {
    #[inline]
    fn from_raw(val: RawComponentProperty) -> Option<Self> {
        match val {
            ComponentProperty_IsStatic => Some(ComponentProperty::IsStatic),
            ComponentProperty_IsVisible => Some(ComponentProperty::IsVisible),
            ComponentProperty_IsTouched => Some(ComponentProperty::IsTouched),
            ComponentProperty_IsPressed => Some(ComponentProperty::IsPressed),
            ComponentProperty_IsScrolled => Some(ComponentProperty::IsScrolled),
            _ => None,
        }
    }
}

impl From<RawComponentProperty> for ComponentProperty {
    fn from(val: RawComponentProperty) -> Self {
        ComponentProperty::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ComponentProperty.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawNotificationType(pub u32);

pub const NotificationType_Transient: RawNotificationType = RawNotificationType(0);
pub const NotificationType_Persistent: RawNotificationType = RawNotificationType(1);
pub const NotificationType_Transient_SystemWithUserValue: RawNotificationType =
    RawNotificationType(2);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NotificationType {
    Transient = 0,
    Persistent = 1,
    Transient_SystemWithUserValue = 2,
}

impl NotificationType {
    #[inline]
    fn from_raw(val: RawNotificationType) -> Option<Self> {
        match val {
            NotificationType_Transient => Some(NotificationType::Transient),
            NotificationType_Persistent => Some(NotificationType::Persistent),
            NotificationType_Transient_SystemWithUserValue => {
                Some(NotificationType::Transient_SystemWithUserValue)
            }
            _ => None,
        }
    }
}

impl From<RawNotificationType> for NotificationType {
    fn from(val: RawNotificationType) -> Self {
        NotificationType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for NotificationType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawNotificationStyle(pub u32);

pub const NotificationStyle_None: RawNotificationStyle = RawNotificationStyle(0);
pub const NotificationStyle_Application: RawNotificationStyle = RawNotificationStyle(100);
pub const NotificationStyle_Contact_Disabled: RawNotificationStyle = RawNotificationStyle(200);
pub const NotificationStyle_Contact_Enabled: RawNotificationStyle = RawNotificationStyle(201);
pub const NotificationStyle_Contact_Active: RawNotificationStyle = RawNotificationStyle(202);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NotificationStyle {
    None = 0,
    Application = 100,
    Contact_Disabled = 200,
    Contact_Enabled = 201,
    Contact_Active = 202,
}

impl NotificationStyle {
    #[inline]
    fn from_raw(val: RawNotificationStyle) -> Option<Self> {
        match val {
            NotificationStyle_None => Some(NotificationStyle::None),
            NotificationStyle_Application => Some(NotificationStyle::Application),
            NotificationStyle_Contact_Disabled => Some(NotificationStyle::Contact_Disabled),
            NotificationStyle_Contact_Enabled => Some(NotificationStyle::Contact_Enabled),
            NotificationStyle_Contact_Active => Some(NotificationStyle::Contact_Active),
            _ => None,
        }
    }
}

impl From<RawNotificationStyle> for NotificationStyle {
    fn from(val: RawNotificationStyle) -> Self {
        NotificationStyle::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for NotificationStyle.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawSettingsError(pub u32);

pub const SettingsError_None: RawSettingsError = RawSettingsError(0);
pub const SettingsError_IPCFailed: RawSettingsError = RawSettingsError(1);
pub const SettingsError_WriteFailed: RawSettingsError = RawSettingsError(2);
pub const SettingsError_ReadFailed: RawSettingsError = RawSettingsError(3);
pub const SettingsError_JsonParseFailed: RawSettingsError = RawSettingsError(4);
pub const SettingsError_UnsetSettingHasNoDefault: RawSettingsError = RawSettingsError(5);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SettingsError {
    None = 0,
    IPCFailed = 1,
    WriteFailed = 2,
    ReadFailed = 3,
    JsonParseFailed = 4,
    UnsetSettingHasNoDefault = 5,
}

impl SettingsError {
    #[inline]
    fn from_raw(val: RawSettingsError) -> Option<Self> {
        match val {
            SettingsError_None => Some(SettingsError::None),
            SettingsError_IPCFailed => Some(SettingsError::IPCFailed),
            SettingsError_WriteFailed => Some(SettingsError::WriteFailed),
            SettingsError_ReadFailed => Some(SettingsError::ReadFailed),
            SettingsError_JsonParseFailed => Some(SettingsError::JsonParseFailed),
            SettingsError_UnsetSettingHasNoDefault => Some(SettingsError::UnsetSettingHasNoDefault),
            _ => None,
        }
    }
}

impl From<RawSettingsError> for SettingsError {
    fn from(val: RawSettingsError) -> Self {
        SettingsError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for SettingsError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawScreenshotError(pub u32);

pub const ScreenshotError_None: RawScreenshotError = RawScreenshotError(0);
pub const ScreenshotError_RequestFailed: RawScreenshotError = RawScreenshotError(1);
pub const ScreenshotError_IncompatibleVersion: RawScreenshotError = RawScreenshotError(100);
pub const ScreenshotError_NotFound: RawScreenshotError = RawScreenshotError(101);
pub const ScreenshotError_BufferTooSmall: RawScreenshotError = RawScreenshotError(102);
pub const ScreenshotError_ScreenshotAlreadyInProgress: RawScreenshotError = RawScreenshotError(108);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ScreenshotError {
    None = 0,
    RequestFailed = 1,
    IncompatibleVersion = 100,
    NotFound = 101,
    BufferTooSmall = 102,
    ScreenshotAlreadyInProgress = 108,
}

impl ScreenshotError {
    #[inline]
    fn from_raw(val: RawScreenshotError) -> Option<Self> {
        match val {
            ScreenshotError_None => Some(ScreenshotError::None),
            ScreenshotError_RequestFailed => Some(ScreenshotError::RequestFailed),
            ScreenshotError_IncompatibleVersion => Some(ScreenshotError::IncompatibleVersion),
            ScreenshotError_NotFound => Some(ScreenshotError::NotFound),
            ScreenshotError_BufferTooSmall => Some(ScreenshotError::BufferTooSmall),
            ScreenshotError_ScreenshotAlreadyInProgress => {
                Some(ScreenshotError::ScreenshotAlreadyInProgress)
            }
            _ => None,
        }
    }
}

impl From<RawScreenshotError> for ScreenshotError {
    fn from(val: RawScreenshotError) -> Self {
        ScreenshotError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for ScreenshotError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawSkeletalTransformSpace(pub u32);

pub const SkeletalTransformSpace_Model: RawSkeletalTransformSpace = RawSkeletalTransformSpace(0);
pub const SkeletalTransformSpace_Parent: RawSkeletalTransformSpace = RawSkeletalTransformSpace(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalTransformSpace {
    Model = 0,
    Parent = 1,
}

impl SkeletalTransformSpace {
    #[inline]
    fn from_raw(val: RawSkeletalTransformSpace) -> Option<Self> {
        match val {
            SkeletalTransformSpace_Model => Some(SkeletalTransformSpace::Model),
            SkeletalTransformSpace_Parent => Some(SkeletalTransformSpace::Parent),
            _ => None,
        }
    }
}

impl From<RawSkeletalTransformSpace> for SkeletalTransformSpace {
    fn from(val: RawSkeletalTransformSpace) -> Self {
        SkeletalTransformSpace::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for SkeletalTransformSpace.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawSkeletalReferencePose(pub u32);

pub const SkeletalReferencePose_BindPose: RawSkeletalReferencePose = RawSkeletalReferencePose(0);
pub const SkeletalReferencePose_OpenHand: RawSkeletalReferencePose = RawSkeletalReferencePose(1);
pub const SkeletalReferencePose_Fist: RawSkeletalReferencePose = RawSkeletalReferencePose(2);
pub const SkeletalReferencePose_GripLimit: RawSkeletalReferencePose = RawSkeletalReferencePose(3);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalReferencePose {
    BindPose = 0,
    OpenHand = 1,
    Fist = 2,
    GripLimit = 3,
}

impl SkeletalReferencePose {
    #[inline]
    fn from_raw(val: RawSkeletalReferencePose) -> Option<Self> {
        match val {
            SkeletalReferencePose_BindPose => Some(SkeletalReferencePose::BindPose),
            SkeletalReferencePose_OpenHand => Some(SkeletalReferencePose::OpenHand),
            SkeletalReferencePose_Fist => Some(SkeletalReferencePose::Fist),
            SkeletalReferencePose_GripLimit => Some(SkeletalReferencePose::GripLimit),
            _ => None,
        }
    }
}

impl From<RawSkeletalReferencePose> for SkeletalReferencePose {
    fn from(val: RawSkeletalReferencePose) -> Self {
        SkeletalReferencePose::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for SkeletalReferencePose.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawFinger(pub u32);

pub const Finger_Thumb: RawFinger = RawFinger(0);
pub const Finger_Index: RawFinger = RawFinger(1);
pub const Finger_Middle: RawFinger = RawFinger(2);
pub const Finger_Ring: RawFinger = RawFinger(3);
pub const Finger_Pinky: RawFinger = RawFinger(4);
pub const Finger_Count: RawFinger = RawFinger(5);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Finger {
    Thumb = 0,
    Index = 1,
    Middle = 2,
    Ring = 3,
    Pinky = 4,
    Count = 5,
}

impl Finger {
    #[inline]
    fn from_raw(val: RawFinger) -> Option<Self> {
        match val {
            Finger_Thumb => Some(Finger::Thumb),
            Finger_Index => Some(Finger::Index),
            Finger_Middle => Some(Finger::Middle),
            Finger_Ring => Some(Finger::Ring),
            Finger_Pinky => Some(Finger::Pinky),
            Finger_Count => Some(Finger::Count),
            _ => None,
        }
    }
}

impl From<RawFinger> for Finger {
    fn from(val: RawFinger) -> Self {
        Finger::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for Finger.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawFingerSplay(pub u32);

pub const FingerSplay_Thumb_Index: RawFingerSplay = RawFingerSplay(0);
pub const FingerSplay_Index_Middle: RawFingerSplay = RawFingerSplay(1);
pub const FingerSplay_Middle_Ring: RawFingerSplay = RawFingerSplay(2);
pub const FingerSplay_Ring_Pinky: RawFingerSplay = RawFingerSplay(3);
pub const FingerSplay_Count: RawFingerSplay = RawFingerSplay(4);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FingerSplay {
    Thumb_Index = 0,
    Index_Middle = 1,
    Middle_Ring = 2,
    Ring_Pinky = 3,
    Count = 4,
}

impl FingerSplay {
    #[inline]
    fn from_raw(val: RawFingerSplay) -> Option<Self> {
        match val {
            FingerSplay_Thumb_Index => Some(FingerSplay::Thumb_Index),
            FingerSplay_Index_Middle => Some(FingerSplay::Index_Middle),
            FingerSplay_Middle_Ring => Some(FingerSplay::Middle_Ring),
            FingerSplay_Ring_Pinky => Some(FingerSplay::Ring_Pinky),
            FingerSplay_Count => Some(FingerSplay::Count),
            _ => None,
        }
    }
}

impl From<RawFingerSplay> for FingerSplay {
    fn from(val: RawFingerSplay) -> Self {
        FingerSplay::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for FingerSplay.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawInputFilterCancelType(pub u32);

pub const InputFilterCancelType_Timers: RawInputFilterCancelType = RawInputFilterCancelType(0);
pub const InputFilterCancelType_Momentum: RawInputFilterCancelType = RawInputFilterCancelType(1);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InputFilterCancelType {
    Timers = 0,
    Momentum = 1,
}

impl InputFilterCancelType {
    #[inline]
    fn from_raw(val: RawInputFilterCancelType) -> Option<Self> {
        match val {
            InputFilterCancelType_Timers => Some(InputFilterCancelType::Timers),
            InputFilterCancelType_Momentum => Some(InputFilterCancelType::Momentum),
            _ => None,
        }
    }
}

impl From<RawInputFilterCancelType> for InputFilterCancelType {
    fn from(val: RawInputFilterCancelType) -> Self {
        InputFilterCancelType::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for InputFilterCancelType.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawInputStringBits(pub u32);

pub const InputStringBits_Hand: RawInputStringBits = RawInputStringBits(1);
pub const InputStringBits_ControllerType: RawInputStringBits = RawInputStringBits(2);
pub const InputStringBits_InputSource: RawInputStringBits = RawInputStringBits(4);
pub const InputStringBits_All: RawInputStringBits = RawInputStringBits(::std::u32::MAX);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InputStringBits {
    Hand = 1,
    ControllerType = 2,
    InputSource = 4,
    All = ::std::u32::MAX,
}

impl InputStringBits {
    #[inline]
    fn from_raw(val: RawInputStringBits) -> Option<Self> {
        match val {
            InputStringBits_Hand => Some(InputStringBits::Hand),
            InputStringBits_ControllerType => Some(InputStringBits::ControllerType),
            InputStringBits_InputSource => Some(InputStringBits::InputSource),
            InputStringBits_All => Some(InputStringBits::All),
            _ => None,
        }
    }
}

impl From<RawInputStringBits> for InputStringBits {
    fn from(val: RawInputStringBits) -> Self {
        InputStringBits::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for InputStringBits.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawIOBufferError(pub u32);

pub const IOBufferError_Success: RawIOBufferError = RawIOBufferError(0);
pub const IOBufferError_OperationFailed: RawIOBufferError = RawIOBufferError(100);
pub const IOBufferError_InvalidHandle: RawIOBufferError = RawIOBufferError(101);
pub const IOBufferError_InvalidArgument: RawIOBufferError = RawIOBufferError(102);
pub const IOBufferError_PathExists: RawIOBufferError = RawIOBufferError(103);
pub const IOBufferError_PathDoesNotExist: RawIOBufferError = RawIOBufferError(104);
pub const IOBufferError_Permission: RawIOBufferError = RawIOBufferError(105);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IOBufferError {
    Success = 0,
    OperationFailed = 100,
    InvalidHandle = 101,
    InvalidArgument = 102,
    PathExists = 103,
    PathDoesNotExist = 104,
    Permission = 105,
}

impl IOBufferError {
    #[inline]
    fn from_raw(val: RawIOBufferError) -> Option<Self> {
        match val {
            IOBufferError_Success => Some(IOBufferError::Success),
            IOBufferError_OperationFailed => Some(IOBufferError::OperationFailed),
            IOBufferError_InvalidHandle => Some(IOBufferError::InvalidHandle),
            IOBufferError_InvalidArgument => Some(IOBufferError::InvalidArgument),
            IOBufferError_PathExists => Some(IOBufferError::PathExists),
            IOBufferError_PathDoesNotExist => Some(IOBufferError::PathDoesNotExist),
            IOBufferError_Permission => Some(IOBufferError::Permission),
            _ => None,
        }
    }
}

impl From<RawIOBufferError> for IOBufferError {
    fn from(val: RawIOBufferError) -> Self {
        IOBufferError::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for IOBufferError.");
        })
    }
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RawIOBufferMode(pub u32);

pub const IOBufferMode_Read: RawIOBufferMode = RawIOBufferMode(1);
pub const IOBufferMode_Write: RawIOBufferMode = RawIOBufferMode(2);
pub const IOBufferMode_Create: RawIOBufferMode = RawIOBufferMode(512);

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IOBufferMode {
    Read = 1,
    Write = 2,
    Create = 512,
}

impl IOBufferMode {
    #[inline]
    fn from_raw(val: RawIOBufferMode) -> Option<Self> {
        match val {
            IOBufferMode_Read => Some(IOBufferMode::Read),
            IOBufferMode_Write => Some(IOBufferMode::Write),
            IOBufferMode_Create => Some(IOBufferMode::Create),
            _ => None,
        }
    }
}

impl From<RawIOBufferMode> for IOBufferMode {
    fn from(val: RawIOBufferMode) -> Self {
        IOBufferMode::from_raw(val).unwrap_or_else(|| {
            panic!("Invalid value {} for IOBufferMode.");
        })
    }
}
