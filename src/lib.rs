//! # datacode-abi
//!
//! Minimal, stable, C-compatible contract between DataCode VM and external modules.
//! Types, version, and VM ↔ module boundary only. Single source of truth for the ABI.

pub mod version;
pub mod value;
pub mod error;
pub mod vm_context;
pub mod module;

pub use version::{AbiVersion, DATACODE_ABI_VERSION, abi_compatible};
pub use value::{Value as AbiValue, NativeHandle};
pub use error::DatacodeError;
pub use vm_context::{VmContext, NativeAbiFn};
pub use module::{DatacodeModule, DatacodeModuleFn, DATACODE_MODULE_SYMBOL};
