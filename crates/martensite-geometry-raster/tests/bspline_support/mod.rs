//! Shared specialized spline contracts use actual geometry and independent expectations.
use super::spline_api;
pub mod fixtures;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
mod native;
mod state;
