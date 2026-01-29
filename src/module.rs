//! Native module entry point and descriptor.

use std::ffi::c_char;

use crate::version::AbiVersion;
use crate::vm_context::VmContext;

/// Module descriptor returned from `datacode_module()`.
#[repr(C)]
pub struct DatacodeModule {
    /// Module ABI version; must be compatible with VM (see `abi_compatible`).
    pub abi_version: AbiVersion,
    /// Module name (UTF-8, null-terminated), e.g. "telegram".
    pub name: *const c_char,
    /// VM calls this after ABI check; module registers functions/constants.
    pub register: extern "C" fn(*mut VmContext),
}

// Safe for use in OnceLock: module descriptor is initialized once and only read.
unsafe impl Send for DatacodeModule {}
unsafe impl Sync for DatacodeModule {}

/// Symbol name for the native module entry point.
pub const DATACODE_MODULE_SYMBOL: &str = "datacode_module";

/// Entry point signature: module exports a function with this name.
pub type DatacodeModuleFn = extern "C" fn() -> *const DatacodeModule;
