//! Earlier admission preserves the referring owner and refusal precedence.
use super::*;
fn old(face: &Face, owners: usize) -> FaceCaptureIncomplete {
    let limits = FaceCaptureLimits {
        owners,
        ..FaceCaptureLimits::bounded()
    };
    capture_face_input(face, limits, || false).unwrap_err()
}
fn assert_refusal(error: FaceCaptureIncomplete, owner: usize, expected: &'static str) {
    match error {
        FaceCaptureIncomplete::Unsupported {
            owner: actual,
            expected: shape,
            ..
        } => {
            assert_eq!(actual, owner);
            assert_eq!(shape, expected);
        }
        other => panic!("{other:?}; expected Unsupported at owner {owner} with shape {expected}"),
    }
}
#[test]
fn tensor_refusal_stays_at_referring_face_before_new_owner_limits() {
    let (face, _, _) = surface_face();
    for owners in [0, 1, 2, 4096] {
        assert_refusal(
            old(&face, owners),
            0,
            "stored plane or linear extrusion surface",
        );
    }
}
#[test]
fn regularity_refusal_stays_at_referring_edge_before_new_owner_limits() {
    let (face, _) = face_fixture();
    let _record = regularity(&face, &tensor());
    for owners in [3, 4, 4096] {
        assert_refusal(
            old(&face, owners),
            1,
            "spatial curve or one/two line pcurves",
        );
    }
}
#[test]
fn unknown_surface_keeps_earlier_expected_shape_and_owner() {
    let (face, fields) = face_fixture();
    fields.set_surface(&Handle::default());
    assert_refusal(
        old(&face, 4096),
        0,
        "stored plane or linear extrusion surface",
    );
}
#[test]
fn first_regular_record_refuses_before_later_unknown_representation() {
    let (face, _) = face_fixture();
    let record = regularity(&face, &tensor());
    let edge = Handle::new(BRepTEdge::new());
    edge.change_curves(|curves| curves.extend([record.cast(), Handle::default()]));
    let (first, _) = face_fixture();
    representations::append_owner(&first, edge.cast());
    let error = old(&first, 4096);
    assert!(matches!(&error,FaceCaptureIncomplete::Unsupported {actual,..} if actual != "null"));
    assert_refusal(error, 1, "spatial curve or one/two line pcurves");
}
#[test]
fn null_spatial_curve_keeps_the_earlier_geometry_refusal() {
    let (face, _) = face_fixture();
    let _record = absent_spatial(&face);
    let error = old(&face, 4096);
    assert!(matches!(&error,FaceCaptureIncomplete::Unsupported {actual,..} if actual == "null"));
    assert_refusal(error, 3, "stored line or clamped spline curve");
}
