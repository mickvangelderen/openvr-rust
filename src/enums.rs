use openvr_sys as sys;
use std::error;
use std::fmt;

pub trait Enum: Sized {
    type Raw: fmt::Display;

    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>>;
    fn into_unchecked(self) -> Unchecked<Self>;
}

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Invalid<E: Enum>(pub(crate) E::Raw);

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Unchecked<E: Enum>(pub(crate) E::Raw);

/// EVREye.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Eye {
    /// EVREye_Eye_Left = 0.
    Left = sys::EVREye_Eye_Left,
    /// EVREye_Eye_Right = 1.
    Right = sys::EVREye_Eye_Right,
}

impl Enum for Eye {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVREye_Eye_Left => Ok(Eye::Left),
             sys::EVREye_Eye_Right => Ok(Eye::Right),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<Eye> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of Eye.", self.0)
    }
}

impl error::Error for Invalid<Eye> {
    fn description(&self) -> &str {
        "Value does not represent any variant of Eye."
    }
}

/// ETextureType.
#[repr(i32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TextureType {
    /// ETextureType_TextureType_Invalid = -1.
    Invalid = sys::ETextureType_TextureType_Invalid,
    /// ETextureType_TextureType_DirectX = 0.
    DirectX = sys::ETextureType_TextureType_DirectX,
    /// ETextureType_TextureType_OpenGL = 1.
    OpenGL = sys::ETextureType_TextureType_OpenGL,
    /// ETextureType_TextureType_Vulkan = 2.
    Vulkan = sys::ETextureType_TextureType_Vulkan,
    /// ETextureType_TextureType_IOSurface = 3.
    Iosurface = sys::ETextureType_TextureType_IOSurface,
    /// ETextureType_TextureType_DirectX12 = 4.
    DirectX12 = sys::ETextureType_TextureType_DirectX12,
    /// ETextureType_TextureType_DXGISharedHandle = 5.
    DxgisharedHandle = sys::ETextureType_TextureType_DXGISharedHandle,
    /// ETextureType_TextureType_Metal = 6.
    Metal = sys::ETextureType_TextureType_Metal,
}

impl Enum for TextureType {
    type Raw = i32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ETextureType_TextureType_Invalid => Ok(TextureType::Invalid),
             sys::ETextureType_TextureType_DirectX => Ok(TextureType::DirectX),
             sys::ETextureType_TextureType_OpenGL => Ok(TextureType::OpenGL),
             sys::ETextureType_TextureType_Vulkan => Ok(TextureType::Vulkan),
             sys::ETextureType_TextureType_IOSurface => Ok(TextureType::Iosurface),
             sys::ETextureType_TextureType_DirectX12 => Ok(TextureType::DirectX12),
             sys::ETextureType_TextureType_DXGISharedHandle => Ok(TextureType::DxgisharedHandle),
             sys::ETextureType_TextureType_Metal => Ok(TextureType::Metal),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TextureType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TextureType.", self.0)
    }
}

impl error::Error for Invalid<TextureType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TextureType."
    }
}

/// EColorSpace.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ColorSpace {
    /// EColorSpace_ColorSpace_Auto = 0.
    Auto = sys::EColorSpace_ColorSpace_Auto,
    /// EColorSpace_ColorSpace_Gamma = 1.
    Gamma = sys::EColorSpace_ColorSpace_Gamma,
    /// EColorSpace_ColorSpace_Linear = 2.
    Linear = sys::EColorSpace_ColorSpace_Linear,
}

impl Enum for ColorSpace {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EColorSpace_ColorSpace_Auto => Ok(ColorSpace::Auto),
             sys::EColorSpace_ColorSpace_Gamma => Ok(ColorSpace::Gamma),
             sys::EColorSpace_ColorSpace_Linear => Ok(ColorSpace::Linear),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ColorSpace> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ColorSpace.", self.0)
    }
}

impl error::Error for Invalid<ColorSpace> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ColorSpace."
    }
}

/// ETrackingResult.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackingResult {
    /// ETrackingResult_TrackingResult_Uninitialized = 1.
    Uninitialized = sys::ETrackingResult_TrackingResult_Uninitialized,
    /// ETrackingResult_TrackingResult_Calibrating_InProgress = 100.
    CalibratingInProgress = sys::ETrackingResult_TrackingResult_Calibrating_InProgress,
    /// ETrackingResult_TrackingResult_Calibrating_OutOfRange = 101.
    CalibratingOutOfRange = sys::ETrackingResult_TrackingResult_Calibrating_OutOfRange,
    /// ETrackingResult_TrackingResult_Running_OK = 200.
    RunningOK = sys::ETrackingResult_TrackingResult_Running_OK,
    /// ETrackingResult_TrackingResult_Running_OutOfRange = 201.
    RunningOutOfRange = sys::ETrackingResult_TrackingResult_Running_OutOfRange,
    /// ETrackingResult_TrackingResult_Fallback_RotationOnly = 300.
    FallbackRotationOnly = sys::ETrackingResult_TrackingResult_Fallback_RotationOnly,
}

impl Enum for TrackingResult {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ETrackingResult_TrackingResult_Uninitialized => Ok(TrackingResult::Uninitialized),
             sys::ETrackingResult_TrackingResult_Calibrating_InProgress => Ok(TrackingResult::CalibratingInProgress),
             sys::ETrackingResult_TrackingResult_Calibrating_OutOfRange => Ok(TrackingResult::CalibratingOutOfRange),
             sys::ETrackingResult_TrackingResult_Running_OK => Ok(TrackingResult::RunningOK),
             sys::ETrackingResult_TrackingResult_Running_OutOfRange => Ok(TrackingResult::RunningOutOfRange),
             sys::ETrackingResult_TrackingResult_Fallback_RotationOnly => Ok(TrackingResult::FallbackRotationOnly),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackingResult> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackingResult.", self.0)
    }
}

impl error::Error for Invalid<TrackingResult> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackingResult."
    }
}

/// ETrackedDeviceClass.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedDeviceClass {
    /// ETrackedDeviceClass_TrackedDeviceClass_Invalid = 0.
    Invalid = sys::ETrackedDeviceClass_TrackedDeviceClass_Invalid,
    /// ETrackedDeviceClass_TrackedDeviceClass_HMD = 1.
    Hmd = sys::ETrackedDeviceClass_TrackedDeviceClass_HMD,
    /// ETrackedDeviceClass_TrackedDeviceClass_Controller = 2.
    Controller = sys::ETrackedDeviceClass_TrackedDeviceClass_Controller,
    /// ETrackedDeviceClass_TrackedDeviceClass_GenericTracker = 3.
    GenericTracker = sys::ETrackedDeviceClass_TrackedDeviceClass_GenericTracker,
    /// ETrackedDeviceClass_TrackedDeviceClass_TrackingReference = 4.
    TrackingReference = sys::ETrackedDeviceClass_TrackedDeviceClass_TrackingReference,
    /// ETrackedDeviceClass_TrackedDeviceClass_DisplayRedirect = 5.
    DisplayRedirect = sys::ETrackedDeviceClass_TrackedDeviceClass_DisplayRedirect,
    /// ETrackedDeviceClass_TrackedDeviceClass_Max = 6.
    Max = sys::ETrackedDeviceClass_TrackedDeviceClass_Max,
}

impl Enum for TrackedDeviceClass {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ETrackedDeviceClass_TrackedDeviceClass_Invalid => Ok(TrackedDeviceClass::Invalid),
             sys::ETrackedDeviceClass_TrackedDeviceClass_HMD => Ok(TrackedDeviceClass::Hmd),
             sys::ETrackedDeviceClass_TrackedDeviceClass_Controller => Ok(TrackedDeviceClass::Controller),
             sys::ETrackedDeviceClass_TrackedDeviceClass_GenericTracker => Ok(TrackedDeviceClass::GenericTracker),
             sys::ETrackedDeviceClass_TrackedDeviceClass_TrackingReference => Ok(TrackedDeviceClass::TrackingReference),
             sys::ETrackedDeviceClass_TrackedDeviceClass_DisplayRedirect => Ok(TrackedDeviceClass::DisplayRedirect),
             sys::ETrackedDeviceClass_TrackedDeviceClass_Max => Ok(TrackedDeviceClass::Max),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackedDeviceClass> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackedDeviceClass.", self.0)
    }
}

impl error::Error for Invalid<TrackedDeviceClass> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackedDeviceClass."
    }
}

/// ETrackedControllerRole.
/// Omitted variants:
///  - TrackedControllerRole_Max
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedControllerRole {
    /// ETrackedControllerRole_TrackedControllerRole_Invalid = 0.
    Invalid = sys::ETrackedControllerRole_TrackedControllerRole_Invalid,
    /// ETrackedControllerRole_TrackedControllerRole_LeftHand = 1.
    LeftHand = sys::ETrackedControllerRole_TrackedControllerRole_LeftHand,
    /// ETrackedControllerRole_TrackedControllerRole_RightHand = 2.
    RightHand = sys::ETrackedControllerRole_TrackedControllerRole_RightHand,
    /// ETrackedControllerRole_TrackedControllerRole_OptOut = 3.
    OptOut = sys::ETrackedControllerRole_TrackedControllerRole_OptOut,
    /// ETrackedControllerRole_TrackedControllerRole_Treadmill = 4.
    Treadmill = sys::ETrackedControllerRole_TrackedControllerRole_Treadmill,
}

impl Enum for TrackedControllerRole {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ETrackedControllerRole_TrackedControllerRole_Invalid => Ok(TrackedControllerRole::Invalid),
             sys::ETrackedControllerRole_TrackedControllerRole_LeftHand => Ok(TrackedControllerRole::LeftHand),
             sys::ETrackedControllerRole_TrackedControllerRole_RightHand => Ok(TrackedControllerRole::RightHand),
             sys::ETrackedControllerRole_TrackedControllerRole_OptOut => Ok(TrackedControllerRole::OptOut),
             sys::ETrackedControllerRole_TrackedControllerRole_Treadmill => Ok(TrackedControllerRole::Treadmill),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackedControllerRole> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackedControllerRole.", self.0)
    }
}

impl error::Error for Invalid<TrackedControllerRole> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackedControllerRole."
    }
}

/// ETrackingUniverseOrigin.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackingUniverseOrigin {
    /// ETrackingUniverseOrigin_TrackingUniverseSeated = 0.
    TrackingUniverseSeated = sys::ETrackingUniverseOrigin_TrackingUniverseSeated,
    /// ETrackingUniverseOrigin_TrackingUniverseStanding = 1.
    TrackingUniverseStanding = sys::ETrackingUniverseOrigin_TrackingUniverseStanding,
    /// ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated = 2.
    TrackingUniverseRawAndUncalibrated = sys::ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated,
}

impl Enum for TrackingUniverseOrigin {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ETrackingUniverseOrigin_TrackingUniverseSeated => Ok(TrackingUniverseOrigin::TrackingUniverseSeated),
             sys::ETrackingUniverseOrigin_TrackingUniverseStanding => Ok(TrackingUniverseOrigin::TrackingUniverseStanding),
             sys::ETrackingUniverseOrigin_TrackingUniverseRawAndUncalibrated => Ok(TrackingUniverseOrigin::TrackingUniverseRawAndUncalibrated),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackingUniverseOrigin> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackingUniverseOrigin.", self.0)
    }
}

impl error::Error for Invalid<TrackingUniverseOrigin> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackingUniverseOrigin."
    }
}

/// ETrackedDeviceProperty.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedDeviceProperty {
    /// ETrackedDeviceProperty_Prop_Invalid = 0.
    Invalid = sys::ETrackedDeviceProperty_Prop_Invalid,
    /// ETrackedDeviceProperty_Prop_TrackingSystemName_String = 1000.
    TrackingSystemNameString = sys::ETrackedDeviceProperty_Prop_TrackingSystemName_String,
    /// ETrackedDeviceProperty_Prop_ModelNumber_String = 1001.
    ModelNumberString = sys::ETrackedDeviceProperty_Prop_ModelNumber_String,
    /// ETrackedDeviceProperty_Prop_SerialNumber_String = 1002.
    SerialNumberString = sys::ETrackedDeviceProperty_Prop_SerialNumber_String,
    /// ETrackedDeviceProperty_Prop_RenderModelName_String = 1003.
    RenderModelNameString = sys::ETrackedDeviceProperty_Prop_RenderModelName_String,
    /// ETrackedDeviceProperty_Prop_WillDriftInYaw_Bool = 1004.
    WillDriftInYawBool = sys::ETrackedDeviceProperty_Prop_WillDriftInYaw_Bool,
    /// ETrackedDeviceProperty_Prop_ManufacturerName_String = 1005.
    ManufacturerNameString = sys::ETrackedDeviceProperty_Prop_ManufacturerName_String,
    /// ETrackedDeviceProperty_Prop_TrackingFirmwareVersion_String = 1006.
    TrackingFirmwareVersionString = sys::ETrackedDeviceProperty_Prop_TrackingFirmwareVersion_String,
    /// ETrackedDeviceProperty_Prop_HardwareRevision_String = 1007.
    HardwareRevisionString = sys::ETrackedDeviceProperty_Prop_HardwareRevision_String,
    /// ETrackedDeviceProperty_Prop_AllWirelessDongleDescriptions_String = 1008.
    AllWirelessDongleDescriptionsString = sys::ETrackedDeviceProperty_Prop_AllWirelessDongleDescriptions_String,
    /// ETrackedDeviceProperty_Prop_ConnectedWirelessDongle_String = 1009.
    ConnectedWirelessDongleString = sys::ETrackedDeviceProperty_Prop_ConnectedWirelessDongle_String,
    /// ETrackedDeviceProperty_Prop_DeviceIsWireless_Bool = 1010.
    DeviceIsWirelessBool = sys::ETrackedDeviceProperty_Prop_DeviceIsWireless_Bool,
    /// ETrackedDeviceProperty_Prop_DeviceIsCharging_Bool = 1011.
    DeviceIsChargingBool = sys::ETrackedDeviceProperty_Prop_DeviceIsCharging_Bool,
    /// ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float = 1012.
    DeviceBatteryPercentageFloat = sys::ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float,
    /// ETrackedDeviceProperty_Prop_StatusDisplayTransform_Matrix34 = 1013.
    StatusDisplayTransformMatrix34 = sys::ETrackedDeviceProperty_Prop_StatusDisplayTransform_Matrix34,
    /// ETrackedDeviceProperty_Prop_Firmware_UpdateAvailable_Bool = 1014.
    FirmwareUpdateAvailableBool = sys::ETrackedDeviceProperty_Prop_Firmware_UpdateAvailable_Bool,
    /// ETrackedDeviceProperty_Prop_Firmware_ManualUpdate_Bool = 1015.
    FirmwareManualUpdateBool = sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdate_Bool,
    /// ETrackedDeviceProperty_Prop_Firmware_ManualUpdateURL_String = 1016.
    FirmwareManualUpdateURLString = sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdateURL_String,
    /// ETrackedDeviceProperty_Prop_HardwareRevision_Uint64 = 1017.
    HardwareRevisionUint64 = sys::ETrackedDeviceProperty_Prop_HardwareRevision_Uint64,
    /// ETrackedDeviceProperty_Prop_FirmwareVersion_Uint64 = 1018.
    FirmwareVersionUint64 = sys::ETrackedDeviceProperty_Prop_FirmwareVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_FPGAVersion_Uint64 = 1019.
    FpgaversionUint64 = sys::ETrackedDeviceProperty_Prop_FPGAVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_VRCVersion_Uint64 = 1020.
    VrcversionUint64 = sys::ETrackedDeviceProperty_Prop_VRCVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_RadioVersion_Uint64 = 1021.
    RadioVersionUint64 = sys::ETrackedDeviceProperty_Prop_RadioVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_DongleVersion_Uint64 = 1022.
    DongleVersionUint64 = sys::ETrackedDeviceProperty_Prop_DongleVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_BlockServerShutdown_Bool = 1023.
    BlockServerShutdownBool = sys::ETrackedDeviceProperty_Prop_BlockServerShutdown_Bool,
    /// ETrackedDeviceProperty_Prop_CanUnifyCoordinateSystemWithHmd_Bool = 1024.
    CanUnifyCoordinateSystemWithHmdBool = sys::ETrackedDeviceProperty_Prop_CanUnifyCoordinateSystemWithHmd_Bool,
    /// ETrackedDeviceProperty_Prop_ContainsProximitySensor_Bool = 1025.
    ContainsProximitySensorBool = sys::ETrackedDeviceProperty_Prop_ContainsProximitySensor_Bool,
    /// ETrackedDeviceProperty_Prop_DeviceProvidesBatteryStatus_Bool = 1026.
    DeviceProvidesBatteryStatusBool = sys::ETrackedDeviceProperty_Prop_DeviceProvidesBatteryStatus_Bool,
    /// ETrackedDeviceProperty_Prop_DeviceCanPowerOff_Bool = 1027.
    DeviceCanPowerOffBool = sys::ETrackedDeviceProperty_Prop_DeviceCanPowerOff_Bool,
    /// ETrackedDeviceProperty_Prop_Firmware_ProgrammingTarget_String = 1028.
    FirmwareProgrammingTargetString = sys::ETrackedDeviceProperty_Prop_Firmware_ProgrammingTarget_String,
    /// ETrackedDeviceProperty_Prop_DeviceClass_Int32 = 1029.
    DeviceClassInt32 = sys::ETrackedDeviceProperty_Prop_DeviceClass_Int32,
    /// ETrackedDeviceProperty_Prop_HasCamera_Bool = 1030.
    HasCameraBool = sys::ETrackedDeviceProperty_Prop_HasCamera_Bool,
    /// ETrackedDeviceProperty_Prop_DriverVersion_String = 1031.
    DriverVersionString = sys::ETrackedDeviceProperty_Prop_DriverVersion_String,
    /// ETrackedDeviceProperty_Prop_Firmware_ForceUpdateRequired_Bool = 1032.
    FirmwareForceUpdateRequiredBool = sys::ETrackedDeviceProperty_Prop_Firmware_ForceUpdateRequired_Bool,
    /// ETrackedDeviceProperty_Prop_ViveSystemButtonFixRequired_Bool = 1033.
    ViveSystemButtonFixRequiredBool = sys::ETrackedDeviceProperty_Prop_ViveSystemButtonFixRequired_Bool,
    /// ETrackedDeviceProperty_Prop_ParentDriver_Uint64 = 1034.
    ParentDriverUint64 = sys::ETrackedDeviceProperty_Prop_ParentDriver_Uint64,
    /// ETrackedDeviceProperty_Prop_ResourceRoot_String = 1035.
    ResourceRootString = sys::ETrackedDeviceProperty_Prop_ResourceRoot_String,
    /// ETrackedDeviceProperty_Prop_RegisteredDeviceType_String = 1036.
    RegisteredDeviceTypeString = sys::ETrackedDeviceProperty_Prop_RegisteredDeviceType_String,
    /// ETrackedDeviceProperty_Prop_InputProfilePath_String = 1037.
    InputProfilePathString = sys::ETrackedDeviceProperty_Prop_InputProfilePath_String,
    /// ETrackedDeviceProperty_Prop_NeverTracked_Bool = 1038.
    NeverTrackedBool = sys::ETrackedDeviceProperty_Prop_NeverTracked_Bool,
    /// ETrackedDeviceProperty_Prop_NumCameras_Int32 = 1039.
    NumCamerasInt32 = sys::ETrackedDeviceProperty_Prop_NumCameras_Int32,
    /// ETrackedDeviceProperty_Prop_CameraFrameLayout_Int32 = 1040.
    CameraFrameLayoutInt32 = sys::ETrackedDeviceProperty_Prop_CameraFrameLayout_Int32,
    /// ETrackedDeviceProperty_Prop_CameraStreamFormat_Int32 = 1041.
    CameraStreamFormatInt32 = sys::ETrackedDeviceProperty_Prop_CameraStreamFormat_Int32,
    /// ETrackedDeviceProperty_Prop_AdditionalDeviceSettingsPath_String = 1042.
    AdditionalDeviceSettingsPathString = sys::ETrackedDeviceProperty_Prop_AdditionalDeviceSettingsPath_String,
    /// ETrackedDeviceProperty_Prop_Identifiable_Bool = 1043.
    IdentifiableBool = sys::ETrackedDeviceProperty_Prop_Identifiable_Bool,
    /// ETrackedDeviceProperty_Prop_ReportsTimeSinceVSync_Bool = 2000.
    ReportsTimeSinceVSyncBool = sys::ETrackedDeviceProperty_Prop_ReportsTimeSinceVSync_Bool,
    /// ETrackedDeviceProperty_Prop_SecondsFromVsyncToPhotons_Float = 2001.
    SecondsFromVsyncToPhotonsFloat = sys::ETrackedDeviceProperty_Prop_SecondsFromVsyncToPhotons_Float,
    /// ETrackedDeviceProperty_Prop_DisplayFrequency_Float = 2002.
    DisplayFrequencyFloat = sys::ETrackedDeviceProperty_Prop_DisplayFrequency_Float,
    /// ETrackedDeviceProperty_Prop_UserIpdMeters_Float = 2003.
    UserIpdMetersFloat = sys::ETrackedDeviceProperty_Prop_UserIpdMeters_Float,
    /// ETrackedDeviceProperty_Prop_CurrentUniverseId_Uint64 = 2004.
    CurrentUniverseIdUint64 = sys::ETrackedDeviceProperty_Prop_CurrentUniverseId_Uint64,
    /// ETrackedDeviceProperty_Prop_PreviousUniverseId_Uint64 = 2005.
    PreviousUniverseIdUint64 = sys::ETrackedDeviceProperty_Prop_PreviousUniverseId_Uint64,
    /// ETrackedDeviceProperty_Prop_DisplayFirmwareVersion_Uint64 = 2006.
    DisplayFirmwareVersionUint64 = sys::ETrackedDeviceProperty_Prop_DisplayFirmwareVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_IsOnDesktop_Bool = 2007.
    IsOnDesktopBool = sys::ETrackedDeviceProperty_Prop_IsOnDesktop_Bool,
    /// ETrackedDeviceProperty_Prop_DisplayMCType_Int32 = 2008.
    DisplayMCTypeInt32 = sys::ETrackedDeviceProperty_Prop_DisplayMCType_Int32,
    /// ETrackedDeviceProperty_Prop_DisplayMCOffset_Float = 2009.
    DisplayMCOffsetFloat = sys::ETrackedDeviceProperty_Prop_DisplayMCOffset_Float,
    /// ETrackedDeviceProperty_Prop_DisplayMCScale_Float = 2010.
    DisplayMCScaleFloat = sys::ETrackedDeviceProperty_Prop_DisplayMCScale_Float,
    /// ETrackedDeviceProperty_Prop_EdidVendorID_Int32 = 2011.
    EdidVendorIDInt32 = sys::ETrackedDeviceProperty_Prop_EdidVendorID_Int32,
    /// ETrackedDeviceProperty_Prop_DisplayMCImageLeft_String = 2012.
    DisplayMCImageLeftString = sys::ETrackedDeviceProperty_Prop_DisplayMCImageLeft_String,
    /// ETrackedDeviceProperty_Prop_DisplayMCImageRight_String = 2013.
    DisplayMCImageRightString = sys::ETrackedDeviceProperty_Prop_DisplayMCImageRight_String,
    /// ETrackedDeviceProperty_Prop_DisplayGCBlackClamp_Float = 2014.
    DisplayGCBlackClampFloat = sys::ETrackedDeviceProperty_Prop_DisplayGCBlackClamp_Float,
    /// ETrackedDeviceProperty_Prop_EdidProductID_Int32 = 2015.
    EdidProductIDInt32 = sys::ETrackedDeviceProperty_Prop_EdidProductID_Int32,
    /// ETrackedDeviceProperty_Prop_CameraToHeadTransform_Matrix34 = 2016.
    CameraToHeadTransformMatrix34 = sys::ETrackedDeviceProperty_Prop_CameraToHeadTransform_Matrix34,
    /// ETrackedDeviceProperty_Prop_DisplayGCType_Int32 = 2017.
    DisplayGCTypeInt32 = sys::ETrackedDeviceProperty_Prop_DisplayGCType_Int32,
    /// ETrackedDeviceProperty_Prop_DisplayGCOffset_Float = 2018.
    DisplayGCOffsetFloat = sys::ETrackedDeviceProperty_Prop_DisplayGCOffset_Float,
    /// ETrackedDeviceProperty_Prop_DisplayGCScale_Float = 2019.
    DisplayGCScaleFloat = sys::ETrackedDeviceProperty_Prop_DisplayGCScale_Float,
    /// ETrackedDeviceProperty_Prop_DisplayGCPrescale_Float = 2020.
    DisplayGCPrescaleFloat = sys::ETrackedDeviceProperty_Prop_DisplayGCPrescale_Float,
    /// ETrackedDeviceProperty_Prop_DisplayGCImage_String = 2021.
    DisplayGCImageString = sys::ETrackedDeviceProperty_Prop_DisplayGCImage_String,
    /// ETrackedDeviceProperty_Prop_LensCenterLeftU_Float = 2022.
    LensCenterLeftUFloat = sys::ETrackedDeviceProperty_Prop_LensCenterLeftU_Float,
    /// ETrackedDeviceProperty_Prop_LensCenterLeftV_Float = 2023.
    LensCenterLeftVFloat = sys::ETrackedDeviceProperty_Prop_LensCenterLeftV_Float,
    /// ETrackedDeviceProperty_Prop_LensCenterRightU_Float = 2024.
    LensCenterRightUFloat = sys::ETrackedDeviceProperty_Prop_LensCenterRightU_Float,
    /// ETrackedDeviceProperty_Prop_LensCenterRightV_Float = 2025.
    LensCenterRightVFloat = sys::ETrackedDeviceProperty_Prop_LensCenterRightV_Float,
    /// ETrackedDeviceProperty_Prop_UserHeadToEyeDepthMeters_Float = 2026.
    UserHeadToEyeDepthMetersFloat = sys::ETrackedDeviceProperty_Prop_UserHeadToEyeDepthMeters_Float,
    /// ETrackedDeviceProperty_Prop_CameraFirmwareVersion_Uint64 = 2027.
    CameraFirmwareVersionUint64 = sys::ETrackedDeviceProperty_Prop_CameraFirmwareVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_CameraFirmwareDescription_String = 2028.
    CameraFirmwareDescriptionString = sys::ETrackedDeviceProperty_Prop_CameraFirmwareDescription_String,
    /// ETrackedDeviceProperty_Prop_DisplayFPGAVersion_Uint64 = 2029.
    DisplayFPGAVersionUint64 = sys::ETrackedDeviceProperty_Prop_DisplayFPGAVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_DisplayBootloaderVersion_Uint64 = 2030.
    DisplayBootloaderVersionUint64 = sys::ETrackedDeviceProperty_Prop_DisplayBootloaderVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_DisplayHardwareVersion_Uint64 = 2031.
    DisplayHardwareVersionUint64 = sys::ETrackedDeviceProperty_Prop_DisplayHardwareVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_AudioFirmwareVersion_Uint64 = 2032.
    AudioFirmwareVersionUint64 = sys::ETrackedDeviceProperty_Prop_AudioFirmwareVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_CameraCompatibilityMode_Int32 = 2033.
    CameraCompatibilityModeInt32 = sys::ETrackedDeviceProperty_Prop_CameraCompatibilityMode_Int32,
    /// ETrackedDeviceProperty_Prop_ScreenshotHorizontalFieldOfViewDegrees_Float = 2034.
    ScreenshotHorizontalFieldOfViewDegreesFloat = sys::ETrackedDeviceProperty_Prop_ScreenshotHorizontalFieldOfViewDegrees_Float,
    /// ETrackedDeviceProperty_Prop_ScreenshotVerticalFieldOfViewDegrees_Float = 2035.
    ScreenshotVerticalFieldOfViewDegreesFloat = sys::ETrackedDeviceProperty_Prop_ScreenshotVerticalFieldOfViewDegrees_Float,
    /// ETrackedDeviceProperty_Prop_DisplaySuppressed_Bool = 2036.
    DisplaySuppressedBool = sys::ETrackedDeviceProperty_Prop_DisplaySuppressed_Bool,
    /// ETrackedDeviceProperty_Prop_DisplayAllowNightMode_Bool = 2037.
    DisplayAllowNightModeBool = sys::ETrackedDeviceProperty_Prop_DisplayAllowNightMode_Bool,
    /// ETrackedDeviceProperty_Prop_DisplayMCImageWidth_Int32 = 2038.
    DisplayMCImageWidthInt32 = sys::ETrackedDeviceProperty_Prop_DisplayMCImageWidth_Int32,
    /// ETrackedDeviceProperty_Prop_DisplayMCImageHeight_Int32 = 2039.
    DisplayMCImageHeightInt32 = sys::ETrackedDeviceProperty_Prop_DisplayMCImageHeight_Int32,
    /// ETrackedDeviceProperty_Prop_DisplayMCImageNumChannels_Int32 = 2040.
    DisplayMCImageNumChannelsInt32 = sys::ETrackedDeviceProperty_Prop_DisplayMCImageNumChannels_Int32,
    /// ETrackedDeviceProperty_Prop_DisplayMCImageData_Binary = 2041.
    DisplayMCImageDataBinary = sys::ETrackedDeviceProperty_Prop_DisplayMCImageData_Binary,
    /// ETrackedDeviceProperty_Prop_SecondsFromPhotonsToVblank_Float = 2042.
    SecondsFromPhotonsToVblankFloat = sys::ETrackedDeviceProperty_Prop_SecondsFromPhotonsToVblank_Float,
    /// ETrackedDeviceProperty_Prop_DriverDirectModeSendsVsyncEvents_Bool = 2043.
    DriverDirectModeSendsVsyncEventsBool = sys::ETrackedDeviceProperty_Prop_DriverDirectModeSendsVsyncEvents_Bool,
    /// ETrackedDeviceProperty_Prop_DisplayDebugMode_Bool = 2044.
    DisplayDebugModeBool = sys::ETrackedDeviceProperty_Prop_DisplayDebugMode_Bool,
    /// ETrackedDeviceProperty_Prop_GraphicsAdapterLuid_Uint64 = 2045.
    GraphicsAdapterLuidUint64 = sys::ETrackedDeviceProperty_Prop_GraphicsAdapterLuid_Uint64,
    /// ETrackedDeviceProperty_Prop_DriverProvidedChaperonePath_String = 2048.
    DriverProvidedChaperonePathString = sys::ETrackedDeviceProperty_Prop_DriverProvidedChaperonePath_String,
    /// ETrackedDeviceProperty_Prop_ExpectedTrackingReferenceCount_Int32 = 2049.
    ExpectedTrackingReferenceCountInt32 = sys::ETrackedDeviceProperty_Prop_ExpectedTrackingReferenceCount_Int32,
    /// ETrackedDeviceProperty_Prop_ExpectedControllerCount_Int32 = 2050.
    ExpectedControllerCountInt32 = sys::ETrackedDeviceProperty_Prop_ExpectedControllerCount_Int32,
    /// ETrackedDeviceProperty_Prop_NamedIconPathControllerLeftDeviceOff_String = 2051.
    NamedIconPathControllerLeftDeviceOffString = sys::ETrackedDeviceProperty_Prop_NamedIconPathControllerLeftDeviceOff_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathControllerRightDeviceOff_String = 2052.
    NamedIconPathControllerRightDeviceOffString = sys::ETrackedDeviceProperty_Prop_NamedIconPathControllerRightDeviceOff_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathTrackingReferenceDeviceOff_String = 2053.
    NamedIconPathTrackingReferenceDeviceOffString = sys::ETrackedDeviceProperty_Prop_NamedIconPathTrackingReferenceDeviceOff_String,
    /// ETrackedDeviceProperty_Prop_DoNotApplyPrediction_Bool = 2054.
    DoNotApplyPredictionBool = sys::ETrackedDeviceProperty_Prop_DoNotApplyPrediction_Bool,
    /// ETrackedDeviceProperty_Prop_CameraToHeadTransforms_Matrix34_Array = 2055.
    CameraToHeadTransformsMatrix34Array = sys::ETrackedDeviceProperty_Prop_CameraToHeadTransforms_Matrix34_Array,
    /// ETrackedDeviceProperty_Prop_DistortionMeshResolution_Int32 = 2056.
    DistortionMeshResolutionInt32 = sys::ETrackedDeviceProperty_Prop_DistortionMeshResolution_Int32,
    /// ETrackedDeviceProperty_Prop_DriverIsDrawingControllers_Bool = 2057.
    DriverIsDrawingControllersBool = sys::ETrackedDeviceProperty_Prop_DriverIsDrawingControllers_Bool,
    /// ETrackedDeviceProperty_Prop_DriverRequestsApplicationPause_Bool = 2058.
    DriverRequestsApplicationPauseBool = sys::ETrackedDeviceProperty_Prop_DriverRequestsApplicationPause_Bool,
    /// ETrackedDeviceProperty_Prop_DriverRequestsReducedRendering_Bool = 2059.
    DriverRequestsReducedRenderingBool = sys::ETrackedDeviceProperty_Prop_DriverRequestsReducedRendering_Bool,
    /// ETrackedDeviceProperty_Prop_MinimumIpdStepMeters_Float = 2060.
    MinimumIpdStepMetersFloat = sys::ETrackedDeviceProperty_Prop_MinimumIpdStepMeters_Float,
    /// ETrackedDeviceProperty_Prop_AudioBridgeFirmwareVersion_Uint64 = 2061.
    AudioBridgeFirmwareVersionUint64 = sys::ETrackedDeviceProperty_Prop_AudioBridgeFirmwareVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_ImageBridgeFirmwareVersion_Uint64 = 2062.
    ImageBridgeFirmwareVersionUint64 = sys::ETrackedDeviceProperty_Prop_ImageBridgeFirmwareVersion_Uint64,
    /// ETrackedDeviceProperty_Prop_ImuToHeadTransform_Matrix34 = 2063.
    ImuToHeadTransformMatrix34 = sys::ETrackedDeviceProperty_Prop_ImuToHeadTransform_Matrix34,
    /// ETrackedDeviceProperty_Prop_ImuFactoryGyroBias_Vector3 = 2064.
    ImuFactoryGyroBiasVector3 = sys::ETrackedDeviceProperty_Prop_ImuFactoryGyroBias_Vector3,
    /// ETrackedDeviceProperty_Prop_ImuFactoryGyroScale_Vector3 = 2065.
    ImuFactoryGyroScaleVector3 = sys::ETrackedDeviceProperty_Prop_ImuFactoryGyroScale_Vector3,
    /// ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerBias_Vector3 = 2066.
    ImuFactoryAccelerometerBiasVector3 = sys::ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerBias_Vector3,
    /// ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerScale_Vector3 = 2067.
    ImuFactoryAccelerometerScaleVector3 = sys::ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerScale_Vector3,
    /// ETrackedDeviceProperty_Prop_ConfigurationIncludesLighthouse20Features_Bool = 2069.
    ConfigurationIncludesLighthouse20FeaturesBool = sys::ETrackedDeviceProperty_Prop_ConfigurationIncludesLighthouse20Features_Bool,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraCorrectionMode_Int32 = 2200.
    DriverRequestedMuraCorrectionModeInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraCorrectionMode_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerLeft_Int32 = 2201.
    DriverRequestedMuraFeatherInnerLeftInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerLeft_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerRight_Int32 = 2202.
    DriverRequestedMuraFeatherInnerRightInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerRight_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerTop_Int32 = 2203.
    DriverRequestedMuraFeatherInnerTopInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerTop_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerBottom_Int32 = 2204.
    DriverRequestedMuraFeatherInnerBottomInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerBottom_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterLeft_Int32 = 2205.
    DriverRequestedMuraFeatherOuterLeftInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterLeft_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterRight_Int32 = 2206.
    DriverRequestedMuraFeatherOuterRightInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterRight_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterTop_Int32 = 2207.
    DriverRequestedMuraFeatherOuterTopInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterTop_Int32,
    /// ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterBottom_Int32 = 2208.
    DriverRequestedMuraFeatherOuterBottomInt32 = sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterBottom_Int32,
    /// ETrackedDeviceProperty_Prop_AttachedDeviceId_String = 3000.
    AttachedDeviceIdString = sys::ETrackedDeviceProperty_Prop_AttachedDeviceId_String,
    /// ETrackedDeviceProperty_Prop_SupportedButtons_Uint64 = 3001.
    SupportedButtonsUint64 = sys::ETrackedDeviceProperty_Prop_SupportedButtons_Uint64,
    /// ETrackedDeviceProperty_Prop_Axis0Type_Int32 = 3002.
    Axis0TypeInt32 = sys::ETrackedDeviceProperty_Prop_Axis0Type_Int32,
    /// ETrackedDeviceProperty_Prop_Axis1Type_Int32 = 3003.
    Axis1TypeInt32 = sys::ETrackedDeviceProperty_Prop_Axis1Type_Int32,
    /// ETrackedDeviceProperty_Prop_Axis2Type_Int32 = 3004.
    Axis2TypeInt32 = sys::ETrackedDeviceProperty_Prop_Axis2Type_Int32,
    /// ETrackedDeviceProperty_Prop_Axis3Type_Int32 = 3005.
    Axis3TypeInt32 = sys::ETrackedDeviceProperty_Prop_Axis3Type_Int32,
    /// ETrackedDeviceProperty_Prop_Axis4Type_Int32 = 3006.
    Axis4TypeInt32 = sys::ETrackedDeviceProperty_Prop_Axis4Type_Int32,
    /// ETrackedDeviceProperty_Prop_ControllerRoleHint_Int32 = 3007.
    ControllerRoleHintInt32 = sys::ETrackedDeviceProperty_Prop_ControllerRoleHint_Int32,
    /// ETrackedDeviceProperty_Prop_FieldOfViewLeftDegrees_Float = 4000.
    FieldOfViewLeftDegreesFloat = sys::ETrackedDeviceProperty_Prop_FieldOfViewLeftDegrees_Float,
    /// ETrackedDeviceProperty_Prop_FieldOfViewRightDegrees_Float = 4001.
    FieldOfViewRightDegreesFloat = sys::ETrackedDeviceProperty_Prop_FieldOfViewRightDegrees_Float,
    /// ETrackedDeviceProperty_Prop_FieldOfViewTopDegrees_Float = 4002.
    FieldOfViewTopDegreesFloat = sys::ETrackedDeviceProperty_Prop_FieldOfViewTopDegrees_Float,
    /// ETrackedDeviceProperty_Prop_FieldOfViewBottomDegrees_Float = 4003.
    FieldOfViewBottomDegreesFloat = sys::ETrackedDeviceProperty_Prop_FieldOfViewBottomDegrees_Float,
    /// ETrackedDeviceProperty_Prop_TrackingRangeMinimumMeters_Float = 4004.
    TrackingRangeMinimumMetersFloat = sys::ETrackedDeviceProperty_Prop_TrackingRangeMinimumMeters_Float,
    /// ETrackedDeviceProperty_Prop_TrackingRangeMaximumMeters_Float = 4005.
    TrackingRangeMaximumMetersFloat = sys::ETrackedDeviceProperty_Prop_TrackingRangeMaximumMeters_Float,
    /// ETrackedDeviceProperty_Prop_ModeLabel_String = 4006.
    ModeLabelString = sys::ETrackedDeviceProperty_Prop_ModeLabel_String,
    /// ETrackedDeviceProperty_Prop_IconPathName_String = 5000.
    IconPathNameString = sys::ETrackedDeviceProperty_Prop_IconPathName_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceOff_String = 5001.
    NamedIconPathDeviceOffString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceOff_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearching_String = 5002.
    NamedIconPathDeviceSearchingString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearching_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearchingAlert_String = 5003.
    NamedIconPathDeviceSearchingAlertString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearchingAlert_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceReady_String = 5004.
    NamedIconPathDeviceReadyString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReady_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceReadyAlert_String = 5005.
    NamedIconPathDeviceReadyAlertString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReadyAlert_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceNotReady_String = 5006.
    NamedIconPathDeviceNotReadyString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceNotReady_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceStandby_String = 5007.
    NamedIconPathDeviceStandbyString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceStandby_String,
    /// ETrackedDeviceProperty_Prop_NamedIconPathDeviceAlertLow_String = 5008.
    NamedIconPathDeviceAlertLowString = sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceAlertLow_String,
    /// ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_Start = 5100.
    DisplayHiddenAreaBinaryStart = sys::ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_Start,
    /// ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_End = 5150.
    DisplayHiddenAreaBinaryEnd = sys::ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_End,
    /// ETrackedDeviceProperty_Prop_ParentContainer = 5151.
    ParentContainer = sys::ETrackedDeviceProperty_Prop_ParentContainer,
    /// ETrackedDeviceProperty_Prop_UserConfigPath_String = 6000.
    UserConfigPathString = sys::ETrackedDeviceProperty_Prop_UserConfigPath_String,
    /// ETrackedDeviceProperty_Prop_InstallPath_String = 6001.
    InstallPathString = sys::ETrackedDeviceProperty_Prop_InstallPath_String,
    /// ETrackedDeviceProperty_Prop_HasDisplayComponent_Bool = 6002.
    HasDisplayComponentBool = sys::ETrackedDeviceProperty_Prop_HasDisplayComponent_Bool,
    /// ETrackedDeviceProperty_Prop_HasControllerComponent_Bool = 6003.
    HasControllerComponentBool = sys::ETrackedDeviceProperty_Prop_HasControllerComponent_Bool,
    /// ETrackedDeviceProperty_Prop_HasCameraComponent_Bool = 6004.
    HasCameraComponentBool = sys::ETrackedDeviceProperty_Prop_HasCameraComponent_Bool,
    /// ETrackedDeviceProperty_Prop_HasDriverDirectModeComponent_Bool = 6005.
    HasDriverDirectModeComponentBool = sys::ETrackedDeviceProperty_Prop_HasDriverDirectModeComponent_Bool,
    /// ETrackedDeviceProperty_Prop_HasVirtualDisplayComponent_Bool = 6006.
    HasVirtualDisplayComponentBool = sys::ETrackedDeviceProperty_Prop_HasVirtualDisplayComponent_Bool,
    /// ETrackedDeviceProperty_Prop_HasSpatialAnchorsSupport_Bool = 6007.
    HasSpatialAnchorsSupportBool = sys::ETrackedDeviceProperty_Prop_HasSpatialAnchorsSupport_Bool,
    /// ETrackedDeviceProperty_Prop_ControllerType_String = 7000.
    ControllerTypeString = sys::ETrackedDeviceProperty_Prop_ControllerType_String,
    /// ETrackedDeviceProperty_Prop_LegacyInputProfile_String = 7001.
    LegacyInputProfileString = sys::ETrackedDeviceProperty_Prop_LegacyInputProfile_String,
    /// ETrackedDeviceProperty_Prop_ControllerHandSelectionPriority_Int32 = 7002.
    ControllerHandSelectionPriorityInt32 = sys::ETrackedDeviceProperty_Prop_ControllerHandSelectionPriority_Int32,
    /// ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_Start = 10000.
    VendorSpecificReservedStart = sys::ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_Start,
    /// ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_End = 10999.
    VendorSpecificReservedEnd = sys::ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_End,
    /// ETrackedDeviceProperty_Prop_TrackedDeviceProperty_Max = 1000000.
    TrackedDevicePropertyMax = sys::ETrackedDeviceProperty_Prop_TrackedDeviceProperty_Max,
}

impl Enum for TrackedDeviceProperty {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ETrackedDeviceProperty_Prop_Invalid => Ok(TrackedDeviceProperty::Invalid),
             sys::ETrackedDeviceProperty_Prop_TrackingSystemName_String => Ok(TrackedDeviceProperty::TrackingSystemNameString),
             sys::ETrackedDeviceProperty_Prop_ModelNumber_String => Ok(TrackedDeviceProperty::ModelNumberString),
             sys::ETrackedDeviceProperty_Prop_SerialNumber_String => Ok(TrackedDeviceProperty::SerialNumberString),
             sys::ETrackedDeviceProperty_Prop_RenderModelName_String => Ok(TrackedDeviceProperty::RenderModelNameString),
             sys::ETrackedDeviceProperty_Prop_WillDriftInYaw_Bool => Ok(TrackedDeviceProperty::WillDriftInYawBool),
             sys::ETrackedDeviceProperty_Prop_ManufacturerName_String => Ok(TrackedDeviceProperty::ManufacturerNameString),
             sys::ETrackedDeviceProperty_Prop_TrackingFirmwareVersion_String => Ok(TrackedDeviceProperty::TrackingFirmwareVersionString),
             sys::ETrackedDeviceProperty_Prop_HardwareRevision_String => Ok(TrackedDeviceProperty::HardwareRevisionString),
             sys::ETrackedDeviceProperty_Prop_AllWirelessDongleDescriptions_String => Ok(TrackedDeviceProperty::AllWirelessDongleDescriptionsString),
             sys::ETrackedDeviceProperty_Prop_ConnectedWirelessDongle_String => Ok(TrackedDeviceProperty::ConnectedWirelessDongleString),
             sys::ETrackedDeviceProperty_Prop_DeviceIsWireless_Bool => Ok(TrackedDeviceProperty::DeviceIsWirelessBool),
             sys::ETrackedDeviceProperty_Prop_DeviceIsCharging_Bool => Ok(TrackedDeviceProperty::DeviceIsChargingBool),
             sys::ETrackedDeviceProperty_Prop_DeviceBatteryPercentage_Float => Ok(TrackedDeviceProperty::DeviceBatteryPercentageFloat),
             sys::ETrackedDeviceProperty_Prop_StatusDisplayTransform_Matrix34 => Ok(TrackedDeviceProperty::StatusDisplayTransformMatrix34),
             sys::ETrackedDeviceProperty_Prop_Firmware_UpdateAvailable_Bool => Ok(TrackedDeviceProperty::FirmwareUpdateAvailableBool),
             sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdate_Bool => Ok(TrackedDeviceProperty::FirmwareManualUpdateBool),
             sys::ETrackedDeviceProperty_Prop_Firmware_ManualUpdateURL_String => Ok(TrackedDeviceProperty::FirmwareManualUpdateURLString),
             sys::ETrackedDeviceProperty_Prop_HardwareRevision_Uint64 => Ok(TrackedDeviceProperty::HardwareRevisionUint64),
             sys::ETrackedDeviceProperty_Prop_FirmwareVersion_Uint64 => Ok(TrackedDeviceProperty::FirmwareVersionUint64),
             sys::ETrackedDeviceProperty_Prop_FPGAVersion_Uint64 => Ok(TrackedDeviceProperty::FpgaversionUint64),
             sys::ETrackedDeviceProperty_Prop_VRCVersion_Uint64 => Ok(TrackedDeviceProperty::VrcversionUint64),
             sys::ETrackedDeviceProperty_Prop_RadioVersion_Uint64 => Ok(TrackedDeviceProperty::RadioVersionUint64),
             sys::ETrackedDeviceProperty_Prop_DongleVersion_Uint64 => Ok(TrackedDeviceProperty::DongleVersionUint64),
             sys::ETrackedDeviceProperty_Prop_BlockServerShutdown_Bool => Ok(TrackedDeviceProperty::BlockServerShutdownBool),
             sys::ETrackedDeviceProperty_Prop_CanUnifyCoordinateSystemWithHmd_Bool => Ok(TrackedDeviceProperty::CanUnifyCoordinateSystemWithHmdBool),
             sys::ETrackedDeviceProperty_Prop_ContainsProximitySensor_Bool => Ok(TrackedDeviceProperty::ContainsProximitySensorBool),
             sys::ETrackedDeviceProperty_Prop_DeviceProvidesBatteryStatus_Bool => Ok(TrackedDeviceProperty::DeviceProvidesBatteryStatusBool),
             sys::ETrackedDeviceProperty_Prop_DeviceCanPowerOff_Bool => Ok(TrackedDeviceProperty::DeviceCanPowerOffBool),
             sys::ETrackedDeviceProperty_Prop_Firmware_ProgrammingTarget_String => Ok(TrackedDeviceProperty::FirmwareProgrammingTargetString),
             sys::ETrackedDeviceProperty_Prop_DeviceClass_Int32 => Ok(TrackedDeviceProperty::DeviceClassInt32),
             sys::ETrackedDeviceProperty_Prop_HasCamera_Bool => Ok(TrackedDeviceProperty::HasCameraBool),
             sys::ETrackedDeviceProperty_Prop_DriverVersion_String => Ok(TrackedDeviceProperty::DriverVersionString),
             sys::ETrackedDeviceProperty_Prop_Firmware_ForceUpdateRequired_Bool => Ok(TrackedDeviceProperty::FirmwareForceUpdateRequiredBool),
             sys::ETrackedDeviceProperty_Prop_ViveSystemButtonFixRequired_Bool => Ok(TrackedDeviceProperty::ViveSystemButtonFixRequiredBool),
             sys::ETrackedDeviceProperty_Prop_ParentDriver_Uint64 => Ok(TrackedDeviceProperty::ParentDriverUint64),
             sys::ETrackedDeviceProperty_Prop_ResourceRoot_String => Ok(TrackedDeviceProperty::ResourceRootString),
             sys::ETrackedDeviceProperty_Prop_RegisteredDeviceType_String => Ok(TrackedDeviceProperty::RegisteredDeviceTypeString),
             sys::ETrackedDeviceProperty_Prop_InputProfilePath_String => Ok(TrackedDeviceProperty::InputProfilePathString),
             sys::ETrackedDeviceProperty_Prop_NeverTracked_Bool => Ok(TrackedDeviceProperty::NeverTrackedBool),
             sys::ETrackedDeviceProperty_Prop_NumCameras_Int32 => Ok(TrackedDeviceProperty::NumCamerasInt32),
             sys::ETrackedDeviceProperty_Prop_CameraFrameLayout_Int32 => Ok(TrackedDeviceProperty::CameraFrameLayoutInt32),
             sys::ETrackedDeviceProperty_Prop_CameraStreamFormat_Int32 => Ok(TrackedDeviceProperty::CameraStreamFormatInt32),
             sys::ETrackedDeviceProperty_Prop_AdditionalDeviceSettingsPath_String => Ok(TrackedDeviceProperty::AdditionalDeviceSettingsPathString),
             sys::ETrackedDeviceProperty_Prop_Identifiable_Bool => Ok(TrackedDeviceProperty::IdentifiableBool),
             sys::ETrackedDeviceProperty_Prop_ReportsTimeSinceVSync_Bool => Ok(TrackedDeviceProperty::ReportsTimeSinceVSyncBool),
             sys::ETrackedDeviceProperty_Prop_SecondsFromVsyncToPhotons_Float => Ok(TrackedDeviceProperty::SecondsFromVsyncToPhotonsFloat),
             sys::ETrackedDeviceProperty_Prop_DisplayFrequency_Float => Ok(TrackedDeviceProperty::DisplayFrequencyFloat),
             sys::ETrackedDeviceProperty_Prop_UserIpdMeters_Float => Ok(TrackedDeviceProperty::UserIpdMetersFloat),
             sys::ETrackedDeviceProperty_Prop_CurrentUniverseId_Uint64 => Ok(TrackedDeviceProperty::CurrentUniverseIdUint64),
             sys::ETrackedDeviceProperty_Prop_PreviousUniverseId_Uint64 => Ok(TrackedDeviceProperty::PreviousUniverseIdUint64),
             sys::ETrackedDeviceProperty_Prop_DisplayFirmwareVersion_Uint64 => Ok(TrackedDeviceProperty::DisplayFirmwareVersionUint64),
             sys::ETrackedDeviceProperty_Prop_IsOnDesktop_Bool => Ok(TrackedDeviceProperty::IsOnDesktopBool),
             sys::ETrackedDeviceProperty_Prop_DisplayMCType_Int32 => Ok(TrackedDeviceProperty::DisplayMCTypeInt32),
             sys::ETrackedDeviceProperty_Prop_DisplayMCOffset_Float => Ok(TrackedDeviceProperty::DisplayMCOffsetFloat),
             sys::ETrackedDeviceProperty_Prop_DisplayMCScale_Float => Ok(TrackedDeviceProperty::DisplayMCScaleFloat),
             sys::ETrackedDeviceProperty_Prop_EdidVendorID_Int32 => Ok(TrackedDeviceProperty::EdidVendorIDInt32),
             sys::ETrackedDeviceProperty_Prop_DisplayMCImageLeft_String => Ok(TrackedDeviceProperty::DisplayMCImageLeftString),
             sys::ETrackedDeviceProperty_Prop_DisplayMCImageRight_String => Ok(TrackedDeviceProperty::DisplayMCImageRightString),
             sys::ETrackedDeviceProperty_Prop_DisplayGCBlackClamp_Float => Ok(TrackedDeviceProperty::DisplayGCBlackClampFloat),
             sys::ETrackedDeviceProperty_Prop_EdidProductID_Int32 => Ok(TrackedDeviceProperty::EdidProductIDInt32),
             sys::ETrackedDeviceProperty_Prop_CameraToHeadTransform_Matrix34 => Ok(TrackedDeviceProperty::CameraToHeadTransformMatrix34),
             sys::ETrackedDeviceProperty_Prop_DisplayGCType_Int32 => Ok(TrackedDeviceProperty::DisplayGCTypeInt32),
             sys::ETrackedDeviceProperty_Prop_DisplayGCOffset_Float => Ok(TrackedDeviceProperty::DisplayGCOffsetFloat),
             sys::ETrackedDeviceProperty_Prop_DisplayGCScale_Float => Ok(TrackedDeviceProperty::DisplayGCScaleFloat),
             sys::ETrackedDeviceProperty_Prop_DisplayGCPrescale_Float => Ok(TrackedDeviceProperty::DisplayGCPrescaleFloat),
             sys::ETrackedDeviceProperty_Prop_DisplayGCImage_String => Ok(TrackedDeviceProperty::DisplayGCImageString),
             sys::ETrackedDeviceProperty_Prop_LensCenterLeftU_Float => Ok(TrackedDeviceProperty::LensCenterLeftUFloat),
             sys::ETrackedDeviceProperty_Prop_LensCenterLeftV_Float => Ok(TrackedDeviceProperty::LensCenterLeftVFloat),
             sys::ETrackedDeviceProperty_Prop_LensCenterRightU_Float => Ok(TrackedDeviceProperty::LensCenterRightUFloat),
             sys::ETrackedDeviceProperty_Prop_LensCenterRightV_Float => Ok(TrackedDeviceProperty::LensCenterRightVFloat),
             sys::ETrackedDeviceProperty_Prop_UserHeadToEyeDepthMeters_Float => Ok(TrackedDeviceProperty::UserHeadToEyeDepthMetersFloat),
             sys::ETrackedDeviceProperty_Prop_CameraFirmwareVersion_Uint64 => Ok(TrackedDeviceProperty::CameraFirmwareVersionUint64),
             sys::ETrackedDeviceProperty_Prop_CameraFirmwareDescription_String => Ok(TrackedDeviceProperty::CameraFirmwareDescriptionString),
             sys::ETrackedDeviceProperty_Prop_DisplayFPGAVersion_Uint64 => Ok(TrackedDeviceProperty::DisplayFPGAVersionUint64),
             sys::ETrackedDeviceProperty_Prop_DisplayBootloaderVersion_Uint64 => Ok(TrackedDeviceProperty::DisplayBootloaderVersionUint64),
             sys::ETrackedDeviceProperty_Prop_DisplayHardwareVersion_Uint64 => Ok(TrackedDeviceProperty::DisplayHardwareVersionUint64),
             sys::ETrackedDeviceProperty_Prop_AudioFirmwareVersion_Uint64 => Ok(TrackedDeviceProperty::AudioFirmwareVersionUint64),
             sys::ETrackedDeviceProperty_Prop_CameraCompatibilityMode_Int32 => Ok(TrackedDeviceProperty::CameraCompatibilityModeInt32),
             sys::ETrackedDeviceProperty_Prop_ScreenshotHorizontalFieldOfViewDegrees_Float => Ok(TrackedDeviceProperty::ScreenshotHorizontalFieldOfViewDegreesFloat),
             sys::ETrackedDeviceProperty_Prop_ScreenshotVerticalFieldOfViewDegrees_Float => Ok(TrackedDeviceProperty::ScreenshotVerticalFieldOfViewDegreesFloat),
             sys::ETrackedDeviceProperty_Prop_DisplaySuppressed_Bool => Ok(TrackedDeviceProperty::DisplaySuppressedBool),
             sys::ETrackedDeviceProperty_Prop_DisplayAllowNightMode_Bool => Ok(TrackedDeviceProperty::DisplayAllowNightModeBool),
             sys::ETrackedDeviceProperty_Prop_DisplayMCImageWidth_Int32 => Ok(TrackedDeviceProperty::DisplayMCImageWidthInt32),
             sys::ETrackedDeviceProperty_Prop_DisplayMCImageHeight_Int32 => Ok(TrackedDeviceProperty::DisplayMCImageHeightInt32),
             sys::ETrackedDeviceProperty_Prop_DisplayMCImageNumChannels_Int32 => Ok(TrackedDeviceProperty::DisplayMCImageNumChannelsInt32),
             sys::ETrackedDeviceProperty_Prop_DisplayMCImageData_Binary => Ok(TrackedDeviceProperty::DisplayMCImageDataBinary),
             sys::ETrackedDeviceProperty_Prop_SecondsFromPhotonsToVblank_Float => Ok(TrackedDeviceProperty::SecondsFromPhotonsToVblankFloat),
             sys::ETrackedDeviceProperty_Prop_DriverDirectModeSendsVsyncEvents_Bool => Ok(TrackedDeviceProperty::DriverDirectModeSendsVsyncEventsBool),
             sys::ETrackedDeviceProperty_Prop_DisplayDebugMode_Bool => Ok(TrackedDeviceProperty::DisplayDebugModeBool),
             sys::ETrackedDeviceProperty_Prop_GraphicsAdapterLuid_Uint64 => Ok(TrackedDeviceProperty::GraphicsAdapterLuidUint64),
             sys::ETrackedDeviceProperty_Prop_DriverProvidedChaperonePath_String => Ok(TrackedDeviceProperty::DriverProvidedChaperonePathString),
             sys::ETrackedDeviceProperty_Prop_ExpectedTrackingReferenceCount_Int32 => Ok(TrackedDeviceProperty::ExpectedTrackingReferenceCountInt32),
             sys::ETrackedDeviceProperty_Prop_ExpectedControllerCount_Int32 => Ok(TrackedDeviceProperty::ExpectedControllerCountInt32),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathControllerLeftDeviceOff_String => Ok(TrackedDeviceProperty::NamedIconPathControllerLeftDeviceOffString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathControllerRightDeviceOff_String => Ok(TrackedDeviceProperty::NamedIconPathControllerRightDeviceOffString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathTrackingReferenceDeviceOff_String => Ok(TrackedDeviceProperty::NamedIconPathTrackingReferenceDeviceOffString),
             sys::ETrackedDeviceProperty_Prop_DoNotApplyPrediction_Bool => Ok(TrackedDeviceProperty::DoNotApplyPredictionBool),
             sys::ETrackedDeviceProperty_Prop_CameraToHeadTransforms_Matrix34_Array => Ok(TrackedDeviceProperty::CameraToHeadTransformsMatrix34Array),
             sys::ETrackedDeviceProperty_Prop_DistortionMeshResolution_Int32 => Ok(TrackedDeviceProperty::DistortionMeshResolutionInt32),
             sys::ETrackedDeviceProperty_Prop_DriverIsDrawingControllers_Bool => Ok(TrackedDeviceProperty::DriverIsDrawingControllersBool),
             sys::ETrackedDeviceProperty_Prop_DriverRequestsApplicationPause_Bool => Ok(TrackedDeviceProperty::DriverRequestsApplicationPauseBool),
             sys::ETrackedDeviceProperty_Prop_DriverRequestsReducedRendering_Bool => Ok(TrackedDeviceProperty::DriverRequestsReducedRenderingBool),
             sys::ETrackedDeviceProperty_Prop_MinimumIpdStepMeters_Float => Ok(TrackedDeviceProperty::MinimumIpdStepMetersFloat),
             sys::ETrackedDeviceProperty_Prop_AudioBridgeFirmwareVersion_Uint64 => Ok(TrackedDeviceProperty::AudioBridgeFirmwareVersionUint64),
             sys::ETrackedDeviceProperty_Prop_ImageBridgeFirmwareVersion_Uint64 => Ok(TrackedDeviceProperty::ImageBridgeFirmwareVersionUint64),
             sys::ETrackedDeviceProperty_Prop_ImuToHeadTransform_Matrix34 => Ok(TrackedDeviceProperty::ImuToHeadTransformMatrix34),
             sys::ETrackedDeviceProperty_Prop_ImuFactoryGyroBias_Vector3 => Ok(TrackedDeviceProperty::ImuFactoryGyroBiasVector3),
             sys::ETrackedDeviceProperty_Prop_ImuFactoryGyroScale_Vector3 => Ok(TrackedDeviceProperty::ImuFactoryGyroScaleVector3),
             sys::ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerBias_Vector3 => Ok(TrackedDeviceProperty::ImuFactoryAccelerometerBiasVector3),
             sys::ETrackedDeviceProperty_Prop_ImuFactoryAccelerometerScale_Vector3 => Ok(TrackedDeviceProperty::ImuFactoryAccelerometerScaleVector3),
             sys::ETrackedDeviceProperty_Prop_ConfigurationIncludesLighthouse20Features_Bool => Ok(TrackedDeviceProperty::ConfigurationIncludesLighthouse20FeaturesBool),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraCorrectionMode_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraCorrectionModeInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerLeft_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherInnerLeftInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerRight_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherInnerRightInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerTop_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherInnerTopInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_InnerBottom_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherInnerBottomInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterLeft_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherOuterLeftInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterRight_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherOuterRightInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterTop_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherOuterTopInt32),
             sys::ETrackedDeviceProperty_Prop_DriverRequestedMuraFeather_OuterBottom_Int32 => Ok(TrackedDeviceProperty::DriverRequestedMuraFeatherOuterBottomInt32),
             sys::ETrackedDeviceProperty_Prop_AttachedDeviceId_String => Ok(TrackedDeviceProperty::AttachedDeviceIdString),
             sys::ETrackedDeviceProperty_Prop_SupportedButtons_Uint64 => Ok(TrackedDeviceProperty::SupportedButtonsUint64),
             sys::ETrackedDeviceProperty_Prop_Axis0Type_Int32 => Ok(TrackedDeviceProperty::Axis0TypeInt32),
             sys::ETrackedDeviceProperty_Prop_Axis1Type_Int32 => Ok(TrackedDeviceProperty::Axis1TypeInt32),
             sys::ETrackedDeviceProperty_Prop_Axis2Type_Int32 => Ok(TrackedDeviceProperty::Axis2TypeInt32),
             sys::ETrackedDeviceProperty_Prop_Axis3Type_Int32 => Ok(TrackedDeviceProperty::Axis3TypeInt32),
             sys::ETrackedDeviceProperty_Prop_Axis4Type_Int32 => Ok(TrackedDeviceProperty::Axis4TypeInt32),
             sys::ETrackedDeviceProperty_Prop_ControllerRoleHint_Int32 => Ok(TrackedDeviceProperty::ControllerRoleHintInt32),
             sys::ETrackedDeviceProperty_Prop_FieldOfViewLeftDegrees_Float => Ok(TrackedDeviceProperty::FieldOfViewLeftDegreesFloat),
             sys::ETrackedDeviceProperty_Prop_FieldOfViewRightDegrees_Float => Ok(TrackedDeviceProperty::FieldOfViewRightDegreesFloat),
             sys::ETrackedDeviceProperty_Prop_FieldOfViewTopDegrees_Float => Ok(TrackedDeviceProperty::FieldOfViewTopDegreesFloat),
             sys::ETrackedDeviceProperty_Prop_FieldOfViewBottomDegrees_Float => Ok(TrackedDeviceProperty::FieldOfViewBottomDegreesFloat),
             sys::ETrackedDeviceProperty_Prop_TrackingRangeMinimumMeters_Float => Ok(TrackedDeviceProperty::TrackingRangeMinimumMetersFloat),
             sys::ETrackedDeviceProperty_Prop_TrackingRangeMaximumMeters_Float => Ok(TrackedDeviceProperty::TrackingRangeMaximumMetersFloat),
             sys::ETrackedDeviceProperty_Prop_ModeLabel_String => Ok(TrackedDeviceProperty::ModeLabelString),
             sys::ETrackedDeviceProperty_Prop_IconPathName_String => Ok(TrackedDeviceProperty::IconPathNameString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceOff_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceOffString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearching_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceSearchingString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceSearchingAlert_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceSearchingAlertString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReady_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceReadyString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceReadyAlert_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceReadyAlertString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceNotReady_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceNotReadyString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceStandby_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceStandbyString),
             sys::ETrackedDeviceProperty_Prop_NamedIconPathDeviceAlertLow_String => Ok(TrackedDeviceProperty::NamedIconPathDeviceAlertLowString),
             sys::ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_Start => Ok(TrackedDeviceProperty::DisplayHiddenAreaBinaryStart),
             sys::ETrackedDeviceProperty_Prop_DisplayHiddenArea_Binary_End => Ok(TrackedDeviceProperty::DisplayHiddenAreaBinaryEnd),
             sys::ETrackedDeviceProperty_Prop_ParentContainer => Ok(TrackedDeviceProperty::ParentContainer),
             sys::ETrackedDeviceProperty_Prop_UserConfigPath_String => Ok(TrackedDeviceProperty::UserConfigPathString),
             sys::ETrackedDeviceProperty_Prop_InstallPath_String => Ok(TrackedDeviceProperty::InstallPathString),
             sys::ETrackedDeviceProperty_Prop_HasDisplayComponent_Bool => Ok(TrackedDeviceProperty::HasDisplayComponentBool),
             sys::ETrackedDeviceProperty_Prop_HasControllerComponent_Bool => Ok(TrackedDeviceProperty::HasControllerComponentBool),
             sys::ETrackedDeviceProperty_Prop_HasCameraComponent_Bool => Ok(TrackedDeviceProperty::HasCameraComponentBool),
             sys::ETrackedDeviceProperty_Prop_HasDriverDirectModeComponent_Bool => Ok(TrackedDeviceProperty::HasDriverDirectModeComponentBool),
             sys::ETrackedDeviceProperty_Prop_HasVirtualDisplayComponent_Bool => Ok(TrackedDeviceProperty::HasVirtualDisplayComponentBool),
             sys::ETrackedDeviceProperty_Prop_HasSpatialAnchorsSupport_Bool => Ok(TrackedDeviceProperty::HasSpatialAnchorsSupportBool),
             sys::ETrackedDeviceProperty_Prop_ControllerType_String => Ok(TrackedDeviceProperty::ControllerTypeString),
             sys::ETrackedDeviceProperty_Prop_LegacyInputProfile_String => Ok(TrackedDeviceProperty::LegacyInputProfileString),
             sys::ETrackedDeviceProperty_Prop_ControllerHandSelectionPriority_Int32 => Ok(TrackedDeviceProperty::ControllerHandSelectionPriorityInt32),
             sys::ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_Start => Ok(TrackedDeviceProperty::VendorSpecificReservedStart),
             sys::ETrackedDeviceProperty_Prop_VendorSpecific_Reserved_End => Ok(TrackedDeviceProperty::VendorSpecificReservedEnd),
             sys::ETrackedDeviceProperty_Prop_TrackedDeviceProperty_Max => Ok(TrackedDeviceProperty::TrackedDevicePropertyMax),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackedDeviceProperty> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackedDeviceProperty.", self.0)
    }
}

impl error::Error for Invalid<TrackedDeviceProperty> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackedDeviceProperty."
    }
}

/// ETrackedPropertyError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedPropertyError {
    /// ETrackedPropertyError_TrackedProp_Success = 0.
    Success = sys::ETrackedPropertyError_TrackedProp_Success,
    /// ETrackedPropertyError_TrackedProp_WrongDataType = 1.
    WrongDataType = sys::ETrackedPropertyError_TrackedProp_WrongDataType,
    /// ETrackedPropertyError_TrackedProp_WrongDeviceClass = 2.
    WrongDeviceClass = sys::ETrackedPropertyError_TrackedProp_WrongDeviceClass,
    /// ETrackedPropertyError_TrackedProp_BufferTooSmall = 3.
    BufferTooSmall = sys::ETrackedPropertyError_TrackedProp_BufferTooSmall,
    /// ETrackedPropertyError_TrackedProp_UnknownProperty = 4.
    UnknownProperty = sys::ETrackedPropertyError_TrackedProp_UnknownProperty,
    /// ETrackedPropertyError_TrackedProp_InvalidDevice = 5.
    InvalidDevice = sys::ETrackedPropertyError_TrackedProp_InvalidDevice,
    /// ETrackedPropertyError_TrackedProp_CouldNotContactServer = 6.
    CouldNotContactServer = sys::ETrackedPropertyError_TrackedProp_CouldNotContactServer,
    /// ETrackedPropertyError_TrackedProp_ValueNotProvidedByDevice = 7.
    ValueNotProvidedByDevice = sys::ETrackedPropertyError_TrackedProp_ValueNotProvidedByDevice,
    /// ETrackedPropertyError_TrackedProp_StringExceedsMaximumLength = 8.
    StringExceedsMaximumLength = sys::ETrackedPropertyError_TrackedProp_StringExceedsMaximumLength,
    /// ETrackedPropertyError_TrackedProp_NotYetAvailable = 9.
    NotYetAvailable = sys::ETrackedPropertyError_TrackedProp_NotYetAvailable,
    /// ETrackedPropertyError_TrackedProp_PermissionDenied = 10.
    PermissionDenied = sys::ETrackedPropertyError_TrackedProp_PermissionDenied,
    /// ETrackedPropertyError_TrackedProp_InvalidOperation = 11.
    InvalidOperation = sys::ETrackedPropertyError_TrackedProp_InvalidOperation,
    /// ETrackedPropertyError_TrackedProp_CannotWriteToWildcards = 12.
    CannotWriteToWildcard = sys::ETrackedPropertyError_TrackedProp_CannotWriteToWildcards,
    /// ETrackedPropertyError_TrackedProp_IPCReadFailure = 13.
    IpcreadFailure = sys::ETrackedPropertyError_TrackedProp_IPCReadFailure,
}

impl Enum for TrackedPropertyError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ETrackedPropertyError_TrackedProp_Success => Ok(TrackedPropertyError::Success),
             sys::ETrackedPropertyError_TrackedProp_WrongDataType => Ok(TrackedPropertyError::WrongDataType),
             sys::ETrackedPropertyError_TrackedProp_WrongDeviceClass => Ok(TrackedPropertyError::WrongDeviceClass),
             sys::ETrackedPropertyError_TrackedProp_BufferTooSmall => Ok(TrackedPropertyError::BufferTooSmall),
             sys::ETrackedPropertyError_TrackedProp_UnknownProperty => Ok(TrackedPropertyError::UnknownProperty),
             sys::ETrackedPropertyError_TrackedProp_InvalidDevice => Ok(TrackedPropertyError::InvalidDevice),
             sys::ETrackedPropertyError_TrackedProp_CouldNotContactServer => Ok(TrackedPropertyError::CouldNotContactServer),
             sys::ETrackedPropertyError_TrackedProp_ValueNotProvidedByDevice => Ok(TrackedPropertyError::ValueNotProvidedByDevice),
             sys::ETrackedPropertyError_TrackedProp_StringExceedsMaximumLength => Ok(TrackedPropertyError::StringExceedsMaximumLength),
             sys::ETrackedPropertyError_TrackedProp_NotYetAvailable => Ok(TrackedPropertyError::NotYetAvailable),
             sys::ETrackedPropertyError_TrackedProp_PermissionDenied => Ok(TrackedPropertyError::PermissionDenied),
             sys::ETrackedPropertyError_TrackedProp_InvalidOperation => Ok(TrackedPropertyError::InvalidOperation),
             sys::ETrackedPropertyError_TrackedProp_CannotWriteToWildcards => Ok(TrackedPropertyError::CannotWriteToWildcard),
             sys::ETrackedPropertyError_TrackedProp_IPCReadFailure => Ok(TrackedPropertyError::IpcreadFailure),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackedPropertyError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackedPropertyError.", self.0)
    }
}

impl error::Error for Invalid<TrackedPropertyError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackedPropertyError."
    }
}

/// EVRSubmitFlags.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SubmitFlag {
    /// EVRSubmitFlags_Submit_Default = 0.
    Default = sys::EVRSubmitFlags_Submit_Default,
    /// EVRSubmitFlags_Submit_LensDistortionAlreadyApplied = 1.
    LensDistortionAlreadyApplied = sys::EVRSubmitFlags_Submit_LensDistortionAlreadyApplied,
    /// EVRSubmitFlags_Submit_GlRenderBuffer = 2.
    GlRenderBuffer = sys::EVRSubmitFlags_Submit_GlRenderBuffer,
    /// EVRSubmitFlags_Submit_Reserved = 4.
    Reserved = sys::EVRSubmitFlags_Submit_Reserved,
    /// EVRSubmitFlags_Submit_TextureWithPose = 8.
    TextureWithPose = sys::EVRSubmitFlags_Submit_TextureWithPose,
    /// EVRSubmitFlags_Submit_TextureWithDepth = 16.
    TextureWithDepth = sys::EVRSubmitFlags_Submit_TextureWithDepth,
}

impl Enum for SubmitFlag {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRSubmitFlags_Submit_Default => Ok(SubmitFlag::Default),
             sys::EVRSubmitFlags_Submit_LensDistortionAlreadyApplied => Ok(SubmitFlag::LensDistortionAlreadyApplied),
             sys::EVRSubmitFlags_Submit_GlRenderBuffer => Ok(SubmitFlag::GlRenderBuffer),
             sys::EVRSubmitFlags_Submit_Reserved => Ok(SubmitFlag::Reserved),
             sys::EVRSubmitFlags_Submit_TextureWithPose => Ok(SubmitFlag::TextureWithPose),
             sys::EVRSubmitFlags_Submit_TextureWithDepth => Ok(SubmitFlag::TextureWithDepth),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<SubmitFlag> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of SubmitFlag.", self.0)
    }
}

impl error::Error for Invalid<SubmitFlag> {
    fn description(&self) -> &str {
        "Value does not represent any variant of SubmitFlag."
    }
}

/// EVRState.
#[repr(i32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum State {
    /// EVRState_VRState_Undefined = -1.
    Undefined = sys::EVRState_VRState_Undefined,
    /// EVRState_VRState_Off = 0.
    Off = sys::EVRState_VRState_Off,
    /// EVRState_VRState_Searching = 1.
    Searching = sys::EVRState_VRState_Searching,
    /// EVRState_VRState_Searching_Alert = 2.
    SearchingAlert = sys::EVRState_VRState_Searching_Alert,
    /// EVRState_VRState_Ready = 3.
    Ready = sys::EVRState_VRState_Ready,
    /// EVRState_VRState_Ready_Alert = 4.
    ReadyAlert = sys::EVRState_VRState_Ready_Alert,
    /// EVRState_VRState_NotReady = 5.
    NotReady = sys::EVRState_VRState_NotReady,
    /// EVRState_VRState_Standby = 6.
    Standby = sys::EVRState_VRState_Standby,
    /// EVRState_VRState_Ready_Alert_Low = 7.
    ReadyAlertLow = sys::EVRState_VRState_Ready_Alert_Low,
}

impl Enum for State {
    type Raw = i32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRState_VRState_Undefined => Ok(State::Undefined),
             sys::EVRState_VRState_Off => Ok(State::Off),
             sys::EVRState_VRState_Searching => Ok(State::Searching),
             sys::EVRState_VRState_Searching_Alert => Ok(State::SearchingAlert),
             sys::EVRState_VRState_Ready => Ok(State::Ready),
             sys::EVRState_VRState_Ready_Alert => Ok(State::ReadyAlert),
             sys::EVRState_VRState_NotReady => Ok(State::NotReady),
             sys::EVRState_VRState_Standby => Ok(State::Standby),
             sys::EVRState_VRState_Ready_Alert_Low => Ok(State::ReadyAlertLow),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<State> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of State.", self.0)
    }
}

impl error::Error for Invalid<State> {
    fn description(&self) -> &str {
        "Value does not represent any variant of State."
    }
}

/// EVREventType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EventType {
    /// EVREventType_VREvent_None = 0.
    None = sys::EVREventType_VREvent_None,
    /// EVREventType_VREvent_TrackedDeviceActivated = 100.
    TrackedDeviceActivated = sys::EVREventType_VREvent_TrackedDeviceActivated,
    /// EVREventType_VREvent_TrackedDeviceDeactivated = 101.
    TrackedDeviceDeactivated = sys::EVREventType_VREvent_TrackedDeviceDeactivated,
    /// EVREventType_VREvent_TrackedDeviceUpdated = 102.
    TrackedDeviceUpdated = sys::EVREventType_VREvent_TrackedDeviceUpdated,
    /// EVREventType_VREvent_TrackedDeviceUserInteractionStarted = 103.
    TrackedDeviceUserInteractionStarted = sys::EVREventType_VREvent_TrackedDeviceUserInteractionStarted,
    /// EVREventType_VREvent_TrackedDeviceUserInteractionEnded = 104.
    TrackedDeviceUserInteractionEnded = sys::EVREventType_VREvent_TrackedDeviceUserInteractionEnded,
    /// EVREventType_VREvent_IpdChanged = 105.
    IpdChanged = sys::EVREventType_VREvent_IpdChanged,
    /// EVREventType_VREvent_EnterStandbyMode = 106.
    EnterStandbyMode = sys::EVREventType_VREvent_EnterStandbyMode,
    /// EVREventType_VREvent_LeaveStandbyMode = 107.
    LeaveStandbyMode = sys::EVREventType_VREvent_LeaveStandbyMode,
    /// EVREventType_VREvent_TrackedDeviceRoleChanged = 108.
    TrackedDeviceRoleChanged = sys::EVREventType_VREvent_TrackedDeviceRoleChanged,
    /// EVREventType_VREvent_WatchdogWakeUpRequested = 109.
    WatchdogWakeUpRequested = sys::EVREventType_VREvent_WatchdogWakeUpRequested,
    /// EVREventType_VREvent_LensDistortionChanged = 110.
    LensDistortionChanged = sys::EVREventType_VREvent_LensDistortionChanged,
    /// EVREventType_VREvent_PropertyChanged = 111.
    PropertyChanged = sys::EVREventType_VREvent_PropertyChanged,
    /// EVREventType_VREvent_WirelessDisconnect = 112.
    WirelessDisconnect = sys::EVREventType_VREvent_WirelessDisconnect,
    /// EVREventType_VREvent_WirelessReconnect = 113.
    WirelessReconnect = sys::EVREventType_VREvent_WirelessReconnect,
    /// EVREventType_VREvent_ButtonPress = 200.
    ButtonPress = sys::EVREventType_VREvent_ButtonPress,
    /// EVREventType_VREvent_ButtonUnpress = 201.
    ButtonUnpress = sys::EVREventType_VREvent_ButtonUnpress,
    /// EVREventType_VREvent_ButtonTouch = 202.
    ButtonTouch = sys::EVREventType_VREvent_ButtonTouch,
    /// EVREventType_VREvent_ButtonUntouch = 203.
    ButtonUntouch = sys::EVREventType_VREvent_ButtonUntouch,
    /// EVREventType_VREvent_DualAnalog_Press = 250.
    DualAnalogPress = sys::EVREventType_VREvent_DualAnalog_Press,
    /// EVREventType_VREvent_DualAnalog_Unpress = 251.
    DualAnalogUnpress = sys::EVREventType_VREvent_DualAnalog_Unpress,
    /// EVREventType_VREvent_DualAnalog_Touch = 252.
    DualAnalogTouch = sys::EVREventType_VREvent_DualAnalog_Touch,
    /// EVREventType_VREvent_DualAnalog_Untouch = 253.
    DualAnalogUntouch = sys::EVREventType_VREvent_DualAnalog_Untouch,
    /// EVREventType_VREvent_DualAnalog_Move = 254.
    DualAnalogMove = sys::EVREventType_VREvent_DualAnalog_Move,
    /// EVREventType_VREvent_DualAnalog_ModeSwitch1 = 255.
    DualAnalogModeSwitch1 = sys::EVREventType_VREvent_DualAnalog_ModeSwitch1,
    /// EVREventType_VREvent_DualAnalog_ModeSwitch2 = 256.
    DualAnalogModeSwitch2 = sys::EVREventType_VREvent_DualAnalog_ModeSwitch2,
    /// EVREventType_VREvent_DualAnalog_Cancel = 257.
    DualAnalogCancel = sys::EVREventType_VREvent_DualAnalog_Cancel,
    /// EVREventType_VREvent_MouseMove = 300.
    MouseMove = sys::EVREventType_VREvent_MouseMove,
    /// EVREventType_VREvent_MouseButtonDown = 301.
    MouseButtonDown = sys::EVREventType_VREvent_MouseButtonDown,
    /// EVREventType_VREvent_MouseButtonUp = 302.
    MouseButtonUp = sys::EVREventType_VREvent_MouseButtonUp,
    /// EVREventType_VREvent_FocusEnter = 303.
    FocusEnter = sys::EVREventType_VREvent_FocusEnter,
    /// EVREventType_VREvent_FocusLeave = 304.
    FocusLeave = sys::EVREventType_VREvent_FocusLeave,
    /// EVREventType_VREvent_Scroll = 305.
    Scroll = sys::EVREventType_VREvent_Scroll,
    /// EVREventType_VREvent_TouchPadMove = 306.
    TouchPadMove = sys::EVREventType_VREvent_TouchPadMove,
    /// EVREventType_VREvent_OverlayFocusChanged = 307.
    OverlayFocusChanged = sys::EVREventType_VREvent_OverlayFocusChanged,
    /// EVREventType_VREvent_ReloadOverlays = 308.
    ReloadOverlay = sys::EVREventType_VREvent_ReloadOverlays,
    /// EVREventType_VREvent_InputFocusCaptured = 400.
    InputFocusCaptured = sys::EVREventType_VREvent_InputFocusCaptured,
    /// EVREventType_VREvent_InputFocusReleased = 401.
    InputFocusReleased = sys::EVREventType_VREvent_InputFocusReleased,
    /// EVREventType_VREvent_SceneFocusLost = 402.
    SceneFocusLost = sys::EVREventType_VREvent_SceneFocusLost,
    /// EVREventType_VREvent_SceneFocusGained = 403.
    SceneFocusGained = sys::EVREventType_VREvent_SceneFocusGained,
    /// EVREventType_VREvent_SceneApplicationChanged = 404.
    SceneApplicationChanged = sys::EVREventType_VREvent_SceneApplicationChanged,
    /// EVREventType_VREvent_SceneFocusChanged = 405.
    SceneFocusChanged = sys::EVREventType_VREvent_SceneFocusChanged,
    /// EVREventType_VREvent_InputFocusChanged = 406.
    InputFocusChanged = sys::EVREventType_VREvent_InputFocusChanged,
    /// EVREventType_VREvent_SceneApplicationSecondaryRenderingStarted = 407.
    SceneApplicationSecondaryRenderingStarted = sys::EVREventType_VREvent_SceneApplicationSecondaryRenderingStarted,
    /// EVREventType_VREvent_SceneApplicationUsingWrongGraphicsAdapter = 408.
    SceneApplicationUsingWrongGraphicsAdapter = sys::EVREventType_VREvent_SceneApplicationUsingWrongGraphicsAdapter,
    /// EVREventType_VREvent_ActionBindingReloaded = 409.
    ActionBindingReloaded = sys::EVREventType_VREvent_ActionBindingReloaded,
    /// EVREventType_VREvent_HideRenderModels = 410.
    HideRenderModel = sys::EVREventType_VREvent_HideRenderModels,
    /// EVREventType_VREvent_ShowRenderModels = 411.
    ShowRenderModel = sys::EVREventType_VREvent_ShowRenderModels,
    /// EVREventType_VREvent_ConsoleOpened = 420.
    ConsoleOpened = sys::EVREventType_VREvent_ConsoleOpened,
    /// EVREventType_VREvent_ConsoleClosed = 421.
    ConsoleClosed = sys::EVREventType_VREvent_ConsoleClosed,
    /// EVREventType_VREvent_OverlayShown = 500.
    OverlayShown = sys::EVREventType_VREvent_OverlayShown,
    /// EVREventType_VREvent_OverlayHidden = 501.
    OverlayHidden = sys::EVREventType_VREvent_OverlayHidden,
    /// EVREventType_VREvent_DashboardActivated = 502.
    DashboardActivated = sys::EVREventType_VREvent_DashboardActivated,
    /// EVREventType_VREvent_DashboardDeactivated = 503.
    DashboardDeactivated = sys::EVREventType_VREvent_DashboardDeactivated,
    /// EVREventType_VREvent_DashboardThumbSelected = 504.
    DashboardThumbSelected = sys::EVREventType_VREvent_DashboardThumbSelected,
    /// EVREventType_VREvent_DashboardRequested = 505.
    DashboardRequested = sys::EVREventType_VREvent_DashboardRequested,
    /// EVREventType_VREvent_ResetDashboard = 506.
    ResetDashboard = sys::EVREventType_VREvent_ResetDashboard,
    /// EVREventType_VREvent_RenderToast = 507.
    RenderToast = sys::EVREventType_VREvent_RenderToast,
    /// EVREventType_VREvent_ImageLoaded = 508.
    ImageLoaded = sys::EVREventType_VREvent_ImageLoaded,
    /// EVREventType_VREvent_ShowKeyboard = 509.
    ShowKeyboard = sys::EVREventType_VREvent_ShowKeyboard,
    /// EVREventType_VREvent_HideKeyboard = 510.
    HideKeyboard = sys::EVREventType_VREvent_HideKeyboard,
    /// EVREventType_VREvent_OverlayGamepadFocusGained = 511.
    OverlayGamepadFocusGained = sys::EVREventType_VREvent_OverlayGamepadFocusGained,
    /// EVREventType_VREvent_OverlayGamepadFocusLost = 512.
    OverlayGamepadFocusLost = sys::EVREventType_VREvent_OverlayGamepadFocusLost,
    /// EVREventType_VREvent_OverlaySharedTextureChanged = 513.
    OverlaySharedTextureChanged = sys::EVREventType_VREvent_OverlaySharedTextureChanged,
    /// EVREventType_VREvent_ScreenshotTriggered = 516.
    ScreenshotTriggered = sys::EVREventType_VREvent_ScreenshotTriggered,
    /// EVREventType_VREvent_ImageFailed = 517.
    ImageFailed = sys::EVREventType_VREvent_ImageFailed,
    /// EVREventType_VREvent_DashboardOverlayCreated = 518.
    DashboardOverlayCreated = sys::EVREventType_VREvent_DashboardOverlayCreated,
    /// EVREventType_VREvent_SwitchGamepadFocus = 519.
    SwitchGamepadFocu = sys::EVREventType_VREvent_SwitchGamepadFocus,
    /// EVREventType_VREvent_RequestScreenshot = 520.
    RequestScreenshot = sys::EVREventType_VREvent_RequestScreenshot,
    /// EVREventType_VREvent_ScreenshotTaken = 521.
    ScreenshotTaken = sys::EVREventType_VREvent_ScreenshotTaken,
    /// EVREventType_VREvent_ScreenshotFailed = 522.
    ScreenshotFailed = sys::EVREventType_VREvent_ScreenshotFailed,
    /// EVREventType_VREvent_SubmitScreenshotToDashboard = 523.
    SubmitScreenshotToDashboard = sys::EVREventType_VREvent_SubmitScreenshotToDashboard,
    /// EVREventType_VREvent_ScreenshotProgressToDashboard = 524.
    ScreenshotProgressToDashboard = sys::EVREventType_VREvent_ScreenshotProgressToDashboard,
    /// EVREventType_VREvent_PrimaryDashboardDeviceChanged = 525.
    PrimaryDashboardDeviceChanged = sys::EVREventType_VREvent_PrimaryDashboardDeviceChanged,
    /// EVREventType_VREvent_RoomViewShown = 526.
    RoomViewShown = sys::EVREventType_VREvent_RoomViewShown,
    /// EVREventType_VREvent_RoomViewHidden = 527.
    RoomViewHidden = sys::EVREventType_VREvent_RoomViewHidden,
    /// EVREventType_VREvent_ShowUI = 528.
    ShowUI = sys::EVREventType_VREvent_ShowUI,
    /// EVREventType_VREvent_Notification_Shown = 600.
    NotificationShown = sys::EVREventType_VREvent_Notification_Shown,
    /// EVREventType_VREvent_Notification_Hidden = 601.
    NotificationHidden = sys::EVREventType_VREvent_Notification_Hidden,
    /// EVREventType_VREvent_Notification_BeginInteraction = 602.
    NotificationBeginInteraction = sys::EVREventType_VREvent_Notification_BeginInteraction,
    /// EVREventType_VREvent_Notification_Destroyed = 603.
    NotificationDestroyed = sys::EVREventType_VREvent_Notification_Destroyed,
    /// EVREventType_VREvent_Quit = 700.
    Quit = sys::EVREventType_VREvent_Quit,
    /// EVREventType_VREvent_ProcessQuit = 701.
    ProcessQuit = sys::EVREventType_VREvent_ProcessQuit,
    /// EVREventType_VREvent_QuitAborted_UserPrompt = 702.
    QuitAbortedUserPrompt = sys::EVREventType_VREvent_QuitAborted_UserPrompt,
    /// EVREventType_VREvent_QuitAcknowledged = 703.
    QuitAcknowledged = sys::EVREventType_VREvent_QuitAcknowledged,
    /// EVREventType_VREvent_DriverRequestedQuit = 704.
    DriverRequestedQuit = sys::EVREventType_VREvent_DriverRequestedQuit,
    /// EVREventType_VREvent_ChaperoneDataHasChanged = 800.
    ChaperoneDataHasChanged = sys::EVREventType_VREvent_ChaperoneDataHasChanged,
    /// EVREventType_VREvent_ChaperoneUniverseHasChanged = 801.
    ChaperoneUniverseHasChanged = sys::EVREventType_VREvent_ChaperoneUniverseHasChanged,
    /// EVREventType_VREvent_ChaperoneTempDataHasChanged = 802.
    ChaperoneTempDataHasChanged = sys::EVREventType_VREvent_ChaperoneTempDataHasChanged,
    /// EVREventType_VREvent_ChaperoneSettingsHaveChanged = 803.
    ChaperoneSettingsHaveChanged = sys::EVREventType_VREvent_ChaperoneSettingsHaveChanged,
    /// EVREventType_VREvent_SeatedZeroPoseReset = 804.
    SeatedZeroPoseReset = sys::EVREventType_VREvent_SeatedZeroPoseReset,
    /// EVREventType_VREvent_ChaperoneFlushCache = 805.
    ChaperoneFlushCache = sys::EVREventType_VREvent_ChaperoneFlushCache,
    /// EVREventType_VREvent_AudioSettingsHaveChanged = 820.
    AudioSettingsHaveChanged = sys::EVREventType_VREvent_AudioSettingsHaveChanged,
    /// EVREventType_VREvent_BackgroundSettingHasChanged = 850.
    BackgroundSettingHasChanged = sys::EVREventType_VREvent_BackgroundSettingHasChanged,
    /// EVREventType_VREvent_CameraSettingsHaveChanged = 851.
    CameraSettingsHaveChanged = sys::EVREventType_VREvent_CameraSettingsHaveChanged,
    /// EVREventType_VREvent_ReprojectionSettingHasChanged = 852.
    ReprojectionSettingHasChanged = sys::EVREventType_VREvent_ReprojectionSettingHasChanged,
    /// EVREventType_VREvent_ModelSkinSettingsHaveChanged = 853.
    ModelSkinSettingsHaveChanged = sys::EVREventType_VREvent_ModelSkinSettingsHaveChanged,
    /// EVREventType_VREvent_EnvironmentSettingsHaveChanged = 854.
    EnvironmentSettingsHaveChanged = sys::EVREventType_VREvent_EnvironmentSettingsHaveChanged,
    /// EVREventType_VREvent_PowerSettingsHaveChanged = 855.
    PowerSettingsHaveChanged = sys::EVREventType_VREvent_PowerSettingsHaveChanged,
    /// EVREventType_VREvent_EnableHomeAppSettingsHaveChanged = 856.
    EnableHomeAppSettingsHaveChanged = sys::EVREventType_VREvent_EnableHomeAppSettingsHaveChanged,
    /// EVREventType_VREvent_SteamVRSectionSettingChanged = 857.
    SteamVRSectionSettingChanged = sys::EVREventType_VREvent_SteamVRSectionSettingChanged,
    /// EVREventType_VREvent_LighthouseSectionSettingChanged = 858.
    LighthouseSectionSettingChanged = sys::EVREventType_VREvent_LighthouseSectionSettingChanged,
    /// EVREventType_VREvent_NullSectionSettingChanged = 859.
    NullSectionSettingChanged = sys::EVREventType_VREvent_NullSectionSettingChanged,
    /// EVREventType_VREvent_UserInterfaceSectionSettingChanged = 860.
    UserInterfaceSectionSettingChanged = sys::EVREventType_VREvent_UserInterfaceSectionSettingChanged,
    /// EVREventType_VREvent_NotificationsSectionSettingChanged = 861.
    NotificationsSectionSettingChanged = sys::EVREventType_VREvent_NotificationsSectionSettingChanged,
    /// EVREventType_VREvent_KeyboardSectionSettingChanged = 862.
    KeyboardSectionSettingChanged = sys::EVREventType_VREvent_KeyboardSectionSettingChanged,
    /// EVREventType_VREvent_PerfSectionSettingChanged = 863.
    PerfSectionSettingChanged = sys::EVREventType_VREvent_PerfSectionSettingChanged,
    /// EVREventType_VREvent_DashboardSectionSettingChanged = 864.
    DashboardSectionSettingChanged = sys::EVREventType_VREvent_DashboardSectionSettingChanged,
    /// EVREventType_VREvent_WebInterfaceSectionSettingChanged = 865.
    WebInterfaceSectionSettingChanged = sys::EVREventType_VREvent_WebInterfaceSectionSettingChanged,
    /// EVREventType_VREvent_TrackersSectionSettingChanged = 866.
    TrackersSectionSettingChanged = sys::EVREventType_VREvent_TrackersSectionSettingChanged,
    /// EVREventType_VREvent_LastKnownSectionSettingChanged = 867.
    LastKnownSectionSettingChanged = sys::EVREventType_VREvent_LastKnownSectionSettingChanged,
    /// EVREventType_VREvent_StatusUpdate = 900.
    StatusUpdate = sys::EVREventType_VREvent_StatusUpdate,
    /// EVREventType_VREvent_WebInterface_InstallDriverCompleted = 950.
    WebInterfaceInstallDriverCompleted = sys::EVREventType_VREvent_WebInterface_InstallDriverCompleted,
    /// EVREventType_VREvent_MCImageUpdated = 1000.
    McimageUpdated = sys::EVREventType_VREvent_MCImageUpdated,
    /// EVREventType_VREvent_FirmwareUpdateStarted = 1100.
    FirmwareUpdateStarted = sys::EVREventType_VREvent_FirmwareUpdateStarted,
    /// EVREventType_VREvent_FirmwareUpdateFinished = 1101.
    FirmwareUpdateFinished = sys::EVREventType_VREvent_FirmwareUpdateFinished,
    /// EVREventType_VREvent_KeyboardClosed = 1200.
    KeyboardClosed = sys::EVREventType_VREvent_KeyboardClosed,
    /// EVREventType_VREvent_KeyboardCharInput = 1201.
    KeyboardCharInput = sys::EVREventType_VREvent_KeyboardCharInput,
    /// EVREventType_VREvent_KeyboardDone = 1202.
    KeyboardDone = sys::EVREventType_VREvent_KeyboardDone,
    /// EVREventType_VREvent_ApplicationTransitionStarted = 1300.
    ApplicationTransitionStarted = sys::EVREventType_VREvent_ApplicationTransitionStarted,
    /// EVREventType_VREvent_ApplicationTransitionAborted = 1301.
    ApplicationTransitionAborted = sys::EVREventType_VREvent_ApplicationTransitionAborted,
    /// EVREventType_VREvent_ApplicationTransitionNewAppStarted = 1302.
    ApplicationTransitionNewAppStarted = sys::EVREventType_VREvent_ApplicationTransitionNewAppStarted,
    /// EVREventType_VREvent_ApplicationListUpdated = 1303.
    ApplicationListUpdated = sys::EVREventType_VREvent_ApplicationListUpdated,
    /// EVREventType_VREvent_ApplicationMimeTypeLoad = 1304.
    ApplicationMimeTypeLoad = sys::EVREventType_VREvent_ApplicationMimeTypeLoad,
    /// EVREventType_VREvent_ApplicationTransitionNewAppLaunchComplete = 1305.
    ApplicationTransitionNewAppLaunchComplete = sys::EVREventType_VREvent_ApplicationTransitionNewAppLaunchComplete,
    /// EVREventType_VREvent_ProcessConnected = 1306.
    ProcessConnected = sys::EVREventType_VREvent_ProcessConnected,
    /// EVREventType_VREvent_ProcessDisconnected = 1307.
    ProcessDisconnected = sys::EVREventType_VREvent_ProcessDisconnected,
    /// EVREventType_VREvent_Compositor_MirrorWindowShown = 1400.
    CompositorMirrorWindowShown = sys::EVREventType_VREvent_Compositor_MirrorWindowShown,
    /// EVREventType_VREvent_Compositor_MirrorWindowHidden = 1401.
    CompositorMirrorWindowHidden = sys::EVREventType_VREvent_Compositor_MirrorWindowHidden,
    /// EVREventType_VREvent_Compositor_ChaperoneBoundsShown = 1410.
    CompositorChaperoneBoundsShown = sys::EVREventType_VREvent_Compositor_ChaperoneBoundsShown,
    /// EVREventType_VREvent_Compositor_ChaperoneBoundsHidden = 1411.
    CompositorChaperoneBoundsHidden = sys::EVREventType_VREvent_Compositor_ChaperoneBoundsHidden,
    /// EVREventType_VREvent_TrackedCamera_StartVideoStream = 1500.
    TrackedCameraStartVideoStream = sys::EVREventType_VREvent_TrackedCamera_StartVideoStream,
    /// EVREventType_VREvent_TrackedCamera_StopVideoStream = 1501.
    TrackedCameraStopVideoStream = sys::EVREventType_VREvent_TrackedCamera_StopVideoStream,
    /// EVREventType_VREvent_TrackedCamera_PauseVideoStream = 1502.
    TrackedCameraPauseVideoStream = sys::EVREventType_VREvent_TrackedCamera_PauseVideoStream,
    /// EVREventType_VREvent_TrackedCamera_ResumeVideoStream = 1503.
    TrackedCameraResumeVideoStream = sys::EVREventType_VREvent_TrackedCamera_ResumeVideoStream,
    /// EVREventType_VREvent_TrackedCamera_EditingSurface = 1550.
    TrackedCameraEditingSurface = sys::EVREventType_VREvent_TrackedCamera_EditingSurface,
    /// EVREventType_VREvent_PerformanceTest_EnableCapture = 1600.
    PerformanceTestEnableCapture = sys::EVREventType_VREvent_PerformanceTest_EnableCapture,
    /// EVREventType_VREvent_PerformanceTest_DisableCapture = 1601.
    PerformanceTestDisableCapture = sys::EVREventType_VREvent_PerformanceTest_DisableCapture,
    /// EVREventType_VREvent_PerformanceTest_FidelityLevel = 1602.
    PerformanceTestFidelityLevel = sys::EVREventType_VREvent_PerformanceTest_FidelityLevel,
    /// EVREventType_VREvent_MessageOverlay_Closed = 1650.
    MessageOverlayClosed = sys::EVREventType_VREvent_MessageOverlay_Closed,
    /// EVREventType_VREvent_MessageOverlayCloseRequested = 1651.
    MessageOverlayCloseRequested = sys::EVREventType_VREvent_MessageOverlayCloseRequested,
    /// EVREventType_VREvent_Input_HapticVibration = 1700.
    InputHapticVibration = sys::EVREventType_VREvent_Input_HapticVibration,
    /// EVREventType_VREvent_Input_BindingLoadFailed = 1701.
    InputBindingLoadFailed = sys::EVREventType_VREvent_Input_BindingLoadFailed,
    /// EVREventType_VREvent_Input_BindingLoadSuccessful = 1702.
    InputBindingLoadSuccessful = sys::EVREventType_VREvent_Input_BindingLoadSuccessful,
    /// EVREventType_VREvent_Input_ActionManifestReloaded = 1703.
    InputActionManifestReloaded = sys::EVREventType_VREvent_Input_ActionManifestReloaded,
    /// EVREventType_VREvent_Input_ActionManifestLoadFailed = 1704.
    InputActionManifestLoadFailed = sys::EVREventType_VREvent_Input_ActionManifestLoadFailed,
    /// EVREventType_VREvent_Input_ProgressUpdate = 1705.
    InputProgressUpdate = sys::EVREventType_VREvent_Input_ProgressUpdate,
    /// EVREventType_VREvent_Input_TrackerActivated = 1706.
    InputTrackerActivated = sys::EVREventType_VREvent_Input_TrackerActivated,
    /// EVREventType_VREvent_SpatialAnchors_PoseUpdated = 1800.
    SpatialAnchorsPoseUpdated = sys::EVREventType_VREvent_SpatialAnchors_PoseUpdated,
    /// EVREventType_VREvent_SpatialAnchors_DescriptorUpdated = 1801.
    SpatialAnchorsDescriptorUpdated = sys::EVREventType_VREvent_SpatialAnchors_DescriptorUpdated,
    /// EVREventType_VREvent_SpatialAnchors_RequestPoseUpdate = 1802.
    SpatialAnchorsRequestPoseUpdate = sys::EVREventType_VREvent_SpatialAnchors_RequestPoseUpdate,
    /// EVREventType_VREvent_SpatialAnchors_RequestDescriptorUpdate = 1803.
    SpatialAnchorsRequestDescriptorUpdate = sys::EVREventType_VREvent_SpatialAnchors_RequestDescriptorUpdate,
    /// EVREventType_VREvent_VendorSpecific_Reserved_Start = 10000.
    VendorSpecificReservedStart = sys::EVREventType_VREvent_VendorSpecific_Reserved_Start,
    /// EVREventType_VREvent_VendorSpecific_Reserved_End = 19999.
    VendorSpecificReservedEnd = sys::EVREventType_VREvent_VendorSpecific_Reserved_End,
}

impl Enum for EventType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVREventType_VREvent_None => Ok(EventType::None),
             sys::EVREventType_VREvent_TrackedDeviceActivated => Ok(EventType::TrackedDeviceActivated),
             sys::EVREventType_VREvent_TrackedDeviceDeactivated => Ok(EventType::TrackedDeviceDeactivated),
             sys::EVREventType_VREvent_TrackedDeviceUpdated => Ok(EventType::TrackedDeviceUpdated),
             sys::EVREventType_VREvent_TrackedDeviceUserInteractionStarted => Ok(EventType::TrackedDeviceUserInteractionStarted),
             sys::EVREventType_VREvent_TrackedDeviceUserInteractionEnded => Ok(EventType::TrackedDeviceUserInteractionEnded),
             sys::EVREventType_VREvent_IpdChanged => Ok(EventType::IpdChanged),
             sys::EVREventType_VREvent_EnterStandbyMode => Ok(EventType::EnterStandbyMode),
             sys::EVREventType_VREvent_LeaveStandbyMode => Ok(EventType::LeaveStandbyMode),
             sys::EVREventType_VREvent_TrackedDeviceRoleChanged => Ok(EventType::TrackedDeviceRoleChanged),
             sys::EVREventType_VREvent_WatchdogWakeUpRequested => Ok(EventType::WatchdogWakeUpRequested),
             sys::EVREventType_VREvent_LensDistortionChanged => Ok(EventType::LensDistortionChanged),
             sys::EVREventType_VREvent_PropertyChanged => Ok(EventType::PropertyChanged),
             sys::EVREventType_VREvent_WirelessDisconnect => Ok(EventType::WirelessDisconnect),
             sys::EVREventType_VREvent_WirelessReconnect => Ok(EventType::WirelessReconnect),
             sys::EVREventType_VREvent_ButtonPress => Ok(EventType::ButtonPress),
             sys::EVREventType_VREvent_ButtonUnpress => Ok(EventType::ButtonUnpress),
             sys::EVREventType_VREvent_ButtonTouch => Ok(EventType::ButtonTouch),
             sys::EVREventType_VREvent_ButtonUntouch => Ok(EventType::ButtonUntouch),
             sys::EVREventType_VREvent_DualAnalog_Press => Ok(EventType::DualAnalogPress),
             sys::EVREventType_VREvent_DualAnalog_Unpress => Ok(EventType::DualAnalogUnpress),
             sys::EVREventType_VREvent_DualAnalog_Touch => Ok(EventType::DualAnalogTouch),
             sys::EVREventType_VREvent_DualAnalog_Untouch => Ok(EventType::DualAnalogUntouch),
             sys::EVREventType_VREvent_DualAnalog_Move => Ok(EventType::DualAnalogMove),
             sys::EVREventType_VREvent_DualAnalog_ModeSwitch1 => Ok(EventType::DualAnalogModeSwitch1),
             sys::EVREventType_VREvent_DualAnalog_ModeSwitch2 => Ok(EventType::DualAnalogModeSwitch2),
             sys::EVREventType_VREvent_DualAnalog_Cancel => Ok(EventType::DualAnalogCancel),
             sys::EVREventType_VREvent_MouseMove => Ok(EventType::MouseMove),
             sys::EVREventType_VREvent_MouseButtonDown => Ok(EventType::MouseButtonDown),
             sys::EVREventType_VREvent_MouseButtonUp => Ok(EventType::MouseButtonUp),
             sys::EVREventType_VREvent_FocusEnter => Ok(EventType::FocusEnter),
             sys::EVREventType_VREvent_FocusLeave => Ok(EventType::FocusLeave),
             sys::EVREventType_VREvent_Scroll => Ok(EventType::Scroll),
             sys::EVREventType_VREvent_TouchPadMove => Ok(EventType::TouchPadMove),
             sys::EVREventType_VREvent_OverlayFocusChanged => Ok(EventType::OverlayFocusChanged),
             sys::EVREventType_VREvent_ReloadOverlays => Ok(EventType::ReloadOverlay),
             sys::EVREventType_VREvent_InputFocusCaptured => Ok(EventType::InputFocusCaptured),
             sys::EVREventType_VREvent_InputFocusReleased => Ok(EventType::InputFocusReleased),
             sys::EVREventType_VREvent_SceneFocusLost => Ok(EventType::SceneFocusLost),
             sys::EVREventType_VREvent_SceneFocusGained => Ok(EventType::SceneFocusGained),
             sys::EVREventType_VREvent_SceneApplicationChanged => Ok(EventType::SceneApplicationChanged),
             sys::EVREventType_VREvent_SceneFocusChanged => Ok(EventType::SceneFocusChanged),
             sys::EVREventType_VREvent_InputFocusChanged => Ok(EventType::InputFocusChanged),
             sys::EVREventType_VREvent_SceneApplicationSecondaryRenderingStarted => Ok(EventType::SceneApplicationSecondaryRenderingStarted),
             sys::EVREventType_VREvent_SceneApplicationUsingWrongGraphicsAdapter => Ok(EventType::SceneApplicationUsingWrongGraphicsAdapter),
             sys::EVREventType_VREvent_ActionBindingReloaded => Ok(EventType::ActionBindingReloaded),
             sys::EVREventType_VREvent_HideRenderModels => Ok(EventType::HideRenderModel),
             sys::EVREventType_VREvent_ShowRenderModels => Ok(EventType::ShowRenderModel),
             sys::EVREventType_VREvent_ConsoleOpened => Ok(EventType::ConsoleOpened),
             sys::EVREventType_VREvent_ConsoleClosed => Ok(EventType::ConsoleClosed),
             sys::EVREventType_VREvent_OverlayShown => Ok(EventType::OverlayShown),
             sys::EVREventType_VREvent_OverlayHidden => Ok(EventType::OverlayHidden),
             sys::EVREventType_VREvent_DashboardActivated => Ok(EventType::DashboardActivated),
             sys::EVREventType_VREvent_DashboardDeactivated => Ok(EventType::DashboardDeactivated),
             sys::EVREventType_VREvent_DashboardThumbSelected => Ok(EventType::DashboardThumbSelected),
             sys::EVREventType_VREvent_DashboardRequested => Ok(EventType::DashboardRequested),
             sys::EVREventType_VREvent_ResetDashboard => Ok(EventType::ResetDashboard),
             sys::EVREventType_VREvent_RenderToast => Ok(EventType::RenderToast),
             sys::EVREventType_VREvent_ImageLoaded => Ok(EventType::ImageLoaded),
             sys::EVREventType_VREvent_ShowKeyboard => Ok(EventType::ShowKeyboard),
             sys::EVREventType_VREvent_HideKeyboard => Ok(EventType::HideKeyboard),
             sys::EVREventType_VREvent_OverlayGamepadFocusGained => Ok(EventType::OverlayGamepadFocusGained),
             sys::EVREventType_VREvent_OverlayGamepadFocusLost => Ok(EventType::OverlayGamepadFocusLost),
             sys::EVREventType_VREvent_OverlaySharedTextureChanged => Ok(EventType::OverlaySharedTextureChanged),
             sys::EVREventType_VREvent_ScreenshotTriggered => Ok(EventType::ScreenshotTriggered),
             sys::EVREventType_VREvent_ImageFailed => Ok(EventType::ImageFailed),
             sys::EVREventType_VREvent_DashboardOverlayCreated => Ok(EventType::DashboardOverlayCreated),
             sys::EVREventType_VREvent_SwitchGamepadFocus => Ok(EventType::SwitchGamepadFocu),
             sys::EVREventType_VREvent_RequestScreenshot => Ok(EventType::RequestScreenshot),
             sys::EVREventType_VREvent_ScreenshotTaken => Ok(EventType::ScreenshotTaken),
             sys::EVREventType_VREvent_ScreenshotFailed => Ok(EventType::ScreenshotFailed),
             sys::EVREventType_VREvent_SubmitScreenshotToDashboard => Ok(EventType::SubmitScreenshotToDashboard),
             sys::EVREventType_VREvent_ScreenshotProgressToDashboard => Ok(EventType::ScreenshotProgressToDashboard),
             sys::EVREventType_VREvent_PrimaryDashboardDeviceChanged => Ok(EventType::PrimaryDashboardDeviceChanged),
             sys::EVREventType_VREvent_RoomViewShown => Ok(EventType::RoomViewShown),
             sys::EVREventType_VREvent_RoomViewHidden => Ok(EventType::RoomViewHidden),
             sys::EVREventType_VREvent_ShowUI => Ok(EventType::ShowUI),
             sys::EVREventType_VREvent_Notification_Shown => Ok(EventType::NotificationShown),
             sys::EVREventType_VREvent_Notification_Hidden => Ok(EventType::NotificationHidden),
             sys::EVREventType_VREvent_Notification_BeginInteraction => Ok(EventType::NotificationBeginInteraction),
             sys::EVREventType_VREvent_Notification_Destroyed => Ok(EventType::NotificationDestroyed),
             sys::EVREventType_VREvent_Quit => Ok(EventType::Quit),
             sys::EVREventType_VREvent_ProcessQuit => Ok(EventType::ProcessQuit),
             sys::EVREventType_VREvent_QuitAborted_UserPrompt => Ok(EventType::QuitAbortedUserPrompt),
             sys::EVREventType_VREvent_QuitAcknowledged => Ok(EventType::QuitAcknowledged),
             sys::EVREventType_VREvent_DriverRequestedQuit => Ok(EventType::DriverRequestedQuit),
             sys::EVREventType_VREvent_ChaperoneDataHasChanged => Ok(EventType::ChaperoneDataHasChanged),
             sys::EVREventType_VREvent_ChaperoneUniverseHasChanged => Ok(EventType::ChaperoneUniverseHasChanged),
             sys::EVREventType_VREvent_ChaperoneTempDataHasChanged => Ok(EventType::ChaperoneTempDataHasChanged),
             sys::EVREventType_VREvent_ChaperoneSettingsHaveChanged => Ok(EventType::ChaperoneSettingsHaveChanged),
             sys::EVREventType_VREvent_SeatedZeroPoseReset => Ok(EventType::SeatedZeroPoseReset),
             sys::EVREventType_VREvent_ChaperoneFlushCache => Ok(EventType::ChaperoneFlushCache),
             sys::EVREventType_VREvent_AudioSettingsHaveChanged => Ok(EventType::AudioSettingsHaveChanged),
             sys::EVREventType_VREvent_BackgroundSettingHasChanged => Ok(EventType::BackgroundSettingHasChanged),
             sys::EVREventType_VREvent_CameraSettingsHaveChanged => Ok(EventType::CameraSettingsHaveChanged),
             sys::EVREventType_VREvent_ReprojectionSettingHasChanged => Ok(EventType::ReprojectionSettingHasChanged),
             sys::EVREventType_VREvent_ModelSkinSettingsHaveChanged => Ok(EventType::ModelSkinSettingsHaveChanged),
             sys::EVREventType_VREvent_EnvironmentSettingsHaveChanged => Ok(EventType::EnvironmentSettingsHaveChanged),
             sys::EVREventType_VREvent_PowerSettingsHaveChanged => Ok(EventType::PowerSettingsHaveChanged),
             sys::EVREventType_VREvent_EnableHomeAppSettingsHaveChanged => Ok(EventType::EnableHomeAppSettingsHaveChanged),
             sys::EVREventType_VREvent_SteamVRSectionSettingChanged => Ok(EventType::SteamVRSectionSettingChanged),
             sys::EVREventType_VREvent_LighthouseSectionSettingChanged => Ok(EventType::LighthouseSectionSettingChanged),
             sys::EVREventType_VREvent_NullSectionSettingChanged => Ok(EventType::NullSectionSettingChanged),
             sys::EVREventType_VREvent_UserInterfaceSectionSettingChanged => Ok(EventType::UserInterfaceSectionSettingChanged),
             sys::EVREventType_VREvent_NotificationsSectionSettingChanged => Ok(EventType::NotificationsSectionSettingChanged),
             sys::EVREventType_VREvent_KeyboardSectionSettingChanged => Ok(EventType::KeyboardSectionSettingChanged),
             sys::EVREventType_VREvent_PerfSectionSettingChanged => Ok(EventType::PerfSectionSettingChanged),
             sys::EVREventType_VREvent_DashboardSectionSettingChanged => Ok(EventType::DashboardSectionSettingChanged),
             sys::EVREventType_VREvent_WebInterfaceSectionSettingChanged => Ok(EventType::WebInterfaceSectionSettingChanged),
             sys::EVREventType_VREvent_TrackersSectionSettingChanged => Ok(EventType::TrackersSectionSettingChanged),
             sys::EVREventType_VREvent_LastKnownSectionSettingChanged => Ok(EventType::LastKnownSectionSettingChanged),
             sys::EVREventType_VREvent_StatusUpdate => Ok(EventType::StatusUpdate),
             sys::EVREventType_VREvent_WebInterface_InstallDriverCompleted => Ok(EventType::WebInterfaceInstallDriverCompleted),
             sys::EVREventType_VREvent_MCImageUpdated => Ok(EventType::McimageUpdated),
             sys::EVREventType_VREvent_FirmwareUpdateStarted => Ok(EventType::FirmwareUpdateStarted),
             sys::EVREventType_VREvent_FirmwareUpdateFinished => Ok(EventType::FirmwareUpdateFinished),
             sys::EVREventType_VREvent_KeyboardClosed => Ok(EventType::KeyboardClosed),
             sys::EVREventType_VREvent_KeyboardCharInput => Ok(EventType::KeyboardCharInput),
             sys::EVREventType_VREvent_KeyboardDone => Ok(EventType::KeyboardDone),
             sys::EVREventType_VREvent_ApplicationTransitionStarted => Ok(EventType::ApplicationTransitionStarted),
             sys::EVREventType_VREvent_ApplicationTransitionAborted => Ok(EventType::ApplicationTransitionAborted),
             sys::EVREventType_VREvent_ApplicationTransitionNewAppStarted => Ok(EventType::ApplicationTransitionNewAppStarted),
             sys::EVREventType_VREvent_ApplicationListUpdated => Ok(EventType::ApplicationListUpdated),
             sys::EVREventType_VREvent_ApplicationMimeTypeLoad => Ok(EventType::ApplicationMimeTypeLoad),
             sys::EVREventType_VREvent_ApplicationTransitionNewAppLaunchComplete => Ok(EventType::ApplicationTransitionNewAppLaunchComplete),
             sys::EVREventType_VREvent_ProcessConnected => Ok(EventType::ProcessConnected),
             sys::EVREventType_VREvent_ProcessDisconnected => Ok(EventType::ProcessDisconnected),
             sys::EVREventType_VREvent_Compositor_MirrorWindowShown => Ok(EventType::CompositorMirrorWindowShown),
             sys::EVREventType_VREvent_Compositor_MirrorWindowHidden => Ok(EventType::CompositorMirrorWindowHidden),
             sys::EVREventType_VREvent_Compositor_ChaperoneBoundsShown => Ok(EventType::CompositorChaperoneBoundsShown),
             sys::EVREventType_VREvent_Compositor_ChaperoneBoundsHidden => Ok(EventType::CompositorChaperoneBoundsHidden),
             sys::EVREventType_VREvent_TrackedCamera_StartVideoStream => Ok(EventType::TrackedCameraStartVideoStream),
             sys::EVREventType_VREvent_TrackedCamera_StopVideoStream => Ok(EventType::TrackedCameraStopVideoStream),
             sys::EVREventType_VREvent_TrackedCamera_PauseVideoStream => Ok(EventType::TrackedCameraPauseVideoStream),
             sys::EVREventType_VREvent_TrackedCamera_ResumeVideoStream => Ok(EventType::TrackedCameraResumeVideoStream),
             sys::EVREventType_VREvent_TrackedCamera_EditingSurface => Ok(EventType::TrackedCameraEditingSurface),
             sys::EVREventType_VREvent_PerformanceTest_EnableCapture => Ok(EventType::PerformanceTestEnableCapture),
             sys::EVREventType_VREvent_PerformanceTest_DisableCapture => Ok(EventType::PerformanceTestDisableCapture),
             sys::EVREventType_VREvent_PerformanceTest_FidelityLevel => Ok(EventType::PerformanceTestFidelityLevel),
             sys::EVREventType_VREvent_MessageOverlay_Closed => Ok(EventType::MessageOverlayClosed),
             sys::EVREventType_VREvent_MessageOverlayCloseRequested => Ok(EventType::MessageOverlayCloseRequested),
             sys::EVREventType_VREvent_Input_HapticVibration => Ok(EventType::InputHapticVibration),
             sys::EVREventType_VREvent_Input_BindingLoadFailed => Ok(EventType::InputBindingLoadFailed),
             sys::EVREventType_VREvent_Input_BindingLoadSuccessful => Ok(EventType::InputBindingLoadSuccessful),
             sys::EVREventType_VREvent_Input_ActionManifestReloaded => Ok(EventType::InputActionManifestReloaded),
             sys::EVREventType_VREvent_Input_ActionManifestLoadFailed => Ok(EventType::InputActionManifestLoadFailed),
             sys::EVREventType_VREvent_Input_ProgressUpdate => Ok(EventType::InputProgressUpdate),
             sys::EVREventType_VREvent_Input_TrackerActivated => Ok(EventType::InputTrackerActivated),
             sys::EVREventType_VREvent_SpatialAnchors_PoseUpdated => Ok(EventType::SpatialAnchorsPoseUpdated),
             sys::EVREventType_VREvent_SpatialAnchors_DescriptorUpdated => Ok(EventType::SpatialAnchorsDescriptorUpdated),
             sys::EVREventType_VREvent_SpatialAnchors_RequestPoseUpdate => Ok(EventType::SpatialAnchorsRequestPoseUpdate),
             sys::EVREventType_VREvent_SpatialAnchors_RequestDescriptorUpdate => Ok(EventType::SpatialAnchorsRequestDescriptorUpdate),
             sys::EVREventType_VREvent_VendorSpecific_Reserved_Start => Ok(EventType::VendorSpecificReservedStart),
             sys::EVREventType_VREvent_VendorSpecific_Reserved_End => Ok(EventType::VendorSpecificReservedEnd),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<EventType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of EventType.", self.0)
    }
}

impl error::Error for Invalid<EventType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of EventType."
    }
}

/// EDeviceActivityLevel.
#[repr(i32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DeviceActivityLevel {
    /// EDeviceActivityLevel_k_EDeviceActivityLevel_Unknown = -1.
    EdeviceActivityLevelUnknown = sys::EDeviceActivityLevel_k_EDeviceActivityLevel_Unknown,
    /// EDeviceActivityLevel_k_EDeviceActivityLevel_Idle = 0.
    EdeviceActivityLevelIdle = sys::EDeviceActivityLevel_k_EDeviceActivityLevel_Idle,
    /// EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction = 1.
    EdeviceActivityLevelUserInteraction = sys::EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction,
    /// EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction_Timeout = 2.
    EdeviceActivityLevelUserInteractionTimeout = sys::EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction_Timeout,
    /// EDeviceActivityLevel_k_EDeviceActivityLevel_Standby = 3.
    EdeviceActivityLevelStandby = sys::EDeviceActivityLevel_k_EDeviceActivityLevel_Standby,
}

impl Enum for DeviceActivityLevel {
    type Raw = i32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EDeviceActivityLevel_k_EDeviceActivityLevel_Unknown => Ok(DeviceActivityLevel::EdeviceActivityLevelUnknown),
             sys::EDeviceActivityLevel_k_EDeviceActivityLevel_Idle => Ok(DeviceActivityLevel::EdeviceActivityLevelIdle),
             sys::EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction => Ok(DeviceActivityLevel::EdeviceActivityLevelUserInteraction),
             sys::EDeviceActivityLevel_k_EDeviceActivityLevel_UserInteraction_Timeout => Ok(DeviceActivityLevel::EdeviceActivityLevelUserInteractionTimeout),
             sys::EDeviceActivityLevel_k_EDeviceActivityLevel_Standby => Ok(DeviceActivityLevel::EdeviceActivityLevelStandby),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<DeviceActivityLevel> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of DeviceActivityLevel.", self.0)
    }
}

impl error::Error for Invalid<DeviceActivityLevel> {
    fn description(&self) -> &str {
        "Value does not represent any variant of DeviceActivityLevel."
    }
}

/// EVRButtonId.
/// Omitted variants:
///  - k_EButton_SteamVR_Touchpad
///  - k_EButton_SteamVR_Trigger
///  - k_EButton_Dashboard_Back
///  - k_EButton_Knuckles_A
///  - k_EButton_Knuckles_B
///  - k_EButton_Knuckles_JoyStick
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ButtonId {
    /// EVRButtonId_k_EButton_System = 0.
    EbuttonSystem = sys::EVRButtonId_k_EButton_System,
    /// EVRButtonId_k_EButton_ApplicationMenu = 1.
    EbuttonApplicationMenu = sys::EVRButtonId_k_EButton_ApplicationMenu,
    /// EVRButtonId_k_EButton_Grip = 2.
    EbuttonGrip = sys::EVRButtonId_k_EButton_Grip,
    /// EVRButtonId_k_EButton_DPad_Left = 3.
    EbuttonDPadLeft = sys::EVRButtonId_k_EButton_DPad_Left,
    /// EVRButtonId_k_EButton_DPad_Up = 4.
    EbuttonDPadUp = sys::EVRButtonId_k_EButton_DPad_Up,
    /// EVRButtonId_k_EButton_DPad_Right = 5.
    EbuttonDPadRight = sys::EVRButtonId_k_EButton_DPad_Right,
    /// EVRButtonId_k_EButton_DPad_Down = 6.
    EbuttonDPadDown = sys::EVRButtonId_k_EButton_DPad_Down,
    /// EVRButtonId_k_EButton_A = 7.
    EbuttonA = sys::EVRButtonId_k_EButton_A,
    /// EVRButtonId_k_EButton_ProximitySensor = 31.
    EbuttonProximitySensor = sys::EVRButtonId_k_EButton_ProximitySensor,
    /// EVRButtonId_k_EButton_Axis0 = 32.
    EbuttonAxis0 = sys::EVRButtonId_k_EButton_Axis0,
    /// EVRButtonId_k_EButton_Axis1 = 33.
    EbuttonAxis1 = sys::EVRButtonId_k_EButton_Axis1,
    /// EVRButtonId_k_EButton_Axis2 = 34.
    EbuttonAxis2 = sys::EVRButtonId_k_EButton_Axis2,
    /// EVRButtonId_k_EButton_Axis3 = 35.
    EbuttonAxis3 = sys::EVRButtonId_k_EButton_Axis3,
    /// EVRButtonId_k_EButton_Axis4 = 36.
    EbuttonAxis4 = sys::EVRButtonId_k_EButton_Axis4,
    /// EVRButtonId_k_EButton_Max = 64.
    EbuttonMax = sys::EVRButtonId_k_EButton_Max,
}

impl Enum for ButtonId {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRButtonId_k_EButton_System => Ok(ButtonId::EbuttonSystem),
             sys::EVRButtonId_k_EButton_ApplicationMenu => Ok(ButtonId::EbuttonApplicationMenu),
             sys::EVRButtonId_k_EButton_Grip => Ok(ButtonId::EbuttonGrip),
             sys::EVRButtonId_k_EButton_DPad_Left => Ok(ButtonId::EbuttonDPadLeft),
             sys::EVRButtonId_k_EButton_DPad_Up => Ok(ButtonId::EbuttonDPadUp),
             sys::EVRButtonId_k_EButton_DPad_Right => Ok(ButtonId::EbuttonDPadRight),
             sys::EVRButtonId_k_EButton_DPad_Down => Ok(ButtonId::EbuttonDPadDown),
             sys::EVRButtonId_k_EButton_A => Ok(ButtonId::EbuttonA),
             sys::EVRButtonId_k_EButton_ProximitySensor => Ok(ButtonId::EbuttonProximitySensor),
             sys::EVRButtonId_k_EButton_Axis0 => Ok(ButtonId::EbuttonAxis0),
             sys::EVRButtonId_k_EButton_Axis1 => Ok(ButtonId::EbuttonAxis1),
             sys::EVRButtonId_k_EButton_Axis2 => Ok(ButtonId::EbuttonAxis2),
             sys::EVRButtonId_k_EButton_Axis3 => Ok(ButtonId::EbuttonAxis3),
             sys::EVRButtonId_k_EButton_Axis4 => Ok(ButtonId::EbuttonAxis4),
             sys::EVRButtonId_k_EButton_Max => Ok(ButtonId::EbuttonMax),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ButtonId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ButtonId.", self.0)
    }
}

impl error::Error for Invalid<ButtonId> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ButtonId."
    }
}

/// EVRMouseButton.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MouseButton {
    /// EVRMouseButton_VRMouseButton_Left = 1.
    Left = sys::EVRMouseButton_VRMouseButton_Left,
    /// EVRMouseButton_VRMouseButton_Right = 2.
    Right = sys::EVRMouseButton_VRMouseButton_Right,
    /// EVRMouseButton_VRMouseButton_Middle = 4.
    Middle = sys::EVRMouseButton_VRMouseButton_Middle,
}

impl Enum for MouseButton {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRMouseButton_VRMouseButton_Left => Ok(MouseButton::Left),
             sys::EVRMouseButton_VRMouseButton_Right => Ok(MouseButton::Right),
             sys::EVRMouseButton_VRMouseButton_Middle => Ok(MouseButton::Middle),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<MouseButton> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of MouseButton.", self.0)
    }
}

impl error::Error for Invalid<MouseButton> {
    fn description(&self) -> &str {
        "Value does not represent any variant of MouseButton."
    }
}

/// EDualAnalogWhich.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DualAnalogWhich {
    /// EDualAnalogWhich_k_EDualAnalog_Left = 0.
    EdualAnalogLeft = sys::EDualAnalogWhich_k_EDualAnalog_Left,
    /// EDualAnalogWhich_k_EDualAnalog_Right = 1.
    EdualAnalogRight = sys::EDualAnalogWhich_k_EDualAnalog_Right,
}

impl Enum for DualAnalogWhich {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EDualAnalogWhich_k_EDualAnalog_Left => Ok(DualAnalogWhich::EdualAnalogLeft),
             sys::EDualAnalogWhich_k_EDualAnalog_Right => Ok(DualAnalogWhich::EdualAnalogRight),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<DualAnalogWhich> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of DualAnalogWhich.", self.0)
    }
}

impl error::Error for Invalid<DualAnalogWhich> {
    fn description(&self) -> &str {
        "Value does not represent any variant of DualAnalogWhich."
    }
}

/// EShowUIType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ShowUIType {
    /// EShowUIType_ShowUI_ControllerBinding = 0.
    ControllerBinding = sys::EShowUIType_ShowUI_ControllerBinding,
    /// EShowUIType_ShowUI_ManageTrackers = 1.
    ManageTracker = sys::EShowUIType_ShowUI_ManageTrackers,
    /// EShowUIType_ShowUI_QuickStart = 2.
    QuickStart = sys::EShowUIType_ShowUI_QuickStart,
}

impl Enum for ShowUIType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EShowUIType_ShowUI_ControllerBinding => Ok(ShowUIType::ControllerBinding),
             sys::EShowUIType_ShowUI_ManageTrackers => Ok(ShowUIType::ManageTracker),
             sys::EShowUIType_ShowUI_QuickStart => Ok(ShowUIType::QuickStart),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ShowUIType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ShowUIType.", self.0)
    }
}

impl error::Error for Invalid<ShowUIType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ShowUIType."
    }
}

/// EVRInputError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InputError {
    /// EVRInputError_VRInputError_None = 0.
    None = sys::EVRInputError_VRInputError_None,
    /// EVRInputError_VRInputError_NameNotFound = 1.
    NameNotFound = sys::EVRInputError_VRInputError_NameNotFound,
    /// EVRInputError_VRInputError_WrongType = 2.
    WrongType = sys::EVRInputError_VRInputError_WrongType,
    /// EVRInputError_VRInputError_InvalidHandle = 3.
    InvalidHandle = sys::EVRInputError_VRInputError_InvalidHandle,
    /// EVRInputError_VRInputError_InvalidParam = 4.
    InvalidParam = sys::EVRInputError_VRInputError_InvalidParam,
    /// EVRInputError_VRInputError_NoSteam = 5.
    NoSteam = sys::EVRInputError_VRInputError_NoSteam,
    /// EVRInputError_VRInputError_MaxCapacityReached = 6.
    MaxCapacityReached = sys::EVRInputError_VRInputError_MaxCapacityReached,
    /// EVRInputError_VRInputError_IPCError = 7.
    Ipcerror = sys::EVRInputError_VRInputError_IPCError,
    /// EVRInputError_VRInputError_NoActiveActionSet = 8.
    NoActiveActionSet = sys::EVRInputError_VRInputError_NoActiveActionSet,
    /// EVRInputError_VRInputError_InvalidDevice = 9.
    InvalidDevice = sys::EVRInputError_VRInputError_InvalidDevice,
    /// EVRInputError_VRInputError_InvalidSkeleton = 10.
    InvalidSkeleton = sys::EVRInputError_VRInputError_InvalidSkeleton,
    /// EVRInputError_VRInputError_InvalidBoneCount = 11.
    InvalidBoneCount = sys::EVRInputError_VRInputError_InvalidBoneCount,
    /// EVRInputError_VRInputError_InvalidCompressedData = 12.
    InvalidCompressedDaum = sys::EVRInputError_VRInputError_InvalidCompressedData,
    /// EVRInputError_VRInputError_NoData = 13.
    NoDaum = sys::EVRInputError_VRInputError_NoData,
    /// EVRInputError_VRInputError_BufferTooSmall = 14.
    BufferTooSmall = sys::EVRInputError_VRInputError_BufferTooSmall,
    /// EVRInputError_VRInputError_MismatchedActionManifest = 15.
    MismatchedActionManifest = sys::EVRInputError_VRInputError_MismatchedActionManifest,
    /// EVRInputError_VRInputError_MissingSkeletonData = 16.
    MissingSkeletonDaum = sys::EVRInputError_VRInputError_MissingSkeletonData,
    /// EVRInputError_VRInputError_InvalidBoneIndex = 17.
    InvalidBoneIndex = sys::EVRInputError_VRInputError_InvalidBoneIndex,
}

impl Enum for InputError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRInputError_VRInputError_None => Ok(InputError::None),
             sys::EVRInputError_VRInputError_NameNotFound => Ok(InputError::NameNotFound),
             sys::EVRInputError_VRInputError_WrongType => Ok(InputError::WrongType),
             sys::EVRInputError_VRInputError_InvalidHandle => Ok(InputError::InvalidHandle),
             sys::EVRInputError_VRInputError_InvalidParam => Ok(InputError::InvalidParam),
             sys::EVRInputError_VRInputError_NoSteam => Ok(InputError::NoSteam),
             sys::EVRInputError_VRInputError_MaxCapacityReached => Ok(InputError::MaxCapacityReached),
             sys::EVRInputError_VRInputError_IPCError => Ok(InputError::Ipcerror),
             sys::EVRInputError_VRInputError_NoActiveActionSet => Ok(InputError::NoActiveActionSet),
             sys::EVRInputError_VRInputError_InvalidDevice => Ok(InputError::InvalidDevice),
             sys::EVRInputError_VRInputError_InvalidSkeleton => Ok(InputError::InvalidSkeleton),
             sys::EVRInputError_VRInputError_InvalidBoneCount => Ok(InputError::InvalidBoneCount),
             sys::EVRInputError_VRInputError_InvalidCompressedData => Ok(InputError::InvalidCompressedDaum),
             sys::EVRInputError_VRInputError_NoData => Ok(InputError::NoDaum),
             sys::EVRInputError_VRInputError_BufferTooSmall => Ok(InputError::BufferTooSmall),
             sys::EVRInputError_VRInputError_MismatchedActionManifest => Ok(InputError::MismatchedActionManifest),
             sys::EVRInputError_VRInputError_MissingSkeletonData => Ok(InputError::MissingSkeletonDaum),
             sys::EVRInputError_VRInputError_InvalidBoneIndex => Ok(InputError::InvalidBoneIndex),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<InputError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of InputError.", self.0)
    }
}

impl error::Error for Invalid<InputError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of InputError."
    }
}

/// EVRSpatialAnchorError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SpatialAnchorError {
    /// EVRSpatialAnchorError_VRSpatialAnchorError_Success = 0.
    Success = sys::EVRSpatialAnchorError_VRSpatialAnchorError_Success,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_Internal = 1.
    Internal = sys::EVRSpatialAnchorError_VRSpatialAnchorError_Internal,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_UnknownHandle = 2.
    UnknownHandle = sys::EVRSpatialAnchorError_VRSpatialAnchorError_UnknownHandle,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_ArrayTooSmall = 3.
    ArrayTooSmall = sys::EVRSpatialAnchorError_VRSpatialAnchorError_ArrayTooSmall,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_InvalidDescriptorChar = 4.
    InvalidDescriptorChar = sys::EVRSpatialAnchorError_VRSpatialAnchorError_InvalidDescriptorChar,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_NotYetAvailable = 5.
    NotYetAvailable = sys::EVRSpatialAnchorError_VRSpatialAnchorError_NotYetAvailable,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_NotAvailableInThisUniverse = 6.
    NotAvailableInThisUniverse = sys::EVRSpatialAnchorError_VRSpatialAnchorError_NotAvailableInThisUniverse,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_PermanentlyUnavailable = 7.
    PermanentlyUnavailable = sys::EVRSpatialAnchorError_VRSpatialAnchorError_PermanentlyUnavailable,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_WrongDriver = 8.
    WrongDriver = sys::EVRSpatialAnchorError_VRSpatialAnchorError_WrongDriver,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_DescriptorTooLong = 9.
    DescriptorTooLong = sys::EVRSpatialAnchorError_VRSpatialAnchorError_DescriptorTooLong,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_Unknown = 10.
    Unknown = sys::EVRSpatialAnchorError_VRSpatialAnchorError_Unknown,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_NoRoomCalibration = 11.
    NoRoomCalibration = sys::EVRSpatialAnchorError_VRSpatialAnchorError_NoRoomCalibration,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_InvalidArgument = 12.
    InvalidArgument = sys::EVRSpatialAnchorError_VRSpatialAnchorError_InvalidArgument,
    /// EVRSpatialAnchorError_VRSpatialAnchorError_UnknownDriver = 13.
    UnknownDriver = sys::EVRSpatialAnchorError_VRSpatialAnchorError_UnknownDriver,
}

impl Enum for SpatialAnchorError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_Success => Ok(SpatialAnchorError::Success),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_Internal => Ok(SpatialAnchorError::Internal),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_UnknownHandle => Ok(SpatialAnchorError::UnknownHandle),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_ArrayTooSmall => Ok(SpatialAnchorError::ArrayTooSmall),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_InvalidDescriptorChar => Ok(SpatialAnchorError::InvalidDescriptorChar),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_NotYetAvailable => Ok(SpatialAnchorError::NotYetAvailable),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_NotAvailableInThisUniverse => Ok(SpatialAnchorError::NotAvailableInThisUniverse),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_PermanentlyUnavailable => Ok(SpatialAnchorError::PermanentlyUnavailable),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_WrongDriver => Ok(SpatialAnchorError::WrongDriver),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_DescriptorTooLong => Ok(SpatialAnchorError::DescriptorTooLong),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_Unknown => Ok(SpatialAnchorError::Unknown),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_NoRoomCalibration => Ok(SpatialAnchorError::NoRoomCalibration),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_InvalidArgument => Ok(SpatialAnchorError::InvalidArgument),
             sys::EVRSpatialAnchorError_VRSpatialAnchorError_UnknownDriver => Ok(SpatialAnchorError::UnknownDriver),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<SpatialAnchorError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of SpatialAnchorError.", self.0)
    }
}

impl error::Error for Invalid<SpatialAnchorError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of SpatialAnchorError."
    }
}

/// EHiddenAreaMeshType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum HiddenAreaMeshType {
    /// EHiddenAreaMeshType_k_eHiddenAreaMesh_Standard = 0.
    EhiddenAreaMeshStandard = sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Standard,
    /// EHiddenAreaMeshType_k_eHiddenAreaMesh_Inverse = 1.
    EhiddenAreaMeshInverse = sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Inverse,
    /// EHiddenAreaMeshType_k_eHiddenAreaMesh_LineLoop = 2.
    EhiddenAreaMeshLineLoop = sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_LineLoop,
    /// EHiddenAreaMeshType_k_eHiddenAreaMesh_Max = 3.
    EhiddenAreaMeshMax = sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Max,
}

impl Enum for HiddenAreaMeshType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Standard => Ok(HiddenAreaMeshType::EhiddenAreaMeshStandard),
             sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Inverse => Ok(HiddenAreaMeshType::EhiddenAreaMeshInverse),
             sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_LineLoop => Ok(HiddenAreaMeshType::EhiddenAreaMeshLineLoop),
             sys::EHiddenAreaMeshType_k_eHiddenAreaMesh_Max => Ok(HiddenAreaMeshType::EhiddenAreaMeshMax),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<HiddenAreaMeshType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of HiddenAreaMeshType.", self.0)
    }
}

impl error::Error for Invalid<HiddenAreaMeshType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of HiddenAreaMeshType."
    }
}

/// EVRControllerAxisType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ControllerAxisType {
    /// EVRControllerAxisType_k_eControllerAxis_None = 0.
    EcontrollerAxisNone = sys::EVRControllerAxisType_k_eControllerAxis_None,
    /// EVRControllerAxisType_k_eControllerAxis_TrackPad = 1.
    EcontrollerAxisTrackPad = sys::EVRControllerAxisType_k_eControllerAxis_TrackPad,
    /// EVRControllerAxisType_k_eControllerAxis_Joystick = 2.
    EcontrollerAxisJoystick = sys::EVRControllerAxisType_k_eControllerAxis_Joystick,
    /// EVRControllerAxisType_k_eControllerAxis_Trigger = 3.
    EcontrollerAxisTrigger = sys::EVRControllerAxisType_k_eControllerAxis_Trigger,
}

impl Enum for ControllerAxisType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRControllerAxisType_k_eControllerAxis_None => Ok(ControllerAxisType::EcontrollerAxisNone),
             sys::EVRControllerAxisType_k_eControllerAxis_TrackPad => Ok(ControllerAxisType::EcontrollerAxisTrackPad),
             sys::EVRControllerAxisType_k_eControllerAxis_Joystick => Ok(ControllerAxisType::EcontrollerAxisJoystick),
             sys::EVRControllerAxisType_k_eControllerAxis_Trigger => Ok(ControllerAxisType::EcontrollerAxisTrigger),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ControllerAxisType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ControllerAxisType.", self.0)
    }
}

impl error::Error for Invalid<ControllerAxisType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ControllerAxisType."
    }
}

/// EVRControllerEventOutputType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ControllerEventOutputType {
    /// EVRControllerEventOutputType_ControllerEventOutput_OSEvents = 0.
    Osevent = sys::EVRControllerEventOutputType_ControllerEventOutput_OSEvents,
    /// EVRControllerEventOutputType_ControllerEventOutput_VREvents = 1.
    Vrevent = sys::EVRControllerEventOutputType_ControllerEventOutput_VREvents,
}

impl Enum for ControllerEventOutputType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRControllerEventOutputType_ControllerEventOutput_OSEvents => Ok(ControllerEventOutputType::Osevent),
             sys::EVRControllerEventOutputType_ControllerEventOutput_VREvents => Ok(ControllerEventOutputType::Vrevent),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ControllerEventOutputType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ControllerEventOutputType.", self.0)
    }
}

impl error::Error for Invalid<ControllerEventOutputType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ControllerEventOutputType."
    }
}

/// ECollisionBoundsStyle.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CollisionBoundsStyle {
    /// ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_BEGINNER = 0.
    BoundsStyleBeginner = sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_BEGINNER,
    /// ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_INTERMEDIATE = 1.
    BoundsStyleIntermediate = sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_INTERMEDIATE,
    /// ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_SQUARES = 2.
    BoundsStyleSquare = sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_SQUARES,
    /// ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_ADVANCED = 3.
    BoundsStyleAdvanced = sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_ADVANCED,
    /// ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_NONE = 4.
    BoundsStyleNone = sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_NONE,
    /// ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_COUNT = 5.
    BoundsStyleCount = sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_COUNT,
}

impl Enum for CollisionBoundsStyle {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_BEGINNER => Ok(CollisionBoundsStyle::BoundsStyleBeginner),
             sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_INTERMEDIATE => Ok(CollisionBoundsStyle::BoundsStyleIntermediate),
             sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_SQUARES => Ok(CollisionBoundsStyle::BoundsStyleSquare),
             sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_ADVANCED => Ok(CollisionBoundsStyle::BoundsStyleAdvanced),
             sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_NONE => Ok(CollisionBoundsStyle::BoundsStyleNone),
             sys::ECollisionBoundsStyle_COLLISION_BOUNDS_STYLE_COUNT => Ok(CollisionBoundsStyle::BoundsStyleCount),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<CollisionBoundsStyle> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of CollisionBoundsStyle.", self.0)
    }
}

impl error::Error for Invalid<CollisionBoundsStyle> {
    fn description(&self) -> &str {
        "Value does not represent any variant of CollisionBoundsStyle."
    }
}

/// EVROverlayError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayError {
    /// EVROverlayError_VROverlayError_None = 0.
    None = sys::EVROverlayError_VROverlayError_None,
    /// EVROverlayError_VROverlayError_UnknownOverlay = 10.
    UnknownOverlay = sys::EVROverlayError_VROverlayError_UnknownOverlay,
    /// EVROverlayError_VROverlayError_InvalidHandle = 11.
    InvalidHandle = sys::EVROverlayError_VROverlayError_InvalidHandle,
    /// EVROverlayError_VROverlayError_PermissionDenied = 12.
    PermissionDenied = sys::EVROverlayError_VROverlayError_PermissionDenied,
    /// EVROverlayError_VROverlayError_OverlayLimitExceeded = 13.
    OverlayLimitExceeded = sys::EVROverlayError_VROverlayError_OverlayLimitExceeded,
    /// EVROverlayError_VROverlayError_WrongVisibilityType = 14.
    WrongVisibilityType = sys::EVROverlayError_VROverlayError_WrongVisibilityType,
    /// EVROverlayError_VROverlayError_KeyTooLong = 15.
    KeyTooLong = sys::EVROverlayError_VROverlayError_KeyTooLong,
    /// EVROverlayError_VROverlayError_NameTooLong = 16.
    NameTooLong = sys::EVROverlayError_VROverlayError_NameTooLong,
    /// EVROverlayError_VROverlayError_KeyInUse = 17.
    KeyInUse = sys::EVROverlayError_VROverlayError_KeyInUse,
    /// EVROverlayError_VROverlayError_WrongTransformType = 18.
    WrongTransformType = sys::EVROverlayError_VROverlayError_WrongTransformType,
    /// EVROverlayError_VROverlayError_InvalidTrackedDevice = 19.
    InvalidTrackedDevice = sys::EVROverlayError_VROverlayError_InvalidTrackedDevice,
    /// EVROverlayError_VROverlayError_InvalidParameter = 20.
    InvalidParameter = sys::EVROverlayError_VROverlayError_InvalidParameter,
    /// EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed = 21.
    ThumbnailCantBeDestroyed = sys::EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed,
    /// EVROverlayError_VROverlayError_ArrayTooSmall = 22.
    ArrayTooSmall = sys::EVROverlayError_VROverlayError_ArrayTooSmall,
    /// EVROverlayError_VROverlayError_RequestFailed = 23.
    RequestFailed = sys::EVROverlayError_VROverlayError_RequestFailed,
    /// EVROverlayError_VROverlayError_InvalidTexture = 24.
    InvalidTexture = sys::EVROverlayError_VROverlayError_InvalidTexture,
    /// EVROverlayError_VROverlayError_UnableToLoadFile = 25.
    UnableToLoadFile = sys::EVROverlayError_VROverlayError_UnableToLoadFile,
    /// EVROverlayError_VROverlayError_KeyboardAlreadyInUse = 26.
    KeyboardAlreadyInUse = sys::EVROverlayError_VROverlayError_KeyboardAlreadyInUse,
    /// EVROverlayError_VROverlayError_NoNeighbor = 27.
    NoNeighbor = sys::EVROverlayError_VROverlayError_NoNeighbor,
    /// EVROverlayError_VROverlayError_TooManyMaskPrimitives = 29.
    TooManyMaskPrimi = sys::EVROverlayError_VROverlayError_TooManyMaskPrimitives,
    /// EVROverlayError_VROverlayError_BadMaskPrimitive = 30.
    BadMaskPrimitive = sys::EVROverlayError_VROverlayError_BadMaskPrimitive,
    /// EVROverlayError_VROverlayError_TextureAlreadyLocked = 31.
    TextureAlreadyLocked = sys::EVROverlayError_VROverlayError_TextureAlreadyLocked,
    /// EVROverlayError_VROverlayError_TextureLockCapacityReached = 32.
    TextureLockCapacityReached = sys::EVROverlayError_VROverlayError_TextureLockCapacityReached,
    /// EVROverlayError_VROverlayError_TextureNotLocked = 33.
    TextureNotLocked = sys::EVROverlayError_VROverlayError_TextureNotLocked,
}

impl Enum for OverlayError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVROverlayError_VROverlayError_None => Ok(OverlayError::None),
             sys::EVROverlayError_VROverlayError_UnknownOverlay => Ok(OverlayError::UnknownOverlay),
             sys::EVROverlayError_VROverlayError_InvalidHandle => Ok(OverlayError::InvalidHandle),
             sys::EVROverlayError_VROverlayError_PermissionDenied => Ok(OverlayError::PermissionDenied),
             sys::EVROverlayError_VROverlayError_OverlayLimitExceeded => Ok(OverlayError::OverlayLimitExceeded),
             sys::EVROverlayError_VROverlayError_WrongVisibilityType => Ok(OverlayError::WrongVisibilityType),
             sys::EVROverlayError_VROverlayError_KeyTooLong => Ok(OverlayError::KeyTooLong),
             sys::EVROverlayError_VROverlayError_NameTooLong => Ok(OverlayError::NameTooLong),
             sys::EVROverlayError_VROverlayError_KeyInUse => Ok(OverlayError::KeyInUse),
             sys::EVROverlayError_VROverlayError_WrongTransformType => Ok(OverlayError::WrongTransformType),
             sys::EVROverlayError_VROverlayError_InvalidTrackedDevice => Ok(OverlayError::InvalidTrackedDevice),
             sys::EVROverlayError_VROverlayError_InvalidParameter => Ok(OverlayError::InvalidParameter),
             sys::EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed => Ok(OverlayError::ThumbnailCantBeDestroyed),
             sys::EVROverlayError_VROverlayError_ArrayTooSmall => Ok(OverlayError::ArrayTooSmall),
             sys::EVROverlayError_VROverlayError_RequestFailed => Ok(OverlayError::RequestFailed),
             sys::EVROverlayError_VROverlayError_InvalidTexture => Ok(OverlayError::InvalidTexture),
             sys::EVROverlayError_VROverlayError_UnableToLoadFile => Ok(OverlayError::UnableToLoadFile),
             sys::EVROverlayError_VROverlayError_KeyboardAlreadyInUse => Ok(OverlayError::KeyboardAlreadyInUse),
             sys::EVROverlayError_VROverlayError_NoNeighbor => Ok(OverlayError::NoNeighbor),
             sys::EVROverlayError_VROverlayError_TooManyMaskPrimitives => Ok(OverlayError::TooManyMaskPrimi),
             sys::EVROverlayError_VROverlayError_BadMaskPrimitive => Ok(OverlayError::BadMaskPrimitive),
             sys::EVROverlayError_VROverlayError_TextureAlreadyLocked => Ok(OverlayError::TextureAlreadyLocked),
             sys::EVROverlayError_VROverlayError_TextureLockCapacityReached => Ok(OverlayError::TextureLockCapacityReached),
             sys::EVROverlayError_VROverlayError_TextureNotLocked => Ok(OverlayError::TextureNotLocked),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<OverlayError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of OverlayError.", self.0)
    }
}

impl error::Error for Invalid<OverlayError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of OverlayError."
    }
}

/// EVRApplicationType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationType {
    /// EVRApplicationType_VRApplication_Other = 0.
    Other = sys::EVRApplicationType_VRApplication_Other,
    /// EVRApplicationType_VRApplication_Scene = 1.
    Scene = sys::EVRApplicationType_VRApplication_Scene,
    /// EVRApplicationType_VRApplication_Overlay = 2.
    Overlay = sys::EVRApplicationType_VRApplication_Overlay,
    /// EVRApplicationType_VRApplication_Background = 3.
    Background = sys::EVRApplicationType_VRApplication_Background,
    /// EVRApplicationType_VRApplication_Utility = 4.
    Utility = sys::EVRApplicationType_VRApplication_Utility,
    /// EVRApplicationType_VRApplication_VRMonitor = 5.
    Vrmonitor = sys::EVRApplicationType_VRApplication_VRMonitor,
    /// EVRApplicationType_VRApplication_SteamWatchdog = 6.
    SteamWatchdog = sys::EVRApplicationType_VRApplication_SteamWatchdog,
    /// EVRApplicationType_VRApplication_Bootstrapper = 7.
    Bootstrapper = sys::EVRApplicationType_VRApplication_Bootstrapper,
    /// EVRApplicationType_VRApplication_WebHelper = 8.
    WebHelper = sys::EVRApplicationType_VRApplication_WebHelper,
    /// EVRApplicationType_VRApplication_Max = 9.
    Max = sys::EVRApplicationType_VRApplication_Max,
}

impl Enum for ApplicationType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRApplicationType_VRApplication_Other => Ok(ApplicationType::Other),
             sys::EVRApplicationType_VRApplication_Scene => Ok(ApplicationType::Scene),
             sys::EVRApplicationType_VRApplication_Overlay => Ok(ApplicationType::Overlay),
             sys::EVRApplicationType_VRApplication_Background => Ok(ApplicationType::Background),
             sys::EVRApplicationType_VRApplication_Utility => Ok(ApplicationType::Utility),
             sys::EVRApplicationType_VRApplication_VRMonitor => Ok(ApplicationType::Vrmonitor),
             sys::EVRApplicationType_VRApplication_SteamWatchdog => Ok(ApplicationType::SteamWatchdog),
             sys::EVRApplicationType_VRApplication_Bootstrapper => Ok(ApplicationType::Bootstrapper),
             sys::EVRApplicationType_VRApplication_WebHelper => Ok(ApplicationType::WebHelper),
             sys::EVRApplicationType_VRApplication_Max => Ok(ApplicationType::Max),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ApplicationType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ApplicationType.", self.0)
    }
}

impl error::Error for Invalid<ApplicationType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ApplicationType."
    }
}

/// EVRFirmwareError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FirmwareError {
    /// EVRFirmwareError_VRFirmwareError_None = 0.
    None = sys::EVRFirmwareError_VRFirmwareError_None,
    /// EVRFirmwareError_VRFirmwareError_Success = 1.
    Success = sys::EVRFirmwareError_VRFirmwareError_Success,
    /// EVRFirmwareError_VRFirmwareError_Fail = 2.
    Fail = sys::EVRFirmwareError_VRFirmwareError_Fail,
}

impl Enum for FirmwareError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRFirmwareError_VRFirmwareError_None => Ok(FirmwareError::None),
             sys::EVRFirmwareError_VRFirmwareError_Success => Ok(FirmwareError::Success),
             sys::EVRFirmwareError_VRFirmwareError_Fail => Ok(FirmwareError::Fail),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<FirmwareError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of FirmwareError.", self.0)
    }
}

impl error::Error for Invalid<FirmwareError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of FirmwareError."
    }
}

/// EVRNotificationError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NotificationError {
    /// EVRNotificationError_VRNotificationError_OK = 0.
    Ok = sys::EVRNotificationError_VRNotificationError_OK,
    /// EVRNotificationError_VRNotificationError_InvalidNotificationId = 100.
    InvalidNotificationId = sys::EVRNotificationError_VRNotificationError_InvalidNotificationId,
    /// EVRNotificationError_VRNotificationError_NotificationQueueFull = 101.
    NotificationQueueFull = sys::EVRNotificationError_VRNotificationError_NotificationQueueFull,
    /// EVRNotificationError_VRNotificationError_InvalidOverlayHandle = 102.
    InvalidOverlayHandle = sys::EVRNotificationError_VRNotificationError_InvalidOverlayHandle,
    /// EVRNotificationError_VRNotificationError_SystemWithUserValueAlreadyExists = 103.
    SystemWithUserValueAlreadyExist = sys::EVRNotificationError_VRNotificationError_SystemWithUserValueAlreadyExists,
}

impl Enum for NotificationError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRNotificationError_VRNotificationError_OK => Ok(NotificationError::Ok),
             sys::EVRNotificationError_VRNotificationError_InvalidNotificationId => Ok(NotificationError::InvalidNotificationId),
             sys::EVRNotificationError_VRNotificationError_NotificationQueueFull => Ok(NotificationError::NotificationQueueFull),
             sys::EVRNotificationError_VRNotificationError_InvalidOverlayHandle => Ok(NotificationError::InvalidOverlayHandle),
             sys::EVRNotificationError_VRNotificationError_SystemWithUserValueAlreadyExists => Ok(NotificationError::SystemWithUserValueAlreadyExist),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<NotificationError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of NotificationError.", self.0)
    }
}

impl error::Error for Invalid<NotificationError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of NotificationError."
    }
}

/// EVRSkeletalMotionRange.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalMotionRange {
    /// EVRSkeletalMotionRange_VRSkeletalMotionRange_WithController = 0.
    WithController = sys::EVRSkeletalMotionRange_VRSkeletalMotionRange_WithController,
    /// EVRSkeletalMotionRange_VRSkeletalMotionRange_WithoutController = 1.
    WithoutController = sys::EVRSkeletalMotionRange_VRSkeletalMotionRange_WithoutController,
}

impl Enum for SkeletalMotionRange {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRSkeletalMotionRange_VRSkeletalMotionRange_WithController => Ok(SkeletalMotionRange::WithController),
             sys::EVRSkeletalMotionRange_VRSkeletalMotionRange_WithoutController => Ok(SkeletalMotionRange::WithoutController),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<SkeletalMotionRange> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of SkeletalMotionRange.", self.0)
    }
}

impl error::Error for Invalid<SkeletalMotionRange> {
    fn description(&self) -> &str {
        "Value does not represent any variant of SkeletalMotionRange."
    }
}

/// EVRSkeletalTrackingLevel.
/// Omitted variants:
///  - VRSkeletalTrackingLevel_Max
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalTrackingLevel {
    /// EVRSkeletalTrackingLevel_VRSkeletalTracking_Estimated = 0.
    Estimated = sys::EVRSkeletalTrackingLevel_VRSkeletalTracking_Estimated,
    /// EVRSkeletalTrackingLevel_VRSkeletalTracking_Partial = 1.
    Partial = sys::EVRSkeletalTrackingLevel_VRSkeletalTracking_Partial,
    /// EVRSkeletalTrackingLevel_VRSkeletalTracking_Full = 2.
    Full = sys::EVRSkeletalTrackingLevel_VRSkeletalTracking_Full,
    /// EVRSkeletalTrackingLevel_VRSkeletalTrackingLevel_Count = 3.
    Count = sys::EVRSkeletalTrackingLevel_VRSkeletalTrackingLevel_Count,
}

impl Enum for SkeletalTrackingLevel {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRSkeletalTrackingLevel_VRSkeletalTracking_Estimated => Ok(SkeletalTrackingLevel::Estimated),
             sys::EVRSkeletalTrackingLevel_VRSkeletalTracking_Partial => Ok(SkeletalTrackingLevel::Partial),
             sys::EVRSkeletalTrackingLevel_VRSkeletalTracking_Full => Ok(SkeletalTrackingLevel::Full),
             sys::EVRSkeletalTrackingLevel_VRSkeletalTrackingLevel_Count => Ok(SkeletalTrackingLevel::Count),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<SkeletalTrackingLevel> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of SkeletalTrackingLevel.", self.0)
    }
}

impl error::Error for Invalid<SkeletalTrackingLevel> {
    fn description(&self) -> &str {
        "Value does not represent any variant of SkeletalTrackingLevel."
    }
}

/// EVRInitError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InitError {
    /// EVRInitError_VRInitError_None = 0.
    None = sys::EVRInitError_VRInitError_None,
    /// EVRInitError_VRInitError_Unknown = 1.
    Unknown = sys::EVRInitError_VRInitError_Unknown,
    /// EVRInitError_VRInitError_Init_InstallationNotFound = 100.
    InitInstallationNotFound = sys::EVRInitError_VRInitError_Init_InstallationNotFound,
    /// EVRInitError_VRInitError_Init_InstallationCorrupt = 101.
    InitInstallationCorrupt = sys::EVRInitError_VRInitError_Init_InstallationCorrupt,
    /// EVRInitError_VRInitError_Init_VRClientDLLNotFound = 102.
    InitVRClientDLLNotFound = sys::EVRInitError_VRInitError_Init_VRClientDLLNotFound,
    /// EVRInitError_VRInitError_Init_FileNotFound = 103.
    InitFileNotFound = sys::EVRInitError_VRInitError_Init_FileNotFound,
    /// EVRInitError_VRInitError_Init_FactoryNotFound = 104.
    InitFactoryNotFound = sys::EVRInitError_VRInitError_Init_FactoryNotFound,
    /// EVRInitError_VRInitError_Init_InterfaceNotFound = 105.
    InitInterfaceNotFound = sys::EVRInitError_VRInitError_Init_InterfaceNotFound,
    /// EVRInitError_VRInitError_Init_InvalidInterface = 106.
    InitInvalidInterface = sys::EVRInitError_VRInitError_Init_InvalidInterface,
    /// EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid = 107.
    InitUserConfigDirectoryInvalid = sys::EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid,
    /// EVRInitError_VRInitError_Init_HmdNotFound = 108.
    InitHmdNotFound = sys::EVRInitError_VRInitError_Init_HmdNotFound,
    /// EVRInitError_VRInitError_Init_NotInitialized = 109.
    InitNotInitialized = sys::EVRInitError_VRInitError_Init_NotInitialized,
    /// EVRInitError_VRInitError_Init_PathRegistryNotFound = 110.
    InitPathRegistryNotFound = sys::EVRInitError_VRInitError_Init_PathRegistryNotFound,
    /// EVRInitError_VRInitError_Init_NoConfigPath = 111.
    InitNoConfigPath = sys::EVRInitError_VRInitError_Init_NoConfigPath,
    /// EVRInitError_VRInitError_Init_NoLogPath = 112.
    InitNoLogPath = sys::EVRInitError_VRInitError_Init_NoLogPath,
    /// EVRInitError_VRInitError_Init_PathRegistryNotWritable = 113.
    InitPathRegistryNotWritable = sys::EVRInitError_VRInitError_Init_PathRegistryNotWritable,
    /// EVRInitError_VRInitError_Init_AppInfoInitFailed = 114.
    InitAppInfoInitFailed = sys::EVRInitError_VRInitError_Init_AppInfoInitFailed,
    /// EVRInitError_VRInitError_Init_Retry = 115.
    InitRetry = sys::EVRInitError_VRInitError_Init_Retry,
    /// EVRInitError_VRInitError_Init_InitCanceledByUser = 116.
    InitInitCanceledByUser = sys::EVRInitError_VRInitError_Init_InitCanceledByUser,
    /// EVRInitError_VRInitError_Init_AnotherAppLaunching = 117.
    InitAnotherAppLaunching = sys::EVRInitError_VRInitError_Init_AnotherAppLaunching,
    /// EVRInitError_VRInitError_Init_SettingsInitFailed = 118.
    InitSettingsInitFailed = sys::EVRInitError_VRInitError_Init_SettingsInitFailed,
    /// EVRInitError_VRInitError_Init_ShuttingDown = 119.
    InitShuttingDown = sys::EVRInitError_VRInitError_Init_ShuttingDown,
    /// EVRInitError_VRInitError_Init_TooManyObjects = 120.
    InitTooManyObject = sys::EVRInitError_VRInitError_Init_TooManyObjects,
    /// EVRInitError_VRInitError_Init_NoServerForBackgroundApp = 121.
    InitNoServerForBackgroundApp = sys::EVRInitError_VRInitError_Init_NoServerForBackgroundApp,
    /// EVRInitError_VRInitError_Init_NotSupportedWithCompositor = 122.
    InitNotSupportedWithCompositor = sys::EVRInitError_VRInitError_Init_NotSupportedWithCompositor,
    /// EVRInitError_VRInitError_Init_NotAvailableToUtilityApps = 123.
    InitNotAvailableToUtilityApp = sys::EVRInitError_VRInitError_Init_NotAvailableToUtilityApps,
    /// EVRInitError_VRInitError_Init_Internal = 124.
    InitInternal = sys::EVRInitError_VRInitError_Init_Internal,
    /// EVRInitError_VRInitError_Init_HmdDriverIdIsNone = 125.
    InitHmdDriverIdIsNone = sys::EVRInitError_VRInitError_Init_HmdDriverIdIsNone,
    /// EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed = 126.
    InitHmdNotFoundPresenceFailed = sys::EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed,
    /// EVRInitError_VRInitError_Init_VRMonitorNotFound = 127.
    InitVRMonitorNotFound = sys::EVRInitError_VRInitError_Init_VRMonitorNotFound,
    /// EVRInitError_VRInitError_Init_VRMonitorStartupFailed = 128.
    InitVRMonitorStartupFailed = sys::EVRInitError_VRInitError_Init_VRMonitorStartupFailed,
    /// EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported = 129.
    InitLowPowerWatchdogNotSupported = sys::EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported,
    /// EVRInitError_VRInitError_Init_InvalidApplicationType = 130.
    InitInvalidApplicationType = sys::EVRInitError_VRInitError_Init_InvalidApplicationType,
    /// EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps = 131.
    InitNotAvailableToWatchdogApp = sys::EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps,
    /// EVRInitError_VRInitError_Init_WatchdogDisabledInSettings = 132.
    InitWatchdogDisabledInSetting = sys::EVRInitError_VRInitError_Init_WatchdogDisabledInSettings,
    /// EVRInitError_VRInitError_Init_VRDashboardNotFound = 133.
    InitVRDashboardNotFound = sys::EVRInitError_VRInitError_Init_VRDashboardNotFound,
    /// EVRInitError_VRInitError_Init_VRDashboardStartupFailed = 134.
    InitVRDashboardStartupFailed = sys::EVRInitError_VRInitError_Init_VRDashboardStartupFailed,
    /// EVRInitError_VRInitError_Init_VRHomeNotFound = 135.
    InitVRHomeNotFound = sys::EVRInitError_VRInitError_Init_VRHomeNotFound,
    /// EVRInitError_VRInitError_Init_VRHomeStartupFailed = 136.
    InitVRHomeStartupFailed = sys::EVRInitError_VRInitError_Init_VRHomeStartupFailed,
    /// EVRInitError_VRInitError_Init_RebootingBusy = 137.
    InitRebootingBusy = sys::EVRInitError_VRInitError_Init_RebootingBusy,
    /// EVRInitError_VRInitError_Init_FirmwareUpdateBusy = 138.
    InitFirmwareUpdateBusy = sys::EVRInitError_VRInitError_Init_FirmwareUpdateBusy,
    /// EVRInitError_VRInitError_Init_FirmwareRecoveryBusy = 139.
    InitFirmwareRecoveryBusy = sys::EVRInitError_VRInitError_Init_FirmwareRecoveryBusy,
    /// EVRInitError_VRInitError_Init_USBServiceBusy = 140.
    InitUSBServiceBusy = sys::EVRInitError_VRInitError_Init_USBServiceBusy,
    /// EVRInitError_VRInitError_Init_VRWebHelperStartupFailed = 141.
    InitVRWebHelperStartupFailed = sys::EVRInitError_VRInitError_Init_VRWebHelperStartupFailed,
    /// EVRInitError_VRInitError_Init_TrackerManagerInitFailed = 142.
    InitTrackerManagerInitFailed = sys::EVRInitError_VRInitError_Init_TrackerManagerInitFailed,
    /// EVRInitError_VRInitError_Driver_Failed = 200.
    DriverFailed = sys::EVRInitError_VRInitError_Driver_Failed,
    /// EVRInitError_VRInitError_Driver_Unknown = 201.
    DriverUnknown = sys::EVRInitError_VRInitError_Driver_Unknown,
    /// EVRInitError_VRInitError_Driver_HmdUnknown = 202.
    DriverHmdUnknown = sys::EVRInitError_VRInitError_Driver_HmdUnknown,
    /// EVRInitError_VRInitError_Driver_NotLoaded = 203.
    DriverNotLoaded = sys::EVRInitError_VRInitError_Driver_NotLoaded,
    /// EVRInitError_VRInitError_Driver_RuntimeOutOfDate = 204.
    DriverRuntimeOutOfDate = sys::EVRInitError_VRInitError_Driver_RuntimeOutOfDate,
    /// EVRInitError_VRInitError_Driver_HmdInUse = 205.
    DriverHmdInUse = sys::EVRInitError_VRInitError_Driver_HmdInUse,
    /// EVRInitError_VRInitError_Driver_NotCalibrated = 206.
    DriverNotCalibrated = sys::EVRInitError_VRInitError_Driver_NotCalibrated,
    /// EVRInitError_VRInitError_Driver_CalibrationInvalid = 207.
    DriverCalibrationInvalid = sys::EVRInitError_VRInitError_Driver_CalibrationInvalid,
    /// EVRInitError_VRInitError_Driver_HmdDisplayNotFound = 208.
    DriverHmdDisplayNotFound = sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFound,
    /// EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown = 209.
    DriverTrackedDeviceInterfaceUnknown = sys::EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown,
    /// EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds = 211.
    DriverHmdDriverIdOutOfBound = sys::EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds,
    /// EVRInitError_VRInitError_Driver_HmdDisplayMirrored = 212.
    DriverHmdDisplayMirrored = sys::EVRInitError_VRInitError_Driver_HmdDisplayMirrored,
    /// EVRInitError_VRInitError_IPC_ServerInitFailed = 300.
    IpcServerInitFailed = sys::EVRInitError_VRInitError_IPC_ServerInitFailed,
    /// EVRInitError_VRInitError_IPC_ConnectFailed = 301.
    IpcConnectFailed = sys::EVRInitError_VRInitError_IPC_ConnectFailed,
    /// EVRInitError_VRInitError_IPC_SharedStateInitFailed = 302.
    IpcSharedStateInitFailed = sys::EVRInitError_VRInitError_IPC_SharedStateInitFailed,
    /// EVRInitError_VRInitError_IPC_CompositorInitFailed = 303.
    IpcCompositorInitFailed = sys::EVRInitError_VRInitError_IPC_CompositorInitFailed,
    /// EVRInitError_VRInitError_IPC_MutexInitFailed = 304.
    IpcMutexInitFailed = sys::EVRInitError_VRInitError_IPC_MutexInitFailed,
    /// EVRInitError_VRInitError_IPC_Failed = 305.
    IpcFailed = sys::EVRInitError_VRInitError_IPC_Failed,
    /// EVRInitError_VRInitError_IPC_CompositorConnectFailed = 306.
    IpcCompositorConnectFailed = sys::EVRInitError_VRInitError_IPC_CompositorConnectFailed,
    /// EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse = 307.
    IpcCompositorInvalidConnectResponse = sys::EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse,
    /// EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts = 308.
    IpcConnectFailedAfterMultipleAttempt = sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts,
    /// EVRInitError_VRInitError_Compositor_Failed = 400.
    CompositorFailed = sys::EVRInitError_VRInitError_Compositor_Failed,
    /// EVRInitError_VRInitError_Compositor_D3D11HardwareRequired = 401.
    CompositorD3D11HardwareRequired = sys::EVRInitError_VRInitError_Compositor_D3D11HardwareRequired,
    /// EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate = 402.
    CompositorFirmwareRequiresUpdate = sys::EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate,
    /// EVRInitError_VRInitError_Compositor_OverlayInitFailed = 403.
    CompositorOverlayInitFailed = sys::EVRInitError_VRInitError_Compositor_OverlayInitFailed,
    /// EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed = 404.
    CompositorScreenshotsInitFailed = sys::EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed,
    /// EVRInitError_VRInitError_Compositor_UnableToCreateDevice = 405.
    CompositorUnableToCreateDevice = sys::EVRInitError_VRInitError_Compositor_UnableToCreateDevice,
    /// EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime = 1000.
    VendorSpecificUnableToConnectToOculusRuntime = sys::EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime,
    /// EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode = 1001.
    VendorSpecificWindowsNotInDevMode = sys::EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice = 1101.
    VendorSpecificHmdFoundCantOpenDevice = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart = 1102.
    VendorSpecificHmdFoundUnableToRequestConfigStart = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig = 1103.
    VendorSpecificHmdFoundNoStoredConfig = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig = 1104.
    VendorSpecificHmdFoundConfigTooBig = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall = 1105.
    VendorSpecificHmdFoundConfigTooSmall = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib = 1106.
    VendorSpecificHmdFoundUnableToInitZLib = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion = 1107.
    VendorSpecificHmdFoundCantReadFirmwareVersion = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart = 1108.
    VendorSpecificHmdFoundUnableToSendUserDataStart = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart = 1109.
    VendorSpecificHmdFoundUnableToGetUserDataStart = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext = 1110.
    VendorSpecificHmdFoundUnableToGetUserDataNext = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange = 1111.
    VendorSpecificHmdFoundUserDataAddressRange = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError = 1112.
    VendorSpecificHmdFoundUserDataError = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError,
    /// EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck = 1113.
    VendorSpecificHmdFoundConfigFailedSanityCheck = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck,
    /// EVRInitError_VRInitError_Steam_SteamInstallationNotFound = 2000.
    SteamSteamInstallationNotFound = sys::EVRInitError_VRInitError_Steam_SteamInstallationNotFound,
}

impl Enum for InitError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRInitError_VRInitError_None => Ok(InitError::None),
             sys::EVRInitError_VRInitError_Unknown => Ok(InitError::Unknown),
             sys::EVRInitError_VRInitError_Init_InstallationNotFound => Ok(InitError::InitInstallationNotFound),
             sys::EVRInitError_VRInitError_Init_InstallationCorrupt => Ok(InitError::InitInstallationCorrupt),
             sys::EVRInitError_VRInitError_Init_VRClientDLLNotFound => Ok(InitError::InitVRClientDLLNotFound),
             sys::EVRInitError_VRInitError_Init_FileNotFound => Ok(InitError::InitFileNotFound),
             sys::EVRInitError_VRInitError_Init_FactoryNotFound => Ok(InitError::InitFactoryNotFound),
             sys::EVRInitError_VRInitError_Init_InterfaceNotFound => Ok(InitError::InitInterfaceNotFound),
             sys::EVRInitError_VRInitError_Init_InvalidInterface => Ok(InitError::InitInvalidInterface),
             sys::EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid => Ok(InitError::InitUserConfigDirectoryInvalid),
             sys::EVRInitError_VRInitError_Init_HmdNotFound => Ok(InitError::InitHmdNotFound),
             sys::EVRInitError_VRInitError_Init_NotInitialized => Ok(InitError::InitNotInitialized),
             sys::EVRInitError_VRInitError_Init_PathRegistryNotFound => Ok(InitError::InitPathRegistryNotFound),
             sys::EVRInitError_VRInitError_Init_NoConfigPath => Ok(InitError::InitNoConfigPath),
             sys::EVRInitError_VRInitError_Init_NoLogPath => Ok(InitError::InitNoLogPath),
             sys::EVRInitError_VRInitError_Init_PathRegistryNotWritable => Ok(InitError::InitPathRegistryNotWritable),
             sys::EVRInitError_VRInitError_Init_AppInfoInitFailed => Ok(InitError::InitAppInfoInitFailed),
             sys::EVRInitError_VRInitError_Init_Retry => Ok(InitError::InitRetry),
             sys::EVRInitError_VRInitError_Init_InitCanceledByUser => Ok(InitError::InitInitCanceledByUser),
             sys::EVRInitError_VRInitError_Init_AnotherAppLaunching => Ok(InitError::InitAnotherAppLaunching),
             sys::EVRInitError_VRInitError_Init_SettingsInitFailed => Ok(InitError::InitSettingsInitFailed),
             sys::EVRInitError_VRInitError_Init_ShuttingDown => Ok(InitError::InitShuttingDown),
             sys::EVRInitError_VRInitError_Init_TooManyObjects => Ok(InitError::InitTooManyObject),
             sys::EVRInitError_VRInitError_Init_NoServerForBackgroundApp => Ok(InitError::InitNoServerForBackgroundApp),
             sys::EVRInitError_VRInitError_Init_NotSupportedWithCompositor => Ok(InitError::InitNotSupportedWithCompositor),
             sys::EVRInitError_VRInitError_Init_NotAvailableToUtilityApps => Ok(InitError::InitNotAvailableToUtilityApp),
             sys::EVRInitError_VRInitError_Init_Internal => Ok(InitError::InitInternal),
             sys::EVRInitError_VRInitError_Init_HmdDriverIdIsNone => Ok(InitError::InitHmdDriverIdIsNone),
             sys::EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed => Ok(InitError::InitHmdNotFoundPresenceFailed),
             sys::EVRInitError_VRInitError_Init_VRMonitorNotFound => Ok(InitError::InitVRMonitorNotFound),
             sys::EVRInitError_VRInitError_Init_VRMonitorStartupFailed => Ok(InitError::InitVRMonitorStartupFailed),
             sys::EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported => Ok(InitError::InitLowPowerWatchdogNotSupported),
             sys::EVRInitError_VRInitError_Init_InvalidApplicationType => Ok(InitError::InitInvalidApplicationType),
             sys::EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps => Ok(InitError::InitNotAvailableToWatchdogApp),
             sys::EVRInitError_VRInitError_Init_WatchdogDisabledInSettings => Ok(InitError::InitWatchdogDisabledInSetting),
             sys::EVRInitError_VRInitError_Init_VRDashboardNotFound => Ok(InitError::InitVRDashboardNotFound),
             sys::EVRInitError_VRInitError_Init_VRDashboardStartupFailed => Ok(InitError::InitVRDashboardStartupFailed),
             sys::EVRInitError_VRInitError_Init_VRHomeNotFound => Ok(InitError::InitVRHomeNotFound),
             sys::EVRInitError_VRInitError_Init_VRHomeStartupFailed => Ok(InitError::InitVRHomeStartupFailed),
             sys::EVRInitError_VRInitError_Init_RebootingBusy => Ok(InitError::InitRebootingBusy),
             sys::EVRInitError_VRInitError_Init_FirmwareUpdateBusy => Ok(InitError::InitFirmwareUpdateBusy),
             sys::EVRInitError_VRInitError_Init_FirmwareRecoveryBusy => Ok(InitError::InitFirmwareRecoveryBusy),
             sys::EVRInitError_VRInitError_Init_USBServiceBusy => Ok(InitError::InitUSBServiceBusy),
             sys::EVRInitError_VRInitError_Init_VRWebHelperStartupFailed => Ok(InitError::InitVRWebHelperStartupFailed),
             sys::EVRInitError_VRInitError_Init_TrackerManagerInitFailed => Ok(InitError::InitTrackerManagerInitFailed),
             sys::EVRInitError_VRInitError_Driver_Failed => Ok(InitError::DriverFailed),
             sys::EVRInitError_VRInitError_Driver_Unknown => Ok(InitError::DriverUnknown),
             sys::EVRInitError_VRInitError_Driver_HmdUnknown => Ok(InitError::DriverHmdUnknown),
             sys::EVRInitError_VRInitError_Driver_NotLoaded => Ok(InitError::DriverNotLoaded),
             sys::EVRInitError_VRInitError_Driver_RuntimeOutOfDate => Ok(InitError::DriverRuntimeOutOfDate),
             sys::EVRInitError_VRInitError_Driver_HmdInUse => Ok(InitError::DriverHmdInUse),
             sys::EVRInitError_VRInitError_Driver_NotCalibrated => Ok(InitError::DriverNotCalibrated),
             sys::EVRInitError_VRInitError_Driver_CalibrationInvalid => Ok(InitError::DriverCalibrationInvalid),
             sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFound => Ok(InitError::DriverHmdDisplayNotFound),
             sys::EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown => Ok(InitError::DriverTrackedDeviceInterfaceUnknown),
             sys::EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds => Ok(InitError::DriverHmdDriverIdOutOfBound),
             sys::EVRInitError_VRInitError_Driver_HmdDisplayMirrored => Ok(InitError::DriverHmdDisplayMirrored),
             sys::EVRInitError_VRInitError_IPC_ServerInitFailed => Ok(InitError::IpcServerInitFailed),
             sys::EVRInitError_VRInitError_IPC_ConnectFailed => Ok(InitError::IpcConnectFailed),
             sys::EVRInitError_VRInitError_IPC_SharedStateInitFailed => Ok(InitError::IpcSharedStateInitFailed),
             sys::EVRInitError_VRInitError_IPC_CompositorInitFailed => Ok(InitError::IpcCompositorInitFailed),
             sys::EVRInitError_VRInitError_IPC_MutexInitFailed => Ok(InitError::IpcMutexInitFailed),
             sys::EVRInitError_VRInitError_IPC_Failed => Ok(InitError::IpcFailed),
             sys::EVRInitError_VRInitError_IPC_CompositorConnectFailed => Ok(InitError::IpcCompositorConnectFailed),
             sys::EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse => Ok(InitError::IpcCompositorInvalidConnectResponse),
             sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts => Ok(InitError::IpcConnectFailedAfterMultipleAttempt),
             sys::EVRInitError_VRInitError_Compositor_Failed => Ok(InitError::CompositorFailed),
             sys::EVRInitError_VRInitError_Compositor_D3D11HardwareRequired => Ok(InitError::CompositorD3D11HardwareRequired),
             sys::EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate => Ok(InitError::CompositorFirmwareRequiresUpdate),
             sys::EVRInitError_VRInitError_Compositor_OverlayInitFailed => Ok(InitError::CompositorOverlayInitFailed),
             sys::EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed => Ok(InitError::CompositorScreenshotsInitFailed),
             sys::EVRInitError_VRInitError_Compositor_UnableToCreateDevice => Ok(InitError::CompositorUnableToCreateDevice),
             sys::EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime => Ok(InitError::VendorSpecificUnableToConnectToOculusRuntime),
             sys::EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode => Ok(InitError::VendorSpecificWindowsNotInDevMode),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice => Ok(InitError::VendorSpecificHmdFoundCantOpenDevice),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart => Ok(InitError::VendorSpecificHmdFoundUnableToRequestConfigStart),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig => Ok(InitError::VendorSpecificHmdFoundNoStoredConfig),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig => Ok(InitError::VendorSpecificHmdFoundConfigTooBig),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall => Ok(InitError::VendorSpecificHmdFoundConfigTooSmall),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib => Ok(InitError::VendorSpecificHmdFoundUnableToInitZLib),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion => Ok(InitError::VendorSpecificHmdFoundCantReadFirmwareVersion),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart => Ok(InitError::VendorSpecificHmdFoundUnableToSendUserDataStart),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart => Ok(InitError::VendorSpecificHmdFoundUnableToGetUserDataStart),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext => Ok(InitError::VendorSpecificHmdFoundUnableToGetUserDataNext),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange => Ok(InitError::VendorSpecificHmdFoundUserDataAddressRange),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError => Ok(InitError::VendorSpecificHmdFoundUserDataError),
             sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck => Ok(InitError::VendorSpecificHmdFoundConfigFailedSanityCheck),
             sys::EVRInitError_VRInitError_Steam_SteamInstallationNotFound => Ok(InitError::SteamSteamInstallationNotFound),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<InitError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of InitError.", self.0)
    }
}

impl error::Error for Invalid<InitError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of InitError."
    }
}

/// EVRScreenshotType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ScreenshotType {
    /// EVRScreenshotType_VRScreenshotType_None = 0.
    None = sys::EVRScreenshotType_VRScreenshotType_None,
    /// EVRScreenshotType_VRScreenshotType_Mono = 1.
    Mono = sys::EVRScreenshotType_VRScreenshotType_Mono,
    /// EVRScreenshotType_VRScreenshotType_Stereo = 2.
    Stereo = sys::EVRScreenshotType_VRScreenshotType_Stereo,
    /// EVRScreenshotType_VRScreenshotType_Cubemap = 3.
    Cubemap = sys::EVRScreenshotType_VRScreenshotType_Cubemap,
    /// EVRScreenshotType_VRScreenshotType_MonoPanorama = 4.
    MonoPanorama = sys::EVRScreenshotType_VRScreenshotType_MonoPanorama,
    /// EVRScreenshotType_VRScreenshotType_StereoPanorama = 5.
    StereoPanorama = sys::EVRScreenshotType_VRScreenshotType_StereoPanorama,
}

impl Enum for ScreenshotType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRScreenshotType_VRScreenshotType_None => Ok(ScreenshotType::None),
             sys::EVRScreenshotType_VRScreenshotType_Mono => Ok(ScreenshotType::Mono),
             sys::EVRScreenshotType_VRScreenshotType_Stereo => Ok(ScreenshotType::Stereo),
             sys::EVRScreenshotType_VRScreenshotType_Cubemap => Ok(ScreenshotType::Cubemap),
             sys::EVRScreenshotType_VRScreenshotType_MonoPanorama => Ok(ScreenshotType::MonoPanorama),
             sys::EVRScreenshotType_VRScreenshotType_StereoPanorama => Ok(ScreenshotType::StereoPanorama),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ScreenshotType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ScreenshotType.", self.0)
    }
}

impl error::Error for Invalid<ScreenshotType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ScreenshotType."
    }
}

/// EVRScreenshotPropertyFilenames.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ScreenshotPropertyFilename {
    /// EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_Preview = 0.
    Preview = sys::EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_Preview,
    /// EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_VR = 1.
    Vr = sys::EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_VR,
}

impl Enum for ScreenshotPropertyFilename {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_Preview => Ok(ScreenshotPropertyFilename::Preview),
             sys::EVRScreenshotPropertyFilenames_VRScreenshotPropertyFilenames_VR => Ok(ScreenshotPropertyFilename::Vr),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ScreenshotPropertyFilename> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ScreenshotPropertyFilename.", self.0)
    }
}

impl error::Error for Invalid<ScreenshotPropertyFilename> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ScreenshotPropertyFilename."
    }
}

/// EVRTrackedCameraError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedCameraError {
    /// EVRTrackedCameraError_VRTrackedCameraError_None = 0.
    None = sys::EVRTrackedCameraError_VRTrackedCameraError_None,
    /// EVRTrackedCameraError_VRTrackedCameraError_OperationFailed = 100.
    OperationFailed = sys::EVRTrackedCameraError_VRTrackedCameraError_OperationFailed,
    /// EVRTrackedCameraError_VRTrackedCameraError_InvalidHandle = 101.
    InvalidHandle = sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidHandle,
    /// EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameHeaderVersion = 102.
    InvalidFrameHeaderVersion = sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameHeaderVersion,
    /// EVRTrackedCameraError_VRTrackedCameraError_OutOfHandles = 103.
    OutOfHandle = sys::EVRTrackedCameraError_VRTrackedCameraError_OutOfHandles,
    /// EVRTrackedCameraError_VRTrackedCameraError_IPCFailure = 104.
    Ipcfailure = sys::EVRTrackedCameraError_VRTrackedCameraError_IPCFailure,
    /// EVRTrackedCameraError_VRTrackedCameraError_NotSupportedForThisDevice = 105.
    NotSupportedForThisDevice = sys::EVRTrackedCameraError_VRTrackedCameraError_NotSupportedForThisDevice,
    /// EVRTrackedCameraError_VRTrackedCameraError_SharedMemoryFailure = 106.
    SharedMemoryFailure = sys::EVRTrackedCameraError_VRTrackedCameraError_SharedMemoryFailure,
    /// EVRTrackedCameraError_VRTrackedCameraError_FrameBufferingFailure = 107.
    FrameBufferingFailure = sys::EVRTrackedCameraError_VRTrackedCameraError_FrameBufferingFailure,
    /// EVRTrackedCameraError_VRTrackedCameraError_StreamSetupFailure = 108.
    StreamSetupFailure = sys::EVRTrackedCameraError_VRTrackedCameraError_StreamSetupFailure,
    /// EVRTrackedCameraError_VRTrackedCameraError_InvalidGLTextureId = 109.
    InvalidGLTextureId = sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidGLTextureId,
    /// EVRTrackedCameraError_VRTrackedCameraError_InvalidSharedTextureHandle = 110.
    InvalidSharedTextureHandle = sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidSharedTextureHandle,
    /// EVRTrackedCameraError_VRTrackedCameraError_FailedToGetGLTextureId = 111.
    FailedToGetGLTextureId = sys::EVRTrackedCameraError_VRTrackedCameraError_FailedToGetGLTextureId,
    /// EVRTrackedCameraError_VRTrackedCameraError_SharedTextureFailure = 112.
    SharedTextureFailure = sys::EVRTrackedCameraError_VRTrackedCameraError_SharedTextureFailure,
    /// EVRTrackedCameraError_VRTrackedCameraError_NoFrameAvailable = 113.
    NoFrameAvailable = sys::EVRTrackedCameraError_VRTrackedCameraError_NoFrameAvailable,
    /// EVRTrackedCameraError_VRTrackedCameraError_InvalidArgument = 114.
    InvalidArgument = sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidArgument,
    /// EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameBufferSize = 115.
    InvalidFrameBufferSize = sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameBufferSize,
}

impl Enum for TrackedCameraError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRTrackedCameraError_VRTrackedCameraError_None => Ok(TrackedCameraError::None),
             sys::EVRTrackedCameraError_VRTrackedCameraError_OperationFailed => Ok(TrackedCameraError::OperationFailed),
             sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidHandle => Ok(TrackedCameraError::InvalidHandle),
             sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameHeaderVersion => Ok(TrackedCameraError::InvalidFrameHeaderVersion),
             sys::EVRTrackedCameraError_VRTrackedCameraError_OutOfHandles => Ok(TrackedCameraError::OutOfHandle),
             sys::EVRTrackedCameraError_VRTrackedCameraError_IPCFailure => Ok(TrackedCameraError::Ipcfailure),
             sys::EVRTrackedCameraError_VRTrackedCameraError_NotSupportedForThisDevice => Ok(TrackedCameraError::NotSupportedForThisDevice),
             sys::EVRTrackedCameraError_VRTrackedCameraError_SharedMemoryFailure => Ok(TrackedCameraError::SharedMemoryFailure),
             sys::EVRTrackedCameraError_VRTrackedCameraError_FrameBufferingFailure => Ok(TrackedCameraError::FrameBufferingFailure),
             sys::EVRTrackedCameraError_VRTrackedCameraError_StreamSetupFailure => Ok(TrackedCameraError::StreamSetupFailure),
             sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidGLTextureId => Ok(TrackedCameraError::InvalidGLTextureId),
             sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidSharedTextureHandle => Ok(TrackedCameraError::InvalidSharedTextureHandle),
             sys::EVRTrackedCameraError_VRTrackedCameraError_FailedToGetGLTextureId => Ok(TrackedCameraError::FailedToGetGLTextureId),
             sys::EVRTrackedCameraError_VRTrackedCameraError_SharedTextureFailure => Ok(TrackedCameraError::SharedTextureFailure),
             sys::EVRTrackedCameraError_VRTrackedCameraError_NoFrameAvailable => Ok(TrackedCameraError::NoFrameAvailable),
             sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidArgument => Ok(TrackedCameraError::InvalidArgument),
             sys::EVRTrackedCameraError_VRTrackedCameraError_InvalidFrameBufferSize => Ok(TrackedCameraError::InvalidFrameBufferSize),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackedCameraError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackedCameraError.", self.0)
    }
}

impl error::Error for Invalid<TrackedCameraError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackedCameraError."
    }
}

/// EVRTrackedCameraFrameLayout.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedCameraFrameLayout {
    /// EVRTrackedCameraFrameLayout_Mono = 1.
    Mono = sys::EVRTrackedCameraFrameLayout_Mono,
    /// EVRTrackedCameraFrameLayout_Stereo = 2.
    Stereo = sys::EVRTrackedCameraFrameLayout_Stereo,
    /// EVRTrackedCameraFrameLayout_VerticalLayout = 16.
    VerticalLayout = sys::EVRTrackedCameraFrameLayout_VerticalLayout,
    /// EVRTrackedCameraFrameLayout_HorizontalLayout = 32.
    HorizontalLayout = sys::EVRTrackedCameraFrameLayout_HorizontalLayout,
}

impl Enum for TrackedCameraFrameLayout {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRTrackedCameraFrameLayout_Mono => Ok(TrackedCameraFrameLayout::Mono),
             sys::EVRTrackedCameraFrameLayout_Stereo => Ok(TrackedCameraFrameLayout::Stereo),
             sys::EVRTrackedCameraFrameLayout_VerticalLayout => Ok(TrackedCameraFrameLayout::VerticalLayout),
             sys::EVRTrackedCameraFrameLayout_HorizontalLayout => Ok(TrackedCameraFrameLayout::HorizontalLayout),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackedCameraFrameLayout> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackedCameraFrameLayout.", self.0)
    }
}

impl error::Error for Invalid<TrackedCameraFrameLayout> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackedCameraFrameLayout."
    }
}

/// EVRTrackedCameraFrameType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TrackedCameraFrameType {
    /// EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Distorted = 0.
    Distorted = sys::EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Distorted,
    /// EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Undistorted = 1.
    Undistorted = sys::EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Undistorted,
    /// EVRTrackedCameraFrameType_VRTrackedCameraFrameType_MaximumUndistorted = 2.
    MaximumUndistorted = sys::EVRTrackedCameraFrameType_VRTrackedCameraFrameType_MaximumUndistorted,
    /// EVRTrackedCameraFrameType_MAX_CAMERA_FRAME_TYPES = 3.
    CameraFrameType = sys::EVRTrackedCameraFrameType_MAX_CAMERA_FRAME_TYPES,
}

impl Enum for TrackedCameraFrameType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Distorted => Ok(TrackedCameraFrameType::Distorted),
             sys::EVRTrackedCameraFrameType_VRTrackedCameraFrameType_Undistorted => Ok(TrackedCameraFrameType::Undistorted),
             sys::EVRTrackedCameraFrameType_VRTrackedCameraFrameType_MaximumUndistorted => Ok(TrackedCameraFrameType::MaximumUndistorted),
             sys::EVRTrackedCameraFrameType_MAX_CAMERA_FRAME_TYPES => Ok(TrackedCameraFrameType::CameraFrameType),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<TrackedCameraFrameType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of TrackedCameraFrameType.", self.0)
    }
}

impl error::Error for Invalid<TrackedCameraFrameType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of TrackedCameraFrameType."
    }
}

/// EVRDistortionFunctionType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DistortionFunctionType {
    /// EVRDistortionFunctionType_VRDistortionFunctionType_None = 0.
    None = sys::EVRDistortionFunctionType_VRDistortionFunctionType_None,
    /// EVRDistortionFunctionType_VRDistortionFunctionType_FTheta = 1.
    Ftheum = sys::EVRDistortionFunctionType_VRDistortionFunctionType_FTheta,
    /// EVRDistortionFunctionType_VRDistortionFucntionType_Extended_FTheta = 2.
    ExtendedFTheum = sys::EVRDistortionFunctionType_VRDistortionFucntionType_Extended_FTheta,
    /// EVRDistortionFunctionType_MAX_DISTORTION_FUNCTION_TYPES = 3.
    DistortionFunctionType = sys::EVRDistortionFunctionType_MAX_DISTORTION_FUNCTION_TYPES,
}

impl Enum for DistortionFunctionType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRDistortionFunctionType_VRDistortionFunctionType_None => Ok(DistortionFunctionType::None),
             sys::EVRDistortionFunctionType_VRDistortionFunctionType_FTheta => Ok(DistortionFunctionType::Ftheum),
             sys::EVRDistortionFunctionType_VRDistortionFucntionType_Extended_FTheta => Ok(DistortionFunctionType::ExtendedFTheum),
             sys::EVRDistortionFunctionType_MAX_DISTORTION_FUNCTION_TYPES => Ok(DistortionFunctionType::DistortionFunctionType),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<DistortionFunctionType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of DistortionFunctionType.", self.0)
    }
}

impl error::Error for Invalid<DistortionFunctionType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of DistortionFunctionType."
    }
}

/// EVSync.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Vsync {
    /// EVSync_VSync_None = 0.
    None = sys::EVSync_VSync_None,
    /// EVSync_VSync_WaitRender = 1.
    WaitRender = sys::EVSync_VSync_WaitRender,
    /// EVSync_VSync_NoWaitRender = 2.
    NoWaitRender = sys::EVSync_VSync_NoWaitRender,
}

impl Enum for Vsync {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVSync_VSync_None => Ok(Vsync::None),
             sys::EVSync_VSync_WaitRender => Ok(Vsync::WaitRender),
             sys::EVSync_VSync_NoWaitRender => Ok(Vsync::NoWaitRender),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<Vsync> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of Vsync.", self.0)
    }
}

impl error::Error for Invalid<Vsync> {
    fn description(&self) -> &str {
        "Value does not represent any variant of Vsync."
    }
}

/// EVRMuraCorrectionMode.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MuraCorrectionMode {
    /// EVRMuraCorrectionMode_Default = 0.
    Default = sys::EVRMuraCorrectionMode_Default,
    /// EVRMuraCorrectionMode_NoCorrection = 1.
    NoCorrection = sys::EVRMuraCorrectionMode_NoCorrection,
}

impl Enum for MuraCorrectionMode {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRMuraCorrectionMode_Default => Ok(MuraCorrectionMode::Default),
             sys::EVRMuraCorrectionMode_NoCorrection => Ok(MuraCorrectionMode::NoCorrection),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<MuraCorrectionMode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of MuraCorrectionMode.", self.0)
    }
}

impl error::Error for Invalid<MuraCorrectionMode> {
    fn description(&self) -> &str {
        "Value does not represent any variant of MuraCorrectionMode."
    }
}

/// Imu_OffScaleFlags.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ImuOffScaleFlag {
    /// Imu_OffScaleFlags_OffScale_AccelX = 1.
    AccelX = sys::Imu_OffScaleFlags_OffScale_AccelX,
    /// Imu_OffScaleFlags_OffScale_AccelY = 2.
    AccelY = sys::Imu_OffScaleFlags_OffScale_AccelY,
    /// Imu_OffScaleFlags_OffScale_AccelZ = 4.
    AccelZ = sys::Imu_OffScaleFlags_OffScale_AccelZ,
    /// Imu_OffScaleFlags_OffScale_GyroX = 8.
    GyroX = sys::Imu_OffScaleFlags_OffScale_GyroX,
    /// Imu_OffScaleFlags_OffScale_GyroY = 16.
    GyroY = sys::Imu_OffScaleFlags_OffScale_GyroY,
    /// Imu_OffScaleFlags_OffScale_GyroZ = 32.
    GyroZ = sys::Imu_OffScaleFlags_OffScale_GyroZ,
}

impl Enum for ImuOffScaleFlag {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::Imu_OffScaleFlags_OffScale_AccelX => Ok(ImuOffScaleFlag::AccelX),
             sys::Imu_OffScaleFlags_OffScale_AccelY => Ok(ImuOffScaleFlag::AccelY),
             sys::Imu_OffScaleFlags_OffScale_AccelZ => Ok(ImuOffScaleFlag::AccelZ),
             sys::Imu_OffScaleFlags_OffScale_GyroX => Ok(ImuOffScaleFlag::GyroX),
             sys::Imu_OffScaleFlags_OffScale_GyroY => Ok(ImuOffScaleFlag::GyroY),
             sys::Imu_OffScaleFlags_OffScale_GyroZ => Ok(ImuOffScaleFlag::GyroZ),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ImuOffScaleFlag> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ImuOffScaleFlag.", self.0)
    }
}

impl error::Error for Invalid<ImuOffScaleFlag> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ImuOffScaleFlag."
    }
}

/// EVRApplicationError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationError {
    /// EVRApplicationError_VRApplicationError_None = 0.
    None = sys::EVRApplicationError_VRApplicationError_None,
    /// EVRApplicationError_VRApplicationError_AppKeyAlreadyExists = 100.
    AppKeyAlreadyExist = sys::EVRApplicationError_VRApplicationError_AppKeyAlreadyExists,
    /// EVRApplicationError_VRApplicationError_NoManifest = 101.
    NoManifest = sys::EVRApplicationError_VRApplicationError_NoManifest,
    /// EVRApplicationError_VRApplicationError_NoApplication = 102.
    NoApplication = sys::EVRApplicationError_VRApplicationError_NoApplication,
    /// EVRApplicationError_VRApplicationError_InvalidIndex = 103.
    InvalidIndex = sys::EVRApplicationError_VRApplicationError_InvalidIndex,
    /// EVRApplicationError_VRApplicationError_UnknownApplication = 104.
    UnknownApplication = sys::EVRApplicationError_VRApplicationError_UnknownApplication,
    /// EVRApplicationError_VRApplicationError_IPCFailed = 105.
    Ipcfailed = sys::EVRApplicationError_VRApplicationError_IPCFailed,
    /// EVRApplicationError_VRApplicationError_ApplicationAlreadyRunning = 106.
    ApplicationAlreadyRunning = sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyRunning,
    /// EVRApplicationError_VRApplicationError_InvalidManifest = 107.
    InvalidManifest = sys::EVRApplicationError_VRApplicationError_InvalidManifest,
    /// EVRApplicationError_VRApplicationError_InvalidApplication = 108.
    InvalidApplication = sys::EVRApplicationError_VRApplicationError_InvalidApplication,
    /// EVRApplicationError_VRApplicationError_LaunchFailed = 109.
    LaunchFailed = sys::EVRApplicationError_VRApplicationError_LaunchFailed,
    /// EVRApplicationError_VRApplicationError_ApplicationAlreadyStarting = 110.
    ApplicationAlreadyStarting = sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyStarting,
    /// EVRApplicationError_VRApplicationError_LaunchInProgress = 111.
    LaunchInProgress = sys::EVRApplicationError_VRApplicationError_LaunchInProgress,
    /// EVRApplicationError_VRApplicationError_OldApplicationQuitting = 112.
    OldApplicationQuitting = sys::EVRApplicationError_VRApplicationError_OldApplicationQuitting,
    /// EVRApplicationError_VRApplicationError_TransitionAborted = 113.
    TransitionAborted = sys::EVRApplicationError_VRApplicationError_TransitionAborted,
    /// EVRApplicationError_VRApplicationError_IsTemplate = 114.
    IsTemplate = sys::EVRApplicationError_VRApplicationError_IsTemplate,
    /// EVRApplicationError_VRApplicationError_SteamVRIsExiting = 115.
    SteamVRIsExiting = sys::EVRApplicationError_VRApplicationError_SteamVRIsExiting,
    /// EVRApplicationError_VRApplicationError_BufferTooSmall = 200.
    BufferTooSmall = sys::EVRApplicationError_VRApplicationError_BufferTooSmall,
    /// EVRApplicationError_VRApplicationError_PropertyNotSet = 201.
    PropertyNotSet = sys::EVRApplicationError_VRApplicationError_PropertyNotSet,
    /// EVRApplicationError_VRApplicationError_UnknownProperty = 202.
    UnknownProperty = sys::EVRApplicationError_VRApplicationError_UnknownProperty,
    /// EVRApplicationError_VRApplicationError_InvalidParameter = 203.
    InvalidParameter = sys::EVRApplicationError_VRApplicationError_InvalidParameter,
}

impl Enum for ApplicationError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRApplicationError_VRApplicationError_None => Ok(ApplicationError::None),
             sys::EVRApplicationError_VRApplicationError_AppKeyAlreadyExists => Ok(ApplicationError::AppKeyAlreadyExist),
             sys::EVRApplicationError_VRApplicationError_NoManifest => Ok(ApplicationError::NoManifest),
             sys::EVRApplicationError_VRApplicationError_NoApplication => Ok(ApplicationError::NoApplication),
             sys::EVRApplicationError_VRApplicationError_InvalidIndex => Ok(ApplicationError::InvalidIndex),
             sys::EVRApplicationError_VRApplicationError_UnknownApplication => Ok(ApplicationError::UnknownApplication),
             sys::EVRApplicationError_VRApplicationError_IPCFailed => Ok(ApplicationError::Ipcfailed),
             sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyRunning => Ok(ApplicationError::ApplicationAlreadyRunning),
             sys::EVRApplicationError_VRApplicationError_InvalidManifest => Ok(ApplicationError::InvalidManifest),
             sys::EVRApplicationError_VRApplicationError_InvalidApplication => Ok(ApplicationError::InvalidApplication),
             sys::EVRApplicationError_VRApplicationError_LaunchFailed => Ok(ApplicationError::LaunchFailed),
             sys::EVRApplicationError_VRApplicationError_ApplicationAlreadyStarting => Ok(ApplicationError::ApplicationAlreadyStarting),
             sys::EVRApplicationError_VRApplicationError_LaunchInProgress => Ok(ApplicationError::LaunchInProgress),
             sys::EVRApplicationError_VRApplicationError_OldApplicationQuitting => Ok(ApplicationError::OldApplicationQuitting),
             sys::EVRApplicationError_VRApplicationError_TransitionAborted => Ok(ApplicationError::TransitionAborted),
             sys::EVRApplicationError_VRApplicationError_IsTemplate => Ok(ApplicationError::IsTemplate),
             sys::EVRApplicationError_VRApplicationError_SteamVRIsExiting => Ok(ApplicationError::SteamVRIsExiting),
             sys::EVRApplicationError_VRApplicationError_BufferTooSmall => Ok(ApplicationError::BufferTooSmall),
             sys::EVRApplicationError_VRApplicationError_PropertyNotSet => Ok(ApplicationError::PropertyNotSet),
             sys::EVRApplicationError_VRApplicationError_UnknownProperty => Ok(ApplicationError::UnknownProperty),
             sys::EVRApplicationError_VRApplicationError_InvalidParameter => Ok(ApplicationError::InvalidParameter),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ApplicationError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ApplicationError.", self.0)
    }
}

impl error::Error for Invalid<ApplicationError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ApplicationError."
    }
}

/// EVRApplicationProperty.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationProperty {
    /// EVRApplicationProperty_VRApplicationProperty_Name_String = 0.
    NameString = sys::EVRApplicationProperty_VRApplicationProperty_Name_String,
    /// EVRApplicationProperty_VRApplicationProperty_LaunchType_String = 11.
    LaunchTypeString = sys::EVRApplicationProperty_VRApplicationProperty_LaunchType_String,
    /// EVRApplicationProperty_VRApplicationProperty_WorkingDirectory_String = 12.
    WorkingDirectoryString = sys::EVRApplicationProperty_VRApplicationProperty_WorkingDirectory_String,
    /// EVRApplicationProperty_VRApplicationProperty_BinaryPath_String = 13.
    BinaryPathString = sys::EVRApplicationProperty_VRApplicationProperty_BinaryPath_String,
    /// EVRApplicationProperty_VRApplicationProperty_Arguments_String = 14.
    ArgumentsString = sys::EVRApplicationProperty_VRApplicationProperty_Arguments_String,
    /// EVRApplicationProperty_VRApplicationProperty_URL_String = 15.
    UrlString = sys::EVRApplicationProperty_VRApplicationProperty_URL_String,
    /// EVRApplicationProperty_VRApplicationProperty_Description_String = 50.
    DescriptionString = sys::EVRApplicationProperty_VRApplicationProperty_Description_String,
    /// EVRApplicationProperty_VRApplicationProperty_NewsURL_String = 51.
    NewsURLString = sys::EVRApplicationProperty_VRApplicationProperty_NewsURL_String,
    /// EVRApplicationProperty_VRApplicationProperty_ImagePath_String = 52.
    ImagePathString = sys::EVRApplicationProperty_VRApplicationProperty_ImagePath_String,
    /// EVRApplicationProperty_VRApplicationProperty_Source_String = 53.
    SourceString = sys::EVRApplicationProperty_VRApplicationProperty_Source_String,
    /// EVRApplicationProperty_VRApplicationProperty_ActionManifestURL_String = 54.
    ActionManifestURLString = sys::EVRApplicationProperty_VRApplicationProperty_ActionManifestURL_String,
    /// EVRApplicationProperty_VRApplicationProperty_IsDashboardOverlay_Bool = 60.
    IsDashboardOverlayBool = sys::EVRApplicationProperty_VRApplicationProperty_IsDashboardOverlay_Bool,
    /// EVRApplicationProperty_VRApplicationProperty_IsTemplate_Bool = 61.
    IsTemplateBool = sys::EVRApplicationProperty_VRApplicationProperty_IsTemplate_Bool,
    /// EVRApplicationProperty_VRApplicationProperty_IsInstanced_Bool = 62.
    IsInstancedBool = sys::EVRApplicationProperty_VRApplicationProperty_IsInstanced_Bool,
    /// EVRApplicationProperty_VRApplicationProperty_IsInternal_Bool = 63.
    IsInternalBool = sys::EVRApplicationProperty_VRApplicationProperty_IsInternal_Bool,
    /// EVRApplicationProperty_VRApplicationProperty_WantsCompositorPauseInStandby_Bool = 64.
    WantsCompositorPauseInStandbyBool = sys::EVRApplicationProperty_VRApplicationProperty_WantsCompositorPauseInStandby_Bool,
    /// EVRApplicationProperty_VRApplicationProperty_LastLaunchTime_Uint64 = 70.
    LastLaunchTimeUint64 = sys::EVRApplicationProperty_VRApplicationProperty_LastLaunchTime_Uint64,
}

impl Enum for ApplicationProperty {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRApplicationProperty_VRApplicationProperty_Name_String => Ok(ApplicationProperty::NameString),
             sys::EVRApplicationProperty_VRApplicationProperty_LaunchType_String => Ok(ApplicationProperty::LaunchTypeString),
             sys::EVRApplicationProperty_VRApplicationProperty_WorkingDirectory_String => Ok(ApplicationProperty::WorkingDirectoryString),
             sys::EVRApplicationProperty_VRApplicationProperty_BinaryPath_String => Ok(ApplicationProperty::BinaryPathString),
             sys::EVRApplicationProperty_VRApplicationProperty_Arguments_String => Ok(ApplicationProperty::ArgumentsString),
             sys::EVRApplicationProperty_VRApplicationProperty_URL_String => Ok(ApplicationProperty::UrlString),
             sys::EVRApplicationProperty_VRApplicationProperty_Description_String => Ok(ApplicationProperty::DescriptionString),
             sys::EVRApplicationProperty_VRApplicationProperty_NewsURL_String => Ok(ApplicationProperty::NewsURLString),
             sys::EVRApplicationProperty_VRApplicationProperty_ImagePath_String => Ok(ApplicationProperty::ImagePathString),
             sys::EVRApplicationProperty_VRApplicationProperty_Source_String => Ok(ApplicationProperty::SourceString),
             sys::EVRApplicationProperty_VRApplicationProperty_ActionManifestURL_String => Ok(ApplicationProperty::ActionManifestURLString),
             sys::EVRApplicationProperty_VRApplicationProperty_IsDashboardOverlay_Bool => Ok(ApplicationProperty::IsDashboardOverlayBool),
             sys::EVRApplicationProperty_VRApplicationProperty_IsTemplate_Bool => Ok(ApplicationProperty::IsTemplateBool),
             sys::EVRApplicationProperty_VRApplicationProperty_IsInstanced_Bool => Ok(ApplicationProperty::IsInstancedBool),
             sys::EVRApplicationProperty_VRApplicationProperty_IsInternal_Bool => Ok(ApplicationProperty::IsInternalBool),
             sys::EVRApplicationProperty_VRApplicationProperty_WantsCompositorPauseInStandby_Bool => Ok(ApplicationProperty::WantsCompositorPauseInStandbyBool),
             sys::EVRApplicationProperty_VRApplicationProperty_LastLaunchTime_Uint64 => Ok(ApplicationProperty::LastLaunchTimeUint64),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ApplicationProperty> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ApplicationProperty.", self.0)
    }
}

impl error::Error for Invalid<ApplicationProperty> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ApplicationProperty."
    }
}

/// EVRApplicationTransitionState.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ApplicationTransitionState {
    /// EVRApplicationTransitionState_VRApplicationTransition_None = 0.
    None = sys::EVRApplicationTransitionState_VRApplicationTransition_None,
    /// EVRApplicationTransitionState_VRApplicationTransition_OldAppQuitSent = 10.
    OldAppQuitSent = sys::EVRApplicationTransitionState_VRApplicationTransition_OldAppQuitSent,
    /// EVRApplicationTransitionState_VRApplicationTransition_WaitingForExternalLaunch = 11.
    WaitingForExternalLaunch = sys::EVRApplicationTransitionState_VRApplicationTransition_WaitingForExternalLaunch,
    /// EVRApplicationTransitionState_VRApplicationTransition_NewAppLaunched = 20.
    NewAppLaunched = sys::EVRApplicationTransitionState_VRApplicationTransition_NewAppLaunched,
}

impl Enum for ApplicationTransitionState {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRApplicationTransitionState_VRApplicationTransition_None => Ok(ApplicationTransitionState::None),
             sys::EVRApplicationTransitionState_VRApplicationTransition_OldAppQuitSent => Ok(ApplicationTransitionState::OldAppQuitSent),
             sys::EVRApplicationTransitionState_VRApplicationTransition_WaitingForExternalLaunch => Ok(ApplicationTransitionState::WaitingForExternalLaunch),
             sys::EVRApplicationTransitionState_VRApplicationTransition_NewAppLaunched => Ok(ApplicationTransitionState::NewAppLaunched),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ApplicationTransitionState> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ApplicationTransitionState.", self.0)
    }
}

impl error::Error for Invalid<ApplicationTransitionState> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ApplicationTransitionState."
    }
}

/// ChaperoneCalibrationState.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChaperoneCalibrationState {
    /// ChaperoneCalibrationState_OK = 1.
    Ok = sys::ChaperoneCalibrationState_OK,
    /// ChaperoneCalibrationState_Warning = 100.
    Warning = sys::ChaperoneCalibrationState_Warning,
    /// ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved = 101.
    WarningBaseStationMayHaveMoved = sys::ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved,
    /// ChaperoneCalibrationState_Warning_BaseStationRemoved = 102.
    WarningBaseStationRemoved = sys::ChaperoneCalibrationState_Warning_BaseStationRemoved,
    /// ChaperoneCalibrationState_Warning_SeatedBoundsInvalid = 103.
    WarningSeatedBoundsInvalid = sys::ChaperoneCalibrationState_Warning_SeatedBoundsInvalid,
    /// ChaperoneCalibrationState_Error = 200.
    Error = sys::ChaperoneCalibrationState_Error,
    /// ChaperoneCalibrationState_Error_BaseStationUninitialized = 201.
    ErrorBaseStationUninitialized = sys::ChaperoneCalibrationState_Error_BaseStationUninitialized,
    /// ChaperoneCalibrationState_Error_BaseStationConflict = 202.
    ErrorBaseStationConflict = sys::ChaperoneCalibrationState_Error_BaseStationConflict,
    /// ChaperoneCalibrationState_Error_PlayAreaInvalid = 203.
    ErrorPlayAreaInvalid = sys::ChaperoneCalibrationState_Error_PlayAreaInvalid,
    /// ChaperoneCalibrationState_Error_CollisionBoundsInvalid = 204.
    ErrorCollisionBoundsInvalid = sys::ChaperoneCalibrationState_Error_CollisionBoundsInvalid,
}

impl Enum for ChaperoneCalibrationState {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::ChaperoneCalibrationState_OK => Ok(ChaperoneCalibrationState::Ok),
             sys::ChaperoneCalibrationState_Warning => Ok(ChaperoneCalibrationState::Warning),
             sys::ChaperoneCalibrationState_Warning_BaseStationMayHaveMoved => Ok(ChaperoneCalibrationState::WarningBaseStationMayHaveMoved),
             sys::ChaperoneCalibrationState_Warning_BaseStationRemoved => Ok(ChaperoneCalibrationState::WarningBaseStationRemoved),
             sys::ChaperoneCalibrationState_Warning_SeatedBoundsInvalid => Ok(ChaperoneCalibrationState::WarningSeatedBoundsInvalid),
             sys::ChaperoneCalibrationState_Error => Ok(ChaperoneCalibrationState::Error),
             sys::ChaperoneCalibrationState_Error_BaseStationUninitialized => Ok(ChaperoneCalibrationState::ErrorBaseStationUninitialized),
             sys::ChaperoneCalibrationState_Error_BaseStationConflict => Ok(ChaperoneCalibrationState::ErrorBaseStationConflict),
             sys::ChaperoneCalibrationState_Error_PlayAreaInvalid => Ok(ChaperoneCalibrationState::ErrorPlayAreaInvalid),
             sys::ChaperoneCalibrationState_Error_CollisionBoundsInvalid => Ok(ChaperoneCalibrationState::ErrorCollisionBoundsInvalid),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ChaperoneCalibrationState> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ChaperoneCalibrationState.", self.0)
    }
}

impl error::Error for Invalid<ChaperoneCalibrationState> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ChaperoneCalibrationState."
    }
}

/// EChaperoneConfigFile.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChaperoneConfigFile {
    /// EChaperoneConfigFile_Live = 1.
    Live = sys::EChaperoneConfigFile_Live,
    /// EChaperoneConfigFile_Temp = 2.
    Temp = sys::EChaperoneConfigFile_Temp,
}

impl Enum for ChaperoneConfigFile {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EChaperoneConfigFile_Live => Ok(ChaperoneConfigFile::Live),
             sys::EChaperoneConfigFile_Temp => Ok(ChaperoneConfigFile::Temp),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ChaperoneConfigFile> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ChaperoneConfigFile.", self.0)
    }
}

impl error::Error for Invalid<ChaperoneConfigFile> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ChaperoneConfigFile."
    }
}

/// EChaperoneImportFlags.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ChaperoneImportFlag {
    /// EChaperoneImportFlags_EChaperoneImport_BoundsOnly = 1.
    BoundsOnly = sys::EChaperoneImportFlags_EChaperoneImport_BoundsOnly,
}

impl Enum for ChaperoneImportFlag {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EChaperoneImportFlags_EChaperoneImport_BoundsOnly => Ok(ChaperoneImportFlag::BoundsOnly),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ChaperoneImportFlag> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ChaperoneImportFlag.", self.0)
    }
}

impl error::Error for Invalid<ChaperoneImportFlag> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ChaperoneImportFlag."
    }
}

/// EVRCompositorError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CompositorError {
    /// EVRCompositorError_VRCompositorError_None = 0.
    None = sys::EVRCompositorError_VRCompositorError_None,
    /// EVRCompositorError_VRCompositorError_RequestFailed = 1.
    RequestFailed = sys::EVRCompositorError_VRCompositorError_RequestFailed,
    /// EVRCompositorError_VRCompositorError_IncompatibleVersion = 100.
    IncompatibleVersion = sys::EVRCompositorError_VRCompositorError_IncompatibleVersion,
    /// EVRCompositorError_VRCompositorError_DoNotHaveFocus = 101.
    DoNotHaveFocu = sys::EVRCompositorError_VRCompositorError_DoNotHaveFocus,
    /// EVRCompositorError_VRCompositorError_InvalidTexture = 102.
    InvalidTexture = sys::EVRCompositorError_VRCompositorError_InvalidTexture,
    /// EVRCompositorError_VRCompositorError_IsNotSceneApplication = 103.
    IsNotSceneApplication = sys::EVRCompositorError_VRCompositorError_IsNotSceneApplication,
    /// EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice = 104.
    TextureIsOnWrongDevice = sys::EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice,
    /// EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat = 105.
    TextureUsesUnsupportedFormat = sys::EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat,
    /// EVRCompositorError_VRCompositorError_SharedTexturesNotSupported = 106.
    SharedTexturesNotSupported = sys::EVRCompositorError_VRCompositorError_SharedTexturesNotSupported,
    /// EVRCompositorError_VRCompositorError_IndexOutOfRange = 107.
    IndexOutOfRange = sys::EVRCompositorError_VRCompositorError_IndexOutOfRange,
    /// EVRCompositorError_VRCompositorError_AlreadySubmitted = 108.
    AlreadySubmitted = sys::EVRCompositorError_VRCompositorError_AlreadySubmitted,
    /// EVRCompositorError_VRCompositorError_InvalidBounds = 109.
    InvalidBound = sys::EVRCompositorError_VRCompositorError_InvalidBounds,
}

impl Enum for CompositorError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRCompositorError_VRCompositorError_None => Ok(CompositorError::None),
             sys::EVRCompositorError_VRCompositorError_RequestFailed => Ok(CompositorError::RequestFailed),
             sys::EVRCompositorError_VRCompositorError_IncompatibleVersion => Ok(CompositorError::IncompatibleVersion),
             sys::EVRCompositorError_VRCompositorError_DoNotHaveFocus => Ok(CompositorError::DoNotHaveFocu),
             sys::EVRCompositorError_VRCompositorError_InvalidTexture => Ok(CompositorError::InvalidTexture),
             sys::EVRCompositorError_VRCompositorError_IsNotSceneApplication => Ok(CompositorError::IsNotSceneApplication),
             sys::EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice => Ok(CompositorError::TextureIsOnWrongDevice),
             sys::EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat => Ok(CompositorError::TextureUsesUnsupportedFormat),
             sys::EVRCompositorError_VRCompositorError_SharedTexturesNotSupported => Ok(CompositorError::SharedTexturesNotSupported),
             sys::EVRCompositorError_VRCompositorError_IndexOutOfRange => Ok(CompositorError::IndexOutOfRange),
             sys::EVRCompositorError_VRCompositorError_AlreadySubmitted => Ok(CompositorError::AlreadySubmitted),
             sys::EVRCompositorError_VRCompositorError_InvalidBounds => Ok(CompositorError::InvalidBound),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<CompositorError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of CompositorError.", self.0)
    }
}

impl error::Error for Invalid<CompositorError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of CompositorError."
    }
}

/// EVRCompositorTimingMode.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CompositorTimingMode {
    /// EVRCompositorTimingMode_VRCompositorTimingMode_Implicit = 0.
    Implicit = sys::EVRCompositorTimingMode_VRCompositorTimingMode_Implicit,
    /// EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_RuntimePerformsPostPresentHandoff = 1.
    ExplicitRuntimePerformsPostPresentHandoff = sys::EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_RuntimePerformsPostPresentHandoff,
    /// EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_ApplicationPerformsPostPresentHandoff = 2.
    ExplicitApplicationPerformsPostPresentHandoff = sys::EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_ApplicationPerformsPostPresentHandoff,
}

impl Enum for CompositorTimingMode {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRCompositorTimingMode_VRCompositorTimingMode_Implicit => Ok(CompositorTimingMode::Implicit),
             sys::EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_RuntimePerformsPostPresentHandoff => Ok(CompositorTimingMode::ExplicitRuntimePerformsPostPresentHandoff),
             sys::EVRCompositorTimingMode_VRCompositorTimingMode_Explicit_ApplicationPerformsPostPresentHandoff => Ok(CompositorTimingMode::ExplicitApplicationPerformsPostPresentHandoff),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<CompositorTimingMode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of CompositorTimingMode.", self.0)
    }
}

impl error::Error for Invalid<CompositorTimingMode> {
    fn description(&self) -> &str {
        "Value does not represent any variant of CompositorTimingMode."
    }
}

/// VROverlayInputMethod.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayInputMethod {
    /// VROverlayInputMethod_None = 0.
    None = sys::VROverlayInputMethod_None,
    /// VROverlayInputMethod_Mouse = 1.
    Mouse = sys::VROverlayInputMethod_Mouse,
    /// VROverlayInputMethod_DualAnalog = 2.
    DualAnalog = sys::VROverlayInputMethod_DualAnalog,
}

impl Enum for OverlayInputMethod {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::VROverlayInputMethod_None => Ok(OverlayInputMethod::None),
             sys::VROverlayInputMethod_Mouse => Ok(OverlayInputMethod::Mouse),
             sys::VROverlayInputMethod_DualAnalog => Ok(OverlayInputMethod::DualAnalog),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<OverlayInputMethod> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of OverlayInputMethod.", self.0)
    }
}

impl error::Error for Invalid<OverlayInputMethod> {
    fn description(&self) -> &str {
        "Value does not represent any variant of OverlayInputMethod."
    }
}

/// VROverlayTransformType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayTransformType {
    /// VROverlayTransformType_VROverlayTransform_Absolute = 0.
    Absolute = sys::VROverlayTransformType_VROverlayTransform_Absolute,
    /// VROverlayTransformType_VROverlayTransform_TrackedDeviceRelative = 1.
    TrackedDeviceRelative = sys::VROverlayTransformType_VROverlayTransform_TrackedDeviceRelative,
    /// VROverlayTransformType_VROverlayTransform_SystemOverlay = 2.
    SystemOverlay = sys::VROverlayTransformType_VROverlayTransform_SystemOverlay,
    /// VROverlayTransformType_VROverlayTransform_TrackedComponent = 3.
    TrackedComponent = sys::VROverlayTransformType_VROverlayTransform_TrackedComponent,
}

impl Enum for OverlayTransformType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::VROverlayTransformType_VROverlayTransform_Absolute => Ok(OverlayTransformType::Absolute),
             sys::VROverlayTransformType_VROverlayTransform_TrackedDeviceRelative => Ok(OverlayTransformType::TrackedDeviceRelative),
             sys::VROverlayTransformType_VROverlayTransform_SystemOverlay => Ok(OverlayTransformType::SystemOverlay),
             sys::VROverlayTransformType_VROverlayTransform_TrackedComponent => Ok(OverlayTransformType::TrackedComponent),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<OverlayTransformType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of OverlayTransformType.", self.0)
    }
}

impl error::Error for Invalid<OverlayTransformType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of OverlayTransformType."
    }
}

/// VROverlayFlags.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayFlag {
    /// VROverlayFlags_None = 0.
    None = sys::VROverlayFlags_None,
    /// VROverlayFlags_Curved = 1.
    Curved = sys::VROverlayFlags_Curved,
    /// VROverlayFlags_RGSS4X = 2.
    Rgss4X = sys::VROverlayFlags_RGSS4X,
    /// VROverlayFlags_NoDashboardTab = 3.
    NoDashboardTab = sys::VROverlayFlags_NoDashboardTab,
    /// VROverlayFlags_AcceptsGamepadEvents = 4.
    AcceptsGamepadEvent = sys::VROverlayFlags_AcceptsGamepadEvents,
    /// VROverlayFlags_ShowGamepadFocus = 5.
    ShowGamepadFocu = sys::VROverlayFlags_ShowGamepadFocus,
    /// VROverlayFlags_SendVRScrollEvents = 6.
    SendVRScrollEvent = sys::VROverlayFlags_SendVRScrollEvents,
    /// VROverlayFlags_SendVRTouchpadEvents = 7.
    SendVRTouchpadEvent = sys::VROverlayFlags_SendVRTouchpadEvents,
    /// VROverlayFlags_ShowTouchPadScrollWheel = 8.
    ShowTouchPadScrollWheel = sys::VROverlayFlags_ShowTouchPadScrollWheel,
    /// VROverlayFlags_TransferOwnershipToInternalProcess = 9.
    TransferOwnershipToInternalProcess = sys::VROverlayFlags_TransferOwnershipToInternalProcess,
    /// VROverlayFlags_SideBySide_Parallel = 10.
    SideBySideParallel = sys::VROverlayFlags_SideBySide_Parallel,
    /// VROverlayFlags_SideBySide_Crossed = 11.
    SideBySideCrossed = sys::VROverlayFlags_SideBySide_Crossed,
    /// VROverlayFlags_Panorama = 12.
    Panorama = sys::VROverlayFlags_Panorama,
    /// VROverlayFlags_StereoPanorama = 13.
    StereoPanorama = sys::VROverlayFlags_StereoPanorama,
    /// VROverlayFlags_SortWithNonSceneOverlays = 14.
    SortWithNonSceneOverlay = sys::VROverlayFlags_SortWithNonSceneOverlays,
    /// VROverlayFlags_VisibleInDashboard = 15.
    VisibleInDashboard = sys::VROverlayFlags_VisibleInDashboard,
    /// VROverlayFlags_MakeOverlaysInteractiveIfVisible = 16.
    MakeOverlaysInteractiveIfVisible = sys::VROverlayFlags_MakeOverlaysInteractiveIfVisible,
}

impl Enum for OverlayFlag {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::VROverlayFlags_None => Ok(OverlayFlag::None),
             sys::VROverlayFlags_Curved => Ok(OverlayFlag::Curved),
             sys::VROverlayFlags_RGSS4X => Ok(OverlayFlag::Rgss4X),
             sys::VROverlayFlags_NoDashboardTab => Ok(OverlayFlag::NoDashboardTab),
             sys::VROverlayFlags_AcceptsGamepadEvents => Ok(OverlayFlag::AcceptsGamepadEvent),
             sys::VROverlayFlags_ShowGamepadFocus => Ok(OverlayFlag::ShowGamepadFocu),
             sys::VROverlayFlags_SendVRScrollEvents => Ok(OverlayFlag::SendVRScrollEvent),
             sys::VROverlayFlags_SendVRTouchpadEvents => Ok(OverlayFlag::SendVRTouchpadEvent),
             sys::VROverlayFlags_ShowTouchPadScrollWheel => Ok(OverlayFlag::ShowTouchPadScrollWheel),
             sys::VROverlayFlags_TransferOwnershipToInternalProcess => Ok(OverlayFlag::TransferOwnershipToInternalProcess),
             sys::VROverlayFlags_SideBySide_Parallel => Ok(OverlayFlag::SideBySideParallel),
             sys::VROverlayFlags_SideBySide_Crossed => Ok(OverlayFlag::SideBySideCrossed),
             sys::VROverlayFlags_Panorama => Ok(OverlayFlag::Panorama),
             sys::VROverlayFlags_StereoPanorama => Ok(OverlayFlag::StereoPanorama),
             sys::VROverlayFlags_SortWithNonSceneOverlays => Ok(OverlayFlag::SortWithNonSceneOverlay),
             sys::VROverlayFlags_VisibleInDashboard => Ok(OverlayFlag::VisibleInDashboard),
             sys::VROverlayFlags_MakeOverlaysInteractiveIfVisible => Ok(OverlayFlag::MakeOverlaysInteractiveIfVisible),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<OverlayFlag> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of OverlayFlag.", self.0)
    }
}

impl error::Error for Invalid<OverlayFlag> {
    fn description(&self) -> &str {
        "Value does not represent any variant of OverlayFlag."
    }
}

/// VRMessageOverlayResponse.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MessageOverlayResponse {
    /// VRMessageOverlayResponse_ButtonPress_0 = 0.
    ButtonPress0 = sys::VRMessageOverlayResponse_ButtonPress_0,
    /// VRMessageOverlayResponse_ButtonPress_1 = 1.
    ButtonPress1 = sys::VRMessageOverlayResponse_ButtonPress_1,
    /// VRMessageOverlayResponse_ButtonPress_2 = 2.
    ButtonPress2 = sys::VRMessageOverlayResponse_ButtonPress_2,
    /// VRMessageOverlayResponse_ButtonPress_3 = 3.
    ButtonPress3 = sys::VRMessageOverlayResponse_ButtonPress_3,
    /// VRMessageOverlayResponse_CouldntFindSystemOverlay = 4.
    CouldntFindSystemOverlay = sys::VRMessageOverlayResponse_CouldntFindSystemOverlay,
    /// VRMessageOverlayResponse_CouldntFindOrCreateClientOverlay = 5.
    CouldntFindOrCreateClientOverlay = sys::VRMessageOverlayResponse_CouldntFindOrCreateClientOverlay,
    /// VRMessageOverlayResponse_ApplicationQuit = 6.
    ApplicationQuit = sys::VRMessageOverlayResponse_ApplicationQuit,
}

impl Enum for MessageOverlayResponse {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::VRMessageOverlayResponse_ButtonPress_0 => Ok(MessageOverlayResponse::ButtonPress0),
             sys::VRMessageOverlayResponse_ButtonPress_1 => Ok(MessageOverlayResponse::ButtonPress1),
             sys::VRMessageOverlayResponse_ButtonPress_2 => Ok(MessageOverlayResponse::ButtonPress2),
             sys::VRMessageOverlayResponse_ButtonPress_3 => Ok(MessageOverlayResponse::ButtonPress3),
             sys::VRMessageOverlayResponse_CouldntFindSystemOverlay => Ok(MessageOverlayResponse::CouldntFindSystemOverlay),
             sys::VRMessageOverlayResponse_CouldntFindOrCreateClientOverlay => Ok(MessageOverlayResponse::CouldntFindOrCreateClientOverlay),
             sys::VRMessageOverlayResponse_ApplicationQuit => Ok(MessageOverlayResponse::ApplicationQuit),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<MessageOverlayResponse> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of MessageOverlayResponse.", self.0)
    }
}

impl error::Error for Invalid<MessageOverlayResponse> {
    fn description(&self) -> &str {
        "Value does not represent any variant of MessageOverlayResponse."
    }
}

/// EGamepadTextInputMode.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GamepadTextInputMode {
    /// EGamepadTextInputMode_k_EGamepadTextInputModeNormal = 0.
    EgamepadTextInputModeNormal = sys::EGamepadTextInputMode_k_EGamepadTextInputModeNormal,
    /// EGamepadTextInputMode_k_EGamepadTextInputModePassword = 1.
    EgamepadTextInputModePassword = sys::EGamepadTextInputMode_k_EGamepadTextInputModePassword,
    /// EGamepadTextInputMode_k_EGamepadTextInputModeSubmit = 2.
    EgamepadTextInputModeSubmit = sys::EGamepadTextInputMode_k_EGamepadTextInputModeSubmit,
}

impl Enum for GamepadTextInputMode {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EGamepadTextInputMode_k_EGamepadTextInputModeNormal => Ok(GamepadTextInputMode::EgamepadTextInputModeNormal),
             sys::EGamepadTextInputMode_k_EGamepadTextInputModePassword => Ok(GamepadTextInputMode::EgamepadTextInputModePassword),
             sys::EGamepadTextInputMode_k_EGamepadTextInputModeSubmit => Ok(GamepadTextInputMode::EgamepadTextInputModeSubmit),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<GamepadTextInputMode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of GamepadTextInputMode.", self.0)
    }
}

impl error::Error for Invalid<GamepadTextInputMode> {
    fn description(&self) -> &str {
        "Value does not represent any variant of GamepadTextInputMode."
    }
}

/// EGamepadTextInputLineMode.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GamepadTextInputLineMode {
    /// EGamepadTextInputLineMode_k_EGamepadTextInputLineModeSingleLine = 0.
    EgamepadTextInputLineModeSingleLine = sys::EGamepadTextInputLineMode_k_EGamepadTextInputLineModeSingleLine,
    /// EGamepadTextInputLineMode_k_EGamepadTextInputLineModeMultipleLines = 1.
    EgamepadTextInputLineModeMultipleLine = sys::EGamepadTextInputLineMode_k_EGamepadTextInputLineModeMultipleLines,
}

impl Enum for GamepadTextInputLineMode {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EGamepadTextInputLineMode_k_EGamepadTextInputLineModeSingleLine => Ok(GamepadTextInputLineMode::EgamepadTextInputLineModeSingleLine),
             sys::EGamepadTextInputLineMode_k_EGamepadTextInputLineModeMultipleLines => Ok(GamepadTextInputLineMode::EgamepadTextInputLineModeMultipleLine),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<GamepadTextInputLineMode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of GamepadTextInputLineMode.", self.0)
    }
}

impl error::Error for Invalid<GamepadTextInputLineMode> {
    fn description(&self) -> &str {
        "Value does not represent any variant of GamepadTextInputLineMode."
    }
}

/// EOverlayDirection.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayDirection {
    /// EOverlayDirection_OverlayDirection_Up = 0.
    Up = sys::EOverlayDirection_OverlayDirection_Up,
    /// EOverlayDirection_OverlayDirection_Down = 1.
    Down = sys::EOverlayDirection_OverlayDirection_Down,
    /// EOverlayDirection_OverlayDirection_Left = 2.
    Left = sys::EOverlayDirection_OverlayDirection_Left,
    /// EOverlayDirection_OverlayDirection_Right = 3.
    Right = sys::EOverlayDirection_OverlayDirection_Right,
    /// EOverlayDirection_OverlayDirection_Count = 4.
    Count = sys::EOverlayDirection_OverlayDirection_Count,
}

impl Enum for OverlayDirection {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EOverlayDirection_OverlayDirection_Up => Ok(OverlayDirection::Up),
             sys::EOverlayDirection_OverlayDirection_Down => Ok(OverlayDirection::Down),
             sys::EOverlayDirection_OverlayDirection_Left => Ok(OverlayDirection::Left),
             sys::EOverlayDirection_OverlayDirection_Right => Ok(OverlayDirection::Right),
             sys::EOverlayDirection_OverlayDirection_Count => Ok(OverlayDirection::Count),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<OverlayDirection> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of OverlayDirection.", self.0)
    }
}

impl error::Error for Invalid<OverlayDirection> {
    fn description(&self) -> &str {
        "Value does not represent any variant of OverlayDirection."
    }
}

/// EVROverlayIntersectionMaskPrimitiveType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OverlayIntersectionMaskPrimitiveType {
    /// EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Rectangle = 0.
    Rectangle = sys::EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Rectangle,
    /// EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Circle = 1.
    Circle = sys::EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Circle,
}

impl Enum for OverlayIntersectionMaskPrimitiveType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Rectangle => Ok(OverlayIntersectionMaskPrimitiveType::Rectangle),
             sys::EVROverlayIntersectionMaskPrimitiveType_OverlayIntersectionPrimitiveType_Circle => Ok(OverlayIntersectionMaskPrimitiveType::Circle),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<OverlayIntersectionMaskPrimitiveType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of OverlayIntersectionMaskPrimitiveType.", self.0)
    }
}

impl error::Error for Invalid<OverlayIntersectionMaskPrimitiveType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of OverlayIntersectionMaskPrimitiveType."
    }
}

/// EVRRenderModelError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum RenderModelError {
    /// EVRRenderModelError_VRRenderModelError_None = 0.
    None = sys::EVRRenderModelError_VRRenderModelError_None,
    /// EVRRenderModelError_VRRenderModelError_Loading = 100.
    Loading = sys::EVRRenderModelError_VRRenderModelError_Loading,
    /// EVRRenderModelError_VRRenderModelError_NotSupported = 200.
    NotSupported = sys::EVRRenderModelError_VRRenderModelError_NotSupported,
    /// EVRRenderModelError_VRRenderModelError_InvalidArg = 300.
    InvalidArg = sys::EVRRenderModelError_VRRenderModelError_InvalidArg,
    /// EVRRenderModelError_VRRenderModelError_InvalidModel = 301.
    InvalidModel = sys::EVRRenderModelError_VRRenderModelError_InvalidModel,
    /// EVRRenderModelError_VRRenderModelError_NoShapes = 302.
    NoShape = sys::EVRRenderModelError_VRRenderModelError_NoShapes,
    /// EVRRenderModelError_VRRenderModelError_MultipleShapes = 303.
    MultipleShape = sys::EVRRenderModelError_VRRenderModelError_MultipleShapes,
    /// EVRRenderModelError_VRRenderModelError_TooManyVertices = 304.
    TooManyVertice = sys::EVRRenderModelError_VRRenderModelError_TooManyVertices,
    /// EVRRenderModelError_VRRenderModelError_MultipleTextures = 305.
    MultipleTexture = sys::EVRRenderModelError_VRRenderModelError_MultipleTextures,
    /// EVRRenderModelError_VRRenderModelError_BufferTooSmall = 306.
    BufferTooSmall = sys::EVRRenderModelError_VRRenderModelError_BufferTooSmall,
    /// EVRRenderModelError_VRRenderModelError_NotEnoughNormals = 307.
    NotEnoughNormal = sys::EVRRenderModelError_VRRenderModelError_NotEnoughNormals,
    /// EVRRenderModelError_VRRenderModelError_NotEnoughTexCoords = 308.
    NotEnoughTexCoord = sys::EVRRenderModelError_VRRenderModelError_NotEnoughTexCoords,
    /// EVRRenderModelError_VRRenderModelError_InvalidTexture = 400.
    InvalidTexture = sys::EVRRenderModelError_VRRenderModelError_InvalidTexture,
}

impl Enum for RenderModelError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRRenderModelError_VRRenderModelError_None => Ok(RenderModelError::None),
             sys::EVRRenderModelError_VRRenderModelError_Loading => Ok(RenderModelError::Loading),
             sys::EVRRenderModelError_VRRenderModelError_NotSupported => Ok(RenderModelError::NotSupported),
             sys::EVRRenderModelError_VRRenderModelError_InvalidArg => Ok(RenderModelError::InvalidArg),
             sys::EVRRenderModelError_VRRenderModelError_InvalidModel => Ok(RenderModelError::InvalidModel),
             sys::EVRRenderModelError_VRRenderModelError_NoShapes => Ok(RenderModelError::NoShape),
             sys::EVRRenderModelError_VRRenderModelError_MultipleShapes => Ok(RenderModelError::MultipleShape),
             sys::EVRRenderModelError_VRRenderModelError_TooManyVertices => Ok(RenderModelError::TooManyVertice),
             sys::EVRRenderModelError_VRRenderModelError_MultipleTextures => Ok(RenderModelError::MultipleTexture),
             sys::EVRRenderModelError_VRRenderModelError_BufferTooSmall => Ok(RenderModelError::BufferTooSmall),
             sys::EVRRenderModelError_VRRenderModelError_NotEnoughNormals => Ok(RenderModelError::NotEnoughNormal),
             sys::EVRRenderModelError_VRRenderModelError_NotEnoughTexCoords => Ok(RenderModelError::NotEnoughTexCoord),
             sys::EVRRenderModelError_VRRenderModelError_InvalidTexture => Ok(RenderModelError::InvalidTexture),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<RenderModelError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of RenderModelError.", self.0)
    }
}

impl error::Error for Invalid<RenderModelError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of RenderModelError."
    }
}

/// EVRComponentProperty.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ComponentProperty {
    /// EVRComponentProperty_VRComponentProperty_IsStatic = 1.
    IsStatic = sys::EVRComponentProperty_VRComponentProperty_IsStatic,
    /// EVRComponentProperty_VRComponentProperty_IsVisible = 2.
    IsVisible = sys::EVRComponentProperty_VRComponentProperty_IsVisible,
    /// EVRComponentProperty_VRComponentProperty_IsTouched = 4.
    IsTouched = sys::EVRComponentProperty_VRComponentProperty_IsTouched,
    /// EVRComponentProperty_VRComponentProperty_IsPressed = 8.
    IsPressed = sys::EVRComponentProperty_VRComponentProperty_IsPressed,
    /// EVRComponentProperty_VRComponentProperty_IsScrolled = 16.
    IsScrolled = sys::EVRComponentProperty_VRComponentProperty_IsScrolled,
}

impl Enum for ComponentProperty {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRComponentProperty_VRComponentProperty_IsStatic => Ok(ComponentProperty::IsStatic),
             sys::EVRComponentProperty_VRComponentProperty_IsVisible => Ok(ComponentProperty::IsVisible),
             sys::EVRComponentProperty_VRComponentProperty_IsTouched => Ok(ComponentProperty::IsTouched),
             sys::EVRComponentProperty_VRComponentProperty_IsPressed => Ok(ComponentProperty::IsPressed),
             sys::EVRComponentProperty_VRComponentProperty_IsScrolled => Ok(ComponentProperty::IsScrolled),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ComponentProperty> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ComponentProperty.", self.0)
    }
}

impl error::Error for Invalid<ComponentProperty> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ComponentProperty."
    }
}

/// EVRNotificationType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NotificationType {
    /// EVRNotificationType_Transient = 0.
    Transient = sys::EVRNotificationType_Transient,
    /// EVRNotificationType_Persistent = 1.
    Persistent = sys::EVRNotificationType_Persistent,
    /// EVRNotificationType_Transient_SystemWithUserValue = 2.
    TransientSystemWithUserValue = sys::EVRNotificationType_Transient_SystemWithUserValue,
}

impl Enum for NotificationType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRNotificationType_Transient => Ok(NotificationType::Transient),
             sys::EVRNotificationType_Persistent => Ok(NotificationType::Persistent),
             sys::EVRNotificationType_Transient_SystemWithUserValue => Ok(NotificationType::TransientSystemWithUserValue),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<NotificationType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of NotificationType.", self.0)
    }
}

impl error::Error for Invalid<NotificationType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of NotificationType."
    }
}

/// EVRNotificationStyle.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NotificationStyle {
    /// EVRNotificationStyle_None = 0.
    None = sys::EVRNotificationStyle_None,
    /// EVRNotificationStyle_Application = 100.
    Application = sys::EVRNotificationStyle_Application,
    /// EVRNotificationStyle_Contact_Disabled = 200.
    ContactDisabled = sys::EVRNotificationStyle_Contact_Disabled,
    /// EVRNotificationStyle_Contact_Enabled = 201.
    ContactEnabled = sys::EVRNotificationStyle_Contact_Enabled,
    /// EVRNotificationStyle_Contact_Active = 202.
    ContactActive = sys::EVRNotificationStyle_Contact_Active,
}

impl Enum for NotificationStyle {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRNotificationStyle_None => Ok(NotificationStyle::None),
             sys::EVRNotificationStyle_Application => Ok(NotificationStyle::Application),
             sys::EVRNotificationStyle_Contact_Disabled => Ok(NotificationStyle::ContactDisabled),
             sys::EVRNotificationStyle_Contact_Enabled => Ok(NotificationStyle::ContactEnabled),
             sys::EVRNotificationStyle_Contact_Active => Ok(NotificationStyle::ContactActive),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<NotificationStyle> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of NotificationStyle.", self.0)
    }
}

impl error::Error for Invalid<NotificationStyle> {
    fn description(&self) -> &str {
        "Value does not represent any variant of NotificationStyle."
    }
}

/// EVRSettingsError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SettingsError {
    /// EVRSettingsError_VRSettingsError_None = 0.
    None = sys::EVRSettingsError_VRSettingsError_None,
    /// EVRSettingsError_VRSettingsError_IPCFailed = 1.
    Ipcfailed = sys::EVRSettingsError_VRSettingsError_IPCFailed,
    /// EVRSettingsError_VRSettingsError_WriteFailed = 2.
    WriteFailed = sys::EVRSettingsError_VRSettingsError_WriteFailed,
    /// EVRSettingsError_VRSettingsError_ReadFailed = 3.
    ReadFailed = sys::EVRSettingsError_VRSettingsError_ReadFailed,
    /// EVRSettingsError_VRSettingsError_JsonParseFailed = 4.
    JsonParseFailed = sys::EVRSettingsError_VRSettingsError_JsonParseFailed,
    /// EVRSettingsError_VRSettingsError_UnsetSettingHasNoDefault = 5.
    UnsetSettingHasNoDefault = sys::EVRSettingsError_VRSettingsError_UnsetSettingHasNoDefault,
}

impl Enum for SettingsError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRSettingsError_VRSettingsError_None => Ok(SettingsError::None),
             sys::EVRSettingsError_VRSettingsError_IPCFailed => Ok(SettingsError::Ipcfailed),
             sys::EVRSettingsError_VRSettingsError_WriteFailed => Ok(SettingsError::WriteFailed),
             sys::EVRSettingsError_VRSettingsError_ReadFailed => Ok(SettingsError::ReadFailed),
             sys::EVRSettingsError_VRSettingsError_JsonParseFailed => Ok(SettingsError::JsonParseFailed),
             sys::EVRSettingsError_VRSettingsError_UnsetSettingHasNoDefault => Ok(SettingsError::UnsetSettingHasNoDefault),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<SettingsError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of SettingsError.", self.0)
    }
}

impl error::Error for Invalid<SettingsError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of SettingsError."
    }
}

/// EVRScreenshotError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ScreenshotError {
    /// EVRScreenshotError_VRScreenshotError_None = 0.
    None = sys::EVRScreenshotError_VRScreenshotError_None,
    /// EVRScreenshotError_VRScreenshotError_RequestFailed = 1.
    RequestFailed = sys::EVRScreenshotError_VRScreenshotError_RequestFailed,
    /// EVRScreenshotError_VRScreenshotError_IncompatibleVersion = 100.
    IncompatibleVersion = sys::EVRScreenshotError_VRScreenshotError_IncompatibleVersion,
    /// EVRScreenshotError_VRScreenshotError_NotFound = 101.
    NotFound = sys::EVRScreenshotError_VRScreenshotError_NotFound,
    /// EVRScreenshotError_VRScreenshotError_BufferTooSmall = 102.
    BufferTooSmall = sys::EVRScreenshotError_VRScreenshotError_BufferTooSmall,
    /// EVRScreenshotError_VRScreenshotError_ScreenshotAlreadyInProgress = 108.
    ScreenshotAlreadyInProgress = sys::EVRScreenshotError_VRScreenshotError_ScreenshotAlreadyInProgress,
}

impl Enum for ScreenshotError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRScreenshotError_VRScreenshotError_None => Ok(ScreenshotError::None),
             sys::EVRScreenshotError_VRScreenshotError_RequestFailed => Ok(ScreenshotError::RequestFailed),
             sys::EVRScreenshotError_VRScreenshotError_IncompatibleVersion => Ok(ScreenshotError::IncompatibleVersion),
             sys::EVRScreenshotError_VRScreenshotError_NotFound => Ok(ScreenshotError::NotFound),
             sys::EVRScreenshotError_VRScreenshotError_BufferTooSmall => Ok(ScreenshotError::BufferTooSmall),
             sys::EVRScreenshotError_VRScreenshotError_ScreenshotAlreadyInProgress => Ok(ScreenshotError::ScreenshotAlreadyInProgress),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<ScreenshotError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of ScreenshotError.", self.0)
    }
}

impl error::Error for Invalid<ScreenshotError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of ScreenshotError."
    }
}

/// EVRSkeletalTransformSpace.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalTransformSpace {
    /// EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Model = 0.
    Model = sys::EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Model,
    /// EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Parent = 1.
    Parent = sys::EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Parent,
}

impl Enum for SkeletalTransformSpace {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Model => Ok(SkeletalTransformSpace::Model),
             sys::EVRSkeletalTransformSpace_VRSkeletalTransformSpace_Parent => Ok(SkeletalTransformSpace::Parent),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<SkeletalTransformSpace> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of SkeletalTransformSpace.", self.0)
    }
}

impl error::Error for Invalid<SkeletalTransformSpace> {
    fn description(&self) -> &str {
        "Value does not represent any variant of SkeletalTransformSpace."
    }
}

/// EVRSkeletalReferencePose.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SkeletalReferencePose {
    /// EVRSkeletalReferencePose_VRSkeletalReferencePose_BindPose = 0.
    BindPose = sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_BindPose,
    /// EVRSkeletalReferencePose_VRSkeletalReferencePose_OpenHand = 1.
    OpenHand = sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_OpenHand,
    /// EVRSkeletalReferencePose_VRSkeletalReferencePose_Fist = 2.
    Fist = sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_Fist,
    /// EVRSkeletalReferencePose_VRSkeletalReferencePose_GripLimit = 3.
    GripLimit = sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_GripLimit,
}

impl Enum for SkeletalReferencePose {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_BindPose => Ok(SkeletalReferencePose::BindPose),
             sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_OpenHand => Ok(SkeletalReferencePose::OpenHand),
             sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_Fist => Ok(SkeletalReferencePose::Fist),
             sys::EVRSkeletalReferencePose_VRSkeletalReferencePose_GripLimit => Ok(SkeletalReferencePose::GripLimit),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<SkeletalReferencePose> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of SkeletalReferencePose.", self.0)
    }
}

impl error::Error for Invalid<SkeletalReferencePose> {
    fn description(&self) -> &str {
        "Value does not represent any variant of SkeletalReferencePose."
    }
}

/// EVRFinger.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Finger {
    /// EVRFinger_VRFinger_Thumb = 0.
    Thumb = sys::EVRFinger_VRFinger_Thumb,
    /// EVRFinger_VRFinger_Index = 1.
    Index = sys::EVRFinger_VRFinger_Index,
    /// EVRFinger_VRFinger_Middle = 2.
    Middle = sys::EVRFinger_VRFinger_Middle,
    /// EVRFinger_VRFinger_Ring = 3.
    Ring = sys::EVRFinger_VRFinger_Ring,
    /// EVRFinger_VRFinger_Pinky = 4.
    Pinky = sys::EVRFinger_VRFinger_Pinky,
    /// EVRFinger_VRFinger_Count = 5.
    Count = sys::EVRFinger_VRFinger_Count,
}

impl Enum for Finger {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRFinger_VRFinger_Thumb => Ok(Finger::Thumb),
             sys::EVRFinger_VRFinger_Index => Ok(Finger::Index),
             sys::EVRFinger_VRFinger_Middle => Ok(Finger::Middle),
             sys::EVRFinger_VRFinger_Ring => Ok(Finger::Ring),
             sys::EVRFinger_VRFinger_Pinky => Ok(Finger::Pinky),
             sys::EVRFinger_VRFinger_Count => Ok(Finger::Count),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<Finger> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of Finger.", self.0)
    }
}

impl error::Error for Invalid<Finger> {
    fn description(&self) -> &str {
        "Value does not represent any variant of Finger."
    }
}

/// EVRFingerSplay.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FingerSplay {
    /// EVRFingerSplay_VRFingerSplay_Thumb_Index = 0.
    ThumbIndex = sys::EVRFingerSplay_VRFingerSplay_Thumb_Index,
    /// EVRFingerSplay_VRFingerSplay_Index_Middle = 1.
    IndexMiddle = sys::EVRFingerSplay_VRFingerSplay_Index_Middle,
    /// EVRFingerSplay_VRFingerSplay_Middle_Ring = 2.
    MiddleRing = sys::EVRFingerSplay_VRFingerSplay_Middle_Ring,
    /// EVRFingerSplay_VRFingerSplay_Ring_Pinky = 3.
    RingPinky = sys::EVRFingerSplay_VRFingerSplay_Ring_Pinky,
    /// EVRFingerSplay_VRFingerSplay_Count = 4.
    Count = sys::EVRFingerSplay_VRFingerSplay_Count,
}

impl Enum for FingerSplay {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRFingerSplay_VRFingerSplay_Thumb_Index => Ok(FingerSplay::ThumbIndex),
             sys::EVRFingerSplay_VRFingerSplay_Index_Middle => Ok(FingerSplay::IndexMiddle),
             sys::EVRFingerSplay_VRFingerSplay_Middle_Ring => Ok(FingerSplay::MiddleRing),
             sys::EVRFingerSplay_VRFingerSplay_Ring_Pinky => Ok(FingerSplay::RingPinky),
             sys::EVRFingerSplay_VRFingerSplay_Count => Ok(FingerSplay::Count),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<FingerSplay> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of FingerSplay.", self.0)
    }
}

impl error::Error for Invalid<FingerSplay> {
    fn description(&self) -> &str {
        "Value does not represent any variant of FingerSplay."
    }
}

/// EVRInputFilterCancelType.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InputFilterCancelType {
    /// EVRInputFilterCancelType_VRInputFilterCancel_Timers = 0.
    Timer = sys::EVRInputFilterCancelType_VRInputFilterCancel_Timers,
    /// EVRInputFilterCancelType_VRInputFilterCancel_Momentum = 1.
    Momentum = sys::EVRInputFilterCancelType_VRInputFilterCancel_Momentum,
}

impl Enum for InputFilterCancelType {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRInputFilterCancelType_VRInputFilterCancel_Timers => Ok(InputFilterCancelType::Timer),
             sys::EVRInputFilterCancelType_VRInputFilterCancel_Momentum => Ok(InputFilterCancelType::Momentum),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<InputFilterCancelType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of InputFilterCancelType.", self.0)
    }
}

impl error::Error for Invalid<InputFilterCancelType> {
    fn description(&self) -> &str {
        "Value does not represent any variant of InputFilterCancelType."
    }
}

/// EVRInputStringBits.
#[repr(i32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InputStringBit {
    /// EVRInputStringBits_VRInputString_Hand = 1.
    Hand = sys::EVRInputStringBits_VRInputString_Hand,
    /// EVRInputStringBits_VRInputString_ControllerType = 2.
    ControllerType = sys::EVRInputStringBits_VRInputString_ControllerType,
    /// EVRInputStringBits_VRInputString_InputSource = 4.
    InputSource = sys::EVRInputStringBits_VRInputString_InputSource,
    /// EVRInputStringBits_VRInputString_All = -1.
    All = sys::EVRInputStringBits_VRInputString_All,
}

impl Enum for InputStringBit {
    type Raw = i32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EVRInputStringBits_VRInputString_Hand => Ok(InputStringBit::Hand),
             sys::EVRInputStringBits_VRInputString_ControllerType => Ok(InputStringBit::ControllerType),
             sys::EVRInputStringBits_VRInputString_InputSource => Ok(InputStringBit::InputSource),
             sys::EVRInputStringBits_VRInputString_All => Ok(InputStringBit::All),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<InputStringBit> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of InputStringBit.", self.0)
    }
}

impl error::Error for Invalid<InputStringBit> {
    fn description(&self) -> &str {
        "Value does not represent any variant of InputStringBit."
    }
}

/// EIOBufferError.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IobufferError {
    /// EIOBufferError_IOBuffer_Success = 0.
    Success = sys::EIOBufferError_IOBuffer_Success,
    /// EIOBufferError_IOBuffer_OperationFailed = 100.
    OperationFailed = sys::EIOBufferError_IOBuffer_OperationFailed,
    /// EIOBufferError_IOBuffer_InvalidHandle = 101.
    InvalidHandle = sys::EIOBufferError_IOBuffer_InvalidHandle,
    /// EIOBufferError_IOBuffer_InvalidArgument = 102.
    InvalidArgument = sys::EIOBufferError_IOBuffer_InvalidArgument,
    /// EIOBufferError_IOBuffer_PathExists = 103.
    PathExist = sys::EIOBufferError_IOBuffer_PathExists,
    /// EIOBufferError_IOBuffer_PathDoesNotExist = 104.
    PathDoesNotExist = sys::EIOBufferError_IOBuffer_PathDoesNotExist,
    /// EIOBufferError_IOBuffer_Permission = 105.
    Permission = sys::EIOBufferError_IOBuffer_Permission,
}

impl Enum for IobufferError {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EIOBufferError_IOBuffer_Success => Ok(IobufferError::Success),
             sys::EIOBufferError_IOBuffer_OperationFailed => Ok(IobufferError::OperationFailed),
             sys::EIOBufferError_IOBuffer_InvalidHandle => Ok(IobufferError::InvalidHandle),
             sys::EIOBufferError_IOBuffer_InvalidArgument => Ok(IobufferError::InvalidArgument),
             sys::EIOBufferError_IOBuffer_PathExists => Ok(IobufferError::PathExist),
             sys::EIOBufferError_IOBuffer_PathDoesNotExist => Ok(IobufferError::PathDoesNotExist),
             sys::EIOBufferError_IOBuffer_Permission => Ok(IobufferError::Permission),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<IobufferError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of IobufferError.", self.0)
    }
}

impl error::Error for Invalid<IobufferError> {
    fn description(&self) -> &str {
        "Value does not represent any variant of IobufferError."
    }
}

/// EIOBufferMode.
#[repr(u32)]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IobufferMode {
    /// EIOBufferMode_IOBufferMode_Read = 1.
    Read = sys::EIOBufferMode_IOBufferMode_Read,
    /// EIOBufferMode_IOBufferMode_Write = 2.
    Write = sys::EIOBufferMode_IOBufferMode_Write,
    /// EIOBufferMode_IOBufferMode_Create = 512.
    Create = sys::EIOBufferMode_IOBufferMode_Create,
}

impl Enum for IobufferMode {
    type Raw = u32;

    #[inline]
    fn from_unchecked(val: Unchecked<Self>) -> Result<Self, Invalid<Self>> {
        let raw = val.0;
        match raw {
             sys::EIOBufferMode_IOBufferMode_Read => Ok(IobufferMode::Read),
             sys::EIOBufferMode_IOBufferMode_Write => Ok(IobufferMode::Write),
             sys::EIOBufferMode_IOBufferMode_Create => Ok(IobufferMode::Create),
            _ => Err(Invalid(raw)),
        }
    }

    fn into_unchecked(self) -> Unchecked<Self> {
        Unchecked(self as Self::Raw)
    }
}

impl fmt::Display for Invalid<IobufferMode> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The value {} does not represent any variant of IobufferMode.", self.0)
    }
}

impl error::Error for Invalid<IobufferMode> {
    fn description(&self) -> &str {
        "Value does not represent any variant of IobufferMode."
    }
}

