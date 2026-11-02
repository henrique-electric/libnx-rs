//! Wrappers for kernel syscalls. (`switch/kernel/svc.h`)

use core::ffi::{c_char, c_int, c_void};

use crate::arm::thread_context::ThreadContext;
use crate::types::*;

/// Pseudo handle for the current process.
pub const CUR_PROCESS_HANDLE: Handle = 0xFFFF8001;
/// Pseudo handle for the current thread.
pub const CUR_THREAD_HANDLE: Handle = 0xFFFF8000;
/// Maximum number of objects that can be waited on by [`svcWaitSynchronization`] (Horizon kernel limitation).
pub const MAX_WAIT_OBJECTS: usize = 0x40;

/// Memory type enumeration (lower 8 bits of [`MemoryState`])
pub type MemoryType = u32;
/// Unmapped memory.
pub const MemType_Unmapped: MemoryType = 0x00;
/// Mapped by kernel capability parsing in [`svcCreateProcess`].
pub const MemType_Io: MemoryType = 0x01;
/// Mapped by kernel capability parsing in [`svcCreateProcess`].
pub const MemType_Normal: MemoryType = 0x02;
/// Mapped during [`svcCreateProcess`].
pub const MemType_CodeStatic: MemoryType = 0x03;
/// Transition from MemType_CodeStatic performed by [`svcSetProcessMemoryPermission`].
pub const MemType_CodeMutable: MemoryType = 0x04;
/// Mapped using [`svcSetHeapSize`].
pub const MemType_Heap: MemoryType = 0x05;
/// Mapped using [`svcMapSharedMemory`].
pub const MemType_SharedMem: MemoryType = 0x06;
/// Mapped using [`svcMapMemory`].
pub const MemType_WeirdMappedMem: MemoryType = 0x07;
/// Mapped using [`svcMapProcessCodeMemory`].
pub const MemType_ModuleCodeStatic: MemoryType = 0x08;
/// Transition from [`MemType_ModuleCodeStatic`] performed by [`svcSetProcessMemoryPermission`].
pub const MemType_ModuleCodeMutable: MemoryType = 0x09;
/// IPC buffers with descriptor flags=0.
pub const MemType_IpcBuffer0: MemoryType = 0x0A;
/// Mapped using [`svcMapMemory`].
pub const MemType_MappedMemory: MemoryType = 0x0B;
/// Mapped during [`svcCreateThread`].
pub const MemType_ThreadLocal: MemoryType = 0x0C;
/// Mapped using [`svcMapTransferMemory`] when the owning process has perm=0.
pub const MemType_TransferMemIsolated: MemoryType = 0x0D;
/// Mapped using [`svcMapTransferMemory`] when the owning process has perm!=0.
pub const MemType_TransferMem: MemoryType = 0x0E;
/// Mapped using [`svcMapProcessMemory`].
pub const MemType_ProcessMem: MemoryType = 0x0F;
/// Reserved.
pub const MemType_Reserved: MemoryType = 0x10;
/// IPC buffers with descriptor flags=1.
pub const MemType_IpcBuffer1: MemoryType = 0x11;
/// IPC buffers with descriptor flags=3.
pub const MemType_IpcBuffer3: MemoryType = 0x12;
/// Mapped in kernel during [`svcCreateThread`].
pub const MemType_KernelStack: MemoryType = 0x13;
/// Mapped in kernel during [`svcControlCodeMemory`].
pub const MemType_CodeReadOnly: MemoryType = 0x14;
/// Mapped in kernel during [`svcControlCodeMemory`].
pub const MemType_CodeWritable: MemoryType = 0x15;
/// Not available.
pub const MemType_Coverage: MemoryType = 0x16;
/// Mapped in kernel during [`svcMapInsecurePhysicalMemory`].
pub const MemType_Insecure: MemoryType = 0x17;

/// Memory state bitmasks.
pub type MemoryState = u32;
/// Type field (see [`MemoryType`]).
pub const MemState_Type: MemoryState = 0xFF;
/// Permission change allowed.
pub const MemState_PermChangeAllowed: MemoryState = BIT(8);
/// Force read/writable by debug syscalls.
pub const MemState_ForceRwByDebugSyscalls: MemoryState = BIT(9);
/// IPC type 0 send allowed.
pub const MemState_IpcSendAllowed_Type0: MemoryState = BIT(10);
/// IPC type 3 send allowed.
pub const MemState_IpcSendAllowed_Type3: MemoryState = BIT(11);
/// IPC type 1 send allowed.
pub const MemState_IpcSendAllowed_Type1: MemoryState = BIT(12);
/// Process permission change allowed.
pub const MemState_ProcessPermChangeAllowed: MemoryState = BIT(14);
/// Map allowed.
pub const MemState_MapAllowed: MemoryState = BIT(15);
/// Unmap process code memory allowed.
pub const MemState_UnmapProcessCodeMemAllowed: MemoryState = BIT(16);
/// Transfer memory allowed.
pub const MemState_TransferMemAllowed: MemoryState = BIT(17);
/// Query physical address allowed.
pub const MemState_QueryPAddrAllowed: MemoryState = BIT(18);
/// Map device allowed ([`svcMapDeviceAddressSpace`] and [`svcMapDeviceAddressSpaceByForce`]).
pub const MemState_MapDeviceAllowed: MemoryState = BIT(19);
/// Map device aligned allowed.
pub const MemState_MapDeviceAlignedAllowed: MemoryState = BIT(20);
/// IPC buffer allowed.
pub const MemState_IpcBufferAllowed: MemoryState = BIT(21);
/// Is pool allocated.
pub const MemState_IsPoolAllocated: MemoryState = BIT(22);
/// Alias for [`MemState_IsPoolAllocated`].
pub const MemState_IsRefCounted: MemoryState = MemState_IsPoolAllocated;
/// Map process allowed.
pub const MemState_MapProcessAllowed: MemoryState = BIT(23);
/// Attribute change allowed.
pub const MemState_AttrChangeAllowed: MemoryState = BIT(24);
/// Code memory allowed.
pub const MemState_CodeMemAllowed: MemoryState = BIT(25);

/// Memory attribute bitmasks.
pub type MemoryAttribute = u32;
/// Is borrowed memory.
pub const MemAttr_IsBorrowed: MemoryAttribute = BIT(0);
/// Is IPC mapped (when IpcRefCount > 0).
pub const MemAttr_IsIpcMapped: MemoryAttribute = BIT(1);
/// Is device mapped (when DeviceRefCount > 0).
pub const MemAttr_IsDeviceMapped: MemoryAttribute = BIT(2);
/// Is uncached.
pub const MemAttr_IsUncached: MemoryAttribute = BIT(3);
/// Is permission locked.
pub const MemAttr_IsPermissionLocked: MemoryAttribute = BIT(4);

/// Memory permission bitmasks.
pub type Permission = u32;
/// No permissions.
pub const Perm_None: Permission = 0;
/// Read permission.
pub const Perm_R: Permission = BIT(0);
/// Write permission.
pub const Perm_W: Permission = BIT(1);
/// Execute permission.
pub const Perm_X: Permission = BIT(2);
/// Read/write permissions.
pub const Perm_Rw: Permission = Perm_R | Perm_W;
/// Read/execute permissions.
pub const Perm_Rx: Permission = Perm_R | Perm_X;
/// Don't care
pub const Perm_DontCare: Permission = BIT(28);

/// Break reasons
pub type BreakReason = u32;
pub const BreakReason_Panic: BreakReason = 0;
pub const BreakReason_Assert: BreakReason = 1;
pub const BreakReason_User: BreakReason = 2;
pub const BreakReason_PreLoadDll: BreakReason = 3;
pub const BreakReason_PostLoadDll: BreakReason = 4;
pub const BreakReason_PreUnloadDll: BreakReason = 5;
pub const BreakReason_PostUnloadDll: BreakReason = 6;
pub const BreakReason_CppException: BreakReason = 7;
pub const BreakReason_NotificationOnlyFlag: BreakReason = 0x80000000;

/// Code memory mapping operations
pub type CodeMapOperation = u32;
/// Map owner.
pub const CodeMapOperation_MapOwner: CodeMapOperation = 0;
/// Map slave.
pub const CodeMapOperation_MapSlave: CodeMapOperation = 1;
/// Unmap owner.
pub const CodeMapOperation_UnmapOwner: CodeMapOperation = 2;
/// Unmap slave.
pub const CodeMapOperation_UnmapSlave: CodeMapOperation = 3;

/// Limitable Resources.
pub type LimitableResource = u32;
/// How much memory can a process map.
pub const LimitableResource_Memory: LimitableResource = 0;
/// How many threads can a process spawn.
pub const LimitableResource_Threads: LimitableResource = 1;
/// How many events can a process have.
pub const LimitableResource_Events: LimitableResource = 2;
/// How many transfer memories can a process make.
pub const LimitableResource_TransferMemories: LimitableResource = 3;
/// How many sessions can a process own.
pub const LimitableResource_Sessions: LimitableResource = 4;

/// Thread Activity.
pub type ThreadActivity = u32;
/// Thread can run.
pub const ThreadActivity_Runnable: ThreadActivity = 0;
/// Thread is paused.
pub const ThreadActivity_Paused: ThreadActivity = 1;

/// Process Information.
pub type ProcessInfoType = u32;
/// What state is a process in.
pub const ProcessInfoType_ProcessState: ProcessInfoType = 0;

/// Process States.
pub type ProcessState = u32;
/// Newly-created process, not yet started.
pub const ProcessState_Created: ProcessState = 0;
/// Newly-created process, not yet started but attached to debugger.
pub const ProcessState_CreatedAttached: ProcessState = 1;
/// Process that is running normally (and detached from any debugger).
pub const ProcessState_Running: ProcessState = 2;
/// Process that has just crashed.
pub const ProcessState_Crashed: ProcessState = 3;
/// Process that is running normally, attached to a debugger.
pub const ProcessState_RunningAttached: ProcessState = 4;
/// Process has begun exiting.
pub const ProcessState_Exiting: ProcessState = 5;
/// Process has finished exiting.
pub const ProcessState_Exited: ProcessState = 6;
/// Process execution suspended by debugger.
pub const ProcessState_DebugSuspended: ProcessState = 7;

/// Process Activity.
pub type ProcessActivity = u32;
/// Process can run.
pub const ProcessActivity_Runnable: ProcessActivity = 0;
/// Process is paused.
pub const ProcessActivity_Paused: ProcessActivity = 1;

/// Debug Thread Parameters.
pub type DebugThreadParam = u32;
pub const DebugThreadParam_ActualPriority: DebugThreadParam = 0;
pub const DebugThreadParam_State: DebugThreadParam = 1;
pub const DebugThreadParam_IdealCore: DebugThreadParam = 2;
pub const DebugThreadParam_CurrentCore: DebugThreadParam = 3;
pub const DebugThreadParam_CoreMask: DebugThreadParam = 4;

/// GetInfo IDs.
pub type InfoType = u32;
/// Bitmask of allowed Core IDs.
pub const InfoType_CoreMask: InfoType = 0;
/// Bitmask of allowed Thread Priorities.
pub const InfoType_PriorityMask: InfoType = 1;
/// Base of the Alias memory region.
pub const InfoType_AliasRegionAddress: InfoType = 2;
/// Size of the Alias memory region.
pub const InfoType_AliasRegionSize: InfoType = 3;
/// Base of the Heap memory region.
pub const InfoType_HeapRegionAddress: InfoType = 4;
/// Size of the Heap memory region.
pub const InfoType_HeapRegionSize: InfoType = 5;
/// Total amount of memory available for process.
pub const InfoType_TotalMemorySize: InfoType = 6;
/// Amount of memory currently used by process.
pub const InfoType_UsedMemorySize: InfoType = 7;
/// Whether current process is being debugged.
pub const InfoType_DebuggerAttached: InfoType = 8;
/// Current process's resource limit handle.
pub const InfoType_ResourceLimit: InfoType = 9;
/// Number of idle ticks on CPU (only usable with current thread's core).
pub const InfoType_IdleTickCount: InfoType = 10;
/// [2.0.0+] Random entropy for current process.
pub const InfoType_RandomEntropy: InfoType = 11;
/// [2.0.0+] Base of the process's address space.
pub const InfoType_AslrRegionAddress: InfoType = 12;
/// [2.0.0+] Size of the process's address space.
pub const InfoType_AslrRegionSize: InfoType = 13;
/// [2.0.0+] Base of the Stack memory region.
pub const InfoType_StackRegionAddress: InfoType = 14;
/// [2.0.0+] Size of the Stack memory region.
pub const InfoType_StackRegionSize: InfoType = 15;
/// [3.0.0+] Total memory allocated for process memory management.
pub const InfoType_SystemResourceSizeTotal: InfoType = 16;
/// [3.0.0+] Amount of memory currently used by process memory management.
pub const InfoType_SystemResourceSizeUsed: InfoType = 17;
/// [3.0.0+] Program ID for the process.
pub const InfoType_ProgramId: InfoType = 18;
/// [4.0.0-4.1.0] Min/max initial process IDs.
pub const InfoType_InitialProcessIdRange: InfoType = 19;
/// [5.0.0+] Address of the process's exception context (for break).
pub const InfoType_UserExceptionContextAddress: InfoType = 20;
/// [6.0.0+] Total amount of memory available for process, excluding that for process memory management.
pub const InfoType_TotalNonSystemMemorySize: InfoType = 21;
/// [6.0.0+] Amount of memory used by process, excluding that for process memory management.
pub const InfoType_UsedNonSystemMemorySize: InfoType = 22;
/// [9.0.0+] Whether the specified process is an Application.
pub const InfoType_IsApplication: InfoType = 23;
/// [11.0.0+] The number of free threads available to the process's resource limit.
pub const InfoType_FreeThreadCount: InfoType = 24;
/// [13.0.0+] Number of ticks spent on thread.
pub const InfoType_ThreadTickCount: InfoType = 25;
/// [14.0.0+] Does process have access to SVC (only usable with [`svcSynchronizePreemptionState`] at present).
pub const InfoType_IsSvcPermitted: InfoType = 26;
/// [16.0.0+] Low bits of the physical address for a KIoRegion.
pub const InfoType_IoRegionHint: InfoType = 27;
/// [18.0.0+] Extra size added to the reserved region.
pub const InfoType_AliasRegionExtraSize: InfoType = 28;
/// [19.0.0+] Low bits of the process address for a KTransferMemory.
pub const InfoType_TransferMemoryHint: InfoType = 34;
/// [1.0.0-12.1.0] Number of ticks spent on thread.
pub const InfoType_ThreadTickCountDeprecated: InfoType = 0xF0000002;

/// GetSystemInfo IDs.
pub type SystemInfoType = u32;
/// Total amount of DRAM available to system.
pub const SystemInfoType_TotalPhysicalMemorySize: SystemInfoType = 0;
/// Current amount of DRAM used by system.
pub const SystemInfoType_UsedPhysicalMemorySize: SystemInfoType = 1;
/// Min/max initial process IDs.
pub const SystemInfoType_InitialProcessIdRange: SystemInfoType = 2;

/// GetInfo Idle/Thread Tick Count Sub IDs.
pub type TickCountInfo = u64;
/// Tick count on core 0.
pub const TickCountInfo_Core0: TickCountInfo = 0;
/// Tick count on core 1.
pub const TickCountInfo_Core1: TickCountInfo = 1;
/// Tick count on core 2.
pub const TickCountInfo_Core2: TickCountInfo = 2;
/// Tick count on core 3.
pub const TickCountInfo_Core3: TickCountInfo = 3;
/// ThreadTickCount: Tick count on all cores, IdleTickCount: Thread's current core.
pub const TickCountInfo_Total: TickCountInfo = u64::MAX;

/// GetInfo InitialProcessIdRange Sub IDs.
pub type InitialProcessIdRangeInfo = u32;
/// Lowest initial process ID.
pub const InitialProcessIdRangeInfo_Minimum: InitialProcessIdRangeInfo = 0;
/// Highest initial process ID.
pub const InitialProcessIdRangeInfo_Maximum: InitialProcessIdRangeInfo = 1;

/// GetSystemInfo PhysicalMemory Sub IDs.
pub type PhysicalMemorySystemInfo = u32;
/// Memory allocated for application usage.
pub const PhysicalMemorySystemInfo_Application: PhysicalMemorySystemInfo = 0;
/// Memory allocated for applet usage.
pub const PhysicalMemorySystemInfo_Applet: PhysicalMemorySystemInfo = 1;
/// Memory allocated for system usage.
pub const PhysicalMemorySystemInfo_System: PhysicalMemorySystemInfo = 2;
/// Memory allocated for unsafe system usage (accessible to devices).
pub const PhysicalMemorySystemInfo_SystemUnsafe: PhysicalMemorySystemInfo = 3;

/// SleepThread yield types.
pub type YieldType = i64;
/// Yields to another thread on the same core.
pub const YieldType_WithoutCoreMigration: YieldType = 0;
/// Yields to another thread (possibly on a different core).
pub const YieldType_WithCoreMigration: YieldType = -1;
/// Yields and performs forced load-balancing.
pub const YieldType_ToAnyThread: YieldType = -2;

/// SignalToAddress behaviors.
pub type SignalType = u32;
/// Signals the address.
pub const SignalType_Signal: SignalType = 0;
/// Signals the address and increments its value if equal to argument.
pub const SignalType_SignalAndIncrementIfEqual: SignalType = 1;
/// Signals the address and updates its value if equal to argument.
pub const SignalType_SignalAndModifyBasedOnWaitingThreadCountIfEqual: SignalType = 2;

/// WaitForAddress behaviors.
pub type ArbitrationType = u32;
/// Wait if the 32-bit value is less than argument.
pub const ArbitrationType_WaitIfLessThan: ArbitrationType = 0;
/// Decrement the 32-bit value and wait if it is less than argument.
pub const ArbitrationType_DecrementAndWaitIfLessThan: ArbitrationType = 1;
/// Wait if the 32-bit value is equal to argument.
pub const ArbitrationType_WaitIfEqual: ArbitrationType = 2;
/// [19.0.0+] Wait if the 64-bit value is equal to argument.
pub const ArbitrationType_WaitIfEqual64: ArbitrationType = 3;

/// Memory mapping type.
pub type MemoryMapping = u32;
/// Mapping IO registers.
pub const MemoryMapping_IoRegister: MemoryMapping = 0;
/// Mapping normal memory without cache.
pub const MemoryMapping_Uncached: MemoryMapping = 1;
/// Mapping normal memory.
pub const MemoryMapping_Memory: MemoryMapping = 2;

/// Io Pools.
pub type IoPoolType = u32;
/// Physical address range 0x12000000-0x1FFFFFFF
pub const IoPoolType_PcieA2: IoPoolType = 0;

/// DebugEvent types
pub type DebugEventType = u32;
pub const DebugEventType_CreateProcess: DebugEventType = 0;
pub const DebugEventType_CreateThread: DebugEventType = 1;
pub const DebugEventType_ExitProcess: DebugEventType = 2;
pub const DebugEventType_ExitThread: DebugEventType = 3;
pub const DebugEventType_Exception: DebugEventType = 4;

/// Process exit reasons
pub type ProcessExitReason = u32;
pub const ProcessExitReason_ExitProcess: ProcessExitReason = 0;
pub const ProcessExitReason_TerminateProcess: ProcessExitReason = 1;
pub const ProcessExitReason_Exception: ProcessExitReason = 2;

/// Thread exit reasons
pub type ThreadExitReason = u32;
pub const ThreadExitReason_ExitThread: ThreadExitReason = 0;
pub const ThreadExitReason_TerminateThread: ThreadExitReason = 1;
pub const ThreadExitReason_ExitProcess: ThreadExitReason = 2;
pub const ThreadExitReason_TerminateProcess: ThreadExitReason = 3;

/// Debug exception types
pub type DebugException = u32;
pub const DebugException_UndefinedInstruction: DebugException = 0;
pub const DebugException_InstructionAbort: DebugException = 1;
pub const DebugException_DataAbort: DebugException = 2;
pub const DebugException_AlignmentFault: DebugException = 3;
pub const DebugException_DebuggerAttached: DebugException = 4;
pub const DebugException_BreakPoint: DebugException = 5;
pub const DebugException_UserBreak: DebugException = 6;
pub const DebugException_DebuggerBreak: DebugException = 7;
pub const DebugException_UndefinedSystemCall: DebugException = 8;
/// [2.0.0+]
pub const DebugException_MemorySystemError: DebugException = 9;

/// Break point types
pub type BreakPointType = u32;
pub const BreakPointType_HardwareInstruction: BreakPointType = 0;
pub const BreakPointType_HardwareData: BreakPointType = 1;

/// DebugEvent flags
pub type DebugEventFlag = u32;
pub const DebugEventFlag_Stopped: DebugEventFlag = BIT(0);

/// Address space types for CreateProcessFlags
pub type CreateProcessFlagAddressSpace = u32;
pub const CreateProcessFlagAddressSpace_32bit: CreateProcessFlagAddressSpace = 0;
/// 36-bit width
pub const CreateProcessFlagAddressSpace_64bitDeprecated: CreateProcessFlagAddressSpace = 1;
pub const CreateProcessFlagAddressSpace_32bitWithoutAlias: CreateProcessFlagAddressSpace = 2;
/// [2.0.0+] 39-bit width
pub const CreateProcessFlagAddressSpace_64bit: CreateProcessFlagAddressSpace = 3;

/// Memory regions
pub type MemoryRegion = u32;
/// [1.0.0-4.1.0]
pub const MemoryRegion_Default: MemoryRegion = 0;
/// [2.0.0-4.1.0]
pub const MemoryRegion_Secure: MemoryRegion = 1;

/// Memory information structure.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct MemoryInfo {
    /// Base address.
    pub addr: u64,
    /// Size.
    pub size: u64,
    /// Memory type (see lower 8 bits of [`MemoryState`]).
    pub type_: u32,
    /// Memory attributes (see [`MemoryAttribute`]).
    pub attr: u32,
    /// Memory permissions (see [`Permission`]).
    pub perm: u32,
    /// IPC reference count.
    pub ipc_refcount: u32,
    /// Device reference count.
    pub device_refcount: u32,
    /// Padding.
    pub padding: u32,
}

/// Physical memory information structure.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PhysicalMemoryInfo {
    /// Physical address.
    pub physical_address: u64,
    /// Virtual address.
    pub virtual_address: u64,
    /// Size.
    pub size: u64,
}

/// Secure monitor arguments.
#[repr(C, packed)]
#[derive(Debug, Default, Clone, Copy)]
pub struct SecmonArgs {
    /// Values of X0 through X7.
    pub X: [u64; 8],
}

/// Context of a scheduled thread.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct LastThreadContext {
    /// Frame Pointer for the thread.
    pub fp: u64,
    /// Stack Pointer for the thread.
    pub sp: u64,
    /// Link Register for the thread.
    pub lr: u64,
    /// Program Counter for the thread.
    pub pc: u64,
}

/// Flags for svcCreateProcess and CreateProcess event
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CreateProcessFlags {
    /// Raw bitfield word.
    pub bits: u32,
}

impl CreateProcessFlags {
    bitfields!(bits {
        is_64bit / set_is_64bit: 0, 1 => u32;
        /// [`CreateProcessFlagAddressSpace`]
        address_space / set_address_space: 1, 3 => u32;
        /// [2.0.0+]
        enable_debug / set_enable_debug: 4, 1 => u32;
        enable_aslr / set_enable_aslr: 5, 1 => u32;
        is_application / set_is_application: 6, 1 => u32;
        /// [4.0.0-4.1.0] [`MemoryRegion`], [5.0.0+] [`PhysicalMemorySystemInfo`]
        pool_partition / set_pool_partition: 7, 4 => u32;
        /// [7.0.0+] Only allowed in combination with is_application
        optimize_memory_allocation / set_optimize_memory_allocation: 11, 1 => u32;
        /// [11.0.0+]
        disable_device_address_space_merge / set_disable_device_address_space_merge: 12, 1 => u32;
        /// [18.0.0+]
        enable_alias_region_extra_size / set_enable_alias_region_extra_size: 13, 1 => u32;
        reserved / set_reserved: 14, 18 => u32;
    });
}

/// DebugEventType_CreateProcess
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_CreateProcess {
    pub program_id: u64,
    pub process_id: u64,
    pub name: [c_char; 0xC],
    /// [`CreateProcessFlags`]
    pub flags: u32,
    /// [5.0.0+]
    pub user_exception_context_address: *mut c_void,
}

/// DebugEventType_CreateThread
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_CreateThread {
    pub thread_id: u64,
    pub tls_address: *mut c_void,
    /// [1.0.0-10.2.0]
    pub entrypoint: *mut c_void,
}

/// DebugEventType_ExitProcess
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_ExitProcess {
    /// [`ProcessExitReason`]
    pub reason: u32,
}

/// DebugEventType_ExitThread
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_ExitThread {
    /// [`ThreadExitReason`]
    pub reason: u32,
}

/// DebugException_UndefinedInstruction
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_Exception_UndefinedInstruction {
    pub insn: u32,
}

/// DebugException_DataAbort
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_Exception_DataAbort {
    pub address: *mut c_void,
}

/// DebugException_AlignmentFault
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_Exception_AlignmentFault {
    pub address: *mut c_void,
}

/// DebugException_BreakPoint
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_Exception_BreakPoint {
    /// [`BreakPointType`]
    pub type_: u32,
    pub address: *mut c_void,
}

/// DebugException_UserBreak
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_Exception_UserBreak {
    /// [`BreakReason`]
    pub break_reason: u32,
    pub address: *mut c_void,
    pub size: usize,
}

/// DebugException_DebuggerBreak
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_Exception_DebuggerBreak {
    pub active_thread_ids: [u64; 4],
}

/// DebugException_UndefinedSystemCall
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DebugEventInfo_Exception_UndefinedSystemCall {
    pub id: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DebugEventInfo_Exception_Specific {
    pub undefined_instruction: DebugEventInfo_Exception_UndefinedInstruction,
    pub data_abort: DebugEventInfo_Exception_DataAbort,
    pub alignment_fault: DebugEventInfo_Exception_AlignmentFault,
    pub break_point: DebugEventInfo_Exception_BreakPoint,
    pub user_break: DebugEventInfo_Exception_UserBreak,
    pub debugger_break: DebugEventInfo_Exception_DebuggerBreak,
    pub undefined_system_call: DebugEventInfo_Exception_UndefinedSystemCall,
    pub raw: u64,
}

/// DebugEventType_Exception
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DebugEventInfo_Exception {
    /// [`DebugException`]
    pub type_: u32,
    pub address: *mut c_void,
    pub specific: DebugEventInfo_Exception_Specific,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union DebugEventInfo_Info {
    pub create_process: DebugEventInfo_CreateProcess,
    pub create_thread: DebugEventInfo_CreateThread,
    pub exit_process: DebugEventInfo_ExitProcess,
    pub exit_thread: DebugEventInfo_ExitThread,
    pub exception: DebugEventInfo_Exception,
}

/// DebugEvent structure
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DebugEventInfo {
    /// [`DebugEventType`]
    pub type_: u32,
    /// [`DebugEventFlag`]
    pub flags: u32,
    pub thread_id: u64,
    pub info: DebugEventInfo_Info,
}

impl_zeroed_default!(DebugEventInfo);

unsafe extern "C" {
    /// Set the process heap to a given size. It can both extend and shrink the heap.
    ///
    /// # Arguments
    /// - `out_addr` (out): Variable to which write the address of the heap (which is randomized and fixed by the kernel)
    /// - `size` (in): Size of the heap, must be a multiple of 0x200000.
    ///
    /// **Note:** Syscall number 0x01.
    pub fn svcSetHeapSize(out_addr: *mut *mut c_void, size: u64) -> Result;

    /// Set the memory permissions of a (page-aligned) range of memory.
    ///
    /// # Arguments
    /// - `addr` (in): Start address of the range.
    /// - `size` (in): Size of the range, in bytes.
    /// - `perm` (in): Permissions (see [`Permission`]).
    ///
    /// **Remark:** Perm_X is not allowed. Setting write-only is not allowed either (Perm_W).
    /// This can be used to move back and forth between Perm_None, Perm_R and Perm_Rw.
    ///
    /// **Note:** Syscall number 0x02.
    pub fn svcSetMemoryPermission(addr: *mut c_void, size: u64, perm: u32) -> Result;

    /// Set the memory attributes of a (page-aligned) range of memory.
    ///
    /// # Arguments
    /// - `addr` (in): Start address of the range.
    /// - `size` (in): Size of the range, in bytes.
    /// - `val0` (in): State0
    /// - `val1` (in): State1
    ///
    /// **Remark:** See [switchbrew.org Wiki](https://switchbrew.org/wiki/SVC#svcSetMemoryAttribute) for more details.
    ///
    /// **Note:** Syscall number 0x03.
    pub fn svcSetMemoryAttribute(addr: *mut c_void, size: u64, val0: u32, val1: u32) -> Result;

    /// Maps a memory range into a different range. Mainly used for adding guard pages around stack.
    /// Source range gets reprotected to Perm_None (it can no longer be accessed), and [`MemAttr_IsBorrowed`] is set in the source [`MemoryAttribute`].
    ///
    /// # Arguments
    /// - `dst_addr` (in): Destination address.
    /// - `src_addr` (in): Source address.
    /// - `size` (in): Size of the range.
    ///
    /// **Note:** Syscall number 0x04.
    pub fn svcMapMemory(dst_addr: *mut c_void, src_addr: *mut c_void, size: u64) -> Result;

    /// Unmaps a region that was previously mapped with [`svcMapMemory`].
    ///
    /// # Arguments
    /// - `dst_addr` (in): Destination address.
    /// - `src_addr` (in): Source address.
    /// - `size` (in): Size of the range.
    ///
    /// **Note:** Syscall number 0x05.
    pub fn svcUnmapMemory(dst_addr: *mut c_void, src_addr: *mut c_void, size: u64) -> Result;

    /// Query information about an address. Will always fetch the lowest page-aligned mapping that contains the provided address.
    ///
    /// # Arguments
    /// - `meminfo_ptr` (out): [`MemoryInfo`] structure which will be filled in.
    /// - `pageinfo` (out): Page information which will be filled in.
    /// - `addr` (in): Address to query.
    ///
    /// **Note:** Syscall number 0x06.
    pub fn svcQueryMemory(meminfo_ptr: *mut MemoryInfo, pageinfo: *mut u32, addr: u64) -> Result;

    /// Exits the current process.
    ///
    /// **Note:** Syscall number 0x07.
    pub fn svcExitProcess() -> !;

    /// Creates a thread.
    ///
    /// **Note:** Syscall number 0x08.
    pub fn svcCreateThread(out: *mut Handle, entry: *mut c_void, arg: *mut c_void, stack_top: *mut c_void, prio: c_int, cpuid: c_int) -> Result;

    /// Starts a freshly created thread.
    ///
    /// **Note:** Syscall number 0x09.
    pub fn svcStartThread(handle: Handle) -> Result;

    /// Exits the current thread.
    ///
    /// **Note:** Syscall number 0x0A.
    pub fn svcExitThread() -> !;

    /// Sleeps the current thread for the specified amount of time.
    ///
    /// # Arguments
    /// - `nano` (in): Number of nanoseconds to sleep, or [`YieldType`] for yield.
    ///
    /// **Note:** Syscall number 0x0B.
    pub fn svcSleepThread(nano: s64);

    /// Gets a thread's priority.
    ///
    /// **Note:** Syscall number 0x0C.
    pub fn svcGetThreadPriority(priority: *mut s32, handle: Handle) -> Result;

    /// Sets a thread's priority.
    ///
    /// **Note:** Syscall number 0x0D.
    pub fn svcSetThreadPriority(handle: Handle, priority: u32) -> Result;

    /// Gets a thread's core mask.
    ///
    /// **Note:** Syscall number 0x0E.
    pub fn svcGetThreadCoreMask(preferred_core: *mut s32, affinity_mask: *mut u64, handle: Handle) -> Result;

    /// Sets a thread's core mask.
    ///
    /// **Note:** Syscall number 0x0F.
    pub fn svcSetThreadCoreMask(handle: Handle, preferred_core: s32, affinity_mask: u32) -> Result;

    /// Gets the current processor's number.
    ///
    /// **Note:** Syscall number 0x10.
    pub fn svcGetCurrentProcessorNumber() -> u32;

    /// Sets an event's signalled status.
    ///
    /// **Note:** Syscall number 0x11.
    pub fn svcSignalEvent(handle: Handle) -> Result;

    /// Clears an event's signalled status.
    ///
    /// **Note:** Syscall number 0x12.
    pub fn svcClearEvent(handle: Handle) -> Result;

    /// Maps a block of shared memory.
    ///
    /// **Note:** Syscall number 0x13.
    pub fn svcMapSharedMemory(handle: Handle, addr: *mut c_void, size: usize, perm: u32) -> Result;

    /// Unmaps a block of shared memory.
    ///
    /// **Note:** Syscall number 0x14.
    pub fn svcUnmapSharedMemory(handle: Handle, addr: *mut c_void, size: usize) -> Result;

    /// Creates a block of transfer memory.
    ///
    /// **Note:** Syscall number 0x15.
    pub fn svcCreateTransferMemory(out: *mut Handle, addr: *mut c_void, size: usize, perm: u32) -> Result;

    /// Closes a handle, decrementing the reference count of the corresponding kernel object.
    /// This might result in the kernel freeing the object.
    ///
    /// # Arguments
    /// - `handle`: Handle to close.
    ///
    /// **Note:** Syscall number 0x16.
    pub fn svcCloseHandle(handle: Handle) -> Result;

    /// Resets a signal.
    ///
    /// **Note:** Syscall number 0x17.
    pub fn svcResetSignal(handle: Handle) -> Result;

    /// Waits on one or more synchronization objects, optionally with a timeout.
    ///
    /// **Note:** Syscall number 0x18.
    ///
    /// **Note:** \p handleCount must not be greater than [`MAX_WAIT_OBJECTS`]. This is a Horizon kernel limitation.
    ///
    /// **Note:** This is the raw syscall, which can be cancelled by [`svcCancelSynchronization`] or other means. `waitHandles` or `waitMultiHandle` should normally be used instead.
    pub fn svcWaitSynchronization(index: *mut s32, handles: *const Handle, handleCount: s32, timeout: u64) -> Result;

    /// Waits a [`svcWaitSynchronization`] operation being done on a synchronization object in another thread.
    ///
    /// **Note:** Syscall number 0x19.
    pub fn svcCancelSynchronization(thread: Handle) -> Result;

    /// Arbitrates a mutex lock operation in userspace.
    ///
    /// **Note:** Syscall number 0x1A.
    pub fn svcArbitrateLock(wait_tag: u32, tag_location: *mut u32, self_tag: u32) -> Result;

    /// Arbitrates a mutex unlock operation in userspace.
    ///
    /// **Note:** Syscall number 0x1B.
    pub fn svcArbitrateUnlock(tag_location: *mut u32) -> Result;

    /// Performs a condition variable wait operation in userspace.
    ///
    /// **Note:** Syscall number 0x1C.
    pub fn svcWaitProcessWideKeyAtomic(key: *mut u32, tag_location: *mut u32, self_tag: u32, timeout: u64) -> Result;

    /// Performs a condition variable wake-up operation in userspace.
    ///
    /// **Note:** Syscall number 0x1D.
    pub fn svcSignalProcessWideKey(key: *mut u32, num: s32);

    /// Gets the current system tick.
    ///
    /// **Note:** Syscall number 0x1E.
    pub fn svcGetSystemTick() -> u64;

    /// Connects to a registered named port.
    ///
    /// **Note:** Syscall number 0x1F.
    pub fn svcConnectToNamedPort(session: *mut Handle, name: *const c_char) -> Result;

    /// Sends a light IPC synchronization request to a session.
    ///
    /// **Note:** Syscall number 0x20.
    pub fn svcSendSyncRequestLight(session: Handle) -> Result;

    /// Sends an IPC synchronization request to a session.
    ///
    /// **Note:** Syscall number 0x21.
    pub fn svcSendSyncRequest(session: Handle) -> Result;

    /// Sends an IPC synchronization request to a session from an user allocated buffer.
    ///
    /// **Remark:** size must be allocated to 0x1000 bytes.
    ///
    /// **Note:** Syscall number 0x22.
    pub fn svcSendSyncRequestWithUserBuffer(usrBuffer: *mut c_void, size: u64, session: Handle) -> Result;

    /// Sends an IPC synchronization request to a session from an user allocated buffer (asynchronous version).
    ///
    /// **Remark:** size must be allocated to 0x1000 bytes.
    ///
    /// **Note:** Syscall number 0x23.
    pub fn svcSendAsyncRequestWithUserBuffer(handle: *mut Handle, usrBuffer: *mut c_void, size: u64, session: Handle) -> Result;

    /// Gets the PID associated with a process.
    ///
    /// **Note:** Syscall number 0x24.
    pub fn svcGetProcessId(processID: *mut u64, handle: Handle) -> Result;

    /// Gets the TID associated with a process.
    ///
    /// **Note:** Syscall number 0x25.
    pub fn svcGetThreadId(threadID: *mut u64, handle: Handle) -> Result;

    /// Breaks execution.
    ///
    /// # Arguments
    /// - `breakReason` (in): Break reason (see [`BreakReason`]).
    /// - `address` (in): Address of the buffer to pass to the debugger.
    /// - `size` (in): Size of the buffer to pass to the debugger.
    ///
    /// **Note:** Syscall number 0x26.
    pub fn svcBreak(breakReason: u32, address: usize, size: usize) -> Result;

    /// Outputs debug text, if used during debugging.
    ///
    /// # Arguments
    /// - `str` (in): Text to output.
    /// - `size` (in): Size of the text in bytes.
    ///
    /// **Note:** Syscall number 0x27.
    pub fn svcOutputDebugString(str: *const c_char, size: u64) -> Result;

    /// Returns from an exception.
    ///
    /// # Arguments
    /// - `res` (in): Result code.
    ///
    /// **Note:** Syscall number 0x28.
    pub fn svcReturnFromException(res: Result) -> !;

    /// Retrieves information about the system, or a certain kernel object.
    ///
    /// # Arguments
    /// - `out` (out): Variable to which store the information.
    /// - `id0` (in): First ID of the property to retrieve.
    /// - `handle` (in): Handle of the object to retrieve information from, or [`INVALID_HANDLE`] to retrieve information about the system.
    /// - `id1` (in): Second ID of the property to retrieve.
    ///
    /// **Remark:** The full list of property IDs can be found on the [switchbrew.org wiki](https://switchbrew.org/wiki/SVC#svcGetInfo).
    ///
    /// **Note:** Syscall number 0x29.
    pub fn svcGetInfo(out: *mut u64, id0: u32, handle: Handle, id1: u64) -> Result;

    /// Flushes the entire data cache (by set/way).
    ///
    /// **Note:** Syscall number 0x2A.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    ///
    /// **Warning:** This syscall is dangerous, and should not be used.
    pub fn svcFlushEntireDataCache();

    /// Flushes data cache for a virtual address range.
    ///
    /// # Arguments
    /// - `address` (in): Address of region to flush.
    /// - `size` (in): Size of region to flush.
    ///
    /// **Remark:** armDCacheFlush should be used instead of this syscall whenever possible.
    ///
    /// **Note:** Syscall number 0x2B.
    pub fn svcFlushDataCache(address: *mut c_void, size: usize) -> Result;

    /// Maps new heap memory at the desired address. [3.0.0+]
    ///
    /// **Note:** Syscall number 0x2C.
    pub fn svcMapPhysicalMemory(address: *mut c_void, size: u64) -> Result;

    /// Undoes the effects of [`svcMapPhysicalMemory`]. [3.0.0+]
    ///
    /// **Note:** Syscall number 0x2D.
    pub fn svcUnmapPhysicalMemory(address: *mut c_void, size: u64) -> Result;

    /// Gets information about a thread that will be scheduled in the future. [5.0.0+]
    ///
    /// # Arguments
    /// - `out_context` (out): Output [`LastThreadContext`] for the thread that will be scheduled.
    /// - `out_thread_id` (out): Output thread id for the thread that will be scheduled.
    /// - `debug` (in): Debug handle.
    /// - `ns` (in): Nanoseconds in the future to get scheduled thread at.
    ///
    /// **Note:** Syscall number 0x2E.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetDebugFutureThreadInfo(out_context: *mut LastThreadContext, out_thread_id: *mut u64, debug: Handle, ns: s64) -> Result;

    /// Gets information about the previously-scheduled thread.
    ///
    /// # Arguments
    /// - `out_context` (out): Output [`LastThreadContext`] for the previously scheduled thread.
    /// - `out_tls_address` (out): Output tls address for the previously scheduled thread.
    /// - `out_flags` (out): Output flags for the previously scheduled thread.
    ///
    /// **Note:** Syscall number 0x2F.
    pub fn svcGetLastThreadInfo(out_context: *mut LastThreadContext, out_tls_address: *mut u64, out_flags: *mut u32) -> Result;

    /// Gets the maximum value a LimitableResource can have, for a Resource Limit handle.
    ///
    /// **Note:** Syscall number 0x30.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetResourceLimitLimitValue(out: *mut s64, reslimit_h: Handle, which: LimitableResource) -> Result;

    /// Gets the maximum value a LimitableResource can have, for a Resource Limit handle.
    ///
    /// **Note:** Syscall number 0x31.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetResourceLimitCurrentValue(out: *mut s64, reslimit_h: Handle, which: LimitableResource) -> Result;

    /// Configures the pause/unpause status of a thread.
    ///
    /// **Note:** Syscall number 0x32.
    pub fn svcSetThreadActivity(thread: Handle, paused: ThreadActivity) -> Result;

    /// Dumps the registers of a thread paused by @ref svcSetThreadActivity (register groups: all).
    ///
    /// # Arguments
    /// - `ctx` (out): Output thread context (register dump).
    /// - `thread` (in): Thread handle.
    ///
    /// **Note:** Syscall number 0x33.
    ///
    /// **Warning:** Official kernel will not dump x0..x18 if the thread is currently executing a system call, and prior to 6.0.0 doesn't dump TPIDR_EL0.
    pub fn svcGetThreadContext3(ctx: *mut ThreadContext, thread: Handle) -> Result;

    /// Arbitrates an address depending on type and value. [4.0.0+]
    ///
    /// # Arguments
    /// - `address` (in): Address to arbitrate.
    /// - `arb_type` (in): [`ArbitrationType`] to use.
    /// - `value` (in): Value to arbitrate on.
    /// - `timeout` (in): Maximum time in nanoseconds to wait.
    ///
    /// **Note:** Syscall number 0x34.
    pub fn svcWaitForAddress(address: *mut c_void, arb_type: u32, value: s64, timeout: s64) -> Result;

    /// Signals (and updates) an address depending on type and value. [4.0.0+]
    ///
    /// # Arguments
    /// - `address` (in): Address to arbitrate.
    /// - `signal_type` (in): [`SignalType`] to use.
    /// - `value` (in): Value to arbitrate on.
    /// - `count` (in): Number of waiting threads to signal.
    ///
    /// **Note:** Syscall number 0x35.
    pub fn svcSignalToAddress(address: *mut c_void, signal_type: u32, value: s32, count: s32) -> Result;

    /// Sets thread preemption state (used during abort/panic). [8.0.0+]
    ///
    /// **Note:** Syscall number 0x36.
    pub fn svcSynchronizePreemptionState();

    /// Gets the peak value a LimitableResource has had, for a Resource Limit handle. [11.0.0+]
    ///
    /// **Note:** Syscall number 0x37.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetResourceLimitPeakValue(out: *mut s64, reslimit_h: Handle, which: LimitableResource) -> Result;

    /// Creates an IO Pool. [13.0.0+]
    ///
    /// **Note:** Syscall number 0x39.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateIoPool(out_handle: *mut Handle, pool_type: u32) -> Result;

    /// Creates an IO Region. [13.0.0+]
    ///
    /// **Note:** Syscall number 0x3A.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateIoRegion(out_handle: *mut Handle, io_pool_h: Handle, physical_address: u64, size: u64, memory_mapping: u32, perm: u32) -> Result;

    /// Causes the kernel to dump debug information. [1.0.0-3.0.2]
    ///
    /// # Arguments
    /// - `dump_info_type` (in): Type of information to dump.
    /// - `arg0` (in): Argument to the debugging operation.
    ///
    /// **Note:** Syscall number 0x3C.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcDumpInfo(dump_info_type: u32, arg0: u64);

    /// Performs a debugging operation on the kernel. [4.0.0+]
    ///
    /// # Arguments
    /// - `kern_debug_type` (in): Type of debugging operation to perform.
    /// - `arg0` (in): First argument to the debugging operation.
    /// - `arg1` (in): Second argument to the debugging operation.
    /// - `arg2` (in): Third argument to the debugging operation.
    ///
    /// **Note:** Syscall number 0x3C.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcKernelDebug(kern_debug_type: u32, arg0: u64, arg1: u64, arg2: u64);

    /// Performs a debugging operation on the kernel. [4.0.0+]
    ///
    /// # Arguments
    /// - `kern_trace_state` (in): Type of tracing the kernel should perform.
    ///
    /// **Note:** Syscall number 0x3D.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcChangeKernelTraceState(kern_trace_state: u32);

    /// Creates an IPC session.
    ///
    /// **Note:** Syscall number 0x40.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateSession(server_handle: *mut Handle, client_handle: *mut Handle, unk0: u32, unk1: u64) -> Result;

    /// Accepts an IPC session.
    ///
    /// **Note:** Syscall number 0x41.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcAcceptSession(session_handle: *mut Handle, port_handle: Handle) -> Result;

    /// Performs light IPC input/output.
    ///
    /// # Arguments
    /// - `handle` (in): Server or port handle to act on.
    ///
    /// **Note:** Syscall number 0x42.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcReplyAndReceiveLight(handle: Handle) -> Result;

    /// Performs IPC input/output.
    ///
    /// **Note:** Syscall number 0x43.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcReplyAndReceive(index: *mut s32, handles: *const Handle, handleCount: s32, replyTarget: Handle, timeout: u64) -> Result;

    /// Performs IPC input/output from an user allocated buffer.
    ///
    /// **Note:** Syscall number 0x44.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcReplyAndReceiveWithUserBuffer(index: *mut s32, usrBuffer: *mut c_void, size: u64, handles: *const Handle, handleCount: s32, replyTarget: Handle, timeout: u64) -> Result;

    /// Creates a system event.
    ///
    /// **Note:** Syscall number 0x45.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateEvent(server_handle: *mut Handle, client_handle: *mut Handle) -> Result;

    /// Maps an IO Region. [13.0.0+]
    ///
    /// **Note:** Syscall number 0x46.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapIoRegion(io_region_h: Handle, address: *mut c_void, size: u64, perm: u32) -> Result;

    /// Undoes the effects of [`svcMapIoRegion`]. [13.0.0+]
    ///
    /// **Note:** Syscall number 0x47.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcUnmapIoRegion(io_region_h: Handle, address: *mut c_void, size: u64) -> Result;

    /// Maps unsafe memory (usable for GPU DMA) for a system module at the desired address. [5.0.0+]
    ///
    /// **Note:** Syscall number 0x48.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapPhysicalMemoryUnsafe(address: *mut c_void, size: u64) -> Result;

    /// Undoes the effects of [`svcMapPhysicalMemoryUnsafe`]. [5.0.0+]
    ///
    /// **Note:** Syscall number 0x49.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcUnmapPhysicalMemoryUnsafe(address: *mut c_void, size: u64) -> Result;

    /// Sets the system-wide limit for unsafe memory mappable using [`svcMapPhysicalMemoryUnsafe`]. [5.0.0+]
    ///
    /// **Note:** Syscall number 0x4A.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcSetUnsafeLimit(size: u64) -> Result;

    /// Creates code memory in the caller's address space [4.0.0+].
    ///
    /// **Note:** Syscall number 0x4B.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateCodeMemory(code_handle: *mut Handle, src_addr: *mut c_void, size: u64) -> Result;

    /// Maps code memory in the caller's address space [4.0.0+].
    ///
    /// **Note:** Syscall number 0x4C.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcControlCodeMemory(code_handle: Handle, op: CodeMapOperation, dst_addr: *mut c_void, size: u64, perm: u64) -> Result;

    /// Causes the system to enter deep sleep.
    ///
    /// **Note:** Syscall number 0x4D.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcSleepSystem();

    /// Reads/writes a protected MMIO register.
    ///
    /// **Note:** Syscall number 0x4E.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcReadWriteRegister(outVal: *mut u32, regAddr: u64, rwMask: u32, inVal: u32) -> Result;

    /// Configures the pause/unpause status of a process.
    ///
    /// **Note:** Syscall number 0x4F.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcSetProcessActivity(process: Handle, paused: ProcessActivity) -> Result;

    /// Creates a block of shared memory.
    ///
    /// **Note:** Syscall number 0x50.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateSharedMemory(out: *mut Handle, size: usize, local_perm: u32, other_perm: u32) -> Result;

    /// Maps a block of transfer memory.
    ///
    /// **Note:** Syscall number 0x51.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapTransferMemory(tmem_handle: Handle, addr: *mut c_void, size: usize, perm: u32) -> Result;

    /// Unmaps a block of transfer memory.
    ///
    /// **Note:** Syscall number 0x52.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcUnmapTransferMemory(tmem_handle: Handle, addr: *mut c_void, size: usize) -> Result;

    /// Creates an event and binds it to a specific hardware interrupt.
    ///
    /// **Note:** Syscall number 0x53.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateInterruptEvent(handle: *mut Handle, irqNum: u64, flag: u32) -> Result;

    /// Queries information about a certain virtual address, including its physical address.
    ///
    /// **Note:** Syscall number 0x54.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcQueryPhysicalAddress(out: *mut PhysicalMemoryInfo, virtaddr: u64) -> Result;

    /// Returns a virtual address mapped to a given IO range.
    ///
    /// **Note:** Syscall number 0x55.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    ///
    /// **Warning:** Only exists on [10.0.0+]. For older versions use [`svcLegacyQueryIoMapping`].
    pub fn svcQueryMemoryMapping(virtaddr: *mut u64, out_size: *mut u64, physaddr: u64, size: u64) -> Result;

    /// Returns a virtual address mapped to a given IO range.
    ///
    /// **Note:** Syscall number 0x55.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    ///
    /// **Warning:** Only exists on [1.0.0-9.2.0]. For newer versions use [`svcQueryMemoryMapping`].
    pub fn svcLegacyQueryIoMapping(virtaddr: *mut u64, physaddr: u64, size: u64) -> Result;

    /// Creates a virtual address space for binding device address spaces.
    ///
    /// **Note:** Syscall number 0x56.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateDeviceAddressSpace(handle: *mut Handle, dev_addr: u64, dev_size: u64) -> Result;

    /// Attaches a device address space to a device.
    ///
    /// **Note:** Syscall number 0x57.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcAttachDeviceAddressSpace(device: u64, handle: Handle) -> Result;

    /// Detaches a device address space from a device.
    ///
    /// **Note:** Syscall number 0x58.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcDetachDeviceAddressSpace(device: u64, handle: Handle) -> Result;

    /// Maps an attached device address space to an userspace address.
    ///
    /// **Remark:** The userspace destination address must have the [`MemState_MapDeviceAllowed`] bit set.
    ///
    /// **Note:** Syscall number 0x59.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapDeviceAddressSpaceByForce(handle: Handle, proc_handle: Handle, map_addr: u64, dev_size: u64, dev_addr: u64, option: u32) -> Result;

    /// Maps an attached device address space to an userspace address.
    ///
    /// **Remark:** The userspace destination address must have the [`MemState_MapDeviceAlignedAllowed`] bit set.
    ///
    /// **Note:** Syscall number 0x5A.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapDeviceAddressSpaceAligned(handle: Handle, proc_handle: Handle, map_addr: u64, dev_size: u64, dev_addr: u64, option: u32) -> Result;

    /// Maps an attached device address space to an userspace address. [1.0.0-12.1.0]
    ///
    /// **Remark:** The userspace destination address must have the [`MemState_MapDeviceAlignedAllowed`] bit set.
    ///
    /// **Note:** Syscall number 0x5B.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapDeviceAddressSpace(out_mapped_size: *mut u64, handle: Handle, proc_handle: Handle, map_addr: u64, dev_size: u64, dev_addr: u64, perm: u32) -> Result;

    /// Unmaps an attached device address space from an userspace address.
    ///
    /// **Note:** Syscall number 0x5C.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcUnmapDeviceAddressSpace(handle: Handle, proc_handle: Handle, map_addr: u64, map_size: u64, dev_addr: u64) -> Result;

    /// Invalidates data cache for a virtual address range within a process.
    ///
    /// # Arguments
    /// - `address` (in): Address of region to invalidate.
    /// - `size` (in): Size of region to invalidate.
    ///
    /// **Note:** Syscall number 0x5D.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcInvalidateProcessDataCache(process: Handle, address: usize, size: usize) -> Result;

    /// Stores data cache for a virtual address range within a process.
    ///
    /// # Arguments
    /// - `address` (in): Address of region to store.
    /// - `size` (in): Size of region to store.
    ///
    /// **Note:** Syscall number 0x5E.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcStoreProcessDataCache(process: Handle, address: usize, size: usize) -> Result;

    /// Flushes data cache for a virtual address range within a process.
    ///
    /// # Arguments
    /// - `address` (in): Address of region to flush.
    /// - `size` (in): Size of region to flush.
    ///
    /// **Note:** Syscall number 0x5F.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcFlushProcessDataCache(process: Handle, address: usize, size: usize) -> Result;

    /// Debugs an active process.
    ///
    /// **Note:** Syscall number 0x60.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcDebugActiveProcess(debug: *mut Handle, processID: u64) -> Result;

    /// Breaks an active debugging session.
    ///
    /// **Note:** Syscall number 0x61.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcBreakDebugProcess(debug: Handle) -> Result;

    /// Terminates the process of an active debugging session.
    ///
    /// **Note:** Syscall number 0x62.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcTerminateDebugProcess(debug: Handle) -> Result;

    /// Gets an incoming debug event from a debugging session.
    ///
    /// **Note:** Syscall number 0x63.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetDebugEvent(event_out: *mut DebugEventInfo, debug: Handle) -> Result;

    /// Continues a debugging session.
    ///
    /// **Note:** Syscall number 0x64.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    ///
    /// **Warning:** Only exists on [3.0.0+]. For older versions use [`svcLegacyContinueDebugEvent`].
    pub fn svcContinueDebugEvent(debug: Handle, flags: u32, tid_list: *mut u64, num_tids: u32) -> Result;

    /// Continues a debugging session.
    ///
    /// **Note:** Syscall number 0x64.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    ///
    /// **Warning:** Only exists on [1.0.0-2.3.0]. For newer versions use [`svcContinueDebugEvent`].
    pub fn svcLegacyContinueDebugEvent(debug: Handle, flags: u32, threadID: u64) -> Result;

    /// Gets the context (dump the registers) of a thread in a debugging session.
    ///
    /// # Arguments
    /// - `ctx` (out): Output thread context (register dump).
    /// - `debug` (in): Debug handle.
    /// - `threadID` (in): ID of the thread to dump the context of.
    /// - `flags` (in): Register groups to select, combination of @ref RegisterGroup flags.
    ///
    /// **Note:** Syscall number 0x67.
    ///
    /// **Warning:** Official kernel will not dump any CPU GPR if the thread is currently executing a system call (except @ref svcBreak and @ref svcReturnFromException).
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetDebugThreadContext(ctx: *mut ThreadContext, debug: Handle, threadID: u64, flags: u32) -> Result;

    /// Gets the context (dump the registers) of a thread in a debugging session.
    ///
    /// # Arguments
    /// - `debug` (in): Debug handle.
    /// - `threadID` (in): ID of the thread to set the context of.
    /// - `ctx` (in): Input thread context (register dump).
    /// - `flags` (in): Register groups to select, combination of @ref RegisterGroup flags.
    ///
    /// **Note:** Syscall number 0x68.
    ///
    /// **Warning:** Official kernel will return an error if the thread is currently executing a system call (except @ref svcBreak and @ref svcReturnFromException).
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcSetDebugThreadContext(debug: Handle, threadID: u64, ctx: *const ThreadContext, flags: u32) -> Result;

    /// Retrieves a list of all running processes.
    ///
    /// **Note:** Syscall number 0x65.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetProcessList(num_out: *mut s32, pids_out: *mut u64, max_pids: u32) -> Result;

    /// Retrieves a list of all threads for a debug handle (or zero).
    ///
    /// **Note:** Syscall number 0x66.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetThreadList(num_out: *mut s32, tids_out: *mut u64, max_tids: u32, debug: Handle) -> Result;

    /// Queries memory information from a process that is being debugged.
    ///
    /// **Note:** Syscall number 0x69.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcQueryDebugProcessMemory(meminfo_ptr: *mut MemoryInfo, pageinfo: *mut u32, debug: Handle, addr: u64) -> Result;

    /// Reads memory from a process that is being debugged.
    ///
    /// **Note:** Syscall number 0x6A.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcReadDebugProcessMemory(buffer: *mut c_void, debug: Handle, addr: u64, size: u64) -> Result;

    /// Writes to memory in a process that is being debugged.
    ///
    /// **Note:** Syscall number 0x6B.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcWriteDebugProcessMemory(debug: Handle, buffer: *const c_void, addr: u64, size: u64) -> Result;

    /// Sets one of the hardware breakpoints.
    ///
    /// **Note:** Syscall number 0x6C.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcSetHardwareBreakPoint(which: u32, flags: u64, value: u64) -> Result;

    /// Gets parameters from a thread in a debugging session.
    ///
    /// **Note:** Syscall number 0x6D.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetDebugThreadParam(out_64: *mut u64, out_32: *mut u32, debug: Handle, threadID: u64, param: DebugThreadParam) -> Result;

    /// Retrieves privileged information about the system, or a certain kernel object.
    ///
    /// # Arguments
    /// - `out` (out): Variable to which store the information.
    /// - `id0` (in): First ID of the property to retrieve.
    /// - `handle` (in): Handle of the object to retrieve information from, or [`INVALID_HANDLE`] to retrieve information about the system.
    /// - `id1` (in): Second ID of the property to retrieve.
    ///
    /// **Remark:** The full list of property IDs can be found on the [switchbrew.org wiki](https://switchbrew.org/wiki/SVC#svcGetSystemInfo).
    ///
    /// **Note:** Syscall number 0x6F.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetSystemInfo(out: *mut u64, id0: u64, handle: Handle, id1: u64) -> Result;

    /// Creates a port.
    ///
    /// **Note:** Syscall number 0x70.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreatePort(portServer: *mut Handle, portClient: *mut Handle, max_sessions: s32, is_light: bool, name: *const c_char) -> Result;

    /// Manages a named port.
    ///
    /// **Note:** Syscall number 0x71.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcManageNamedPort(portServer: *mut Handle, name: *const c_char, maxSessions: s32) -> Result;

    /// Manages a named port.
    ///
    /// **Note:** Syscall number 0x72.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcConnectToPort(session: *mut Handle, port: Handle) -> Result;

    /// Sets the memory permissions for the specified memory with the supplied process handle.
    ///
    /// # Arguments
    /// - `proc` (in): Process handle.
    /// - `addr` (in): Address of the memory.
    /// - `size` (in): Size of the memory.
    /// - `perm` (in): Permissions (see [`Permission`]).
    ///
    /// **Remark:** This returns an error (0xD801) when \p perm is >0x5, hence -WX and RWX are not allowed.
    ///
    /// **Note:** Syscall number 0x73.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcSetProcessMemoryPermission(proc: Handle, addr: u64, size: u64, perm: u32) -> Result;

    /// Maps the src address from the supplied process handle into the current process.
    ///
    /// # Arguments
    /// - `dst` (in): Address to which map the memory in the current process.
    /// - `proc` (in): Process handle.
    /// - `src` (in): Source mapping address.
    /// - `size` (in): Size of the memory.
    ///
    /// **Remark:** This allows mapping code and rodata with RW- permission.
    ///
    /// **Note:** Syscall number 0x74.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapProcessMemory(dst: *mut c_void, proc: Handle, src: u64, size: u64) -> Result;

    /// Undoes the effects of [`svcMapProcessMemory`].
    ///
    /// # Arguments
    /// - `dst` (in): Destination mapping address
    /// - `proc` (in): Process handle.
    /// - `src` (in): Address of the memory in the process.
    /// - `size` (in): Size of the memory.
    ///
    /// **Remark:** This allows mapping code and rodata with RW- permission.
    ///
    /// **Note:** Syscall number 0x75.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcUnmapProcessMemory(dst: *mut c_void, proc: Handle, src: u64, size: u64) -> Result;

    /// Equivalent to [`svcQueryMemory`], for another process.
    ///
    /// # Arguments
    /// - `meminfo_ptr` (out): [`MemoryInfo`] structure which will be filled in.
    /// - `pageinfo` (out): Page information which will be filled in.
    /// - `proc` (in): Process handle.
    /// - `addr` (in): Address to query.
    ///
    /// **Note:** Syscall number 0x76.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcQueryProcessMemory(meminfo_ptr: *mut MemoryInfo, pageinfo: *mut u32, proc: Handle, addr: u64) -> Result;

    /// Maps normal heap in a certain process as executable code (used when loading NROs).
    ///
    /// # Arguments
    /// - `proc` (in): Process handle (cannot be [`CUR_PROCESS_HANDLE`]).
    /// - `dst` (in): Destination mapping address.
    /// - `src` (in): Source mapping address.
    /// - `size` (in): Size of the mapping.
    ///
    /// **Note:** Syscall number 0x77.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcMapProcessCodeMemory(proc: Handle, dst: u64, src: u64, size: u64) -> Result;

    /// Undoes the effects of [`svcMapProcessCodeMemory`].
    ///
    /// # Arguments
    /// - `proc` (in): Process handle (cannot be [`CUR_PROCESS_HANDLE`]).
    /// - `dst` (in): Destination mapping address.
    /// - `src` (in): Source mapping address.
    /// - `size` (in): Size of the mapping.
    ///
    /// **Note:** Syscall number 0x78.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcUnmapProcessCodeMemory(proc: Handle, dst: u64, src: u64, size: u64) -> Result;

    /// Creates a new process.
    ///
    /// **Note:** Syscall number 0x79.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateProcess(out: *mut Handle, proc_info: *const c_void, caps: *const u32, cap_num: u64) -> Result;

    /// Starts executing a freshly created process.
    ///
    /// **Note:** Syscall number 0x7A.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcStartProcess(proc: Handle, main_prio: s32, default_cpu: s32, stack_size: u32) -> Result;

    /// Terminates a running process.
    ///
    /// **Note:** Syscall number 0x7B.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcTerminateProcess(proc: Handle) -> Result;

    /// Gets a [`ProcessInfoType`] for a process.
    ///
    /// **Note:** Syscall number 0x7C.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcGetProcessInfo(out: *mut s64, proc: Handle, which: ProcessInfoType) -> Result;

    /// Creates a new Resource Limit handle.
    ///
    /// **Note:** Syscall number 0x7D.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCreateResourceLimit(out: *mut Handle) -> Result;

    /// Sets the value for a [`LimitableResource`] for a Resource Limit handle.
    ///
    /// **Note:** Syscall number 0x7E.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcSetResourceLimitLimitValue(reslimit: Handle, which: LimitableResource, value: u64) -> Result;

    /// Calls a secure monitor function (TrustZone, EL3).
    ///
    /// # Arguments
    /// - `regs`: Arguments to pass to the secure monitor.
    ///
    /// **Note:** Syscall number 0x7F.
    ///
    /// **Warning:** This is a privileged syscall. Use `envIsSyscallHinted` to check if it is available.
    pub fn svcCallSecureMonitor(regs: *mut SecmonArgs);

    /// Maps new insecure memory at the desired address. [15.0.0+]
    ///
    /// **Note:** Syscall number 0x90.
    pub fn svcMapInsecurePhysicalMemory(address: *mut c_void, size: u64) -> Result;

    /// Undoes the effects of `svcMapInsecureMemory`. [15.0.0+]
    ///
    /// **Note:** Syscall number 0x91.
    pub fn svcUnmapInsecurePhysicalMemory(address: *mut c_void, size: u64) -> Result;
}

/// Waits on a single synchronization object, optionally with a timeout.
///
/// **Note:** Wrapper for [`svcWaitSynchronization`].
///
/// **Note:** This is the raw syscall, which can be cancelled by [`svcCancelSynchronization`] or other means. `waitSingleHandle` should normally be used instead.
///
/// # Safety
/// Same requirements as [`svcWaitSynchronization`].
#[inline(always)]
pub unsafe fn svcWaitSynchronizationSingle(handle: Handle, timeout: u64) -> Result {
    let mut tmp: s32 = 0;
    unsafe { svcWaitSynchronization(&mut tmp, &handle, 1, timeout) }
}
