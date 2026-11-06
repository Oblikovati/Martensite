//! Native replay of the GDML multi-extrusion construction.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, CreateCylinder, PerformFuse, RotateAroundAxis, Translate};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn final_placement(
    mut shape: martensite_tkbrep::topods::Shape,
) -> martensite_tkbrep::topods::Shape {
    RotateAroundAxis(&mut shape, Point3::default(), [0., 1., 0.], 90.);
    Translate(&shape, 254., 0., -127.)
}

fn full_rail(y: f64) -> martensite_tkbrep::topods::Shape {
    final_placement(Translate(
        &Translate(&CreateCylinder(50.8, 508.), 0., 0., -254.),
        0.,
        y,
        0.,
    ))
}

fn half_rail(y: f64, reversed: bool) -> martensite_tkbrep::topods::Shape {
    let mut maker = BRepPrimAPIMakeCylinder::with_angle(50.8, 508., std::f64::consts::PI)
        .expect("GDML B5 half cylinder");
    let mut shape = maker.shape().expect("GDML B5 half-cylinder shape").clone();
    if reversed {
        shape = Translate(&shape, 0., 0., 254.);
        RotateAroundAxis(&mut shape, Point3::default(), [1., 0., 0.], 180.);
    } else {
        shape = Translate(&shape, 0., 0., -254.);
    }
    final_placement(Translate(&shape, 177.8, y, 0.))
}

fn rail_unit(center_y: f64) -> martensite_tkbrep::topods::Shape {
    let mut box_shape = CreateBox(Point3::new(-25.4, -50.8, -254.), 50.8, 101.6, 508.);
    RotateAroundAxis(&mut box_shape, Point3::default(), [0., 0., -1.], 90.);
    let box_shape = final_placement(Translate(&box_shape, 177.8, center_y, 0.));
    let lower = half_rail(center_y - 25.4, true);
    let upper = half_rail(center_y + 25.4, false);
    PerformFuse(&PerformFuse(&box_shape, &lower), &upper)
}

#[test]
fn GdmlPublic__B5() {
    let source = "tests/boolean/gdml_public/B5";
    let mut result = full_rail(0.);
    for y in [254., 508., -254., -508.] {
        result = PerformFuse(&result, &full_rail(y));
        assert!(!result.is_null(), "{source}: full rail at y={y}");
    }
    for y in [406.4, 152.4, -152.4, -406.4] {
        result = PerformFuse(&result, &rail_unit(y));
        assert!(!result.is_null(), "{source}: rail unit at y={y}");
    }
}
