#[cfg(any(target_os = "macos"))]
pub mod macos;

#[cfg(any(target_os = "linux"))]
pub mod linux;

#[allow(dead_code)]
pub(crate) mod shared;
pub use shared::*;
