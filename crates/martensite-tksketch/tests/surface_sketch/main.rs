//! Native feasibility controls for normal-extruded cylindrical rectangles (#416).
use martensite_cmath::{cos, sin};
use martensite_tkbo::body_check::validate_body;
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeRevol;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

fn radial_face(start: f64) -> martensite_tkbrep::topods::Face {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for (radius, z) in [(9., 0.), (12., 0.), (12., 4.), (9., 4.)] {
        polygon
            .add_point(Point3::new(radius * cos(start), radius * sin(start), z))
            .unwrap();
    }
    polygon.close().unwrap();
    BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
}
fn native_solid(start: f64) -> martensite_tkbrep::topods::Shape {
    let axis = Axis1::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    );
    BRepPrimAPIMakeRevol::new(&radial_face(start), axis, 0.7, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}
fn check_native_body(solid: &martensite_tkbrep::topods::Shape) {
    let display = DisplayTolerance {
        chordal: 1e-3,
        angular: 0.1,
    };
    let resolution = Resolution::new(1e-6, 1e-7, 1e-9, display, ToleranceCap::Refuse(1e-5));
    let validation = validate_body(solid, &resolution).unwrap();
    assert!(validation.is_valid(), "{validation:?}");
    let census = &validation.census[0];
    assert_eq!((census.faces, census.edges, census.vertices), (6, 12, 8));
}
fn check_native_volume(solid: &martensite_tkbrep::topods::Shape) {
    let mut properties = GProps::new();
    let error = BRepGProp::volume_properties_gk(
        solid,
        &mut properties,
        1e-9,
        true,
        VolumeGKFlags::default(),
    )
    .unwrap();
    assert!(
        error.is_finite() && (0. ..=1e-9).contains(&error),
        "error={error}"
    );
    assert!(
        (properties.mass() - 88.2).abs() < 1e-6,
        "mass={} error={error}",
        properties.mass()
    );
}
#[test]
fn native_radial_rectangle_revolves_across_the_seam() {
    for start in [0.2, -0.3] {
        let solid = native_solid(start);
        check_native_body(&solid);
        check_native_volume(&solid);
    }
}
mod acceptance;
mod refusals;
mod replay;

mod contracts;

mod native_cone;

mod cone_acceptance;
