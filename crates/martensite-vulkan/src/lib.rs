//! Martensite's Vulkan 1.3 renderer with immutable submissions and offscreen captures.
//!
//! Backend-agnostic Graphic3d lowering and scene representation live in
//! `martensite-render-core` (#633) and are re-exported here for full source compatibility.
#![cfg_attr(test, allow(clippy::duplicate_mod))] // Unit tests share scene fixtures by path.

pub use martensite_render_core::*;

#[allow(unsafe_code)]
mod backend;
mod driver_factory;
mod thin_line;
#[path = "user_program/mod.rs"]
mod vulkan_user_program;

pub use backend::{VulkanOptions, VulkanRenderer, active_validation_layer};
pub use driver_factory::{VULKAN_DRIVER_NAME, VulkanDriverFactory};
pub use vulkan_user_program::phong_shading_program;

// Private library-test harness keeping Vulkan GPU and frozen-fixture unit tests in this crate.
#[cfg(test)]
extern crate self as martensite_vulkan;
#[cfg(test)]
mod capture_owned_fixture;
#[cfg(test)]
mod clipped_depth_partition;
#[cfg(test)]
mod clipped_depth_plane;
#[cfg(test)]
mod clipped_depth_polygon;
#[cfg(test)]
mod clipped_depth_reference_empty;
#[cfg(test)]
mod clipped_depth_shared_reference;
#[cfg(test)]
mod driver_tests;
#[cfg(test)]
mod fixed_grid_point_gpu_tests;
#[cfg(test)]
mod gradient_tests;
#[cfg(test)]
mod hatch_surface;
#[cfg(test)]
mod immediate_redraw_fixture;
#[cfg(test)]
mod point_surface_texture_tests;
#[cfg(test)]
mod point_uv_pixels_tests;
#[cfg(all(test, feature = "vertex-output-diagnostics"))]
mod vertex_output_fixture;
#[cfg(test)]
mod view_capture_owned_tests;
#[cfg(test)]
mod window_depth_float64_fixture;
#[cfg(test)]
mod window_depth_float64_tests;
#[cfg(test)]
mod window_depth_plane;
#[cfg(test)]
mod window_depth_projection;
