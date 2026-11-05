//! Boolean results of dataset wires, boxes and trimmed cylinder faces, checked, then shown
//! (OCC492, OCC578, OCC2785).
use super::display_result_view;
use super::prelude::*;
use crate::common_operation;
use crate::primitive_shapes::{box_sized, cylinder};
use martensite_tkbo::bop_algo::Operation;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};

/// How a regression subtracts: a direct cut, or intersections first and then the cut built
/// from them.
#[derive(Clone, Copy, Debug)]
pub(crate) enum CutPath {
    /// `bcut`.
    Direct,
    /// `bop` followed by the cut command.
    Prepared,
}

fn cut(object: &Shape, tool: &Shape, path: CutPath, kind: Operation) -> Shape {
    match path {
        CutPath::Direct => common_operation::direct(object, tool, kind),
        CutPath::Prepared => common_operation::prepared(object, tool, kind),
    }
}

/// `bug578_1/2`: a box minus the fusion of two dataset wires keeps its area.
/// Example: `box_minus_wires_view(CutPath::Direct, "bug578_1.png")?`.
pub(crate) fn box_minus_wires_view(path: CutPath, image: &'static str) -> Outcome {
    banner(&[
        "================",
        "OCC578",
        "OCC584",
        "================",
        "",
    ]);
    let w1 = read_dataset_shape("OCC578_w1.brep")?;
    check_shape_structure(&w1)?;
    let w2 = read_dataset_shape("OCC578_w2.brep")?;
    check_shape_structure(&w2)?;
    let b = box_sized([1., 1., 0.5])?;
    check_shape_structure(&b)?;
    let wires = common_operation::direct(&w1, &w2, Operation::Fuse);
    check_shape_structure(&wires)?;
    let result = cut(&b, &wires, path, Operation::Cut);
    checked_view(&result, 4.84596, image)
}

fn checked_view(result: &Shape, area: f64, image: &'static str) -> Outcome {
    check_mass(result, &MassCheck::near(MassKind::Area, area))?;
    check_shape_structure(result)?;
    let mut run = SessionRun::new()?;
    display_result_view(&mut run, result, image)?;
    run.finish()
}

/// `bug492`: two dataset solids fuse without failure.
pub(crate) fn dataset_fuse_view() -> Outcome {
    banner(&["=========", " OCC492 ", "=========", ""]);
    let a_1 = read_dataset_shape("OCC492a.brep")?;
    check_shape_structure(&a_1)?;
    let a_2 = read_dataset_shape("OCC492b.brep")?;
    check_shape_structure(&a_2)?;
    let result = common_operation::direct(&a_1, &a_2, Operation::Fuse);
    println!("OCC492 : function FUSE works without hangs up ");
    checked_view(&result, 64.2386, "bug492.png")
}

/// `bug2785_1/2`: a cylinder's lateral face minus, then fused with, two trimmed patches of
/// another cylinder's lateral face; the second cut is `boptuc` or a direct `bcut`.
pub(crate) fn patched_cylinder_view(path: CutPath, image: &'static str) -> Outcome {
    banner(&["============", "OCC2785", "============", ""]);
    let c1 = first_face(&cylinder(2., 10.)?)?;
    let lateral = BRepTool::surface(&topods::face(&first_face(&cylinder(2., 2.)?)?)?)?;
    let ff = trimmed_face(&lateral, (1., 2.))?;
    let ff2 = trimmed_face(&lateral, (0., 1.))?;
    let rcut1 = common_operation::direct(&c1, &ff, Operation::Cut);
    check_shape_structure(&rcut1)?;
    let rcut2 = match path {
        CutPath::Prepared => cut(&ff2, &rcut1, path, Operation::Cut21),
        CutPath::Direct => cut(&rcut1, &ff2, path, Operation::Cut),
    };
    check_shape_structure(&rcut2)?;
    let fuse1 = common_operation::direct(&rcut2, &ff2, Operation::Fuse);
    check_shape_structure(&fuse1)?;
    let result = common_operation::direct(&fuse1, &ff, Operation::Fuse);
    checked_view(&result, 125.664, image)
}

fn first_face(shape: &Shape) -> Result<Shape, Failure> {
    let explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    Ok(explorer.current().clone())
}

/// The face of `surface` trimmed to `u` and V in [2, 4].
fn trimmed_face(surface: &Handle<dyn GeomSurface>, u: (f64, f64)) -> Result<Shape, Failure> {
    let trimmed = GeomRectangularTrimmedSurface::new(surface, u, (2., 4.), (true, true))?;
    let trimmed: Handle<dyn GeomSurface> = Handle::new(trimmed).cast();
    let mut maker = BRepBuilderAPIMakeFace::from_surface(&trimmed, precision::confusion())?;
    Ok(maker.shape()?.clone())
}
