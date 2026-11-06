//! Shared analytic construction for native volume-maker regression cases.
use martensite_tkbo::bop_algo::BOPAlgoMakerVolume;
use martensite_tkbrep::{
    topexp::{self, Explorer},
    topods::Shape,
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Axis3, Cone3, Cylinder3, Direction3, Plane3, Point3, Sphere3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use martensite_tktopalgo::brep_gprop::BRepGProp;

#[allow(dead_code)]
pub fn faces(shape: &Shape) -> List<Shape> {
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut result = List::default();
    while explorer.more() {
        result.append(explorer.current().clone());
        explorer.next().unwrap();
    }
    result
}

pub fn plane_face(origin: Point3, normal: Direction3) -> Shape {
    BRepBuilderAPIMakeFace::from_plane_bounds(
        Plane3::new(origin, normal).unwrap(),
        (-1e6, 1e6, -1e6, 1e6),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape()
}

#[allow(dead_code)]
pub fn cone_face(origin: Point3, axis: Direction3, angle: f64, radius: f64) -> Shape {
    let frame = Axis3::from_axis2(Axis2::new(origin, axis).unwrap());
    let cone = Cone3::new(frame, angle.to_radians(), radius).unwrap();
    BRepBuilderAPIMakeFace::from_cone_bounds(cone, (0., std::f64::consts::TAU, 0., 1e6))
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

#[allow(dead_code)]
pub fn cylinder_face(origin: Point3, axis: Direction3, radius: f64) -> Shape {
    let frame = Axis3::from_axis2(Axis2::new(origin, axis).unwrap());
    BRepBuilderAPIMakeFace::from_cylinder_bounds(
        Cylinder3::new(frame, radius).unwrap(),
        (0., std::f64::consts::TAU, -1e6, 1e6),
    )
    .unwrap()
    .face()
    .unwrap()
    .into_shape()
}

/// Builds the untrimmed spherical boundary used by volume-maker regressions.
#[allow(dead_code)]
pub fn sphere_face(radius: f64) -> Shape {
    BRepBuilderAPIMakeFace::from_sphere(Sphere3::new(Axis3::default(), radius).unwrap())
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

/// Builds an untrimmed sphere with the source surface frame.
#[allow(dead_code)]
pub fn sphere_face_at(origin: Point3, axis: Direction3, radius: f64) -> Shape {
    let frame = Axis3::from_axis2(Axis2::new(origin, axis).unwrap());
    BRepBuilderAPIMakeFace::from_sphere(Sphere3::new(frame, radius).unwrap())
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

/// Executes volume construction and returns the published result.
#[allow(dead_code)]
pub fn make_volume(inputs: List<Shape>) -> Shape {
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_arguments(&inputs);
    maker.options_mut().set_run_parallel(false);
    maker.perform();
    assert!(!maker.options().has_errors());
    maker.shape().clone()
}

/// Checks the source regression's relative surface tolerance.
#[allow(dead_code)]
pub fn expect_surface(shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false).unwrap();
    assert!((properties.mass() - expected).abs() / expected <= 0.01);
}

/// Checks a source regression's shape-count assertion.
#[allow(dead_code)]
pub fn expect_shape_count(shape: &Shape, kind: ShapeEnum, expected: usize) {
    let mut found = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut found).unwrap();
    assert_eq!(found.extent(), expected, "{kind:?}");
}

/// Checks that a source regression expects no surface in the result.
#[allow(dead_code)]
pub fn expect_empty_surface(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false).unwrap();
    assert!(properties.mass().abs() <= f64::EPSILON);
}
