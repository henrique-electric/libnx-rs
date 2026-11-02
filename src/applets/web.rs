//! `switch/applets/web.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// This indicates the type of web-applet.
pub type WebShimKind = u32;
pub const WebShimKind_Shop: WebShimKind = 1;
pub const WebShimKind_Login: WebShimKind = 2;
pub const WebShimKind_Offline: WebShimKind = 3;
pub const WebShimKind_Share: WebShimKind = 4;
pub const WebShimKind_Web: WebShimKind = 5;
pub const WebShimKind_Wifi: WebShimKind = 6;
pub const WebShimKind_Lobby: WebShimKind = 7;

/// ExitReason
pub type WebExitReason = u32;
/// User pressed the X button to exit.
pub const WebExitReason_ExitButton: WebExitReason = 0x0;
/// User pressed the B button to exit, on the initial page.
pub const WebExitReason_BackButton: WebExitReason = 0x1;
/// The applet exited since `webConfigRequestExit` was used.
pub const WebExitReason_Requested: WebExitReason = 0x2;
/// The applet exited due to LastUrl handling, see `webReplyGetLastUrl`.
pub const WebExitReason_LastUrl: WebExitReason = 0x3;
/// The applet exited after displaying an error dialog.
pub const WebExitReason_ErrorDialog: WebExitReason = 0x7;
/// Unknown
pub const WebExitReason_UnknownE: WebExitReason = 0xE;

/// Button values for `webConfigSetBootFooterButtonVisible`.
pub type WebFooterButtonId = u32;
/// None, for empty `WebBootFooterButtonEntry`. Invalid for `webConfigSetBootFooterButtonVisible` input.
pub const WebFooterButtonId_None: WebFooterButtonId = 0;
/// Unknown button Id 1.
pub const WebFooterButtonId_Type1: WebFooterButtonId = 1;
/// Unknown button Id 2.
pub const WebFooterButtonId_Type2: WebFooterButtonId = 2;
/// Unknown button Id 3.
pub const WebFooterButtonId_Type3: WebFooterButtonId = 3;
/// Unknown button Id 4.
pub const WebFooterButtonId_Type4: WebFooterButtonId = 4;
/// Unknown button Id 5.
pub const WebFooterButtonId_Type5: WebFooterButtonId = 5;
/// Unknown button Id 6.
pub const WebFooterButtonId_Type6: WebFooterButtonId = 6;
/// Values starting with this are invalid.
pub const WebFooterButtonId_Max: WebFooterButtonId = 7;

/// WebSessionBootMode
pub type WebSessionBootMode = u32;
/// AllForeground. This is the default.
pub const WebSessionBootMode_AllForeground: WebSessionBootMode = 0;
/// AllForegroundInitiallyHidden
pub const WebSessionBootMode_AllForegroundInitiallyHidden: WebSessionBootMode = 1;

/// WebSessionSendMessageKind
pub type WebSessionSendMessageKind = u32;
/// BrowserEngine Content
pub const WebSessionSendMessageKind_BrowserEngineContent: WebSessionSendMessageKind = 0x0;
/// SystemMessage Appear
pub const WebSessionSendMessageKind_SystemMessageAppear: WebSessionSendMessageKind = 0x100;
/// Ack
pub const WebSessionSendMessageKind_Ack: WebSessionSendMessageKind = 0x1000;

/// WebSessionReceiveMessageKind
pub type WebSessionReceiveMessageKind = u32;
/// BrowserEngine Content
pub const WebSessionReceiveMessageKind_BrowserEngineContent: WebSessionReceiveMessageKind = 0x0;
/// Ack BrowserEngine
pub const WebSessionReceiveMessageKind_AckBrowserEngine: WebSessionReceiveMessageKind = 0x1000;
/// Ack SystemMessage
pub const WebSessionReceiveMessageKind_AckSystemMessage: WebSessionReceiveMessageKind = 0x1001;

/// Struct for the WebWifi applet input storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebWifiPageArg {
    /// Official sw sets this to 0 with appletStorageWrite, separately from the rest of the config struct.
    pub unk_x0: u32,
    /// Connection-test URL.
    pub conntest_url: [c_char; 256],
    /// Initial URL navigated to by the applet.
    pub initial_url: [c_char; 1024],
    /// NIFM Network UUID. Only used by the applet when conntest_url is set.
    pub uuid: Uuid,
    /// Input value for nifm cmd SetRequirementByRevision. Only used by the applet when conntest_url is set.
    pub rev: u32,
}

/// Struct for the WebWifi applet output storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebWifiReturnValue {
    /// Unknown.
    pub unk_x0: u32,
    /// Result
    pub res: Result,
}

/// Config for WebWifi.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebWifiConfig {
    /// Arg data.
    pub arg: WebWifiPageArg,
}

/// TLV storage, starts with `WebArgHeader` followed by `WebArgTLV` entries.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebCommonTLVStorage {
    /// Raw TLV data storage.
    pub data: [u8; 8192],
}

/// Common struct for the applet output storage, for non-TLV-storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebCommonReturnValue {
    /// ExitReason
    pub exitReason: WebExitReason,
    /// Padding
    pub pad: u32,
    /// LastUrl string
    pub lastUrl: [c_char; 4096],
    /// Size of LastUrl, including NUL-terminator.
    pub lastUrlSize: u64,
}

/// Header struct at offset 0 in the web Arg storage (non-webWifi).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebArgHeader {
    /// Total `WebArgTLV` entries following this struct.
    pub total_entries: u16,
    /// Padding
    pub pad: u16,
    /// ShimKind
    pub shimKind: WebShimKind,
}

/// Web TLV used in the web Arg storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebArgTLV {
    /// Type of this arg.
    pub type_: u16,
    /// Size of the arg data following this struct.
    pub size: u16,
    /// Padding
    pub pad: [u8; 4],
}

/// Config struct for web applets, non-WebWifi.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebCommonConfig {
    /// TLV storage.
    pub arg: WebCommonTLVStorage,
    /// AppletId
    pub appletid: AppletId,
    /// CommonArgs applet version.
    pub version: u32,
    /// AppletHolder
    pub holder: AppletHolder,
}

/// Common container struct for applets' reply data, from the output storage.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebCommonReply {
    /// Type of reply: false = ret, true = storage.
    pub type_: bool,
    /// ShimKind
    pub shimKind: WebShimKind,
    /// Reply data for reply=false.
    pub ret: WebCommonReturnValue,
    /// Reply data for reply=true.
    pub storage: WebCommonTLVStorage,
}

/// Entry data for ::WebArgType_BootFooterButton.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebBootFooterButtonEntry {
    pub id: WebFooterButtonId,
    pub visible: u8,
    pub unk_x5: u16,
    pub unk_x7: u8,
}

/// StorageHandleQueue
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebSessionStorageHandleQueue {
    pub read_pos: s32,
    pub write_pos: s32,
    pub max_storages: s32,
    pub is_full: bool,
    pub storages: [AppletStorage; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebSession_queue {
    pub count: u32,
    pub cur_size: u32,
}

/// WebSession
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebSession {
    pub mutex: Mutex,
    pub config: *mut WebCommonConfig,
    pub queue: [WebSession_queue; 2],
    pub storage_queue: WebSessionStorageHandleQueue,
}

/// SessionMessageHeader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WebSessionMessageHeader {
    /// Message Kind (`WebSessionSendMessageKind` / `WebSessionReceiveMessageKind`)
    pub kind: u32,
    /// Data size following the header.
    pub size: u32,
    /// Unused
    pub reserved: [u8; 8],
}

/// Types for `WebArgTLV`, input storage.
pub type WebArgType = u32;
/// [1.0.0+] String, size 0xC00. Initial URL.
pub const WebArgType_Url: WebArgType = 0x1;
/// [1.0.0+] String, size 0x400.
pub const WebArgType_CallbackUrl: WebArgType = 0x3;
/// [1.0.0+] String, size 0x400.
pub const WebArgType_CallbackableUrl: WebArgType = 0x4;
/// [1.0.0+] Offline-applet, u64 ApplicationId
pub const WebArgType_ApplicationId: WebArgType = 0x5;
/// [1.0.0+] Offline-applet, string with size 0xC00.
pub const WebArgType_DocumentPath: WebArgType = 0x6;
/// [1.0.0+] Offline-applet, u32 enum \WebDocumentKind.
pub const WebArgType_DocumentKind: WebArgType = 0x7;
/// [1.0.0+] Offline-applet, u64 SystemDataId
pub const WebArgType_SystemDataId: WebArgType = 0x8;
/// [1.0.0+] u32 enum \WebShareStartPage
pub const WebArgType_ShareStartPage: WebArgType = 0x9;
/// [1.0.0+] String, size 0x1000.
pub const WebArgType_Whitelist: WebArgType = 0xA;
/// [1.0.0+] u8 bool
pub const WebArgType_NewsFlag: WebArgType = 0xB;
/// [1.0.0+] u8
pub const WebArgType_UnknownC: WebArgType = 0xC;
/// [1.0.0+] u8
pub const WebArgType_UnknownD: WebArgType = 0xD;
/// [1.0.0+] `AccountUid`, controls which user-specific savedata to mount.
pub const WebArgType_Uid: WebArgType = 0xE;
/// [1.0.0+] Share-applet caps AlbumEntry, entry 0.
pub const WebArgType_AlbumEntry0: WebArgType = 0xF;
/// [1.0.0+] u8 bool
pub const WebArgType_ScreenShot: WebArgType = 0x10;
/// [1.0.0+] u8 bool
pub const WebArgType_EcClientCert: WebArgType = 0x11;
/// [1.0.0+] u8
pub const WebArgType_Unknown12: WebArgType = 0x12;
/// [1.0.0+] u8 bool
pub const WebArgType_PlayReport: WebArgType = 0x13;
/// [1.0.0+] u8
pub const WebArgType_Unknown14: WebArgType = 0x14;
/// [1.0.0+] u8
pub const WebArgType_Unknown15: WebArgType = 0x15;
/// [1.0.0+] u32 enum `WebBootDisplayKind`
pub const WebArgType_BootDisplayKind: WebArgType = 0x17;
/// [1.0.0+] u32 enum `WebBackgroundKind`
pub const WebArgType_BackgroundKind: WebArgType = 0x18;
/// [1.0.0+] u8 bool
pub const WebArgType_Footer: WebArgType = 0x19;
/// [1.0.0+] u8 bool
pub const WebArgType_Pointer: WebArgType = 0x1A;
/// [1.0.0+] u32 enum `WebLeftStickMode`
pub const WebArgType_LeftStickMode: WebArgType = 0x1B;
/// [1.0.0+] s32 KeyRepeatFrame, first param
pub const WebArgType_KeyRepeatFrame0: WebArgType = 0x1C;
/// [1.0.0+] s32 KeyRepeatFrame, second param
pub const WebArgType_KeyRepeatFrame1: WebArgType = 0x1D;
/// [1.0.0+] u8 bool. With News on [3.0.0+] this is set after BootAsMediaPlayer with the value inverted.
pub const WebArgType_BootAsMediaPlayerInverted: WebArgType = 0x1E;
/// [1.0.0+] u8 bool, DisplayUrlKind (value = (input_enumval==0x1)).
pub const WebArgType_DisplayUrlKind: WebArgType = 0x1F;
/// [2.0.0+] u8 bool
pub const WebArgType_BootAsMediaPlayer: WebArgType = 0x21;
/// [2.0.0+] u8 bool
pub const WebArgType_ShopJump: WebArgType = 0x22;
/// [2.0.0-5.1.0] u8 bool
pub const WebArgType_MediaPlayerUserGestureRestriction: WebArgType = 0x23;
/// [6.0.0+] u8 bool
pub const WebArgType_MediaAutoPlay: WebArgType = 0x23;
/// [2.0.0+] String, size 0x100.
pub const WebArgType_LobbyParameter: WebArgType = 0x24;
/// [3.0.0+] Share-applet caps ApplicationAlbumEntry
pub const WebArgType_ApplicationAlbumEntry: WebArgType = 0x26;
/// [3.0.0+] u8 bool
pub const WebArgType_JsExtension: WebArgType = 0x27;
/// [4.0.0+] String, size 0x100. Share-applet AdditionalCommentText.
pub const WebArgType_AdditionalCommentText: WebArgType = 0x28;
/// [4.0.0+] u8 bool
pub const WebArgType_TouchEnabledOnContents: WebArgType = 0x29;
/// [4.0.0+] String, size 0x80.
pub const WebArgType_UserAgentAdditionalString: WebArgType = 0x2A;
/// [4.0.0+] Share-applet 0x10-byte u8 array, AdditionalMediaData. Entry 0. If the user-input size is less than 0x10, the remaining data used for the TLV is cleared.
pub const WebArgType_AdditionalMediaData0: WebArgType = 0x2B;
/// [4.0.0+] u8 bool
pub const WebArgType_MediaPlayerAutoClose: WebArgType = 0x2C;
/// [4.0.0+] u8 bool
pub const WebArgType_PageCache: WebArgType = 0x2D;
/// [4.0.0+] u8 bool
pub const WebArgType_WebAudio: WebArgType = 0x2E;
/// [5.0.0+] u8
pub const WebArgType_2F: WebArgType = 0x2F;
/// [5.0.0+] u8 bool Indicates that the built-in whitelist for YouTubeVideo should be used.
pub const WebArgType_YouTubeVideoFlag: WebArgType = 0x31;
/// [5.0.0+] u32 enum `WebFooterFixedKind`
pub const WebArgType_FooterFixedKind: WebArgType = 0x32;
/// [5.0.0+] u8 bool
pub const WebArgType_PageFade: WebArgType = 0x33;
/// [5.0.0+] Share-applet 0x20-byte s8 array, MediaCreatorApplicationRatingAge.
pub const WebArgType_MediaCreatorApplicationRatingAge: WebArgType = 0x34;
/// [5.0.0+] u8 bool
pub const WebArgType_BootLoadingIcon: WebArgType = 0x35;
/// [5.0.0+] u8 bool
pub const WebArgType_PageScrollIndicator: WebArgType = 0x36;
/// [6.0.0+] u8 bool
pub const WebArgType_MediaPlayerSpeedControl: WebArgType = 0x37;
/// [6.0.0+] Share-applet caps AlbumEntry, entry 1.
pub const WebArgType_AlbumEntry1: WebArgType = 0x38;
/// [6.0.0+] Share-applet caps AlbumEntry, entry 2.
pub const WebArgType_AlbumEntry2: WebArgType = 0x39;
/// [6.0.0+] Share-applet caps AlbumEntry, entry 3.
pub const WebArgType_AlbumEntry3: WebArgType = 0x3A;
/// [6.0.0+] Share-applet 0x10-byte u8 array, AdditionalMediaData. Entry 1.
pub const WebArgType_AdditionalMediaData1: WebArgType = 0x3B;
/// [6.0.0+] Share-applet 0x10-byte u8 array, AdditionalMediaData. Entry 2.
pub const WebArgType_AdditionalMediaData2: WebArgType = 0x3C;
/// [6.0.0+] Share-applet 0x10-byte u8 array, AdditionalMediaData. Entry 3.
pub const WebArgType_AdditionalMediaData3: WebArgType = 0x3D;
/// [6.0.0+] Array of `WebBootFooterButtonEntry` with 0x10 entries.
pub const WebArgType_BootFooterButton: WebArgType = 0x3E;
/// [6.0.0+] float
pub const WebArgType_OverrideWebAudioVolume: WebArgType = 0x3F;
/// [6.0.0+] float
pub const WebArgType_OverrideMediaAudioVolume: WebArgType = 0x40;
/// [7.0.0+] u32 enum `WebSessionBootMode`
pub const WebArgType_SessionBootMode: WebArgType = 0x41;
/// [7.0.0+] u8 bool, enables using WebSession when set.
pub const WebArgType_SessionFlag: WebArgType = 0x42;
/// [8.0.0+] u8 bool
pub const WebArgType_MediaPlayerUi: WebArgType = 0x43;
/// [11.0.0+] u8 bool
pub const WebArgType_TransferMemory: WebArgType = 0x44;

/// Types for `WebArgTLV`, output storage.
pub type WebReplyType = u32;
/// [3.0.0+] u32 ExitReason
pub const WebReplyType_ExitReason: WebReplyType = 0x1;
/// [3.0.0+] string
pub const WebReplyType_LastUrl: WebReplyType = 0x2;
/// [3.0.0+] u64
pub const WebReplyType_LastUrlSize: WebReplyType = 0x3;
/// [3.0.0+] u32 SharePostResult
pub const WebReplyType_SharePostResult: WebReplyType = 0x4;
/// [3.0.0+] string
pub const WebReplyType_PostServiceName: WebReplyType = 0x5;
/// [3.0.0+] u64
pub const WebReplyType_PostServiceNameSize: WebReplyType = 0x6;
/// [3.0.0+] string
pub const WebReplyType_PostId: WebReplyType = 0x7;
/// [3.0.0+] u64
pub const WebReplyType_PostIdSize: WebReplyType = 0x8;
/// [8.0.0+] u8 bool
pub const WebReplyType_MediaPlayerAutoClosedByCompletion: WebReplyType = 0x9;

/// This controls the kind of content to mount with Offline-applet.
pub type WebDocumentKind = u32;
/// Use the HtmlDocument NCA content from the application.
pub const WebDocumentKind_OfflineHtmlPage: WebDocumentKind = 0x1;
/// Use the LegalInformation NCA content from the application.
pub const WebDocumentKind_ApplicationLegalInformation: WebDocumentKind = 0x2;
/// Use the Data NCA content from the specified SystemData, see also: <https://switchbrew.org/wiki/Title_list#System_Data_Archives>
pub const WebDocumentKind_SystemDataPage: WebDocumentKind = 0x3;

/// This controls the initial page for ShareApplet, used by `webShareCreate`.
pub type WebShareStartPage = u32;
/// The default "/" page.
pub const WebShareStartPage_Default: WebShareStartPage = 0;
/// The "/settings/" page.
pub const WebShareStartPage_Settings: WebShareStartPage = 1;

/// Kind values for `webConfigSetBootDisplayKind`. Controls the background color while displaying the loading screen during applet boot. Also controls the BackgroundKind when value is non-zero.
pub type WebBootDisplayKind = u32;
/// Default. BackgroundKind is controlled by `WebBackgroundKind`.
pub const WebBootDisplayKind_Default: WebBootDisplayKind = 0;
/// White background. Used by `webOfflineCreate` for docKind ::WebDocumentKind_ApplicationLegalInformation/::WebDocumentKind_SystemDataPage.
pub const WebBootDisplayKind_White: WebBootDisplayKind = 1;
/// Black background.
pub const WebBootDisplayKind_Black: WebBootDisplayKind = 2;
/// Unknown. Used by `webShareCreate`.
pub const WebBootDisplayKind_Unknown3: WebBootDisplayKind = 3;
/// Unknown. Used by `webLobbyCreate`.
pub const WebBootDisplayKind_Unknown4: WebBootDisplayKind = 4;

/// Kind values for `webConfigSetBackgroundKind`. Controls the background color while displaying the loading screen during applet boot. Only used when `WebBootDisplayKind` is ::WebBootDisplayKind_Default. If the applet was not launched by an Application, the applet will only use WebBackgroundKind_Default.
pub type WebBackgroundKind = u32;
/// Default. Same as ::WebBootDisplayKind_White/::WebBootDisplayKind_Black, determined via ::WebArgType_BootAsMediaPlayer.
pub const WebBackgroundKind_Default: WebBackgroundKind = 0;
/// Unknown. Same as ::WebBootDisplayKind_Unknown3.
pub const WebBackgroundKind_Unknown1: WebBackgroundKind = 1;
/// Unknown. Same as ::WebBootDisplayKind_Unknown4. Used by `webLobbyCreate`.
pub const WebBackgroundKind_Unknown2: WebBackgroundKind = 2;

/// Mode values for `webConfigSetLeftStickMode`. Controls the initial mode, this can be toggled by the user via the pressing the left-stick button. If the Pointer flag is set to false (`webConfigSetPointer`), only ::WebLeftStickMode_Cursor will be used and mode toggle by the user is disabled (input value ignored).
pub type WebLeftStickMode = u32;
/// The user can directly control the pointer via the left-stick.
pub const WebLeftStickMode_Pointer: WebLeftStickMode = 0;
/// The user can only select elements on the page via the left-stick.
pub const WebLeftStickMode_Cursor: WebLeftStickMode = 1;

/// Kind values for `webConfigSetFooterFixedKind`. Controls UI footer display behaviour.
pub type WebFooterFixedKind = u32;
/// Default. Footer is hidden while scrolling.
pub const WebFooterFixedKind_Default: WebFooterFixedKind = 0;
/// Footer is always displayed regardless of scrolling.
pub const WebFooterFixedKind_Always: WebFooterFixedKind = 1;
/// Footer is hidden regardless of scrolling.
pub const WebFooterFixedKind_Hidden: WebFooterFixedKind = 2;

impl_zeroed_default!(WebWifiPageArg, WebWifiReturnValue, WebWifiConfig, WebCommonTLVStorage, WebCommonReturnValue, WebArgHeader, WebArgTLV, WebCommonConfig, WebCommonReply, WebBootFooterButtonEntry, WebSessionStorageHandleQueue, WebSession_queue, WebSession, WebSessionMessageHeader);

unsafe extern "C" {
    /// Creates the config for WifiWebAuthApplet. This is the captive portal applet.
    ///
    /// # Arguments
    /// - `config`: WebWifiConfig object.
    /// - `conntest_url`: URL used for the connection-test requests. When empty/NULL the applet will test the connection with nifm and throw an error on failure.
    /// - `initial_url`: Initial URL navigated to by the applet.
    /// - `uuid`: NIFM Network UUID, for nifm cmd SetNetworkProfileId. Value 0 can be used. Only used by the applet when conntest_url is set.
    /// - `rev`: Input value for nifm cmd SetRequirementByRevision. Value 0 can be used. Only used by the applet when conntest_url is set.
    pub fn webWifiCreate(config: *mut WebWifiConfig, conntest_url: *const c_char, initial_url: *const c_char, uuid: Uuid, rev: u32);

    /// Launches WifiWebAuthApplet with the specified config and waits for it to exit.
    ///
    /// # Arguments
    /// - `config`: WebWifiConfig object.
    /// - `out`: Optional output applet reply data, can be NULL.
    pub fn webWifiShow(config: *mut WebWifiConfig, out: *mut WebWifiReturnValue) -> Result;

    /// Creates the config for WebApplet. This applet uses an URL whitelist loaded from the user-process host Application, which is only loaded when running under an Application.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `url`: Initial URL navigated to by the applet.
    ///
    /// **Note:** Sets ::WebArgType_UnknownD, and ::WebArgType_Unknown12 on pre-3.0.0, to value 1.
    pub fn webPageCreate(config: *mut WebCommonConfig, url: *const c_char) -> Result;

    /// Creates the config for WebApplet. This is based on `webPageCreate`, for News. Hence other functions referencing `webPageCreate` also apply to this.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `url`: Initial URL navigated to by the applet.
    ///
    /// **Note:** The domain from the input URL is automatically whitelisted, in addition to any already loaded whitelist.
    ///
    /// **Note:** Sets ::WebArgType_UnknownD to value 1, and sets ::WebArgType_NewsFlag to true. Also uses `webConfigSetEcClientCert` and `webConfigSetShopJump` with flag=true.
    pub fn webNewsCreate(config: *mut WebCommonConfig, url: *const c_char) -> Result;

    /// Creates the config for WebApplet. This is based on `webPageCreate`, for YouTubeVideo. Hence other functions referencing `webPageCreate` also apply to this. This uses a whitelist which essentially only allows youtube embed/ URLs (without mounting content from the host Application).
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `url`: Initial URL navigated to by the applet.
    ///
    /// **Note:** This is only available on [5.0.0+].
    ///
    /// **Note:** Sets ::WebArgType_UnknownD to value 1, and sets ::WebArgType_YouTubeVideoFlag to true. Also uses `webConfigSetBootAsMediaPlayer` with flag=true.
    pub fn webYouTubeVideoCreate(config: *mut WebCommonConfig, url: *const c_char) -> Result;

    /// Creates the config for Offline-applet. This applet uses data loaded from content.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `docKind`: `WebDocumentKind`
    /// - `id`: Id to load the content from. With docKind = ::WebDocumentKind_OfflineHtmlPage, id=0 should be used to specify the user-process application (non-zero is ignored with this docKind).
    /// - `docPath`: Initial document path in RomFS, without the leading '/'. For ::WebDocumentKind_OfflineHtmlPage, this is relative to "html-document/" in RomFS. For the other docKind values, this is relative to "/" in RomFS. This path must contain ".htdocs/".
    ///
    /// **Note:** Uses `webConfigSetLeftStickMode` with ::WebLeftStickMode_Cursor and sets ::WebArgType_BootAsMediaPlayerInverted to false. Uses `webConfigSetPointer` with flag = docKind == ::WebDocumentKind_OfflineHtmlPage.
    ///
    /// **Note:** For docKind ::WebDocumentKind_ApplicationLegalInformation / ::WebDocumentKind_SystemDataPage, uses `webConfigSetFooter` with flag=true and `webConfigSetBackgroundKind` with ::WebBackgroundKind_Default.
    ///
    /// **Note:** For docKind ::WebDocumentKind_SystemDataPage, uses `webConfigSetBootDisplayKind` with ::WebBootDisplayKind_White.
    ///
    /// **Note:** Sets ::WebArgType_Unknown14/::WebArgType_Unknown15 to value 1. With docKind ::WebDocumentKind_ApplicationLegalInformation, uses `webConfigSetBootDisplayKind` with ::WebBootDisplayKind_White.
    ///
    /// **Note:** Sets ::WebArgType_UnknownC to value 1.
    ///
    /// **Note:** With docKind ::WebDocumentKind_ApplicationLegalInformation, uses `webConfigSetEcClientCert` with flag=true.
    ///
    /// **Note:** With docKind ::WebDocumentKind_OfflineHtmlPage on pre-3.0.0, sets ::WebArgType_Unknown12 to value 1.
    ///
    /// **Note:** Lastly, sets the TLVs as needed for the input params.
    pub fn webOfflineCreate(config: *mut WebCommonConfig, docKind: WebDocumentKind, id: u64, docPath: *const c_char) -> Result;

    /// Creates the config for ShareApplet. This applet is for social media posting/settings.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `page`: `WebShareStartPage`
    ///
    /// **Note:** If a non-zero uid isn't set with `webConfigSetUid` prior to using `webConfigShow`, the applet will launch the profile-selector applet to select an account.
    ///
    /// **Note:** An error will be displayed if neither `webConfigSetAlbumEntry`, nor `webConfigSetApplicationAlbumEntry`, nor `webConfigAddAlbumEntryAndMediaData` are used prior to using `webConfigShow`, with ::WebShareStartPage_Default.
    ///
    /// **Note:** Uses `webConfigSetLeftStickMode` with ::WebLeftStickMode_Cursor, `webConfigSetUid` with uid=0, `webConfigSetDisplayUrlKind` with kind=true, and sets ::WebArgType_Unknown14/::WebArgType_Unknown15 to value 1. Uses `webConfigSetBootDisplayKind` with ::WebBootDisplayKind_Unknown3.
    pub fn webShareCreate(config: *mut WebCommonConfig, page: WebShareStartPage) -> Result;

    /// Creates the config for LobbyApplet. This applet is for "Nintendo Switch Online Lounge".
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    ///
    /// **Note:** Only available on [2.0.0+].
    ///
    /// **Note:** If a non-zero uid isn't set with `webConfigSetUid` prior to using `webConfigShow`, the applet will launch the profile-selector applet to select an account.
    ///
    /// **Note:** Uses `webConfigSetLeftStickMode` with ::WebLeftStickMode_Cursor, `webConfigSetPointer` with flag=false on [3.0.0+], `webConfigSetUid` with uid=0, and sets ::WebArgType_Unknown14/::WebArgType_Unknown15 to value 1. Uses `webConfigSetBootDisplayKind` with ::WebBootDisplayKind_Unknown4, `webConfigSetBackgroundKind` with ::WebBackgroundKind_Unknown2, and sets ::WebArgType_BootAsMediaPlayerInverted to false.
    pub fn webLobbyCreate(config: *mut WebCommonConfig) -> Result;

    /// Sets the CallbackUrl. See also `webReplyGetLastUrl`.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `url`: URL
    ///
    /// **Note:** With Offline-applet for LastUrl handling, it compares the domain with "localhost" instead.
    ///
    /// **Note:** Only available with config created by `webPageCreate` or with Share-applet.
    pub fn webConfigSetCallbackUrl(config: *mut WebCommonConfig, url: *const c_char) -> Result;

    /// Sets the CallbackableUrl.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `url`: URL
    ///
    /// **Note:** Only available with config created by `webPageCreate`.
    pub fn webConfigSetCallbackableUrl(config: *mut WebCommonConfig, url: *const c_char) -> Result;

    /// Sets the whitelist.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `whitelist`: Whitelist string, each line is a regex for each whitelisted URL.
    ///
    /// **Note:** Only available with config created by `webPageCreate`.
    ///
    /// **Note:** If the whitelist isn't formatted properly, the applet will exit briefly after the applet is launched.
    pub fn webConfigSetWhitelist(config: *mut WebCommonConfig, whitelist: *const c_char) -> Result;

    /// Sets the account uid. Controls which user-specific savedata to mount.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `uid`: `AccountUid`
    ///
    /// **Note:** Only available with config created by `webPageCreate`, `webLobbyCreate`, or with Share-applet.
    ///
    /// **Note:** Used automatically by `webShareCreate` and `webLobbyCreate` with uid=0.
    pub fn webConfigSetUid(config: *mut WebCommonConfig, uid: AccountUid) -> Result;

    /// Sets the Share CapsAlbumEntry.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `entry`: `CapsAlbumEntry`
    ///
    /// **Note:** Only available with config created by `webShareCreate`.
    pub fn webConfigSetAlbumEntry(config: *mut WebCommonConfig, entry: *const CapsAlbumEntry) -> Result;

    /// Sets the ScreenShot flag, which controls whether screen-shot capture is allowed.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webPageCreate`.
    pub fn webConfigSetScreenShot(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the EcClientCert flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`.
    ///
    /// **Note:** Used automatically by `webOfflineCreate`, depending on the docKind.
    ///
    /// **Note:** Used automatically by `webNewsCreate` with flag=true.
    pub fn webConfigSetEcClientCert(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets whether PlayReport is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate`.
    pub fn webConfigSetPlayReport(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the BootDisplayKind.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `kind`: `WebBootDisplayKind`
    ///
    /// **Note:** Only available with config created by `webOfflineCreate`, `webShareCreate`, `webPageCreate`, or `webLobbyCreate`..
    ///
    /// **Note:** Used automatically by `webOfflineCreate`, depending on the docKind.
    ///
    /// **Note:** Used automatically by `webShareCreate` with kind=::WebBootDisplayKind_Unknown3.
    ///
    /// **Note:** Used automatically by `webLobbyCreate` with kind=::WebBootDisplayKind_Unknown4.
    pub fn webConfigSetBootDisplayKind(config: *mut WebCommonConfig, kind: WebBootDisplayKind) -> Result;

    /// Sets the BackgroundKind.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `kind`: `WebBackgroundKind`
    ///
    /// **Note:** Only available with config created by `webOfflineCreate`, `webPageCreate`, or `webLobbyCreate`.
    ///
    /// **Note:** Used automatically by `webOfflineCreate`, depending on the docKind.
    ///
    /// **Note:** Used automatically by `webLobbyCreate` with kind=2.
    pub fn webConfigSetBackgroundKind(config: *mut WebCommonConfig, kind: WebBackgroundKind) -> Result;

    /// Sets the whether the UI footer is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`.
    ///
    /// **Note:** Used automatically by `webOfflineCreate`, depending on the docKind.
    pub fn webConfigSetFooter(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the whether the Pointer is enabled. See also `WebLeftStickMode`.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate`, `webPageCreate`, or `webLobbyCreate`.
    ///
    /// **Note:** Used automatically by `webOfflineCreate`.
    ///
    /// **Note:** Used automatically by `webLobbyCreate` with flag=false on [3.0.0+].
    pub fn webConfigSetPointer(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the LeftStickMode.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `mode`: Mode, different enums for Web and Offline.
    ///
    /// **Note:** Only available with config created by `webOfflineCreate`, `webShareCreate`, `webPageCreate`, or `webLobbyCreate`.
    ///
    /// **Note:** Used automatically by `webOfflineCreate`, `webShareCreate`, and `webLobbyCreate` with ::WebLeftStickMode_Cursor.
    pub fn webConfigSetLeftStickMode(config: *mut WebCommonConfig, mode: WebLeftStickMode) -> Result;

    /// Sets the KeyRepeatFrame.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `inval0`: First input param.
    /// - `inval1`: Second input param.
    ///
    /// **Note:** Only available with config created by `webOfflineCreate`.
    pub fn webConfigSetKeyRepeatFrame(config: *mut WebCommonConfig, inval0: s32, inval1: s32) -> Result;

    /// Sets the DisplayUrlKind.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `kind`: Kind
    ///
    /// **Note:** Only available with config created by `webShareCreate` or `webPageCreate`.
    ///
    /// **Note:** Used automatically by `webShareCreate` with kind=true.
    pub fn webConfigSetDisplayUrlKind(config: *mut WebCommonConfig, kind: bool) -> Result;

    /// Sets the BootAsMediaPlayer flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag. true = BootAsMediaPlayer, false = BootAsWebPage.
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [2.0.0+].
    ///
    /// **Note:** With config created by `webNewsCreate` on [3.0.0+], this also sets ::WebArgType_BootAsMediaPlayerInverted to !flag.
    pub fn webConfigSetBootAsMediaPlayer(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the ShopJump flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webPageCreate` on [2.0.0+].
    ///
    /// **Note:** Used automatically by `webNewsCreate` with flag=true.
    pub fn webConfigSetShopJump(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the MediaPlayerUserGestureRestriction flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webPageCreate` on [2.0.0-5.1.0].
    pub fn webConfigSetMediaPlayerUserGestureRestriction(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets whether MediaAutoPlay is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [6.0.0+].
    pub fn webConfigSetMediaAutoPlay(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the LobbyParameter.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `str`: String
    ///
    /// **Note:** Only available with config created by `webLobbyCreate`.
    pub fn webConfigSetLobbyParameter(config: *mut WebCommonConfig, str: *const c_char) -> Result;

    /// Sets the Share CapsApplicationAlbumEntry.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `entry`: `CapsApplicationAlbumEntry`, see also capssu.h.
    ///
    /// **Note:** Only available with config created by `webShareCreate` on [3.0.0+].
    pub fn webConfigSetApplicationAlbumEntry(config: *mut WebCommonConfig, entry: *mut CapsApplicationAlbumEntry) -> Result;

    /// Sets whether JsExtension is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [3.0.0+].
    pub fn webConfigSetJsExtension(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the Share AdditionalCommentText.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `str`: String
    ///
    /// **Note:** Only available with config created by `webShareCreate` on [4.0.0+].
    pub fn webConfigSetAdditionalCommentText(config: *mut WebCommonConfig, str: *const c_char) -> Result;

    /// Sets the TouchEnabledOnContents flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [4.0.0+].
    pub fn webConfigSetTouchEnabledOnContents(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the UserAgentAdditionalString. " " followed by this string are appended to the normal User-Agent string.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `str`: String
    ///
    /// **Note:** Only available with config created by `webPageCreate` on [4.0.0+].
    pub fn webConfigSetUserAgentAdditionalString(config: *mut WebCommonConfig, str: *const c_char) -> Result;

    /// Sets the Share AdditionalMediaData.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `data`: Input data
    /// - `size`: Size of the input data, max size is 0x10.
    ///
    /// **Note:** Only available with config created by `webShareCreate` on [4.0.0+].
    pub fn webConfigSetAdditionalMediaData(config: *mut WebCommonConfig, data: *const u8, size: usize) -> Result;

    /// Sets the MediaPlayerAutoClose flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [4.0.0+].
    pub fn webConfigSetMediaPlayerAutoClose(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets whether PageCache is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [4.0.0+].
    pub fn webConfigSetPageCache(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets whether WebAudio is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [4.0.0+].
    pub fn webConfigSetWebAudio(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the FooterFixedKind.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `kind`: `WebFooterFixedKind`
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [5.0.0+].
    pub fn webConfigSetFooterFixedKind(config: *mut WebCommonConfig, kind: WebFooterFixedKind) -> Result;

    /// Sets the PageFade flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [5.0.0+].
    pub fn webConfigSetPageFade(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the Share MediaCreatorApplicationRatingAge.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `data`: 0x20-byte input data
    ///
    /// **Note:** Only available with config created by `webShareCreate` on [5.0.0+].
    pub fn webConfigSetMediaCreatorApplicationRatingAge(config: *mut WebCommonConfig, data: *const s8) -> Result;

    /// Sets the BootLoadingIcon flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` on [5.0.0+].
    pub fn webConfigSetBootLoadingIcon(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets the PageScrollIndicator flag.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [5.0.0+].
    pub fn webConfigSetPageScrollIndicator(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets whether MediaPlayerSpeedControl is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [6.0.0+].
    pub fn webConfigSetMediaPlayerSpeedControl(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Adds a pair of Share CapsAlbumEntry + optionally AdditionalMediaData. This can be used up to 4 times, for setting multiple pairs.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `entry`: `CapsAlbumEntry`
    /// - `data`: Input data for AdditionalMediaData. Optional, can be NULL.
    /// - `size`: Size of the input data, max size is 0x10. Optional, can be 0.
    ///
    /// **Note:** Only available with config created by `webShareCreate` on [6.0.0+].
    pub fn webConfigAddAlbumEntryAndMediaData(config: *mut WebCommonConfig, entry: *const CapsAlbumEntry, data: *const u8, size: usize) -> Result;

    /// Sets whether the specified BootFooterButton is visible.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `button`: `WebFooterButtonId`
    /// - `visible`: Visible flag.
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` on [6.0.0+].
    pub fn webConfigSetBootFooterButtonVisible(config: *mut WebCommonConfig, button: WebFooterButtonId, visible: bool) -> Result;

    /// Sets OverrideWebAudioVolume.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `value`: Value
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [6.0.0+].
    pub fn webConfigSetOverrideWebAudioVolume(config: *mut WebCommonConfig, value: f32) -> Result;

    /// Sets OverrideMediaAudioVolume.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `value`: Value
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [6.0.0+].
    pub fn webConfigSetOverrideMediaAudioVolume(config: *mut WebCommonConfig, value: f32) -> Result;

    /// Sets `WebSessionBootMode`.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `mode`: `WebSessionBootMode`
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [7.0.0+].
    pub fn webConfigSetBootMode(config: *mut WebCommonConfig, mode: WebSessionBootMode) -> Result;

    /// Sets whether MediaPlayerUi is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` on [8.0.0+].
    pub fn webConfigSetMediaPlayerUi(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Sets whether TransferMemory is enabled.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `flag`: Flag
    ///
    /// **Note:** Only available with config created by `webPageCreate` on [11.0.0+].
    pub fn webConfigSetTransferMemory(config: *mut WebCommonConfig, flag: bool) -> Result;

    /// Launches the {web applet} with the specified config and waits for it to exit.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    /// - `out`: Optional output applet reply data, can be NULL.
    pub fn webConfigShow(config: *mut WebCommonConfig, out: *mut WebCommonReply) -> Result;

    /// Request the applet to exit after `webConfigShow` was used, while the applet is still running. This is for use from another thread.
    ///
    /// # Arguments
    /// - `config`: WebCommonConfig object.
    pub fn webConfigRequestExit(config: *mut WebCommonConfig) -> Result;

    /// Gets the ExitReason from the specified reply.
    ///
    /// # Arguments
    /// - `reply`: WebCommonReply object.
    /// - `exitReason`: Output `WebExitReason`
    pub fn webReplyGetExitReason(reply: *mut WebCommonReply, exitReason: *mut WebExitReason) -> Result;

    /// Gets the LastUrl from the specified reply. When the applet loads a page where the beginning of the URL matches the URL from `webConfigSetCallbackUrl`, the applet will exit and set LastUrl to that URL (exit doesn't occur when `webConfigSetCallbackableUrl` was used).
    ///
    /// # Arguments
    /// - `reply`: WebCommonReply object.
    /// - `outstr`: Output string buffer. If NULL, the string is not loaded.
    /// - `outstr_maxsize`: Size of the buffer, including NUL-terminator. If outstr is set, this size must be >1. The size used for the actual string-copy is this size-1, to make sure the output is NUL-terminated (the entire buffer is cleared first).
    /// - `out_size`: Output string length including NUL-terminator, for the original input string in the reply loaded from a separate size field.
    ///
    /// **Note:** This is only available with ::WebExitReason_LastUrl (string is empty otherwise).
    ///
    /// **Note:** If you want to allocate a string buffer on heap, you can call this with outstr=NULL/outstr_maxsize=0 to get the out_size, then call it again with the allocated buffer.
    pub fn webReplyGetLastUrl(reply: *mut WebCommonReply, outstr: *mut c_char, outstr_maxsize: usize, out_size: *mut usize) -> Result;

    /// Gets the SharePostResult from the specified reply.
    ///
    /// # Arguments
    /// - `reply`: WebCommonReply object.
    /// - `sharePostResult`: Output sharePostResult
    ///
    /// **Note:** Only available with reply data from ShareApplet on [3.0.0+].
    pub fn webReplyGetSharePostResult(reply: *mut WebCommonReply, sharePostResult: *mut u32) -> Result;

    /// Gets the PostServiceName from the specified reply.
    ///
    /// # Arguments
    /// - `reply`: WebCommonReply object.
    /// - `outstr`: Output string buffer. If NULL, the string is not loaded.
    /// - `outstr_maxsize`: Size of the buffer, including NUL-terminator. If outstr is set, this size must be >1. The size used for the actual string-copy is this size-1, to make sure the output is NUL-terminated (the entire buffer is cleared first).
    /// - `out_size`: Output string length including NUL-terminator, for the original input string in the reply loaded from a separate size field.
    ///
    /// **Note:** Only available with reply data from ShareApplet on [3.0.0+].
    ///
    /// **Note:** If you want to allocate a string buffer on heap, you can call this with outstr=NULL/outstr_maxsize=0 to get the out_size, then call it again with the allocated buffer.
    pub fn webReplyGetPostServiceName(reply: *mut WebCommonReply, outstr: *mut c_char, outstr_maxsize: usize, out_size: *mut usize) -> Result;

    /// Gets the PostId from the specified reply.
    ///
    /// # Arguments
    /// - `reply`: WebCommonReply object.
    /// - `outstr`: Output string buffer. If NULL, the string is not loaded.
    /// - `outstr_maxsize`: Size of the buffer, including NUL-terminator. If outstr is set, this size must be >1. The size used for the actual string-copy is this size-1, to make sure the output is NUL-terminated (the entire buffer is cleared first).
    /// - `out_size`: Output string length including NUL-terminator, for the original input string in the reply loaded from a separate size field.
    ///
    /// **Note:** Only available with reply data from ShareApplet on [3.0.0+].
    ///
    /// **Note:** If you want to allocate a string buffer on heap, you can call this with outstr=NULL/outstr_maxsize=0 to get the out_size, then call it again with the allocated buffer.
    pub fn webReplyGetPostId(reply: *mut WebCommonReply, outstr: *mut c_char, outstr_maxsize: usize, out_size: *mut usize) -> Result;

    /// Gets the MediaPlayerAutoClosedByCompletion flag from the specified reply.
    ///
    /// # Arguments
    /// - `reply`: WebCommonReply object.
    /// - `flag`: Output flag
    ///
    /// **Note:** Only available with reply data from Web on [8.0.0+].
    pub fn webReplyGetMediaPlayerAutoClosedByCompletion(reply: *mut WebCommonReply, flag: *mut bool) -> Result;

    /// Creates a `WebSession` object.
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    /// - `config`: WebCommonConfig object.
    pub fn webSessionCreate(s: *mut WebSession, config: *mut WebCommonConfig);

    /// Closes a `WebSession` object.
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    pub fn webSessionClose(s: *mut WebSession);

    /// Launches the applet for `WebSession`.
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    /// - `out_event` (out): Output Event with autoclear=false, from `appletHolderGetExitEvent`. Optional, can be NULL.
    ///
    /// **Note:** Only available with config created by `webOfflineCreate` or `webPageCreate`, on [7.0.0+].
    ///
    /// **Note:** Do not use `webConfigShow` when using WebSession.
    pub fn webSessionStart(s: *mut WebSession, out_event: *mut *mut Event) -> Result;

    /// Waits for the applet to exit.
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    /// - `out`: Optional output applet reply data, can be NULL.
    ///
    /// **Note:** This must be used before `webSessionClose`, when `webSessionStart` was used successfully.
    pub fn webSessionWaitForExit(s: *mut WebSession, out: *mut WebCommonReply) -> Result;

    /// Request the applet to exit.
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    ///
    /// **Note:** Use this instead of `webConfigRequestExit`, when using WebSession.
    pub fn webSessionRequestExit(s: *mut WebSession) -> Result;

    /// Request the applet to Appear, this is only needed with ::WebSessionBootMode_AllForegroundInitiallyHidden.
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    /// - `flag` (out): Whether the message was sent successfully.
    ///
    /// **Note:** This should not be used before `webSessionStart`.
    pub fn webSessionAppear(s: *mut WebSession, flag: *mut bool) -> Result;

    /// TrySendContentMessage
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    /// - `content` (in): Input content NUL-terminated string.
    /// - `size` (in): Size of content.
    /// - `flag` (out): Whether the message was sent successfully.
    ///
    /// **Note:** This should not be used before `webSessionStart`.
    ///
    /// **Note:** The JS-side for this is only available when JsExtension is enabled via `webConfigSetJsExtension`.
    ///
    /// **Note:** The JS-side may ignore this if it's sent too soon after the applet launches.
    pub fn webSessionTrySendContentMessage(s: *mut WebSession, content: *const c_char, size: u32, flag: *mut bool) -> Result;

    /// TryReceiveContentMessage
    ///
    /// # Arguments
    /// - `s`: `WebSession`
    /// - `content` (out): Output content string, always NUL-terminated.
    /// - `size` (in): Max size of content.
    /// - `out_size` (out): Original content size, prior to being clamped to the specified size param.
    /// - `flag` (out): Whether the message was received successfully.
    ///
    /// **Note:** This should not be used before `webSessionStart`.
    ///
    /// **Note:** The JS-side for this is only available when JsExtension is enabled via `webConfigSetJsExtension`.
    pub fn webSessionTryReceiveContentMessage(s: *mut WebSession, content: *mut c_char, size: u64, out_size: *mut u64, flag: *mut bool) -> Result;
}
