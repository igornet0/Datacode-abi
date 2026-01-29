//! ABI version and compatibility.

/// Contract version (major.minor). Two u16s — C/FFI compatible.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbiVersion {
    pub major: u16,
    pub minor: u16,
}

/// Current datacode-abi version. Bump major on incompatible ABI changes.
pub const DATACODE_ABI_VERSION: AbiVersion = AbiVersion { major: 1, minor: 0 };

/// Compatible iff same major and module.minor <= vm.minor.
#[inline]
pub fn abi_compatible(module: &AbiVersion, vm: &AbiVersion) -> bool {
    module.major == vm.major && module.minor <= vm.minor
}
