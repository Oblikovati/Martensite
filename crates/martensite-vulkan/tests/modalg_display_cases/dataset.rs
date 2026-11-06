//! Dataset shapes converted, mirrored and fused, or analysed as Boolean arguments, then shown
//! (OCC24220, OCC24654, OCC24890, OCC25982).
use super::display_result_view;
use super::prelude::*;
use crate::case_argument_response::default_argument_response;
use crate::common_operation;
use crate::shape_report_checks::{ShapeCounts, check_shape_counts};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Transform3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert,
    BRepBuilderAPITransform,
};

/// The phrase the argument analysis prints for faulty arguments.
const FAULTY: &str = "Faulties, that can not be treated by BOP, are detected";

fn shown(result: &Shape, image: &'static str) -> Outcome {
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, result, image)?;
    run.finish()
}

/// `bug24220`: the argument analysis of a dataset shape reports its faults.
pub(crate) fn faulty_argument_view() -> Outcome {
    banner(&["============", "OCC24220", "============", ""]);
    let result = read_dataset_shape("bug24220_A.brep")?;
    if default_argument_response(&result).contains(FAULTY) {
        println!("OK : message is correct");
    } else {
        println!("Error : message is not correct");
    }
    shown(&result, "bug24220.png")
}

/// `bug24654`: the argument analysis of a dataset shape finds no fault.
pub(crate) fn valid_argument_view() -> Outcome {
    banner(&["============", "OCC24654", "============", ""]);
    let result = read_dataset_shape("bug24654_qf.brep")?;
    if default_argument_response(&result).contains(FAULTY) {
        println!("Error : Wrong result");
    } else {
        println!("OK : Good result");
    }
    shown(&result, "bug24654.png")
}

/// `bug24890`: a locked dataset face converts to NURBS as a valid shape.
pub(crate) fn locked_face_nurbs_view() -> Outcome {
    banner(&["============", "OCC24890", "============", ""]);
    let f0 = read_dataset_shape("bug24890_f0.brep")?;
    f0.set_locked(true)?;
    let mut converter = BRepBuilderAPINurbsConvert::new();
    converter.perform(&f0, false)?;
    let result = converter.shape()?.clone();
    check_shape_structure(&result)?;
    shown(&result, "bug24890.png")
}

/// `bug25982`: the fourth face of a dataset shape and of its mirror copy split each other.
pub(crate) fn mirrored_face_fuse_view() -> Outcome {
    banner(&["============", "OCC25982", "============", ""]);
    let result = mirrored_face_result()?;
    let expected = ShapeCounts {
        per_location: true,
        subject: "Result obtained by General Fuse operator",
        ..ShapeCounts::all([6, 11, 4, 2, 0, 0, 0, 1, 24])
    };
    check_shape_counts(&result, "result", &expected)?;
    check_mass(&result, &MassCheck::near(MassKind::Area, 306.285))?;
    check_shape_structure(&result)?;
    shown(&result, "bug25982.png")
}

/// Preserve copy, mirror, source fourth-face selection and prepared actual general fuse.
/// Example: `let result = mirrored_face_result()?;`.
pub(crate) fn mirrored_face_result() -> Result<Shape, Box<dyn Error>> {
    let b1 = read_dataset_shape("bug25982_human_head_3.brep")?;
    let b2 = mirrored(
        &BRepBuilderAPICopy::with_shape(&b1, true, false)?
            .shape()?
            .clone(),
    )?;
    let b1 = fourth_face(&b1)?;
    let b2 = fourth_face(&b2)?;
    Ok(common_operation::general_fuse(&b1, &b2))
}

/// The mirror image of `shape` in the plane x = 0.
fn mirrored(shape: &Shape) -> Result<Shape, Failure> {
    let mut transform = Transform3::default();
    transform.set_mirror_plane(Axis2::new(
        Point3::new(0., 0., 0.),
        Direction3::new(1., 0., 0.)?,
    )?);
    let mut maker = BRepBuilderAPITransform::new(transform);
    maker.perform(shape, false, false)?;
    Ok(maker.shape()?.clone())
}

/// The fourth distinct face in exploration order, as an explode numbers it.
fn fourth_face(shape: &Shape) -> Result<Shape, Failure> {
    let mut seen = MapOfShape::default();
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while explorer.more() {
        if seen.add(explorer.current().clone()) && seen.extent() == 4 {
            return Ok(explorer.current().clone());
        }
        explorer.next()?;
    }
    Err(Failure::with_message(
        "Standard_OutOfRange",
        &format!("shape has {} faces; expected at least 4", seen.extent()),
    ))
}
