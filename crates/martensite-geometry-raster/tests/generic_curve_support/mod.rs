//! Shared generic curve contracts use source facades or independent owned drawables.
use super::curve_api;
pub(super) mod fixtures;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
mod native;
mod state_cases;
