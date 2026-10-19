//! Closed-form planar geometry controls, separate from retained numeric observations.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomPlane, GeomSurface, GeomTrimmedCurve};
use martensite_tkgeombase::geom_proj_lib as gpl;
use martensite_tkmath::gp::{Axis3, Direction3, Point3};

fn plane() -> Handle<GeomPlane> {
    Handle::new(GeomPlane::new(Axis3::default()))
}

fn line(origin: Point3, direction: Direction3) -> Handle<dyn GeomCurve> {
    Handle::new(GeomLine::from_point(origin, direction)).into_cast()
}

#[test]
fn line_in_plane_has_closed_form_parameter_coordinates() {
    let source = line(
        Point3::new(1.0, 2.0, 0.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    );
    let surface: Handle<dyn GeomSurface> = plane().cast();
    let projected = gpl::curve2d(&source, &surface).unwrap();
    for parameter in [-2.0, 0.0, 3.0] {
        let point = projected.value(parameter).unwrap();
        assert_eq!((point.x(), point.y()), (1.0 + parameter, 2.0));
    }
    assert_eq!(source.value(0.0).unwrap(), Point3::new(1.0, 2.0, 0.0));
}

#[test]
fn normal_projection_removes_only_line_height() {
    let source = line(
        Point3::new(1.0, 2.0, 3.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    );
    let surface: Handle<dyn GeomSurface> = plane().cast();
    let projected = gpl::project(&source, &surface).unwrap();
    for parameter in [-2.0, 0.0, 3.0] {
        assert_eq!(
            projected.value(parameter).unwrap(),
            Point3::new(1.0 + parameter, 2.0, 0.0)
        );
    }
    assert_eq!(source.value(0.0).unwrap(), Point3::new(1.0, 2.0, 3.0));
}

fn near_point(actual: Point3, expected: Point3) {
    for (coordinate, reference) in [
        (actual.x(), expected.x()),
        (actual.y(), expected.y()),
        (actual.z(), expected.z()),
    ] {
        assert!((coordinate - reference).abs() < 1e-12);
    }
}

#[test]
fn oblique_projection_retains_or_rescales_line_parameters() {
    let basis = line(
        Point3::new(1.0, 2.0, 3.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    );
    let source: Handle<dyn GeomCurve> =
        Handle::new(GeomTrimmedCurve::new(&basis, -1.0, 2.0, true, true).unwrap()).into_cast();
    let direction = Direction3::new(2.0, 0.0, 1.0).unwrap();
    let kept = gpl::project_on_plane(&source, &plane(), direction, true).unwrap();
    let rescaled = gpl::project_on_plane(&source, &plane(), direction, false).unwrap();
    for parameter in [-1.0, 0.0, 2.0] {
        let expected = Point3::new(-5.0 - 2.0 * parameter, 2.0, 0.0);
        near_point(kept.value(parameter).unwrap(), expected);
        near_point(rescaled.value(2.0 * parameter).unwrap(), expected);
    }
    assert_eq!(source.value(0.0).unwrap(), Point3::new(1.0, 2.0, 3.0));
}
