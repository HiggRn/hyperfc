//! The LLVM module handles converting a HF AST to LLVM IR.

use std::ffi::CStr;
use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::transforms::pass_builder::*;

pub fn get_default_target_triple() -> CString {
    unsafe {
        let target_triple_ptr = LLVMGetDefaultTargetTriple();
        let target_triple = CStr::from_ptr(target_triple_ptr as *const _).to_owned();
        LLVMDisposeMessage(target_triple_ptr);
        target_triple
    }
}
