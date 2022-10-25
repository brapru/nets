#[cfg(any(target_os = "macos"))]
pub mod macos;

pub(crate) mod shared;
pub use shared::*;
