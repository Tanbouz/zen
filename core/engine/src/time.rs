//! Platform-agnostic time types for WASM compatibility.
//!
//! WASM environments don't have std::time::Instant available since it requires
//! OS-level system calls. We use web-time as a drop-in replacement that works
//! in browser environments.

#[cfg(target_arch = "wasm32")]
pub use web_time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
pub use std::time::{Duration, Instant};
