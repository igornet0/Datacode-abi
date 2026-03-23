//! ABI version and compatibility.

/// Contract version (major.minor). Two u16s — C/FFI compatible.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbiVersion {
    pub major: u16,
    pub minor: u16,
}

/// Current datacode-abi version. Bump major on incompatible ABI changes.
/// Minor 2+: root [`crate::module::AbiModuleDescriptor`], [`DATACODE_MODULE_ENTRY_SYMBOL`], class/global tables.
/// Minor 3+: [`Value::PluginOpaque`] for opaque plugin handles (`tag` + `id`).
pub const DATACODE_ABI_VERSION: AbiVersion = AbiVersion { major: 1, minor: 3 };

/// Compatible iff same major and module.minor <= vm.minor.
#[inline]
pub fn abi_compatible(module: &AbiVersion, vm: &AbiVersion) -> bool {
    module.major == vm.major && module.minor <= vm.minor
}
