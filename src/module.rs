//! Native module entry point and descriptor.

use std::ffi::c_char;

use crate::version::AbiVersion;
use crate::vm_context::{NativeAbiFn, VmContext};

/// One exported native function in a static table (`#[repr(C)]`).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AbiExport {
    /// UTF-8 export name, null-terminated; not owned by VM.
    pub name: *const c_char,
    /// Trampoline with ABI [`NativeAbiFn`].
    pub func: NativeAbiFn,
    /// Arity (argument count); `usize::MAX` may mean varargs in future.
    pub arity: usize,
    /// Reserved flags (0 = none).
    pub flags: u32,
}

/// Legacy nested table: **only** flat function exports (used inside [`DatacodeModule`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AbiExportTable {
    pub exports: *const AbiExport,
    pub exports_len: usize,
}

/// Class export: name + method table (same row shape as [`AbiExport`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AbiClassDescriptor {
    pub name: *const c_char,
    pub methods: *const AbiExport,
    pub methods_len: usize,
}

/// Module-level global binding (getter trampoline).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AbiGlobalDescriptor {
    pub name: *const c_char,
    pub getter: NativeAbiFn,
    pub flags: u32,
}

/// Root module descriptor returned from [`DATACODE_MODULE_ENTRY_SYMBOL`] (production path).
///
/// VM reads this **only** (no `register` callback). [`AbiVersion`] is checked before use.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AbiModuleDescriptor {
    pub abi_version: AbiVersion,
    /// Module name (UTF-8, null-terminated); may be null.
    pub name: *const c_char,
    pub functions: *const AbiExport,
    pub functions_len: usize,
    pub classes: *const AbiClassDescriptor,
    pub classes_len: usize,
    pub globals: *const AbiGlobalDescriptor,
    pub globals_len: usize,
}

/// Layout for modules built against ABI **1.0** only (`abi_version.minor == 0`): three fields.
/// VM must interpret `module_ptr` as this type when `abi_version.minor == 0`.
#[repr(C)]
pub struct DatacodeModuleLegacy {
    pub abi_version: AbiVersion,
    pub name: *const c_char,
    /// VM calls this after ABI check; module registers functions via `VmContext::register_native`.
    pub register: extern "C" fn(*mut VmContext),
}

/// Module descriptor returned from [`DATACODE_MODULE_SYMBOL`] for ABI **1.1+** (`minor >= 1`).
///
/// - If `export_table` is non-null, VM reads exports from [`AbiExportTable`] only and **does not** call `register`.
/// - If `export_table` is null and `register` is non-null, VM uses the legacy registration path (capture `register_native` calls).
/// - If both are null/empty, load fails.
#[repr(C)]
pub struct DatacodeModule {
    pub abi_version: AbiVersion,
    pub name: *const c_char,
    /// Static flat export table; null to use `register` instead.
    pub export_table: *const AbiExportTable,
    /// Null = descriptor-only module; Some = registration callback (optional with 1.1+).
    pub register: Option<extern "C" fn(*mut VmContext)>,
}

unsafe impl Send for DatacodeModule {}
unsafe impl Sync for DatacodeModule {}
unsafe impl Send for DatacodeModuleLegacy {}
unsafe impl Sync for DatacodeModuleLegacy {}

/// Symbol for [`DatacodeModule`] entry (legacy / transitional).
pub const DATACODE_MODULE_SYMBOL: &str = "datacode_module";

/// Preferred production entry: returns root [`AbiModuleDescriptor`] directly.
pub const DATACODE_MODULE_ENTRY_SYMBOL: &str = "datacode_module_entry";

/// Entry: `datacode_module()` → [`DatacodeModule`] wrapper.
pub type DatacodeModuleFn = extern "C" fn() -> *const DatacodeModule;

/// Entry: `datacode_module_entry()` → root descriptor (no wrapper).
pub type DatacodeModuleEntryFn = extern "C" fn() -> *const AbiModuleDescriptor;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn export_table_is_two_usize_sized_fields() {
        assert_eq!(size_of::<AbiExportTable>(), 2 * size_of::<usize>());
        assert!(align_of::<AbiExportTable>() <= align_of::<usize>() * 2);
    }

    #[test]
    fn module_descriptor_has_version_and_tables() {
        assert!(size_of::<AbiModuleDescriptor>() >= size_of::<AbiVersion>());
    }
}
