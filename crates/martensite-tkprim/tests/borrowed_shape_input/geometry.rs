//! Literal dimensions and closed-form mass independently constrain decoded geometry.
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::BRepAdaptorSurface,
    topexp::Explorer,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::collections::BTreeSet;
/// Check topology, analytic planes, literal corners and mass; `assert_box(&shape)`.
pub(super) fn assert_box(shape: &Shape) {
    assert_eq!(shape.shape_type().unwrap(), ShapeEnum::Solid);
    for (kind, count) in [
        (ShapeEnum::Solid, 1),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Face, 6),
        (ShapeEnum::Wire, 6),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Vertex, 8),
    ] {
        assert_eq!(unique(shape, kind).len(), count);
    }
    assert_planes(shape);
    assert_corners(shape);
    assert_volume(shape);
}
fn unique(shape: &Shape, kind: ShapeEnum) -> IndexedMapOfShape {
    let mut entities = IndexedMapOfShape::default();
    for entity in Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
        entities.add(entity);
    }
    entities
}
fn assert_planes(shape: &Shape) {
    for entity in unique(shape, ShapeEnum::Face).iter() {
        let face = topods::face(entity).unwrap();
        assert_eq!(
            BRepAdaptorSurface::with_face(&face, true)
                .unwrap()
                .get_type()
                .unwrap(),
            SurfaceType::Plane
        );
    }
}
fn assert_corners(shape: &Shape) {
    let mut corners = BTreeSet::new();
    for entity in unique(shape, ShapeEnum::Vertex).iter() {
        let point = BRepTool::pnt(&topods::vertex(entity).unwrap()).unwrap();
        assert_axis_endpoint(point.x(), 10.0);
        assert_axis_endpoint(point.y(), 20.0);
        assert_axis_endpoint(point.z(), 30.0);
        corners.insert([point.x() == 10.0, point.y() == 20.0, point.z() == 30.0]);
    }
    assert_eq!(corners.len(), 8);
}
fn assert_volume(shape: &Shape) {
    let mut properties = GProps::default();
    let error =
        BRepGProp::volume_properties_eps(shape, &mut properties, 1e-12, true, false).unwrap();
    let mass = properties.mass();
    let budget: f64 = 64.0 * f64::EPSILON * 6000.0;
    println!(
        "{{\"rectangular_mass\":{mass:.17},\"integration_error\":{error:.17},\"roundoff_budget\":{budget:.17}}}"
    );
    assert!(
        (mass - 6000.0).abs() <= 64.0 * f64::EPSILON * 6000.0,
        "mass {mass:.17}, expected 6000 within {budget:.17}"
    );
    assert!(
        error.is_finite(),
        "integration error {error:.17}, expected finite error"
    );
}
fn assert_axis_endpoint(actual: f64, extent: f64) {
    assert!(
        actual == 0.0 || actual == extent,
        "coordinate {actual:.17}, expected 0 or {extent:.17}"
    );
}
