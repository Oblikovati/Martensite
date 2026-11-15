//! The unit tests of the support modules that several test targets share.
//!
//! A support module declares its unit tests through `support_unit_tests!`. This binary expands
//! those declarations; every other target that includes the module expands them to nothing
//! (`support/support_unit_tests_elsewhere.rs`). Each test therefore compiles and runs once, not
//! once per including binary (#471).
// This binary calls none of the support code itself; only the unit tests do.
#![allow(dead_code, unused_imports, unused_macros)]
// `object_fill_program`'s controls load `primitive_shapes` again under their own path.
#![allow(clippy::duplicate_mod)]

macro_rules! support_unit_tests {
    ($($tests:tt)*) => { $($tests)* };
}

// The case host modules, the viewer session and the shape checks of the grouped regression hosts.
include!("case_host/session_module_tree.rs");

#[path = "support/case_v3d_policy_controls.rs"]
mod case_v3d_policy_controls;
#[path = "viewer_commands/opengl_commands/color_table.rs"]
mod color_table;
#[path = "case_host/dump_diff.rs"]
mod dump_diff;

// The dimension command fixtures of the dimension and relation regression groups.
#[path = "support/dimension_params.rs"]
mod dimension_params;
#[path = "support/dimension_request.rs"]
mod dimension_request;
#[path = "support/identic_relation_controls.rs"]
mod identic_relation_controls;

// Deterministic CPU-only regular-grid construction and stored boundary contracts.
#[path = "support/regular_surface_tessellation/mod.rs"]
mod regular_surface_tessellation;

// The native replay fixtures of the toolkit executable.
#[path = "support/cross_driver_bound.rs"]
mod cross_driver_bound;
#[path = "support/ffp_reference_profile.rs"]
mod ffp_reference_profile;
#[path = "../examples/support/framebuffer_capture_image.rs"]
mod framebuffer_capture_image;
#[path = "../examples/support/native_golden.rs"]
mod native_golden;
#[cfg(target_os = "linux")] // X11 window fixtures.
#[path = "../examples/support/native_window.rs"]
mod native_window;
#[path = "support/object_fill_program.rs"]
mod object_fill_program;
#[path = "support/user_program_compile_path.rs"]
mod user_program_compile_path;

// `capping_texture_scene` reaches its scene fixtures as siblings.
#[path = "support/capping_texture_scene.rs"]
mod capping_texture_scene;
#[path = "support/viewer_scene.rs"]
mod scene;
#[path = "support/surface_texture_scene.rs"]
mod texture_scene;
#[path = "support/view_setup.rs"]
mod view_setup;
