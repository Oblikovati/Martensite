//! Viewer regressions of modelling results: Booleans, meshes and hidden lines shown in a real
//! view. Each runs in an isolated worker under its group's stream policy.
//!
//! A case's libtest name is `<host>::<case>` and its worker identity literal says the same. Modules
//! the hosts share are declared here once, at the crate root, because they name each other through
//! `crate::`.
#![cfg(target_os = "linux")]
#![allow(
    clippy::duplicate_mod,
    clippy::duplicated_attributes,
    clippy::items_after_test_module
)]
#![allow(clippy::excessive_precision, clippy::approx_constant)] // Expected values keep their recorded decimals.
#![allow(clippy::needless_borrow)] // Case bodies keep the commands' reference forms.
include!("case_host/session_modules.rs");

use martensite_boolean_cases::boolean_direct_section as direct_section_adapter;
use martensite_boolean_cases::common_operation;
use martensite_boolean_cases::common_operation::case_bop_alerts as source_bop_alerts;
#[allow(dead_code, unused_imports)]
mod bugs_mesh_cases;
#[allow(dead_code, unused_imports)]
#[path = "case_support/case_argument_response.rs"]
mod case_argument_response;
#[allow(dead_code, unused_imports)]
#[path = "support/case_demo_process_case.rs"]
mod case_demo_process_case;
#[allow(dead_code, unused_imports)]
mod demo_dimensions_cases;
#[path = "modeling_view_hosts/demo_samples_regressions.rs"]
mod demo_samples_regressions;
#[allow(dead_code, unused_imports)]
#[path = "case_support/full_argument_response/mod.rs"]
mod full_argument_response;
#[allow(dead_code, unused_imports)]
mod hlr_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/hlr_regressions.rs"]
mod hlr_regressions;
#[allow(dead_code, unused_imports)]
#[path = "support/shape_mesh_execution.rs"]
mod mesh_execution;
#[allow(dead_code, unused_imports)]
#[path = "v3d_state_curve_cases/mesh_information.rs"]
mod mesh_information;
#[allow(dead_code, unused_imports)]
mod mesh_view_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/mesh_view_regressions.rs"]
mod mesh_view_regressions;
#[allow(dead_code, unused_imports)]
mod modalg_display_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/modalg_display_regressions.rs"]
mod modalg_display_regressions;
#[cfg(not(debug_assertions))]
#[allow(dead_code, unused_imports)]
mod modalg_display_release_cases;
#[cfg(not(debug_assertions))]
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/modalg_display_release_regressions.rs"]
mod modalg_display_release_regressions;
#[allow(dead_code, unused_imports)]
#[path = "modalg_fused_properties.rs"]
mod modalg_fused_properties;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/modalg_fused_properties_regressions.rs"]
mod modalg_fused_properties_regressions;
#[allow(dead_code, unused_imports)]
#[path = "modalg_tool_cuts.rs"]
mod modalg_tool_cuts;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/modalg_tool_cuts_regressions.rs"]
mod modalg_tool_cuts_regressions;
#[allow(dead_code, unused_imports)]
#[path = "support/global_mode_command_contract.rs"]
mod mode_command_contract;
#[allow(dead_code, unused_imports)]
#[path = "support/global_mode_command_fixture.rs"]
mod mode_command_fixture;
#[allow(dead_code, unused_imports)]
mod model_view_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/model_view_regressions.rs"]
mod model_view_regressions;
#[allow(dead_code, unused_imports)]
#[path = "support/shape_dump_text.rs"]
mod shape_dump;
#[allow(dead_code, unused_imports)]
#[path = "support/shape_report_checks.rs"]
mod shape_report_checks;
#[allow(dead_code, unused_imports)]
mod v3d_aisobject;
#[allow(dead_code, unused_imports)]
mod v3d_angular_spline;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_angular_spline_regressions.rs"]
mod v3d_angular_spline_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_argument_diagnostics_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_argument_diagnostics_regressions.rs"]
mod v3d_argument_diagnostics_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_argument_lists_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_argument_lists_regressions.rs"]
mod v3d_argument_lists_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_checked_bores;
#[allow(dead_code, unused_imports)]
mod v3d_display_cases;
#[path = "modeling_view_hosts/v3d_display_regressions.rs"]
mod v3d_display_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_fuzzy_tools;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_fuzzy_tools_regressions.rs"]
mod v3d_fuzzy_tools_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_geometry_cleanup;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_geometry_cleanup_regressions.rs"]
mod v3d_geometry_cleanup_regressions;
#[allow(dead_code, unused_imports)]
#[path = "v3d_hollow_cut_cpu.rs"]
mod v3d_hollow_cut_cpu;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_hollow_cut_regressions.rs"]
mod v3d_hollow_cut_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_materials_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_mesh_foundation_regressions.rs"]
mod v3d_mesh_foundation_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_mesh_healing_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_mesh_healing_regressions.rs"]
mod v3d_mesh_healing_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_mirrored_faces_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_mirrored_faces_regressions.rs"]
mod v3d_mirrored_faces_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_null_surface;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_null_surface_regressions.rs"]
mod v3d_null_surface_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_parallel_build;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_parallel_build_regressions.rs"]
mod v3d_parallel_build_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_penrose;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_revolution_perf_regressions.rs"]
mod v3d_revolution_perf_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_section_quality_cases;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_section_quality_regressions.rs"]
mod v3d_section_quality_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_sewn_boolean;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_sewn_boolean_regressions.rs"]
mod v3d_sewn_boolean_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_shared_solids;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_shared_solids_regressions.rs"]
mod v3d_shared_solids_regressions;
#[allow(dead_code, unused_imports)]
mod v3d_text_cases;
#[allow(dead_code, unused_imports)]
mod v3d_trimmed_cones;
#[allow(dead_code, unused_imports)]
#[path = "modeling_view_hosts/v3d_trimmed_cones_regressions.rs"]
mod v3d_trimmed_cones_regressions;
// The output controls of the capture transport. A second compile of the shared source: adding them
// to session_modules.rs would register them in every other host.
#[allow(dead_code)]
#[path = "source_output"]
mod capture_output_controls {
    include!("opengl/source_output.rs");
    mod additional_capture;
    #[cfg(test)]
    mod output_controls;
}
// Named through `super::` by the shared modules above.
#[allow(unused_imports)]
use case_view as source_scene;
#[allow(unused_imports)]
use case_word_profile as source_word_profile;
#[allow(unused_imports)]
use martensite_tkbrep::topods::Shape;
#[allow(unused_imports)]
use martensite_tkernel::standard::Failure;
#[allow(unused_imports)]
use modalg_fused_properties as property_queries;
