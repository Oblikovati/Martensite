//! Boolean operations and the general fuse of argument lists: 121 boxes against 100
//! cylinders, each list given member by member (OCC25614, `baddcompound`, `baddctools`,
//! `bapibop`, `bapibuild`).
use super::result_view::display_result_view;
use crate::primitive_shapes::{box_sized, cylinder};
use crate::session_run::{SessionRun, banner};
use crate::shape_report_checks::{ShapeCounts, check_shape_counts};
use martensite_tkbo::bop_algo::{Glue, Operation};
use martensite_tkbo::brep_algo_api::{BRepAlgoAPIBooleanOperation, BRepAlgoAPIBuilderAlgo};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakeShape, BRepBuilderAPITransform,
};
use std::error::Error;

/// The `bapibop` operation number, or `bapibuild`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ArgumentListOperation {
    /// `bapibop result 0`.
    Common,
    /// `bapibop result 1`.
    Fuse,
    /// `bapibop result 2`.
    Cut,
    /// `bapibop result 3`: a cut with the object and tool lists swapped.
    Cut21,
    /// `bapibop result 4`.
    Section,
    /// `bapibuild result` with both lists as arguments (`baddcompound b2`).
    GeneralFuse,
}

/// `bug25614_*`: the operation on the box and cylinder lists, its sub-shape counts
/// (`checknbshapes`, vertex first, the total last), then `checkview -display result -3d`.
/// Example: `argument_list_view(ArgumentListOperation::Cut, [888, ...], "bug25614_cut.png")?`.
pub(crate) fn argument_list_view(
    operation: ArgumentListOperation,
    counts: [usize; 9],
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    banner(&["============", "OCC25614", "============", ""]);
    let (boxes, cylinders) = (box_grid()?, cylinder_grid()?);
    let result = match operation {
        ArgumentListOperation::GeneralFuse => general_fuse(&boxes, &cylinders)?,
        _ => boolean(operation, &boxes, &cylinders)?,
    };
    check_shape_counts(&result, "result", &ShapeCounts::all(counts))?;
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, &result, image)?;
    run.finish()
}

/// `box b 10 10 4`, moved down by 2, copied over an 11 x 11 grid of pitch 10.
pub(crate) fn box_grid() -> Result<Vec<Shape>, Failure> {
    let base = moved(&box_sized([10., 10., 4.])?, [0., 0., -2.])?;
    copied_grid(&base, 11)
}

/// `pcylinder b 6 10` moved by (10, 10, -5), copied over a 10 x 10 grid of pitch 10.
pub(crate) fn cylinder_grid() -> Result<Vec<Shape>, Failure> {
    let base = moved(&cylinder(6., 10.)?, [10., 10., -5.])?;
    copied_grid(&base, 10)
}

/// `tcopy b b_i_j; ttranslate b_i_j [i*10] [j*10] 0`, in the script's i-major order.
fn copied_grid(base: &Shape, n: usize) -> Result<Vec<Shape>, Failure> {
    let mut copies = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            let copy = BRepBuilderAPICopy::with_shape(base, true, false)?
                .shape()?
                .clone();
            copies.push(moved(&copy, [i as f64 * 10., j as f64 * 10., 0.])?);
        }
    }
    Ok(copies)
}

/// `ttranslate`: a transform without copying, which moves the location.
fn moved(shape: &Shape, [x, y, z]: [f64; 3]) -> Result<Shape, Failure> {
    let mut transform = Transform3::default();
    transform.set_translation_part(Vector3::new(x, y, z));
    let mut maker = BRepBuilderAPITransform::new(transform);
    maker.perform(shape, false, false)?;
    Ok(maker.shape()?.clone())
}

fn list(shapes: &[Shape]) -> List<Shape> {
    let mut list = List::default();
    for shape in shapes {
        list.append(shape.clone());
    }
    list
}

/// `bapibop`: Common, Fuse, Cut or Section; Cut21 builds a Cut with the lists swapped. The
/// named API classes are the operation with its type set at construction.
pub(crate) fn boolean(
    operation: ArgumentListOperation,
    objects: &[Shape],
    tools: &[Shape],
) -> Result<Shape, Box<dyn Error>> {
    let (kind, arguments, tools) = match operation {
        ArgumentListOperation::Common => (Operation::Common, objects, tools),
        ArgumentListOperation::Fuse => (Operation::Fuse, objects, tools),
        ArgumentListOperation::Cut => (Operation::Cut, objects, tools),
        ArgumentListOperation::Cut21 => (Operation::Cut, tools, objects),
        ArgumentListOperation::Section => (Operation::Section, objects, tools),
        ArgumentListOperation::GeneralFuse => unreachable!("bapibuild is not a bapibop"),
    };
    let mut builder = BRepAlgoAPIBooleanOperation::new();
    builder.set_operation(kind);
    builder.set_arguments(&list(arguments));
    builder.set_tools(&list(tools));
    configure(&mut builder);
    builder.build()?;
    finished(&mut builder, "bapibop")
}

/// `bapibuild`: the general fuse of the objects followed by the tools.
pub(crate) fn general_fuse(objects: &[Shape], tools: &[Shape]) -> Result<Shape, Box<dyn Error>> {
    let mut builder = BRepAlgoAPIBuilderAlgo::new();
    builder.set_arguments(&list(&[objects, tools].concat()));
    configure(&mut builder);
    builder.build()?;
    finished(&mut builder, "bapibuild")
}

/// The BOPTest object defaults: serial, fuzzy value Precision::Confusion(), destructive,
/// no glue, inverted solids checked, no oriented boxes, history filled.
fn configure(builder: &mut BRepAlgoAPIBuilderAlgo) {
    builder.set_run_parallel(false);
    builder.set_fuzzy_value(precision::confusion());
    builder.set_non_destructive(false);
    builder.set_glue(Glue::Off);
    builder.set_check_inverted(true);
    builder.set_use_obb(false);
    builder.set_to_fill_history(true);
}

/// The result after `SimplifyResult(false, false, angular)` (the BOPTest defaults), or the
/// reported errors.
fn finished(builder: &mut BRepAlgoAPIBuilderAlgo, command: &str) -> Result<Shape, Box<dyn Error>> {
    builder.simplify_result(false, false, precision::angular())?;
    if builder.has_errors() {
        let mut report = Vec::new();
        builder.dump_errors(&mut report)?;
        return Err(format!("{command}: {}", String::from_utf8_lossy(&report)).into());
    }
    if builder.shape().is_null() {
        return Err(format!("{command}: Result is a null shape").into());
    }
    Ok(builder.shape().clone())
}
