//! Exact retained owned shell recipes are shared by unit and external public-call tests.
use super::ShellSplitter;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Build the exact duplicated-face recipe; `irregular_splitter(false)`.
pub(super) fn irregular_splitter(parallel: bool) -> ShellSplitter {
    let faces = box_faces_at(Point3::default());
    let mut splitter = ShellSplitter::new();
    splitter.options_mut().set_run_parallel(parallel);
    for _ in 0..2 {
        for face in &faces {
            splitter.add_start_element(face);
        }
    }
    splitter
}

/// Retain the ten-unit box face recipe; `box_faces_at(Point3::default())`.
pub(super) fn box_faces_at(origin: Point3) -> Vec<Shape> {
    let side = 10.;
    box_faces_with_side(origin, side)
}

/// Retain the literal box dimensions and face order; `box_faces_with_side(origin, 2.)`.
pub(super) fn box_faces_with_side(origin: Point3, side: f64) -> Vec<Shape> {
    let shape = BRepPrimAPIMakeBox::at_point(origin, side, side, side)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    Explorer::new(&shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .collect()
}

/// Retain both original duplicate-face blocks; `two_irregular_blocks(true)`.
pub(super) fn two_irregular_blocks(parallel: bool) -> ShellSplitter {
    let mut splitter = irregular_splitter(parallel);
    for face in box_faces_at(Point3::new(20., 0., 0.)) {
        splitter.add_start_element(&face);
        splitter.add_start_element(&face);
    }
    splitter
}

/// Select only the captured native shell recipes; `native_shell_fixture("regular_shell")`.
pub(super) fn native_shell_fixture(kind: &str) -> ShellSplitter {
    if kind == "irregular_shell" {
        return irregular_splitter(false);
    }
    let mut splitter = ShellSplitter::new();
    for face in box_faces_at(Point3::default()) {
        splitter.add_start_element(&face);
    }
    for face in box_faces_with_side(Point3::new(20., 0., 0.), 2.) {
        splitter.add_start_element(&face);
    }
    splitter
}
