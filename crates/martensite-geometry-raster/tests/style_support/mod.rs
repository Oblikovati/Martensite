//! Shared compatibility/owned helper contracts with independently derived expectations.
use super::style_api;
mod curvature_cases;
mod finite_cases;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
mod native;
mod pole_cases;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
mod search_cases;
mod style_cases;
