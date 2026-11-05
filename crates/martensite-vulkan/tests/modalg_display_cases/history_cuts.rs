//! The OCC29807 cut through the prepared intersections with its history saved: the faces the
//! cut modified from the tool's second face must keep their degenerated edges.
use super::prelude::*;
use super::tool_cuts::{
    FIRST_AXIS, REGRESSION_CAMERA, ToolCut, placed_tool, tighten_second_vertex,
};
use crate::boolean_typed_explode::all_distinct;
use crate::case_argument_response::default_argument_response;
use crate::common_operation::{intersections, prepared_with_filler_and_history};
use crate::mesh_topology_check::check_mesh_topology;
use crate::shape_report_checks::{ShapeCounts, check_shape_counts};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Builder, Compound};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// Whether some edge of `face` is degenerated, as `dump` then shows `degenerated`.
fn has_degenerated_edge(face: &Shape) -> Result<bool, Failure> {
    for edge in all_distinct(face, ShapeEnum::Edge) {
        if BRepTool::degenerated(&topods::edge(&edge)?)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// `modified reshm ResHist b2_2`, `checknbshapes reshm -face 2`, then the degenerated-edge
/// check of every face.
fn check_modified_faces(modified: &[Shape]) -> Outcome {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    for shape in modified {
        Builder.add(&mut compound, shape)?;
    }
    let reshm = compound.into_shape();
    let mut counts = [None; 9];
    counts[3] = Some(2);
    let expected = ShapeCounts {
        expected: counts,
        ..ShapeCounts::default()
    };
    check_shape_counts(&reshm, "reshm", &expected)?;
    for face in all_distinct(&reshm, ShapeEnum::Face) {
        if !has_degenerated_edge(&face)? {
            println!("Error: Degenerated edge is not found");
        }
    }
    Ok(())
}

/// The cut result's checks of `bug29807_b3a/b3b`.
fn check_result(result: &Shape) -> Outcome {
    check_shape_structure(result)?;
    check_mass(result, &MassCheck::near(MassKind::Volume, 9465.07))?;
    check_mass(result, &MassCheck::near(MassKind::Area, 4012.74))?;
    let mut counts = [None; 9];
    counts[2] = Some(9);
    counts[3] = Some(9);
    let expected = ShapeCounts {
        expected: counts,
        ..ShapeCounts::default()
    };
    check_shape_counts(result, "result", &expected)?;
    if default_argument_response(result).contains("Faulties") {
        println!("Error: bopargcheck has found some faulties in result");
    }
    Ok(())
}

/// `tricheck result` reports no face without a triangulation.
fn check_triangulated(result: &Shape) -> Outcome {
    if !check_mesh_topology(result)?.untriangulated_faces.is_empty() {
        println!("Error: Result contains not triangulated face");
    }
    Ok(())
}

/// `bug29807_b3a` (and `b3b` when `tighten`): the tool turned by -5 degrees without location,
/// cut by `bbop result 2` over `bfillds`, its history checked, then shown shaded.
pub(crate) fn history_cut_view(tighten: bool, image: &'static str) -> Outcome {
    banner(&[
        "========",
        "0029807: Impossible to cut cone from prism",
        "========",
        "",
    ]);
    let b1 = read_dataset_shape("bug29807-obj.brep")?;
    let b2 = placed_tool(&ToolCut {
        rotations: &[(FIRST_AXIS, -5.)],
        lift: None,
        remove_location: true,
        tighten_vertex: false,
        area: 0.,
        volume: 0.,
        image,
    })?;
    if tighten {
        tighten_second_vertex(&b2)?;
    }
    let mut filler = intersections(&b1, &b2);
    let (result, history) = prepared_with_filler_and_history(&b1, &b2, Operation::Cut, &mut filler);
    check_result(&result)?;
    let face = all_distinct(&b2, ShapeEnum::Face).swap_remove(1);
    let modified = history
        .get()
        .map(|h| h.modified(&face))
        .transpose()?
        .unwrap_or_default();
    check_modified_faces(&modified)?;
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", &result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().set_view_parameters(&REGRESSION_CAMERA)?;
    check_triangulated(&result)?;
    run.dump(image)?;
    run.finish()
}
