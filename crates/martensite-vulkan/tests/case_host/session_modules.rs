// Shared module tree of the grouped regression hosts; each host `include!`s this file.
// The grouped hosts run each regression in an isolated worker under its group's stream policy
// and drive the viewer through `case_view::viewer_session::ViewerSession`.
#[path = "../../examples/support/device_options.rs"]
mod device_options;
// Recorded failures re-grade through a worker companion (`recorded_case!`).
#[allow(unused_macros, dead_code)] // Hosts without recorded failures do not expand it.
#[macro_use]
#[path = "recorded_case.rs"]
mod recorded_case;
#[allow(dead_code)] // Only the complete-source receipt framing executes for these fixed groups.
#[path = "../report_rules/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[allow(dead_code)] // Canonical framing retains bugs-policy conveniences unused by these groups.
#[path = "../report_rules/case_bugs_bop_rules.rs"]
mod case_bugs_bop_rules;
#[allow(dead_code)] // The groups select the inherited-common entry point.
#[path = "../report_rules/case_common_bop_rules.rs"]
mod case_common_bop_rules;
#[allow(dead_code)] // Each host selects either the V3d or the OpenGL group policy.
#[path = "../support/case_opengl_policy.rs"]
mod case_opengl_policy;
#[allow(dead_code)] // Each host selects either the V3d or the OpenGL group policy.
#[path = "../support/case_opengl_process_case.rs"]
mod case_opengl_process_case;
#[allow(dead_code)] // Only perf-group hosts grade with the perf rules.
#[path = "../report_rules/case_perf_bop_rules.rs"]
mod case_perf_bop_rules;
#[allow(dead_code)] // Only perf-group hosts run perf regressions.
#[path = "../support/case_perf_process_case.rs"]
mod case_perf_process_case;
#[allow(dead_code)] // Only complete-source framing executes; bugs-only receipt helpers stay unchanged.
#[path = "../report_rules/case_report_receipt.rs"]
mod case_report_receipt;
#[allow(dead_code)] // Only cases with command-host TODO lines grade expected failures.
#[path = "../support/case_todo.rs"]
mod case_todo;
#[allow(dead_code)] // Each host selects either the V3d or the OpenGL group policy.
#[path = "../support/case_v3d_policy.rs"]
mod case_v3d_policy;
#[allow(dead_code)] // Each host selects either the V3d or the OpenGL group policy.
#[path = "../support/case_v3d_process_case.rs"]
mod case_v3d_process_case;
#[allow(dead_code)] // The grouped hosts use the session, not the single-object shortcuts.
#[path = "../support/hatch_scene.rs"]
pub mod case_view;
#[allow(dead_code)] // The graders reuse the captured word profile, without registering its controls.
#[path = "../report_rules/caf_word_profile.rs"]
mod case_word_profile;
mod hatch_surface {
    pub(crate) use crate::case_view as scene;
}
#[allow(dead_code)] // Multi-dump completion is shared; single-dump hosts leave parts unused.
#[path = "../source_output"]
mod source_output {
    include!("../opengl/source_output.rs");
    mod additional_capture;
    pub(crate) mod control_capture;
}
#[allow(dead_code)] // Each host selects the policy of its regression group.
#[path = "../support/case_bugs_process_case.rs"]
mod case_bugs_process_case;
#[allow(dead_code)] // Only the hosts of parent/child regressions build these scenes.
#[path = "../support/child_scenes.rs"]
mod child_scenes;
#[allow(dead_code)] // Hosts restore dataset inputs only when their regressions read files.
#[path = "../support/dataset_shape.rs"]
mod dataset_shape;
#[allow(dead_code)] // Only hosts whose images need a faces-only control use it.
#[path = "faces_only_control.rs"]
mod faces_only_control;
#[allow(dead_code)] // Only the hidden-line hosts remove hidden lines.
#[path = "../support/hidden_lines.rs"]
mod hidden_lines;
#[allow(dead_code)] // Only hosts whose regressions check mesh topology use it.
#[path = "../support/mesh_topology_check.rs"]
mod mesh_topology_check;
#[allow(dead_code)] // Only hosts dumping a wireframe top view use it.
#[path = "../support/plane_capture.rs"]
mod plane_capture;
#[allow(dead_code)] // Each host builds only the primitives its regressions use.
#[path = "primitive_shapes.rs"]
mod primitive_shapes;
#[allow(dead_code)] // Older grouped hosts drive their sessions without the run wrapper.
#[path = "session_run.rs"]
mod session_run;
#[allow(dead_code)] // Hosts check shape validity and mass only when their regressions do.
#[path = "../support/shape_checks.rs"]
mod shape_checks;
#[allow(dead_code)] // Hosts mesh and check shapes only when their regressions do.
#[path = "../support/shape_mesh.rs"]
mod shape_mesh;
#[allow(dead_code)] // Hosts use the check view and sub-shape counts their regressions need.
#[path = "../support/shape_view_checks.rs"]
mod shape_view_checks;
#[path = "snap_pixels.rs"]
mod snap_pixels;
#[path = "supersample_control.rs"]
mod supersample_control;
#[path = "tie_edges.rs"]
mod tie_edges;
#[allow(dead_code)] // Only hosts drawing the viewer commands' own objects use it.
#[path = "../support/viewer_objects.rs"]
mod viewer_objects;
#[allow(dead_code)] // Hosts explode shapes only when their regressions do.
#[allow(unused_imports)] // Hosts select only the topology queries their cases require.
use martensite_topology_test_support::typed_exploration as boolean_typed_explode;
