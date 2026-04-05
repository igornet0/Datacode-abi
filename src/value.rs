//! Value type on the VM ↔ module boundary. `#[repr(C)]` required for FFI.

use std::ffi::c_char;
use std::ffi::c_void;

/// Opaque handle for object/dict managed by VM or module.
pub type NativeHandle = *mut c_void;

/// ABI value. Pointers and strings from VM are valid for the duration of the native call.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    /// UTF-8, null-terminated. Not owned.
    Str(*const c_char),
    Null,
    /// Array elements. Pointer valid for the call.
    Array(*mut Value, usize),
    /// Opaque object (dict) handle.
    Object(NativeHandle),
    /// Opaque plugin-owned object; `tag`/`id` semantics are defined only by the plugin (VM core is domain-neutral).
    PluginOpaque { tag: u8, id: u64 },
    /// Tabular data: `headers_len` column names (`Str`), then `rows * cols` cell values row-major.
    /// Pointers valid for the duration of the native call.
    Table {
        headers: *mut Value,
        headers_len: usize,
        cells: *mut Value,
        rows: usize,
        cols: usize,
    },
    /// Raw bytes (e.g. `read_file_bin`); pointer valid for the duration of the native call.
    Bytes {
        ptr: *const u8,
        len: usize,
    },
}
