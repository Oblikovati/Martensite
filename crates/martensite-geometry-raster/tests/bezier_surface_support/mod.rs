//! Shared specialized surface fixtures bind actual geometry, controls and native pixels.
use super::surface_api;
pub mod fixtures;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
mod native;
mod state;
