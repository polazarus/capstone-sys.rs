extern crate capstone_sys;

use std::os::raw::c_char;
use capstone_sys::*;
use capstone_sys::x86::*;

const X86_CODE16: &'static [u8] = b"\x8d\x4c\x32\x08\x01\xd8\x81\xc6\x34\x12\x00\x00\x05\x23\x01\x00\x00\x36\x8b\x84\x91\x23\x01\x00\x00\x41\x8d\x84\x39\x89\x67\x00\x00\x8d\x87\x89\x67\x00\x00\xb4\xc6";
const X86_CODE32: &'static [u8] = b"\x8d\x4c\x32\x08\x01\xd8\x81\xc6\x34\x12\x00\x00\x05\x23\x01\x00\x00\x36\x8b\x84\x91\x23\x01\x00\x00\x41\x8d\x84\x39\x89\x67\x00\x00\x8d\x87\x89\x67\x00\x00\xb4\xc6";
const X86_CODE64: &'static [u8] = b"\x55\x48\x8b\x05\xb8\x13\x00\x00";

fn to_str(raw: &[c_char]) -> &str {
    unsafe {
        ::std::ffi::CStr::from_ptr(raw.as_ptr()).to_str().expect("invalid UTF-8")
    }
}


fn generic(arch: cs_arch, mode: cs_mode, code: &[u8], expected_size: size_t, expected_first: (&str, &str, &[x86_op_type])) {
    let mut handle: csh = 0;
    let err = unsafe { cs_open(arch, mode, &mut handle) };
    assert_eq!(err, CS_ERR_OK);

    // let err = unsafe { cs_option(handle, CS_OPT_SYNTAX, CS_OPT_SYNTAX_ATT as size_t) };
    // assert_eq!(err, CS_ERR_OK);

    let err = unsafe { cs_option(handle, CS_OPT_DETAIL, CS_OPT_ON as size_t) };
    assert_eq!(err, CS_ERR_OK);

    let mut instrs: *mut cs_insn = ::std::ptr::null_mut();
    let size = unsafe { cs_disasm(handle, code.as_ptr(), code.len(), 0, 0 /* read as much as possible */, &mut instrs) };
    let err = unsafe { cs_errno(handle) };
    assert_eq!(err, CS_ERR_OK);
    assert_eq!(size, expected_size);
    assert!(!instrs.is_null());

    let instr = unsafe { &*instrs };
    let (mnemonic, op_str, ops) = expected_first;
    assert_eq!(to_str(&instr.mnemonic), mnemonic);
    assert_eq!(to_str(&instr.op_str), op_str);
    assert!(!instr.detail.is_null());
    let detail = unsafe { (&*instr.detail).x86() };
    assert_eq!(detail.op_count, ops.len() as u8);
    for i in 0..ops.len() {
        assert_eq!(detail.operands[i].typ, ops[i]);
    }

    unsafe {
        cs_close(&mut handle);
    }
}

#[test]
fn test_x86_16() {
    generic(CS_ARCH_X86, CS_MODE_16, X86_CODE16, 15, ("lea", "cx, word ptr [si + 0x32]", &[X86_OP_REG, X86_OP_MEM]));
}

#[test]
fn test_x86_32() {
    generic(CS_ARCH_X86, CS_MODE_32, X86_CODE32, 9, ("lea", "ecx, dword ptr [edx + esi + 8]", &[X86_OP_REG, X86_OP_MEM]));
}

#[test]
fn test_x86_64() {
    generic(CS_ARCH_X86, CS_MODE_64, X86_CODE64, 2, ("push", "rbp", &[X86_OP_REG]));
}
