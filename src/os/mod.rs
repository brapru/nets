#[cfg(any(target_os = "macos"))]
pub mod macos;

#[allow(dead_code)]
pub(crate) mod shared;
pub use shared::*;
