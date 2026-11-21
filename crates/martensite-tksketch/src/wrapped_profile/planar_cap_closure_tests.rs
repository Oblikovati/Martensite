//! Equal coordinates do not replace corner ownership; the closing edge is checked too.
use super::*;
fn request() -> Request {
    super::super::super::super::tests::request()
}
#[test]
fn admitted_contours_close_under_rotation_and_signed_zero() {
    let mut input = request();
    certify(&input).unwrap();
    for contour in &mut input.loops {
        contour.rotate_left(1);
    }
    certify(&input).unwrap();
    assert!(equal_endpoints(Some([-0., 1.]), Some([0., 1.])));
}
#[test]
fn coincident_points_with_different_corner_owners_refuse() {
    let mut input = request();
    input.loops[0][0].end = CornerSource::Authored(SurfaceVertexId(u64::MAX));
    assert!(matches!(certify(&input), Err(ConeFailure::CapJoin { .. })));
}
#[test]
fn the_last_to_first_join_requires_equal_finite_geometry() {
    let mut input = request();
    input.loops[0][0].poles[0][1] += 0.01;
    assert!(matches!(certify(&input), Err(ConeFailure::CapJoin { .. })));
    assert!(!equal_endpoints(Some([f64::NAN, 0.]), Some([f64::NAN, 0.])));
    assert!(!equal_endpoints(
        Some([f64::INFINITY, 0.]),
        Some([f64::INFINITY, 0.])
    ));
}
#[test]
fn empty_contours_or_missing_endpoints_refuse() {
    let mut input = request();
    input.loops[0].clear();
    assert!(matches!(certify(&input), Err(ConeFailure::Binding { .. })));
    assert!(!equal_endpoints(None, Some([0., 0.])));
    assert!(!equal_endpoints(Some([0., 0.]), None));
}
