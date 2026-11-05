//! Native replay of the GDML protrusion construction with partial cylinders.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, PerformCut, PerformFuse, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn partial_cylinder(radius: f64) -> martensite_tkbrep::topods::Shape {
    let mut maker = BRepPrimAPIMakeCylinder::with_angle(radius, 885.4346013, 300_f64.to_radians())
        .expect("GDML B8 partial cylinder");
    let mut shape = maker
        .shape()
        .expect("GDML B8 partial cylinder shape")
        .clone();
    RotateAroundAxis(&mut shape, Point3::default(), [0., 0., 1.], -90.);
    Translate(&shape, 0., 0., -442.71730065)
}

fn placed_protrusion(
    mut shape: martensite_tkbrep::topods::Shape,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], -161.93649349);
    Translate(&shape, 127., -258.32178088, -177.99555741)
}

#[test]
fn GdmlPublic__B8() {
    let source = "tests/boolean/gdml_public/B8";
    let outer = placed_protrusion(partial_cylinder(702.71927595));
    let inner = placed_protrusion(partial_cylinder(153.59337693));
    let mut box_shape = CreateBox(
        Point3::new(-150.798933325, -150.798933325, -508.),
        301.59786665,
        301.59786665,
        1016.,
    );
    RotateAroundAxis(
        &mut box_shape,
        Point3::default(),
        [
            -0.70710678118654735,
            0.70710678118654768,
            8.7731777483398112e-17,
        ],
        180.,
    );
    box_shape = Translate(
        &box_shape,
        399.74468245,
        178.4271923835945,
        -950.7173006534674,
    );
    let box_shape = placed_protrusion(box_shape);
    let cut = PerformCut(&outer, &inner);
    assert!(!cut.is_null(), "{source}: radial cut");
    let result = PerformFuse(&cut, &box_shape);
    assert!(!result.is_null(), "{source}: protrusion fuse");
}
