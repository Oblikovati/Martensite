//! Preserve the actual rounded evaluator cache alongside its immutable source chain.
use super::*;
use martensite_tkmath::gp::{Transform3, Vector3};

fn translation(value: f64) -> Location {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(value, 0., 0.));
    Location::new(transform)
}
fn rounded_power_chain() -> Location {
    let first = translation(1e16);
    let middle = translation(1.).powered(3).unwrap();
    first
        .multiplied(&middle)
        .unwrap()
        .multiplied(&translation(-1e16))
        .unwrap()
}
fn face_location(snapshot: &FaceInputSnapshot) -> &CapturedLocation {
    snapshot
        .cells()
        .iter()
        .find_map(|(_, cell)| match cell {
            CapturedFaceCell::Face { location, .. } => Some(location),
            _ => None,
        })
        .unwrap()
}
#[test]
fn identity_location_keeps_the_actual_identity_cache() {
    let (face, _) = face_fixture();
    let snapshot = captured(&face);
    assert!(snapshot.root().location.steps().is_empty());
    assert_eq!(
        snapshot.root().location.effective_transform(),
        Transform3::default()
    );
    assert_eq!(
        face_location(&snapshot).effective_transform(),
        Transform3::default()
    );
}
fn cached_x_bits(location: &CapturedLocation) -> u64 {
    location
        .effective_transform()
        .value(1, 4)
        .unwrap()
        .to_bits()
}
#[test]
fn rounded_multidatum_power_cache_is_bit_exact_and_survives_replacement() {
    let (face, fields) = face_fixture();
    let source = rounded_power_chain();
    fields.set_location(&source);
    let snapshot = captured(&face);
    let location = face_location(&snapshot);
    assert_eq!(location.steps().len(), 3);
    assert!(location.steps().iter().any(|(_, power, _)| *power == 3));
    // The exact translation sum is 3; the stored binary64 chain rounds to 4.
    assert_eq!(cached_x_bits(location), 4f64.to_bits());
    assert_eq!(location.effective_transform(), source.transformation());
    fields.set_location(&translation(99.));
    assert_eq!(cached_x_bits(location), 4f64.to_bits());
    assert_eq!(
        cached_x_bits(face_location(&captured(&face))),
        99f64.to_bits()
    );
}
fn overflowing_location() -> Location {
    let mut transform = Transform3::default();
    transform.set_scale(Point3::default(), 1e200).unwrap();
    assert!(transform.value(1, 1).unwrap().is_finite());
    Location::new(transform).powered(2).unwrap()
}
#[test]
fn finite_datums_with_overflowing_effective_cache_are_incomplete() {
    let (face, fields) = face_fixture();
    let location = overflowing_location();
    assert!(!location.transformation().value(1, 1).unwrap().is_finite());
    fields.set_location(&location);
    assert!(matches!(
        capture_closed_extrusion_face(&face, FaceCaptureLimits::bounded(), || false),
        Err(FaceCaptureIncomplete::NonFinite {
            field: "location transform",
            ..
        })
    ));
}
