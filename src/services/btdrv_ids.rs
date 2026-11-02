//! `switch/services/btdrv_ids.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

/// AdType
pub type BtdrvAdType = u32;
/// Flags
pub const BtdrvAdType_Flags: BtdrvAdType = 0x01;
/// Incomplete List of 16-bit Service Class UUIDs
pub const BtdrvAdType_ServiceClassUuid16ListIncomplete: BtdrvAdType = 0x02;
/// Complete List of 16-bit Service Class UUIDs
pub const BtdrvAdType_ServiceClassUuid16ListComplete: BtdrvAdType = 0x03;
/// Incomplete List of 32-bit Service Class UUIDs
pub const BtdrvAdType_ServiceClassUuid32ListIncomplete: BtdrvAdType = 0x04;
/// Complete List of 32-bit Service Class UUIDs
pub const BtdrvAdType_ServiceClassUuid32ListComplete: BtdrvAdType = 0x05;
/// Incomplete List of 128-bit Service Class UUIDs
pub const BtdrvAdType_ServiceClassUuid128ListIncomplete: BtdrvAdType = 0x06;
/// Complete List of 128-bit Service Class UUIDs
pub const BtdrvAdType_ServiceClassUuid128ListComplete: BtdrvAdType = 0x07;
/// Shortened Local Name
pub const BtdrvAdType_ShortenedLocalName: BtdrvAdType = 0x08;
/// Complete Local Name
pub const BtdrvAdType_CompleteLocalName: BtdrvAdType = 0x09;
/// Tx Power Level
pub const BtdrvAdType_TxPowerLevel: BtdrvAdType = 0x0A;
/// Class of Device
pub const BtdrvAdType_ClassOfDevice: BtdrvAdType = 0x0D;
/// Simple Pairing Hash C-192
pub const BtdrvAdType_SimplePairingHashC192: BtdrvAdType = 0x0E;
/// Simple Pairing Randomizer R-192
pub const BtdrvAdType_SimplePairingRandomizerR192: BtdrvAdType = 0x0F;
/// Device ID
pub const BtdrvAdType_DeviceId: BtdrvAdType = 0x10;
/// Security Manager TK Value
pub const BtdrvAdType_SecurityManagerTkValue: BtdrvAdType = 0x10;
/// Security Manager Out of Band Flags
pub const BtdrvAdType_SecurityManagerOutOfBandFlags: BtdrvAdType = 0x11;
/// Peripheral Connection Interval Range
pub const BtdrvAdType_PeripheralConnectionIntervalRange: BtdrvAdType = 0x12;
/// List of 16-bit Service Solicitation UUIDs
pub const BtdrvAdType_ServiceSolicitationUuid16List: BtdrvAdType = 0x14;
/// List of 128-bit Service Solicitation UUIDs
pub const BtdrvAdType_ServiceSolicitationUuid128List: BtdrvAdType = 0x15;
/// Service Data - 16-bit UUID
pub const BtdrvAdType_ServiceDataUuid16: BtdrvAdType = 0x16;
/// Public Target Address
pub const BtdrvAdType_PublicTargetAddress: BtdrvAdType = 0x17;
/// Random Target Address
pub const BtdrvAdType_RandomTargetAddress: BtdrvAdType = 0x18;
/// Appearance
pub const BtdrvAdType_Appearance: BtdrvAdType = 0x19;
/// Advertising Interval
pub const BtdrvAdType_AdvertisingInterval: BtdrvAdType = 0x1A;
/// LE Bluetooth Device Address
pub const BtdrvAdType_LeBluetoothDeviceAddress: BtdrvAdType = 0x1B;
/// LE Role
pub const BtdrvAdType_LeRole: BtdrvAdType = 0x1C;
/// Simple Pairing Hash C-256
pub const BtdrvAdType_SimplePairingHashC256: BtdrvAdType = 0x1D;
/// Simple Pairing Randomizer R-256
pub const BtdrvAdType_SimplePairingRandomizerR256: BtdrvAdType = 0x1E;
/// List of 32-bit Service Solicitation UUIDs
pub const BtdrvAdType_ServiceSolicitationUuid32List: BtdrvAdType = 0x1F;
/// Service Data - 32-bit UUID
pub const BtdrvAdType_ServiceDataUuid32: BtdrvAdType = 0x20;
/// Service Data - 128-bit UUID
pub const BtdrvAdType_ServiceDataUuid128: BtdrvAdType = 0x21;
/// LE Secure Connections Confirmation Value
pub const BtdrvAdType_LeSecureConnectionsConfirmationValue: BtdrvAdType = 0x22;
/// LE Secure Connections Random Value
pub const BtdrvAdType_LeSecureConnectionsRandomValue: BtdrvAdType = 0x23;
/// URI
pub const BtdrvAdType_Uri: BtdrvAdType = 0x24;
/// Indoor Positioning
pub const BtdrvAdType_IndoorPositioning: BtdrvAdType = 0x25;
/// Transport Discovery Data
pub const BtdrvAdType_TransportDiscoveryData: BtdrvAdType = 0x26;
/// LE Supported Features
pub const BtdrvAdType_LeSupportedFeatures: BtdrvAdType = 0x27;
/// Channel Map Update Indication
pub const BtdrvAdType_ChannelMapUpdateIndication: BtdrvAdType = 0x28;
/// PB-ADV
pub const BtdrvAdType_PbAdv: BtdrvAdType = 0x29;
/// Mesh Message
pub const BtdrvAdType_MeshMessage: BtdrvAdType = 0x2A;
/// Mesh Beacon
pub const BtdrvAdType_MeshBeacon: BtdrvAdType = 0x2B;
/// BIGInfo
pub const BtdrvAdType_Biginfo: BtdrvAdType = 0x2C;
/// Broadcast_Code
pub const BtdrvAdType_BroadcastCode: BtdrvAdType = 0x2D;
/// Resolvable Set Identifier
pub const BtdrvAdType_ResolvableSetIdentifier: BtdrvAdType = 0x2E;
/// Advertising Interval - long
pub const BtdrvAdType_AdvertisingIntervalLong: BtdrvAdType = 0x2F;
/// Broadcast_Name
pub const BtdrvAdType_BroadcastName: BtdrvAdType = 0x30;
/// Encrypted Advertising Data
pub const BtdrvAdType_EncryptedAdvertisingData: BtdrvAdType = 0x31;
/// Periodic Advertising Response Timing Information
pub const BtdrvAdType_PeriodicAdvertisingResponseTimingInformation: BtdrvAdType = 0x32;
/// Electronic Shelf Label
pub const BtdrvAdType_ElectronicShelfLabel: BtdrvAdType = 0x34;
/// 3D Information Data
pub const BtdrvAdType_3DInformationData: BtdrvAdType = 0x3D;
/// Manufacturer Specific Data
pub const BtdrvAdType_ManufacturerSpecificData: BtdrvAdType = 0xFF;

/// AppearanceCategoryType
pub type BtdrvAppearanceCategoryType = u32;
/// Unknown
pub const BtdrvAppearanceCategoryType_Unknown: BtdrvAppearanceCategoryType = 0x000;
/// Phone
pub const BtdrvAppearanceCategoryType_Phone: BtdrvAppearanceCategoryType = 0x001;
/// Computer
pub const BtdrvAppearanceCategoryType_Computer: BtdrvAppearanceCategoryType = 0x002;
/// Watch
pub const BtdrvAppearanceCategoryType_Watch: BtdrvAppearanceCategoryType = 0x003;
/// Clock
pub const BtdrvAppearanceCategoryType_Clock: BtdrvAppearanceCategoryType = 0x004;
/// Display
pub const BtdrvAppearanceCategoryType_Display: BtdrvAppearanceCategoryType = 0x005;
/// Remote Control
pub const BtdrvAppearanceCategoryType_RemoteControl: BtdrvAppearanceCategoryType = 0x006;
/// Eye-glasses
pub const BtdrvAppearanceCategoryType_EyeGlasses: BtdrvAppearanceCategoryType = 0x007;
/// Tag
pub const BtdrvAppearanceCategoryType_Tag: BtdrvAppearanceCategoryType = 0x008;
/// Keyring
pub const BtdrvAppearanceCategoryType_Keyring: BtdrvAppearanceCategoryType = 0x009;
/// Media Player
pub const BtdrvAppearanceCategoryType_MediaPlayer: BtdrvAppearanceCategoryType = 0x00A;
/// Barcode Scanner
pub const BtdrvAppearanceCategoryType_BarcodeScanner: BtdrvAppearanceCategoryType = 0x00B;
/// Thermometer
pub const BtdrvAppearanceCategoryType_Thermometer: BtdrvAppearanceCategoryType = 0x00C;
/// Heart Rate Sensor
pub const BtdrvAppearanceCategoryType_HeartRateSensor: BtdrvAppearanceCategoryType = 0x00D;
/// Blood Pressure
pub const BtdrvAppearanceCategoryType_BloodPressure: BtdrvAppearanceCategoryType = 0x00E;
/// Human Interface Device
pub const BtdrvAppearanceCategoryType_HumanInterfaceDevice: BtdrvAppearanceCategoryType = 0x00F;
/// Glucose Meter
pub const BtdrvAppearanceCategoryType_GlucoseMeter: BtdrvAppearanceCategoryType = 0x010;
/// Running Walking Sensor
pub const BtdrvAppearanceCategoryType_RunningWalkingSensor: BtdrvAppearanceCategoryType = 0x011;
/// Cycling
pub const BtdrvAppearanceCategoryType_Cycling: BtdrvAppearanceCategoryType = 0x012;
/// Control Device
pub const BtdrvAppearanceCategoryType_ControlDevice: BtdrvAppearanceCategoryType = 0x013;
/// Network Device
pub const BtdrvAppearanceCategoryType_NetworkDevice: BtdrvAppearanceCategoryType = 0x014;
/// Sensor
pub const BtdrvAppearanceCategoryType_Sensor: BtdrvAppearanceCategoryType = 0x015;
/// Light Fixtures
pub const BtdrvAppearanceCategoryType_LightFixtures: BtdrvAppearanceCategoryType = 0x016;
/// Fan
pub const BtdrvAppearanceCategoryType_Fan: BtdrvAppearanceCategoryType = 0x017;
/// HVAC
pub const BtdrvAppearanceCategoryType_Hvac: BtdrvAppearanceCategoryType = 0x018;
/// Air Conditioning
pub const BtdrvAppearanceCategoryType_AirConditioning: BtdrvAppearanceCategoryType = 0x019;
/// Humidifier
pub const BtdrvAppearanceCategoryType_Humidifier: BtdrvAppearanceCategoryType = 0x01A;
/// Heating
pub const BtdrvAppearanceCategoryType_Heating: BtdrvAppearanceCategoryType = 0x01B;
/// Access Control
pub const BtdrvAppearanceCategoryType_AccessControl: BtdrvAppearanceCategoryType = 0x01C;
/// Motorized Device
pub const BtdrvAppearanceCategoryType_MotorizedDevice: BtdrvAppearanceCategoryType = 0x01D;
/// Power Device
pub const BtdrvAppearanceCategoryType_PowerDevice: BtdrvAppearanceCategoryType = 0x01E;
/// Light Source
pub const BtdrvAppearanceCategoryType_LightSource: BtdrvAppearanceCategoryType = 0x01F;
/// Window Covering
pub const BtdrvAppearanceCategoryType_WindowCovering: BtdrvAppearanceCategoryType = 0x020;
/// Audio Sink
pub const BtdrvAppearanceCategoryType_AudioSink: BtdrvAppearanceCategoryType = 0x021;
/// Audio Source
pub const BtdrvAppearanceCategoryType_AudioSource: BtdrvAppearanceCategoryType = 0x022;
/// Motorized Vehicle
pub const BtdrvAppearanceCategoryType_MotorizedVehicle: BtdrvAppearanceCategoryType = 0x023;
/// Domestic Appliance
pub const BtdrvAppearanceCategoryType_DomesticAppliance: BtdrvAppearanceCategoryType = 0x024;
/// Wearable Audio Device
pub const BtdrvAppearanceCategoryType_WearableAudioDevice: BtdrvAppearanceCategoryType = 0x025;
/// Aircraft
pub const BtdrvAppearanceCategoryType_Aircraft: BtdrvAppearanceCategoryType = 0x026;
/// AV Equipment
pub const BtdrvAppearanceCategoryType_AvEquipment: BtdrvAppearanceCategoryType = 0x027;
/// Display Equipment
pub const BtdrvAppearanceCategoryType_DisplayEquipment: BtdrvAppearanceCategoryType = 0x028;
/// Hearing aid
pub const BtdrvAppearanceCategoryType_HearingAid: BtdrvAppearanceCategoryType = 0x029;
/// Gaming
pub const BtdrvAppearanceCategoryType_Gaming: BtdrvAppearanceCategoryType = 0x02A;
/// Pulse Oximeter
pub const BtdrvAppearanceCategoryType_PulseOximeter: BtdrvAppearanceCategoryType = 0x031;
/// Weight Scale
pub const BtdrvAppearanceCategoryType_WeightScale: BtdrvAppearanceCategoryType = 0x032;
/// Personal Mobility Device
pub const BtdrvAppearanceCategoryType_PersonalMobilityDevice: BtdrvAppearanceCategoryType = 0x033;
/// Continuous Glucose Monitor
pub const BtdrvAppearanceCategoryType_ContinuousGlucoseMonitor: BtdrvAppearanceCategoryType = 0x034;
/// Insulin Pump
pub const BtdrvAppearanceCategoryType_InsulinPump: BtdrvAppearanceCategoryType = 0x035;
/// Medication Delivery
pub const BtdrvAppearanceCategoryType_MedicationDelivery: BtdrvAppearanceCategoryType = 0x036;
/// Spirometer
pub const BtdrvAppearanceCategoryType_Spirometer: BtdrvAppearanceCategoryType = 0x037;
/// Outdoor Sports Activity
pub const BtdrvAppearanceCategoryType_OutdoorSportsActivity: BtdrvAppearanceCategoryType = 0x051;

/// AppearanceType
pub type BtdrvAppearanceType = u32;
/// Desktop Workstation
pub const BtdrvAppearanceType_DesktopWorkstation: BtdrvAppearanceType = 0x0081;
/// Server-class Computer
pub const BtdrvAppearanceType_ServerClassComputer: BtdrvAppearanceType = 0x0082;
/// Laptop
pub const BtdrvAppearanceType_Laptop: BtdrvAppearanceType = 0x0083;
/// Handheld PC/PDA (clamshell)
pub const BtdrvAppearanceType_HandheldPcPdaClamshell: BtdrvAppearanceType = 0x0084;
/// Palm-size PC/PDA
pub const BtdrvAppearanceType_PalmSizePcPda: BtdrvAppearanceType = 0x0085;
/// Wearable computer (watch size)
pub const BtdrvAppearanceType_WearableComputerWatchSize: BtdrvAppearanceType = 0x0086;
/// Tablet
pub const BtdrvAppearanceType_Tablet: BtdrvAppearanceType = 0x0087;
/// Docking Station
pub const BtdrvAppearanceType_DockingStation: BtdrvAppearanceType = 0x0088;
/// All in One
pub const BtdrvAppearanceType_AllInOne: BtdrvAppearanceType = 0x0089;
/// Blade Server
pub const BtdrvAppearanceType_BladeServer: BtdrvAppearanceType = 0x008A;
/// Convertible
pub const BtdrvAppearanceType_Convertible: BtdrvAppearanceType = 0x008B;
/// Detachable
pub const BtdrvAppearanceType_Detachable: BtdrvAppearanceType = 0x008C;
/// IoT Gateway
pub const BtdrvAppearanceType_IotGateway: BtdrvAppearanceType = 0x008D;
/// Mini PC
pub const BtdrvAppearanceType_MiniPc: BtdrvAppearanceType = 0x008E;
/// Stick PC
pub const BtdrvAppearanceType_StickPc: BtdrvAppearanceType = 0x008F;
/// Sports Watch
pub const BtdrvAppearanceType_SportsWatch: BtdrvAppearanceType = 0x00C1;
/// Smartwatch
pub const BtdrvAppearanceType_Smartwatch: BtdrvAppearanceType = 0x00C2;
/// Ear Thermometer
pub const BtdrvAppearanceType_EarThermometer: BtdrvAppearanceType = 0x0301;
/// Heart Rate Belt
pub const BtdrvAppearanceType_HeartRateBelt: BtdrvAppearanceType = 0x0341;
/// Arm Blood Pressure
pub const BtdrvAppearanceType_ArmBloodPressure: BtdrvAppearanceType = 0x0381;
/// Wrist Blood Pressure
pub const BtdrvAppearanceType_WristBloodPressure: BtdrvAppearanceType = 0x0382;
/// Keyboard
pub const BtdrvAppearanceType_Keyboard: BtdrvAppearanceType = 0x03C1;
/// Mouse
pub const BtdrvAppearanceType_Mouse: BtdrvAppearanceType = 0x03C2;
/// Joystick
pub const BtdrvAppearanceType_Joystick: BtdrvAppearanceType = 0x03C3;
/// Gamepad
pub const BtdrvAppearanceType_Gamepad: BtdrvAppearanceType = 0x03C4;
/// Digitizer Tablet
pub const BtdrvAppearanceType_DigitizerTablet: BtdrvAppearanceType = 0x03C5;
/// Card Reader
pub const BtdrvAppearanceType_CardReader: BtdrvAppearanceType = 0x03C6;
/// Digital Pen
pub const BtdrvAppearanceType_DigitalPen: BtdrvAppearanceType = 0x03C7;
/// Barcode Scanner
pub const BtdrvAppearanceType_BarcodeScanner: BtdrvAppearanceType = 0x03C8;
/// Touchpad
pub const BtdrvAppearanceType_Touchpad: BtdrvAppearanceType = 0x03C9;
/// Presentation Remote
pub const BtdrvAppearanceType_PresentationRemote: BtdrvAppearanceType = 0x03CA;
/// In-Shoe Running Walking Sensor
pub const BtdrvAppearanceType_InShoeRunningWalkingSensor: BtdrvAppearanceType = 0x0441;
/// On-Shoe Running Walking Sensor
pub const BtdrvAppearanceType_OnShoeRunningWalkingSensor: BtdrvAppearanceType = 0x0442;
/// On-Hip Running Walking Sensor
pub const BtdrvAppearanceType_OnHipRunningWalkingSensor: BtdrvAppearanceType = 0x0443;
/// Cycling Computer
pub const BtdrvAppearanceType_CyclingComputer: BtdrvAppearanceType = 0x0481;
/// Speed Sensor
pub const BtdrvAppearanceType_SpeedSensor: BtdrvAppearanceType = 0x0482;
/// Cadence Sensor
pub const BtdrvAppearanceType_CadenceSensor: BtdrvAppearanceType = 0x0483;
/// Power Sensor
pub const BtdrvAppearanceType_PowerSensor: BtdrvAppearanceType = 0x0484;
/// Speed and Cadence Sensor
pub const BtdrvAppearanceType_SpeedAndCadenceSensor: BtdrvAppearanceType = 0x0485;
/// Switch
pub const BtdrvAppearanceType_Switch: BtdrvAppearanceType = 0x04C1;
/// Multi-switch
pub const BtdrvAppearanceType_MultiSwitch: BtdrvAppearanceType = 0x04C2;
/// Button
pub const BtdrvAppearanceType_Button: BtdrvAppearanceType = 0x04C3;
/// Slider
pub const BtdrvAppearanceType_Slider: BtdrvAppearanceType = 0x04C4;
/// Rotary Switch
pub const BtdrvAppearanceType_RotarySwitch: BtdrvAppearanceType = 0x04C5;
/// Touch Panel
pub const BtdrvAppearanceType_TouchPanel: BtdrvAppearanceType = 0x04C6;
/// Single Switch
pub const BtdrvAppearanceType_SingleSwitch: BtdrvAppearanceType = 0x04C7;
/// Double Switch
pub const BtdrvAppearanceType_DoubleSwitch: BtdrvAppearanceType = 0x04C8;
/// Triple Switch
pub const BtdrvAppearanceType_TripleSwitch: BtdrvAppearanceType = 0x04C9;
/// Battery Switch
pub const BtdrvAppearanceType_BatterySwitch: BtdrvAppearanceType = 0x04CA;
/// Energy Harvesting Switch
pub const BtdrvAppearanceType_EnergyHarvestingSwitch: BtdrvAppearanceType = 0x04CB;
/// Push Button
pub const BtdrvAppearanceType_PushButton: BtdrvAppearanceType = 0x04CC;
/// Access Point
pub const BtdrvAppearanceType_AccessPoint: BtdrvAppearanceType = 0x0501;
/// Mesh Device
pub const BtdrvAppearanceType_MeshDevice: BtdrvAppearanceType = 0x0502;
/// Mesh Network Proxy
pub const BtdrvAppearanceType_MeshNetworkProxy: BtdrvAppearanceType = 0x0503;
/// Motion Sensor
pub const BtdrvAppearanceType_MotionSensor: BtdrvAppearanceType = 0x0541;
/// Air quality Sensor
pub const BtdrvAppearanceType_AirQualitySensor: BtdrvAppearanceType = 0x0542;
/// Temperature Sensor
pub const BtdrvAppearanceType_TemperatureSensor: BtdrvAppearanceType = 0x0543;
/// Humidity Sensor
pub const BtdrvAppearanceType_HumiditySensor: BtdrvAppearanceType = 0x0544;
/// Leak Sensor
pub const BtdrvAppearanceType_LeakSensor: BtdrvAppearanceType = 0x0545;
/// Smoke Sensor
pub const BtdrvAppearanceType_SmokeSensor: BtdrvAppearanceType = 0x0546;
/// Occupancy Sensor
pub const BtdrvAppearanceType_OccupancySensor: BtdrvAppearanceType = 0x0547;
/// Contact Sensor
pub const BtdrvAppearanceType_ContactSensor: BtdrvAppearanceType = 0x0548;
/// Carbon Monoxide Sensor
pub const BtdrvAppearanceType_CarbonMonoxideSensor: BtdrvAppearanceType = 0x0549;
/// Carbon Dioxide Sensor
pub const BtdrvAppearanceType_CarbonDioxideSensor: BtdrvAppearanceType = 0x054A;
/// Ambient Light Sensor
pub const BtdrvAppearanceType_AmbientLightSensor: BtdrvAppearanceType = 0x054B;
/// Energy Sensor
pub const BtdrvAppearanceType_EnergySensor: BtdrvAppearanceType = 0x054C;
/// Color Light Sensor
pub const BtdrvAppearanceType_ColorLightSensor: BtdrvAppearanceType = 0x054D;
/// Rain Sensor
pub const BtdrvAppearanceType_RainSensor: BtdrvAppearanceType = 0x054E;
/// Fire Sensor
pub const BtdrvAppearanceType_FireSensor: BtdrvAppearanceType = 0x054F;
/// Wind Sensor
pub const BtdrvAppearanceType_WindSensor: BtdrvAppearanceType = 0x0550;
/// Proximity Sensor
pub const BtdrvAppearanceType_ProximitySensor: BtdrvAppearanceType = 0x0551;
/// Multi-Sensor
pub const BtdrvAppearanceType_MultiSensor: BtdrvAppearanceType = 0x0552;
/// Flush Mounted Sensor
pub const BtdrvAppearanceType_FlushMountedSensor: BtdrvAppearanceType = 0x0553;
/// Ceiling Mounted Sensor
pub const BtdrvAppearanceType_CeilingMountedSensor: BtdrvAppearanceType = 0x0554;
/// Wall Mounted Sensor
pub const BtdrvAppearanceType_WallMountedSensor: BtdrvAppearanceType = 0x0555;
/// Multisensor
pub const BtdrvAppearanceType_Multisensor: BtdrvAppearanceType = 0x0556;
/// Energy Meter
pub const BtdrvAppearanceType_EnergyMeter: BtdrvAppearanceType = 0x0557;
/// Flame Detector
pub const BtdrvAppearanceType_FlameDetector: BtdrvAppearanceType = 0x0558;
/// Vehicle Tire Pressure Sensor
pub const BtdrvAppearanceType_VehicleTirePressureSensor: BtdrvAppearanceType = 0x0559;
/// Wall Light
pub const BtdrvAppearanceType_WallLight: BtdrvAppearanceType = 0x0581;
/// Ceiling Light
pub const BtdrvAppearanceType_CeilingLight: BtdrvAppearanceType = 0x0582;
/// Floor Light
pub const BtdrvAppearanceType_FloorLight: BtdrvAppearanceType = 0x0583;
/// Cabinet Light
pub const BtdrvAppearanceType_CabinetLight: BtdrvAppearanceType = 0x0584;
/// Desk Light
pub const BtdrvAppearanceType_DeskLight: BtdrvAppearanceType = 0x0585;
/// Troffer Light
pub const BtdrvAppearanceType_TrofferLight: BtdrvAppearanceType = 0x0586;
/// Pendant Light
pub const BtdrvAppearanceType_PendantLight: BtdrvAppearanceType = 0x0587;
/// In-ground Light
pub const BtdrvAppearanceType_InGroundLight: BtdrvAppearanceType = 0x0588;
/// Flood Light
pub const BtdrvAppearanceType_FloodLight: BtdrvAppearanceType = 0x0589;
/// Underwater Light
pub const BtdrvAppearanceType_UnderwaterLight: BtdrvAppearanceType = 0x058A;
/// Bollard with Light
pub const BtdrvAppearanceType_BollardWithLight: BtdrvAppearanceType = 0x058B;
/// Pathway Light
pub const BtdrvAppearanceType_PathwayLight: BtdrvAppearanceType = 0x058C;
/// Garden Light
pub const BtdrvAppearanceType_GardenLight: BtdrvAppearanceType = 0x058D;
/// Pole-top Light
pub const BtdrvAppearanceType_PoleTopLight: BtdrvAppearanceType = 0x058E;
/// Spotlight
pub const BtdrvAppearanceType_Spotlight: BtdrvAppearanceType = 0x058F;
/// Linear Light
pub const BtdrvAppearanceType_LinearLight: BtdrvAppearanceType = 0x0590;
/// Street Light
pub const BtdrvAppearanceType_StreetLight: BtdrvAppearanceType = 0x0591;
/// Shelves Light
pub const BtdrvAppearanceType_ShelvesLight: BtdrvAppearanceType = 0x0592;
/// Bay Light
pub const BtdrvAppearanceType_BayLight: BtdrvAppearanceType = 0x0593;
/// Emergency Exit Light
pub const BtdrvAppearanceType_EmergencyExitLight: BtdrvAppearanceType = 0x0594;
/// Light Controller
pub const BtdrvAppearanceType_LightController: BtdrvAppearanceType = 0x0595;
/// Light Driver
pub const BtdrvAppearanceType_LightDriver: BtdrvAppearanceType = 0x0596;
/// Bulb
pub const BtdrvAppearanceType_Bulb: BtdrvAppearanceType = 0x0597;
/// Low-bay Light
pub const BtdrvAppearanceType_LowBayLight: BtdrvAppearanceType = 0x0598;
/// High-bay Light
pub const BtdrvAppearanceType_HighBayLight: BtdrvAppearanceType = 0x0599;
/// Ceiling Fan
pub const BtdrvAppearanceType_CeilingFan: BtdrvAppearanceType = 0x05C1;
/// Axial Fan
pub const BtdrvAppearanceType_AxialFan: BtdrvAppearanceType = 0x05C2;
/// Exhaust Fan
pub const BtdrvAppearanceType_ExhaustFan: BtdrvAppearanceType = 0x05C3;
/// Pedestal Fan
pub const BtdrvAppearanceType_PedestalFan: BtdrvAppearanceType = 0x05C4;
/// Desk Fan
pub const BtdrvAppearanceType_DeskFan: BtdrvAppearanceType = 0x05C5;
/// Wall Fan
pub const BtdrvAppearanceType_WallFan: BtdrvAppearanceType = 0x05C6;
/// Thermostat
pub const BtdrvAppearanceType_Thermostat: BtdrvAppearanceType = 0x0601;
/// Humidifier
pub const BtdrvAppearanceType_Humidifier: BtdrvAppearanceType = 0x0602;
/// De-humidifier
pub const BtdrvAppearanceType_DeHumidifier: BtdrvAppearanceType = 0x0603;
/// Heater
pub const BtdrvAppearanceType_Heater: BtdrvAppearanceType = 0x0604;
/// HVAC Radiator
pub const BtdrvAppearanceType_HvacRadiator: BtdrvAppearanceType = 0x0605;
/// HVAC Boiler
pub const BtdrvAppearanceType_HvacBoiler: BtdrvAppearanceType = 0x0606;
/// HVAC Heat Pump
pub const BtdrvAppearanceType_HvacHeatPump: BtdrvAppearanceType = 0x0607;
/// HVAC Infrared Heater
pub const BtdrvAppearanceType_HvacInfraredHeater: BtdrvAppearanceType = 0x0608;
/// HVAC Radiant Panel Heater
pub const BtdrvAppearanceType_HvacRadiantPanelHeater: BtdrvAppearanceType = 0x0609;
/// HVAC Fan Heater
pub const BtdrvAppearanceType_HvacFanHeater: BtdrvAppearanceType = 0x060A;
/// HVAC Air Curtain
pub const BtdrvAppearanceType_HvacAirCurtain: BtdrvAppearanceType = 0x060B;
/// Heating Radiator
pub const BtdrvAppearanceType_HeatingRadiator: BtdrvAppearanceType = 0x06C1;
/// Heating Boiler
pub const BtdrvAppearanceType_HeatingBoiler: BtdrvAppearanceType = 0x06C2;
/// Heating Heat Pump
pub const BtdrvAppearanceType_HeatingHeatPump: BtdrvAppearanceType = 0x06C3;
/// Heating Infrared Heater
pub const BtdrvAppearanceType_HeatingInfraredHeater: BtdrvAppearanceType = 0x06C4;
/// Heating Radiant Panel Heater
pub const BtdrvAppearanceType_HeatingRadiantPanelHeater: BtdrvAppearanceType = 0x06C5;
/// Heating Fan Heater
pub const BtdrvAppearanceType_HeatingFanHeater: BtdrvAppearanceType = 0x06C6;
/// Heating Air Curtain
pub const BtdrvAppearanceType_HeatingAirCurtain: BtdrvAppearanceType = 0x06C7;
/// Access Door
pub const BtdrvAppearanceType_AccessDoor: BtdrvAppearanceType = 0x0701;
/// Garage Door
pub const BtdrvAppearanceType_GarageDoor: BtdrvAppearanceType = 0x0702;
/// Emergency Exit Door
pub const BtdrvAppearanceType_EmergencyExitDoor: BtdrvAppearanceType = 0x0703;
/// Access Lock
pub const BtdrvAppearanceType_AccessLock: BtdrvAppearanceType = 0x0704;
/// Elevator
pub const BtdrvAppearanceType_Elevator: BtdrvAppearanceType = 0x0705;
/// Window
pub const BtdrvAppearanceType_Window: BtdrvAppearanceType = 0x0706;
/// Entrance Gate
pub const BtdrvAppearanceType_EntranceGate: BtdrvAppearanceType = 0x0707;
/// Door Lock
pub const BtdrvAppearanceType_DoorLock: BtdrvAppearanceType = 0x0708;
/// Locker
pub const BtdrvAppearanceType_Locker: BtdrvAppearanceType = 0x0709;
/// Motorized Gate
pub const BtdrvAppearanceType_MotorizedGate: BtdrvAppearanceType = 0x0741;
/// Awning
pub const BtdrvAppearanceType_Awning: BtdrvAppearanceType = 0x0742;
/// Blinds or Shades
pub const BtdrvAppearanceType_BlindsOrShades: BtdrvAppearanceType = 0x0743;
/// Curtains
pub const BtdrvAppearanceType_Curtains: BtdrvAppearanceType = 0x0744;
/// Screen
pub const BtdrvAppearanceType_Screen: BtdrvAppearanceType = 0x0745;
/// Power Outlet
pub const BtdrvAppearanceType_PowerOutlet: BtdrvAppearanceType = 0x0781;
/// Power Strip
pub const BtdrvAppearanceType_PowerStrip: BtdrvAppearanceType = 0x0782;
/// Plug
pub const BtdrvAppearanceType_Plug: BtdrvAppearanceType = 0x0783;
/// Power Supply
pub const BtdrvAppearanceType_PowerSupply: BtdrvAppearanceType = 0x0784;
/// LED Driver
pub const BtdrvAppearanceType_LedDriver: BtdrvAppearanceType = 0x0785;
/// Fluorescent Lamp Gear
pub const BtdrvAppearanceType_FluorescentLampGear: BtdrvAppearanceType = 0x0786;
/// HID Lamp Gear
pub const BtdrvAppearanceType_HidLampGear: BtdrvAppearanceType = 0x0787;
/// Charge Case
pub const BtdrvAppearanceType_ChargeCase: BtdrvAppearanceType = 0x0788;
/// Power Bank
pub const BtdrvAppearanceType_PowerBank: BtdrvAppearanceType = 0x0789;
/// Incandescent Light Bulb
pub const BtdrvAppearanceType_IncandescentLightBulb: BtdrvAppearanceType = 0x07C1;
/// LED Lamp
pub const BtdrvAppearanceType_LedLamp: BtdrvAppearanceType = 0x07C2;
/// HID Lamp
pub const BtdrvAppearanceType_HidLamp: BtdrvAppearanceType = 0x07C3;
/// Fluorescent Lamp
pub const BtdrvAppearanceType_FluorescentLamp: BtdrvAppearanceType = 0x07C4;
/// LED Array
pub const BtdrvAppearanceType_LedArray: BtdrvAppearanceType = 0x07C5;
/// Multi-Color LED Array
pub const BtdrvAppearanceType_MultiColorLedArray: BtdrvAppearanceType = 0x07C6;
/// Low voltage halogen
pub const BtdrvAppearanceType_LowVoltageHalogen: BtdrvAppearanceType = 0x07C7;
/// Organic light emitting diode (OLED)
pub const BtdrvAppearanceType_OrganicLightEmittingDiodeOled: BtdrvAppearanceType = 0x07C8;
/// Window Shades
pub const BtdrvAppearanceType_WindowShades: BtdrvAppearanceType = 0x0801;
/// Window Blinds
pub const BtdrvAppearanceType_WindowBlinds: BtdrvAppearanceType = 0x0802;
/// Window Awning
pub const BtdrvAppearanceType_WindowAwning: BtdrvAppearanceType = 0x0803;
/// Window Curtain
pub const BtdrvAppearanceType_WindowCurtain: BtdrvAppearanceType = 0x0804;
/// Exterior Shutter
pub const BtdrvAppearanceType_ExteriorShutter: BtdrvAppearanceType = 0x0805;
/// Exterior Screen
pub const BtdrvAppearanceType_ExteriorScreen: BtdrvAppearanceType = 0x0806;
/// Standalone Speaker
pub const BtdrvAppearanceType_StandaloneSpeaker: BtdrvAppearanceType = 0x0841;
/// Soundbar
pub const BtdrvAppearanceType_Soundbar: BtdrvAppearanceType = 0x0842;
/// Bookshelf Speaker
pub const BtdrvAppearanceType_BookshelfSpeaker: BtdrvAppearanceType = 0x0843;
/// Standmounted Speaker
pub const BtdrvAppearanceType_StandmountedSpeaker: BtdrvAppearanceType = 0x0844;
/// Speakerphone
pub const BtdrvAppearanceType_Speakerphone: BtdrvAppearanceType = 0x0845;
/// Microphone
pub const BtdrvAppearanceType_Microphone: BtdrvAppearanceType = 0x0881;
/// Alarm
pub const BtdrvAppearanceType_Alarm: BtdrvAppearanceType = 0x0882;
/// Bell
pub const BtdrvAppearanceType_Bell: BtdrvAppearanceType = 0x0883;
/// Horn
pub const BtdrvAppearanceType_Horn: BtdrvAppearanceType = 0x0884;
/// Broadcasting Device
pub const BtdrvAppearanceType_BroadcastingDevice: BtdrvAppearanceType = 0x0885;
/// Service Desk
pub const BtdrvAppearanceType_ServiceDesk: BtdrvAppearanceType = 0x0886;
/// Kiosk
pub const BtdrvAppearanceType_Kiosk: BtdrvAppearanceType = 0x0887;
/// Broadcasting Room
pub const BtdrvAppearanceType_BroadcastingRoom: BtdrvAppearanceType = 0x0888;
/// Auditorium
pub const BtdrvAppearanceType_Auditorium: BtdrvAppearanceType = 0x0889;
/// Car
pub const BtdrvAppearanceType_Car: BtdrvAppearanceType = 0x08C1;
/// Large Goods Vehicle
pub const BtdrvAppearanceType_LargeGoodsVehicle: BtdrvAppearanceType = 0x08C2;
/// 2-Wheeled Vehicle
pub const BtdrvAppearanceType_2WheeledVehicle: BtdrvAppearanceType = 0x08C3;
/// Motorbike
pub const BtdrvAppearanceType_Motorbike: BtdrvAppearanceType = 0x08C4;
/// Scooter
pub const BtdrvAppearanceType_Scooter: BtdrvAppearanceType = 0x08C5;
/// Moped
pub const BtdrvAppearanceType_Moped: BtdrvAppearanceType = 0x08C6;
/// 3-Wheeled Vehicle
pub const BtdrvAppearanceType_3WheeledVehicle: BtdrvAppearanceType = 0x08C7;
/// Light Vehicle
pub const BtdrvAppearanceType_LightVehicle: BtdrvAppearanceType = 0x08C8;
/// Quad Bike
pub const BtdrvAppearanceType_QuadBike: BtdrvAppearanceType = 0x08C9;
/// Minibus
pub const BtdrvAppearanceType_Minibus: BtdrvAppearanceType = 0x08CA;
/// Bus
pub const BtdrvAppearanceType_Bus: BtdrvAppearanceType = 0x08CB;
/// Trolley
pub const BtdrvAppearanceType_Trolley: BtdrvAppearanceType = 0x08CC;
/// Agricultural Vehicle
pub const BtdrvAppearanceType_AgriculturalVehicle: BtdrvAppearanceType = 0x08CD;
/// Camper / Caravan
pub const BtdrvAppearanceType_CamperCaravan: BtdrvAppearanceType = 0x08CE;
/// Recreational Vehicle / Motor Home
pub const BtdrvAppearanceType_RecreationalVehicleMotorHome: BtdrvAppearanceType = 0x08CF;
/// Refrigerator
pub const BtdrvAppearanceType_Refrigerator: BtdrvAppearanceType = 0x0901;
/// Freezer
pub const BtdrvAppearanceType_Freezer: BtdrvAppearanceType = 0x0902;
/// Oven
pub const BtdrvAppearanceType_Oven: BtdrvAppearanceType = 0x0903;
/// Microwave
pub const BtdrvAppearanceType_Microwave: BtdrvAppearanceType = 0x0904;
/// Toaster
pub const BtdrvAppearanceType_Toaster: BtdrvAppearanceType = 0x0905;
/// Washing Machine
pub const BtdrvAppearanceType_WashingMachine: BtdrvAppearanceType = 0x0906;
/// Dryer
pub const BtdrvAppearanceType_Dryer: BtdrvAppearanceType = 0x0907;
/// Coffee maker
pub const BtdrvAppearanceType_CoffeeMaker: BtdrvAppearanceType = 0x0908;
/// Clothes iron
pub const BtdrvAppearanceType_ClothesIron: BtdrvAppearanceType = 0x0909;
/// Curling iron
pub const BtdrvAppearanceType_CurlingIron: BtdrvAppearanceType = 0x090A;
/// Hair dryer
pub const BtdrvAppearanceType_HairDryer: BtdrvAppearanceType = 0x090B;
/// Vacuum cleaner
pub const BtdrvAppearanceType_VacuumCleaner: BtdrvAppearanceType = 0x090C;
/// Robotic vacuum cleaner
pub const BtdrvAppearanceType_RoboticVacuumCleaner: BtdrvAppearanceType = 0x090D;
/// Rice cooker
pub const BtdrvAppearanceType_RiceCooker: BtdrvAppearanceType = 0x090E;
/// Clothes steamer
pub const BtdrvAppearanceType_ClothesSteamer: BtdrvAppearanceType = 0x090F;
/// Earbud
pub const BtdrvAppearanceType_Earbud: BtdrvAppearanceType = 0x0941;
/// Headset
pub const BtdrvAppearanceType_Headset: BtdrvAppearanceType = 0x0942;
/// Headphones
pub const BtdrvAppearanceType_Headphones: BtdrvAppearanceType = 0x0943;
/// Neck Band
pub const BtdrvAppearanceType_NeckBand: BtdrvAppearanceType = 0x0944;
/// Light Aircraft
pub const BtdrvAppearanceType_LightAircraft: BtdrvAppearanceType = 0x0981;
/// Microlight
pub const BtdrvAppearanceType_Microlight: BtdrvAppearanceType = 0x0982;
/// Paraglider
pub const BtdrvAppearanceType_Paraglider: BtdrvAppearanceType = 0x0983;
/// Large Passenger Aircraft
pub const BtdrvAppearanceType_LargePassengerAircraft: BtdrvAppearanceType = 0x0984;
/// Amplifier
pub const BtdrvAppearanceType_Amplifier: BtdrvAppearanceType = 0x09C1;
/// Receiver
pub const BtdrvAppearanceType_Receiver: BtdrvAppearanceType = 0x09C2;
/// Radio
pub const BtdrvAppearanceType_Radio: BtdrvAppearanceType = 0x09C3;
/// Tuner
pub const BtdrvAppearanceType_Tuner: BtdrvAppearanceType = 0x09C4;
/// Turntable
pub const BtdrvAppearanceType_Turntable: BtdrvAppearanceType = 0x09C5;
/// CD Player
pub const BtdrvAppearanceType_CdPlayer: BtdrvAppearanceType = 0x09C6;
/// DVD Player
pub const BtdrvAppearanceType_DvdPlayer: BtdrvAppearanceType = 0x09C7;
/// Bluray Player
pub const BtdrvAppearanceType_BlurayPlayer: BtdrvAppearanceType = 0x09C8;
/// Optical Disc Player
pub const BtdrvAppearanceType_OpticalDiscPlayer: BtdrvAppearanceType = 0x09C9;
/// Set-Top Box
pub const BtdrvAppearanceType_SetTopBox: BtdrvAppearanceType = 0x09CA;
/// Television
pub const BtdrvAppearanceType_Television: BtdrvAppearanceType = 0x0A01;
/// Monitor
pub const BtdrvAppearanceType_Monitor: BtdrvAppearanceType = 0x0A02;
/// Projector
pub const BtdrvAppearanceType_Projector: BtdrvAppearanceType = 0x0A03;
/// In-ear hearing aid
pub const BtdrvAppearanceType_InEarHearingAid: BtdrvAppearanceType = 0x0A41;
/// Behind-ear hearing aid
pub const BtdrvAppearanceType_BehindEarHearingAid: BtdrvAppearanceType = 0x0A42;
/// Cochlear Implant
pub const BtdrvAppearanceType_CochlearImplant: BtdrvAppearanceType = 0x0A43;
/// Home Video Game Console
pub const BtdrvAppearanceType_HomeVideoGameConsole: BtdrvAppearanceType = 0x0A81;
/// Portable handheld console
pub const BtdrvAppearanceType_PortableHandheldConsole: BtdrvAppearanceType = 0x0A82;
/// Fingertip Pulse Oximeter
pub const BtdrvAppearanceType_FingertipPulseOximeter: BtdrvAppearanceType = 0x0C41;
/// Wrist Worn Pulse Oximeter
pub const BtdrvAppearanceType_WristWornPulseOximeter: BtdrvAppearanceType = 0x0C42;
/// Powered Wheelchair
pub const BtdrvAppearanceType_PoweredWheelchair: BtdrvAppearanceType = 0x0CC1;
/// Mobility Scooter
pub const BtdrvAppearanceType_MobilityScooter: BtdrvAppearanceType = 0x0CC2;
/// Insulin Pump, durable pump
pub const BtdrvAppearanceType_InsulinPumpDurablePump: BtdrvAppearanceType = 0x0D41;
/// Insulin Pump, patch pump
pub const BtdrvAppearanceType_InsulinPumpPatchPump: BtdrvAppearanceType = 0x0D44;
/// Insulin Pen
pub const BtdrvAppearanceType_InsulinPen: BtdrvAppearanceType = 0x0D48;
/// Handheld Spirometer
pub const BtdrvAppearanceType_HandheldSpirometer: BtdrvAppearanceType = 0x0DC1;
/// Location Display
pub const BtdrvAppearanceType_LocationDisplay: BtdrvAppearanceType = 0x1441;
/// Location and Navigation Display
pub const BtdrvAppearanceType_LocationAndNavigationDisplay: BtdrvAppearanceType = 0x1442;
/// Location Pod
pub const BtdrvAppearanceType_LocationPod: BtdrvAppearanceType = 0x1443;
/// Location and Navigation Pod
pub const BtdrvAppearanceType_LocationAndNavigationPod: BtdrvAppearanceType = 0x1444;

/// GattServiceUuidType
pub type BtdrvGattServiceUuidType = u32;
/// Generic Access
pub const BtdrvGattServiceUuidType_GenericAccess: BtdrvGattServiceUuidType = 0x1800;
/// Generic Attribute
pub const BtdrvGattServiceUuidType_GenericAttribute: BtdrvGattServiceUuidType = 0x1801;
/// Immediate Alert
pub const BtdrvGattServiceUuidType_ImmediateAlert: BtdrvGattServiceUuidType = 0x1802;
/// Link Loss
pub const BtdrvGattServiceUuidType_LinkLoss: BtdrvGattServiceUuidType = 0x1803;
/// Tx Power
pub const BtdrvGattServiceUuidType_TxPower: BtdrvGattServiceUuidType = 0x1804;
/// Current Time
pub const BtdrvGattServiceUuidType_CurrentTime: BtdrvGattServiceUuidType = 0x1805;
/// Reference Time Update
pub const BtdrvGattServiceUuidType_ReferenceTimeUpdate: BtdrvGattServiceUuidType = 0x1806;
/// Next DST Change
pub const BtdrvGattServiceUuidType_NextDstChange: BtdrvGattServiceUuidType = 0x1807;
/// Glucose
pub const BtdrvGattServiceUuidType_Glucose: BtdrvGattServiceUuidType = 0x1808;
/// Health Thermometer
pub const BtdrvGattServiceUuidType_HealthThermometer: BtdrvGattServiceUuidType = 0x1809;
/// Device Information
pub const BtdrvGattServiceUuidType_DeviceInformation: BtdrvGattServiceUuidType = 0x180A;
/// Heart Rate
pub const BtdrvGattServiceUuidType_HeartRate: BtdrvGattServiceUuidType = 0x180D;
/// Phone Alert Status
pub const BtdrvGattServiceUuidType_PhoneAlertStatus: BtdrvGattServiceUuidType = 0x180E;
/// Battery
pub const BtdrvGattServiceUuidType_Battery: BtdrvGattServiceUuidType = 0x180F;
/// Blood Pressure
pub const BtdrvGattServiceUuidType_BloodPressure: BtdrvGattServiceUuidType = 0x1810;
/// Alert Notification
pub const BtdrvGattServiceUuidType_AlertNotification: BtdrvGattServiceUuidType = 0x1811;
/// Human Interface Device
pub const BtdrvGattServiceUuidType_HumanInterfaceDevice: BtdrvGattServiceUuidType = 0x1812;
/// Scan Parameters
pub const BtdrvGattServiceUuidType_ScanParameters: BtdrvGattServiceUuidType = 0x1813;
/// Running Speed and Cadence
pub const BtdrvGattServiceUuidType_RunningSpeedAndCadence: BtdrvGattServiceUuidType = 0x1814;
/// Automation IO
pub const BtdrvGattServiceUuidType_AutomationIo: BtdrvGattServiceUuidType = 0x1815;
/// Cycling Speed and Cadence
pub const BtdrvGattServiceUuidType_CyclingSpeedAndCadence: BtdrvGattServiceUuidType = 0x1816;
/// Cycling Power
pub const BtdrvGattServiceUuidType_CyclingPower: BtdrvGattServiceUuidType = 0x1818;
/// Location and Navigation
pub const BtdrvGattServiceUuidType_LocationAndNavigation: BtdrvGattServiceUuidType = 0x1819;
/// Environmental Sensing
pub const BtdrvGattServiceUuidType_EnvironmentalSensing: BtdrvGattServiceUuidType = 0x181A;
/// Body Composition
pub const BtdrvGattServiceUuidType_BodyComposition: BtdrvGattServiceUuidType = 0x181B;
/// User Data
pub const BtdrvGattServiceUuidType_UserData: BtdrvGattServiceUuidType = 0x181C;
/// Weight Scale
pub const BtdrvGattServiceUuidType_WeightScale: BtdrvGattServiceUuidType = 0x181D;
/// Bond Management
pub const BtdrvGattServiceUuidType_BondManagement: BtdrvGattServiceUuidType = 0x181E;
/// Continuous Glucose Monitoring
pub const BtdrvGattServiceUuidType_ContinuousGlucoseMonitoring: BtdrvGattServiceUuidType = 0x181F;
/// Internet Protocol Support
pub const BtdrvGattServiceUuidType_InternetProtocolSupport: BtdrvGattServiceUuidType = 0x1820;
/// Indoor Positioning
pub const BtdrvGattServiceUuidType_IndoorPositioning: BtdrvGattServiceUuidType = 0x1821;
/// Pulse Oximeter
pub const BtdrvGattServiceUuidType_PulseOximeter: BtdrvGattServiceUuidType = 0x1822;
/// HTTP Proxy
pub const BtdrvGattServiceUuidType_HttpProxy: BtdrvGattServiceUuidType = 0x1823;
/// Transport Discovery
pub const BtdrvGattServiceUuidType_TransportDiscovery: BtdrvGattServiceUuidType = 0x1824;
/// Object Transfer
pub const BtdrvGattServiceUuidType_ObjectTransfer: BtdrvGattServiceUuidType = 0x1825;
/// Fitness Machine
pub const BtdrvGattServiceUuidType_FitnessMachine: BtdrvGattServiceUuidType = 0x1826;
/// Mesh Provisioning
pub const BtdrvGattServiceUuidType_MeshProvisioning: BtdrvGattServiceUuidType = 0x1827;
/// Mesh Proxy
pub const BtdrvGattServiceUuidType_MeshProxy: BtdrvGattServiceUuidType = 0x1828;
/// Reconnection Configuration
pub const BtdrvGattServiceUuidType_ReconnectionConfiguration: BtdrvGattServiceUuidType = 0x1829;
/// Insulin Delivery
pub const BtdrvGattServiceUuidType_InsulinDelivery: BtdrvGattServiceUuidType = 0x183A;
/// Binary Sensor
pub const BtdrvGattServiceUuidType_BinarySensor: BtdrvGattServiceUuidType = 0x183B;
/// Emergency Configuration
pub const BtdrvGattServiceUuidType_EmergencyConfiguration: BtdrvGattServiceUuidType = 0x183C;
/// Authorization Control
pub const BtdrvGattServiceUuidType_AuthorizationControl: BtdrvGattServiceUuidType = 0x183D;
/// Physical Activity Monitor
pub const BtdrvGattServiceUuidType_PhysicalActivityMonitor: BtdrvGattServiceUuidType = 0x183E;
/// Elapsed Time
pub const BtdrvGattServiceUuidType_ElapsedTime: BtdrvGattServiceUuidType = 0x183F;
/// Generic Health Sensor
pub const BtdrvGattServiceUuidType_GenericHealthSensor: BtdrvGattServiceUuidType = 0x1840;
/// Audio Input Control
pub const BtdrvGattServiceUuidType_AudioInputControl: BtdrvGattServiceUuidType = 0x1843;
/// Volume Control
pub const BtdrvGattServiceUuidType_VolumeControl: BtdrvGattServiceUuidType = 0x1844;
/// Volume Offset Control
pub const BtdrvGattServiceUuidType_VolumeOffsetControl: BtdrvGattServiceUuidType = 0x1845;
/// Coordinated Set Identification
pub const BtdrvGattServiceUuidType_CoordinatedSetIdentification: BtdrvGattServiceUuidType = 0x1846;
/// Device Time
pub const BtdrvGattServiceUuidType_DeviceTime: BtdrvGattServiceUuidType = 0x1847;
/// Media Control
pub const BtdrvGattServiceUuidType_MediaControl: BtdrvGattServiceUuidType = 0x1848;
/// Generic Media Control
pub const BtdrvGattServiceUuidType_GenericMediaControl: BtdrvGattServiceUuidType = 0x1849;
/// Constant Tone Extension
pub const BtdrvGattServiceUuidType_ConstantToneExtension: BtdrvGattServiceUuidType = 0x184A;
/// Telephone Bearer
pub const BtdrvGattServiceUuidType_TelephoneBearer: BtdrvGattServiceUuidType = 0x184B;
/// Generic Telephone Bearer
pub const BtdrvGattServiceUuidType_GenericTelephoneBearer: BtdrvGattServiceUuidType = 0x184C;
/// Microphone Control
pub const BtdrvGattServiceUuidType_MicrophoneControl: BtdrvGattServiceUuidType = 0x184D;
/// Audio Stream Control
pub const BtdrvGattServiceUuidType_AudioStreamControl: BtdrvGattServiceUuidType = 0x184E;
/// Broadcast Audio Scan
pub const BtdrvGattServiceUuidType_BroadcastAudioScan: BtdrvGattServiceUuidType = 0x184F;
/// Published Audio Capabilities
pub const BtdrvGattServiceUuidType_PublishedAudioCapabilities: BtdrvGattServiceUuidType = 0x1850;
/// Basic Audio Announcement
pub const BtdrvGattServiceUuidType_BasicAudioAnnouncement: BtdrvGattServiceUuidType = 0x1851;
/// Broadcast Audio Announcement
pub const BtdrvGattServiceUuidType_BroadcastAudioAnnouncement: BtdrvGattServiceUuidType = 0x1852;
/// Common Audio
pub const BtdrvGattServiceUuidType_CommonAudio: BtdrvGattServiceUuidType = 0x1853;
/// Hearing Access
pub const BtdrvGattServiceUuidType_HearingAccess: BtdrvGattServiceUuidType = 0x1854;
/// Telephony and Media Audio
pub const BtdrvGattServiceUuidType_TelephonyAndMediaAudio: BtdrvGattServiceUuidType = 0x1855;
/// Public Broadcast Announcement
pub const BtdrvGattServiceUuidType_PublicBroadcastAnnouncement: BtdrvGattServiceUuidType = 0x1856;
/// Electronic Shelf Label
pub const BtdrvGattServiceUuidType_ElectronicShelfLabel: BtdrvGattServiceUuidType = 0x1857;

/// GattCharacteristicUuidType
pub type BtdrvGattCharacteristicUuidType = u32;
/// Device Name
pub const BtdrvGattCharacteristicUuidType_DeviceName: BtdrvGattCharacteristicUuidType = 0x2A00;
/// Appearance
pub const BtdrvGattCharacteristicUuidType_Appearance: BtdrvGattCharacteristicUuidType = 0x2A01;
/// Peripheral Privacy Flag
pub const BtdrvGattCharacteristicUuidType_PeripheralPrivacyFlag: BtdrvGattCharacteristicUuidType = 0x2A02;
/// Reconnection Address
pub const BtdrvGattCharacteristicUuidType_ReconnectionAddress: BtdrvGattCharacteristicUuidType = 0x2A03;
/// Peripheral Preferred Connection Parameters
pub const BtdrvGattCharacteristicUuidType_PeripheralPreferredConnectionParameters: BtdrvGattCharacteristicUuidType = 0x2A04;
/// Service Changed
pub const BtdrvGattCharacteristicUuidType_ServiceChanged: BtdrvGattCharacteristicUuidType = 0x2A05;
/// Alert Level
pub const BtdrvGattCharacteristicUuidType_AlertLevel: BtdrvGattCharacteristicUuidType = 0x2A06;
/// Tx Power Level
pub const BtdrvGattCharacteristicUuidType_TxPowerLevel: BtdrvGattCharacteristicUuidType = 0x2A07;
/// Date Time
pub const BtdrvGattCharacteristicUuidType_DateTime: BtdrvGattCharacteristicUuidType = 0x2A08;
/// Day of Week
pub const BtdrvGattCharacteristicUuidType_DayOfWeek: BtdrvGattCharacteristicUuidType = 0x2A09;
/// Day Date Time
pub const BtdrvGattCharacteristicUuidType_DayDateTime: BtdrvGattCharacteristicUuidType = 0x2A0A;
/// Exact Time 256
pub const BtdrvGattCharacteristicUuidType_ExactTime256: BtdrvGattCharacteristicUuidType = 0x2A0C;
/// DST Offset
pub const BtdrvGattCharacteristicUuidType_DstOffset: BtdrvGattCharacteristicUuidType = 0x2A0D;
/// Time Zone
pub const BtdrvGattCharacteristicUuidType_TimeZone: BtdrvGattCharacteristicUuidType = 0x2A0E;
/// Local Time Information
pub const BtdrvGattCharacteristicUuidType_LocalTimeInformation: BtdrvGattCharacteristicUuidType = 0x2A0F;
/// Time with DST
pub const BtdrvGattCharacteristicUuidType_TimeWithDst: BtdrvGattCharacteristicUuidType = 0x2A11;
/// Time Accuracy
pub const BtdrvGattCharacteristicUuidType_TimeAccuracy: BtdrvGattCharacteristicUuidType = 0x2A12;
/// Time Source
pub const BtdrvGattCharacteristicUuidType_TimeSource: BtdrvGattCharacteristicUuidType = 0x2A13;
/// Reference Time Information
pub const BtdrvGattCharacteristicUuidType_ReferenceTimeInformation: BtdrvGattCharacteristicUuidType = 0x2A14;
/// Time Update Control Point
pub const BtdrvGattCharacteristicUuidType_TimeUpdateControlPoint: BtdrvGattCharacteristicUuidType = 0x2A16;
/// Time Update State
pub const BtdrvGattCharacteristicUuidType_TimeUpdateState: BtdrvGattCharacteristicUuidType = 0x2A17;
/// Glucose Measurement
pub const BtdrvGattCharacteristicUuidType_GlucoseMeasurement: BtdrvGattCharacteristicUuidType = 0x2A18;
/// Battery Level
pub const BtdrvGattCharacteristicUuidType_BatteryLevel: BtdrvGattCharacteristicUuidType = 0x2A19;
/// Temperature Measurement
pub const BtdrvGattCharacteristicUuidType_TemperatureMeasurement: BtdrvGattCharacteristicUuidType = 0x2A1C;
/// Temperature Type
pub const BtdrvGattCharacteristicUuidType_TemperatureType: BtdrvGattCharacteristicUuidType = 0x2A1D;
/// Intermediate Temperature
pub const BtdrvGattCharacteristicUuidType_IntermediateTemperature: BtdrvGattCharacteristicUuidType = 0x2A1E;
/// Measurement Interval
pub const BtdrvGattCharacteristicUuidType_MeasurementInterval: BtdrvGattCharacteristicUuidType = 0x2A21;
/// Boot Keyboard Input Report
pub const BtdrvGattCharacteristicUuidType_BootKeyboardInputReport: BtdrvGattCharacteristicUuidType = 0x2A22;
/// System ID
pub const BtdrvGattCharacteristicUuidType_SystemId: BtdrvGattCharacteristicUuidType = 0x2A23;
/// Model Number String
pub const BtdrvGattCharacteristicUuidType_ModelNumberString: BtdrvGattCharacteristicUuidType = 0x2A24;
/// Serial Number String
pub const BtdrvGattCharacteristicUuidType_SerialNumberString: BtdrvGattCharacteristicUuidType = 0x2A25;
/// Firmware Revision String
pub const BtdrvGattCharacteristicUuidType_FirmwareRevisionString: BtdrvGattCharacteristicUuidType = 0x2A26;
/// Hardware Revision String
pub const BtdrvGattCharacteristicUuidType_HardwareRevisionString: BtdrvGattCharacteristicUuidType = 0x2A27;
/// Software Revision String
pub const BtdrvGattCharacteristicUuidType_SoftwareRevisionString: BtdrvGattCharacteristicUuidType = 0x2A28;
/// Manufacturer Name String
pub const BtdrvGattCharacteristicUuidType_ManufacturerNameString: BtdrvGattCharacteristicUuidType = 0x2A29;
/// IEEE 11073-20601 Regulatory Certification Data List
pub const BtdrvGattCharacteristicUuidType_IeeeRegulatoryCertificationDataList: BtdrvGattCharacteristicUuidType = 0x2A2A;
/// Current Time
pub const BtdrvGattCharacteristicUuidType_CurrentTime: BtdrvGattCharacteristicUuidType = 0x2A2B;
/// Magnetic Declination
pub const BtdrvGattCharacteristicUuidType_MagneticDeclination: BtdrvGattCharacteristicUuidType = 0x2A2C;
/// Scan Refresh
pub const BtdrvGattCharacteristicUuidType_ScanRefresh: BtdrvGattCharacteristicUuidType = 0x2A31;
/// Boot Keyboard Output Report
pub const BtdrvGattCharacteristicUuidType_BootKeyboardOutputReport: BtdrvGattCharacteristicUuidType = 0x2A32;
/// Boot Mouse Input Report
pub const BtdrvGattCharacteristicUuidType_BootMouseInputReport: BtdrvGattCharacteristicUuidType = 0x2A33;
/// Glucose Measurement Context
pub const BtdrvGattCharacteristicUuidType_GlucoseMeasurementContext: BtdrvGattCharacteristicUuidType = 0x2A34;
/// Blood Pressure Measurement
pub const BtdrvGattCharacteristicUuidType_BloodPressureMeasurement: BtdrvGattCharacteristicUuidType = 0x2A35;
/// Intermediate Cuff Pressure
pub const BtdrvGattCharacteristicUuidType_IntermediateCuffPressure: BtdrvGattCharacteristicUuidType = 0x2A36;
/// Heart Rate Measurement
pub const BtdrvGattCharacteristicUuidType_HeartRateMeasurement: BtdrvGattCharacteristicUuidType = 0x2A37;
/// Body Sensor Location
pub const BtdrvGattCharacteristicUuidType_BodySensorLocation: BtdrvGattCharacteristicUuidType = 0x2A38;
/// Heart Rate Control Point
pub const BtdrvGattCharacteristicUuidType_HeartRateControlPoint: BtdrvGattCharacteristicUuidType = 0x2A39;
/// Alert Status
pub const BtdrvGattCharacteristicUuidType_AlertStatus: BtdrvGattCharacteristicUuidType = 0x2A3F;
/// Ringer Control Point
pub const BtdrvGattCharacteristicUuidType_RingerControlPoint: BtdrvGattCharacteristicUuidType = 0x2A40;
/// Ringer Setting
pub const BtdrvGattCharacteristicUuidType_RingerSetting: BtdrvGattCharacteristicUuidType = 0x2A41;
/// Alert Category ID Bit Mask
pub const BtdrvGattCharacteristicUuidType_AlertCategoryIdBitMask: BtdrvGattCharacteristicUuidType = 0x2A42;
/// Alert Category ID
pub const BtdrvGattCharacteristicUuidType_AlertCategoryId: BtdrvGattCharacteristicUuidType = 0x2A43;
/// Alert Notification Control Point
pub const BtdrvGattCharacteristicUuidType_AlertNotificationControlPoint: BtdrvGattCharacteristicUuidType = 0x2A44;
/// Unread Alert Status
pub const BtdrvGattCharacteristicUuidType_UnreadAlertStatus: BtdrvGattCharacteristicUuidType = 0x2A45;
/// New Alert
pub const BtdrvGattCharacteristicUuidType_NewAlert: BtdrvGattCharacteristicUuidType = 0x2A46;
/// Supported New Alert Category
pub const BtdrvGattCharacteristicUuidType_SupportedNewAlertCategory: BtdrvGattCharacteristicUuidType = 0x2A47;
/// Supported Unread Alert Category
pub const BtdrvGattCharacteristicUuidType_SupportedUnreadAlertCategory: BtdrvGattCharacteristicUuidType = 0x2A48;
/// Blood Pressure Feature
pub const BtdrvGattCharacteristicUuidType_BloodPressureFeature: BtdrvGattCharacteristicUuidType = 0x2A49;
/// HID Information
pub const BtdrvGattCharacteristicUuidType_HidInformation: BtdrvGattCharacteristicUuidType = 0x2A4A;
/// Report Map
pub const BtdrvGattCharacteristicUuidType_ReportMap: BtdrvGattCharacteristicUuidType = 0x2A4B;
/// HID Control Point
pub const BtdrvGattCharacteristicUuidType_HidControlPoint: BtdrvGattCharacteristicUuidType = 0x2A4C;
/// Report
pub const BtdrvGattCharacteristicUuidType_Report: BtdrvGattCharacteristicUuidType = 0x2A4D;
/// Protocol Mode
pub const BtdrvGattCharacteristicUuidType_ProtocolMode: BtdrvGattCharacteristicUuidType = 0x2A4E;
/// Scan Interval Window
pub const BtdrvGattCharacteristicUuidType_ScanIntervalWindow: BtdrvGattCharacteristicUuidType = 0x2A4F;
/// PnP ID
pub const BtdrvGattCharacteristicUuidType_PnpId: BtdrvGattCharacteristicUuidType = 0x2A50;
/// Glucose Feature
pub const BtdrvGattCharacteristicUuidType_GlucoseFeature: BtdrvGattCharacteristicUuidType = 0x2A51;
/// Record Access Control Point
pub const BtdrvGattCharacteristicUuidType_RecordAccessControlPoint: BtdrvGattCharacteristicUuidType = 0x2A52;
/// RSC Measurement
pub const BtdrvGattCharacteristicUuidType_RscMeasurement: BtdrvGattCharacteristicUuidType = 0x2A53;
/// RSC Feature
pub const BtdrvGattCharacteristicUuidType_RscFeature: BtdrvGattCharacteristicUuidType = 0x2A54;
/// SC Control Point
pub const BtdrvGattCharacteristicUuidType_ScControlPoint: BtdrvGattCharacteristicUuidType = 0x2A55;
/// Aggregate
pub const BtdrvGattCharacteristicUuidType_Aggregate: BtdrvGattCharacteristicUuidType = 0x2A5A;
/// CSC Measurement
pub const BtdrvGattCharacteristicUuidType_CscMeasurement: BtdrvGattCharacteristicUuidType = 0x2A5B;
/// CSC Feature
pub const BtdrvGattCharacteristicUuidType_CscFeature: BtdrvGattCharacteristicUuidType = 0x2A5C;
/// Sensor Location
pub const BtdrvGattCharacteristicUuidType_SensorLocation: BtdrvGattCharacteristicUuidType = 0x2A5D;
/// PLX Spot-Check Measurement
pub const BtdrvGattCharacteristicUuidType_PlxSpotCheckMeasurement: BtdrvGattCharacteristicUuidType = 0x2A5E;
/// PLX Continuous Measurement
pub const BtdrvGattCharacteristicUuidType_PlxContinuousMeasurement: BtdrvGattCharacteristicUuidType = 0x2A5F;
/// PLX Features
pub const BtdrvGattCharacteristicUuidType_PlxFeatures: BtdrvGattCharacteristicUuidType = 0x2A60;
/// Cycling Power Measurement
pub const BtdrvGattCharacteristicUuidType_CyclingPowerMeasurement: BtdrvGattCharacteristicUuidType = 0x2A63;
/// Cycling Power Vector
pub const BtdrvGattCharacteristicUuidType_CyclingPowerVector: BtdrvGattCharacteristicUuidType = 0x2A64;
/// Cycling Power Feature
pub const BtdrvGattCharacteristicUuidType_CyclingPowerFeature: BtdrvGattCharacteristicUuidType = 0x2A65;
/// Cycling Power Control Point
pub const BtdrvGattCharacteristicUuidType_CyclingPowerControlPoint: BtdrvGattCharacteristicUuidType = 0x2A66;
/// Location and Speed
pub const BtdrvGattCharacteristicUuidType_LocationAndSpeed: BtdrvGattCharacteristicUuidType = 0x2A67;
/// Navigation
pub const BtdrvGattCharacteristicUuidType_Navigation: BtdrvGattCharacteristicUuidType = 0x2A68;
/// Position Quality
pub const BtdrvGattCharacteristicUuidType_PositionQuality: BtdrvGattCharacteristicUuidType = 0x2A69;
/// LN Feature
pub const BtdrvGattCharacteristicUuidType_LnFeature: BtdrvGattCharacteristicUuidType = 0x2A6A;
/// LN Control Point
pub const BtdrvGattCharacteristicUuidType_LnControlPoint: BtdrvGattCharacteristicUuidType = 0x2A6B;
/// Elevation
pub const BtdrvGattCharacteristicUuidType_Elevation: BtdrvGattCharacteristicUuidType = 0x2A6C;
/// Pressure
pub const BtdrvGattCharacteristicUuidType_Pressure: BtdrvGattCharacteristicUuidType = 0x2A6D;
/// Temperature
pub const BtdrvGattCharacteristicUuidType_Temperature: BtdrvGattCharacteristicUuidType = 0x2A6E;
/// Humidity
pub const BtdrvGattCharacteristicUuidType_Humidity: BtdrvGattCharacteristicUuidType = 0x2A6F;
/// True Wind Speed
pub const BtdrvGattCharacteristicUuidType_TrueWindSpeed: BtdrvGattCharacteristicUuidType = 0x2A70;
/// True Wind Direction
pub const BtdrvGattCharacteristicUuidType_TrueWindDirection: BtdrvGattCharacteristicUuidType = 0x2A71;
/// Apparent Wind Speed
pub const BtdrvGattCharacteristicUuidType_ApparentWindSpeed: BtdrvGattCharacteristicUuidType = 0x2A72;
/// Apparent Wind Direction
pub const BtdrvGattCharacteristicUuidType_ApparentWindDirection: BtdrvGattCharacteristicUuidType = 0x2A73;
/// Gust Factor
pub const BtdrvGattCharacteristicUuidType_GustFactor: BtdrvGattCharacteristicUuidType = 0x2A74;
/// Pollen Concentration
pub const BtdrvGattCharacteristicUuidType_PollenConcentration: BtdrvGattCharacteristicUuidType = 0x2A75;
/// UV Index
pub const BtdrvGattCharacteristicUuidType_UvIndex: BtdrvGattCharacteristicUuidType = 0x2A76;
/// Irradiance
pub const BtdrvGattCharacteristicUuidType_Irradiance: BtdrvGattCharacteristicUuidType = 0x2A77;
/// Rainfall
pub const BtdrvGattCharacteristicUuidType_Rainfall: BtdrvGattCharacteristicUuidType = 0x2A78;
/// Wind Chill
pub const BtdrvGattCharacteristicUuidType_WindChill: BtdrvGattCharacteristicUuidType = 0x2A79;
/// Heat Index
pub const BtdrvGattCharacteristicUuidType_HeatIndex: BtdrvGattCharacteristicUuidType = 0x2A7A;
/// Dew Point
pub const BtdrvGattCharacteristicUuidType_DewPoint: BtdrvGattCharacteristicUuidType = 0x2A7B;
/// Descriptor Value Changed
pub const BtdrvGattCharacteristicUuidType_DescriptorValueChanged: BtdrvGattCharacteristicUuidType = 0x2A7D;
/// Aerobic Heart Rate Lower Limit
pub const BtdrvGattCharacteristicUuidType_AerobicHeartRateLowerLimit: BtdrvGattCharacteristicUuidType = 0x2A7E;
/// Aerobic Threshold
pub const BtdrvGattCharacteristicUuidType_AerobicThreshold: BtdrvGattCharacteristicUuidType = 0x2A7F;
/// Age
pub const BtdrvGattCharacteristicUuidType_Age: BtdrvGattCharacteristicUuidType = 0x2A80;
/// Anaerobic Heart Rate Lower Limit
pub const BtdrvGattCharacteristicUuidType_AnaerobicHeartRateLowerLimit: BtdrvGattCharacteristicUuidType = 0x2A81;
/// Anaerobic Heart Rate Upper Limit
pub const BtdrvGattCharacteristicUuidType_AnaerobicHeartRateUpperLimit: BtdrvGattCharacteristicUuidType = 0x2A82;
/// Anaerobic Threshold
pub const BtdrvGattCharacteristicUuidType_AnaerobicThreshold: BtdrvGattCharacteristicUuidType = 0x2A83;
/// Aerobic Heart Rate Upper Limit
pub const BtdrvGattCharacteristicUuidType_AerobicHeartRateUpperLimit: BtdrvGattCharacteristicUuidType = 0x2A84;
/// Date of Birth
pub const BtdrvGattCharacteristicUuidType_DateOfBirth: BtdrvGattCharacteristicUuidType = 0x2A85;
/// Date of Threshold Assessment
pub const BtdrvGattCharacteristicUuidType_DateOfThresholdAssessment: BtdrvGattCharacteristicUuidType = 0x2A86;
/// Email Address
pub const BtdrvGattCharacteristicUuidType_EmailAddress: BtdrvGattCharacteristicUuidType = 0x2A87;
/// Fat Burn Heart Rate Lower Limit
pub const BtdrvGattCharacteristicUuidType_FatBurnHeartRateLowerLimit: BtdrvGattCharacteristicUuidType = 0x2A88;
/// Fat Burn Heart Rate Upper Limit
pub const BtdrvGattCharacteristicUuidType_FatBurnHeartRateUpperLimit: BtdrvGattCharacteristicUuidType = 0x2A89;
/// First Name
pub const BtdrvGattCharacteristicUuidType_FirstName: BtdrvGattCharacteristicUuidType = 0x2A8A;
/// Five Zone Heart Rate Limits
pub const BtdrvGattCharacteristicUuidType_FiveZoneHeartRateLimits: BtdrvGattCharacteristicUuidType = 0x2A8B;
/// Gender
pub const BtdrvGattCharacteristicUuidType_Gender: BtdrvGattCharacteristicUuidType = 0x2A8C;
/// Heart Rate Max
pub const BtdrvGattCharacteristicUuidType_HeartRateMax: BtdrvGattCharacteristicUuidType = 0x2A8D;
/// Height
pub const BtdrvGattCharacteristicUuidType_Height: BtdrvGattCharacteristicUuidType = 0x2A8E;
/// Hip Circumference
pub const BtdrvGattCharacteristicUuidType_HipCircumference: BtdrvGattCharacteristicUuidType = 0x2A8F;
/// Last Name
pub const BtdrvGattCharacteristicUuidType_LastName: BtdrvGattCharacteristicUuidType = 0x2A90;
/// Maximum Recommended Heart Rate
pub const BtdrvGattCharacteristicUuidType_MaximumRecommendedHeartRate: BtdrvGattCharacteristicUuidType = 0x2A91;
/// Resting Heart Rate
pub const BtdrvGattCharacteristicUuidType_RestingHeartRate: BtdrvGattCharacteristicUuidType = 0x2A92;
/// Sport Type for Aerobic and Anaerobic Thresholds
pub const BtdrvGattCharacteristicUuidType_SportTypeForAerobicAndAnaerobicThresholds: BtdrvGattCharacteristicUuidType = 0x2A93;
/// Three Zone Heart Rate Limits
pub const BtdrvGattCharacteristicUuidType_ThreeZoneHeartRateLimits: BtdrvGattCharacteristicUuidType = 0x2A94;
/// Two Zone Heart Rate Limits
pub const BtdrvGattCharacteristicUuidType_TwoZoneHeartRateLimits: BtdrvGattCharacteristicUuidType = 0x2A95;
/// VO2 Max
pub const BtdrvGattCharacteristicUuidType_Vo2Max: BtdrvGattCharacteristicUuidType = 0x2A96;
/// Waist Circumference
pub const BtdrvGattCharacteristicUuidType_WaistCircumference: BtdrvGattCharacteristicUuidType = 0x2A97;
/// Weight
pub const BtdrvGattCharacteristicUuidType_Weight: BtdrvGattCharacteristicUuidType = 0x2A98;
/// Database Change Increment
pub const BtdrvGattCharacteristicUuidType_DatabaseChangeIncrement: BtdrvGattCharacteristicUuidType = 0x2A99;
/// User Index
pub const BtdrvGattCharacteristicUuidType_UserIndex: BtdrvGattCharacteristicUuidType = 0x2A9A;
/// Body Composition Feature
pub const BtdrvGattCharacteristicUuidType_BodyCompositionFeature: BtdrvGattCharacteristicUuidType = 0x2A9B;
/// Body Composition Measurement
pub const BtdrvGattCharacteristicUuidType_BodyCompositionMeasurement: BtdrvGattCharacteristicUuidType = 0x2A9C;
/// Weight Measurement
pub const BtdrvGattCharacteristicUuidType_WeightMeasurement: BtdrvGattCharacteristicUuidType = 0x2A9D;
/// Weight Scale Feature
pub const BtdrvGattCharacteristicUuidType_WeightScaleFeature: BtdrvGattCharacteristicUuidType = 0x2A9E;
/// User Control Point
pub const BtdrvGattCharacteristicUuidType_UserControlPoint: BtdrvGattCharacteristicUuidType = 0x2A9F;
/// Magnetic Flux Density - 2D
pub const BtdrvGattCharacteristicUuidType_MagneticFluxDensity2D: BtdrvGattCharacteristicUuidType = 0x2AA0;
/// Magnetic Flux Density - 3D
pub const BtdrvGattCharacteristicUuidType_MagneticFluxDensity3D: BtdrvGattCharacteristicUuidType = 0x2AA1;
/// Language
pub const BtdrvGattCharacteristicUuidType_Language: BtdrvGattCharacteristicUuidType = 0x2AA2;
/// Barometric Pressure Trend
pub const BtdrvGattCharacteristicUuidType_BarometricPressureTrend: BtdrvGattCharacteristicUuidType = 0x2AA3;
/// Bond Management Control Point
pub const BtdrvGattCharacteristicUuidType_BondManagementControlPoint: BtdrvGattCharacteristicUuidType = 0x2AA4;
/// Bond Management Feature
pub const BtdrvGattCharacteristicUuidType_BondManagementFeature: BtdrvGattCharacteristicUuidType = 0x2AA5;
/// Central Address Resolution
pub const BtdrvGattCharacteristicUuidType_CentralAddressResolution: BtdrvGattCharacteristicUuidType = 0x2AA6;
/// CGM Measurement
pub const BtdrvGattCharacteristicUuidType_CgmMeasurement: BtdrvGattCharacteristicUuidType = 0x2AA7;
/// CGM Feature
pub const BtdrvGattCharacteristicUuidType_CgmFeature: BtdrvGattCharacteristicUuidType = 0x2AA8;
/// CGM Status
pub const BtdrvGattCharacteristicUuidType_CgmStatus: BtdrvGattCharacteristicUuidType = 0x2AA9;
/// CGM Session Start Time
pub const BtdrvGattCharacteristicUuidType_CgmSessionStartTime: BtdrvGattCharacteristicUuidType = 0x2AAA;
/// CGM Session Run Time
pub const BtdrvGattCharacteristicUuidType_CgmSessionRunTime: BtdrvGattCharacteristicUuidType = 0x2AAB;
/// CGM Specific Ops Control Point
pub const BtdrvGattCharacteristicUuidType_CgmSpecificOpsControlPoint: BtdrvGattCharacteristicUuidType = 0x2AAC;
/// Indoor Positioning Configuration
pub const BtdrvGattCharacteristicUuidType_IndoorPositioningConfiguration: BtdrvGattCharacteristicUuidType = 0x2AAD;
/// Latitude
pub const BtdrvGattCharacteristicUuidType_Latitude: BtdrvGattCharacteristicUuidType = 0x2AAE;
/// Longitude
pub const BtdrvGattCharacteristicUuidType_Longitude: BtdrvGattCharacteristicUuidType = 0x2AAF;
/// Local North Coordinate
pub const BtdrvGattCharacteristicUuidType_LocalNorthCoordinate: BtdrvGattCharacteristicUuidType = 0x2AB0;
/// Local East Coordinate
pub const BtdrvGattCharacteristicUuidType_LocalEastCoordinate: BtdrvGattCharacteristicUuidType = 0x2AB1;
/// Floor Number
pub const BtdrvGattCharacteristicUuidType_FloorNumber: BtdrvGattCharacteristicUuidType = 0x2AB2;
/// Altitude
pub const BtdrvGattCharacteristicUuidType_Altitude: BtdrvGattCharacteristicUuidType = 0x2AB3;
/// Uncertainty
pub const BtdrvGattCharacteristicUuidType_Uncertainty: BtdrvGattCharacteristicUuidType = 0x2AB4;
/// Location Name
pub const BtdrvGattCharacteristicUuidType_LocationName: BtdrvGattCharacteristicUuidType = 0x2AB5;
/// URI
pub const BtdrvGattCharacteristicUuidType_Uri: BtdrvGattCharacteristicUuidType = 0x2AB6;
/// HTTP Headers
pub const BtdrvGattCharacteristicUuidType_HttpHeaders: BtdrvGattCharacteristicUuidType = 0x2AB7;
/// HTTP Status Code
pub const BtdrvGattCharacteristicUuidType_HttpStatusCode: BtdrvGattCharacteristicUuidType = 0x2AB8;
/// HTTP Entity Body
pub const BtdrvGattCharacteristicUuidType_HttpEntityBody: BtdrvGattCharacteristicUuidType = 0x2AB9;
/// HTTP Control Point
pub const BtdrvGattCharacteristicUuidType_HttpControlPoint: BtdrvGattCharacteristicUuidType = 0x2ABA;
/// HTTPS Security
pub const BtdrvGattCharacteristicUuidType_HttpsSecurity: BtdrvGattCharacteristicUuidType = 0x2ABB;
/// TDS Control Point
pub const BtdrvGattCharacteristicUuidType_TdsControlPoint: BtdrvGattCharacteristicUuidType = 0x2ABC;
/// OTS Feature
pub const BtdrvGattCharacteristicUuidType_OtsFeature: BtdrvGattCharacteristicUuidType = 0x2ABD;
/// Object Name
pub const BtdrvGattCharacteristicUuidType_ObjectName: BtdrvGattCharacteristicUuidType = 0x2ABE;
/// Object Type
pub const BtdrvGattCharacteristicUuidType_ObjectType: BtdrvGattCharacteristicUuidType = 0x2ABF;
/// Object Size
pub const BtdrvGattCharacteristicUuidType_ObjectSize: BtdrvGattCharacteristicUuidType = 0x2AC0;
/// Object First-Created
pub const BtdrvGattCharacteristicUuidType_ObjectFirstCreated: BtdrvGattCharacteristicUuidType = 0x2AC1;
/// Object Last-Modified
pub const BtdrvGattCharacteristicUuidType_ObjectLastModified: BtdrvGattCharacteristicUuidType = 0x2AC2;
/// Object ID
pub const BtdrvGattCharacteristicUuidType_ObjectId: BtdrvGattCharacteristicUuidType = 0x2AC3;
/// Object Properties
pub const BtdrvGattCharacteristicUuidType_ObjectProperties: BtdrvGattCharacteristicUuidType = 0x2AC4;
/// Object Action Control Point
pub const BtdrvGattCharacteristicUuidType_ObjectActionControlPoint: BtdrvGattCharacteristicUuidType = 0x2AC5;
/// Object List Control Point
pub const BtdrvGattCharacteristicUuidType_ObjectListControlPoint: BtdrvGattCharacteristicUuidType = 0x2AC6;
/// Object List Filter
pub const BtdrvGattCharacteristicUuidType_ObjectListFilter: BtdrvGattCharacteristicUuidType = 0x2AC7;
/// Object Changed
pub const BtdrvGattCharacteristicUuidType_ObjectChanged: BtdrvGattCharacteristicUuidType = 0x2AC8;
/// Resolvable Private Address Only
pub const BtdrvGattCharacteristicUuidType_ResolvablePrivateAddressOnly: BtdrvGattCharacteristicUuidType = 0x2AC9;
/// Fitness Machine Feature
pub const BtdrvGattCharacteristicUuidType_FitnessMachineFeature: BtdrvGattCharacteristicUuidType = 0x2ACC;
/// Treadmill Data
pub const BtdrvGattCharacteristicUuidType_TreadmillData: BtdrvGattCharacteristicUuidType = 0x2ACD;
/// Cross Trainer Data
pub const BtdrvGattCharacteristicUuidType_CrossTrainerData: BtdrvGattCharacteristicUuidType = 0x2ACE;
/// Step Climber Data
pub const BtdrvGattCharacteristicUuidType_StepClimberData: BtdrvGattCharacteristicUuidType = 0x2ACF;
/// Stair Climber Data
pub const BtdrvGattCharacteristicUuidType_StairClimberData: BtdrvGattCharacteristicUuidType = 0x2AD0;
/// Rower Data
pub const BtdrvGattCharacteristicUuidType_RowerData: BtdrvGattCharacteristicUuidType = 0x2AD1;
/// Indoor Bike Data
pub const BtdrvGattCharacteristicUuidType_IndoorBikeData: BtdrvGattCharacteristicUuidType = 0x2AD2;
/// Training Status
pub const BtdrvGattCharacteristicUuidType_TrainingStatus: BtdrvGattCharacteristicUuidType = 0x2AD3;
/// Supported Speed Range
pub const BtdrvGattCharacteristicUuidType_SupportedSpeedRange: BtdrvGattCharacteristicUuidType = 0x2AD4;
/// Supported Inclination Range
pub const BtdrvGattCharacteristicUuidType_SupportedInclinationRange: BtdrvGattCharacteristicUuidType = 0x2AD5;
/// Supported Resistance Level Range
pub const BtdrvGattCharacteristicUuidType_SupportedResistanceLevelRange: BtdrvGattCharacteristicUuidType = 0x2AD6;
/// Supported Heart Rate Range
pub const BtdrvGattCharacteristicUuidType_SupportedHeartRateRange: BtdrvGattCharacteristicUuidType = 0x2AD7;
/// Supported Power Range
pub const BtdrvGattCharacteristicUuidType_SupportedPowerRange: BtdrvGattCharacteristicUuidType = 0x2AD8;
/// Fitness Machine Control Point
pub const BtdrvGattCharacteristicUuidType_FitnessMachineControlPoint: BtdrvGattCharacteristicUuidType = 0x2AD9;
/// Fitness Machine Status
pub const BtdrvGattCharacteristicUuidType_FitnessMachineStatus: BtdrvGattCharacteristicUuidType = 0x2ADA;
/// Mesh Provisioning Data In
pub const BtdrvGattCharacteristicUuidType_MeshProvisioningDataIn: BtdrvGattCharacteristicUuidType = 0x2ADB;
/// Mesh Provisioning Data Out
pub const BtdrvGattCharacteristicUuidType_MeshProvisioningDataOut: BtdrvGattCharacteristicUuidType = 0x2ADC;
/// Mesh Proxy Data In
pub const BtdrvGattCharacteristicUuidType_MeshProxyDataIn: BtdrvGattCharacteristicUuidType = 0x2ADD;
/// Mesh Proxy Data Out
pub const BtdrvGattCharacteristicUuidType_MeshProxyDataOut: BtdrvGattCharacteristicUuidType = 0x2ADE;
/// Average Current
pub const BtdrvGattCharacteristicUuidType_AverageCurrent: BtdrvGattCharacteristicUuidType = 0x2AE0;
/// Average Voltage
pub const BtdrvGattCharacteristicUuidType_AverageVoltage: BtdrvGattCharacteristicUuidType = 0x2AE1;
/// Boolean
pub const BtdrvGattCharacteristicUuidType_Boolean: BtdrvGattCharacteristicUuidType = 0x2AE2;
/// Chromatic Distance from Planckian
pub const BtdrvGattCharacteristicUuidType_ChromaticDistanceFromPlanckian: BtdrvGattCharacteristicUuidType = 0x2AE3;
/// Chromaticity Coordinates
pub const BtdrvGattCharacteristicUuidType_ChromaticityCoordinates: BtdrvGattCharacteristicUuidType = 0x2AE4;
/// Chromaticity in CCT and Duv Values
pub const BtdrvGattCharacteristicUuidType_ChromaticityInCctAndDuvValues: BtdrvGattCharacteristicUuidType = 0x2AE5;
/// Chromaticity Tolerance
pub const BtdrvGattCharacteristicUuidType_ChromaticityTolerance: BtdrvGattCharacteristicUuidType = 0x2AE6;
/// CIE 13.3-1995 Color Rendering Index
pub const BtdrvGattCharacteristicUuidType_CieColorRenderingIndex: BtdrvGattCharacteristicUuidType = 0x2AE7;
/// Coefficient
pub const BtdrvGattCharacteristicUuidType_Coefficient: BtdrvGattCharacteristicUuidType = 0x2AE8;
/// Correlated Color Temperature
pub const BtdrvGattCharacteristicUuidType_CorrelatedColorTemperature: BtdrvGattCharacteristicUuidType = 0x2AE9;
/// Count 16
pub const BtdrvGattCharacteristicUuidType_Count16: BtdrvGattCharacteristicUuidType = 0x2AEA;
/// Count 24
pub const BtdrvGattCharacteristicUuidType_Count24: BtdrvGattCharacteristicUuidType = 0x2AEB;
/// Country Code
pub const BtdrvGattCharacteristicUuidType_CountryCode: BtdrvGattCharacteristicUuidType = 0x2AEC;
/// Date UTC
pub const BtdrvGattCharacteristicUuidType_DateUtc: BtdrvGattCharacteristicUuidType = 0x2AED;
/// Electric Current
pub const BtdrvGattCharacteristicUuidType_ElectricCurrent: BtdrvGattCharacteristicUuidType = 0x2AEE;
/// Electric Current Range
pub const BtdrvGattCharacteristicUuidType_ElectricCurrentRange: BtdrvGattCharacteristicUuidType = 0x2AEF;
/// Electric Current Specification
pub const BtdrvGattCharacteristicUuidType_ElectricCurrentSpecification: BtdrvGattCharacteristicUuidType = 0x2AF0;
/// Electric Current Statistics
pub const BtdrvGattCharacteristicUuidType_ElectricCurrentStatistics: BtdrvGattCharacteristicUuidType = 0x2AF1;
/// Energy
pub const BtdrvGattCharacteristicUuidType_Energy: BtdrvGattCharacteristicUuidType = 0x2AF2;
/// Energy in a Period of Day
pub const BtdrvGattCharacteristicUuidType_EnergyInAPeriodOfDay: BtdrvGattCharacteristicUuidType = 0x2AF3;
/// Event Statistics
pub const BtdrvGattCharacteristicUuidType_EventStatistics: BtdrvGattCharacteristicUuidType = 0x2AF4;
/// Fixed String 16
pub const BtdrvGattCharacteristicUuidType_FixedString16: BtdrvGattCharacteristicUuidType = 0x2AF5;
/// Fixed String 24
pub const BtdrvGattCharacteristicUuidType_FixedString24: BtdrvGattCharacteristicUuidType = 0x2AF6;
/// Fixed String 36
pub const BtdrvGattCharacteristicUuidType_FixedString36: BtdrvGattCharacteristicUuidType = 0x2AF7;
/// Fixed String 8
pub const BtdrvGattCharacteristicUuidType_FixedString8: BtdrvGattCharacteristicUuidType = 0x2AF8;
/// Generic Level
pub const BtdrvGattCharacteristicUuidType_GenericLevel: BtdrvGattCharacteristicUuidType = 0x2AF9;
/// Global Trade Item Number
pub const BtdrvGattCharacteristicUuidType_GlobalTradeItemNumber: BtdrvGattCharacteristicUuidType = 0x2AFA;
/// Illuminance
pub const BtdrvGattCharacteristicUuidType_Illuminance: BtdrvGattCharacteristicUuidType = 0x2AFB;
/// Luminous Efficacy
pub const BtdrvGattCharacteristicUuidType_LuminousEfficacy: BtdrvGattCharacteristicUuidType = 0x2AFC;
/// Luminous Energy
pub const BtdrvGattCharacteristicUuidType_LuminousEnergy: BtdrvGattCharacteristicUuidType = 0x2AFD;
/// Luminous Exposure
pub const BtdrvGattCharacteristicUuidType_LuminousExposure: BtdrvGattCharacteristicUuidType = 0x2AFE;
/// Luminous Flux
pub const BtdrvGattCharacteristicUuidType_LuminousFlux: BtdrvGattCharacteristicUuidType = 0x2AFF;
/// Luminous Flux Range
pub const BtdrvGattCharacteristicUuidType_LuminousFluxRange: BtdrvGattCharacteristicUuidType = 0x2B00;
/// Luminous Intensity
pub const BtdrvGattCharacteristicUuidType_LuminousIntensity: BtdrvGattCharacteristicUuidType = 0x2B01;
/// Mass Flow
pub const BtdrvGattCharacteristicUuidType_MassFlow: BtdrvGattCharacteristicUuidType = 0x2B02;
/// Perceived Lightness
pub const BtdrvGattCharacteristicUuidType_PerceivedLightness: BtdrvGattCharacteristicUuidType = 0x2B03;
/// Percentage 8
pub const BtdrvGattCharacteristicUuidType_Percentage8: BtdrvGattCharacteristicUuidType = 0x2B04;
/// Power
pub const BtdrvGattCharacteristicUuidType_Power: BtdrvGattCharacteristicUuidType = 0x2B05;
/// Power Specification
pub const BtdrvGattCharacteristicUuidType_PowerSpecification: BtdrvGattCharacteristicUuidType = 0x2B06;
/// Relative Runtime in a Current Range
pub const BtdrvGattCharacteristicUuidType_RelativeRuntimeInACurrentRange: BtdrvGattCharacteristicUuidType = 0x2B07;
/// Relative Runtime in a Generic Level Range
pub const BtdrvGattCharacteristicUuidType_RelativeRuntimeInAGenericLevelRange: BtdrvGattCharacteristicUuidType = 0x2B08;
/// Relative Value in a Voltage Range
pub const BtdrvGattCharacteristicUuidType_RelativeValueInAVoltageRange: BtdrvGattCharacteristicUuidType = 0x2B09;
/// Relative Value in an Illuminance Range
pub const BtdrvGattCharacteristicUuidType_RelativeValueInAnIlluminanceRange: BtdrvGattCharacteristicUuidType = 0x2B0A;
/// Relative Value in a Period of Day
pub const BtdrvGattCharacteristicUuidType_RelativeValueInAPeriodOfDay: BtdrvGattCharacteristicUuidType = 0x2B0B;
/// Relative Value in a Temperature Range
pub const BtdrvGattCharacteristicUuidType_RelativeValueInATemperatureRange: BtdrvGattCharacteristicUuidType = 0x2B0C;
/// Temperature 8
pub const BtdrvGattCharacteristicUuidType_Temperature8: BtdrvGattCharacteristicUuidType = 0x2B0D;
/// Temperature 8 in a Period of Day
pub const BtdrvGattCharacteristicUuidType_Temperature8InAPeriodOfDay: BtdrvGattCharacteristicUuidType = 0x2B0E;
/// Temperature 8 Statistics
pub const BtdrvGattCharacteristicUuidType_Temperature8Statistics: BtdrvGattCharacteristicUuidType = 0x2B0F;
/// Temperature Range
pub const BtdrvGattCharacteristicUuidType_TemperatureRange: BtdrvGattCharacteristicUuidType = 0x2B10;
/// Temperature Statistics
pub const BtdrvGattCharacteristicUuidType_TemperatureStatistics: BtdrvGattCharacteristicUuidType = 0x2B11;
/// Time Decihour 8
pub const BtdrvGattCharacteristicUuidType_TimeDecihour8: BtdrvGattCharacteristicUuidType = 0x2B12;
/// Time Exponential 8
pub const BtdrvGattCharacteristicUuidType_TimeExponential8: BtdrvGattCharacteristicUuidType = 0x2B13;
/// Time Hour 24
pub const BtdrvGattCharacteristicUuidType_TimeHour24: BtdrvGattCharacteristicUuidType = 0x2B14;
/// Time Millisecond 24
pub const BtdrvGattCharacteristicUuidType_TimeMillisecond24: BtdrvGattCharacteristicUuidType = 0x2B15;
/// Time Second 16
pub const BtdrvGattCharacteristicUuidType_TimeSecond16: BtdrvGattCharacteristicUuidType = 0x2B16;
/// Time Second 8
pub const BtdrvGattCharacteristicUuidType_TimeSecond8: BtdrvGattCharacteristicUuidType = 0x2B17;
/// Voltage
pub const BtdrvGattCharacteristicUuidType_Voltage: BtdrvGattCharacteristicUuidType = 0x2B18;
/// Voltage Specification
pub const BtdrvGattCharacteristicUuidType_VoltageSpecification: BtdrvGattCharacteristicUuidType = 0x2B19;
/// Voltage Statistics
pub const BtdrvGattCharacteristicUuidType_VoltageStatistics: BtdrvGattCharacteristicUuidType = 0x2B1A;
/// Volume Flow
pub const BtdrvGattCharacteristicUuidType_VolumeFlow: BtdrvGattCharacteristicUuidType = 0x2B1B;
/// Chromaticity Coordinate
pub const BtdrvGattCharacteristicUuidType_ChromaticityCoordinate: BtdrvGattCharacteristicUuidType = 0x2B1C;
/// RC Feature
pub const BtdrvGattCharacteristicUuidType_RcFeature: BtdrvGattCharacteristicUuidType = 0x2B1D;
/// RC Settings
pub const BtdrvGattCharacteristicUuidType_RcSettings: BtdrvGattCharacteristicUuidType = 0x2B1E;
/// Reconnection Configuration Control Point
pub const BtdrvGattCharacteristicUuidType_ReconnectionConfigurationControlPoint: BtdrvGattCharacteristicUuidType = 0x2B1F;
/// IDD Status Changed
pub const BtdrvGattCharacteristicUuidType_IddStatusChanged: BtdrvGattCharacteristicUuidType = 0x2B20;
/// IDD Status
pub const BtdrvGattCharacteristicUuidType_IddStatus: BtdrvGattCharacteristicUuidType = 0x2B21;
/// IDD Annunciation Status
pub const BtdrvGattCharacteristicUuidType_IddAnnunciationStatus: BtdrvGattCharacteristicUuidType = 0x2B22;
/// IDD Features
pub const BtdrvGattCharacteristicUuidType_IddFeatures: BtdrvGattCharacteristicUuidType = 0x2B23;
/// IDD Status Reader Control Point
pub const BtdrvGattCharacteristicUuidType_IddStatusReaderControlPoint: BtdrvGattCharacteristicUuidType = 0x2B24;
/// IDD Command Control Point
pub const BtdrvGattCharacteristicUuidType_IddCommandControlPoint: BtdrvGattCharacteristicUuidType = 0x2B25;
/// IDD Command Data
pub const BtdrvGattCharacteristicUuidType_IddCommandData: BtdrvGattCharacteristicUuidType = 0x2B26;
/// IDD Record Access Control Point
pub const BtdrvGattCharacteristicUuidType_IddRecordAccessControlPoint: BtdrvGattCharacteristicUuidType = 0x2B27;
/// IDD History Data
pub const BtdrvGattCharacteristicUuidType_IddHistoryData: BtdrvGattCharacteristicUuidType = 0x2B28;
/// Client Supported Features
pub const BtdrvGattCharacteristicUuidType_ClientSupportedFeatures: BtdrvGattCharacteristicUuidType = 0x2B29;
/// Database Hash
pub const BtdrvGattCharacteristicUuidType_DatabaseHash: BtdrvGattCharacteristicUuidType = 0x2B2A;
/// BSS Control Point
pub const BtdrvGattCharacteristicUuidType_BssControlPoint: BtdrvGattCharacteristicUuidType = 0x2B2B;
/// BSS Response
pub const BtdrvGattCharacteristicUuidType_BssResponse: BtdrvGattCharacteristicUuidType = 0x2B2C;
/// Emergency ID
pub const BtdrvGattCharacteristicUuidType_EmergencyId: BtdrvGattCharacteristicUuidType = 0x2B2D;
/// Emergency Text
pub const BtdrvGattCharacteristicUuidType_EmergencyText: BtdrvGattCharacteristicUuidType = 0x2B2E;
/// ACS Status
pub const BtdrvGattCharacteristicUuidType_AcsStatus: BtdrvGattCharacteristicUuidType = 0x2B2F;
/// ACS Data In
pub const BtdrvGattCharacteristicUuidType_AcsDataIn: BtdrvGattCharacteristicUuidType = 0x2B30;
/// ACS Data Out Notify
pub const BtdrvGattCharacteristicUuidType_AcsDataOutNotify: BtdrvGattCharacteristicUuidType = 0x2B31;
/// ACS Data Out Indicate
pub const BtdrvGattCharacteristicUuidType_AcsDataOutIndicate: BtdrvGattCharacteristicUuidType = 0x2B32;
/// ACS Control Point
pub const BtdrvGattCharacteristicUuidType_AcsControlPoint: BtdrvGattCharacteristicUuidType = 0x2B33;
/// Enhanced Blood Pressure Measurement
pub const BtdrvGattCharacteristicUuidType_EnhancedBloodPressureMeasurement: BtdrvGattCharacteristicUuidType = 0x2B34;
/// Enhanced Intermediate Cuff Pressure
pub const BtdrvGattCharacteristicUuidType_EnhancedIntermediateCuffPressure: BtdrvGattCharacteristicUuidType = 0x2B35;
/// Blood Pressure Record
pub const BtdrvGattCharacteristicUuidType_BloodPressureRecord: BtdrvGattCharacteristicUuidType = 0x2B36;
/// Registered User
pub const BtdrvGattCharacteristicUuidType_RegisteredUser: BtdrvGattCharacteristicUuidType = 0x2B37;
/// BR-EDR Handover Data
pub const BtdrvGattCharacteristicUuidType_BrEdrHandoverData: BtdrvGattCharacteristicUuidType = 0x2B38;
/// Bluetooth SIG Data
pub const BtdrvGattCharacteristicUuidType_BluetoothSigData: BtdrvGattCharacteristicUuidType = 0x2B39;
/// Server Supported Features
pub const BtdrvGattCharacteristicUuidType_ServerSupportedFeatures: BtdrvGattCharacteristicUuidType = 0x2B3A;
/// Physical Activity Monitor Features
pub const BtdrvGattCharacteristicUuidType_PhysicalActivityMonitorFeatures: BtdrvGattCharacteristicUuidType = 0x2B3B;
/// General Activity Instantaneous Data
pub const BtdrvGattCharacteristicUuidType_GeneralActivityInstantaneousData: BtdrvGattCharacteristicUuidType = 0x2B3C;
/// General Activity Summary Data
pub const BtdrvGattCharacteristicUuidType_GeneralActivitySummaryData: BtdrvGattCharacteristicUuidType = 0x2B3D;
/// CardioRespiratory Activity Instantaneous Data
pub const BtdrvGattCharacteristicUuidType_CardiorespiratoryActivityInstantaneousData: BtdrvGattCharacteristicUuidType = 0x2B3E;
/// CardioRespiratory Activity Summary Data
pub const BtdrvGattCharacteristicUuidType_CardiorespiratoryActivitySummaryData: BtdrvGattCharacteristicUuidType = 0x2B3F;
/// Step Counter Activity Summary Data
pub const BtdrvGattCharacteristicUuidType_StepCounterActivitySummaryData: BtdrvGattCharacteristicUuidType = 0x2B40;
/// Sleep Activity Instantaneous Data
pub const BtdrvGattCharacteristicUuidType_SleepActivityInstantaneousData: BtdrvGattCharacteristicUuidType = 0x2B41;
/// Sleep Activity Summary Data
pub const BtdrvGattCharacteristicUuidType_SleepActivitySummaryData: BtdrvGattCharacteristicUuidType = 0x2B42;
/// Physical Activity Monitor Control Point
pub const BtdrvGattCharacteristicUuidType_PhysicalActivityMonitorControlPoint: BtdrvGattCharacteristicUuidType = 0x2B43;
/// Activity Current Session
pub const BtdrvGattCharacteristicUuidType_ActivityCurrentSession: BtdrvGattCharacteristicUuidType = 0x2B44;
/// Physical Activity Session Descriptor
pub const BtdrvGattCharacteristicUuidType_PhysicalActivitySessionDescriptor: BtdrvGattCharacteristicUuidType = 0x2B45;
/// Preferred Units
pub const BtdrvGattCharacteristicUuidType_PreferredUnits: BtdrvGattCharacteristicUuidType = 0x2B46;
/// High Resolution Height
pub const BtdrvGattCharacteristicUuidType_HighResolutionHeight: BtdrvGattCharacteristicUuidType = 0x2B47;
/// Middle Name
pub const BtdrvGattCharacteristicUuidType_MiddleName: BtdrvGattCharacteristicUuidType = 0x2B48;
/// Stride Length
pub const BtdrvGattCharacteristicUuidType_StrideLength: BtdrvGattCharacteristicUuidType = 0x2B49;
/// Handedness
pub const BtdrvGattCharacteristicUuidType_Handedness: BtdrvGattCharacteristicUuidType = 0x2B4A;
/// Device Wearing Position
pub const BtdrvGattCharacteristicUuidType_DeviceWearingPosition: BtdrvGattCharacteristicUuidType = 0x2B4B;
/// Four Zone Heart Rate Limits
pub const BtdrvGattCharacteristicUuidType_FourZoneHeartRateLimits: BtdrvGattCharacteristicUuidType = 0x2B4C;
/// High Intensity Exercise Threshold
pub const BtdrvGattCharacteristicUuidType_HighIntensityExerciseThreshold: BtdrvGattCharacteristicUuidType = 0x2B4D;
/// Activity Goal
pub const BtdrvGattCharacteristicUuidType_ActivityGoal: BtdrvGattCharacteristicUuidType = 0x2B4E;
/// Sedentary Interval Notification
pub const BtdrvGattCharacteristicUuidType_SedentaryIntervalNotification: BtdrvGattCharacteristicUuidType = 0x2B4F;
/// Caloric Intake
pub const BtdrvGattCharacteristicUuidType_CaloricIntake: BtdrvGattCharacteristicUuidType = 0x2B50;
/// TMAP Role
pub const BtdrvGattCharacteristicUuidType_TmapRole: BtdrvGattCharacteristicUuidType = 0x2B51;
/// Audio Input State
pub const BtdrvGattCharacteristicUuidType_AudioInputState: BtdrvGattCharacteristicUuidType = 0x2B77;
/// Gain Settings Attribute
pub const BtdrvGattCharacteristicUuidType_GainSettingsAttribute: BtdrvGattCharacteristicUuidType = 0x2B78;
/// Audio Input Type
pub const BtdrvGattCharacteristicUuidType_AudioInputType: BtdrvGattCharacteristicUuidType = 0x2B79;
/// Audio Input Status
pub const BtdrvGattCharacteristicUuidType_AudioInputStatus: BtdrvGattCharacteristicUuidType = 0x2B7A;
/// Audio Input Control Point
pub const BtdrvGattCharacteristicUuidType_AudioInputControlPoint: BtdrvGattCharacteristicUuidType = 0x2B7B;
/// Audio Input Description
pub const BtdrvGattCharacteristicUuidType_AudioInputDescription: BtdrvGattCharacteristicUuidType = 0x2B7C;
/// Volume State
pub const BtdrvGattCharacteristicUuidType_VolumeState: BtdrvGattCharacteristicUuidType = 0x2B7D;
/// Volume Control Point
pub const BtdrvGattCharacteristicUuidType_VolumeControlPoint: BtdrvGattCharacteristicUuidType = 0x2B7E;
/// Volume Flags
pub const BtdrvGattCharacteristicUuidType_VolumeFlags: BtdrvGattCharacteristicUuidType = 0x2B7F;
/// Volume Offset State
pub const BtdrvGattCharacteristicUuidType_VolumeOffsetState: BtdrvGattCharacteristicUuidType = 0x2B80;
/// Audio Location
pub const BtdrvGattCharacteristicUuidType_AudioLocation: BtdrvGattCharacteristicUuidType = 0x2B81;
/// Volume Offset Control Point
pub const BtdrvGattCharacteristicUuidType_VolumeOffsetControlPoint: BtdrvGattCharacteristicUuidType = 0x2B82;
/// Audio Output Description
pub const BtdrvGattCharacteristicUuidType_AudioOutputDescription: BtdrvGattCharacteristicUuidType = 0x2B83;
/// Set Identity Resolving Key
pub const BtdrvGattCharacteristicUuidType_SetIdentityResolvingKey: BtdrvGattCharacteristicUuidType = 0x2B84;
/// Coordinated Set Size
pub const BtdrvGattCharacteristicUuidType_CoordinatedSetSize: BtdrvGattCharacteristicUuidType = 0x2B85;
/// Set Member Lock
pub const BtdrvGattCharacteristicUuidType_SetMemberLock: BtdrvGattCharacteristicUuidType = 0x2B86;
/// Set Member Rank
pub const BtdrvGattCharacteristicUuidType_SetMemberRank: BtdrvGattCharacteristicUuidType = 0x2B87;
/// Encrypted Data Key Material
pub const BtdrvGattCharacteristicUuidType_EncryptedDataKeyMaterial: BtdrvGattCharacteristicUuidType = 0x2B88;
/// Apparent Energy 32
pub const BtdrvGattCharacteristicUuidType_ApparentEnergy32: BtdrvGattCharacteristicUuidType = 0x2B89;
/// Apparent Power
pub const BtdrvGattCharacteristicUuidType_ApparentPower: BtdrvGattCharacteristicUuidType = 0x2B8A;
/// Live Health Observations
pub const BtdrvGattCharacteristicUuidType_LiveHealthObservations: BtdrvGattCharacteristicUuidType = 0x2B8B;
/// CO\textsubscript{2} Concentration
pub const BtdrvGattCharacteristicUuidType_CoTextsubscript2Concentration: BtdrvGattCharacteristicUuidType = 0x2B8C;
/// Cosine of the Angle
pub const BtdrvGattCharacteristicUuidType_CosineOfTheAngle: BtdrvGattCharacteristicUuidType = 0x2B8D;
/// Device Time Feature
pub const BtdrvGattCharacteristicUuidType_DeviceTimeFeature: BtdrvGattCharacteristicUuidType = 0x2B8E;
/// Device Time Parameters
pub const BtdrvGattCharacteristicUuidType_DeviceTimeParameters: BtdrvGattCharacteristicUuidType = 0x2B8F;
/// Device Time
pub const BtdrvGattCharacteristicUuidType_DeviceTime: BtdrvGattCharacteristicUuidType = 0x2B90;
/// Device Time Control Point
pub const BtdrvGattCharacteristicUuidType_DeviceTimeControlPoint: BtdrvGattCharacteristicUuidType = 0x2B91;
/// Time Change Log Data
pub const BtdrvGattCharacteristicUuidType_TimeChangeLogData: BtdrvGattCharacteristicUuidType = 0x2B92;
/// Media Player Name
pub const BtdrvGattCharacteristicUuidType_MediaPlayerName: BtdrvGattCharacteristicUuidType = 0x2B93;
/// Media Player Icon Object ID
pub const BtdrvGattCharacteristicUuidType_MediaPlayerIconObjectId: BtdrvGattCharacteristicUuidType = 0x2B94;
/// Media Player Icon URL
pub const BtdrvGattCharacteristicUuidType_MediaPlayerIconUrl: BtdrvGattCharacteristicUuidType = 0x2B95;
/// Track Changed
pub const BtdrvGattCharacteristicUuidType_TrackChanged: BtdrvGattCharacteristicUuidType = 0x2B96;
/// Track Title
pub const BtdrvGattCharacteristicUuidType_TrackTitle: BtdrvGattCharacteristicUuidType = 0x2B97;
/// Track Duration
pub const BtdrvGattCharacteristicUuidType_TrackDuration: BtdrvGattCharacteristicUuidType = 0x2B98;
/// Track Position
pub const BtdrvGattCharacteristicUuidType_TrackPosition: BtdrvGattCharacteristicUuidType = 0x2B99;
/// Playback Speed
pub const BtdrvGattCharacteristicUuidType_PlaybackSpeed: BtdrvGattCharacteristicUuidType = 0x2B9A;
/// Seeking Speed
pub const BtdrvGattCharacteristicUuidType_SeekingSpeed: BtdrvGattCharacteristicUuidType = 0x2B9B;
/// Current Track Segments Object ID
pub const BtdrvGattCharacteristicUuidType_CurrentTrackSegmentsObjectId: BtdrvGattCharacteristicUuidType = 0x2B9C;
/// Current Track Object ID
pub const BtdrvGattCharacteristicUuidType_CurrentTrackObjectId: BtdrvGattCharacteristicUuidType = 0x2B9D;
/// Next Track Object ID
pub const BtdrvGattCharacteristicUuidType_NextTrackObjectId: BtdrvGattCharacteristicUuidType = 0x2B9E;
/// Parent Group Object ID
pub const BtdrvGattCharacteristicUuidType_ParentGroupObjectId: BtdrvGattCharacteristicUuidType = 0x2B9F;
/// Current Group Object ID
pub const BtdrvGattCharacteristicUuidType_CurrentGroupObjectId: BtdrvGattCharacteristicUuidType = 0x2BA0;
/// Playing Order
pub const BtdrvGattCharacteristicUuidType_PlayingOrder: BtdrvGattCharacteristicUuidType = 0x2BA1;
/// Playing Orders Supported
pub const BtdrvGattCharacteristicUuidType_PlayingOrdersSupported: BtdrvGattCharacteristicUuidType = 0x2BA2;
/// Media State
pub const BtdrvGattCharacteristicUuidType_MediaState: BtdrvGattCharacteristicUuidType = 0x2BA3;
/// Media Control Point
pub const BtdrvGattCharacteristicUuidType_MediaControlPoint: BtdrvGattCharacteristicUuidType = 0x2BA4;
/// Media Control Point Opcodes Supported
pub const BtdrvGattCharacteristicUuidType_MediaControlPointOpcodesSupported: BtdrvGattCharacteristicUuidType = 0x2BA5;
/// Search Results Object ID
pub const BtdrvGattCharacteristicUuidType_SearchResultsObjectId: BtdrvGattCharacteristicUuidType = 0x2BA6;
/// Search Control Point
pub const BtdrvGattCharacteristicUuidType_SearchControlPoint: BtdrvGattCharacteristicUuidType = 0x2BA7;
/// Energy 32
pub const BtdrvGattCharacteristicUuidType_Energy32: BtdrvGattCharacteristicUuidType = 0x2BA8;
/// Media Player Icon Object Type
pub const BtdrvGattCharacteristicUuidType_MediaPlayerIconObjectType: BtdrvGattCharacteristicUuidType = 0x2BA9;
/// Track Segments Object Type
pub const BtdrvGattCharacteristicUuidType_TrackSegmentsObjectType: BtdrvGattCharacteristicUuidType = 0x2BAA;
/// Track Object Type
pub const BtdrvGattCharacteristicUuidType_TrackObjectType: BtdrvGattCharacteristicUuidType = 0x2BAB;
/// Group Object Type
pub const BtdrvGattCharacteristicUuidType_GroupObjectType: BtdrvGattCharacteristicUuidType = 0x2BAC;
/// Constant Tone Extension Enable
pub const BtdrvGattCharacteristicUuidType_ConstantToneExtensionEnable: BtdrvGattCharacteristicUuidType = 0x2BAD;
/// Advertising Constant Tone Extension Minimum Length
pub const BtdrvGattCharacteristicUuidType_AdvertisingConstantToneExtensionMinimumLength: BtdrvGattCharacteristicUuidType = 0x2BAE;
/// Advertising Constant Tone Extension Minimum Transmit Count
pub const BtdrvGattCharacteristicUuidType_AdvertisingConstantToneExtensionMinimumTransmitCount: BtdrvGattCharacteristicUuidType = 0x2BAF;
/// Advertising Constant Tone Extension Transmit Duration
pub const BtdrvGattCharacteristicUuidType_AdvertisingConstantToneExtensionTransmitDuration: BtdrvGattCharacteristicUuidType = 0x2BB0;
/// Advertising Constant Tone Extension Interval
pub const BtdrvGattCharacteristicUuidType_AdvertisingConstantToneExtensionInterval: BtdrvGattCharacteristicUuidType = 0x2BB1;
/// Advertising Constant Tone Extension PHY
pub const BtdrvGattCharacteristicUuidType_AdvertisingConstantToneExtensionPhy: BtdrvGattCharacteristicUuidType = 0x2BB2;
/// Bearer Provider Name
pub const BtdrvGattCharacteristicUuidType_BearerProviderName: BtdrvGattCharacteristicUuidType = 0x2BB3;
/// Bearer UCI
pub const BtdrvGattCharacteristicUuidType_BearerUci: BtdrvGattCharacteristicUuidType = 0x2BB4;
/// Bearer Technology
pub const BtdrvGattCharacteristicUuidType_BearerTechnology: BtdrvGattCharacteristicUuidType = 0x2BB5;
/// Bearer URI Schemes Supported List
pub const BtdrvGattCharacteristicUuidType_BearerUriSchemesSupportedList: BtdrvGattCharacteristicUuidType = 0x2BB6;
/// Bearer Signal Strength
pub const BtdrvGattCharacteristicUuidType_BearerSignalStrength: BtdrvGattCharacteristicUuidType = 0x2BB7;
/// Bearer Signal Strength Reporting Interval
pub const BtdrvGattCharacteristicUuidType_BearerSignalStrengthReportingInterval: BtdrvGattCharacteristicUuidType = 0x2BB8;
/// Bearer List Current Calls
pub const BtdrvGattCharacteristicUuidType_BearerListCurrentCalls: BtdrvGattCharacteristicUuidType = 0x2BB9;
/// Content Control ID
pub const BtdrvGattCharacteristicUuidType_ContentControlId: BtdrvGattCharacteristicUuidType = 0x2BBA;
/// Status Flags
pub const BtdrvGattCharacteristicUuidType_StatusFlags: BtdrvGattCharacteristicUuidType = 0x2BBB;
/// Incoming Call Target Bearer URI
pub const BtdrvGattCharacteristicUuidType_IncomingCallTargetBearerUri: BtdrvGattCharacteristicUuidType = 0x2BBC;
/// Call State
pub const BtdrvGattCharacteristicUuidType_CallState: BtdrvGattCharacteristicUuidType = 0x2BBD;
/// Call Control Point
pub const BtdrvGattCharacteristicUuidType_CallControlPoint: BtdrvGattCharacteristicUuidType = 0x2BBE;
/// Call Control Point Optional Opcodes
pub const BtdrvGattCharacteristicUuidType_CallControlPointOptionalOpcodes: BtdrvGattCharacteristicUuidType = 0x2BBF;
/// Termination Reason
pub const BtdrvGattCharacteristicUuidType_TerminationReason: BtdrvGattCharacteristicUuidType = 0x2BC0;
/// Incoming Call
pub const BtdrvGattCharacteristicUuidType_IncomingCall: BtdrvGattCharacteristicUuidType = 0x2BC1;
/// Call Friendly Name
pub const BtdrvGattCharacteristicUuidType_CallFriendlyName: BtdrvGattCharacteristicUuidType = 0x2BC2;
/// Mute
pub const BtdrvGattCharacteristicUuidType_Mute: BtdrvGattCharacteristicUuidType = 0x2BC3;
/// Sink ASE
pub const BtdrvGattCharacteristicUuidType_SinkAse: BtdrvGattCharacteristicUuidType = 0x2BC4;
/// Source ASE
pub const BtdrvGattCharacteristicUuidType_SourceAse: BtdrvGattCharacteristicUuidType = 0x2BC5;
/// ASE Control Point
pub const BtdrvGattCharacteristicUuidType_AseControlPoint: BtdrvGattCharacteristicUuidType = 0x2BC6;
/// Broadcast Audio Scan Control Point
pub const BtdrvGattCharacteristicUuidType_BroadcastAudioScanControlPoint: BtdrvGattCharacteristicUuidType = 0x2BC7;
/// Broadcast Receive State
pub const BtdrvGattCharacteristicUuidType_BroadcastReceiveState: BtdrvGattCharacteristicUuidType = 0x2BC8;
/// Sink PAC
pub const BtdrvGattCharacteristicUuidType_SinkPac: BtdrvGattCharacteristicUuidType = 0x2BC9;
/// Sink Audio Locations
pub const BtdrvGattCharacteristicUuidType_SinkAudioLocations: BtdrvGattCharacteristicUuidType = 0x2BCA;
/// Source PAC
pub const BtdrvGattCharacteristicUuidType_SourcePac: BtdrvGattCharacteristicUuidType = 0x2BCB;
/// Source Audio Locations
pub const BtdrvGattCharacteristicUuidType_SourceAudioLocations: BtdrvGattCharacteristicUuidType = 0x2BCC;
/// Available Audio Contexts
pub const BtdrvGattCharacteristicUuidType_AvailableAudioContexts: BtdrvGattCharacteristicUuidType = 0x2BCD;
/// Supported Audio Contexts
pub const BtdrvGattCharacteristicUuidType_SupportedAudioContexts: BtdrvGattCharacteristicUuidType = 0x2BCE;
/// Ammonia Concentration
pub const BtdrvGattCharacteristicUuidType_AmmoniaConcentration: BtdrvGattCharacteristicUuidType = 0x2BCF;
/// Carbon Monoxide Concentration
pub const BtdrvGattCharacteristicUuidType_CarbonMonoxideConcentration: BtdrvGattCharacteristicUuidType = 0x2BD0;
/// Methane Concentration
pub const BtdrvGattCharacteristicUuidType_MethaneConcentration: BtdrvGattCharacteristicUuidType = 0x2BD1;
/// Nitrogen Dioxide Concentration
pub const BtdrvGattCharacteristicUuidType_NitrogenDioxideConcentration: BtdrvGattCharacteristicUuidType = 0x2BD2;
/// Non-Methane Volatile Organic Compounds Concentration
pub const BtdrvGattCharacteristicUuidType_NonMethaneVolatileOrganicCompoundsConcentration: BtdrvGattCharacteristicUuidType = 0x2BD3;
/// Ozone Concentration
pub const BtdrvGattCharacteristicUuidType_OzoneConcentration: BtdrvGattCharacteristicUuidType = 0x2BD4;
/// Particulate Matter - PM1 Concentration
pub const BtdrvGattCharacteristicUuidType_ParticulateMatterPm1Concentration: BtdrvGattCharacteristicUuidType = 0x2BD5;
/// Particulate Matter - PM2.5 Concentration
pub const BtdrvGattCharacteristicUuidType_ParticulateMatterPm25Concentration: BtdrvGattCharacteristicUuidType = 0x2BD6;
/// Particulate Matter - PM10 Concentration
pub const BtdrvGattCharacteristicUuidType_ParticulateMatterPm10Concentration: BtdrvGattCharacteristicUuidType = 0x2BD7;
/// Sulfur Dioxide Concentration
pub const BtdrvGattCharacteristicUuidType_SulfurDioxideConcentration: BtdrvGattCharacteristicUuidType = 0x2BD8;
/// Sulfur Hexafluoride Concentration
pub const BtdrvGattCharacteristicUuidType_SulfurHexafluorideConcentration: BtdrvGattCharacteristicUuidType = 0x2BD9;
/// Hearing Aid Features
pub const BtdrvGattCharacteristicUuidType_HearingAidFeatures: BtdrvGattCharacteristicUuidType = 0x2BDA;
/// Hearing Aid Preset Control Point
pub const BtdrvGattCharacteristicUuidType_HearingAidPresetControlPoint: BtdrvGattCharacteristicUuidType = 0x2BDB;
/// Active Preset Index
pub const BtdrvGattCharacteristicUuidType_ActivePresetIndex: BtdrvGattCharacteristicUuidType = 0x2BDC;
/// Stored Health Observations
pub const BtdrvGattCharacteristicUuidType_StoredHealthObservations: BtdrvGattCharacteristicUuidType = 0x2BDD;
/// Fixed String 64
pub const BtdrvGattCharacteristicUuidType_FixedString64: BtdrvGattCharacteristicUuidType = 0x2BDE;
/// High Temperature
pub const BtdrvGattCharacteristicUuidType_HighTemperature: BtdrvGattCharacteristicUuidType = 0x2BDF;
/// High Voltage
pub const BtdrvGattCharacteristicUuidType_HighVoltage: BtdrvGattCharacteristicUuidType = 0x2BE0;
/// Light Distribution
pub const BtdrvGattCharacteristicUuidType_LightDistribution: BtdrvGattCharacteristicUuidType = 0x2BE1;
/// Light Output
pub const BtdrvGattCharacteristicUuidType_LightOutput: BtdrvGattCharacteristicUuidType = 0x2BE2;
/// Light Source Type
pub const BtdrvGattCharacteristicUuidType_LightSourceType: BtdrvGattCharacteristicUuidType = 0x2BE3;
/// Noise
pub const BtdrvGattCharacteristicUuidType_Noise: BtdrvGattCharacteristicUuidType = 0x2BE4;
/// Relative Runtime in a Correlated Color Temperature Range
pub const BtdrvGattCharacteristicUuidType_RelativeRuntimeInACorrelatedColorTemperatureRange: BtdrvGattCharacteristicUuidType = 0x2BE5;
/// Time Second 32
pub const BtdrvGattCharacteristicUuidType_TimeSecond32: BtdrvGattCharacteristicUuidType = 0x2BE6;
/// VOC Concentration
pub const BtdrvGattCharacteristicUuidType_VocConcentration: BtdrvGattCharacteristicUuidType = 0x2BE7;
/// Voltage Frequency
pub const BtdrvGattCharacteristicUuidType_VoltageFrequency: BtdrvGattCharacteristicUuidType = 0x2BE8;
/// Battery Critical Status
pub const BtdrvGattCharacteristicUuidType_BatteryCriticalStatus: BtdrvGattCharacteristicUuidType = 0x2BE9;
/// Battery Health Status
pub const BtdrvGattCharacteristicUuidType_BatteryHealthStatus: BtdrvGattCharacteristicUuidType = 0x2BEA;
/// Battery Health Information
pub const BtdrvGattCharacteristicUuidType_BatteryHealthInformation: BtdrvGattCharacteristicUuidType = 0x2BEB;
/// Battery Information
pub const BtdrvGattCharacteristicUuidType_BatteryInformation: BtdrvGattCharacteristicUuidType = 0x2BEC;
/// Battery Level Status
pub const BtdrvGattCharacteristicUuidType_BatteryLevelStatus: BtdrvGattCharacteristicUuidType = 0x2BED;
/// Battery Time Status
pub const BtdrvGattCharacteristicUuidType_BatteryTimeStatus: BtdrvGattCharacteristicUuidType = 0x2BEE;
/// Estimated Service Date
pub const BtdrvGattCharacteristicUuidType_EstimatedServiceDate: BtdrvGattCharacteristicUuidType = 0x2BEF;
/// Battery Energy Status
pub const BtdrvGattCharacteristicUuidType_BatteryEnergyStatus: BtdrvGattCharacteristicUuidType = 0x2BF0;
/// Observation Schedule Changed
pub const BtdrvGattCharacteristicUuidType_ObservationScheduleChanged: BtdrvGattCharacteristicUuidType = 0x2BF1;
/// Current Elapsed Time
pub const BtdrvGattCharacteristicUuidType_CurrentElapsedTime: BtdrvGattCharacteristicUuidType = 0x2BF2;
/// Health Sensor Features
pub const BtdrvGattCharacteristicUuidType_HealthSensorFeatures: BtdrvGattCharacteristicUuidType = 0x2BF3;
/// GHS Control Point
pub const BtdrvGattCharacteristicUuidType_GhsControlPoint: BtdrvGattCharacteristicUuidType = 0x2BF4;
/// LE GATT Security Levels
pub const BtdrvGattCharacteristicUuidType_LeGattSecurityLevels: BtdrvGattCharacteristicUuidType = 0x2BF5;
/// ESL Address
pub const BtdrvGattCharacteristicUuidType_EslAddress: BtdrvGattCharacteristicUuidType = 0x2BF6;
/// AP Sync Key Material
pub const BtdrvGattCharacteristicUuidType_ApSyncKeyMaterial: BtdrvGattCharacteristicUuidType = 0x2BF7;
/// ESL Response Key Material
pub const BtdrvGattCharacteristicUuidType_EslResponseKeyMaterial: BtdrvGattCharacteristicUuidType = 0x2BF8;
/// ESL Current Absolute Time
pub const BtdrvGattCharacteristicUuidType_EslCurrentAbsoluteTime: BtdrvGattCharacteristicUuidType = 0x2BF9;
/// ESL Display Information
pub const BtdrvGattCharacteristicUuidType_EslDisplayInformation: BtdrvGattCharacteristicUuidType = 0x2BFA;
/// ESL Image Information
pub const BtdrvGattCharacteristicUuidType_EslImageInformation: BtdrvGattCharacteristicUuidType = 0x2BFB;
/// ESL Sensor Information
pub const BtdrvGattCharacteristicUuidType_EslSensorInformation: BtdrvGattCharacteristicUuidType = 0x2BFC;
/// ESL LED Information
pub const BtdrvGattCharacteristicUuidType_EslLedInformation: BtdrvGattCharacteristicUuidType = 0x2BFD;
/// ESL Control Point
pub const BtdrvGattCharacteristicUuidType_EslControlPoint: BtdrvGattCharacteristicUuidType = 0x2BFE;
/// UDI for Medical Devices
pub const BtdrvGattCharacteristicUuidType_UdiForMedicalDevices: BtdrvGattCharacteristicUuidType = 0x2BFF;

/// GattDescriptorUuidType
pub type BtdrvGattDescriptorUuidType = u32;
/// Characteristic Extended Properties
pub const BtdrvGattDescriptorUuidType_CharacteristicExtendedProperties: BtdrvGattDescriptorUuidType = 0x2900;
/// Characteristic User Description
pub const BtdrvGattDescriptorUuidType_CharacteristicUserDescription: BtdrvGattDescriptorUuidType = 0x2901;
/// Client Characteristic Configuration
pub const BtdrvGattDescriptorUuidType_ClientCharacteristicConfiguration: BtdrvGattDescriptorUuidType = 0x2902;
/// Server Characteristic Configuration
pub const BtdrvGattDescriptorUuidType_ServerCharacteristicConfiguration: BtdrvGattDescriptorUuidType = 0x2903;
/// Characteristic Presentation Format
pub const BtdrvGattDescriptorUuidType_CharacteristicPresentationFormat: BtdrvGattDescriptorUuidType = 0x2904;
/// Characteristic Aggregate Format
pub const BtdrvGattDescriptorUuidType_CharacteristicAggregateFormat: BtdrvGattDescriptorUuidType = 0x2905;
/// Valid Range
pub const BtdrvGattDescriptorUuidType_ValidRange: BtdrvGattDescriptorUuidType = 0x2906;
/// External Report Reference
pub const BtdrvGattDescriptorUuidType_ExternalReportReference: BtdrvGattDescriptorUuidType = 0x2907;
/// Report Reference
pub const BtdrvGattDescriptorUuidType_ReportReference: BtdrvGattDescriptorUuidType = 0x2908;
/// Number of Digitals
pub const BtdrvGattDescriptorUuidType_NumberOfDigitals: BtdrvGattDescriptorUuidType = 0x2909;
/// Value Trigger Setting
pub const BtdrvGattDescriptorUuidType_ValueTriggerSetting: BtdrvGattDescriptorUuidType = 0x290A;
/// Environmental Sensing Configuration
pub const BtdrvGattDescriptorUuidType_EnvironmentalSensingConfiguration: BtdrvGattDescriptorUuidType = 0x290B;
/// Environmental Sensing Measurement
pub const BtdrvGattDescriptorUuidType_EnvironmentalSensingMeasurement: BtdrvGattDescriptorUuidType = 0x290C;
/// Environmental Sensing Trigger Setting
pub const BtdrvGattDescriptorUuidType_EnvironmentalSensingTriggerSetting: BtdrvGattDescriptorUuidType = 0x290D;
/// Time Trigger Setting
pub const BtdrvGattDescriptorUuidType_TimeTriggerSetting: BtdrvGattDescriptorUuidType = 0x290E;
/// Complete BR-EDR Transport Block Data
pub const BtdrvGattDescriptorUuidType_CompleteBrEdrTransportBlockData: BtdrvGattDescriptorUuidType = 0x290F;
/// Observation Schedule
pub const BtdrvGattDescriptorUuidType_ObservationSchedule: BtdrvGattDescriptorUuidType = 0x2910;
/// Valid Range and Accuracy
pub const BtdrvGattDescriptorUuidType_ValidRangeAndAccuracy: BtdrvGattDescriptorUuidType = 0x2911;

/// Extracts the category from a Bluetooth appearance value.
#[inline(always)]
pub const fn APPEARANCE_CATEGORY(x: u32) -> u32 {
    (x & 0xFFC0) >> 6
}
