//! #592: a composed BRep adaptor must retain the native specialized grid path.
use super::*;
use martensite_tkg3d::geom::{GeomCircle, GeomSurfaceOfLinearExtrusion};
use martensite_tkg3d::geom_adaptor::GeomTransformedSurfaceSource;
use martensite_tkg3d::geom_grid_eval::{GeomGridEvalSurface, GeomGridEvalSurfaceEvaluator};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Transform3, TransformForm, Vector3};
use martensite_tkmath::toploc::Location;

fn extrusion_source() -> Handle<dyn GeomSurface> {
    let normal = Direction3::new(0.3, -0.4, 0.5).unwrap();
    let frame =
        Axis2::with_x_direction(Point3::new(1.25, -2.5, 0.75), normal, Direction3::X).unwrap();
    let curve = Handle::new(GeomCircle::new(frame, 1.5).unwrap()).into_upcast();
    Handle::new(
        GeomSurfaceOfLinearExtrusion::new(&curve, Direction3::new(1., 1., 1.).unwrap()).unwrap(),
    )
    .into_upcast()
}
fn face_adaptor(
    geometry: &Handle<dyn GeomSurface>,
    transform: Transform3,
) -> Handle<BRepAdaptorSurface> {
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, geometry, 1e-7)
        .unwrap();
    face.set_location(&Location::new(transform), false).unwrap();
    Handle::new(BRepAdaptorSurface::with_face(&face, false).unwrap())
}
fn parameters() -> (Array1<'static, f64>, Array1<'static, f64>) {
    (
        Array1::from_shared(&[0., 0.37, 1.1, 2.9], 1),
        Array1::from_shared(&[-0., 0.25, 0.91], 1),
    )
}
fn grid_bits(grid: &Array2<'_, Point3>) -> Vec<[u64; 3]> {
    grid.as_slice()
        .iter()
        .map(|p| [p.x().to_bits(), p.y().to_bits(), p.z().to_bits()])
        .collect()
}
fn assert_grid(geometry: &Handle<dyn GeomSurface>, transform: Transform3) {
    let adaptor = face_adaptor(geometry, transform);
    let source = adaptor.upcast::<dyn GeomTransformedSurfaceSource>();
    assert_eq!(adaptor.object_address(), source.object_address());
    assert!(std::ptr::eq(
        source.transformed_surface(),
        adaptor.transformed_surface()
    ));
    let actual = GeomGridEvalSurface::from_adaptor(&*adaptor).unwrap();
    assert!(matches!(
        actual.evaluator(),
        GeomGridEvalSurfaceEvaluator::Extrusion(_)
    ));
    let (u, v) = parameters();
    let expected = expected_grid_bits(geometry, transform);
    assert_eq!(grid_bits(&actual.evaluate_grid(&u, &v).unwrap()), expected);
}

fn expected_grid_bits(geometry: &Handle<dyn GeomSurface>, transform: Transform3) -> Vec<[u64; 3]> {
    let (u, v) = parameters();
    let native = GeomGridEvalSurface::from_surface(geometry)
        .evaluate_grid(&u, &v)
        .unwrap();
    native
        .as_slice()
        .iter()
        .map(|point| placed_bits(*point, transform))
        .collect()
}

fn placed_bits(point: Point3, transform: Transform3) -> [u64; 3] {
    let placed = if transform.form() == TransformForm::Identity {
        point
    } else {
        point.transformed(transform)
    };
    [
        placed.x().to_bits(),
        placed.y().to_bits(),
        placed.z().to_bits(),
    ]
}

#[test]
fn composed_extrusion_uses_exact_native_grid_instead_of_scalar_fallback() {
    assert_grid(&extrusion_source(), Transform3::default());
}
#[test]
fn composed_extrusion_retains_one_nonidentity_rotation_and_translation() {
    let mut transform = Transform3::default();
    transform
        .set_rotation(Axis1::new(Point3::new(0.25, -0.5, 1.), Direction3::Z), 0.37)
        .unwrap();
    transform.set_translation_part(Vector3::new(8., -4., 2.));
    assert_grid(&extrusion_source(), transform);
}
