use crate::*;

macro_rules! impl_enum {
    ($Enum: ident, $Sys: ty, $Invalid: ident {
        $( $Variant: ident = $Value: path, )*
    }) => {
        #[derive(Debug)]
        pub struct $Invalid($Sys);

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum $Enum {
            $(
                $Variant,
            )*
        }

        impl $Enum {
            #[inline]
            pub fn from_sys(val: $Sys) -> Self {
                Self::try_from_sys(val).unwrap()
            }

            #[inline]
            pub fn try_from_sys(val: $Sys) -> Result<Self, $Invalid> {
                match val {
                    $(
                        $Value => Ok($Enum::$Variant),
                    )*
                    invalid => Err($Invalid(invalid))
                }
            }

            #[inline]
            pub fn into_sys(self) -> $Sys {
                match self {
                    $(
                        $Enum::$Variant => $Value, 
                    )*
                }
            }
        }
    };
}

macro_rules! impl_error_enum {
    ($Enum: ident, $Sys: ty, $Invalid: ident {
        None = $None: path,
        $( $Variant: ident = $Value: path, )*
    }) => {
        #[derive(Debug)]
        pub struct $Invalid($Sys);

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub enum $Enum {
            $(
                $Variant,
            )*
        }

        impl $Enum {
            #[inline]
            pub fn from_sys(val: $Sys) -> Option<Self> {
                Self::try_from_sys(val).unwrap()
            }

            #[inline]
            pub fn try_from_sys(val: $Sys) -> Result<Option<Self>, $Invalid> {
                match val {
                    $None => Ok(None),
                    $(
                        $Value => Ok(Some($Enum::$Variant)),
                    )*
                    invalid => Err($Invalid(invalid))
                }
            }

            #[inline]
            pub fn into_sys(self) -> $Sys {
                match self {
                    $(
                        $Enum::$Variant => $Value, 
                    )*
                }
            }
        }
    };
}

impl_enum!(ApplicationType, sys::EVRApplicationType, InvalidApplicationType {
    Other = sys::EVRApplicationType_VRApplication_Other,
    Scene = sys::EVRApplicationType_VRApplication_Scene,
    Overlay = sys::EVRApplicationType_VRApplication_Overlay,
    Background = sys::EVRApplicationType_VRApplication_Background,
    Utility = sys::EVRApplicationType_VRApplication_Utility,
    Vrmonitor = sys::EVRApplicationType_VRApplication_VRMonitor,
    SteamWatchdog = sys::EVRApplicationType_VRApplication_SteamWatchdog,
    Bootstrapper = sys::EVRApplicationType_VRApplication_Bootstrapper,
    WebHelper = sys::EVRApplicationType_VRApplication_WebHelper,
});

impl_enum!(Eye, sys::EVREye, InvalidEye {
    Left = sys::EVREye_Eye_Left,
    Right = sys::EVREye_Eye_Right,
});

bitflags::bitflags! {
    pub struct SubmitFlag: u32 {
        const DEFAULT = sys::EVRSubmitFlags_Submit_Default as u32;
        const LENS_DISTORTION_ALREADY_APPLIED = sys::EVRSubmitFlags_Submit_LensDistortionAlreadyApplied as u32;
        const GL_RENDERBUFFER = sys::EVRSubmitFlags_Submit_GlRenderBuffer as u32;
        const TEXTURE_WITH_POSE = sys::EVRSubmitFlags_Submit_TextureWithPose as u32;
        const TEXTURE_WITH_DEPTH = sys::EVRSubmitFlags_Submit_TextureWithDepth as u32;
    }
}

impl_error_enum!(CompositorError, sys::EVRCompositorError, InvalidCompositorError {
    None = sys::EVRCompositorError_VRCompositorError_None,
    RequestFailed = sys::EVRCompositorError_VRCompositorError_RequestFailed,
    IncompatibleVersion = sys::EVRCompositorError_VRCompositorError_IncompatibleVersion,
    DoNotHaveFocus = sys::EVRCompositorError_VRCompositorError_DoNotHaveFocus,
    InvalidTexture = sys::EVRCompositorError_VRCompositorError_InvalidTexture,
    IsNotSceneApplication = sys::EVRCompositorError_VRCompositorError_IsNotSceneApplication,
    TextureIsOnWrongDevice = sys::EVRCompositorError_VRCompositorError_TextureIsOnWrongDevice,
    TextureUsesUnsupportedFormat = sys::EVRCompositorError_VRCompositorError_TextureUsesUnsupportedFormat,
    SharedTexturesNotSupported = sys::EVRCompositorError_VRCompositorError_SharedTexturesNotSupported,
    IndexOutOfRange = sys::EVRCompositorError_VRCompositorError_IndexOutOfRange,
    AlreadySubmitted = sys::EVRCompositorError_VRCompositorError_AlreadySubmitted,
    InvalidBounds = sys::EVRCompositorError_VRCompositorError_InvalidBounds,
});

impl_error_enum!(InitError, sys::EVRInitError, InvalidInitError {
    None = sys::EVRInitError_VRInitError_None,
    Unknown = sys::EVRInitError_VRInitError_Unknown,
    InitInstallationNotFound = sys::EVRInitError_VRInitError_Init_InstallationNotFound,
    InitInstallationCorrupt = sys::EVRInitError_VRInitError_Init_InstallationCorrupt,
    InitVRClientDLLNotFound = sys::EVRInitError_VRInitError_Init_VRClientDLLNotFound,
    InitFileNotFound = sys::EVRInitError_VRInitError_Init_FileNotFound,
    InitFactoryNotFound = sys::EVRInitError_VRInitError_Init_FactoryNotFound,
    InitInterfaceNotFound = sys::EVRInitError_VRInitError_Init_InterfaceNotFound,
    InitInvalidInterface = sys::EVRInitError_VRInitError_Init_InvalidInterface,
    InitUserConfigDirectoryInvalid = sys::EVRInitError_VRInitError_Init_UserConfigDirectoryInvalid,
    InitHmdNotFound = sys::EVRInitError_VRInitError_Init_HmdNotFound,
    InitNotInitialized = sys::EVRInitError_VRInitError_Init_NotInitialized,
    InitPathRegistryNotFound = sys::EVRInitError_VRInitError_Init_PathRegistryNotFound,
    InitNoConfigPath = sys::EVRInitError_VRInitError_Init_NoConfigPath,
    InitNoLogPath = sys::EVRInitError_VRInitError_Init_NoLogPath,
    InitPathRegistryNotWritable = sys::EVRInitError_VRInitError_Init_PathRegistryNotWritable,
    InitAppInfoInitFailed = sys::EVRInitError_VRInitError_Init_AppInfoInitFailed,
    InitRetry = sys::EVRInitError_VRInitError_Init_Retry,
    InitInitCanceledByUser = sys::EVRInitError_VRInitError_Init_InitCanceledByUser,
    InitAnotherAppLaunching = sys::EVRInitError_VRInitError_Init_AnotherAppLaunching,
    InitSettingsInitFailed = sys::EVRInitError_VRInitError_Init_SettingsInitFailed,
    InitShuttingDown = sys::EVRInitError_VRInitError_Init_ShuttingDown,
    InitTooManyObjects = sys::EVRInitError_VRInitError_Init_TooManyObjects,
    InitNoServerForBackgroundApp = sys::EVRInitError_VRInitError_Init_NoServerForBackgroundApp,
    InitNotSupportedWithCompositor = sys::EVRInitError_VRInitError_Init_NotSupportedWithCompositor,
    InitNotAvailableToUtilityApps = sys::EVRInitError_VRInitError_Init_NotAvailableToUtilityApps,
    InitInternal = sys::EVRInitError_VRInitError_Init_Internal,
    InitHmdDriverIdIsNone = sys::EVRInitError_VRInitError_Init_HmdDriverIdIsNone,
    InitHmdNotFoundPresenceFailed = sys::EVRInitError_VRInitError_Init_HmdNotFoundPresenceFailed,
    InitVRMonitorNotFound = sys::EVRInitError_VRInitError_Init_VRMonitorNotFound,
    InitVRMonitorStartupFailed = sys::EVRInitError_VRInitError_Init_VRMonitorStartupFailed,
    InitLowPowerWatchdogNotSupported = sys::EVRInitError_VRInitError_Init_LowPowerWatchdogNotSupported,
    InitInvalidApplicationType = sys::EVRInitError_VRInitError_Init_InvalidApplicationType,
    InitNotAvailableToWatchdogApps = sys::EVRInitError_VRInitError_Init_NotAvailableToWatchdogApps,
    InitWatchdogDisabledInSettings = sys::EVRInitError_VRInitError_Init_WatchdogDisabledInSettings,
    InitVRDashboardNotFound = sys::EVRInitError_VRInitError_Init_VRDashboardNotFound,
    InitVRDashboardStartupFailed = sys::EVRInitError_VRInitError_Init_VRDashboardStartupFailed,
    InitVRHomeNotFound = sys::EVRInitError_VRInitError_Init_VRHomeNotFound,
    InitVRHomeStartupFailed = sys::EVRInitError_VRInitError_Init_VRHomeStartupFailed,
    InitRebootingBusy = sys::EVRInitError_VRInitError_Init_RebootingBusy,
    InitFirmwareUpdateBusy = sys::EVRInitError_VRInitError_Init_FirmwareUpdateBusy,
    InitFirmwareRecoveryBusy = sys::EVRInitError_VRInitError_Init_FirmwareRecoveryBusy,
    InitUSBServiceBusy = sys::EVRInitError_VRInitError_Init_USBServiceBusy,
    InitVRWebHelperStartupFailed = sys::EVRInitError_VRInitError_Init_VRWebHelperStartupFailed,
    InitTrackerManagerInitFailed = sys::EVRInitError_VRInitError_Init_TrackerManagerInitFailed,
    InitAlreadyRunning = sys::EVRInitError_VRInitError_Init_AlreadyRunning,
    InitFailedForVrMonitor = sys::EVRInitError_VRInitError_Init_FailedForVrMonitor,
    DriverFailed = sys::EVRInitError_VRInitError_Driver_Failed,
    DriverUnknown = sys::EVRInitError_VRInitError_Driver_Unknown,
    DriverHmdUnknown = sys::EVRInitError_VRInitError_Driver_HmdUnknown,
    DriverNotLoaded = sys::EVRInitError_VRInitError_Driver_NotLoaded,
    DriverRuntimeOutOfDate = sys::EVRInitError_VRInitError_Driver_RuntimeOutOfDate,
    DriverHmdInUse = sys::EVRInitError_VRInitError_Driver_HmdInUse,
    DriverNotCalibrated = sys::EVRInitError_VRInitError_Driver_NotCalibrated,
    DriverCalibrationInvalid = sys::EVRInitError_VRInitError_Driver_CalibrationInvalid,
    DriverHmdDisplayNotFound = sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFound,
    DriverTrackedDeviceInterfaceUnknown = sys::EVRInitError_VRInitError_Driver_TrackedDeviceInterfaceUnknown,
    DriverHmdDriverIdOutOfBounds = sys::EVRInitError_VRInitError_Driver_HmdDriverIdOutOfBounds,
    DriverHmdDisplayMirrored = sys::EVRInitError_VRInitError_Driver_HmdDisplayMirrored,
    DriverHmdDisplayNotFoundLaptop = sys::EVRInitError_VRInitError_Driver_HmdDisplayNotFoundLaptop,
    IPCServerInitFailed = sys::EVRInitError_VRInitError_IPC_ServerInitFailed,
    IPCConnectFailed = sys::EVRInitError_VRInitError_IPC_ConnectFailed,
    IPCSharedStateInitFailed = sys::EVRInitError_VRInitError_IPC_SharedStateInitFailed,
    IPCCompositorInitFailed = sys::EVRInitError_VRInitError_IPC_CompositorInitFailed,
    IPCMutexInitFailed = sys::EVRInitError_VRInitError_IPC_MutexInitFailed,
    IPCFailed = sys::EVRInitError_VRInitError_IPC_Failed,
    IPCCompositorConnectFailed = sys::EVRInitError_VRInitError_IPC_CompositorConnectFailed,
    IPCCompositorInvalidConnectResponse = sys::EVRInitError_VRInitError_IPC_CompositorInvalidConnectResponse,
    IPCConnectFailedAfterMultipleAttempts = sys::EVRInitError_VRInitError_IPC_ConnectFailedAfterMultipleAttempts,
    CompositorFailed = sys::EVRInitError_VRInitError_Compositor_Failed,
    CompositorD3D11HardwareRequired = sys::EVRInitError_VRInitError_Compositor_D3D11HardwareRequired,
    CompositorFirmwareRequiresUpdate = sys::EVRInitError_VRInitError_Compositor_FirmwareRequiresUpdate,
    CompositorOverlayInitFailed = sys::EVRInitError_VRInitError_Compositor_OverlayInitFailed,
    CompositorScreenshotsInitFailed = sys::EVRInitError_VRInitError_Compositor_ScreenshotsInitFailed,
    CompositorUnableToCreateDevice = sys::EVRInitError_VRInitError_Compositor_UnableToCreateDevice,
    CompositorSharedStateIsNull = sys::EVRInitError_VRInitError_Compositor_SharedStateIsNull,
    CompositorNotificationManagerIsNull = sys::EVRInitError_VRInitError_Compositor_NotificationManagerIsNull,
    CompositorResourceManagerClientIsNull = sys::EVRInitError_VRInitError_Compositor_ResourceManagerClientIsNull,
    CompositorMessageOverlaySharedStateInitFailure = sys::EVRInitError_VRInitError_Compositor_MessageOverlaySharedStateInitFailure,
    CompositorPropertiesInterfaceIsNull = sys::EVRInitError_VRInitError_Compositor_PropertiesInterfaceIsNull,
    CompositorCreateFullscreenWindowFailed = sys::EVRInitError_VRInitError_Compositor_CreateFullscreenWindowFailed,
    CompositorSettingsInterfaceIsNull = sys::EVRInitError_VRInitError_Compositor_SettingsInterfaceIsNull,
    CompositorFailedToShowWindow = sys::EVRInitError_VRInitError_Compositor_FailedToShowWindow,
    CompositorDistortInterfaceIsNull = sys::EVRInitError_VRInitError_Compositor_DistortInterfaceIsNull,
    CompositorDisplayFrequencyFailure = sys::EVRInitError_VRInitError_Compositor_DisplayFrequencyFailure,
    CompositorRendererInitializationFailed = sys::EVRInitError_VRInitError_Compositor_RendererInitializationFailed,
    CompositorDXGIFactoryInterfaceIsNull = sys::EVRInitError_VRInitError_Compositor_DXGIFactoryInterfaceIsNull,
    CompositorDXGIFactoryCreateFailed = sys::EVRInitError_VRInitError_Compositor_DXGIFactoryCreateFailed,
    CompositorDXGIFactoryQueryFailed = sys::EVRInitError_VRInitError_Compositor_DXGIFactoryQueryFailed,
    CompositorInvalidAdapterDesktop = sys::EVRInitError_VRInitError_Compositor_InvalidAdapterDesktop,
    CompositorInvalidHmdAttachment = sys::EVRInitError_VRInitError_Compositor_InvalidHmdAttachment,
    CompositorInvalidOutputDesktop = sys::EVRInitError_VRInitError_Compositor_InvalidOutputDesktop,
    CompositorInvalidDeviceProvided = sys::EVRInitError_VRInitError_Compositor_InvalidDeviceProvided,
    CompositorD3D11RendererInitializationFailed = sys::EVRInitError_VRInitError_Compositor_D3D11RendererInitializationFailed,
    CompositorFailedToFindDisplayMode = sys::EVRInitError_VRInitError_Compositor_FailedToFindDisplayMode,
    CompositorFailedToCreateSwapChain = sys::EVRInitError_VRInitError_Compositor_FailedToCreateSwapChain,
    CompositorFailedToGetBackBuffer = sys::EVRInitError_VRInitError_Compositor_FailedToGetBackBuffer,
    CompositorFailedToCreateRenderTarget = sys::EVRInitError_VRInitError_Compositor_FailedToCreateRenderTarget,
    CompositorFailedToCreateDXGI2SwapChain = sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2SwapChain,
    CompositorFailedtoGetDXGI2BackBuffer = sys::EVRInitError_VRInitError_Compositor_FailedtoGetDXGI2BackBuffer,
    CompositorFailedToCreateDXGI2RenderTarget = sys::EVRInitError_VRInitError_Compositor_FailedToCreateDXGI2RenderTarget,
    CompositorFailedToGetDXGIDeviceInterface = sys::EVRInitError_VRInitError_Compositor_FailedToGetDXGIDeviceInterface,
    CompositorSelectDisplayMode = sys::EVRInitError_VRInitError_Compositor_SelectDisplayMode,
    CompositorFailedToCreateNvAPIRenderTargets = sys::EVRInitError_VRInitError_Compositor_FailedToCreateNvAPIRenderTargets,
    CompositorNvAPISetDisplayMode = sys::EVRInitError_VRInitError_Compositor_NvAPISetDisplayMode,
    CompositorFailedToCreateDirectModeDisplay = sys::EVRInitError_VRInitError_Compositor_FailedToCreateDirectModeDisplay,
    CompositorInvalidHmdPropertyContainer = sys::EVRInitError_VRInitError_Compositor_InvalidHmdPropertyContainer,
    CompositorUpdateDisplayFrequency = sys::EVRInitError_VRInitError_Compositor_UpdateDisplayFrequency,
    CompositorCreateRasterizerState = sys::EVRInitError_VRInitError_Compositor_CreateRasterizerState,
    CompositorCreateWireframeRasterizerState = sys::EVRInitError_VRInitError_Compositor_CreateWireframeRasterizerState,
    CompositorCreateSamplerState = sys::EVRInitError_VRInitError_Compositor_CreateSamplerState,
    CompositorCreateClampToBorderSamplerState = sys::EVRInitError_VRInitError_Compositor_CreateClampToBorderSamplerState,
    CompositorCreateAnisoSamplerState = sys::EVRInitError_VRInitError_Compositor_CreateAnisoSamplerState,
    CompositorCreateOverlaySamplerState = sys::EVRInitError_VRInitError_Compositor_CreateOverlaySamplerState,
    CompositorCreatePanoramaSamplerState = sys::EVRInitError_VRInitError_Compositor_CreatePanoramaSamplerState,
    CompositorCreateFontSamplerState = sys::EVRInitError_VRInitError_Compositor_CreateFontSamplerState,
    CompositorCreateNoBlendState = sys::EVRInitError_VRInitError_Compositor_CreateNoBlendState,
    CompositorCreateBlendState = sys::EVRInitError_VRInitError_Compositor_CreateBlendState,
    CompositorCreateAlphaBlendState = sys::EVRInitError_VRInitError_Compositor_CreateAlphaBlendState,
    CompositorCreateBlendStateMaskR = sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskR,
    CompositorCreateBlendStateMaskG = sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskG,
    CompositorCreateBlendStateMaskB = sys::EVRInitError_VRInitError_Compositor_CreateBlendStateMaskB,
    CompositorCreateDepthStencilState = sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilState,
    CompositorCreateDepthStencilStateNoWrite = sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoWrite,
    CompositorCreateDepthStencilStateNoDepth = sys::EVRInitError_VRInitError_Compositor_CreateDepthStencilStateNoDepth,
    CompositorCreateFlushTexture = sys::EVRInitError_VRInitError_Compositor_CreateFlushTexture,
    CompositorCreateDistortionSurfaces = sys::EVRInitError_VRInitError_Compositor_CreateDistortionSurfaces,
    CompositorCreateConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateConstantBuffer,
    CompositorCreateHmdPoseConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseConstantBuffer,
    CompositorCreateHmdPoseStagingConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateHmdPoseStagingConstantBuffer,
    CompositorCreateSharedFrameInfoConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateSharedFrameInfoConstantBuffer,
    CompositorCreateOverlayConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateOverlayConstantBuffer,
    CompositorCreateSceneTextureIndexConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateSceneTextureIndexConstantBuffer,
    CompositorCreateReadableSceneTextureIndexConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateReadableSceneTextureIndexConstantBuffer,
    CompositorCreateLayerGraphicsTextureIndexConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateLayerGraphicsTextureIndexConstantBuffer,
    CompositorCreateLayerComputeTextureIndexConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeTextureIndexConstantBuffer,
    CompositorCreateLayerComputeSceneTextureIndexConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateLayerComputeSceneTextureIndexConstantBuffer,
    CompositorCreateComputeHmdPoseConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateComputeHmdPoseConstantBuffer,
    CompositorCreateGeomConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreateGeomConstantBuffer,
    CompositorCreatePanelMaskConstantBuffer = sys::EVRInitError_VRInitError_Compositor_CreatePanelMaskConstantBuffer,
    CompositorCreatePixelSimUBO = sys::EVRInitError_VRInitError_Compositor_CreatePixelSimUBO,
    CompositorCreateMSAARenderTextures = sys::EVRInitError_VRInitError_Compositor_CreateMSAARenderTextures,
    CompositorCreateResolveRenderTextures = sys::EVRInitError_VRInitError_Compositor_CreateResolveRenderTextures,
    CompositorCreateComputeResolveRenderTextures = sys::EVRInitError_VRInitError_Compositor_CreateComputeResolveRenderTextures,
    CompositorCreateDriverDirectModeResolveTextures = sys::EVRInitError_VRInitError_Compositor_CreateDriverDirectModeResolveTextures,
    CompositorOpenDriverDirectModeResolveTextures = sys::EVRInitError_VRInitError_Compositor_OpenDriverDirectModeResolveTextures,
    CompositorCreateFallbackSyncTexture = sys::EVRInitError_VRInitError_Compositor_CreateFallbackSyncTexture,
    CompositorShareFallbackSyncTexture = sys::EVRInitError_VRInitError_Compositor_ShareFallbackSyncTexture,
    CompositorCreateOverlayIndexBuffer = sys::EVRInitError_VRInitError_Compositor_CreateOverlayIndexBuffer,
    CompositorCreateOverlayVertextBuffer = sys::EVRInitError_VRInitError_Compositor_CreateOverlayVertextBuffer,
    CompositorCreateTextVertexBuffer = sys::EVRInitError_VRInitError_Compositor_CreateTextVertexBuffer,
    CompositorCreateTextIndexBuffer = sys::EVRInitError_VRInitError_Compositor_CreateTextIndexBuffer,
    CompositorCreateMirrorTextures = sys::EVRInitError_VRInitError_Compositor_CreateMirrorTextures,
    CompositorCreateLastFrameRenderTexture = sys::EVRInitError_VRInitError_Compositor_CreateLastFrameRenderTexture,
    VendorSpecificUnableToConnectToOculusRuntime = sys::EVRInitError_VRInitError_VendorSpecific_UnableToConnectToOculusRuntime,
    VendorSpecificWindowsNotInDevMode = sys::EVRInitError_VRInitError_VendorSpecific_WindowsNotInDevMode,
    VendorSpecificHmdFoundCantOpenDevice = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantOpenDevice,
    VendorSpecificHmdFoundUnableToRequestConfigStart = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToRequestConfigStart,
    VendorSpecificHmdFoundNoStoredConfig = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_NoStoredConfig,
    VendorSpecificHmdFoundConfigTooBig = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooBig,
    VendorSpecificHmdFoundConfigTooSmall = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigTooSmall,
    VendorSpecificHmdFoundUnableToInitZLib = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToInitZLib,
    VendorSpecificHmdFoundCantReadFirmwareVersion = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_CantReadFirmwareVersion,
    VendorSpecificHmdFoundUnableToSendUserDataStart = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToSendUserDataStart,
    VendorSpecificHmdFoundUnableToGetUserDataStart = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataStart,
    VendorSpecificHmdFoundUnableToGetUserDataNext = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UnableToGetUserDataNext,
    VendorSpecificHmdFoundUserDataAddressRange = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataAddressRange,
    VendorSpecificHmdFoundUserDataError = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_UserDataError,
    VendorSpecificHmdFoundConfigFailedSanityCheck = sys::EVRInitError_VRInitError_VendorSpecific_HmdFound_ConfigFailedSanityCheck,
    SteamSteamInstallationNotFound = sys::EVRInitError_VRInitError_Steam_SteamInstallationNotFound,
});
