//! Native feasibility controls for normal-extruded conical sectors (#417).
use martensite_cmath::{atan2, cos, sin};
use martensite_tkbo::body_check::validate_body;
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkmath::gp::{Axis1, Direction3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeRevol;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

fn radial_face(start: f64, normal: f64) -> martensite_tkbrep::topods::Face {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    let mut corners = [(8., -1.), (8., 2.), (12., 2.), (12., -1.)];
    // Reverse winding for positive sweep orientation when the normal changes sign.
    if normal < 0. {
        corners.reverse();
    }
    for (slant, depth) in corners {
        polygon
            .add_point(section_point(start, normal, slant, depth))
            .unwrap();
    }
    polygon.close().unwrap();
    BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
}
fn native_solid(start: f64, normal: f64) -> martensite_tkbrep::topods::Shape {
    let axis = Axis1::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    );
    BRepPrimAPIMakeRevol::new(&radial_face(start, normal), axis, 0.7, false)
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
fn check_native_volume(solid: &martensite_tkbrep::topods::Shape, expected: f64) {
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
        (properties.mass() - expected).abs() < 1e-6,
        "mass={} error={error}",
        properties.mass()
    );
}
#[test]
fn native_conical_section_revolves_both_normals_across_seam() {
    for (normal, expected) in [(1., 53.76), (-1., 47.04)] {
        for start in [0.2, -0.3] {
            let solid = native_solid(start, normal);
            check_native_body(&solid);
            check_native_volume(&solid, expected);
            check_carriers(&solid);
        }
    }
}

fn section_point(start: f64, normal: f64, slant: f64, depth: f64) -> Point3 {
    let alpha = atan2(3., 4.);
    let radius = slant * sin(alpha) + normal * depth * cos(alpha);
    let z = slant * cos(alpha) - normal * depth * sin(alpha);
    Point3::new(radius * cos(start), radius * sin(start), z)
}

fn check_carriers(solid: &martensite_tkbrep::topods::Shape) {
    let mut faces = Explorer::new(solid, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let (mut cones, mut planes) = (0, 0);
    while faces.more() {
        let face = topods::face(faces.current()).unwrap();
        let surface = BRepAdaptorSurface::with_face(&face, true).unwrap();
        match surface.get_type().unwrap() {
            SurfaceType::Cone => cones += 1,
            SurfaceType::Plane => planes += 1,
            other => panic!("unexpected carrier {other:?}; expected cone or plane"),
        }
        faces.next().unwrap();
    }
    assert_eq!((cones, planes), (4, 2));
}
