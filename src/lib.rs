#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![recursion_limit="1000"]

extern crate libc;

use libc::size_t;
use std::os::raw::{c_void, c_int, c_uint, c_char};

#[cfg(not(any(target_arch="x86_64",target_arch="i686")))]
pub mod placeholders {
    include!(concat!(env!("OUT_DIR"), "/placeholders.rs"));
}

#[cfg(target_arch="x86_64")]
pub mod placeholders {
    pub type detail_data = [u64; 185];
    pub type arm64_op_data = [u64; 2];
    pub type arm_op_data = [u64; 2];
    pub type mips_op_data = [u64; 2];
    pub type ppc_op_data = [u32; 3];
    pub type sparc_op_data = [u32; 2];
    pub type sysz_op_data = [u64; 3];
    pub type x86_op_data = [u64; 3];
    pub type xcore_op_data = [u32; 3];
}

#[cfg(target_arch="i686")]
pub mod placeholders {
    pub type detail_data = [u32; 333];
    pub type arm64_op_data = [u32; 3];
    pub type arm_op_data = [u64; 2];
    pub type mips_op_data = [u32; 3];
    pub type ppc_op_data = [u32; 3];
    pub type sparc_op_data = [u32; 2];
    pub type sysz_op_data = [u32; 5];
    pub type x86_op_data = [u32; 6];
    pub type xcore_op_data = [u32; 3];
}

#[macro_use]
mod macros;

pub mod arm;

pub mod arm64;

pub mod mips;

pub mod ppc;

pub mod sparc;

pub mod sysz;

pub mod x86;

pub mod xcore;


// automatically generated by rust-bindgen
// then heavily modified

pub type csh = size_t;

fake_enum! {
    /// Architecture type
    pub enum cs_arch {
        /// ARM architecture (including Thumb, Thumb-2)
        CS_ARCH_ARM = 0,
        /// ARM-64, also called AArch64
        CS_ARCH_ARM64 = 1,
        /// Mips architecture
        CS_ARCH_MIPS = 2,
        /// X86 architecture (including x86 & x86-64)
        CS_ARCH_X86 = 3,
        /// PowerPC architecture
        CS_ARCH_PPC = 4,
        /// Sparc architecture
        CS_ARCH_SPARC = 5,
        /// SystemZ architecture
        CS_ARCH_SYSZ = 6,
        /// XCore architecture
        CS_ARCH_XCORE = 7,
        CS_ARCH_MAX = 8,
        /// All architecture for `cs_support`
        CS_ARCH_ALL = 0xFFFF,
        /// Support value to verify diet mode of the engine.
        CS_SUPPORT_DIET = CS_ARCH_ALL+1,
        /// Support value to verify X86 reduce mode of the engine.
        CS_SUPPORT_X86_REDUCE = CS_ARCH_ALL+2,
    }
}

fake_enum! {
    /// Mode type (architecture variant, not all combination are possible)
    pub enum cs_mode {
        /// Little-endian mode (default mode)
        CS_MODE_LITTLE_ENDIAN = 0,
        /// 32-bit ARM
        CS_MODE_ARM = 0,
        /// 16-bit mode X86
        CS_MODE_16 = 1 << 1,
        /// 32-bit mode X86
        CS_MODE_32 = 1 << 2,
        /// 64-bit mode X86
        CS_MODE_64 = 1 << 3,
        /// ARM's Thumb mode, including Thumb-2
        CS_MODE_THUMB = 1 << 4,
        /// ARM's Cortex-M series
        CS_MODE_MCLASS = 1 << 5,
        /// ARMv8 A32 encodings for ARM
        CS_MODE_V8 = 1 << 6,
        /// MicroMips mode (MIPS)
        CS_MODE_MICRO = 1 << 4,
        /// Mips III ISA
        CS_MODE_MIPS3 = 1 << 5,
        /// Mips32r6 ISA
        CS_MODE_MIPS32R6 = 1 << 6,
        /// General Purpose Registers are 64-bit wide (MIPS)
        CS_MODE_MIPSGP64 = 1 << 7,
        /// SparcV9 mode (Sparc)
        CS_MODE_V9 = 1 << 31,
        /// big-endian mode
        CS_MODE_BIG_ENDIAN = 1 << 31,
        /// Mips32 ISA (Mips)
        CS_MODE_MIPS32 = CS_MODE_32,
        /// Mips64 ISA (Mips)
        CS_MODE_MIPS64 = CS_MODE_64,
    }
}

pub type cs_malloc_t = Option<extern "C" fn(size: size_t) -> *mut c_void>;
pub type cs_calloc_t = Option<extern "C" fn(nmemb: size_t, size: size_t) -> *mut c_void>;
pub type cs_realloc_t = Option<unsafe extern "C" fn(ptr: *mut c_void, size: size_t) -> *mut c_void>;
pub type cs_free_t = Option<unsafe extern "C" fn(ptr: *mut c_void)>;

pub type cs_vsnprintf_t = Option<unsafe extern "C" fn()>;
// pub type cs_vsnprintf_t = Option<unsafe extern "C" fn(str: *mut c_char,
//                                                       size: size_t,
//                                                       format: *const c_char,
//                                                       ap: va_list)
//                                                       -> c_int>;

#[repr(C)]
pub struct cs_opt_mem {
    pub malloc: cs_malloc_t,
    pub calloc: cs_calloc_t,
    pub realloc: cs_realloc_t,
    pub free: cs_free_t,
    pub vsnprintf: cs_vsnprintf_t,
}
impl ::std::default::Default for cs_opt_mem {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

fake_enum! {
    /// Runtime option for the disassembled engine
    pub enum cs_opt_type {
        /// Assembly output syntax
        CS_OPT_SYNTAX = 1,
        /// Break down instruction structure into details
        CS_OPT_DETAIL,
        /// Change engine's mode at run-time
        CS_OPT_MODE,
        /// User-defined dynamic memory related functions
        CS_OPT_MEM,
        /// Skip data when disassembling. Then engine is in SKIPDATA mode.
        CS_OPT_SKIPDATA,
        /// Setup user-defined function for SKIPDATA option
        CS_OPT_SKIPDATA_SETUP,
    }
}

fake_enum! {
    /// Runtime option value (associated with option type above)
    pub enum cs_opt_value {
        /// Turn OFF an option - default option of CS_OPT_DETAIL, CS_OPT_SKIPDATA.
        CS_OPT_OFF = 0,
        /// Turn ON an option (CS_OPT_DETAIL, CS_OPT_SKIPDATA).
        CS_OPT_ON = 3,
        /// Default asm syntax (CS_OPT_SYNTAX).
        CS_OPT_SYNTAX_DEFAULT = 0,
        /// X86 Intel asm syntax - default on X86 (CS_OPT_SYNTAX).
        CS_OPT_SYNTAX_INTEL,
        /// X86 ATT asm syntax (CS_OPT_SYNTAX).
        CS_OPT_SYNTAX_ATT,
        /// Prints register name with only number (CS_OPT_SYNTAX)
        CS_OPT_SYNTAX_NOREGNAME,
    }
}

fake_enum! {
    /// Common instruction operand types - to be consistent across all architectures.
    pub enum cs_op_type {
        /// Uninitialized/invalid operand.
        CS_OP_INVALID = 0,
        /// Register operand.
        CS_OP_REG = 1,
        /// Immediate operand.
        CS_OP_IMM = 2,
        /// Memory operand.
        CS_OP_MEM = 3,
        /// Floating-Point operand.
        CS_OP_FP = 4,
    }
}

fake_enum! {
    /// Common instruction groups - to be consistent across all architectures.
    pub enum cs_group_type {
        /// uninitialized/invalid group.
        CS_GRP_INVALID = 0,
        /// all jump instructions (conditional+direct+indirect jumps)
        CS_GRP_JUMP,
        /// all call instructions
        CS_GRP_CALL,
        /// all return instructions
        CS_GRP_RET,
        /// all interrupt instructions (int+syscall)
        CS_GRP_INT,
        /// all interrupt return instructions
        CS_GRP_IRET,
    }
}

pub type cs_skipdata_cb_t = Option<unsafe extern "C" fn(code: *const u8,
                                                        code_size: size_t,
                                                        offset: size_t,
                                                        user_data: *mut c_void)
                                                        -> size_t>;
#[repr(C)]
pub struct cs_opt_skipdata {
    pub mnemonic: *const c_char,
    pub callback: cs_skipdata_cb_t,
    pub user_data: *mut c_void,
}
impl ::std::default::Default for cs_opt_skipdata {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct cs_detail {
    pub regs_read: [u8; 12usize],
    pub regs_read_count: u8,
    pub regs_write: [u8; 20usize],
    pub regs_write_count: u8,
    pub groups: [u8; 8usize],
    pub groups_count: u8,
    data: placeholders::detail_data,
}

impl cs_detail {
    pub unsafe fn x86(&self) -> &x86::cs_x86 {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn arm64(&self) -> &arm64::cs_arm64 {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn arm(&self) -> &arm::cs_arm {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn mips(&self) -> &mips::cs_mips {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn ppc(&self) -> &ppc::cs_ppc {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn sparc(&self) -> &sparc::cs_sparc {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn sysz(&self) -> &sysz::cs_sysz {
        ::std::mem::transmute(&self.data)
    }
    pub unsafe fn xcore(&self) -> &xcore::cs_xcore {
        ::std::mem::transmute(&self.data)
    }
}

#[repr(C)]
pub struct cs_insn {
    pub id: c_uint,
    pub address: u64,
    pub size: u16,
    pub bytes: [u8; 16usize],
    pub mnemonic: [c_char; 32usize],
    pub op_str: [c_char; 160usize],
    pub detail: *mut cs_detail,
}

fake_enum! {
    /// All type of errors encountered by Capstone API.
    /// These are values returned by cs_errno()
    pub enum cs_err {
        /// No error: everything was fine
        CS_ERR_OK = 0,
        /// Out-Of-Memory error: cs_open(), cs_disasm(), cs_disasm_iter()
        CS_ERR_MEM,
        /// Unsupported architecture: cs_open()
        CS_ERR_ARCH,
        /// Invalid handle: cs_op_count(), cs_op_index()
        CS_ERR_HANDLE,
        /// Invalid csh argument: cs_close(), cs_errno(), cs_option()
        CS_ERR_CSH,
        /// Invalid/unsupported mode: cs_open()
        CS_ERR_MODE,
        /// Invalid/unsupported option: cs_option()
        CS_ERR_OPTION,
        /// Information is unavailable because detail option is OFF
        CS_ERR_DETAIL,
        /// Dynamic memory management uninitialized (see CS_OPT_MEM)
        CS_ERR_MEMSETUP,
        /// Unsupported version (bindings)
        CS_ERR_VERSION,
        /// Access irrelevant data in "diet" engine
        CS_ERR_DIET,
        /// Access irrelevant data for "data" instruction in SKIPDATA mode
        CS_ERR_SKIPDATA,
        /// X86 AT&T syntax is unsupported (opt-out at compile time)
        CS_ERR_X86_ATT,
        /// X86 Intel syntax is unsupported (opt-out at compile time)
        CS_ERR_X86_INTEL,
    }
}

#[link(name = "capstone", kind = "dylib")]
extern "C" {
    pub fn cs_version(major: *mut c_int, minor: *mut c_int) -> c_uint;
    pub fn cs_support(query: c_int) -> u8;
    pub fn cs_open(arch: cs_arch, mode: cs_mode, handle: *mut csh) -> cs_err;
    pub fn cs_close(handle: *mut csh) -> cs_err;
    pub fn cs_option(handle: csh, _type: cs_opt_type, value: size_t) -> cs_err;
    pub fn cs_errno(handle: csh) -> cs_err;
    pub fn cs_strerror(code: cs_err) -> *const c_char;
    pub fn cs_disasm(handle: csh,
                     code: *const u8,
                     code_size: size_t,
                     address: u64,
                     count: size_t,
                     insn: *mut *mut cs_insn)
                     -> size_t;
    pub fn cs_free(insn: *mut cs_insn, count: size_t);
    pub fn cs_malloc(handle: csh) -> *mut cs_insn;
    pub fn cs_disasm_iter(handle: csh,
                          code: *mut *const u8,
                          size: *mut size_t,
                          address: *mut u64,
                          insn: *mut cs_insn)
                          -> u8;
    pub fn cs_reg_name(handle: csh, reg_id: c_uint) -> *const c_char;
    pub fn cs_insn_name(handle: csh, insn_id: c_uint) -> *const c_char;
    pub fn cs_group_name(handle: csh, group_id: c_uint) -> *const c_char;
    pub fn cs_insn_group(handle: csh, insn: *const cs_insn, group_id: c_uint) -> u8;
    pub fn cs_reg_read(handle: csh, insn: *const cs_insn, reg_id: c_uint) -> u8;
    pub fn cs_reg_write(handle: csh, insn: *const cs_insn, reg_id: c_uint) -> u8;
    pub fn cs_op_count(handle: csh, insn: *const cs_insn, op_type: c_uint) -> c_int;
    pub fn cs_op_index(handle: csh,
                       insn: *const cs_insn,
                       op_type: c_uint,
                       position: c_uint)
                       -> c_int;
}
