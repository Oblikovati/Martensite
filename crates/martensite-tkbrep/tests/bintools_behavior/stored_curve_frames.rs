//! Native curve coefficients must have an explicit preserving decode (#455).
use super::fakes::written;
use martensite_tkbrep::bintools::{BinCurveSet, BinOStream, CurveFrameReadPolicy as Policy};
use martensite_tkernel::{standard::Handle, storage::IoStream};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomLine, GeomOffsetCurve, GeomTrimmedCurve};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
fn frame() -> Axis2 {
    Axis2::with_x_direction(
        Point3::new(1.25, -2.5, 3.75),
        Direction3::new(1., 2., 3.).unwrap(),
        Direction3::new(3., -1., 2.).unwrap(),
    )
    .unwrap()
}
fn encode(curve: &Handle<dyn GeomCurve>) -> Vec<u8> {
    written(|stream| BinCurveSet::write_curve(curve, &mut BinOStream::new(stream)).unwrap())
}
#[test]
fn isolate_native_circle_and_line_reconstruction() {
    let axis = frame();
    let curves: [Handle<dyn GeomCurve>; 2] = [
        Handle::new(GeomLine::from_point(axis.location(), axis.direction())).into_cast(),
        Handle::new(GeomCircle::new(axis, 10.).unwrap()).into_cast(),
    ];
    for (kind, curve) in ["line", "circle"].into_iter().zip(curves) {
        let bytes = encode(&curve);
        let restored = BinCurveSet::read_curve_with_frame_policy(
            &mut IoStream::from_bytes(bytes.clone()),
            Policy::PreserveStoredV1,
        )
        .unwrap();
        assert_eq!(encode(&restored), bytes, "{kind}");
    }
}
#[test]
fn bounded_line_directions_retain_native_coefficients() {
    for vector in [
        [1., 1., 1.],
        [2., 3., 7.],
        [3., -1., 2.],
        [1., 2., 3.],
        [7., 11., 13.],
    ] {
        let direction = Direction3::new(vector[0], vector[1], vector[2]).unwrap();
        let curve = Handle::new(GeomLine::from_point(Point3::default(), direction)).into_cast();
        assert_preserved(&curve);
    }
}
fn decode(bytes: Vec<u8>, policy: Policy) -> Handle<dyn GeomCurve> {
    BinCurveSet::read_curve_with_frame_policy(&mut IoStream::from_bytes(bytes), policy).unwrap()
}
#[test]
fn legacy_policy_remains_the_default() {
    let curve = Handle::new(GeomCircle::new(frame(), 10.).unwrap()).into_cast();
    let bytes = encode(&curve);
    let legacy = BinCurveSet::read_curve(&mut IoStream::from_bytes(bytes.clone())).unwrap();
    assert_eq!(
        encode(&legacy),
        encode(&decode(bytes.clone(), Policy::LegacyReplay))
    );
    assert_ne!(encode(&legacy), bytes);
}
#[test]
fn every_conic_and_recursive_basis_retains_bytes() {
    for curve in conics() {
        assert_preserved(&curve);
        let trimmed =
            Handle::new(GeomTrimmedCurve::new(&curve, 0.2, 0.8, true, true).unwrap()).into_cast();
        assert_preserved(&trimmed);
        let offset = Handle::new(
            GeomOffsetCurve::new(&trimmed, 0.5, Direction3::new(2., 3., 7.).unwrap(), false)
                .unwrap(),
        )
        .into_cast();
        assert_preserved(&offset);
    }
}
fn conics() -> [Handle<dyn GeomCurve>; 3] {
    use martensite_tkg3d::geom::{GeomEllipse, GeomHyperbola, GeomParabola};
    [
        Handle::new(GeomEllipse::new(frame(), 3., 2.).unwrap()).into_cast(),
        Handle::new(GeomHyperbola::new(frame(), 3., 2.).unwrap()).into_cast(),
        Handle::new(GeomParabola::new(frame(), 2.).unwrap()).into_cast(),
    ]
}
fn assert_preserved(curve: &Handle<dyn GeomCurve>) {
    let bytes = encode(curve);
    assert_eq!(
        encode(&decode(bytes.clone(), Policy::PreserveStoredV1)),
        bytes
    );
}
#[test]
fn malformed_stored_coefficients_are_refused() {
    let curve = Handle::new(GeomCircle::new(frame(), 10.).unwrap()).into_cast();
    let bytes = encode(&curve);
    for length in 0..bytes.len() {
        assert_refused(bytes[..length].to_vec());
    }
    for value in [0., 2., f64::NAN, f64::INFINITY] {
        let mut bad = bytes.clone();
        for index in 0..3 {
            bad[25 + 8 * index..33 + 8 * index].copy_from_slice(&value.to_ne_bytes());
        }
        assert_refused(bad);
    }
}
#[test]
fn checked_constructors_refuse_indirect_and_nonunit_coefficients() {
    let indirect = Axis2::from_stored_coefficients(
        Point3::default(),
        [0., 0., 1.],
        [1., 0., 0.],
        [0., -1., 0.],
    );
    assert!(indirect.is_err());
    for vector in [
        [0.; 3],
        [2., 0., 0.],
        [f64::NAN, 0., 0.],
        [f64::INFINITY, 0., 0.],
    ] {
        assert!(Direction3::from_stored_coefficients(vector).is_err());
    }
}

fn assert_refused(bytes: Vec<u8>) {
    assert!(
        BinCurveSet::read_curve_with_frame_policy(
            &mut IoStream::from_bytes(bytes),
            Policy::PreserveStoredV1
        )
        .is_err()
    );
}
#[test]
fn embedded_surface_curves_propagate_explicit_policy() {
    use martensite_tkg3d::geom::{
        GeomSurface, GeomSurfaceOfLinearExtrusion, GeomSurfaceOfRevolution,
    };
    use martensite_tkmath::gp::Axis1;
    let curve = Handle::new(GeomCircle::new(frame(), 10.).unwrap()).into_cast();
    let surfaces: [Handle<dyn GeomSurface>; 2] = [
        Handle::new(GeomSurfaceOfLinearExtrusion::new(&curve, Direction3::Z).unwrap()).into_cast(),
        Handle::new(
            GeomSurfaceOfRevolution::new(&curve, Axis1::new(Point3::default(), Direction3::Z))
                .unwrap(),
        )
        .into_cast(),
    ];
    for surface in surfaces {
        assert_surface_preserved(&surface);
    }
}
fn assert_surface_preserved(surface: &Handle<dyn martensite_tkg3d::geom::GeomSurface>) {
    use martensite_tkbrep::bintools::{BinSurfaceSet, SurfaceFrameReadPolicy};
    let encode = |surface| {
        written(|stream| {
            BinSurfaceSet::write_surface(surface, &mut BinOStream::new(stream)).unwrap()
        })
    };
    let bytes = encode(surface);
    let restored = BinSurfaceSet::read_surface_with_geometry_policies(
        &mut IoStream::from_bytes(bytes.clone()),
        SurfaceFrameReadPolicy::PreserveStoredV1,
        Policy::PreserveStoredV1,
    )
    .unwrap();
    assert_eq!(encode(&restored), bytes);
}
#[test]
fn bounded_conic_frames_retain_native_coefficients() {
    for vector in [
        [1., 1., 1.],
        [2., 3., 7.],
        [3., -1., 2.],
        [1., 2., 3.],
        [7., 11., 13.],
    ] {
        let normal = Direction3::new(vector[0], vector[1], vector[2]).unwrap();
        let axis = Axis2::new(Point3::new(-1., 2., -3.), normal).unwrap();
        let curve = Handle::new(GeomCircle::new(axis, 3.).unwrap()).into_cast();
        assert_preserved(&curve);
    }
}
#[test]
fn legacy_conic_scalars_precede_parallel_frame_construction() {
    let circle = Handle::new(GeomCircle::new(frame(), 10.).unwrap()).into_cast();
    for curve in conics().into_iter().chain([circle]) {
        let mut bytes = encode(&curve);
        let main = bytes[25..49].to_vec();
        bytes[49..73].copy_from_slice(&main);
        assert_legacy_failure(bytes.clone(), false);
        bytes.truncate(101);
        assert_legacy_failure(bytes, true);
    }
}
fn assert_legacy_failure(bytes: Vec<u8>, truncated: bool) {
    for explicit in [false, true] {
        let mut stream = IoStream::from_bytes(bytes.clone());
        let error = if explicit {
            BinCurveSet::read_curve_with_frame_policy(&mut stream, Policy::LegacyReplay)
        } else {
            BinCurveSet::read_curve(&mut stream)
        }
        .err()
        .unwrap();
        assert_eq!(
            error.message().contains("Storage_StreamTypeMismatchError"),
            truncated
        );
        stream.clear(0);
        assert_eq!(stream.tellg(), bytes.len() as i64);
    }
}

#[test]
fn curve_frame_policy_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Policy>();
}
