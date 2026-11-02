//! Checks for the hand-ported `static inline` functions and function-like macros
//! (`tools/inline_ports/`). Where possible, results are compared with constants that clang
//! evaluated from the original C macros.

use core::mem::size_of;

use libnx::bsd::*;
use libnx::*;

#[test]
fn ioctl_encoders_match_c_constants() {
    // sys/sockio.h, sys/filio.h: SIOCGIFADDR = _IOWR('i', 33, struct ifreq), etc.
    // (The C constants are `int`; clang-evaluated values come out as u32/i32 by sign.)
    assert_eq!(_IOWR(b'i' as u32, 33, size_of::<ifreq>()) as i64, SIOCGIFADDR as i64);
    assert_eq!(_IOW(b'i' as u32, 16, size_of::<ifreq>()) as i64, SIOCSIFFLAGS as i64);
    assert_eq!(_IOR(b'f' as u32, 127, size_of::<i32>()) as i64, FIONREAD as i64);
    assert_eq!(_IOW(b'f' as u32, 126, size_of::<i32>()) as i64, FIONBIO as i64);
    assert_eq!(IOCGROUP(SIOCGIFADDR as u32), b'i' as u32);
    assert_eq!(IOCPARM_LEN(SIOCGIFADDR as u32) as usize, size_of::<ifreq>());
}

#[test]
fn nv_ioctl_roundtrip() {
    let nr = _NV_IOWR(b'H' as u32, 0x12, 24);
    assert_eq!(_NV_IOC_DIR(nr), _NV_IOC_READ | _NV_IOC_WRITE);
    assert_eq!(_NV_IOC_TYPE(nr), b'H' as u32);
    assert_eq!(_NV_IOC_NR(nr), 0x12);
    assert_eq!(_NV_IOC_SIZE(nr), 24);
}

#[test]
fn crc_check_values() {
    // Standard check values for "123456789".
    assert_eq!(crc32Calculate(b"123456789"), 0xCBF43926);
    assert_eq!(crc32cCalculate(b"123456789"), 0xE3069283);
    // Seed chaining, as documented by libnx.
    let data = b"The quick brown fox jumps over the lazy dog";
    let (a, b) = data.split_at(17);
    assert_eq!(crc32CalculateWithSeed(crc32Calculate(a), b), crc32Calculate(data));
    assert_eq!(crc32cCalculateWithSeed(crc32cCalculate(a), b), crc32cCalculate(data));
}

#[test]
fn byte_order_and_versions() {
    assert_eq!(htonl(0x12345678), 0x78563412);
    assert_eq!(ntohs(htons(0xBEEF)), 0xBEEF);
    let v = MAKEHOSVERSION(17, 2, 1);
    assert_eq!(v, 0x110201);
    assert_eq!((HOSVER_MAJOR(v), HOSVER_MINOR(v), HOSVER_MICRO(v)), (17, 2, 1));
}

#[test]
fn colors_and_ids() {
    assert_eq!(RGBA8(0x11, 0x22, 0x33, 0x44), 0x44332211);
    assert_eq!(BGRA8_MAXALPHA(0x11, 0x22, 0x33), 0xFF112233);
    assert_eq!(RGB565_FROM_RGB8(0xFF, 0xFF, 0xFF), 0xFFFF);
    assert_eq!(AUDREN_NODEID(2, 0x123, 0x4567), 0x2123_4567);
    assert_eq!(NV_EVENT_ID_CTRL__SYNCPT(3, 5), (1 << 28) | (5 << 16) | 3);
}

#[test]
fn error_codes() {
    let ec = errorCodeCreateResult(MAKERESULT(2, 3));
    assert_eq!((ec.low, ec.desc), (2002, 3));
    assert!(errorCodeIsValid(ec));
    assert!(!errorCodeIsValid(errorCodeCreateInvalid()));
}

#[test]
fn sockets_and_inet() {
    // cmsghdr is 12 bytes, aligned up to 16 (sizeof(long) alignment).
    assert_eq!(CMSG_LEN(4), 16 + 4);
    assert_eq!(CMSG_SPACE(4), 16 + 8);
    assert!(IN_CLASSA(0x0A000001) && IN_PRIVATE(0x0A000001) && IN_PRIVATE(0xC0A80101));
    assert!(IN_LOOPBACK(0x7F000001) && IN_MULTICAST(0xE0000001));

    let mut a = in6_addr::default();
    a.anon0.s6_addr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    assert!(IN6_IS_ADDR_LOOPBACK(&a) && !IN6_IS_ADDR_UNSPECIFIED(&a));
    a.anon0.s6_addr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 10, 0, 0, 1];
    assert!(IN6_IS_ADDR_V4MAPPED(&a));
    a.anon0.s6_addr = [0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    assert!(IN6_IS_ADDR_MULTICAST(&a) && IN6_IS_ADDR_MC_LINKLOCAL(&a) && !IN6_IS_ADDR_MC_GLOBAL(&a));
    a.anon0.s6_addr = [0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    assert!(IN6_IS_ADDR_LINKLOCAL(&a));

    let insn = BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, 0x800, 0, 1);
    assert_eq!((BPF_CLASS(insn.code as u32), BPF_OP(insn.code as u32)), (BPF_JMP, BPF_JEQ));
}

#[test]
fn struct_accessors() {
    let pad = PadState { active_id_mask: 0b101, buttons_old: 0b0110, buttons_cur: 0b0011, ..Default::default() };
    assert!(padIsNpadActive(&pad, HidNpadIdType_No1) && !padIsNpadActive(&pad, HidNpadIdType_No2));
    assert!(!padIsNpadActive(&pad, HidNpadIdType_Handheld));
    assert_eq!((padGetButtonsDown(&pad), padGetButtonsUp(&pad)), (0b0001, 0b0100));

    let mut c = SwkbdConfig::default();
    swkbdConfigSetTextGrouping(&mut c, 7, 42);
    swkbdConfigSetTextGrouping(&mut c, 8, 1); // out of range: ignored, like the C bounds check
    assert_eq!(c.arg.textGrouping[7], 42);

    let w = waiterForHandle(0x1234);
    assert_eq!((w.type_, unsafe { w.anon0.handle }), (WaiterType_Handle, 0x1234));

    let mut kb = HidKeyboardState::default();
    kb.keys[1] = 1 << 3;
    assert!(hidKeyboardStateGetKey(&kb, 67) && !hidKeyboardStateGetKey(&kb, 66));
}
