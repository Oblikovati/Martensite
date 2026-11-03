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
fn matrix_bits(transform: Transform3) -> [u64; 12] {
    std::array::from_fn(|index| {
        transform
            .value((index / 4 + 1) as i32, (index % 4 + 1) as i32)
            .unwrap()
            .to_bits()
    })
}
fn mapped_face(source: &Location) -> (Face, Handle<BRepTFace>) {
    let (mut face, fields) = face_fixture();
    face.set_location(source, false).unwrap();
    fields.set_location(source);
    let surface = fields.surface().cast::<GeomSurfaceOfLinearExtrusion>();
    let edge = representations::seam_edge(&surface);
    let seam = edge.curves()[1].cast::<BRepCurveOnClosedSurface>();
    seam.curve_representation_base().set_location(source);
    representations::append_owner(&face, edge.cast());
    (face, fields)
}
fn assert_map_bits(location: &CapturedLocation, source: &Location) {
    assert_eq!(
        matrix_bits(location.effective_transform()),
        matrix_bits(source.transformation())
    );
}
fn assert_mapped_roles(snapshot: &FaceInputSnapshot, source: &Location) {
    assert_map_bits(&snapshot.root().location, source);
    assert_map_bits(face_location(snapshot), source);
    let representation = snapshot
        .cells()
        .iter()
        .find_map(|(_, cell)| match cell {
            CapturedFaceCell::Location(location) if !location.steps().is_empty() => Some(location),
            _ => None,
        })
        .unwrap();
    assert_map_bits(representation, source);
}
fn assert_rounded_chain(location: &CapturedLocation) {
    assert_eq!(location.steps().len(), 3);
    assert!(location.steps().iter().any(|(_, power, _)| *power == 3));
    // The exact translation sum is 3; the stored binary64 chain rounds to 4.
    assert_eq!(cached_x_bits(location), 4f64.to_bits());
}
#[test]
fn rounded_multidatum_power_cache_is_bit_exact_and_survives_replacement() {
    let source = rounded_power_chain();
    let (face, fields) = mapped_face(&source);
    let snapshot = captured(&face);
    let location = face_location(&snapshot);
    assert_rounded_chain(location);
    assert_mapped_roles(&snapshot, &source);
    fields.set_location(&translation(99.));
    assert_map_bits(location, &source);
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
