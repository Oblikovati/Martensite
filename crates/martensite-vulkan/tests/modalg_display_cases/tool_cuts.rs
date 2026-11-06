//! A prism minus a cone tool rotated and moved near tangency, checked as a Boolean argument,
//! then shown shaded from a fixed camera (OCC29807).
use super::prelude::*;
use crate::case_argument_response::default_argument_response;
use crate::case_view::viewer_session::ViewParameters;
use crate::common_operation;
use crate::shape_report_checks::{ShapeCounts, check_shape_counts, print_tolerances};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3, Vector3};
use martensite_tkshhealing::shape_fix::ShapeFixShapeTolerance;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeRemoveLocations;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform};

/// The point the tool rotates about.
const PIVOT: [f64; 3] = [23.858571571457155, 12., 5.5];
/// The regressions' `vviewparams`.
pub(super) const REGRESSION_CAMERA: ViewParameters = ViewParameters {
    scale: Some(52.4803),
    proj: Some([0.285421, 0.0158136, 0.958272]),
    up: Some([-0.810772, 0.537155, 0.232624]),
    at: Some([29.6172, 13.0268, 3.82234]),
    eye: Some([47.7439, 14.0311, 64.6808]),
};
/// The first rotation axis direction.
pub(crate) const FIRST_AXIS: [f64; 3] = [7., -7.14142842854285, 0.];
/// The second rotation axis direction.
pub(crate) const SECOND_AXIS: [f64; 3] = [7.1414284285428495, 7.000000000000001, -0.];

/// One placement of the tool and the expected result of the cut.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ToolCut {
    /// Rotations about the pivot: axis direction and angle in degrees, in order.
    pub rotations: &'static [([f64; 3], f64)],
    /// The tool's final Z translation.
    pub lift: Option<f64>,
    /// Bake the tool's location into its geometry.
    pub remove_location: bool,
    /// Report the tool's tolerances and tighten its second vertex to 1e-7.
    pub tighten_vertex: bool,
    /// The expected area.
    pub area: f64,
    /// The expected volume.
    pub volume: f64,
    /// The image dumped.
    pub image: &'static str,
}

/// `bug29807_*`: cut the placed tool from the prism, check the result, then show it.
/// Example: `tool_cut_view(&case)?`.
pub(crate) fn tool_cut_view(case: &ToolCut) -> Outcome {
    tool_cut_view_with_todos(case, &[])
}

/// Execute the same source with authored declarations after its banner; `tool_cut_view_with_todos(&case, TOOL_CUT_TODOS)?`.
pub(crate) fn tool_cut_view_with_todos(case: &ToolCut, todos: &[&str]) -> Outcome {
    source_cut_heading(todos);
    let b1 = read_dataset_shape("bug29807-obj.brep")?;
    let b2 = placed_tool(case)?;
    let result = common_operation::direct(&b1, &b2, Operation::Cut);
    check_cut_result(&result, case)?;
    shaded_camera_view(&result, case.image)
}

fn source_cut_heading(todos: &[&str]) {
    banner(&[
        "========",
        "0029807: Impossible to cut cone from prism",
        "========",
        "",
    ]);
    for todo in todos {
        println!("TODO OCC29883 ALL: {todo}");
    }
}

fn check_cut_result(result: &Shape, case: &ToolCut) -> Outcome {
    check_shape_structure(result)?;
    check_mass(result, &MassCheck::near(MassKind::Area, case.area))?;
    check_mass(result, &MassCheck::near(MassKind::Volume, case.volume))?;
    check_cut_counts(result)?;
    if default_argument_response(result).contains("Faulties") {
        println!("Error: bopargcheck has found some faulties in result");
    }
    Ok(())
}

fn check_cut_counts(result: &Shape) -> Outcome {
    let mut counts = [None; 9];
    counts[2] = Some(9);
    counts[3] = Some(9);
    check_shape_counts(
        result,
        "result",
        &ShapeCounts {
            expected: counts,
            ..ShapeCounts::default()
        },
    )?;
    Ok(())
}

/// The tool read from the dataset, rotated, lifted, without location and tightened as `case`
/// says; `placed_tool(&case)?`.
pub(super) fn placed_tool(case: &ToolCut) -> Result<Shape, Box<dyn Error>> {
    let mut tool = read_dataset_shape("bug29807-tool.brep")?;
    for (axis, degrees) in case.rotations {
        tool = rotated(&tool, *axis, *degrees)?;
    }
    if let Some(z) = case.lift {
        let mut lift = Transform3::default();
        lift.set_translation_part(Vector3::new(0., 0., z));
        tool = transformed(&tool, lift)?;
    }
    if case.remove_location {
        let remover = ShapeUpgradeRemoveLocations::new();
        remover.remove(&tool)?;
        tool = remover.get_result();
    }
    if case.tighten_vertex {
        source_tolerance_fix(&tool)?;
    }
    Ok(tool)
}

fn source_tolerance_fix(tool: &Shape) -> Outcome {
    print_tolerances(tool)?;
    tighten_second_vertex(tool)
}

/// `explode b2 v`, `settolerance b2_2 1.0e-7`, `checkshape b2`; `tighten_second_vertex(&tool)?`.
pub(super) fn tighten_second_vertex(tool: &Shape) -> Outcome {
    println!("Setting Tolerance to 1e-07");
    ShapeFixShapeTolerance::new().set_tolerance(&second_vertex(tool)?, 1.0e-7, ShapeEnum::Shape)?;
    check_shape_structure(tool)?;
    Ok(())
}

fn rotated(shape: &Shape, [x, y, z]: [f64; 3], degrees: f64) -> Result<Shape, Failure> {
    let [px, py, pz] = PIVOT;
    let axis = Axis1::new(Point3::new(px, py, pz), Direction3::new(x, y, z)?);
    let mut rotation = Transform3::default();
    rotation.set_rotation(axis, degrees * (std::f64::consts::PI / 180.0))?;
    transformed(shape, rotation)
}

fn transformed(shape: &Shape, transform: Transform3) -> Result<Shape, Failure> {
    let mut maker = BRepBuilderAPITransform::new(transform);
    maker.perform(shape, false, false)?;
    Ok(maker.shape()?.clone())
}

/// The second distinct vertex in exploration order, as an explode numbers it.
fn second_vertex(shape: &Shape) -> Result<Shape, Failure> {
    let mut seen = MapOfShape::default();
    let mut explorer = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while explorer.more() {
        if seen.add(explorer.current().clone()) && seen.extent() == 2 {
            return Ok(explorer.current().clone());
        }
        explorer.next()?;
    }
    Err(Failure::with_message(
        "Standard_OutOfRange",
        "tool has fewer than 2 vertices; expected at least 2",
    ))
}

/// Display the result, shade every object, set the regression's camera and dump the view.
fn shaded_camera_view(result: &Shape, image: &'static str) -> Outcome {
    let mut run = SessionRun::new()?;
    run.session()
        .display("result", result, &DisplayRequest::default())?;
    run.session().set_display_mode(1)?;
    run.session().set_view_parameters(&REGRESSION_CAMERA)?;
    run.dump(image)?;
    run.finish()
}
