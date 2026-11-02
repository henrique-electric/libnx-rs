//! `switch/services/irs.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub const IRS_MAX_CAMERAS: u32 = 0x9;

/// IrCameraStatus
pub type IrsIrCameraStatus = u32;
/// Available
pub const IrsIrCameraStatus_Available: IrsIrCameraStatus = 0;
/// Unsupported
pub const IrsIrCameraStatus_Unsupported: IrsIrCameraStatus = 1;
/// Unconnected
pub const IrsIrCameraStatus_Unconnected: IrsIrCameraStatus = 2;

/// IrCameraInternalStatus
pub type IrsIrCameraInternalStatus = u32;
/// Stopped
pub const IrsIrCameraInternalStatus_Stopped: IrsIrCameraInternalStatus = 0;
/// FirmwareUpdateNeeded
pub const IrsIrCameraInternalStatus_FirmwareUpdateNeeded: IrsIrCameraInternalStatus = 1;
/// Unknown
pub const IrsIrCameraInternalStatus_Unknown2: IrsIrCameraInternalStatus = 2;
/// Unknown
pub const IrsIrCameraInternalStatus_Unknown3: IrsIrCameraInternalStatus = 3;
/// Unknown
pub const IrsIrCameraInternalStatus_Unknown4: IrsIrCameraInternalStatus = 4;
/// FirmwareVersionRequested
pub const IrsIrCameraInternalStatus_FirmwareVersionRequested: IrsIrCameraInternalStatus = 5;
/// FirmwareVersionIsInvalid
pub const IrsIrCameraInternalStatus_FirmwareVersionIsInvalid: IrsIrCameraInternalStatus = 6;
/// [4.0.0+] Ready
pub const IrsIrCameraInternalStatus_Ready: IrsIrCameraInternalStatus = 7;
/// [4.0.0+] Setting
pub const IrsIrCameraInternalStatus_Setting: IrsIrCameraInternalStatus = 8;

/// IrSensorMode
pub type IrsIrSensorMode = u32;
/// None
pub const IrsIrSensorMode_None: IrsIrSensorMode = 0;
/// MomentProcessor
pub const IrsIrSensorMode_MomentProcessor: IrsIrSensorMode = 1;
/// ClusteringProcessor
pub const IrsIrSensorMode_ClusteringProcessor: IrsIrSensorMode = 2;
/// ImageTransferProcessor
pub const IrsIrSensorMode_ImageTransferProcessor: IrsIrSensorMode = 3;
/// PointingProcessor
pub const IrsIrSensorMode_PointingProcessor: IrsIrSensorMode = 4;
/// TeraPluginProcessor
pub const IrsIrSensorMode_TeraPluginProcessor: IrsIrSensorMode = 5;
/// IrLedProcessor (doesn't apply to IrsDeviceFormat::ir_sensor_mode)
pub const IrsIrSensorMode_IrLedProcessor: IrsIrSensorMode = 6;

/// ImageProcessorStatus
pub type IrsImageProcessorStatus = u32;
/// Stopped
pub const IrsImageProcessorStatus_Stopped: IrsImageProcessorStatus = 0;
/// Running
pub const IrsImageProcessorStatus_Running: IrsImageProcessorStatus = 1;

/// ImageTransferProcessorFormat. IR Sensor image resolution.
pub type IrsImageTransferProcessorFormat = u32;
/// 320x240
pub const IrsImageTransferProcessorFormat_320x240: IrsImageTransferProcessorFormat = 0;
/// 160x120
pub const IrsImageTransferProcessorFormat_160x120: IrsImageTransferProcessorFormat = 1;
/// 80x60
pub const IrsImageTransferProcessorFormat_80x60: IrsImageTransferProcessorFormat = 2;
/// [4.0.0+] 40x30
pub const IrsImageTransferProcessorFormat_40x30: IrsImageTransferProcessorFormat = 3;
/// [4.0.0+] 20x15
pub const IrsImageTransferProcessorFormat_20x15: IrsImageTransferProcessorFormat = 4;

/// AdaptiveClusteringMode
pub type IrsAdaptiveClusteringMode = u32;
/// StaticFov
pub const IrsAdaptiveClusteringMode_StaticFov: IrsAdaptiveClusteringMode = 0;
/// DynamicFov
pub const IrsAdaptiveClusteringMode_DynamicFov: IrsAdaptiveClusteringMode = 1;

/// AdaptiveClusteringTargetDistance
pub type IrsAdaptiveClusteringTargetDistance = u32;
/// Near
pub const IrsAdaptiveClusteringTargetDistance_Near: IrsAdaptiveClusteringTargetDistance = 0;
/// Middle
pub const IrsAdaptiveClusteringTargetDistance_Middle: IrsAdaptiveClusteringTargetDistance = 1;
/// Far
pub const IrsAdaptiveClusteringTargetDistance_Far: IrsAdaptiveClusteringTargetDistance = 2;

/// HandAnalysisMode
pub type IrsHandAnalysisMode = u32;
/// Silhouette
pub const IrsHandAnalysisMode_Silhouette: IrsHandAnalysisMode = 1;
/// Image
pub const IrsHandAnalysisMode_Image: IrsHandAnalysisMode = 2;
/// SilhouetteAndImage
pub const IrsHandAnalysisMode_SilhouetteAndImage: IrsHandAnalysisMode = 3;
/// [4.0.0+] SilhouetteOnly
pub const IrsHandAnalysisMode_SilhouetteOnly: IrsHandAnalysisMode = 4;

/// Internal validation callblack.
pub type IrsValidationCb = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>;

/// IrCameraHandle
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsIrCameraHandle {
    /// PlayerNumber
    pub player_number: u8,
    /// DeviceType
    pub device_type: u8,
    /// Reserved
    pub reserved: [u8; 2],
}

/// PackedMcuVersion
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedMcuVersion {
    /// MajorVersion
    pub major_version: u16,
    /// MinorVersion
    pub minor_version: u16,
}

/// PackedFunctionLevel
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedFunctionLevel {
    /// IrSensorFunctionLevel
    pub ir_sensor_function_level: u8,
    /// Reserved
    pub reserved: [u8; 3],
}

/// Rect
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsRect {
    /// X
    pub x: s16,
    /// Y
    pub y: s16,
    /// Width
    pub width: s16,
    /// Height
    pub height: s16,
}

/// IrsMomentProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsMomentProcessorConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u32,
    /// IR sensor signal's digital gain.
    pub gain: u32,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 7],
    /// WindowOfInterest
    pub window_of_interest: IrsRect,
    /// Preprocess
    pub preprocess: u32,
    /// PreprocessIntensityThreshold
    pub preprocess_intensity_threshold: u32,
}

/// PackedMomentProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedMomentProcessorConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u8,
    /// IR sensor signal's digital gain.
    pub gain: u8,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 5],
    /// WindowOfInterest
    pub window_of_interest: IrsRect,
    /// RequiredMcuVersion
    pub required_mcu_version: IrsPackedMcuVersion,
    /// Preprocess
    pub preprocess: u8,
    /// PreprocessIntensityThreshold
    pub preprocess_intensity_threshold: u8,
    /// Reserved.
    pub reserved2: [u8; 2],
}

/// ClusteringProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsClusteringProcessorConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u32,
    /// IR sensor signal's digital gain.
    pub gain: u32,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 7],
    /// WindowOfInterest
    pub window_of_interest: IrsRect,
    /// ObjectPixelCountMin
    pub object_pixel_count_min: u32,
    /// ObjectPixelCountMax
    pub object_pixel_count_max: u32,
    /// ObjectIntensityMin
    pub object_intensity_min: u32,
    /// IsExternalLightFilterEnabled
    pub is_external_light_filter_enabled: u8,
}

/// PackedClusteringProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedClusteringProcessorConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u8,
    /// IR sensor signal's digital gain.
    pub gain: u8,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 5],
    /// WindowOfInterest
    pub window_of_interest: IrsRect,
    /// RequiredMcuVersion
    pub required_mcu_version: IrsPackedMcuVersion,
    /// ObjectPixelCountMin
    pub object_pixel_count_min: u32,
    /// ObjectPixelCountMax
    pub object_pixel_count_max: u32,
    /// ObjectIntensityMin
    pub object_intensity_min: u8,
    /// IsExternalLightFilterEnabled
    pub is_external_light_filter_enabled: u8,
    /// Reserved.
    pub reserved2: [u8; 2],
}

/// ImageTransferProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsImageTransferProcessorConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u32,
    /// IR sensor signal's digital gain.
    pub gain: u32,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 7],
    /// **Ref:** IrsImageTransferProcessorFormat
    pub format: u32,
}

/// ImageTransferProcessorExConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsImageTransferProcessorExConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u32,
    /// IR sensor signal's digital gain.
    pub gain: u32,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 7],
    /// OrigFormat `IrsImageTransferProcessorFormat`
    pub orig_format: u32,
    /// TrimmingFormat `IrsImageTransferProcessorFormat`
    pub trimming_format: u32,
    /// TrimmingStartX
    pub trimming_start_x: u16,
    /// TrimmingStartY
    pub trimming_start_y: u16,
    /// IsExternalLightFilterEnabled
    pub is_external_light_filter_enabled: u8,
}

/// PackedImageTransferProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedImageTransferProcessorConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u8,
    /// IR sensor signal's digital gain.
    pub gain: u8,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 5],
    /// RequiredMcuVersion
    pub required_mcu_version: IrsPackedMcuVersion,
    /// **Ref:** IrsImageTransferProcessorFormat
    pub format: u8,
    /// Reserved.
    pub reserved2: [u8; 3],
}

/// PackedImageTransferProcessorExConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedImageTransferProcessorExConfig {
    /// IR Sensor exposure time in nanoseconds.
    pub exposure_time: u64,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u8,
    /// IR sensor signal's digital gain.
    pub gain: u8,
    /// Inverts the colors of the captured image. 0: Normal image, 1: Negative image.
    pub is_negative_image_used: u8,
    /// Reserved.
    pub reserved: [u8; 5],
    /// RequiredMcuVersion
    pub required_mcu_version: IrsPackedMcuVersion,
    /// OrigFormat `IrsImageTransferProcessorFormat`
    pub orig_format: u8,
    /// TrimmingFormat `IrsImageTransferProcessorFormat`
    pub trimming_format: u8,
    /// TrimmingStartX
    pub trimming_start_x: u16,
    /// TrimmingStartY
    pub trimming_start_y: u16,
    /// IsExternalLightFilterEnabled
    pub is_external_light_filter_enabled: u8,
    /// Reserved.
    pub reserved2: [u8; 5],
}

/// ImageTransferProcessorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsImageTransferProcessorState {
    /// SamplingNumber
    pub sampling_number: u64,
    /// AmbientNoiseLevel
    pub ambient_noise_level: u32,
    /// Reserved
    pub reserved: [u8; 4],
}

/// PackedPointingProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedPointingProcessorConfig {
    /// WindowOfInterest
    pub window_of_interest: IrsRect,
    /// RequiredMcuVersion
    pub required_mcu_version: IrsPackedMcuVersion,
}

/// TeraPluginProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsTeraPluginProcessorConfig {
    /// Mode
    pub mode: u8,
    /// [6.0.0+] Unknown
    pub unk_x1: u8,
    /// [6.0.0+] Unknown
    pub unk_x2: u8,
    /// [6.0.0+] Unknown
    pub unk_x3: u8,
}

/// PackedTeraPluginProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedTeraPluginProcessorConfig {
    /// RequiredMcuVersion
    pub required_mcu_version: IrsPackedMcuVersion,
    /// Mode
    pub mode: u8,
    /// [6.0.0+] This is set to 0x2 | (IrsTeraPluginProcessorConfig::unk_x1 &lt;&lt; 7).
    pub unk_x5: u8,
    /// [6.0.0+] IrsTeraPluginProcessorConfig::unk_x2
    pub unk_x6: u8,
    /// [6.0.0+] IrsTeraPluginProcessorConfig::unk_x3
    pub unk_x7: u8,
}

/// IrLedProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsIrLedProcessorConfig {
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u32,
}

/// PackedIrLedProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPackedIrLedProcessorConfig {
    /// RequiredMcuVersion
    pub required_mcu_version: IrsPackedMcuVersion,
    /// Controls the IR leds. 0: All leds, 1: Bright group, 2: Dim group, 3: None.
    pub light_target: u8,
    /// Padding
    pub pad: [u8; 3],
}

/// AdaptiveClusteringProcessorConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsAdaptiveClusteringProcessorConfig {
    /// **Ref:** IrsAdaptiveClusteringMode
    pub mode: u32,
    /// [6.0.0+] `IrsAdaptiveClusteringTargetDistance`
    pub target_distance: u32,
}

/// HandAnalysisConfig
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsHandAnalysisConfig {
    /// **Ref:** IrsHandAnalysisMode
    pub mode: u32,
}

/// MomentStatistic
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsMomentStatistic {
    /// AverageIntensity
    pub average_intensity: f32,
    /// CentroidX
    pub centroid_x: f32,
    /// CentroidY
    pub centroid_y: f32,
}

/// MomentProcessorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsMomentProcessorState {
    /// SamplingNumber
    pub sampling_number: s64,
    /// TimeStamp
    pub timestamp: u64,
    /// AmbientNoiseLevel
    pub ambient_noise_level: u32,
    /// Reserved
    pub reserved: [u8; 4],
    /// **Ref:** IrsMomentStatistic
    pub statistic: [IrsMomentStatistic; 48],
}

/// ClusteringData
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsClusteringData {
    /// AverageIntensity
    pub average_intensity: f32,
    /// CentroidX
    pub centroid_x: f32,
    /// CentroidY
    pub centroid_y: f32,
    /// PixelCount
    pub pixel_count: u32,
    /// BoundX
    pub bound_x: u16,
    /// BoundY
    pub bound_y: u16,
    /// BoundtWidth
    pub boundt_width: u16,
    /// BoundHeight
    pub bound_height: u16,
}

/// ClusteringProcessorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsClusteringProcessorState {
    /// SamplingNumber
    pub sampling_number: s64,
    /// TimeStamp
    pub timestamp: u64,
    /// ObjectCount
    pub object_count: u8,
    /// Reserved
    pub reserved: [u8; 3],
    /// AmbientNoiseLevel
    pub ambient_noise_level: u32,
    /// **Ref:** IrsClusteringData
    pub data: [IrsClusteringData; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPointingProcessorMarkerState_data {
    /// PointingStatus
    pub pointing_status: u8,
    /// Reserved
    pub reserved: [u8; 3],
    /// Unknown
    pub unk_x4: [u8; 4],
    /// Unknown
    pub unk_x8: f32,
    /// PositionX
    pub position_x: f32,
    /// PositionY
    pub position_y: f32,
    /// Unknown
    pub unk_x14: f32,
    /// WindowOfInterest
    pub window_of_interest: IrsRect,
}

/// PointingProcessorMarkerState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPointingProcessorMarkerState {
    /// SamplingNumber
    pub sampling_number: s64,
    /// TimeStamp
    pub timestamp: u64,
    pub data: [IrsPointingProcessorMarkerState_data; 3],
}

/// PointingProcessorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsPointingProcessorState {
    /// SamplingNumber
    pub sampling_number: s64,
    /// TimeStamp
    pub timestamp: u64,
    /// PointingStatus
    pub pointing_status: u32,
    /// PositionX
    pub position_x: f32,
    /// PositionY
    pub position_y: f32,
    /// Reserved
    pub reserved: [u8; 4],
}

/// TeraPluginProcessorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsTeraPluginProcessorState {
    /// SamplingNumber
    pub sampling_number: s64,
    /// TimeStamp
    pub timestamp: u64,
    /// AmbientNoiseLevel
    pub ambient_noise_level: u32,
    /// PluginData
    pub plugin_data: [u8; 300],
}

/// ProcessorState
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsProcessorState {
    /// Start
    pub start: s64,
    /// Count
    pub count: u32,
    /// Padding
    pub pad: u32,
    /// Contains an array of *ProcessorState, depending on IrsDeviceFormat::ir_sensor_mode.
    pub data: [u8; 3600],
}

/// DeviceFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsDeviceFormat {
    /// **Ref:** IrsIrCameraStatus
    pub ir_camera_status: u32,
    /// **Ref:** IrsIrCameraInternalStatus
    pub ir_camera_internal_status: u32,
    /// **Ref:** IrsIrSensorMode
    pub ir_sensor_mode: u32,
    /// Padding
    pub pad: u32,
    /// **Ref:** IrsProcessorState
    pub processor_state: IrsProcessorState,
}

/// AruidFormat
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsAruidFormat {
    /// IrSensorAruid
    pub ir_sensor_aruid: u64,
    /// IrSensorAruidStatus
    pub ir_sensor_aruid_status: u32,
    /// Padding
    pub pad: u32,
}

/// StatusManager
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrsStatusManager {
    pub device_format: [IrsDeviceFormat; 9],
    pub aruid_format: [IrsAruidFormat; 5],
}

/// Gets the default configuration for IrLedProcessor.
#[inline(always)]
pub fn irsGetIrLedProcessorDefaultConfig(config: &mut IrsIrLedProcessorConfig) {
    config.light_target = 0;
}

impl_zeroed_default!(IrsIrCameraHandle, IrsPackedMcuVersion, IrsPackedFunctionLevel, IrsRect, IrsMomentProcessorConfig, IrsPackedMomentProcessorConfig, IrsClusteringProcessorConfig, IrsPackedClusteringProcessorConfig, IrsImageTransferProcessorConfig, IrsImageTransferProcessorExConfig, IrsPackedImageTransferProcessorConfig, IrsPackedImageTransferProcessorExConfig, IrsImageTransferProcessorState, IrsPackedPointingProcessorConfig, IrsTeraPluginProcessorConfig, IrsPackedTeraPluginProcessorConfig, IrsIrLedProcessorConfig, IrsPackedIrLedProcessorConfig, IrsAdaptiveClusteringProcessorConfig, IrsHandAnalysisConfig, IrsMomentStatistic, IrsMomentProcessorState, IrsClusteringData, IrsClusteringProcessorState, IrsPointingProcessorMarkerState_data, IrsPointingProcessorMarkerState, IrsPointingProcessorState, IrsTeraPluginProcessorState, IrsProcessorState, IrsDeviceFormat, IrsAruidFormat, IrsStatusManager);

unsafe extern "C" {
    /// Initialize irs.
    pub fn irsInitialize() -> Result;

    /// Exit irs.
    pub fn irsExit();

    /// Gets the Service object for the actual irs service session.
    pub fn irsGetServiceSession() -> *mut Service;

    /// Gets the address of the SharedMemory (`IrsStatusManager`).
    pub fn irsGetSharedmemAddr() -> *mut c_void;

    /// Gets the `IrsIrCameraHandle` for the specified controller.
    pub fn irsGetIrCameraHandle(handle: *mut IrsIrCameraHandle, id: HidNpadIdType) -> Result;

    /// GetIrCameraStatus
    pub fn irsGetIrCameraStatus(handle: IrsIrCameraHandle, out: *mut IrsIrCameraStatus) -> Result;

    /// CheckFirmwareUpdateNecessity
    /// When successful where the output flag is set, the user should use `hidLaShowControllerFirmwareUpdate`.
    /// Only available on [4.0.0+].
    pub fn irsCheckFirmwareUpdateNecessity(handle: IrsIrCameraHandle, out: *mut bool) -> Result;

    /// GetImageProcessorStatus
    /// Only available on [4.0.0+].
    pub fn irsGetImageProcessorStatus(handle: IrsIrCameraHandle, out: *mut IrsImageProcessorStatus) -> Result;

    /// Stop the current Processor.
    ///
    /// **Ref:** irsExit calls this with all IrCameraHandles which were not already used with `irsStopImageProcessor`.
    pub fn irsStopImageProcessor(handle: IrsIrCameraHandle) -> Result;

    /// Stop the current Processor, async.
    /// Only available on [4.0.0+].
    pub fn irsStopImageProcessorAsync(handle: IrsIrCameraHandle) -> Result;

    /// Run the MomentProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    pub fn irsRunMomentProcessor(handle: IrsIrCameraHandle, config: *const IrsMomentProcessorConfig) -> Result;

    /// Gets the states for MomentProcessor or IrLedProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `states` (out): Output array of `IrsMomentProcessorState`.
    /// - `count` (in): Size of the states array in entries. Must be 1-5.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** The official GetIrLedProcessorState is essentially the same as this, except it uses hard-coded count=1 with output-array on stack, without returning that data. Hence we don't implement a seperate func for that.
    pub fn irsGetMomentProcessorStates(handle: IrsIrCameraHandle, states: *mut IrsMomentProcessorState, count: s32, total_out: *mut s32) -> Result;

    /// Calculates an `IrsMomentStatistic` from the specified region in the input `IrsMomentProcessorState`.
    ///
    /// # Arguments
    /// - `state` (in): `IrsMomentProcessorState`
    /// - `rect` (in): `IrsRect`, containing the image width and height.
    /// - `region_x` (in): Region x, must be 0-5 (clamped to this range otherwise). region_x = image_x/6.
    /// - `region_y` (in): Region y, must be 0-7 (clamped to this range otherwise). region_y = image_y/8.
    /// - `region_width` (in): Region width. region_x+region_width must be &lt;=6 (clamped to this range otherwise).
    /// - `region_height` (in): Region height.  region_y+region_height must be &lt;=8 (clamped to this range otherwise).
    pub fn irsCalculateMomentRegionStatistic(state: *const IrsMomentProcessorState, rect: IrsRect, region_x: s32, region_y: s32, region_width: s32, region_height: s32) -> IrsMomentStatistic;

    /// Run the ClusteringProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    pub fn irsRunClusteringProcessor(handle: IrsIrCameraHandle, config: *const IrsClusteringProcessorConfig) -> Result;

    /// Gets the states for ClusteringProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `states` (out): Output array of `IrsClusteringProcessorState`.
    /// - `count` (in): Size of the states array in entries. Must be 1-5.
    /// - `total_out` (out): Total output entries.
    pub fn irsGetClusteringProcessorStates(handle: IrsIrCameraHandle, states: *mut IrsClusteringProcessorState, count: s32, total_out: *mut s32) -> Result;

    /// Run the ImageTransferProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    /// - `size` (in): Work-buffer size, must be 0x1000-byte aligned.
    pub fn irsRunImageTransferProcessor(handle: IrsIrCameraHandle, config: *const IrsImageTransferProcessorConfig, size: usize) -> Result;

    /// Run the ImageTransferExProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    /// - `size` (in): Work-buffer size, must be 0x1000-byte aligned.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn irsRunImageTransferExProcessor(handle: IrsIrCameraHandle, config: *const IrsImageTransferProcessorExConfig, size: usize) -> Result;

    /// GetImageTransferProcessorState
    pub fn irsGetImageTransferProcessorState(handle: IrsIrCameraHandle, buffer: *mut c_void, size: usize, state: *mut IrsImageTransferProcessorState) -> Result;

    /// Run the PointingProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    pub fn irsRunPointingProcessor(handle: IrsIrCameraHandle) -> Result;

    /// Gets the states for PointingProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `states` (out): Output array of `IrsPointingProcessorMarkerState`.
    /// - `count` (in): Size of the states array in entries. Must be 1-6.
    /// - `total_out` (out): Total output entries.
    pub fn irsGetPointingProcessorMarkerStates(handle: IrsIrCameraHandle, states: *mut IrsPointingProcessorMarkerState, count: s32, total_out: *mut s32) -> Result;

    /// Gets the states for `IrsPointingProcessorState`.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `states` (out): Output array of `IrsPointingProcessorState`.
    /// - `count` (in): Size of the states array in entries. Must be 1-6.
    /// - `total_out` (out): Total output entries.
    ///
    /// **Note:** This uses `irsGetPointingProcessorMarkerStates`, then converts the output to `IrsPointingProcessorState`.
    pub fn irsGetPointingProcessorStates(handle: IrsIrCameraHandle, states: *mut IrsPointingProcessorState, count: s32, total_out: *mut s32) -> Result;

    /// Run the TeraPluginProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    pub fn irsRunTeraPluginProcessor(handle: IrsIrCameraHandle, config: *const IrsTeraPluginProcessorConfig) -> Result;

    /// Gets the states for TeraPluginProcessor, filtered using the input params.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `states` (out): Output array of `IrsTeraPluginProcessorState`.
    /// - `count` (in): Size of the states array in entries. Must be 1-5.
    /// - `sampling_number` (in): Minimum value for IrsTeraPluginProcessorState::sampling_number.
    /// - `prefix_data` (in): Only used when prefix_bitcount is not 0. The first prefix_bitcount bits from prefix_data must match the first prefix_bitcount bits in IrsTeraPluginProcessorState::plugin_data.
    /// - `prefix_bitcount` (in): Total bits for prefix_data.
    /// - `total_out` (out): Total output entries.
    pub fn irsGetTeraPluginProcessorStates(handle: IrsIrCameraHandle, states: *mut IrsTeraPluginProcessorState, count: s32, sampling_number: s64, prefix_data: u32, prefix_bitcount: u32, total_out: *mut s32) -> Result;

    /// Run the IrLedProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    ///
    /// **Note:** Only available on [4.0.0+].
    pub fn irsRunIrLedProcessor(handle: IrsIrCameraHandle, config: *const IrsIrLedProcessorConfig) -> Result;

    /// Run the AdaptiveClusteringProcessor.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    ///
    /// **Note:** Only available on [5.0.0+].
    pub fn irsRunAdaptiveClusteringProcessor(handle: IrsIrCameraHandle, config: *const IrsAdaptiveClusteringProcessorConfig) -> Result;

    /// Run HandAnalysis.
    ///
    /// # Arguments
    /// - `handle` (in): `IrsIrCameraHandle`
    /// - `config` (in): Input config.
    pub fn irsRunHandAnalysis(handle: IrsIrCameraHandle, config: *const IrsHandAnalysisConfig) -> Result;

    /// Gets the default configuration for MomentProcessor.
    pub fn irsGetMomentProcessorDefaultConfig(config: *mut IrsMomentProcessorConfig);

    /// Gets the default configuration for ClusteringProcessor.
    pub fn irsGetClusteringProcessorDefaultConfig(config: *mut IrsClusteringProcessorConfig);

    /// Gets the default configuration for ImageTransferProcessor.
    /// Defaults are exposure 300us, 8x digital gain, the rest is all-zero. Format is ::IrsImageTransferProcessorFormat_320x240.
    pub fn irsGetDefaultImageTransferProcessorConfig(config: *mut IrsImageTransferProcessorConfig);

    /// Gets the default configuration for ImageTransferProcessorEx.
    /// Defaults are exposure 300us, 8x digital gain, the rest is all-zero. OrigFormat/TrimmingFormat are ::IrsImageTransferProcessorFormat_320x240.
    pub fn irsGetDefaultImageTransferProcessorExConfig(config: *mut IrsImageTransferProcessorExConfig);
}
