//! Error codes on the VM ↔ module boundary. VM maps them to try/catch.

/// Error codes passed from module to VM.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatacodeError {
    Ok = 0,
    TypeError = 1,
    RuntimeError = 2,
    Panic = 3,
}
