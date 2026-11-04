//! Real direct-child bindings preserve the untyped explode command rather than geometry ranking.
#[cfg(test)]
mod cpu;
mod cylinder;
mod shapes;
use crate::{
    case_view::viewer_session::DisplayRequest,
    session_run::{SessionRun, banner},
};
pub(crate) use cylinder::cylinder_sensitive_area;
use martensite_tkv3d::v3d::V3dTypeOfOrientation;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Display the four actual lateral cone faces and active sensitive entities; `cone_sensitive_faces()?`.
pub(crate) fn cone_sensitive_faces() -> Outcome {
    banner(&[
        "============",
        "0033664: Visualization - Selection does not work for simple shape",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    let registry = shapes::cone_explode_bindings()?;
    let names = ["c1_1_1", "c2_1_1", "c3_1_1", "c4_1_1"];
    let shown: Vec<_> = names.iter().map(|name| (*name, &registry[*name])).collect();
    run.session()
        .display_many(&shown, &DisplayRequest::default().with_display_mode(1))?;
    run.session().display_active_sensitive()?;
    run.session()
        .set_standard_projection(V3dTypeOfOrientation::ZUP_FRONT)?;
    run.session().fit()?;
    run.dump("bug33664_2_cone.png")?;
    run.finish()
}
