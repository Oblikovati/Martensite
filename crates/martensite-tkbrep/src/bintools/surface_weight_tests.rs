//! Stored coefficient replay and native refusal category controls (#464).
use super::*;
use crate::bintools::{BinOStream, BinShapeSet, BinShapeSetBase, BinSurfaceSet};
use crate::{brep::BRepBuilder, topods::Face};
use martensite_tkernel::message::ProgressRange;

fn surface() -> Handle<GeomBezierSurface> {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    let mut weights = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for (index, [u, v]) in [[0, 0], [0, 1], [1, 0], [1, 1]].into_iter().enumerate() {
        let (row, col) = (index as i32 / 2 + 1, index as i32 % 2 + 1);
        let point = Point3::new(f64::from(u), f64::from(v), 0.);
        let weight = if u == 0 {
            1.
        } else {
            f64::from_bits(1f64.to_bits() - 1)
        };
        poles.set_value(row, col, point).unwrap();
        weights.set_value(row, col, weight).unwrap();
    }
    Handle::new(GeomBezierSurface::from_stored_coefficients(&poles, &weights).unwrap())
}
fn encoded(surface: &Handle<dyn GeomSurface>) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    BinSurfaceSet::write_surface(surface, &mut BinOStream::new(&mut stream)).unwrap();
    let mut bytes = vec![0; stream.tellp() as usize];
    stream.seekg(0).read(&mut bytes);
    bytes
}
fn decoded(
    bytes: &[u8],
    policy: BezierSurfaceWeightReadPolicy,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    read_surface_with_policies(
        &mut IoStream::from_bytes(bytes.to_vec()),
        SurfaceFrameReadPolicy::LegacyReplay,
        policy,
    )
}
fn check_retained(surface: &Handle<dyn GeomSurface>) {
    let curve: Handle<GeomBezierSurface> = surface.cast();
    assert_eq!(curve.weight(2, 1).unwrap().to_bits(), 1f64.to_bits() - 1);
    assert!(curve.is_u_rational());
    assert!(!curve.is_v_rational());
}
#[test]
fn strict_decode_preserves_coefficients_and_legacy_decode_keeps_demotion() {
    let bytes = encoded(&surface().cast());
    let strict = decoded(&bytes, BezierSurfaceWeightReadPolicy::PreserveStoredV1).unwrap();
    check_retained(&strict);
    assert_eq!(encoded(&strict), bytes);
    let legacy = decoded(&bytes, BezierSurfaceWeightReadPolicy::LegacyReplay).unwrap();
    let legacy: Handle<GeomBezierSurface> = legacy.cast();
    assert_eq!(legacy.weight(2, 1).unwrap(), 1.);
}
#[test]
fn grouped_shape_set_applies_the_explicit_policy_to_its_surface_table() {
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &surface().cast(), 1e-7)
        .unwrap();
    let mut original = BinShapeSet::new();
    original.add(&face).unwrap();
    let mut stream = IoStream::from_bytes(Vec::new());
    original
        .write(&mut stream, &ProgressRange::default())
        .unwrap();
    stream.seekg(0);
    let mut restored = BinShapeSet::new();
    restored.set_bezier_surface_weight_read_policy(BezierSurfaceWeightReadPolicy::PreserveStoredV1);
    restored
        .read(&mut stream, &ProgressRange::default())
        .unwrap();
    check_retained(&restored.surfaces.surface(1).unwrap());
}
#[test]
fn constant_nonunit_record_preserves_the_refusal_category_and_bits() {
    let mut bytes = encoded(&surface().cast());
    for index in 0..4 {
        bytes[7 + index * 32 + 24..7 + index * 32 + 32].copy_from_slice(&2f64.to_ne_bytes());
    }
    let failure = decoded(&bytes, BezierSurfaceWeightReadPolicy::PreserveStoredV1).unwrap_err();
    assert_eq!(
        failure.exception_type(),
        BezierSurfaceCoefficientError::CONSTANT_NON_UNIT_EXCEPTION
    );
    assert!(failure.to_string().contains("0x4000000000000000"));
    assert!(decoded(&bytes, BezierSurfaceWeightReadPolicy::LegacyReplay).is_ok());
}
#[test]
fn nested_basis_surfaces_keep_the_same_weight_policy() {
    let basis = surface().cast();
    let trimmed =
        GeomRectangularTrimmedSurface::new(&basis, (0.1, 0.9), (0.2, 0.8), (true, true)).unwrap();
    let bytes = encoded(&Handle::new(trimmed).cast());
    let restored = decoded(&bytes, BezierSurfaceWeightReadPolicy::PreserveStoredV1).unwrap();
    let restored: Handle<GeomRectangularTrimmedSurface> = restored.cast();
    check_retained(&restored.basis_surface());
    let offset = GeomOffsetSurface::new(&basis, 0.5, false).unwrap();
    let bytes = encoded(&Handle::new(offset).cast());
    let restored = decoded(&bytes, BezierSurfaceWeightReadPolicy::PreserveStoredV1).unwrap();
    let restored: Handle<GeomOffsetSurface> = restored.cast();
    check_retained(&restored.basis_surface());
}
#[test]
fn read_policy_is_send_and_sync_and_defaults_to_legacy() {
    let policy = BezierSurfaceWeightReadPolicy::default();
    let _: &(dyn Send + Sync) = &policy;
    assert_eq!(
        BezierSurfaceWeightReadPolicy::default(),
        BezierSurfaceWeightReadPolicy::LegacyReplay
    );
}
#[test]
fn omitted_legacy_weights_are_unit_and_are_not_claimed_to_be_recovered() {
    let polynomial = GeomBezierSurface::new(&surface().poles()).unwrap();
    let bytes = encoded(&Handle::new(polynomial).cast());
    let restored = decoded(&bytes, BezierSurfaceWeightReadPolicy::PreserveStoredV1).unwrap();
    let retained: Handle<GeomBezierSurface> = restored.cast();
    assert_eq!(retained.weight(2, 1).unwrap(), 1.);
    assert!(!retained.is_u_rational());
    assert!(!retained.is_v_rational());
    assert_eq!(encoded(&retained.cast()), bytes);
}
fn failed_prefix(bytes: &[u8], policy: BezierSurfaceWeightReadPolicy) -> (String, i64) {
    let mut stream = IoStream::from_bytes(bytes.to_vec());
    let failure =
        read_surface_with_policies(&mut stream, SurfaceFrameReadPolicy::LegacyReplay, policy)
            .unwrap_err();
    (failure.to_string(), stream.tellg())
}
#[test]
fn truncated_records_keep_legacy_error_precedence_and_cursor() {
    let bytes = encoded(&surface().cast());
    for length in 0..bytes.len() {
        let legacy = failed_prefix(
            &bytes[..length],
            BezierSurfaceWeightReadPolicy::LegacyReplay,
        );
        let strict = failed_prefix(
            &bytes[..length],
            BezierSurfaceWeightReadPolicy::PreserveStoredV1,
        );
        assert_eq!(strict, legacy, "prefix length {length}");
    }
}
