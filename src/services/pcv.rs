//! `switch/services/pcv.h`

#![allow(unused_imports, unused_parens)]

use core::ffi::*;

#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use crate::bsd::*;
#[allow(unused_imports)]
use crate::newlib::*;

pub type PcvModule = u32;
pub const PcvModule_CpuBus: PcvModule = 0;
pub const PcvModule_GPU: PcvModule = 1;
pub const PcvModule_I2S1: PcvModule = 2;
pub const PcvModule_I2S2: PcvModule = 3;
pub const PcvModule_I2S3: PcvModule = 4;
pub const PcvModule_PWM: PcvModule = 5;
pub const PcvModule_I2C1: PcvModule = 6;
pub const PcvModule_I2C2: PcvModule = 7;
pub const PcvModule_I2C3: PcvModule = 8;
pub const PcvModule_I2C4: PcvModule = 9;
pub const PcvModule_I2C5: PcvModule = 10;
pub const PcvModule_I2C6: PcvModule = 11;
pub const PcvModule_SPI1: PcvModule = 12;
pub const PcvModule_SPI2: PcvModule = 13;
pub const PcvModule_SPI3: PcvModule = 14;
pub const PcvModule_SPI4: PcvModule = 15;
pub const PcvModule_DISP1: PcvModule = 16;
pub const PcvModule_DISP2: PcvModule = 17;
pub const PcvModule_ISP: PcvModule = 18;
pub const PcvModule_VI: PcvModule = 19;
pub const PcvModule_SDMMC1: PcvModule = 20;
pub const PcvModule_SDMMC2: PcvModule = 21;
pub const PcvModule_SDMMC3: PcvModule = 22;
pub const PcvModule_SDMMC4: PcvModule = 23;
pub const PcvModule_OWR: PcvModule = 24;
pub const PcvModule_CSITE: PcvModule = 25;
pub const PcvModule_TSEC: PcvModule = 26;
pub const PcvModule_MSELECT: PcvModule = 27;
pub const PcvModule_HDA2CODEC_2X: PcvModule = 28;
pub const PcvModule_ACTMON: PcvModule = 29;
pub const PcvModule_I2C_SLOW: PcvModule = 30;
pub const PcvModule_SOR1: PcvModule = 31;
pub const PcvModule_SATA: PcvModule = 32;
pub const PcvModule_HDA: PcvModule = 33;
pub const PcvModule_XUSB_CORE_HOST: PcvModule = 34;
pub const PcvModule_XUSB_FALCON: PcvModule = 35;
pub const PcvModule_XUSB_FS: PcvModule = 36;
pub const PcvModule_XUSB_CORE_DEV: PcvModule = 37;
pub const PcvModule_XUSB_SS_HOSTDEV: PcvModule = 38;
pub const PcvModule_UARTA: PcvModule = 39;
pub const PcvModule_UARTB: PcvModule = 40;
pub const PcvModule_UARTC: PcvModule = 41;
pub const PcvModule_UARTD: PcvModule = 42;
pub const PcvModule_HOST1X: PcvModule = 43;
pub const PcvModule_ENTROPY: PcvModule = 44;
pub const PcvModule_SOC_THERM: PcvModule = 45;
pub const PcvModule_VIC: PcvModule = 46;
pub const PcvModule_NVENC: PcvModule = 47;
pub const PcvModule_NVJPG: PcvModule = 48;
pub const PcvModule_NVDEC: PcvModule = 49;
pub const PcvModule_QSPI: PcvModule = 50;
pub const PcvModule_VI_I2C: PcvModule = 51;
pub const PcvModule_TSECB: PcvModule = 52;
pub const PcvModule_APE: PcvModule = 53;
pub const PcvModule_ACLK: PcvModule = 54;
pub const PcvModule_UARTAPE: PcvModule = 55;
pub const PcvModule_EMC: PcvModule = 56;
pub const PcvModule_PLLE0_0: PcvModule = 57;
pub const PcvModule_PLLE0_1: PcvModule = 58;
pub const PcvModule_DSI: PcvModule = 59;
pub const PcvModule_MAUD: PcvModule = 60;
pub const PcvModule_DPAUX1: PcvModule = 61;
pub const PcvModule_MIPI_CAL: PcvModule = 62;
pub const PcvModule_UART_FST_MIPI_CAL: PcvModule = 63;
pub const PcvModule_OSC: PcvModule = 64;
pub const PcvModule_SCLK: PcvModule = 65;
pub const PcvModule_SOR_SAFE: PcvModule = 66;
pub const PcvModule_XUSB_SS: PcvModule = 67;
pub const PcvModule_XUSB_HOST: PcvModule = 68;
pub const PcvModule_XUSB_DEV: PcvModule = 69;
pub const PcvModule_EXTPERIPH1: PcvModule = 70;
pub const PcvModule_AHUB: PcvModule = 71;
pub const PcvModule_HDA2HDMICODEC: PcvModule = 72;
pub const PcvModule_PLLP5: PcvModule = 73;
pub const PcvModule_USBD: PcvModule = 74;
pub const PcvModule_USB2: PcvModule = 75;
pub const PcvModule_PCIE: PcvModule = 76;
pub const PcvModule_AFI: PcvModule = 77;
pub const PcvModule_PCIEXCLK: PcvModule = 78;
pub const PcvModule_PEX_USB_UPHY: PcvModule = 79;
pub const PcvModule_XUSB_PADCTL: PcvModule = 80;
pub const PcvModule_APBDMA: PcvModule = 81;
pub const PcvModule_USB2_TRK: PcvModule = 82;
pub const PcvModule_PLLE0_2: PcvModule = 83;
pub const PcvModule_PLLE0_3: PcvModule = 84;
pub const PcvModule_CEC: PcvModule = 85;
pub const PcvModule_EXTPERIPH2: PcvModule = 86;
pub const PcvModule_Count: PcvModule = 87;

/// Module id returned by [8.0.0+] pcv services
/// See also: <https://switchbrew.org/wiki/PCV_services#Modules>
pub type PcvModuleId = u32;
pub const PcvModuleId_CpuBus: PcvModuleId = 0x40000001;
pub const PcvModuleId_GPU: PcvModuleId = 0x40000002;
pub const PcvModuleId_I2S1: PcvModuleId = 0x40000003;
pub const PcvModuleId_I2S2: PcvModuleId = 0x40000004;
pub const PcvModuleId_I2S3: PcvModuleId = 0x40000005;
pub const PcvModuleId_PWM: PcvModuleId = 0x40000006;
pub const PcvModuleId_I2C1: PcvModuleId = 0x02000001;
pub const PcvModuleId_I2C2: PcvModuleId = 0x02000002;
pub const PcvModuleId_I2C3: PcvModuleId = 0x02000003;
pub const PcvModuleId_I2C4: PcvModuleId = 0x02000004;
pub const PcvModuleId_I2C5: PcvModuleId = 0x02000005;
pub const PcvModuleId_I2C6: PcvModuleId = 0x02000006;
pub const PcvModuleId_SPI1: PcvModuleId = 0x07000000;
pub const PcvModuleId_SPI2: PcvModuleId = 0x07000001;
pub const PcvModuleId_SPI3: PcvModuleId = 0x07000002;
pub const PcvModuleId_SPI4: PcvModuleId = 0x07000003;
pub const PcvModuleId_DISP1: PcvModuleId = 0x40000011;
pub const PcvModuleId_DISP2: PcvModuleId = 0x40000012;
pub const PcvModuleId_ISP: PcvModuleId = 0x40000013;
pub const PcvModuleId_VI: PcvModuleId = 0x40000014;
pub const PcvModuleId_SDMMC1: PcvModuleId = 0x40000015;
pub const PcvModuleId_SDMMC2: PcvModuleId = 0x40000016;
pub const PcvModuleId_SDMMC3: PcvModuleId = 0x40000017;
pub const PcvModuleId_SDMMC4: PcvModuleId = 0x40000018;
pub const PcvModuleId_OWR: PcvModuleId = 0x40000019;
pub const PcvModuleId_CSITE: PcvModuleId = 0x4000001A;
pub const PcvModuleId_TSEC: PcvModuleId = 0x4000001B;
pub const PcvModuleId_MSELECT: PcvModuleId = 0x4000001C;
pub const PcvModuleId_HDA2CODEC_2X: PcvModuleId = 0x4000001D;
pub const PcvModuleId_ACTMON: PcvModuleId = 0x4000001E;
pub const PcvModuleId_I2C_SLOW: PcvModuleId = 0x4000001F;
pub const PcvModuleId_SOR1: PcvModuleId = 0x40000020;
pub const PcvModuleId_SATA: PcvModuleId = 0x40000021;
pub const PcvModuleId_HDA: PcvModuleId = 0x40000022;
pub const PcvModuleId_XUSB_CORE_HOST: PcvModuleId = 0x40000023;
pub const PcvModuleId_XUSB_FALCON: PcvModuleId = 0x40000024;
pub const PcvModuleId_XUSB_FS: PcvModuleId = 0x40000025;
pub const PcvModuleId_XUSB_CORE_DEV: PcvModuleId = 0x40000026;
pub const PcvModuleId_XUSB_SS_HOSTDEV: PcvModuleId = 0x40000027;
pub const PcvModuleId_UARTA: PcvModuleId = 0x03000001;
pub const PcvModuleId_UARTB: PcvModuleId = 0x35000405;
pub const PcvModuleId_UARTC: PcvModuleId = 0x3500040F;
pub const PcvModuleId_UARTD: PcvModuleId = 0x37000001;
pub const PcvModuleId_HOST1X: PcvModuleId = 0x4000002C;
pub const PcvModuleId_ENTROPY: PcvModuleId = 0x4000002D;
pub const PcvModuleId_SOC_THERM: PcvModuleId = 0x4000002E;
pub const PcvModuleId_VIC: PcvModuleId = 0x4000002F;
pub const PcvModuleId_NVENC: PcvModuleId = 0x40000030;
pub const PcvModuleId_NVJPG: PcvModuleId = 0x40000031;
pub const PcvModuleId_NVDEC: PcvModuleId = 0x40000032;
pub const PcvModuleId_QSPI: PcvModuleId = 0x40000033;
pub const PcvModuleId_VI_I2C: PcvModuleId = 0x40000034;
pub const PcvModuleId_TSECB: PcvModuleId = 0x40000035;
pub const PcvModuleId_APE: PcvModuleId = 0x40000036;
pub const PcvModuleId_ACLK: PcvModuleId = 0x40000037;
pub const PcvModuleId_UARTAPE: PcvModuleId = 0x40000038;
pub const PcvModuleId_EMC: PcvModuleId = 0x40000039;
pub const PcvModuleId_PLLE0_0: PcvModuleId = 0x4000003A;
pub const PcvModuleId_PLLE0_1: PcvModuleId = 0x4000003B;
pub const PcvModuleId_DSI: PcvModuleId = 0x4000003C;
pub const PcvModuleId_MAUD: PcvModuleId = 0x4000003D;
pub const PcvModuleId_DPAUX1: PcvModuleId = 0x4000003E;
pub const PcvModuleId_MIPI_CAL: PcvModuleId = 0x4000003F;
pub const PcvModuleId_UART_FST_MIPI_CAL: PcvModuleId = 0x40000040;
pub const PcvModuleId_OSC: PcvModuleId = 0x40000041;
pub const PcvModuleId_SCLK: PcvModuleId = 0x40000042;
pub const PcvModuleId_SOR_SAFE: PcvModuleId = 0x40000043;
pub const PcvModuleId_XUSB_SS: PcvModuleId = 0x40000044;
pub const PcvModuleId_XUSB_HOST: PcvModuleId = 0x40000045;
pub const PcvModuleId_XUSB_DEV: PcvModuleId = 0x40000046;
pub const PcvModuleId_EXTPERIPH1: PcvModuleId = 0x40000047;
pub const PcvModuleId_AHUB: PcvModuleId = 0x40000048;
pub const PcvModuleId_HDA2HDMICODEC: PcvModuleId = 0x40000049;
pub const PcvModuleId_PLLP5: PcvModuleId = 0x4000004A;
pub const PcvModuleId_USBD: PcvModuleId = 0x4000004B;
pub const PcvModuleId_USB2: PcvModuleId = 0x4000004C;
pub const PcvModuleId_PCIE: PcvModuleId = 0x4000004D;
pub const PcvModuleId_AFI: PcvModuleId = 0x4000004E;
pub const PcvModuleId_PCIEXCLK: PcvModuleId = 0x4000004F;
pub const PcvModuleId_PEX_USB_UPHY: PcvModuleId = 0x40000050;
pub const PcvModuleId_XUSB_PADCTL: PcvModuleId = 0x40000051;
pub const PcvModuleId_APBDMA: PcvModuleId = 0x40000052;
pub const PcvModuleId_USB2_TRK: PcvModuleId = 0x40000053;
pub const PcvModuleId_PLLE0_2: PcvModuleId = 0x40000054;
pub const PcvModuleId_PLLE0_3: PcvModuleId = 0x40000055;
pub const PcvModuleId_CEC: PcvModuleId = 0x40000056;
pub const PcvModuleId_EXTPERIPH2: PcvModuleId = 0x40000057;

pub type PcvClockRatesListType = u32;
pub const PcvClockRatesListType_Invalid: PcvClockRatesListType = 0;
pub const PcvClockRatesListType_Discrete: PcvClockRatesListType = 1;
pub const PcvClockRatesListType_Range: PcvClockRatesListType = 2;

unsafe extern "C" {
    /// Initialize pcv.
    pub fn pcvInitialize() -> Result;

    /// Exit pcv.
    pub fn pcvExit();

    /// Gets the Service object for the actual pcv service session.
    pub fn pcvGetServiceSession() -> *mut Service;

    pub fn pcvGetModuleId(module_id: *mut PcvModuleId, module: PcvModule) -> Result;

    /// Only available on [1.0.0-7.0.1].
    pub fn pcvGetClockRate(module: PcvModule, out_hz: *mut u32) -> Result;

    /// Only available on [1.0.0-7.0.1].
    pub fn pcvSetClockRate(module: PcvModule, hz: u32) -> Result;

    /// Only available on [1.0.0-7.0.1].
    pub fn pcvSetVoltageEnabled(power_domain: u32, state: bool) -> Result;

    /// Only available on [1.0.0-7.0.1].
    pub fn pcvGetVoltageEnabled(isEnabled: *mut bool, power_domain: u32) -> Result;

    /// Only available on [1.0.0-7.0.1].
    pub fn pcvGetPossibleClockRates(module: PcvModule, rates: *mut u32, max_count: s32, out_type: *mut PcvClockRatesListType, out_count: *mut s32) -> Result;
}
