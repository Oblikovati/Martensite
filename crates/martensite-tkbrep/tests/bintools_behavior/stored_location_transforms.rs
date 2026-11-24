//! Frozen placement capture is opt-in; legacy reconstruction remains selectable (#451).
use super::fakes::written;
use martensite_tkbrep::bintools::{
    BinIStream, BinLocationSet, BinOStream, LocationTransformReadPolicy as Policy,
};
use martensite_tkernel::storage::IoStream;
use martensite_tkmath::gp::{Axis3, Direction3, Point3, Transform3};
use martensite_tkmath::toploc::Location;

pub(super) fn oblique(indirect: bool) -> Transform3 {
    let mut axis = Axis3::new(
        Point3::new(1.25, -2.5, 3.75),
        Direction3::new(1., 2., 3.).unwrap(),
        Direction3::new(3., -1., 2.).unwrap(),
    )
    .unwrap();
    if indirect {
        axis.y_reverse();
    }
    let mut transform = Transform3::default();
    transform.set_transformation_to(axis);
    transform
}
fn rows(transform: &Transform3) -> [[f64; 4]; 3] {
    std::array::from_fn(|r| {
        std::array::from_fn(|c| transform.value(r as i32 + 1, c as i32 + 1).unwrap())
    })
}
pub(super) fn bits(transform: &Transform3) -> [[u64; 4]; 3] {
    rows(transform).map(|row| row.map(f64::to_bits))
}
fn decode(
    matrix: [[f64; 4]; 3],
    policy: Policy,
) -> Result<Transform3, martensite_tkernel::standard::Failure> {
    let bytes = written(|stream| {
        let mut out = BinOStream::new(stream);
        for row in matrix {
            for value in row {
                out.write_real(value);
            }
        }
    });
    BinIStream::new(&mut IoStream::from_bytes(bytes)).read_transform_with_policy(policy)
}
#[test]
fn stored_oblique_and_already_normalized_transforms_replay_bit_exactly() {
    for indirect in [false, true] {
        let original = oblique(indirect);
        let mut normalized = Transform3::default();
        normalized.set_values(rows(&original)).unwrap();
        for original in [original, normalized] {
            replay_exactly(original, indirect);
        }
    }
}
fn replay_exactly(original: Transform3, indirect: bool) {
    let mut current = original;
    for _ in 0..4 {
        current = decode(rows(&current), Policy::PreserveStoredRigidV1).unwrap();
        assert_eq!(bits(&current), bits(&original));
        assert_eq!(current.is_negative(), indirect);
    }
}
#[test]
fn location_tables_retain_elementary_datum_coefficients() {
    for indirect in [false, true] {
        let original = oblique(indirect);
        let mut current = original;
        for _ in 0..4 {
            current = location_replay(current);
            assert_eq!(bits(&current), bits(&original));
        }
    }
}
fn location_replay(current: Transform3) -> Transform3 {
    let bytes = written(|stream| {
        let mut table = BinLocationSet::new();
        table.add(&Location::new(current));
        table.write(stream).unwrap();
    });
    let mut table = BinLocationSet::new();
    table
        .read_with_policy(
            &mut IoStream::from_bytes(bytes),
            Policy::PreserveStoredRigidV1,
        )
        .unwrap();
    table.location(1).unwrap().transformation()
}
#[test]
fn legacy_default_stays_equal_to_explicit_normalization() {
    let original = oblique(false);
    let matrix = rows(&original);
    let bytes = written(|stream| BinOStream::new(stream).write_transform(&original));
    let actual = BinIStream::new(&mut IoStream::from_bytes(bytes))
        .read_transform()
        .unwrap();
    let explicit = decode(matrix, Policy::LegacyReplay).unwrap();
    assert_eq!(bits(&actual), bits(&explicit));
    assert_ne!(bits(&actual), bits(&original));
    assert_eq!(bits(&actual)[0][1], 0xbfe3c03650e00e04);
}
#[test]
fn signed_zero_translation_reflection_and_inverse_keep_rigid_semantics() {
    let matrix = [[-1., -0., 0., -0.], [0., 1., -0., 2.], [-0., 0., 1., -3.]];
    let original = decode(matrix, Policy::PreserveStoredRigidV1).unwrap();
    assert_eq!(bits(&original), matrix.map(|row| row.map(f64::to_bits)));
    assert_eq!(original.scale_factor(), -1.);
    assert!(original.is_negative());
    let restored = original.inverted().unwrap();
    assert_eq!(restored.transforms_coordinates(4., 5., 6.), (-4., 3., 9.));
    assert_eq!(original.transforms_coordinates(-4., 3., 9.), (4., 5., 6.));
}
#[test]
fn stored_policy_refuses_unsupported_or_invalid_matrices() {
    let identity = rows(&Transform3::default());
    for value in [f64::NAN, f64::INFINITY, 0., 2., 1.000001] {
        let mut matrix = identity;
        matrix[0][0] = value;
        assert!(
            decode(matrix, Policy::PreserveStoredRigidV1).is_err(),
            "{matrix:?}"
        );
    }
}
#[test]
fn stored_policy_refuses_shear_nonfinite_origin_and_uniform_scale() {
    let identity = rows(&Transform3::default());
    let mut shear = identity;
    shear[0][1] = 0.125;
    assert!(decode(shear, Policy::PreserveStoredRigidV1).is_err());
    let mut nonfinite_origin = identity;
    nonfinite_origin[0][3] = f64::NAN;
    assert!(decode(nonfinite_origin, Policy::PreserveStoredRigidV1).is_err());
    let scaled = identity.map(|row| row.map(|value| value * 2.));
    assert!(decode(scaled, Policy::PreserveStoredRigidV1).is_err());
    assert!(decode(scaled, Policy::LegacyReplay).is_ok());
}

#[test]
fn stored_constructor_is_send_sync_and_admits_identity_translation() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Policy>();
    for original in [
        Transform3::default(),
        Transform3::translation([1., -2., 3.]),
    ] {
        let captured = Transform3::from_stored_rigid_coefficients(rows(&original)).unwrap();
        assert_eq!(bits(&captured), bits(&original));
    }
}
#[test]
fn oblique_inverse_matches_transpose_and_returns_points_with_roundoff_only() {
    for indirect in [false, true] {
        let captured = decode(rows(&oblique(indirect)), Policy::PreserveStoredRigidV1).unwrap();
        let inverse = captured.inverted().unwrap();
        check_transpose(captured, inverse);
        let (x, y, z) = captured.transforms_coordinates(2., -3., 4.);
        let recovered = inverse.transforms_coordinates(x, y, z);
        for (actual, expected) in [recovered.0, recovered.1, recovered.2]
            .into_iter()
            .zip([2., -3., 4.])
        {
            assert!(
                (actual - expected).abs() < 32. * f64::EPSILON,
                "{actual} != {expected}"
            );
        }
    }
}

fn check_transpose(captured: Transform3, inverse: Transform3) {
    for r in 1..=3 {
        for c in 1..=3 {
            assert_eq!(
                inverse.value(r, c).unwrap().to_bits(),
                captured.value(c, r).unwrap().to_bits()
            );
        }
    }
}

#[test]
fn location_table_rejects_scaled_datum_without_legacy_fallback() {
    let mut bytes = b"Locations 1\n\x01".to_vec();
    let scaled = rows(&Transform3::scaling(Point3::default(), 2.).unwrap());
    for value in scaled.into_iter().flatten() {
        bytes.extend_from_slice(&value.to_ne_bytes());
    }
    let mut table = BinLocationSet::new();
    assert!(
        table
            .read_with_policy(
                &mut IoStream::from_bytes(bytes.clone()),
                Policy::PreserveStoredRigidV1
            )
            .is_err()
    );
    table.read(&mut IoStream::from_bytes(bytes)).unwrap();
    let restored = table.location(1).unwrap().transformation();
    assert_eq!(restored.scale_factor(), 2.);
}
