use super::super::brep_curve_numeric::fixture_guards::RecordedCurveInput;
use super::replay::{brep_face_fixture, brep_surface_print};
use martensite_tkgeomalgo::int_curve_surface::{IntersectionPoint, TransitionOnCurve};
use martensite_tkmath::gp::Point3;

#[test]
fn plane_fixture_skips_radius_and_later_face_bounds() {
    let mut input = RecordedCurveInput::new(0, false);
    assert!(brep_face_fixture::geom_surface(&mut input).is_ok());
    assert_eq!(input.reads, ["kind", "point", "direction", "direction"]);
}
#[test]
fn invalid_surface_frame_fails_before_radius_or_face_bounds() {
    let mut input = RecordedCurveInput::new(1, true);
    assert!(brep_face_fixture::face(&mut input).is_err());
    assert_eq!(input.reads, ["kind", "point", "direction", "direction"]);
}

#[derive(Debug, PartialEq)]
enum SurfacePointField {
    Point(Point3),
    Real(f64),
    Integer(i32),
}
#[derive(Default)]
struct RecordedSurfacePointEmitter(Vec<SurfacePointField>);
impl brep_surface_print::SurfacePointEmitter for RecordedSurfacePointEmitter {
    fn pnt(&mut self, point: Point3) {
        self.0.push(SurfacePointField::Point(point));
    }
    fn real(&mut self, value: f64) {
        self.0.push(SurfacePointField::Real(value));
    }
    fn int(&mut self, value: i32) {
        self.0.push(SurfacePointField::Integer(value));
    }
}
#[test]
fn surface_intersection_point_preserves_coordinates_parameters_then_transition() {
    let point = IntersectionPoint::new(Point3::new(1., 2., 3.), 4., 5., 6., TransitionOnCurve::Out);
    let mut emit = RecordedSurfacePointEmitter::default();
    brep_surface_print::ics_point(&mut emit, &point);
    assert_eq!(
        emit.0,
        [
            SurfacePointField::Point(Point3::new(1., 2., 3.)),
            SurfacePointField::Real(4.),
            SurfacePointField::Real(5.),
            SurfacePointField::Real(6.),
            SurfacePointField::Integer(2)
        ]
    );
}
