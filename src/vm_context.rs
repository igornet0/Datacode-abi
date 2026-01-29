//! Bridge from module to VM: alloc, throw_error, register_native only.

use std::ffi::c_char;

use crate::error::DatacodeError;
use crate::value::Value as AbiValue;

/// Native function signature exported by a module via ABI.
pub type NativeAbiFn = extern "C" fn(*mut VmContext, *const AbiValue, usize) -> AbiValue;

/// Context passed to `register`. Module uses only these callbacks.
#[repr(C)]
pub struct VmContext {
    /// Allocate via VM allocator. Returns null on failure.
    pub alloc: extern "C" fn(size: usize) -> *mut u8,
    /// Report error to VM; VM turns it into try/catch. msg: UTF-8, null-terminated; may be null.
    pub throw_error: extern "C" fn(code: DatacodeError, msg: *const c_char),
    /// Register a native function. Called by module from register().
    pub register_native: extern "C" fn(*mut VmContext, name: *const c_char, func: NativeAbiFn),
}
