//! Modelling regressions of the bugs group whose result is checked, then shown in a 3D view.
mod booleans;
mod dataset;
mod dataset_cuts;
mod fused;
mod primitives;
mod sewing;
mod tool_cut_sources;
#[cfg(test)]
pub(crate) use tool_cut_sources::source_tool;
mod tool_cuts;
pub(crate) use tool_cut_sources::{SourceCut, TOOL_CUT_TODOS, tool_cut_spec};
mod triangulated;
pub(crate) use booleans::{
    CutPath, box_minus_wires_view, dataset_fuse_view, patched_cylinder_view,
};
pub(crate) use dataset::{
    faulty_argument_view, locked_face_nurbs_view, mirrored_face_fuse_view, valid_argument_view,
};
pub(crate) use dataset_cuts::{blower_cut_view, parallel_common_view, volume_changing_cut_view};
mod history_cuts;
mod regularity;
mod self_intersection;
pub(crate) use fused::{
    HalfSpaceObject, fused_wire_compounds_view, general_fuse_view, half_space_cut_view,
};
pub(crate) use history_cuts::history_cut_view;
pub(crate) use primitives::{
    ConeCase, RevolvedTriangle, SphereChange, changed_sphere_view, full_sphere_face_view,
    partial_cone_view, revolved_triangle_view, revolved_wedge_view,
};
pub(crate) use regularity::{
    closed_split_regularity, fused_boxes_regularity, fused_capsule_regularity,
    perspective_hidden_lines_view, sewn_spheres_regularity, split_angle_regularity,
    tangent_plane_regularity, turned_copy_regularity,
};
pub(crate) use self_intersection::overlapped_faces_view;
pub(crate) use sewing::{SewingCase, SewingInputs, SewingOptions, sewn_result_view};
pub(crate) use tool_cuts::{
    FIRST_AXIS, SECOND_AXIS, ToolCut, tool_cut_view, tool_cut_view_with_todos,
};
pub(crate) use triangulated::{
    ShownMeshCase, cleared_wireframe_view, incremental_mesh_view, shaded_object_view,
    shaded_screenshot,
};

/// What every modelling program uses.
mod prelude {
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::dataset_shape::read_dataset_shape;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use crate::shape_checks::{MassCheck, MassKind, check_mass};
    pub(super) use crate::shape_report_checks::check_shape_structure;
    pub(super) use martensite_tkbrep::topods::Shape;
    pub(super) use std::error::Error;

    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}
use prelude::*;
mod result_view;
use result_view::display_result_view;

/// Display `result`, shade every object, fit, fit the depth range, then dump the view as it
/// is; `shaded_fit_screenshot(&result, "a.png")?`.
fn shaded_fit_screenshot(result: &Shape, image: &'static str) -> Outcome {
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().fit()?;
    run.session().z_fit()?;
    run.dump(image)?;
    run.finish()
}
