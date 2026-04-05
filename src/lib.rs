//! # datacode-abi
//!
//! Minimal, stable, C-compatible contract between DataCode VM and external modules.
//! Types, version, and VM ↔ module boundary only. Single source of truth for the ABI.
//!
//! ## Versioning
//!
//! [`AbiVersion`] is `{ major, minor }`. The VM checks compatibility with
//! [`DATACODE_ABI_VERSION`] via [`abi_compatible`]. Bump **minor** for additive layout changes
//! that old loaders can still tolerate; bump **major** for breaking changes.
//!
//! ## Entry symbol
//!
//! Native modules should export **`datacode_module_entry`** ([`DATACODE_MODULE_ENTRY_SYMBOL`]),
//! signature [`DatacodeModuleEntryFn`]: `extern "C" fn() -> *const AbiModuleDescriptor`.
//! The VM checks [`AbiModuleDescriptor::abi_version`] on that descriptor before reading tables.
//!
//! Transitional: **`datacode_module`** ([`DATACODE_MODULE_SYMBOL`]) returning [`DatacodeModule`]
//! is still supported.
//!
//! ## Invocation ABI (natives)
//!
//! Natives use [`NativeAbiFn`]: `(ctx, args_ptr, argc) -> AbiValue` — heap-style argument buffer
//! of [`AbiValue`] (see [`value`]); VM bridges to/from internal values via `abi_bridge`.
//! ABI **1.4+** adds [`AbiValue::Table`](crate::AbiValue::Table) for VM `Table` → native modules (see [`version::DATACODE_ABI_VERSION`]).
//! ABI **1.6+** adds [`AbiValue::Bytes`](crate::AbiValue::Bytes) for dense binary buffers.
//!
//! ## Descriptor vs `register` (ABI 1.1+)
//!
//! After the ABI check, the VM branches on [`DatacodeModule::abi_version`]:
//!
//! - **`minor == 0`** — legacy **three-field** layout [`DatacodeModuleLegacy`]: only
//!   `register` is used; the VM calls it and collects `(name, NativeAbiFn)` from the fake
//!   `VmContext`. There is no static descriptor.
//! - **`minor >= 1`** — **four-field** [`DatacodeModule`]:
//!   - If [`DatacodeModule::export_table`] is **non-null**, exports are read **only** from
//!     [`AbiExportTable`] / [`AbiExport`]; **`register` is not called**.
//!   - If `export_table` is null and [`DatacodeModule::register`] is **Some**, the VM uses the
//!     same capture path as legacy (callback registration).
//!   - If both are absent, load fails.
//!
//! The `datacode_sdk` crate provides `define_module!`, `define_module_descriptor!`, and
//! `define_module_entry!` (root descriptor + `datacode_module_entry`).

pub mod version;
pub mod value;
pub mod error;
pub mod vm_context;
pub mod module;

pub use version::{AbiVersion, DATACODE_ABI_VERSION, abi_compatible};
pub use value::{Value as AbiValue, NativeHandle};
pub use error::DatacodeError;
pub use vm_context::{VmContext, NativeAbiFn};
pub use module::{
    AbiClassDescriptor, AbiExport, AbiExportTable, AbiGlobalDescriptor, AbiModuleDescriptor,
    AbiModuleDescriptorV4, AbiNativeParamMeta, AbiOpaqueTypeDescriptor, AbiPluginHooksDescriptor,
    ABI_NATIVE_PARAM_META_SUPPORTS_NAMED_ARGS,
    DatacodeModule, DatacodeModuleEntryFn, DatacodeModuleFn, DatacodeModuleLegacy,
    DATACODE_MODULE_ENTRY_SYMBOL, DATACODE_MODULE_SYMBOL,
};
