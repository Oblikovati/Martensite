//! Regressions of fixture-free `tests/boolean/cells_test` regressions.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::BOPAlgoCellsBuilder;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{self, Axis2, Circle3, Line3, Plane3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeSphere,
};
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
        BRepBuilderAPIMakeWire,
    },
    brep_gprop::BRepGProp,
};

fn sphere(center: Point3) -> Shape {
    BRepPrimAPIMakeSphere::at_center(center, 15.0)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn selected(shapes: &[&Shape]) -> List<Shape> {
    shapes.iter().map(|shape| (*shape).clone()).collect()
}

fn add(cells: &mut BOPAlgoCellsBuilder, take: &[&Shape], avoid: &[&Shape], material: i32) {
    add_with_update(cells, take, avoid, material, false);
}

fn add_with_update(
    cells: &mut BOPAlgoCellsBuilder,
    take: &[&Shape],
    avoid: &[&Shape],
    material: i32,
    update: bool,
) {
    cells
        .add_to_result(&selected(take), &selected(avoid), material, update)
        .unwrap();
}

fn assert_area(source: &str, shape: &Shape, expected_area: f64) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false).unwrap();
    let area = properties.mass();
    assert!(
        (area - expected_area).abs() / expected_area <= 1e-2,
        "{source}: area {area}; expected {expected_area} within 1%"
    );
}

#[path = "cells_test_legacy/compounds.rs"]
mod compounds;
#[path = "cells_test_legacy/mixed_complex.rs"]
mod mixed_complex;
#[path = "cells_test_legacy/multidimensional.rs"]
mod multidimensional;
#[path = "cells_test_legacy/planar.rs"]
mod planar;
#[path = "cells_test_legacy/solids.rs"]
mod solids;
#[path = "cells_test_legacy/spheres.rs"]
mod spheres;
