//! Shared source/owned surface contracts preserve geometry, controls and pixels.
use super::surface_api;
pub mod geometry;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
mod native;
pub mod state;
