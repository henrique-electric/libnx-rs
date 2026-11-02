//! Switch result code tools. (`switch/result.h`)

use crate::types::Result;

/// Checks whether a result code indicates success.
#[inline(always)]
pub const fn R_SUCCEEDED(res: Result) -> bool {
    res == 0
}

/// Checks whether a result code indicates failure.
#[inline(always)]
pub const fn R_FAILED(res: Result) -> bool {
    res != 0
}

/// Returns the module ID of a result code.
#[inline(always)]
pub const fn R_MODULE(res: Result) -> u32 {
    res & 0x1FF
}

/// Returns the description of a result code.
#[inline(always)]
pub const fn R_DESCRIPTION(res: Result) -> u32 {
    (res >> 9) & 0x1FFF
}

/// Masks out unused bits in a result code, retrieving the actual value for use in comparisons.
#[inline(always)]
pub const fn R_VALUE(res: Result) -> u32 {
    res & 0x3FFFFF
}

/// Builds a result code from its constituent components.
#[inline(always)]
pub const fn MAKERESULT(module: u32, description: u32) -> Result {
    (module & 0x1FF) | ((description & 0x1FFF) << 9)
}

/// Builds a kernel error result code.
///
/// Unlike the C macro (which token-pastes), this takes the `KernelError_*` value itself,
/// e.g. `KERNELRESULT(KernelError_TimedOut)`.
#[inline(always)]
pub const fn KERNELRESULT(description: u32) -> Result {
    MAKERESULT(Module_Kernel, description)
}

// Module values
pub const Module_Kernel: u32 = 1;
pub const Module_Libnx: u32 = 345;
pub const Module_HomebrewAbi: u32 = 346;
pub const Module_HomebrewLoader: u32 = 347;
pub const Module_LibnxNvidia: u32 = 348;
pub const Module_LibnxBinder: u32 = 349;

// Kernel error codes
pub const KernelError_OutOfSessions: u32 = 7;
pub const KernelError_InvalidCapabilityDescriptor: u32 = 14;
pub const KernelError_NotImplemented: u32 = 33;
pub const KernelError_ThreadTerminating: u32 = 59;
pub const KernelError_OutOfDebugEvents: u32 = 70;
pub const KernelError_InvalidSize: u32 = 101;
pub const KernelError_InvalidAddress: u32 = 102;
pub const KernelError_ResourceExhausted: u32 = 103;
pub const KernelError_OutOfMemory: u32 = 104;
pub const KernelError_OutOfHandles: u32 = 105;
pub const KernelError_InvalidMemoryState: u32 = 106;
pub const KernelError_InvalidMemoryPermissions: u32 = 108;
pub const KernelError_InvalidMemoryRange: u32 = 110;
pub const KernelError_InvalidPriority: u32 = 112;
pub const KernelError_InvalidCoreId: u32 = 113;
pub const KernelError_InvalidHandle: u32 = 114;
pub const KernelError_InvalidUserBuffer: u32 = 115;
pub const KernelError_InvalidCombination: u32 = 116;
pub const KernelError_TimedOut: u32 = 117;
pub const KernelError_Cancelled: u32 = 118;
pub const KernelError_OutOfRange: u32 = 119;
pub const KernelError_InvalidEnumValue: u32 = 120;
pub const KernelError_NotFound: u32 = 121;
pub const KernelError_AlreadyExists: u32 = 122;
pub const KernelError_ConnectionClosed: u32 = 123;
pub const KernelError_UnhandledUserInterrupt: u32 = 124;
pub const KernelError_InvalidState: u32 = 125;
pub const KernelError_ReservedValue: u32 = 126;
pub const KernelError_InvalidHwBreakpoint: u32 = 127;
pub const KernelError_FatalUserException: u32 = 128;
pub const KernelError_OwnedByAnotherProcess: u32 = 129;
pub const KernelError_ConnectionRefused: u32 = 131;
pub const KernelError_OutOfResource: u32 = 132;
pub const KernelError_IpcMapFailed: u32 = 259;
pub const KernelError_IpcCmdbufTooSmall: u32 = 260;
pub const KernelError_NotDebugged: u32 = 520;

// libnx error codes
pub const LibnxError_BadReloc: u32 = 1;
pub const LibnxError_OutOfMemory: u32 = 2;
pub const LibnxError_AlreadyMapped: u32 = 3;
pub const LibnxError_BadGetInfo_Stack: u32 = 4;
pub const LibnxError_BadGetInfo_Heap: u32 = 5;
pub const LibnxError_BadQueryMemory: u32 = 6;
pub const LibnxError_AlreadyInitialized: u32 = 7;
pub const LibnxError_NotInitialized: u32 = 8;
pub const LibnxError_NotFound: u32 = 9;
pub const LibnxError_IoError: u32 = 10;
pub const LibnxError_BadInput: u32 = 11;
pub const LibnxError_BadReent: u32 = 12;
pub const LibnxError_BufferProducerError: u32 = 13;
pub const LibnxError_HandleTooEarly: u32 = 14;
pub const LibnxError_HeapAllocFailed: u32 = 15;
pub const LibnxError_TooManyOverrides: u32 = 16;
pub const LibnxError_ParcelError: u32 = 17;
pub const LibnxError_BadGfxInit: u32 = 18;
pub const LibnxError_BadGfxEventWait: u32 = 19;
pub const LibnxError_BadGfxQueueBuffer: u32 = 20;
pub const LibnxError_BadGfxDequeueBuffer: u32 = 21;
pub const LibnxError_AppletCmdidNotFound: u32 = 22;
pub const LibnxError_BadAppletReceiveMessage: u32 = 23;
pub const LibnxError_BadAppletNotifyRunning: u32 = 24;
pub const LibnxError_BadAppletGetCurrentFocusState: u32 = 25;
pub const LibnxError_BadAppletGetOperationMode: u32 = 26;
pub const LibnxError_BadAppletGetPerformanceMode: u32 = 27;
pub const LibnxError_BadUsbCommsRead: u32 = 28;
pub const LibnxError_BadUsbCommsWrite: u32 = 29;
pub const LibnxError_InitFail_SM: u32 = 30;
pub const LibnxError_InitFail_AM: u32 = 31;
pub const LibnxError_InitFail_HID: u32 = 32;
pub const LibnxError_InitFail_FS: u32 = 33;
pub const LibnxError_BadGetInfo_Rng: u32 = 34;
pub const LibnxError_JitUnavailable: u32 = 35;
pub const LibnxError_WeirdKernel: u32 = 36;
pub const LibnxError_IncompatSysVer: u32 = 37;
pub const LibnxError_InitFail_Time: u32 = 38;
pub const LibnxError_TooManyDevOpTabs: u32 = 39;
pub const LibnxError_DomainMessageUnknownType: u32 = 40;
pub const LibnxError_DomainMessageTooManyObjectIds: u32 = 41;
pub const LibnxError_AppletFailedToInitialize: u32 = 42;
pub const LibnxError_ApmFailedToInitialize: u32 = 43;
pub const LibnxError_NvinfoFailedToInitialize: u32 = 44;
pub const LibnxError_NvbufFailedToInitialize: u32 = 45;
pub const LibnxError_LibAppletBadExit: u32 = 46;
pub const LibnxError_InvalidCmifOutHeader: u32 = 47;
pub const LibnxError_ShouldNotHappen: u32 = 48;
pub const LibnxError_Timeout: u32 = 49;

// libnx binder error codes
pub const LibnxBinderError_Unknown: u32 = 1;
pub const LibnxBinderError_NoMemory: u32 = 2;
pub const LibnxBinderError_InvalidOperation: u32 = 3;
pub const LibnxBinderError_BadValue: u32 = 4;
pub const LibnxBinderError_BadType: u32 = 5;
pub const LibnxBinderError_NameNotFound: u32 = 6;
pub const LibnxBinderError_PermissionDenied: u32 = 7;
pub const LibnxBinderError_NoInit: u32 = 8;
pub const LibnxBinderError_AlreadyExists: u32 = 9;
pub const LibnxBinderError_DeadObject: u32 = 10;
pub const LibnxBinderError_FailedTransaction: u32 = 11;
pub const LibnxBinderError_BadIndex: u32 = 12;
pub const LibnxBinderError_NotEnoughData: u32 = 13;
pub const LibnxBinderError_WouldBlock: u32 = 14;
pub const LibnxBinderError_TimedOut: u32 = 15;
pub const LibnxBinderError_UnknownTransaction: u32 = 16;
pub const LibnxBinderError_FdsNotAllowed: u32 = 17;

// libnx nvidia error codes
pub const LibnxNvidiaError_Unknown: u32 = 1;
/// Maps to Nvidia: 1
pub const LibnxNvidiaError_NotImplemented: u32 = 2;
/// Maps to Nvidia: 2
pub const LibnxNvidiaError_NotSupported: u32 = 3;
/// Maps to Nvidia: 3
pub const LibnxNvidiaError_NotInitialized: u32 = 4;
/// Maps to Nvidia: 4
pub const LibnxNvidiaError_BadParameter: u32 = 5;
/// Maps to Nvidia: 5
pub const LibnxNvidiaError_Timeout: u32 = 6;
/// Maps to Nvidia: 6
pub const LibnxNvidiaError_InsufficientMemory: u32 = 7;
/// Maps to Nvidia: 7
pub const LibnxNvidiaError_ReadOnlyAttribute: u32 = 8;
/// Maps to Nvidia: 8
pub const LibnxNvidiaError_InvalidState: u32 = 9;
/// Maps to Nvidia: 9
pub const LibnxNvidiaError_InvalidAddress: u32 = 10;
/// Maps to Nvidia: 10
pub const LibnxNvidiaError_InvalidSize: u32 = 11;
/// Maps to Nvidia: 11
pub const LibnxNvidiaError_BadValue: u32 = 12;
/// Maps to Nvidia: 13
pub const LibnxNvidiaError_AlreadyAllocated: u32 = 13;
/// Maps to Nvidia: 14
pub const LibnxNvidiaError_Busy: u32 = 14;
/// Maps to Nvidia: 15
pub const LibnxNvidiaError_ResourceError: u32 = 15;
/// Maps to Nvidia: 16
pub const LibnxNvidiaError_CountMismatch: u32 = 16;
/// Maps to Nvidia: 0x1000
pub const LibnxNvidiaError_SharedMemoryTooSmall: u32 = 17;
/// Maps to Nvidia: 0x30003
pub const LibnxNvidiaError_FileOperationFailed: u32 = 18;
/// Maps to Nvidia: 0x3000F
pub const LibnxNvidiaError_IoctlFailed: u32 = 19;
