//! `switch/services/applet.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// AppletType
pub type AppletType = i32;
pub const AppletType_None: AppletType = -2;
pub const AppletType_Default: AppletType = -1;
pub const AppletType_Application: AppletType = 0;
pub const AppletType_SystemApplet: AppletType = 1;
pub const AppletType_LibraryApplet: AppletType = 2;
pub const AppletType_OverlayApplet: AppletType = 3;
pub const AppletType_SystemApplication: AppletType = 4;

/// OperationMode
pub type AppletOperationMode = u32;
/// Handheld
pub const AppletOperationMode_Handheld: AppletOperationMode = 0;
/// Console (Docked / TV-mode)
pub const AppletOperationMode_Console: AppletOperationMode = 1;

/// applet hook types.
pub type AppletHookType = u32;
/// ::AppletMessage_FocusStateChanged
pub const AppletHookType_OnFocusState: AppletHookType = 0;
/// ::AppletMessage_OperationModeChanged
pub const AppletHookType_OnOperationMode: AppletHookType = 1;
/// ::AppletMessage_PerformanceModeChanged
pub const AppletHookType_OnPerformanceMode: AppletHookType = 2;
/// ::AppletMessage_ExitRequested
pub const AppletHookType_OnExitRequest: AppletHookType = 3;
/// ::AppletMessage_Resume
pub const AppletHookType_OnResume: AppletHookType = 4;
/// ::AppletMessage_CaptureButtonShortPressed
pub const AppletHookType_OnCaptureButtonShortPressed: AppletHookType = 5;
/// ::AppletMessage_AlbumScreenShotTaken
pub const AppletHookType_OnAlbumScreenShotTaken: AppletHookType = 6;
/// ::AppletMessage_RequestToDisplay
pub const AppletHookType_RequestToDisplay: AppletHookType = 7;
/// Number of applet hook types.
pub const AppletHookType_Max: AppletHookType = 8;

/// AppletMessage, for `appletGetMessage`. See also `AppletHookType`.
pub type AppletMessage = u32;
/// Exit request.
pub const AppletMessage_ExitRequest: AppletMessage = 4;
/// FocusState changed.
pub const AppletMessage_FocusStateChanged: AppletMessage = 15;
/// Current applet execution was resumed.
pub const AppletMessage_Resume: AppletMessage = 16;
/// OperationMode changed.
pub const AppletMessage_OperationModeChanged: AppletMessage = 30;
/// PerformanceMode changed.
pub const AppletMessage_PerformanceModeChanged: AppletMessage = 31;
/// Display requested, see `appletApproveToDisplay`.
pub const AppletMessage_RequestToDisplay: AppletMessage = 51;
/// Capture button was short-pressed.
pub const AppletMessage_CaptureButtonShortPressed: AppletMessage = 90;
/// Screenshot was taken.
pub const AppletMessage_AlbumScreenShotTaken: AppletMessage = 92;
/// AlbumRecordingSaved
pub const AppletMessage_AlbumRecordingSaved: AppletMessage = 93;

/// FocusState
pub type AppletFocusState = u32;
/// Applet is focused.
pub const AppletFocusState_InFocus: AppletFocusState = 1;
/// Out of focus - LibraryApplet open.
pub const AppletFocusState_OutOfFocus: AppletFocusState = 2;
/// Out of focus - HOME menu open / console is sleeping.
pub const AppletFocusState_Background: AppletFocusState = 3;

/// FocusHandlingMode
pub type AppletFocusHandlingMode = u32;
/// Suspend only when HOME menu is open / console is sleeping (default).
pub const AppletFocusHandlingMode_SuspendHomeSleep: AppletFocusHandlingMode = 0;
/// Don't suspend when out of focus.
pub const AppletFocusHandlingMode_NoSuspend: AppletFocusHandlingMode = 1;
/// Suspend only when HOME menu is open / console is sleeping but still receive OnFocusState hook.
pub const AppletFocusHandlingMode_SuspendHomeSleepNotify: AppletFocusHandlingMode = 2;
/// Always suspend when out of focus, regardless of the reason.
pub const AppletFocusHandlingMode_AlwaysSuspend: AppletFocusHandlingMode = 3;
/// Number of focus handling modes.
pub const AppletFocusHandlingMode_Max: AppletFocusHandlingMode = 4;

/// LaunchParameterKind
pub type AppletLaunchParameterKind = u32;
/// UserChannel. Application-specific LaunchParameter.
pub const AppletLaunchParameterKind_UserChannel: AppletLaunchParameterKind = 1;
/// account PreselectedUser
pub const AppletLaunchParameterKind_PreselectedUser: AppletLaunchParameterKind = 2;
/// Unknown if used by anything?
pub const AppletLaunchParameterKind_Unknown: AppletLaunchParameterKind = 3;

/// AppletId
pub type AppletId = u32;
/// None
pub const AppletId_None: AppletId = 0x00;
/// Application. Not valid for use with LibraryApplets.
pub const AppletId_application: AppletId = 0x01;
/// 010000000000100C "overlayDisp"
pub const AppletId_OverlayApplet: AppletId = 0x02;
/// 0100000000001000 "qlaunch" (SystemAppletMenu)
pub const AppletId_SystemAppletMenu: AppletId = 0x03;
/// 0100000000001012 "starter" SystemApplication.
pub const AppletId_SystemApplication: AppletId = 0x04;
/// 0100000000001001 "auth"
pub const AppletId_LibraryAppletAuth: AppletId = 0x0A;
/// 0100000000001002 "cabinet"
pub const AppletId_LibraryAppletCabinet: AppletId = 0x0B;
/// 0100000000001003 "controller"
pub const AppletId_LibraryAppletController: AppletId = 0x0C;
/// 0100000000001004 "dataErase"
pub const AppletId_LibraryAppletDataErase: AppletId = 0x0D;
/// 0100000000001005 "error"
pub const AppletId_LibraryAppletError: AppletId = 0x0E;
/// 0100000000001006 "netConnect"
pub const AppletId_LibraryAppletNetConnect: AppletId = 0x0F;
/// 0100000000001007 "playerSelect"
pub const AppletId_LibraryAppletPlayerSelect: AppletId = 0x10;
/// 0100000000001008 "swkbd"
pub const AppletId_LibraryAppletSwkbd: AppletId = 0x11;
/// 0100000000001009 "miiEdit"
pub const AppletId_LibraryAppletMiiEdit: AppletId = 0x12;
/// 010000000000100A "LibAppletWeb" WebApplet applet
pub const AppletId_LibraryAppletWeb: AppletId = 0x13;
/// 010000000000100B "LibAppletShop" ShopN applet
pub const AppletId_LibraryAppletShop: AppletId = 0x14;
/// 010000000000100D "photoViewer"
pub const AppletId_LibraryAppletPhotoViewer: AppletId = 0x15;
/// 010000000000100E "set" (This applet is currently not present on retail devices.)
pub const AppletId_LibraryAppletSet: AppletId = 0x16;
/// 010000000000100F "LibAppletOff" offlineWeb applet
pub const AppletId_LibraryAppletOfflineWeb: AppletId = 0x17;
/// 0100000000001010 "LibAppletLns" loginShare web-applet
pub const AppletId_LibraryAppletLoginShare: AppletId = 0x18;
/// 0100000000001011 "LibAppletAuth" wifiWebAuth applet
pub const AppletId_LibraryAppletWifiWebAuth: AppletId = 0x19;
/// 0100000000001013 "myPage"
pub const AppletId_LibraryAppletMyPage: AppletId = 0x1A;

/// LibraryAppletMode
pub type LibAppletMode = u32;
/// Foreground.
pub const LibAppletMode_AllForeground: LibAppletMode = 0;
/// Background.
pub const LibAppletMode_Background: LibAppletMode = 1;
/// No UI.
pub const LibAppletMode_NoUi: LibAppletMode = 2;
/// Background with indirect display, see `appletHolderGetIndirectLayerConsumerHandle`.
pub const LibAppletMode_BackgroundIndirect: LibAppletMode = 3;
/// Foreground except initially hidden.
pub const LibAppletMode_AllForegroundInitiallyHidden: LibAppletMode = 4;

/// LibraryAppletExitReason
pub type LibAppletExitReason = u32;
pub const LibAppletExitReason_Normal: LibAppletExitReason = 0;
pub const LibAppletExitReason_Canceled: LibAppletExitReason = 1;
pub const LibAppletExitReason_Abnormal: LibAppletExitReason = 2;
pub const LibAppletExitReason_Unexpected: LibAppletExitReason = 10;

/// AppletApplicationExitReason
pub type AppletApplicationExitReason = u32;
pub const AppletApplicationExitReason_Normal: AppletApplicationExitReason = 0;
pub const AppletApplicationExitReason_Unknown1: AppletApplicationExitReason = 1;
pub const AppletApplicationExitReason_Unknown2: AppletApplicationExitReason = 2;
pub const AppletApplicationExitReason_Unknown3: AppletApplicationExitReason = 3;
pub const AppletApplicationExitReason_Unknown4: AppletApplicationExitReason = 4;
pub const AppletApplicationExitReason_Unknown5: AppletApplicationExitReason = 5;
pub const AppletApplicationExitReason_Unexpected: AppletApplicationExitReason = 100;

/// ThemeColorType
pub type AppletThemeColorType = u32;
pub const AppletThemeColorType_Default: AppletThemeColorType = 0;
pub const AppletThemeColorType_Unknown1: AppletThemeColorType = 1;
pub const AppletThemeColorType_Unknown2: AppletThemeColorType = 2;
pub const AppletThemeColorType_Unknown3: AppletThemeColorType = 3;

/// Mode values for `appletSetTvPowerStateMatchingMode`.
pub type AppletTvPowerStateMatchingMode = u32;
/// Unknown.
pub const AppletTvPowerStateMatchingMode_Unknown0: AppletTvPowerStateMatchingMode = 0;
/// Unknown.
pub const AppletTvPowerStateMatchingMode_Unknown1: AppletTvPowerStateMatchingMode = 1;

/// Type values for `appletPerformSystemButtonPressingIfInFocus`.
pub type AppletSystemButtonType = u32;
/// Short-pressing with the HOME-button.
pub const AppletSystemButtonType_HomeButtonShortPressing: AppletSystemButtonType = 1;
/// Long-pressing with the HOME-button.
pub const AppletSystemButtonType_HomeButtonLongPressing: AppletSystemButtonType = 2;
/// Short-pressing with the Power-button. Only available with `appletPerformSystemButtonPressing`.
pub const AppletSystemButtonType_PowerButtonShortPressing: AppletSystemButtonType = 3;
/// Long-pressing with the Power-button. Only available with `appletPerformSystemButtonPressing`.
pub const AppletSystemButtonType_PowerButtonLongPressing: AppletSystemButtonType = 4;
/// Shutdown the system, as if the Power-button was held for longer than ::AppletSystemButtonType_PowerButtonLongPressing. Only available with `appletPerformSystemButtonPressing`.
pub const AppletSystemButtonType_Shutdown: AppletSystemButtonType = 5;
/// Short-pressing with the Capture-button.
pub const AppletSystemButtonType_CaptureButtonShortPressing: AppletSystemButtonType = 6;
/// Long-pressing with the Capture-button.
pub const AppletSystemButtonType_CaptureButtonLongPressing: AppletSystemButtonType = 7;

/// Permission values for `appletSetScreenShotPermission`.
pub type AppletScreenShotPermission = u32;
/// Inherit from parent applet.
pub const AppletScreenShotPermission_Inherit: AppletScreenShotPermission = 0;
/// Enable.
pub const AppletScreenShotPermission_Enable: AppletScreenShotPermission = 1;
/// Disable.
pub const AppletScreenShotPermission_Disable: AppletScreenShotPermission = 2;

/// Extension values for `appletSetIdleTimeDetectionExtension` / `appletGetIdleTimeDetectionExtension`, for extending user inactivity detection.
pub type AppletIdleTimeDetectionExtension = u32;
/// No extension.
pub const AppletIdleTimeDetectionExtension_None: AppletIdleTimeDetectionExtension = 0;
/// Extended
pub const AppletIdleTimeDetectionExtension_Extended: AppletIdleTimeDetectionExtension = 1;
/// ExtendedUnsafe
pub const AppletIdleTimeDetectionExtension_ExtendedUnsafe: AppletIdleTimeDetectionExtension = 2;

/// Input policy values for `appletSetInputDetectionPolicy`.
pub type AppletInputDetectionPolicy = u32;
/// Unknown.
pub const AppletInputDetectionPolicy_Unknown0: AppletInputDetectionPolicy = 0;
/// Unknown.
pub const AppletInputDetectionPolicy_Unknown1: AppletInputDetectionPolicy = 1;

/// Input mode values for `appletSetWirelessPriorityMode`.
pub type AppletWirelessPriorityMode = u32;
/// Default
pub const AppletWirelessPriorityMode_Default: AppletWirelessPriorityMode = 1;
/// OptimizedForWlan
pub const AppletWirelessPriorityMode_OptimizedForWlan: AppletWirelessPriorityMode = 2;

/// CaptureSharedBuffer for the IDisplayController commands.
pub type AppletCaptureSharedBuffer = u32;
/// LastApplication
pub const AppletCaptureSharedBuffer_LastApplication: AppletCaptureSharedBuffer = 0;
/// LastForeground
pub const AppletCaptureSharedBuffer_LastForeground: AppletCaptureSharedBuffer = 1;
/// CallerApplet
pub const AppletCaptureSharedBuffer_CallerApplet: AppletCaptureSharedBuffer = 2;

/// WindowOriginMode
pub type AppletWindowOriginMode = u32;
/// LowerLeft
pub const AppletWindowOriginMode_LowerLeft: AppletWindowOriginMode = 0;
/// UpperLeft
pub const AppletWindowOriginMode_UpperLeft: AppletWindowOriginMode = 1;

/// ProgramSpecifyKind for the ExecuteProgram cmd. Controls the type of the u64 passed to the ExecuteProgram cmd.
pub type AppletProgramSpecifyKind = u32;
/// u8 ProgramIndex.
pub const AppletProgramSpecifyKind_ExecuteProgram: AppletProgramSpecifyKind = 0;
/// u64 application_id. Only available when DebugMode is enabled.
pub const AppletProgramSpecifyKind_JumpToSubApplicationProgramForDevelopment: AppletProgramSpecifyKind = 1;
/// u64 = value 0.
pub const AppletProgramSpecifyKind_RestartProgram: AppletProgramSpecifyKind = 2;

/// applet hook function.
pub type AppletHookFn = Option<unsafe extern "C" fn(AppletHookType, *mut c_void)>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletHookCookie {
    /// Next cookie.
    pub next: *mut AppletHookCookie,
    /// Hook callback.
    pub callback: AppletHookFn,
    /// Callback parameter.
    pub param: *mut c_void,
}

/// LockAccessor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletLockAccessor {
    /// ILockAccessor
    pub s: Service,
    /// Event from the GetEvent cmd, with autoclear=false.
    pub event: Event,
}

/// applet IStorage
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletStorage {
    /// IStorage
    pub s: Service,
    /// TransferMemory
    pub tmem: TransferMemory,
}

/// LibraryApplet state.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletHolder {
    /// ILibraryAppletAccessor
    pub s: Service,
    /// Output from GetAppletStateChangedEvent, autoclear=false.
    pub StateChangedEvent: Event,
    /// Output from GetPopInteractiveOutDataEvent, autoclear=false.
    pub PopInteractiveOutDataEvent: Event,
    /// See ref `LibAppletMode`.
    pub mode: LibAppletMode,
    /// Output from GetIndirectLayerConsumerHandle on [2.0.0+].
    pub layer_handle: u64,
    /// When set, indicates that the LibraryApplet is creating itself.
    pub creating_self: bool,
    /// Set by `appletHolderJoin` using the output from cmd GetResult, see `LibAppletExitReason`.
    pub exitreason: LibAppletExitReason,
}

/// IApplicationAccessor container.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletApplication {
    /// IApplicationAccessor
    pub s: Service,
    /// Output from GetAppletStateChangedEvent, autoclear=false.
    pub StateChangedEvent: Event,
    /// Set by `appletApplicationJoin` using the output from cmd GetResult, see `AppletApplicationExitReason`.
    pub exitreason: AppletApplicationExitReason,
}

/// GpuErrorHandler
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletGpuErrorHandler {
    /// IGpuErrorHandler
    pub s: Service,
}

/// Used by `appletInitialize` with __nx_applet_AppletAttribute for cmd OpenLibraryAppletProxy (AppletType_LibraryApplet), on [3.0.0+]. The default for this struct is all-zero.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletAttribute {
    /// Flag. When non-zero, two state fields are set to 1.
    pub flag: u8,
    /// Unused.
    pub reserved: [u8; 127],
}

/// LibraryAppletInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LibAppletInfo {
    /// **Ref:** AppletId
    pub appletId: AppletId,
    /// **Ref:** LibAppletMode
    pub mode: LibAppletMode,
}

/// AppletProcessLaunchReason, from GetLaunchReason.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletProcessLaunchReason {
    /// When non-zero, indicates that OpenCallingLibraryApplet should be used.
    pub flag: u8,
    /// Always zero.
    pub unk_x1: [u8; 3],
}

/// Cached info for the current LibraryApplet, from `appletGetAppletInfo`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletInfo {
    /// Output from `appletGetLibraryAppletInfo`.
    pub info: LibAppletInfo,
    /// Loaded from AppletProcessLaunchReason::flag, indicates that the below AppletHolder is initialized.
    pub caller_flag: bool,
    /// **Ref:** AppletHolder for the CallingLibraryApplet, automatically closed by `appletExit` when needed.
    pub caller: AppletHolder,
}

/// IdentityInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletIdentityInfo {
    /// **Ref:** AppletId
    pub appletId: AppletId,
    /// Padding.
    pub pad: u32,
    /// ApplicationId, only set with appletId == ::AppletId_application.
    pub application_id: u64,
}

/// Attributes for launching applications for Quest.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletApplicationAttributeForQuest {
    /// See AppletApplicationAttribute::unk_x0.
    pub unk_x0: u32,
    /// See AppletApplicationAttribute::unk_x4.
    pub unk_x4: u32,
    /// [7.0.0+] See AppletApplicationAttribute::volume.
    pub volume: f32,
}

/// ApplicationAttribute
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletApplicationAttribute {
    /// Default is 0 for non-Quest. Only used when non-zero: unknown value in seconds.
    pub unk_x0: u32,
    /// Default is 0 for non-Quest. Only used when non-zero: unknown value in seconds.
    pub unk_x4: u32,
    /// Audio volume. Must be in the range of 0.0f-1.0f. The default is 1.0f.
    pub volume: f32,
    /// Unused. Default is 0.
    pub unused: [u8; 20],
}

/// ApplicationLaunchProperty
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletApplicationLaunchProperty {
    /// ApplicationId.
    pub application_id: u64,
    /// Application version.
    pub version: u32,
    /// **Ref:** NcmStorageId for the Application.
    pub app_storageId: u8,
    /// **Ref:** NcmStorageId for the Application update.
    pub update_storageId: u8,
    /// Unknown.
    pub unk_xa: u8,
    /// Padding.
    pub pad: u8,
}

/// ApplicationLaunchRequestInfo
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletApplicationLaunchRequestInfo {
    /// Unknown. The default is 0x0 with `appletCreateSystemApplication`, 0x3 with `appletCreateApplication`.
    pub unk_x0: u32,
    /// Unknown. The default is 0x0 with `appletCreateSystemApplication`, 0x3 with `appletCreateApplication`.
    pub unk_x4: u32,
    /// Unknown. The default is 0x0.
    pub unk_x8: [u8; 8],
}

/// AppletResourceUsageInfo, from `appletGetAppletResourceUsageInfo`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppletResourceUsageInfo {
    /// Unknown counter.
    pub counter0: u32,
    /// Unknown counter.
    pub counter1: u32,
    /// Output from ns cmd GetRightsEnvironmentCountForDebug.
    pub counter2: u32,
    /// Always zero.
    pub unused: [u8; 20],
}

/// Gets the LibraryApplet StateChangedEvent.
///
/// # Arguments
/// - `h`: AppletHolder object.
#[inline(always)]
pub fn appletHolderGetExitEvent(h: &mut AppletHolder) -> &mut Event {
    &mut h.StateChangedEvent
}

impl_zeroed_default!(AppletHookCookie, AppletLockAccessor, AppletStorage, AppletHolder, AppletApplication, AppletGpuErrorHandler, AppletAttribute, LibAppletInfo, AppletProcessLaunchReason, AppletInfo, AppletIdentityInfo, AppletApplicationAttributeForQuest, AppletApplicationAttribute, AppletApplicationLaunchProperty, AppletApplicationLaunchRequestInfo, AppletResourceUsageInfo);

unsafe extern "C" {
    /// Initialize applet, called automatically during app startup.
    pub fn appletInitialize() -> Result;

    /// Exit applet, called automatically during app exit.
    pub fn appletExit();

    /// Gets the Service object for the actual "appletOE"/"appletAE" service session.
    pub fn appletGetServiceSession_Proxy() -> *mut Service;

    /// Gets the Service object for IAppletCommonFunctions. Only initialized with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [7.0.0+].
    pub fn appletGetServiceSession_AppletCommonFunctions() -> *mut Service;

    /// Gets the Service object for I*Functions, specific to each AppletType (IApplicationFunctions for AppletType_*Application). Not initialized with AppletType_LibraryApplet pre-15.0.0. On [15.0.0+] with AppletType_LibraryApplet this returns the object for IHomeMenuFunctions.
    pub fn appletGetServiceSession_Functions() -> *mut Service;

    /// Gets the Service object for IGlobalStateController. Only initialized with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletGetServiceSession_GlobalStateController() -> *mut Service;

    /// Gets the Service object for IApplicationCreator. Only initialized with AppletType_SystemApplet.
    pub fn appletGetServiceSession_ApplicationCreator() -> *mut Service;

    /// Gets the Service object for ILibraryAppletSelfAccessor. Only initialized with AppletType_LibraryApplet.
    pub fn appletGetServiceSession_LibraryAppletSelfAccessor() -> *mut Service;

    /// Gets the Service object for IProcessWindingController. Only initialized with AppletType_LibraryApplet.
    pub fn appletGetServiceSession_ProcessWindingController() -> *mut Service;

    /// Gets the Service object for ILibraryAppletCreator.
    pub fn appletGetServiceSession_LibraryAppletCreator() -> *mut Service;

    /// Gets the Service object for ICommonStateGetter.
    pub fn appletGetServiceSession_CommonStateGetter() -> *mut Service;

    /// Gets the Service object for ISelfController.
    pub fn appletGetServiceSession_SelfController() -> *mut Service;

    /// Gets the Service object for IWindowController.
    pub fn appletGetServiceSession_WindowController() -> *mut Service;

    /// Gets the Service object for IAudioController.
    pub fn appletGetServiceSession_AudioController() -> *mut Service;

    /// Gets the Service object for IDisplayController.
    pub fn appletGetServiceSession_DisplayController() -> *mut Service;

    /// Gets the Service object for IDebugFunctions.
    pub fn appletGetServiceSession_DebugFunctions() -> *mut Service;

    /// Get the cached AppletResourceUserId.
    pub fn appletGetAppletResourceUserId() -> u64;

    /// Get the `AppletType`.
    pub fn appletGetAppletType() -> AppletType;

    /// Sets the state field for `AppletThemeColorType`.
    pub fn appletSetThemeColorType(theme: AppletThemeColorType);

    /// Gets the state field for `AppletThemeColorType`. Used internally by `libappletArgsCreate`.
    pub fn appletGetThemeColorType() -> AppletThemeColorType;

    /// Gets the CradleStatus.
    ///
    /// # Arguments
    /// - `status` (out): Output Dock status.
    pub fn appletGetCradleStatus(status: *mut u8) -> Result;

    /// Gets the BootMode which originated from `pmbmGetBootMode`.
    ///
    /// # Arguments
    /// - `mode` (out): `PmBootMode`
    pub fn appletGetBootMode(mode: *mut PmBootMode) -> Result;

    /// Request to AcquireSleepLock.
    ///
    /// **Note:** On success, this then uses cmd GetAcquiredSleepLockEvent and waits on that event.
    pub fn appletRequestToAcquireSleepLock() -> Result;

    /// Release the SleepLock.
    pub fn appletReleaseSleepLock() -> Result;

    /// Release the SleepLock transiently.
    ///
    /// **Note:** On success, this then uses cmd GetAcquiredSleepLockEvent and waits on that event.
    pub fn appletReleaseSleepLockTransiently() -> Result;

    /// GetWakeupCount
    ///
    /// # Arguments
    /// - `out` (out): Output value.
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletGetWakeupCount(out: *mut u64) -> Result;

    /// Pushes a storage to the general channel. Used for sending requests to SystemApplet.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    ///
    /// **Note:** This is not usable under an Application, however it is usable under a LibraryApplet.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletPushToGeneralChannel(s: *mut AppletStorage) -> Result;

    /// Gets a `AppletLockAccessor` for HomeButtonReader.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    ///
    /// **Note:** Similar to using `appletGetReaderLockAccessorEx` with inval=0.
    pub fn appletGetHomeButtonReaderLockAccessor(a: *mut AppletLockAccessor) -> Result;

    /// Gets a Reader `AppletLockAccessor`.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    /// - `inval` (in): Input value, must be 0-3. 0 = HomeButton.
    ///
    /// **Note:** Only available with [2.0.0+].
    pub fn appletGetReaderLockAccessorEx(a: *mut AppletLockAccessor, inval: u32) -> Result;

    /// Gets a Writer `AppletLockAccessor`.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    /// - `inval` (in): Input value, must be 0-3. 0 = HomeButton.
    ///
    /// **Note:** Only available with [7.0.0+]. On older sysvers, this is only available with AppletType_SystemApplet on [2.0.0+].
    pub fn appletGetWriterLockAccessorEx(a: *mut AppletLockAccessor, inval: u32) -> Result;

    /// Gets the Dock firmware version.
    ///
    /// # Arguments
    /// - `out0` (out): First output value.
    /// - `out1` (out): Second output value.
    /// - `out2` (out): Third output value.
    /// - `out3` (out): Fourth output value.
    ///
    /// **Note:** Only available with [2.0.0+].
    pub fn appletGetCradleFwVersion(out0: *mut u32, out1: *mut u32, out2: *mut u32, out3: *mut u32) -> Result;

    /// Gets whether VrMode is enabled.
    ///
    /// # Arguments
    /// - `out`: Output flag
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletIsVrModeEnabled(out: *mut bool) -> Result;

    /// Sets whether VrMode is enabled.
    ///
    /// # Arguments
    /// - `flag`: Flag
    ///
    /// **Note:** This is only fully usable system-side with [6.0.0+].
    ///
    /// **Note:** For checking Parental Controls, see `pctlIsStereoVisionPermitted`.
    ///
    /// **Note:** On pre-7.0.0 this uses cmd SetVrModeEnabled internally, while on [7.0.0+] this uses cmds BeginVrModeEx/EndVrModeEx.
    pub fn appletSetVrModeEnabled(flag: bool) -> Result;

    /// Sets whether the LCD screen backlight is turned off.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletSetLcdBacklightOffEnabled(flag: bool) -> Result;

    /// Gets the ControllerFirmwareUpdateSection flag.
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletIsInControllerFirmwareUpdateSection(out: *mut bool) -> Result;

    /// SetVrPositionForDebug
    ///
    /// # Arguments
    /// - `x` (in): X, must not be negative. x+width must be &lt;=1280.
    /// - `y` (in): Y, must not be negative. y+height must be &lt;=720.
    /// - `width` (in): Width, must be 1-1280.
    /// - `height` (in): Height, must be 1-720.
    ///
    /// **Note:** The cached value loaded from `setsysGetDebugModeFlag` must be 1, otherwise an error is returned.
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletSetVrPositionForDebug(x: s32, y: s32, width: s32, height: s32) -> Result;

    /// Gets the DefaultDisplayResolution.
    ///
    /// # Arguments
    /// - `width` (out): Output width.
    /// - `height` (out): Output height.
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletGetDefaultDisplayResolution(width: *mut s32, height: *mut s32) -> Result;

    /// Gets an Event which is signaled when the output from `appletGetDefaultDisplayResolution` changes.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available with [3.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetDefaultDisplayResolutionChangeEvent(out_event: *mut Event) -> Result;

    /// Gets the HdcpAuthenticationState.
    ///
    /// # Arguments
    /// - `state` (out): Output state.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletGetHdcpAuthenticationState(state: *mut s32) -> Result;

    /// Gets an Event which is signaled when the output from `appletGetHdcpAuthenticationState` changes.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=true.
    ///
    /// **Note:** Only available with [4.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetHdcpAuthenticationStateChangeEvent(out_event: *mut Event) -> Result;

    /// Sets the `AppletTvPowerStateMatchingMode`.
    ///
    /// # Arguments
    /// - `mode` (in): `AppletTvPowerStateMatchingMode`
    ///
    /// **Note:** Only available with [5.0.0+].
    pub fn appletSetTvPowerStateMatchingMode(mode: AppletTvPowerStateMatchingMode) -> Result;

    /// Gets the ApplicationId for the specified ContentActionName string.
    ///
    /// # Arguments
    /// - `application_id` (out): ApplicationId.
    /// - `name` (in): ContentActionName string.
    ///
    /// **Note:** Only available when the current applet is an AppletType_SystemApplication on [5.1.0+].
    pub fn appletGetApplicationIdByContentActionName(application_id: *mut u64, name: *const c_char) -> Result;

    /// Sets the `ApmCpuBoostMode`.
    ///
    /// # Arguments
    /// - `mode`: `ApmCpuBoostMode`.
    ///
    /// **Note:** Only available with [7.0.0+] (not fully usable system-side with 6.x).
    pub fn appletSetCpuBoostMode(mode: ApmCpuBoostMode) -> Result;

    /// CancelCpuBoostMode
    ///
    /// **Note:** Only available with [10.0.0+].
    pub fn appletCancelCpuBoostMode() -> Result;

    /// GetBuiltInDisplayType
    ///
    /// # Arguments
    /// - `out` (out): Output value.
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletGetBuiltInDisplayType(out: *mut s32) -> Result;

    /// Perform SystemButtonPressing with the specified `AppletSystemButtonType`. Internally this cmd checks a state field, verifies that the type is allowed, then runs the same func as `appletPerformSystemButtonPressing` internally.
    ///
    /// # Arguments
    /// - `type` (in): `AppletSystemButtonType`
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn appletPerformSystemButtonPressingIfInFocus(type_: AppletSystemButtonType) -> Result;

    /// Sets whether PerformanceConfigurationChangedNotification is enabled.
    ///
    /// # Arguments
    /// - `flag` (in): Whether to enable the notification.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn appletSetPerformanceConfigurationChangedNotification(flag: bool) -> Result;

    /// Gets the current PerformanceConfiguration.
    ///
    /// # Arguments
    /// - `PerformanceConfiguration`: Output PerformanceConfiguration.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn appletGetCurrentPerformanceConfiguration(PerformanceConfiguration: *mut u32) -> Result;

    /// Opens an `AppletGpuErrorHandler`.
    ///
    /// # Arguments
    /// - `g` (out): `AppletGpuErrorHandler`
    ///
    /// **Note:** The cached value loaded from `setsysGetDebugModeFlag` must be 1, otherwise an error is returned.
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletOpenMyGpuErrorHandler(g: *mut AppletGpuErrorHandler) -> Result;

    /// Gets the OperationModeSystemInfo.
    ///
    /// # Arguments
    /// - `info` (out): Output info.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn appletGetOperationModeSystemInfo(info: *mut u32) -> Result;

    /// This uses `setsysGetPlatformRegion` internally.
    ///
    /// # Arguments
    /// - `out` (out): `SetSysPlatformRegion`
    ///
    /// **Note:** Only available with [9.0.0+].
    pub fn appletGetSettingsPlatformRegion(out: *mut SetSysPlatformRegion) -> Result;

    /// ActivateMigrationService
    ///
    /// **Note:** Only available with [10.0.0+].
    pub fn appletActivateMigrationService() -> Result;

    /// DeactivateMigrationService
    ///
    /// **Note:** Only available with [10.0.0+].
    pub fn appletDeactivateMigrationService() -> Result;

    /// DisableSleepTillShutdown
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletDisableSleepTillShutdown() -> Result;

    /// SuppressDisablingSleepTemporarily
    ///
    /// # Arguments
    /// - `val` (in): Nanoseconds value.
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletSuppressDisablingSleepTemporarily(val: u64) -> Result;

    /// SetRequestExitToLibraryAppletAtExecuteNextProgramEnabled
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletSetRequestExitToLibraryAppletAtExecuteNextProgramEnabled() -> Result;

    /// Close an `AppletGpuErrorHandler`.
    ///
    /// # Arguments
    /// - `g`: `AppletGpuErrorHandler`
    pub fn appletGpuErrorHandlerClose(g: *mut AppletGpuErrorHandler);

    /// Gets the size of the info available with `appletGpuErrorHandlerGetManualGpuErrorInfo`.
    ///
    /// # Arguments
    /// - `g`: `AppletGpuErrorHandler`
    /// - `out` (out): Output size.
    pub fn appletGpuErrorHandlerGetManualGpuErrorInfoSize(g: *mut AppletGpuErrorHandler, out: *mut u64) -> Result;

    /// GetManualGpuErrorInfo
    ///
    /// # Arguments
    /// - `g`: `AppletGpuErrorHandler`
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size, must be >= the output size from `appletGpuErrorHandlerGetManualGpuErrorInfoSize`.
    /// - `out` (out): Output value.
    pub fn appletGpuErrorHandlerGetManualGpuErrorInfo(g: *mut AppletGpuErrorHandler, buffer: *mut c_void, size: usize, out: *mut u64) -> Result;

    /// GetManualGpuErrorDetectionSystemEvent
    ///
    /// # Arguments
    /// - `g`: `AppletGpuErrorHandler`
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGpuErrorHandlerGetManualGpuErrorDetectionSystemEvent(g: *mut AppletGpuErrorHandler, out_event: *mut Event) -> Result;

    /// FinishManualGpuErrorHandling
    ///
    /// # Arguments
    /// - `g`: `AppletGpuErrorHandler`
    pub fn appletGpuErrorHandlerFinishManualGpuErrorHandling(g: *mut AppletGpuErrorHandler) -> Result;

    /// Delay exiting until `appletUnlockExit` is called, with a 15 second timeout once exit is requested.
    ///
    /// **Note:** When exit is requested `appletMainLoop` will return false, hence any main-loop using appletMainLoop will exit. This allows the app to handle cleanup post-main-loop instead of being force-terminated.
    ///
    /// **Note:** If the above timeout occurs after exit was requested where `appletUnlockExit` was not called, the process will be forced-terminated.
    ///
    /// **Note:** `appletUnlockExit` must be used before main() returns.
    pub fn appletLockExit() -> Result;

    /// Unlocks exiting, see `appletLockExit`.
    pub fn appletUnlockExit() -> Result;

    /// Enter FatalSection.
    pub fn appletEnterFatalSection() -> Result;

    /// Leave FatalSection.
    pub fn appletLeaveFatalSection() -> Result;

    /// Controls whether screenshot-capture is allowed.
    ///
    /// # Arguments
    /// - `permission`: `AppletScreenShotPermission`
    pub fn appletSetScreenShotPermission(permission: AppletScreenShotPermission) -> Result;

    /// Sets whether ::AppletMessage_Resume is enabled.
    ///
    /// # Arguments
    /// - `flag` (in): Whether to enable the notification.
    pub fn appletSetRestartMessageEnabled(flag: bool) -> Result;

    /// Sets the `AppletIdentityInfo` for screenshots.
    ///
    /// # Arguments
    /// - `info` (in): `AppletIdentityInfo`
    pub fn appletSetScreenShotAppletIdentityInfo(info: *mut AppletIdentityInfo) -> Result;

    /// Sets ControllerFirmwareUpdateSection.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with [3.0.0+].
    ///
    /// **Note:** This throws error 0x40280 when the internal state flag already matches the input value.
    pub fn appletSetControllerFirmwareUpdateSection(flag: bool) -> Result;

    /// Sets whether ::AppletMessage_CaptureButtonShortPressed is enabled.
    ///
    /// # Arguments
    /// - `flag` (in): Whether to enable the notification.
    ///
    /// **Note:** Only available with [3.0.0+].
    ///
    /// **Note:** When enabled with a non-Overlay applet, Overlay applet will not be notified of capture button short-presses for screenshots.
    pub fn appletSetRequiresCaptureButtonShortPressedMessage(flag: bool) -> Result;

    /// Sets the Album screenshot ImageOrientation.
    ///
    /// # Arguments
    /// - `orientation` (in): `AlbumImageOrientation`
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletSetAlbumImageOrientation(orientation: AlbumImageOrientation) -> Result;

    /// Sets the DesirableKeyboardLayout.
    ///
    /// # Arguments
    /// - `layout` (in): Input `SetKeyboardLayout`.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletSetDesirableKeyboardLayout(layout: SetKeyboardLayout) -> Result;

    pub fn appletCreateManagedDisplayLayer(out: *mut u64) -> Result;

    /// Checks whether SystemBufferSharing is enabled, throwing an error otherwise.
    ///
    /// **Note:** Only available with [4.0.0+]. Not usable with AppletType_*Application.
    pub fn appletIsSystemBufferSharingEnabled() -> Result;

    /// Gets the System SharedBufferHandle and SharedLayerHandle.
    ///
    /// # Arguments
    /// - `SharedBufferHandle` (out): Output System SharedBufferHandle.
    /// - `SharedLayerHandle` (out): Output System SharedLayerHandle.
    ///
    /// **Note:** Only available with [4.0.0+]. Not usable with AppletType_*Application.
    pub fn appletGetSystemSharedLayerHandle(SharedBufferHandle: *mut u64, SharedLayerHandle: *mut u64) -> Result;

    /// Same as `appletGetSystemSharedLayerHandle` except this just gets the SharedBufferHandle.
    ///
    /// # Arguments
    /// - `SharedBufferHandle` (out): Output System SharedBufferHandle.
    ///
    /// **Note:** Only available with [5.0.0+]. Not usable with AppletType_*Application.
    pub fn appletGetSystemSharedBufferHandle(SharedBufferHandle: *mut u64) -> Result;

    /// CreateManagedDisplaySeparableLayer
    ///
    /// # Arguments
    /// - `display_layer` (out): Output display_layer.
    /// - `recording_layer` (out): Output recording_layer.
    ///
    /// **Note:** Only available with [10.0.0+].
    pub fn appletCreateManagedDisplaySeparableLayer(display_layer: *mut u64, recording_layer: *mut u64) -> Result;

    /// SetManagedDisplayLayerSeparationMode
    ///
    /// # Arguments
    /// - `mode` (in): Mode. Must be 0-1.
    ///
    /// **Note:** Only available with [10.0.0+].
    pub fn appletSetManagedDisplayLayerSeparationMode(mode: u32) -> Result;

    /// Sets whether ::AppletMessage_RequestToDisplay is enabled.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Sets an internal state flag. When the input flag is 0, this will in additional run the same code as `appletApproveToDisplay`.
    pub fn appletSetHandlesRequestToDisplay(flag: bool) -> Result;

    /// Approve the display requested by ::AppletMessage_RequestToDisplay, see also `appletSetHandlesRequestToDisplay`.
    pub fn appletApproveToDisplay() -> Result;

    /// OverrideAutoSleepTimeAndDimmingTime
    ///
    /// # Arguments
    /// - `inval0` (in): Unknown input value.
    /// - `inval1` (in): Unknown input value.
    /// - `inval2` (in): Unknown input value.
    /// - `inval3` (in): Unknown input value.
    pub fn appletOverrideAutoSleepTimeAndDimmingTime(inval0: s32, inval1: s32, inval2: s32, inval3: s32) -> Result;

    /// Sets the IdleTimeDetectionExtension.
    ///
    /// # Arguments
    /// - `ext` (in): `AppletIdleTimeDetectionExtension` Must be 0-2: 0 = disabled, 1 = Extended, and 2 = ExtendedUnsafe.
    pub fn appletSetIdleTimeDetectionExtension(ext: AppletIdleTimeDetectionExtension) -> Result;

    /// Gets the value set by `appletSetIdleTimeDetectionExtension`.
    ///
    /// # Arguments
    /// - `ext` (out): `AppletIdleTimeDetectionExtension`
    pub fn appletGetIdleTimeDetectionExtension(ext: *mut AppletIdleTimeDetectionExtension) -> Result;

    /// Sets the InputDetectionSourceSet.
    ///
    /// # Arguments
    /// - `val` (in): Input value.
    pub fn appletSetInputDetectionSourceSet(val: u32) -> Result;

    /// Reports that the user is active, for idle detection (screen dimming / auto-sleep). This is equivalent to when the user uses HID input.
    ///
    /// **Note:** Only available with [2.0.0+].
    pub fn appletReportUserIsActive() -> Result;

    /// Gets the current Illuminance from the light sensor.
    ///
    /// # Arguments
    /// - `fLux`: Output fLux
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletGetCurrentIlluminance(fLux: *mut f32) -> Result;

    /// Gets whether Illuminance is available.
    ///
    /// # Arguments
    /// - `out`: Output flag
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletIsIlluminanceAvailable(out: *mut bool) -> Result;

    /// Sets AutoSleepDisabled.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with [5.0.0+].
    pub fn appletSetAutoSleepDisabled(flag: bool) -> Result;

    /// Gets AutoSleepDisabled.
    ///
    /// # Arguments
    /// - `out` (out): Output flag
    ///
    /// **Note:** Only available with [5.0.0+].
    pub fn appletIsAutoSleepDisabled(out: *mut bool) -> Result;

    /// Gets the current Illuminance from the light sensor. Same as `appletGetCurrentIlluminance` except for the additional param.
    ///
    /// # Arguments
    /// - `bOverLimit`: Output bOverLimit
    /// - `fLux`: Output fLux
    ///
    /// **Note:** Only available with [5.0.0+].
    pub fn appletGetCurrentIlluminanceEx(bOverLimit: *mut bool, fLux: *mut f32) -> Result;

    /// Sets the `AppletInputDetectionPolicy`.
    ///
    /// # Arguments
    /// - `policy` (in): `AppletInputDetectionPolicy`
    ///
    /// **Note:** Only available with [9.0.0+].
    pub fn appletSetInputDetectionPolicy(policy: AppletInputDetectionPolicy) -> Result;

    /// Sets the WirelessPriorityMode.
    ///
    /// # Arguments
    /// - `mode` (in): `AppletWirelessPriorityMode`
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletSetWirelessPriorityMode(mode: AppletWirelessPriorityMode) -> Result;

    /// Gets the total time in nanoseconds that the current process was actively running (not suspended), relative to when `appletInitialize` was last used.
    ///
    /// # Arguments
    /// - `activeTime` (out): Output nanoseconds value.
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn appletGetProgramTotalActiveTime(activeTime: *mut u64) -> Result;

    /// Sets whether ::AppletMessage_AlbumScreenShotTaken is enabled.
    ///
    /// # Arguments
    /// - `flag` (in): Whether to enable the notification.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn appletSetAlbumImageTakenNotificationEnabled(flag: bool) -> Result;

    /// Sets the Application AlbumUserData.
    ///
    /// # Arguments
    /// - `buffer` (in): Buffer containing arbitrary UserData.
    /// - `size` (in): Buffer size, must be &lt;=0x400.
    ///
    /// **Note:** Only available with [8.0.0+].
    pub fn appletSetApplicationAlbumUserData(buffer: *const c_void, size: usize) -> Result;

    /// SaveCurrentScreenshot
    ///
    /// # Arguments
    /// - `option` (in): `AlbumReportOption`
    ///
    /// **Note:** Only available with [11.0.0+].
    pub fn appletSaveCurrentScreenshot(option: AlbumReportOption) -> Result;

    /// Gets the AppletResourceUserId of the CallerApplet.
    ///
    /// # Arguments
    /// - `out` (out): AppletResourceUserId
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn appletGetAppletResourceUserIdOfCallerApplet(out: *mut u64) -> Result;

    /// Sets the current applet WindowVisibility.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn appletSetAppletWindowVisibility(flag: bool) -> Result;

    /// Sets the AppletGpuTimeSlice.
    ///
    /// # Arguments
    /// - `val` (in): Input value, must not be negative.
    ///
    /// **Note:** Only available with [7.0.0+].
    pub fn appletSetAppletGpuTimeSlice(val: s64) -> Result;

    /// Sets the ExpectedMasterVolume for MainApplet and LibraryApplet.
    ///
    /// # Arguments
    /// - `mainAppletVolume` (in): MainApplet ExpectedMasterVolume.
    /// - `libraryAppletVolume` (in): LibraryApplet ExpectedMasterVolume.
    ///
    /// **Note:** Used by some official apps before/after launching LibraryApplets. Prior to changing the volume, the official app uses `appletGetExpectedMasterVolume`, with the output being used to restore the volume after LibraryApplet handling.
    pub fn appletSetExpectedMasterVolume(mainAppletVolume: f32, libraryAppletVolume: f32) -> Result;

    /// Gets the ExpectedMasterVolume for MainApplet and LibraryApplet.
    ///
    /// # Arguments
    /// - `mainAppletVolume` (out): MainApplet ExpectedMasterVolume. Optional, can be NULL. Used with cmd GetMainAppletExpectedMasterVolume when not NULL.
    /// - `libraryAppletVolume` (out): LibraryApplet ExpectedMasterVolume. Optional, can be NULL. Used with cmd GetLibraryAppletExpectedMasterVolume when not NULL.
    ///
    /// **Note:** See also `appletSetExpectedMasterVolume`.
    pub fn appletGetExpectedMasterVolume(mainAppletVolume: *mut f32, libraryAppletVolume: *mut f32) -> Result;

    /// Change the MainApplet MasterVolume.
    ///
    /// # Arguments
    /// - `volume` (in): MainApplet MasterVolume.
    /// - `unk` (in): Unknown.
    pub fn appletChangeMainAppletMasterVolume(volume: f32, unk: u64) -> Result;

    /// Sets the TransparentVolumeRate.
    ///
    /// # Arguments
    /// - `val` (in): Input value.
    pub fn appletSetTransparentVolumeRate(val: f32) -> Result;

    /// Update the LastForeground CaptureImage.
    pub fn appletUpdateLastForegroundCaptureImage() -> Result;

    /// Update the CallerApplet CaptureImage.
    pub fn appletUpdateCallerAppletCaptureImage() -> Result;

    /// Gets the LastForeground CaptureImage.
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer containing the 1280x720 RGBA8 image.
    /// - `size` (out): Buffer size, must match 0x384000.
    /// - `flag` (out): Output flag.
    pub fn appletGetLastForegroundCaptureImageEx(buffer: *mut c_void, size: usize, flag: *mut bool) -> Result;

    /// Gets the LastApplication CaptureImage.
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer containing the 1280x720 RGBA8 image.
    /// - `size` (out): Buffer size, must match 0x384000.
    /// - `flag` (out): Output flag.
    pub fn appletGetLastApplicationCaptureImageEx(buffer: *mut c_void, size: usize, flag: *mut bool) -> Result;

    /// Gets the CallerApplet CaptureImage.
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer containing the 1280x720 RGBA8 image.
    /// - `size` (out): Buffer size, must match 0x384000.
    /// - `flag` (out): Output flag.
    pub fn appletGetCallerAppletCaptureImageEx(buffer: *mut c_void, size: usize, flag: *mut bool) -> Result;

    /// Takes a screenshot of the current applet Layer into the specified CaptureSharedBuffer.
    ///
    /// # Arguments
    /// - `flag` (in): Flag.
    /// - `captureBuf` (in): `AppletCaptureSharedBuffer`
    ///
    /// **Note:** Only available with [2.0.0+].
    pub fn appletTakeScreenShotOfOwnLayer(flag: bool, captureBuf: AppletCaptureSharedBuffer) -> Result;

    /// Copies image data from a CaptureSharedBuffer to another CaptureSharedBuffer.
    ///
    /// # Arguments
    /// - `dstCaptureBuf` (in): Destination `AppletCaptureSharedBuffer`.
    /// - `srcCaptureBuf` (in): Source `AppletCaptureSharedBuffer`.
    ///
    /// **Note:** Only available with [5.0.0+].
    pub fn appletCopyBetweenCaptureBuffers(dstCaptureBuf: AppletCaptureSharedBuffer, srcCaptureBuf: AppletCaptureSharedBuffer) -> Result;

    /// Clear the input CaptureSharedBuffer with the specified color.
    ///
    /// # Arguments
    /// - `flag` (in): Flag.
    /// - `captureBuf` (in): `AppletCaptureSharedBuffer`
    /// - `color` (in): RGBA8 color.
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletClearCaptureBuffer(flag: bool, captureBuf: AppletCaptureSharedBuffer, color: u32) -> Result;

    /// Clear the AppletTransitionBuffer with the specified color.
    ///
    /// # Arguments
    /// - `color` (in): RGBA8 color.
    ///
    /// **Note:** Only available with [3.0.0+].
    pub fn appletClearAppletTransitionBuffer(color: u32) -> Result;

    /// Acquire the LastApplication CaptureSharedBuffer.
    ///
    /// # Arguments
    /// - `flag` (out): Output flag.
    /// - `id` (out): Output ID.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletAcquireLastApplicationCaptureSharedBuffer(flag: *mut bool, id: *mut s32) -> Result;

    /// Release the LastApplication CaptureSharedBuffer.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletReleaseLastApplicationCaptureSharedBuffer() -> Result;

    /// Acquire the LastForeground CaptureSharedBuffer.
    ///
    /// # Arguments
    /// - `flag` (out): Output flag.
    /// - `id` (out): Output ID.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletAcquireLastForegroundCaptureSharedBuffer(flag: *mut bool, id: *mut s32) -> Result;

    /// Release the LastForeground CaptureSharedBuffer.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletReleaseLastForegroundCaptureSharedBuffer() -> Result;

    /// Acquire the CallerApplet CaptureSharedBuffer.
    ///
    /// # Arguments
    /// - `flag` (out): Output flag.
    /// - `id` (out): Output ID.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletAcquireCallerAppletCaptureSharedBuffer(flag: *mut bool, id: *mut s32) -> Result;

    /// Release the CallerApplet CaptureSharedBuffer.
    ///
    /// **Note:** Only available with [4.0.0+].
    pub fn appletReleaseCallerAppletCaptureSharedBuffer() -> Result;

    /// Takes a screenshot of the current applet Layer into the specified CaptureSharedBuffer. Same as `appletTakeScreenShotOfOwnLayer` except for the additional immediately param.
    ///
    /// # Arguments
    /// - `flag0` (in): Flag0.
    /// - `immediately` (in): Whether the screenshot should be taken immediately.
    /// - `captureBuf` (in): `AppletCaptureSharedBuffer`
    ///
    /// **Note:** Only available with [6.0.0+].
    pub fn appletTakeScreenShotOfOwnLayerEx(flag0: bool, immediately: bool, captureBuf: AppletCaptureSharedBuffer) -> Result;

    /// Pushes a storage to the ContextStack. Normally this should only be used when AppletInfo::caller_flag is true.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletPushContext(s: *mut AppletStorage) -> Result;

    /// Pops a storage from the ContextStack. Normally this should only be used when AppletInfo::caller_flag is true.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletPopContext(s: *mut AppletStorage) -> Result;

    /// Closes a LockAccessor.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    pub fn appletLockAccessorClose(a: *mut AppletLockAccessor);

    /// TryLock a LockAccessor.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    /// - `flag` (out): Whether locking was successful, when false this indicates that this func should be called again.
    pub fn appletLockAccessorTryLock(a: *mut AppletLockAccessor, flag: *mut bool) -> Result;

    /// Lock a LockAccessor.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    ///
    /// **Note:** Similar to `appletLockAccessorTryLock`, except this uses timeout UINT64_MAX with the eventWait call, and this uses TryLock repeatedly until the output flag value is true.
    pub fn appletLockAccessorLock(a: *mut AppletLockAccessor) -> Result;

    /// Unlock a LockAccessor.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    pub fn appletLockAccessorUnlock(a: *mut AppletLockAccessor) -> Result;

    /// Creates a LibraryApplet.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `id`: See `AppletId`.
    /// - `mode`: See `LibAppletMode`.
    pub fn appletCreateLibraryApplet(h: *mut AppletHolder, id: AppletId, mode: LibAppletMode) -> Result;

    /// Creates a LibraryApplet. This is for when a LibraryApplet creates itself.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `id`: See `AppletId`.
    /// - `mode`: See `LibAppletMode`.
    ///
    /// **Note:** Identical to `appletCreateLibraryApplet` except this sets the creating_self flag to true.
    pub fn appletCreateLibraryAppletSelf(h: *mut AppletHolder, id: AppletId, mode: LibAppletMode) -> Result;

    /// TerminateAllLibraryApplets which were created by the current applet.
    ///
    /// **Note:** Normally LibraryApplet cleanup should be handled via `AppletHolder`.
    pub fn appletTerminateAllLibraryApplets() -> Result;

    /// AreAnyLibraryAppletsLeft which were created by the current applet.
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    pub fn appletAreAnyLibraryAppletsLeft(out: *mut bool) -> Result;

    /// Closes an AppletHolder object.
    pub fn appletHolderClose(h: *mut AppletHolder);

    /// Returns whether the AppletHolder object was initialized.
    pub fn appletHolderActive(h: *mut AppletHolder) -> bool;

    /// Gets the IndirectLayerConsumerHandle loaded during `appletCreateLibraryApplet`, on [2.0.0+].
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `out`: Output IndirectLayerConsumerHandle.
    ///
    /// **Note:** Only available when `LibAppletMode` is ::LibAppletMode_BackgroundIndirect.
    pub fn appletHolderGetIndirectLayerConsumerHandle(h: *mut AppletHolder, out: *mut u64) -> Result;

    /// Starts the LibraryApplet.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    pub fn appletHolderStart(h: *mut AppletHolder) -> Result;

    /// Jumps to the LibraryApplet, with the current-LibraryApplet being terminated. This will enter an infinite-sleep-loop on success.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletHolderJump(h: *mut AppletHolder) -> Result;

    /// Requests the LibraryApplet to exit. The command is only used if `appletHolderCheckFinished` returns false.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    pub fn appletHolderRequestExit(h: *mut AppletHolder) -> Result;

    /// Terminate the LibraryApplet.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    pub fn appletHolderTerminate(h: *mut AppletHolder) -> Result;

    /// Uses cmds GetAppletStateChangedEvent and RequestExit, then waits for the LibraryApplet to exit with the specified timeout. If a timeout occurs, the Terminate cmd is used.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `timeout` (in): Timeout in nanoseconds. UINT64_MAX for no timeout.
    pub fn appletHolderRequestExitOrTerminate(h: *mut AppletHolder, timeout: u64) -> Result;

    /// Waits for the LibraryApplet to exit.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    pub fn appletHolderJoin(h: *mut AppletHolder);

    /// Waits on the LibraryApplet StateChangedEvent with timeout=0, and returns whether it was successful.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    pub fn appletHolderCheckFinished(h: *mut AppletHolder) -> bool;

    /// Gets the `LibAppletExitReason` set by `appletHolderJoin`.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    pub fn appletHolderGetExitReason(h: *mut AppletHolder) -> LibAppletExitReason;

    /// Sets OutOfFocusApplicationSuspendingEnabled.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with AppletType_*Application.
    pub fn appletHolderSetOutOfFocusApplicationSuspendingEnabled(h: *mut AppletHolder, flag: bool) -> Result;

    /// PresetLibraryAppletGpuTimeSliceZero
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    ///
    /// **Note:** Only available with [10.0.0+].
    pub fn appletHolderPresetLibraryAppletGpuTimeSliceZero(h: *mut AppletHolder) -> Result;

    /// Gets the PopInteractiveOutDataEvent.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `out_event` (out): Output Event.
    pub fn appletHolderGetPopInteractiveOutDataEvent(h: *mut AppletHolder, out_event: *mut *mut Event) -> Result;

    /// Waits for the PopInteractiveOutDataEvent and StateChangedEvent.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    ///
    /// **Returns:** false for error / when StateChangedEvent was signaled, and true when PopInteractiveOutDataEvent was signaled. The latter is signaled when a new storage is available with `appletHolderPopInteractiveOutData` where previously no storage was available (this willl not clear the event), this event is automatically cleared by the system once the last storage is popped.
    pub fn appletHolderWaitInteractiveOut(h: *mut AppletHolder) -> bool;

    /// Pushes a storage for LibraryApplet input.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `s` (in): Storage object.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletHolderPushInData(h: *mut AppletHolder, s: *mut AppletStorage) -> Result;

    /// Pops a storage from LibraryApplet output.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `s` (out): Storage object.
    pub fn appletHolderPopOutData(h: *mut AppletHolder, s: *mut AppletStorage) -> Result;

    /// Pushes a storage for LibraryApplet Extra storage input.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `s` (in): Storage object.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletHolderPushExtraStorage(h: *mut AppletHolder, s: *mut AppletStorage) -> Result;

    /// Pushes a storage for LibraryApplet Interactive input.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `s` (in): Storage object.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletHolderPushInteractiveInData(h: *mut AppletHolder, s: *mut AppletStorage) -> Result;

    /// Pops a storage from LibraryApplet Interactive output.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `s` (out): Storage object.
    pub fn appletHolderPopInteractiveOutData(h: *mut AppletHolder, s: *mut AppletStorage) -> Result;

    /// Gets the `LibAppletInfo` for the specified LibraryApplet.
    ///
    /// # Arguments
    /// - `h`: AppletHolder object.
    /// - `info` (out): `LibAppletInfo`
    pub fn appletHolderGetLibraryAppletInfo(h: *mut AppletHolder, info: *mut LibAppletInfo) -> Result;

    /// Creates a storage.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `size`: Size of storage.
    pub fn appletCreateStorage(s: *mut AppletStorage, size: s64) -> Result;

    /// Creates a TransferMemory storage.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `buffer`: TransferMemory buffer, will be automatically allocated if NULL.
    /// - `size`: Size of storage.
    /// - `writable`: Controls whether writing to the storage is allowed with `appletStorageWrite`.
    pub fn appletCreateTransferMemoryStorage(s: *mut AppletStorage, buffer: *mut c_void, size: s64, writable: bool) -> Result;

    /// Creates a HandleStorage.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `inval`: Arbitrary input value.
    /// - `handle`: Arbitrary input handle.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn appletCreateHandleStorage(s: *mut AppletStorage, inval: s64, handle: Handle) -> Result;

    /// Creates a HandleStorage using TransferMemory. Wrapper for `appletCreateHandleStorage`.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `buffer`: TransferMemory buffer, will be automatically allocated if NULL.
    /// - `size`: Size of storage.
    pub fn appletCreateHandleStorageTmem(s: *mut AppletStorage, buffer: *mut c_void, size: s64) -> Result;

    /// Closes the storage object. TransferMemory closing is seperate, see `appletStorageCloseTmem`.
    /// Other applet functions which push an input storage will automatically call this.
    pub fn appletStorageClose(s: *mut AppletStorage);

    /// Closes the TransferMemory in the storage object. For TransferMemory storage created by the current process, this must be called after the LibraryApplet finishes using it (if sent to one).
    pub fn appletStorageCloseTmem(s: *mut AppletStorage);

    /// Gets the size of the storage. This is not usable with HandleStorage, use `appletStorageGetHandle` or `appletStorageMap` instead for that.
    pub fn appletStorageGetSize(s: *mut AppletStorage, size: *mut s64) -> Result;

    /// Writes to a storage. offset(+size) must be within the actual storage size.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `offset`: Offset in storage.
    /// - `buffer`: Input data.
    /// - `size`: Data size.
    ///
    /// **Note:** This is not usable with HandleStorage.
    pub fn appletStorageWrite(s: *mut AppletStorage, offset: s64, buffer: *const c_void, size: usize) -> Result;

    /// Reads from a storage. offset(+size) must be within the actual storage size.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `offset`: Offset in storage.
    /// - `buffer`: Input data.
    /// - `size`: Data size.
    ///
    /// **Note:** This is not usable with HandleStorage.
    pub fn appletStorageRead(s: *mut AppletStorage, offset: s64, buffer: *mut c_void, size: usize) -> Result;

    /// Gets data for a HandleStorage originally from `appletCreateHandleStorage` input.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `out`: Output value.
    /// - `handle`: Output handle.
    ///
    /// **Note:** Only available on [2.0.0+].
    pub fn appletStorageGetHandle(s: *mut AppletStorage, out: *mut s64, handle: *mut Handle) -> Result;

    /// Maps TransferMemory for a HandleStorage. Wrapper for `appletCreateHandleStorage`.
    ///
    /// # Arguments
    /// - `s`: Storage object.
    /// - `addr`: Output mapped address (optional).
    /// - `size`: Output size (optional).
    ///
    /// **Note:** The TransferMemory can be unmapped with `appletStorageCloseTmem`.
    ///
    /// **Note:** Do not use this if the AppletStorage already contains initialized TransferMemory state.
    pub fn appletStorageMap(s: *mut AppletStorage, addr: *mut *mut c_void, size: *mut usize) -> Result;

    /// Pops a LaunchParameter AppletStorage, the storage will be removed from sysmodule state during this.
    ///
    /// # Arguments
    /// - `s` (out): Output storage.
    /// - `kind`: See `AppletLaunchParameterKind`.
    ///
    /// **Note:** Only available with AppletType_*Application.
    ///
    /// **Note:** See also acc.h `accountGetPreselectedUser` (wrapper for appletPopLaunchParameter etc).
    pub fn appletPopLaunchParameter(s: *mut AppletStorage, kind: AppletLaunchParameterKind) -> Result;

    /// Requests to launch the specified application.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId. Value 0 can be used to relaunch the current application.
    /// - `s` (in): Optional AppletStorage object, can be NULL. This is automatically closed. When NULL on pre-4.0.0 (or with AppletType_LibraryApplet), this will internally create a tmp storage with size 0 for use with the cmd. This is the storage available to the launched application via `appletPopLaunchParameter` with ::AppletLaunchParameterKind_UserChannel.
    ///
    /// **Note:** Only available with AppletType_*Application, or AppletType_LibraryApplet on [5.0.0+].
    pub fn appletRequestLaunchApplication(application_id: u64, s: *mut AppletStorage) -> Result;

    /// Requests to launch the specified application, for kiosk systems.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId
    /// - `s` (in): Optional AppletStorage object, can be NULL. This is automatically closed. When NULL on pre-4.0.0, this will internally create a tmp storage with size 0 for use with the cmd. This is the storage available to the launched application via `appletPopLaunchParameter` with ::AppletLaunchParameterKind_UserChannel.
    /// - `attr` (in): Kiosk application attributes.
    ///
    /// **Note:** Only available with AppletType_*Application on [3.0.0+].
    ///
    /// **Note:** Identical to `appletRequestLaunchApplication`, except this allows the user to specify the attribute fields instead of the defaults being used.
    pub fn appletRequestLaunchApplicationForQuest(application_id: u64, s: *mut AppletStorage, attr: *const AppletApplicationAttributeForQuest) -> Result;

    /// Gets the DesiredLanguage for the current host application control.nacp.
    ///
    /// # Arguments
    /// - `LanguageCode` (out): Output LanguageCode, see set.h.
    ///
    /// **Note:** Only available with AppletType_*Application.
    pub fn appletGetDesiredLanguage(LanguageCode: *mut u64) -> Result;

    /// Gets the DisplayVersion for the current host application control.nacp.
    ///
    /// # Arguments
    /// - `displayVersion` (out): Output DisplayVersion string, must be at least 0x10-bytes. This is always NUL-terminated.
    ///
    /// **Note:** Only available with AppletType_*Application.
    pub fn appletGetDisplayVersion(displayVersion: *mut c_char) -> Result;

    /// Blocks the usage of the home button, for short (Home Menu) and long (Overlay) presses.
    ///
    /// # Arguments
    /// - `val`: Unknown. Official sw only uses hard-coded value 0 for this.
    ///
    /// **Note:** Only available with AppletType_*Application.
    pub fn appletBeginBlockingHomeButtonShortAndLongPressed(val: s64) -> Result;

    /// Ends the blocking started by `appletBeginBlockingHomeButtonShortAndLongPressed`.
    ///
    /// **Note:** Only available with AppletType_*Application.
    pub fn appletEndBlockingHomeButtonShortAndLongPressed() -> Result;

    /// Blocks the usage of the home button, for short presses (Home Menu).
    ///
    /// # Arguments
    /// - `val`: Unknown nanoseconds. Value 0 can be used.
    ///
    /// **Note:** Only available with AppletType_*Application.
    pub fn appletBeginBlockingHomeButton(val: s64) -> Result;

    /// Ends the blocking started by `appletBeginBlockingHomeButton`.
    ///
    /// **Note:** Only available with AppletType_*Application.
    pub fn appletEndBlockingHomeButton() -> Result;

    /// Notify that the app is now running, for the Application logo screen. This throws a fatal-error on failure.
    ///
    /// **Note:** This will just return when applet-type isn't AppletType_Application, or when this was already used previously. Used automatically by `appletInitialize` when __nx_applet_auto_notifyrunning is set to true (the default value).
    pub fn appletNotifyRunning(out: *mut bool);

    /// Gets the PseudoDeviceId. This is derived from the output of a ns command, and from data in the host application control.nacp.
    ///
    /// # Arguments
    /// - `out` (out): Output PseudoDeviceId.
    ///
    /// **Note:** Only available with AppletType_*Application on [2.0.0+].
    pub fn appletGetPseudoDeviceId(out: *mut Uuid) -> Result;

    /// Set media playback state.
    /// If state is set to true, screen dimming and auto sleep is disabled.
    /// For *Application, this uses cmd SetMediaPlaybackStateForApplication, otherwise cmd SetMediaPlaybackState is used.
    pub fn appletSetMediaPlaybackState(state: bool) -> Result;

    /// Gets whether video recording is supported.
    /// See also `appletInitializeGamePlayRecording`.
    pub fn appletIsGamePlayRecordingSupported(flag: *mut bool) -> Result;

    /// Disable/enable video recording. Only available after `appletInitializeGamePlayRecording` was used.
    /// See also `appletInitializeGamePlayRecording`.
    pub fn appletSetGamePlayRecordingState(state: bool) -> Result;

    /// Initializes video recording. This allocates a 0x6000000-byte buffer for the TransferMemory, cleanup is handled automatically during app exit in `appletExit`.
    /// Only available with AppletType_Application on [3.0.0+], hence errors from this can be ignored.
    /// Video recording is only fully available system-side with [4.0.0+].
    /// Only usable when running under an application which supports video recording. Using this is only needed when the host application control.nacp has VideoCaptureMode set to Enabled, with Automatic appletInitializeGamePlayRecording is not needed.
    pub fn appletInitializeGamePlayRecording() -> Result;

    /// Requests to save the video recording, as if the Capture-button was held.
    ///
    /// **Note:** Only available with AppletType_*Application on [4.0.0+].
    pub fn appletRequestFlushGamePlayingMovieForDebug() -> Result;

    /// Requests a system shutdown. This will enter an infinite-sleep-loop on success.
    ///
    /// **Note:** Only available with AppletType_*Application on [3.0.0+].
    pub fn appletRequestToShutdown() -> Result;

    /// Requests a system reboot. This will enter an infinite-sleep-loop on success.
    ///
    /// **Note:** Only available with AppletType_*Application on [3.0.0+].
    pub fn appletRequestToReboot() -> Result;

    /// RequestToSleep
    ///
    /// **Note:** Only available with AppletType_*Application on [10.0.0+].
    pub fn appletRequestToSleep() -> Result;

    /// Exit the application and return to the kiosk demo menu. This terminates the current process. This will enter an infinite-sleep-loop on success.
    ///
    /// **Note:** Only available with AppletType_*Application on [4.0.0+], on kiosk systems (QuestFlag set).
    pub fn appletExitAndRequestToShowThanksMessage() -> Result;

    /// Initializes the ApplicationCopyrightFrameBuffer, with dimensions 1280x720 + the tmem for it. This is used as an overlay for screenshots.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    ///
    /// **Note:** Cleanup for this is handled automatically during app exit in `appletExit`.
    pub fn appletInitializeApplicationCopyrightFrameBuffer() -> Result;

    /// Sets the RGBA8 image for use with `appletInitializeApplicationCopyrightFrameBuffer`. Overrides the current image, if this was already used previously.
    ///
    /// # Arguments
    /// - `buffer` (in): Input image buffer.
    /// - `size` (in): Input image buffer size.
    /// - `x` (in): X coordinate. Must not be negative.
    /// - `y` (in): Y coordinate. Must not be negative.
    /// - `width` (in): Image width. Must be >=1.
    /// - `height` (in): Image height. Must be >=1.
    /// - `mode` (in): `AppletWindowOriginMode`
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    ///
    /// **Note:** The specified coordinates and width/height must be within the bounds of the framebuffer setup by `appletInitializeApplicationCopyrightFrameBuffer`.
    pub fn appletSetApplicationCopyrightImage(buffer: *const c_void, size: usize, x: s32, y: s32, width: s32, height: s32, mode: AppletWindowOriginMode) -> Result;

    /// Sets the visibility for the image set by `appletSetApplicationCopyrightImage`, in screenshots.
    ///
    /// # Arguments
    /// - `visible` (in): Whether the image is visible. The default is true.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    pub fn appletSetApplicationCopyrightVisibility(visible: bool) -> Result;

    /// Gets ApplicationPlayStatistics.
    ///
    /// # Arguments
    /// - `stats`: Output `PdmApplicationPlayStatistics` array.
    /// - `application_ids`: Input ApplicationIds array.
    /// - `count`: Total entries in the input/output arrays.
    /// - `total_out`: Total output entries.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    ///
    /// **Note:** The input ApplicationIds must be allowed via control.nacp with the current host application. The minimum allowed ApplicationId is the ApplicationId for the current application.
    pub fn appletQueryApplicationPlayStatistics(stats: *mut PdmApplicationPlayStatistics, application_ids: *const u64, count: s32, total_out: *mut s32) -> Result;

    /// Same as `appletQueryApplicationPlayStatistics` except this gets playstats specific to the input userId.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `stats` (out): Output `PdmApplicationPlayStatistics` array.
    /// - `application_ids` (in): Input ApplicationIds array.
    /// - `count` (in): Total entries in the input/output arrays.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available with AppletType_*Application on [6.0.0+].
    pub fn appletQueryApplicationPlayStatisticsByUid(uid: AccountUid, stats: *mut PdmApplicationPlayStatistics, application_ids: *const u64, count: s32, total_out: *mut s32) -> Result;

    /// Launches Application {current_ApplicationId}+programIndex. This will enter an infinite-sleep-loop on success.
    ///
    /// # Arguments
    /// - `programIndex` (in): ProgramIndex, must be 0x0-0xFF. 0 is the same as the current application. ProgramIndex values where the application is not installed should not be used.
    /// - `buffer` (in): Optional buffer containing the storage data which will be used for ::AppletLaunchParameterKind_UserChannel with the launched Application, can be NULL.
    /// - `size` (in): Size of the above buffer, 0 to not use the storage. Must be &lt;=0x1000.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    ///
    /// **Note:** Creates the storage if needed. Uses cmd ClearUserChannel. Uses cmd UnpopToUserChannel when the storage was created. Lastly cmd ExecuteProgramCmd is used.
    pub fn appletExecuteProgram(programIndex: s32, buffer: *const c_void, size: usize) -> Result;

    /// Launches the specified ApplicationId.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `buffer` (in): Optional buffer containing the storage data which will be used for ::AppletLaunchParameterKind_UserChannel with the launched Application, can be NULL.
    /// - `size` (in): Size of the above buffer, 0 to not use the storage. Must be &lt;=0x1000.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+], with DebugMode enabled.
    ///
    /// **Note:** Creates the storage if needed. Uses cmd ClearUserChannel. Uses cmd UnpopToUserChannel when the storage was created. Lastly cmd ExecuteProgramCmd is used.
    pub fn appletJumpToSubApplicationProgramForDevelopment(application_id: u64, buffer: *const c_void, size: usize) -> Result;

    /// Relaunches the current Application.
    ///
    /// # Arguments
    /// - `buffer` (in): Optional buffer containing the storage data which will be used for ::AppletLaunchParameterKind_UserChannel with the launched Application, can be NULL.
    /// - `size` (in): Size of the above buffer, 0 to not use the storage. Must be &lt;=0x1000.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    ///
    /// **Note:** Creates the storage if needed. Uses cmd ClearUserChannel. Uses cmd UnpopToUserChannel when the storage was created. Lastly cmd ExecuteProgramCmd is used.
    pub fn appletRestartProgram(buffer: *const c_void, size: usize) -> Result;

    /// Gets the ProgramIndex of the program which launched this program.
    ///
    /// # Arguments
    /// - `programIndex` (out): ProgramIndex, -1 when there was no previous program.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    pub fn appletGetPreviousProgramIndex(programIndex: *mut s32) -> Result;

    /// SetDelayTimeToAbortOnGpuError
    ///
    /// # Arguments
    /// - `val` (in): Input nanoseconds value.
    ///
    /// **Note:** Only available with AppletType_*Application on [11.0.0+].
    pub fn appletSetDelayTimeToAbortOnGpuError(val: u64) -> Result;

    /// Gets an Event which is signaled when a new storage is available with `appletTryPopFromFriendInvitationStorageChannel` where previously no storage was available, this event is automatically cleared by the system once the last storage is popped.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** This is used by `friendsGetFriendInvitationNotificationEvent`.
    ///
    /// **Note:** Only available with AppletType_*Application on [9.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetFriendInvitationStorageChannelEvent(out_event: *mut Event) -> Result;

    /// Pops a storage from the FriendInvitation StorageChannel.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** This is used by `friendsTryPopFriendInvitationNotificationInfo`.
    ///
    /// **Note:** Only available with AppletType_*Application on [9.0.0+].
    pub fn appletTryPopFromFriendInvitationStorageChannel(s: *mut AppletStorage) -> Result;

    /// Gets an Event which is signaled when a new storage is available with `appletTryPopFromNotificationStorageChannel` where previously no storage was available, this event is automatically cleared by the system once the last storage is popped.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** This is used by `notifGetNotificationSystemEvent`.
    ///
    /// **Note:** Only available with AppletType_*Application on [9.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetNotificationStorageChannelEvent(out_event: *mut Event) -> Result;

    /// Pops a storage from the Notification StorageChannel.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** This is used by `notifTryPopNotifiedApplicationParameter`.
    ///
    /// **Note:** Only available with AppletType_*Application on [9.0.0+].
    pub fn appletTryPopFromNotificationStorageChannel(s: *mut AppletStorage) -> Result;

    /// GetHealthWarningDisappearedSystemEvent
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with AppletType_*Application on [9.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetHealthWarningDisappearedSystemEvent(out_event: *mut Event) -> Result;

    /// SetHdcpAuthenticationActivated
    ///
    /// # Arguments
    /// - `flag` (in): Whether HdcpAuthentication is activated.
    ///
    /// **Note:** Only available with AppletType_*Application on [9.0.0+].
    pub fn appletSetHdcpAuthenticationActivated(flag: bool) -> Result;

    /// GetLastApplicationExitReason
    ///
    /// # Arguments
    /// - `out` (out): Output value.
    ///
    /// **Note:** Only available with AppletType_*Application on [11.0.0+].
    pub fn appletGetLastApplicationExitReason(out: *mut s32) -> Result;

    /// CreateMovieMaker. Do not use this directly, use `grcCreateMovieMaker` instead.
    ///
    /// # Arguments
    /// - `srv_out` (out): Output Service for applet IMovieMaker.
    /// - `tmem` (in): TransferMemory
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    pub fn appletCreateMovieMaker(srv_out: *mut Service, tmem: *mut TransferMemory) -> Result;

    /// Launches the jit-sysmodule when it was not previously launched by this cmd. Returns 0 when it was previously launched.
    ///
    /// **Note:** Only available with AppletType_*Application on [5.0.0+].
    ///
    /// **Note:** Requires the jit-sysmodule to actually be installed.
    pub fn appletPrepareForJit() -> Result;

    /// RequestToGetForeground
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletRequestToGetForeground() -> Result;

    /// LockForeground
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletLockForeground() -> Result;

    /// UnlockForeground
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletUnlockForeground() -> Result;

    /// Pops a storage from the general channel.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletPopFromGeneralChannel(s: *mut AppletStorage) -> Result;

    /// Gets an Event which is signaled when a new storage is available with `appletPopFromGeneralChannel` where previously no storage was available, this event is automatically cleared by the system once the last storage is popped.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetPopFromGeneralChannelEvent(out_event: *mut Event) -> Result;

    /// Gets a `AppletLockAccessor` for HomeButtonWriter.
    ///
    /// # Arguments
    /// - `a`: LockAccessor object.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet.
    ///
    /// **Note:** Similar to using `appletGetWriterLockAccessorEx` with inval=0.
    pub fn appletGetHomeButtonWriterLockAccessor(a: *mut AppletLockAccessor) -> Result;

    /// IsSleepEnabled
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [11.0.0+], or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletIsSleepEnabled(out: *mut bool) -> Result;

    /// PopRequestLaunchApplicationForDebug
    ///
    /// # Arguments
    /// - `uids` (out): Output array of `AccountUid`.
    /// - `count` (in): Size of the uids array in entries, must be at least the size stored in state.
    /// - `application_id` (out): Output ApplicationId.
    /// - `total_out` (out): Total output userID entries.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [6.0.0+], or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletPopRequestLaunchApplicationForDebug(uids: *mut AccountUid, count: s32, application_id: *mut u64, total_out: *mut s32) -> Result;

    /// IsForceTerminateApplicationDisabledForDebug
    ///
    /// # Arguments
    /// - `out` (out): Output flag. 0 when DebugMode is not enabled, otherwise this is loaded from a system-setting.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [9.0.0+], or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletIsForceTerminateApplicationDisabledForDebug(out: *mut bool) -> Result;

    /// Launches DevMenu and the dev Overlay-applet. This will enter an infinite-sleep-loop on success.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [8.0.0+], or on [15.0.0+] with AppletType_LibraryApplet.
    ///
    /// **Note:** This verifies that DebugMode is enabled, then uses a ns cmd. That cmd then loads the system-settings for these two ProgramIds (which normally only exist on devunits), and verifies that these programs are installed + launches them.
    pub fn appletLaunchDevMenu() -> Result;

    /// SetLastApplicationExitReason
    ///
    /// # Arguments
    /// - `reason` (in): Reason
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [11.0.0+], or on [15.0.0+] with AppletType_LibraryApplet.
    pub fn appletSetLastApplicationExitReason(reason: s32) -> Result;

    /// Start the sequence for entering sleep-mode.
    ///
    /// # Arguments
    /// - `flag` (in): Flag, official sw uses hard-coded value = true.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletStartSleepSequence(flag: bool) -> Result;

    /// Start the system-shutdown sequence.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletStartShutdownSequence() -> Result;

    /// Start the system-reboot sequence.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletStartRebootSequence() -> Result;

    /// IsAutoPowerDownRequested. Uses an idle:sys cmd internally.
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [7.0.0+], or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletIsAutoPowerDownRequested(out: *mut bool) -> Result;

    /// LoadAndApplyIdlePolicySettings. Uses an idle:sys cmd internally.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletLoadAndApplyIdlePolicySettings() -> Result;

    /// NotifyCecSettingsChanged. Uses an omm cmd internally.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [2.0.0+], or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletNotifyCecSettingsChanged() -> Result;

    /// Sets the DefaultHomeButtonLongPressTime.
    ///
    /// # Arguments
    /// - `val` (in): Input value.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [3.0.0+], or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletSetDefaultHomeButtonLongPressTime(val: s64) -> Result;

    /// UpdateDefaultDisplayResolution. Uses an omm cmd internally.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [3.0.0+], or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletUpdateDefaultDisplayResolution() -> Result;

    /// ShouldSleepOnBoot. Uses an omm cmd internally.
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [3.0.0+], or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    pub fn appletShouldSleepOnBoot(out: *mut bool) -> Result;

    /// Gets an Event which is signaled for HdcpAuthenticationFailed.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with AppletType_SystemApplet on [4.0.0+], or on [15.0.0+] with AppletType_LibraryApplet/AppletType_OverlayApplet.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetHdcpAuthenticationFailedEvent(out_event: *mut Event) -> Result;

    /// Creates an Application.
    ///
    /// # Arguments
    /// - `a` (out): `AppletApplication`
    /// - `application_id` (in): ApplicationId.
    ///
    /// **Note:** Only available with AppletType_SystemApplet.
    pub fn appletCreateApplication(a: *mut AppletApplication, application_id: u64) -> Result;

    /// Pops a `AppletApplication` for a requested Application launch.
    ///
    /// # Arguments
    /// - `a` (out): `AppletApplication`
    ///
    /// **Note:** Only available with AppletType_SystemApplet.
    pub fn appletPopLaunchRequestedApplication(a: *mut AppletApplication) -> Result;

    /// Creates a SystemApplication.
    ///
    /// # Arguments
    /// - `a` (out): `AppletApplication`
    /// - `system_application_id` (in): SystemApplicationId.
    ///
    /// **Note:** Only available with AppletType_SystemApplet.
    pub fn appletCreateSystemApplication(a: *mut AppletApplication, system_application_id: u64) -> Result;

    /// PopFloatingApplicationForDevelopment.
    ///
    /// # Arguments
    /// - `a` (out): `AppletApplication`
    ///
    /// **Note:** Only available with AppletType_SystemApplet. Should not be used if no FloatingApplication is available.
    pub fn appletPopFloatingApplicationForDevelopment(a: *mut AppletApplication) -> Result;

    /// Close an `AppletApplication`.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationClose(a: *mut AppletApplication);

    /// Returns whether the AppletApplication object was initialized.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationActive(a: *mut AppletApplication) -> bool;

    /// Starts the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationStart(a: *mut AppletApplication) -> Result;

    /// Requests the Application to exit.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationRequestExit(a: *mut AppletApplication) -> Result;

    /// Terminate the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationTerminate(a: *mut AppletApplication) -> Result;

    /// Waits for the Application to exit.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationJoin(a: *mut AppletApplication);

    /// Waits on the Application StateChangedEvent with timeout=0, and returns whether it was successful.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationCheckFinished(a: *mut AppletApplication) -> bool;

    /// Gets the `AppletApplicationExitReason` set by `appletApplicationJoin`.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationGetExitReason(a: *mut AppletApplication) -> AppletApplicationExitReason;

    /// RequestForApplicationToGetForeground.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    pub fn appletApplicationRequestForApplicationToGetForeground(a: *mut AppletApplication) -> Result;

    /// TerminateAllLibraryApplets which were created by the Application.
    pub fn appletApplicationTerminateAllLibraryApplets(a: *mut AppletApplication) -> Result;

    /// AreAnyLibraryAppletsLeft which were created by the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `out` (out): Output flag.
    pub fn appletApplicationAreAnyLibraryAppletsLeft(a: *mut AppletApplication, out: *mut bool) -> Result;

    /// Calls the same func as `appletHolderRequestExitOrTerminate` with the output IAppletAccessor from the GetCurrentLibraryApplet cmd.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `timeout` (in): Timeout in nanoseconds. UINT64_MAX for no timeout.
    pub fn appletApplicationRequestExitLibraryAppletOrTerminate(a: *mut AppletApplication, timeout: u64) -> Result;

    /// Gets the ApplicationId for the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `application_id` (out): Output ApplicationId.
    pub fn appletApplicationGetApplicationId(a: *mut AppletApplication, application_id: *mut u64) -> Result;

    /// Pushes a LaunchParameter AppletStorage to the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `kind` (in): `AppletLaunchParameterKind`
    /// - `s` (in): Input storage.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletApplicationPushLaunchParameter(a: *mut AppletApplication, kind: AppletLaunchParameterKind, s: *mut AppletStorage) -> Result;

    /// Gets the `NacpStruct` for the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `nacp` (out): `NacpStruct`
    ///
    /// **Note:** Not usable when the `AppletApplication` is for an AppletType_SystemApplication.
    pub fn appletApplicationGetApplicationControlProperty(a: *mut AppletApplication, nacp: *mut NacpStruct) -> Result;

    /// Gets the `AppletApplicationLaunchProperty` for the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `out` (out): `AppletApplicationLaunchProperty`
    ///
    /// **Note:** Only available on [2.0.0+]. Not usable when the `AppletApplication` is for an AppletType_SystemApplication.
    pub fn appletApplicationGetApplicationLaunchProperty(a: *mut AppletApplication, out: *mut AppletApplicationLaunchProperty) -> Result;

    /// Gets the `AppletApplicationLaunchRequestInfo` for the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `out` (out): `AppletApplicationLaunchRequestInfo`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn appletApplicationGetApplicationLaunchRequestInfo(a: *mut AppletApplication, out: *mut AppletApplicationLaunchRequestInfo) -> Result;

    /// SetUsers for the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `uids` (in): Input array of `AccountUid`.
    /// - `count` (in): Size of the uids array in entries, must be &lt;=ACC_USER_LIST_SIZE.
    /// - `flag` (in): When this flag is true, this just clears the users_available state flag to 0 and returns.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn appletApplicationSetUsers(a: *mut AppletApplication, uids: *const AccountUid, count: s32, flag: bool) -> Result;

    /// CheckRightsEnvironmentAvailable.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn appletApplicationCheckRightsEnvironmentAvailable(a: *mut AppletApplication, out: *mut bool) -> Result;

    /// GetNsRightsEnvironmentHandle.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `handle` (out): Output NsRightsEnvironmentHandle.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn appletApplicationGetNsRightsEnvironmentHandle(a: *mut AppletApplication, handle: *mut u64) -> Result;

    /// Gets an array of userIds for the Application DesirableUids.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `uids` (out): Output array of `AccountUid`.
    /// - `count` (in): Size of the uids array in entries, must be at least the size stored in state.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available on [6.0.0+].
    ///
    /// **Note:** qlaunch only uses 1 userId with this.
    pub fn appletApplicationGetDesirableUids(a: *mut AppletApplication, uids: *mut AccountUid, count: s32, total_out: *mut s32) -> Result;

    /// ReportApplicationExitTimeout.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn appletApplicationReportApplicationExitTimeout(a: *mut AppletApplication) -> Result;

    /// Sets the `AppletApplicationAttribute` for the Application.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `attr` (in): `AppletApplicationAttribute`
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn appletApplicationSetApplicationAttribute(a: *mut AppletApplication, attr: *const AppletApplicationAttribute) -> Result;

    /// Gets whether the savedata specified by the input ApplicationId is accessible.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `application_id` (in): ApplicationId for the savedata.
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available on [8.0.0+].
    pub fn appletApplicationHasSaveDataAccessPermission(a: *mut AppletApplication, application_id: u64, out: *mut bool) -> Result;

    /// Creates a storage using the specified input then pushes it to the FriendInvitation StorageChannel.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `uid` (in): `AccountUid`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    ///
    /// **Note:** The system will clear the StorageChannel before pushing the storage.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn appletApplicationPushToFriendInvitationStorageChannel(a: *mut AppletApplication, uid: AccountUid, buffer: *const c_void, size: u64) -> Result;

    /// Creates a storage using the specified input then pushes it to the Notification StorageChannel.
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    ///
    /// **Note:** The system will clear the StorageChannel before pushing the storage.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn appletApplicationPushToNotificationStorageChannel(a: *mut AppletApplication, buffer: *const c_void, size: u64) -> Result;

    /// RequestApplicationSoftReset
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn appletApplicationRequestApplicationSoftReset(a: *mut AppletApplication) -> Result;

    /// RestartApplicationTimer
    ///
    /// # Arguments
    /// - `a`: `AppletApplication`
    ///
    /// **Note:** Only available on [10.0.0+].
    pub fn appletApplicationRestartApplicationTimer(a: *mut AppletApplication) -> Result;

    /// Pops a storage from current-LibraryApplet input.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletPopInData(s: *mut AppletStorage) -> Result;

    /// Pushes a storage for current-LibraryApplet output.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletPushOutData(s: *mut AppletStorage) -> Result;

    /// Pops a storage from current-LibraryApplet Interactive input.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletPopInteractiveInData(s: *mut AppletStorage) -> Result;

    /// Pushes a storage for current-LibraryApplet Interactive output.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletPushInteractiveOutData(s: *mut AppletStorage) -> Result;

    /// Gets an Event which is signaled when a new storage is available with `appletPopInData` where previously no storage was available, this event is automatically cleared by the system once the last storage is popped.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetPopInDataEvent(out_event: *mut Event) -> Result;

    /// Gets an Event which is signaled when a new storage is available with `appletPopInteractiveInData` where previously no storage was available, this event is automatically cleared by the system once the last storage is popped.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetPopInteractiveInDataEvent(out_event: *mut Event) -> Result;

    /// Gets the `LibAppletInfo` for the current LibraryApplet.
    ///
    /// # Arguments
    /// - `info` (out): `LibAppletInfo`
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletGetLibraryAppletInfo(info: *mut LibAppletInfo) -> Result;

    /// Gets the `AppletIdentityInfo` for the MainApplet.
    ///
    /// # Arguments
    /// - `info` (out): `AppletIdentityInfo`
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletGetMainAppletIdentityInfo(info: *mut AppletIdentityInfo) -> Result;

    /// CanUseApplicationCore
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletCanUseApplicationCore(out: *mut bool) -> Result;

    /// Gets the `AppletIdentityInfo` for the CallerApplet.
    ///
    /// # Arguments
    /// - `info` (out): `AppletIdentityInfo`
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletGetCallerAppletIdentityInfo(info: *mut AppletIdentityInfo) -> Result;

    /// Gets the `NacpStruct` for the MainApplet.
    ///
    /// # Arguments
    /// - `nacp` (out): `NacpStruct`
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [2.0.0+].
    pub fn appletGetMainAppletApplicationControlProperty(nacp: *mut NacpStruct) -> Result;

    /// Gets the NcmStorageId for the MainApplet.
    ///
    /// # Arguments
    /// - `storageId` (out): `NcmStorageId`
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [2.0.0+].
    pub fn appletGetMainAppletStorageId(storageId: *mut NcmStorageId) -> Result;

    /// Gets an array of `AppletIdentityInfo` for the CallerStack.
    ///
    /// # Arguments
    /// - `stack` (out): Output array of `AppletIdentityInfo`.
    /// - `count` (in): Size of the stack array.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [3.0.0+].
    pub fn appletGetCallerAppletIdentityInfoStack(stack: *mut AppletIdentityInfo, count: s32, total_out: *mut s32) -> Result;

    /// Gets the `AppletIdentityInfo` for the NextReturnDestinationApplet.
    ///
    /// # Arguments
    /// - `info` (out): `AppletIdentityInfo`
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [4.0.0+].
    pub fn appletGetNextReturnDestinationAppletIdentityInfo(info: *mut AppletIdentityInfo) -> Result;

    /// Gets the DesirableKeyboardLayout previously set by `appletSetDesirableKeyboardLayout`. An error is returned when it's not set.
    ///
    /// # Arguments
    /// - `layout` (out): Output `SetKeyboardLayout`.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [4.0.0+].
    pub fn appletGetDesirableKeyboardLayout(layout: *mut SetKeyboardLayout) -> Result;

    /// Pops a storage from current-LibraryApplet Extra input.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletPopExtraStorage(s: *mut AppletStorage) -> Result;

    /// Gets an Event which is signaled when a new storage is available with `appletPopExtraStorage` where previously no storage was available, this event is automatically cleared by the system once the last storage is popped.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    pub fn appletGetPopExtraStorageEvent(out_event: *mut Event) -> Result;

    /// Unpop a storage for current-LibraryApplet input.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletUnpopInData(s: *mut AppletStorage) -> Result;

    /// Unpop a storage for current-LibraryApplet Extra input.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletUnpopExtraStorage(s: *mut AppletStorage) -> Result;

    /// Gets the IndirectLayerProducerHandle.
    ///
    /// # Arguments
    /// - `out` (out): Output IndirectLayerProducerHandle.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [2.0.0+].
    pub fn appletGetIndirectLayerProducerHandle(out: *mut u64) -> Result;

    /// Gets the DesiredLanguage for the MainApplet.
    ///
    /// # Arguments
    /// - `LanguageCode` (out): Output LanguageCode, see set.h.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [4.0.0+].
    pub fn appletGetMainAppletApplicationDesiredLanguage(LanguageCode: *mut u64) -> Result;

    /// Gets the ApplicationId for the currently running Application.
    ///
    /// # Arguments
    /// - `application_id` (out): Output ApplicationId, 0 when no Application is running.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [8.0.0+].
    pub fn appletGetCurrentApplicationId(application_id: *mut u64) -> Result;

    /// Exits the current applet. Same as `appletHolderRequestExit` except this is for the current applet.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [6.0.0+].
    pub fn appletRequestExitToSelf() -> Result;

    /// CreateGameMovieTrimmer. Do not use this directly, use `grcTrimGameMovie` instead.
    ///
    /// # Arguments
    /// - `srv_out` (out): Output Service for grc IGameMovieTrimmer.
    /// - `tmem` (in): TransferMemory
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [4.0.0+].
    ///
    /// **Note:** See also `appletReserveResourceForMovieOperation` and `appletUnreserveResourceForMovieOperation`.
    pub fn appletCreateGameMovieTrimmer(srv_out: *mut Service, tmem: *mut TransferMemory) -> Result;

    /// ReserveResourceForMovieOperation. Must be used at some point prior to `appletCreateGameMovieTrimmer`.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [5.0.0+].
    pub fn appletReserveResourceForMovieOperation() -> Result;

    /// UnreserveResourceForMovieOperation. Must be used at some point after all finished with GameMovieTrimmer usage (`appletCreateGameMovieTrimmer`).
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [5.0.0+].
    pub fn appletUnreserveResourceForMovieOperation() -> Result;

    /// Gets an array of userIds for the MainApplet AvailableUsers.
    ///
    /// # Arguments
    /// - `uids` (out): Output array of `AccountUid`.
    /// - `count` (in): Size of the uids array in entries, must be at least ACC_USER_LIST_SIZE.
    /// - `flag` (out): When true, this indicates that no users are available.
    /// - `total_out` (out): Total output entries. This is -1 when flag is true.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [6.0.0+].
    pub fn appletGetMainAppletAvailableUsers(uids: *mut AccountUid, count: s32, flag: *mut bool, total_out: *mut s32) -> Result;

    /// SetApplicationMemoryReservation
    ///
    /// # Arguments
    /// - `val` (in): Input value.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [10.0.0+].
    ///
    /// **Note:** An Application must be currently running.
    pub fn appletSetApplicationMemoryReservation(val: u64) -> Result;

    /// ShouldSetGpuTimeSliceManually
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet on [10.0.0+].
    pub fn appletShouldSetGpuTimeSliceManually(out: *mut bool) -> Result;

    /// Stops forwarding the input to the foreground app.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet.
    ///
    /// **Note:** You have to call this to receive inputs through the hid service when running as the overlay applet.
    pub fn appletBeginToWatchShortHomeButtonMessage() -> Result;

    /// Forwards input to the foreground app.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet.
    ///
    /// **Note:** After calling this the overlay applet won't receive any input until `appletBeginToWatchShortHomeButtonMessage` is called again.
    pub fn appletEndToWatchShortHomeButtonMessage() -> Result;

    /// Gets the ApplicationId for displaying the logo screen during application launch.
    ///
    /// # Arguments
    /// - `application_id` (out): Output ApplicationId, 0 when no application is running.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet.
    pub fn appletGetApplicationIdForLogo(application_id: *mut u64) -> Result;

    /// Sets the GpuTimeSliceBoost.
    ///
    /// # Arguments
    /// - `val` (in): Input value.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet.
    pub fn appletSetGpuTimeSliceBoost(val: u64) -> Result;

    /// Sets AutoSleepTimeAndDimmingTimeEnabled.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [2.0.0+].
    pub fn appletSetAutoSleepTimeAndDimmingTimeEnabled(flag: bool) -> Result;

    /// TerminateApplicationAndSetReason
    ///
    /// # Arguments
    /// - `reason` (in): Result reason.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [2.0.0+].
    pub fn appletTerminateApplicationAndSetReason(reason: Result) -> Result;

    /// Sets ScreenShotPermissionGlobally.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [3.0.0+].
    pub fn appletSetScreenShotPermissionGlobally(flag: bool) -> Result;

    /// Start the system-shutdown sequence. This will enter an infinite-sleep-loop on success.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [6.0.0+].
    pub fn appletStartShutdownSequenceForOverlay() -> Result;

    /// Start the system-reboot sequence. This will enter an infinite-sleep-loop on success.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [6.0.0+].
    pub fn appletStartRebootSequenceForOverlay() -> Result;

    /// SetHealthWarningShowingState
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [9.0.0+].
    pub fn appletSetHealthWarningShowingState(flag: bool) -> Result;

    /// IsHealthWarningRequired
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [10.0.0+].
    pub fn appletIsHealthWarningRequired(out: *mut bool) -> Result;

    /// Enables HID input for the OverlayApplet, without disabling input for the foreground applet. Generally `appletBeginToWatchShortHomeButtonMessage` / appletEndToWatchShortHomeButtonMessage should be used instead.
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [5.0.0+].
    pub fn appletBeginToObserveHidInputForDevelop() -> Result;

    /// Reads the ThemeStorage for the current applet.
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer data.
    /// - `size` (in): Size to read.
    /// - `offset` (in): Offset within the ThemeStorage.
    /// - `transfer_size` (out): Actual read size.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [7.0.0+].
    ///
    /// **Note:** offset(+size) must be &lt;=0x400.
    pub fn appletReadThemeStorage(buffer: *mut c_void, size: usize, offset: u64, transfer_size: *mut u64) -> Result;

    /// Writes the ThemeStorage for the current applet.
    ///
    /// # Arguments
    /// - `buffer` (in): Input buffer data.
    /// - `size` (in): Size to write.
    /// - `offset` (in): Offset within the ThemeStorage.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [7.0.0+].
    ///
    /// **Note:** offset(+size) must be &lt;=0x400.
    pub fn appletWriteThemeStorage(buffer: *const c_void, size: usize, offset: u64) -> Result;

    /// This is similar to `appletPushToAppletBoundChannelForDebug` (no DebugMode check), except the used channel is loaded from elsewhere and must be in the range 31-32.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [9.0.0+].
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletPushToAppletBoundChannel(s: *mut AppletStorage) -> Result;

    /// This is similar to `appletTryPopFromAppletBoundChannelForDebug` (no DebugMode check), except the used channel is loaded from elsewhere and must be in the range 31-32.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [9.0.0+].
    pub fn appletTryPopFromAppletBoundChannel(s: *mut AppletStorage) -> Result;

    /// Gets the DisplayLogicalResolution.
    ///
    /// # Arguments
    /// - `width` (out): Output width.
    /// - `height` (out): Output height.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [8.0.0+].
    pub fn appletGetDisplayLogicalResolution(width: *mut s32, height: *mut s32) -> Result;

    /// Sets the DisplayMagnification. This is essentially layer image crop, for everything non-Overlay.
    ///
    /// # Arguments
    /// - `x` (in): X position.
    /// - `y` (in): Y position.
    /// - `width` (in): Width.
    /// - `height` (in): Height.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [8.0.0+].
    ///
    /// **Note:** x and width are multiplied with the same width value returned by `appletGetDisplayLogicalResolution`, so these should be in the range 0.0f-1.0f. Likewise for y and height, except these are multipled with the height value.
    pub fn appletSetDisplayMagnification(x: f32, y: f32, width: f32, height: f32) -> Result;

    /// Sets whether HomeButtonDoubleClick is enabled.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [8.0.0+].
    pub fn appletSetHomeButtonDoubleClickEnabled(flag: bool) -> Result;

    /// Gets whether HomeButtonDoubleClick is enabled.
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [8.0.0+].
    pub fn appletGetHomeButtonDoubleClickEnabled(out: *mut bool) -> Result;

    /// IsHomeButtonShortPressedBlocked
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [10.0.0+].
    pub fn appletIsHomeButtonShortPressedBlocked(out: *mut bool) -> Result;

    /// IsVrModeCurtainRequired
    ///
    /// # Arguments
    /// - `out` (out): Output flag.
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [11.0.0+].
    pub fn appletIsVrModeCurtainRequired(out: *mut bool) -> Result;

    /// SetCpuBoostRequestPriority
    ///
    /// # Arguments
    /// - `priority` (in): Priority
    ///
    /// **Note:** Only available with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [11.0.0+].
    pub fn appletSetCpuBoostRequestPriority(priority: s32) -> Result;

    /// Open an `AppletApplication` for the currently running Application.
    ///
    /// # Arguments
    /// - `a` (out): `AppletApplication`
    ///
    /// **Note:** Should not be used when no Application is running.
    ///
    /// **Note:** Only available on [1.0.0-9.2.0].
    pub fn appletOpenMainApplication(a: *mut AppletApplication) -> Result;

    /// Perform SystemButtonPressing with the specified `AppletSystemButtonType`.
    ///
    /// # Arguments
    /// - `type` (in): `AppletSystemButtonType`
    pub fn appletPerformSystemButtonPressing(type_: AppletSystemButtonType) -> Result;

    /// InvalidateTransitionLayer.
    pub fn appletInvalidateTransitionLayer() -> Result;

    /// Requests to launch the specified Application, with the specified users.
    ///
    /// # Arguments
    /// - `application_id` (in): ApplicationId.
    /// - `uids` (in): Input array of `AccountUid`.
    /// - `total_uids` (in): Total input uids, must be &lt;=ACC_USER_LIST_SIZE.
    /// - `flag` (in): Whether to use the specified buffer to create a storage which will be pushed for ::AppletLaunchParameterKind_UserChannel.
    /// - `buffer` (in): Buffer containing the above storage data.
    /// - `size` (in): Size of the storage buffer.
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn appletRequestLaunchApplicationWithUserAndArgumentForDebug(application_id: u64, uids: *const AccountUid, total_uids: s32, flag: bool, buffer: *const c_void, size: usize) -> Result;

    /// Gets the `AppletResourceUsageInfo`.
    ///
    /// # Arguments
    /// - `info` (out): `AppletResourceUsageInfo`
    ///
    /// **Note:** Only available on [6.0.0+].
    pub fn appletGetAppletResourceUsageInfo(info: *mut AppletResourceUsageInfo) -> Result;

    /// The channel must match the value already stored in state when the state value is non-zero, otherwise an error is returned. When the state value is 0, the channel is written into state. Then the input storage is pushed to the StorageChannel.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    /// - `channel` (in): Channel.
    ///
    /// **Note:** Only available on [9.0.0+]. DebugMode must be enabled.
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletPushToAppletBoundChannelForDebug(s: *mut AppletStorage, channel: s32) -> Result;

    /// The channel must not be 0 and must match the value previously saved by `appletPushToAppletBoundChannelForDebug`, otherwise errors are returned. Then the output storage is popped from the StorageChannel.
    ///
    /// # Arguments
    /// - `s` (out): Storage object.
    /// - `channel` (in): Channel.
    ///
    /// **Note:** Only available on [9.0.0+]. DebugMode must be enabled.
    pub fn appletTryPopFromAppletBoundChannelForDebug(s: *mut AppletStorage, channel: s32) -> Result;

    /// Clears a StorageChannel, pushes the input storage there, and writes the ApplicationId into state.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    /// - `application_id` (in): ApplicationId
    ///
    /// **Note:** Only available on [9.0.0+].
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletAlarmSettingNotificationEnableAppEventReserve(s: *mut AppletStorage, application_id: u64) -> Result;

    /// Clears the StorageChannel/saved-ApplicationId used by `appletAlarmSettingNotificationEnableAppEventReserve`.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn appletAlarmSettingNotificationDisableAppEventReserve() -> Result;

    /// Same as `appletApplicationPushToNotificationStorageChannel` except this uses the MainApplication.
    ///
    /// # Arguments
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn appletAlarmSettingNotificationPushAppEventNotify(buffer: *const c_void, size: u64) -> Result;

    /// Clears a StorageChannel, pushes the input storage there, and writes the ApplicationId into state.
    ///
    /// # Arguments
    /// - `s` (in): Storage object.
    /// - `application_id` (in): ApplicationId
    ///
    /// **Note:** Only available on [9.0.0+].
    ///
    /// **Note:** This uses `appletStorageClose` automatically.
    pub fn appletFriendInvitationSetApplicationParameter(s: *mut AppletStorage, application_id: u64) -> Result;

    /// Clears the StorageChannel/saved-ApplicationId used by `appletFriendInvitationSetApplicationParameter`.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn appletFriendInvitationClearApplicationParameter() -> Result;

    /// Same as `appletApplicationPushToFriendInvitationStorageChannel` except this uses the MainApplication.
    ///
    /// # Arguments
    /// - `uid` (in): `AccountUid`
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    ///
    /// **Note:** Only available on [9.0.0+].
    pub fn appletFriendInvitationPushApplicationParameter(uid: AccountUid, buffer: *const c_void, size: u64) -> Result;

    /// Creates a GeneralStorage with the specified Id and size.
    ///
    /// # Arguments
    /// - `id` (in): Id value, high-byte must be 0.
    /// - `size` (in): Storage size.
    ///
    /// **Note:** Only available on [18.0.0+].
    pub fn appletCreateGeneralStorageForDebug(id: u64, size: u64) -> Result;

    /// Reads data from the buffer previously allocated by `appletCreateGeneralStorageForDebug`.
    ///
    /// # Arguments
    /// - `buffer` (out): Output buffer.
    /// - `size` (in): Output buffer size.
    /// - `id` (in): Id value which was previously used with `appletCreateGeneralStorageForDebug`, high-byte must be 0.
    /// - `offset` (in): Offset in the storage.
    /// - `out_size` (out): Output size.
    ///
    /// **Note:** Only available on [18.0.0+].
    pub fn appletReadGeneralStorageForDebug(buffer: *mut c_void, size: usize, id: u64, offset: u64, out_size: *mut u64) -> Result;

    /// Writes data to the buffer previously allocated by `appletCreateGeneralStorageForDebug`.
    ///
    /// # Arguments
    /// - `buffer` (in): Input buffer.
    /// - `size` (in): Input buffer size.
    /// - `id` (in): Id value which was previously used with `appletCreateGeneralStorageForDebug`, high-byte must be 0.
    /// - `offset` (in): Offset in the storage.
    ///
    /// **Note:** Only available on [18.0.0+].
    pub fn appletWriteGeneralStorageForDebug(buffer: *const c_void, size: usize, id: u64, offset: u64) -> Result;

    /// SetTerminateResult
    ///
    /// # Arguments
    /// - `res` (in): Result
    ///
    /// **Note:** Only available with AppletType_*Application. Or with AppletType_SystemApplet, AppletType_LibraryApplet, or AppletType_OverlayApplet, on [9.0.0+].
    pub fn appletSetTerminateResult(res: Result) -> Result;

    /// Gets the LaunchStorageInfo.
    ///
    /// # Arguments
    /// - `app_storageId` (out): Same as AppletApplicationLaunchProperty::app_storageId.
    /// - `update_storageId` (out): Same as AppletApplicationLaunchProperty::update_storageId.
    ///
    /// **Note:** Only available with AppletType_*Application on [2.0.0+], or with AppletType_LibraryApplet on [9.0.0+].
    pub fn appletGetLaunchStorageInfoForDebug(app_storageId: *mut NcmStorageId, update_storageId: *mut NcmStorageId) -> Result;

    /// Gets an Event which is signaled for GpuErrorDetected.
    ///
    /// # Arguments
    /// - `out_event` (out): Output Event with autoclear=false.
    ///
    /// **Note:** Only available with AppletType_*Application on [8.0.0+], or with AppletType_LibraryApplet on [9.0.0+].
    ///
    /// **Note:** The Event must be closed by the user once finished with it.
    ///
    /// **Note:** Official sw waits on this Event from a seperate thread, triggering an abort when it's signaled.
    pub fn appletGetGpuErrorDetectedSystemEvent(out_event: *mut Event) -> Result;

    /// Sets HandlingHomeButtonShortPressedEnabled.
    ///
    /// # Arguments
    /// - `flag` (in): Flag
    ///
    /// **Note:** Only available with AppletType_OverlayApplet on [8.0.0+]. Or with non-AppletType_OverlayApplet on [9.1.0+].
    pub fn appletSetHandlingHomeButtonShortPressedEnabled(flag: bool) -> Result;

    /// Gets the cached `AppletInfo` loaded during `appletInitialize`. This will return NULL when the info is not initialized, due to not running as AppletType_LibraryApplet, or when any of the used cmds fail.
    ///
    /// **Note:** Only available with AppletType_LibraryApplet.
    pub fn appletGetAppletInfo() -> *mut AppletInfo;

    /// Gets the event associated with notification messages.
    pub fn appletGetMessageEvent() -> *mut Event;

    /// Gets a notification message, see `AppletMessage`.
    pub fn appletGetMessage(msg: *mut u32) -> Result;

    /// Processes the current applet status using the specified msg.
    ///
    /// # Arguments
    /// - `msg`: Notification message, normally from `appletGetMessage`.
    ///
    /// **Returns:** Whether the application should continue running.
    pub fn appletProcessMessage(msg: u32) -> bool;

    /// Processes the current applet status. Generally used within a main loop.
    ///
    /// **Note:** Uses `appletGetMessage` and `appletProcessMessage` internally.
    ///
    /// **Returns:** Whether the application should continue running.
    pub fn appletMainLoop() -> bool;

    /// Sets up an applet status hook.
    ///
    /// # Arguments
    /// - `cookie`: Hook cookie to use.
    /// - `callback`: Function to call when applet's status changes.
    /// - `param`: User-defined parameter to pass to the callback.
    pub fn appletHook(cookie: *mut AppletHookCookie, callback: AppletHookFn, param: *mut c_void);

    /// Removes an applet status hook.
    ///
    /// # Arguments
    /// - `cookie`: Hook cookie to remove.
    pub fn appletUnhook(cookie: *mut AppletHookCookie);

    /// These return state which is updated by appletMainLoop() when notifications are received.
    pub fn appletGetOperationMode() -> AppletOperationMode;

    pub fn appletGetPerformanceMode() -> ApmPerformanceMode;

    pub fn appletGetFocusState() -> AppletFocusState;

    /// Sets the current `AppletFocusHandlingMode`.
    ///
    /// **Note:** Should only be called with AppletType_Application.
    pub fn appletSetFocusHandlingMode(mode: AppletFocusHandlingMode) -> Result;
}
