//! Exact coefficient capture uses an opt-in policy; default replay stays stable.
use super::fakes::{stream_bytes, written};
use martensite_tkbrep::bintools::{
    self, BinOStream, BinShapeReader, BinShapeSetBase, BinShapeWriter, BinSurfaceSet,
    SurfaceFrameReadPolicy as Policy, WriteOptions,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::geom::{
    GeomConicalSurface, GeomCylindricalSurface, GeomOffsetSurface, GeomPlane,
    GeomRectangularTrimmedSurface, GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
};
use martensite_tkmath::gp::{Axis3, Direction3, Point3};

fn assert_send_sync<T: Send + Sync>() {}

fn frame(indirect: bool) -> Axis3 {
    let mut frame = Axis3::new(
        Point3::new(7., -11., 5.),
        Direction3::new(2., 3., 4.).unwrap(),
        Direction3::new(4., -2., 1.).unwrap(),
    )
    .unwrap();
    if indirect {
        frame.y_reverse();
    }
    frame
}
fn surfaces(axis: Axis3) -> Vec<Handle<dyn GeomSurface>> {
    vec![
        Handle::new(GeomPlane::new(axis)).into_cast(),
        Handle::new(GeomCylindricalSurface::new(axis, 2.).unwrap()).into_cast(),
        Handle::new(GeomConicalSurface::new(axis, 0.3, 2.).unwrap()).into_cast(),
        Handle::new(GeomSphericalSurface::new(axis, 2.).unwrap()).into_cast(),
        Handle::new(GeomToroidalSurface::new(axis, 4., 1.).unwrap()).into_cast(),
    ]
}
fn encode(surface: &Handle<dyn GeomSurface>) -> Vec<u8> {
    written(|stream| BinSurfaceSet::write_surface(surface, &mut BinOStream::new(stream)).unwrap())
}
fn decode(bytes: Vec<u8>, policy: Policy) -> Handle<dyn GeomSurface> {
    BinSurfaceSet::read_surface_with_frame_policy(&mut IoStream::from_bytes(bytes), policy).unwrap()
}
#[test]
fn analytic_records_retain_every_stored_byte_for_direct_and_indirect_frames() {
    assert_send_sync::<Policy>();
    for indirect in [false, true] {
        for surface in surfaces(frame(indirect)) {
            let bytes = encode(&surface);
            assert_eq!(
                encode(&decode(bytes.clone(), Policy::PreserveStoredV1)),
                bytes
            );
        }
    }
}
#[test]
fn recursive_trimmed_and_offset_surfaces_propagate_the_policy() {
    let plane = surfaces(frame(false)).remove(0);
    let trimmed: Handle<dyn GeomSurface> = Handle::new(
        GeomRectangularTrimmedSurface::new(&plane, (0., 2.), (0., 3.), (true, true)).unwrap(),
    )
    .into_cast();
    let offset: Handle<dyn GeomSurface> =
        Handle::new(GeomOffsetSurface::new(&trimmed, 0.5, false).unwrap()).into_cast();
    let bytes = encode(&offset);
    assert_eq!(
        encode(&decode(bytes.clone(), Policy::PreserveStoredV1)),
        bytes
    );
}
#[test]
fn malformed_or_truncated_stored_frames_are_refused() {
    let plane = surfaces(frame(false)).remove(0);
    let bytes = encode(&plane);
    for length in 1..bytes.len() {
        assert!(
            BinSurfaceSet::read_surface_with_frame_policy(
                &mut IoStream::from_bytes(bytes[..length].to_vec()),
                Policy::PreserveStoredV1
            )
            .is_err()
        );
    }
    for value in [0., 2., f64::NAN, f64::INFINITY] {
        let mut corrupt = bytes.clone();
        corrupt[25..33].copy_from_slice(&value.to_ne_bytes());
        corrupt[33..41].copy_from_slice(&0.0_f64.to_ne_bytes());
        corrupt[41..49].copy_from_slice(&0.0_f64.to_ne_bytes());
        assert!(
            BinSurfaceSet::read_surface_with_frame_policy(
                &mut IoStream::from_bytes(corrupt),
                Policy::PreserveStoredV1
            )
            .is_err()
        );
    }
}
fn face(surface: &Handle<dyn GeomSurface>) -> Shape {
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, surface, 1e-7)
        .unwrap();
    face.into()
}
fn grouped(shape: &Shape, version: i32) -> Vec<u8> {
    written(|stream| {
        bintools::write(
            shape,
            stream,
            WriteOptions {
                version,
                ..WriteOptions::default()
            },
            &ProgressRange::default(),
        )
        .unwrap()
    })
}
#[test]
fn grouped_versions_retain_the_oblique_surface_record() {
    let surface = surfaces(frame(false)).remove(0);
    let expected = encode(&surface);
    for version in 1..=4 {
        let bytes = grouped(&face(&surface), version);
        let mut restored = Shape::default();
        bintools::read_with_surface_frame_policy(
            &mut restored,
            &mut IoStream::from_bytes(bytes),
            &ProgressRange::default(),
            Policy::PreserveStoredV1,
        )
        .unwrap();
        let again = grouped(&restored, version);
        assert!(
            again
                .windows(expected.len())
                .any(|record| record == expected)
        );
    }
}
#[test]
fn reference_reader_keeps_policy_across_cached_shapes_and_clear() {
    let surface = surfaces(frame(false)).remove(0);
    let original = face(&surface);
    let sharing_surface = face(&surface);
    let bytes = written(|stream| {
        let mut writer = BinShapeWriter::new();
        writer.write_shape(&original, stream).unwrap();
        writer.write_shape(&original, stream).unwrap();
        writer.write_shape(&sharing_surface, stream).unwrap();
    });
    let expected = encode(&surface);
    let mut reader = BinShapeReader::with_surface_frame_policy(Policy::PreserveStoredV1);
    for _ in 0..2 {
        let mut input = IoStream::from_bytes(bytes.clone());
        for _ in 0..3 {
            let mut restored = Shape::default();
            reader.read_shape(&mut input, &mut restored).unwrap();
            let mut output = IoStream::from_bytes(Vec::new());
            BinShapeWriter::new()
                .write_shape(&restored, &mut output)
                .unwrap();
            assert!(
                stream_bytes(output)
                    .windows(expected.len())
                    .any(|record| record == expected)
            );
        }
        reader.clear();
    }
}
#[test]
fn default_and_explicit_legacy_readers_retain_existing_reconstruction() {
    let original = encode(&surfaces(frame(false)).remove(0));
    let legacy = BinSurfaceSet::read_surface(&mut IoStream::from_bytes(original.clone())).unwrap();
    let explicit = decode(original.clone(), Policy::LegacyReplay);
    assert_eq!(encode(&legacy), encode(&explicit));
    assert_ne!(encode(&legacy), original);
    // Owned replay baseline from the pre-fix scalar operation sequence (#432).
    let expected = [
        0x3fd7c4dd663ebb88,
        0x3fe1d3a60caf0ca6,
        0x3fe7c4dd663ebb88,
        0x3fe9d12bb71e16a7,
        0xbfe2ddc75495fcdd,
        0x3fa3dbfa3e172520,
        0x3fdd68f73245b754,
        0x3fe2b728f1722ed8,
        0xbfe5639c81a710f7,
    ];
    let encoded = encode(&legacy);
    let actual: Vec<u64> = encoded[25..97]
        .as_chunks::<8>()
        .0
        .iter()
        .map(|bytes| f64::from_ne_bytes(*bytes).to_bits())
        .collect();
    assert_eq!(actual, expected);

    // #432's existing replay changes this particular cap-frame coefficient.
    let coefficient = |bytes: Vec<u8>| f64::from_ne_bytes(bytes[65..73].try_into().unwrap());
    assert_eq!(
        coefficient(original).to_bits(),
        0.03878767021121578_f64.to_bits()
    );
    assert_eq!(
        coefficient(encode(&legacy)).to_bits(),
        0.03878767021121576_f64.to_bits()
    );
}

#[test]
fn recognized_legacy_headers_do_not_select_frame_interpretation() {
    let surface = surfaces(frame(false)).remove(0);
    let original = face(&surface);
    let record = encode(&surface);
    for (version, header) in [
        (3, "Open CASCADE Topology V3 (c)"),
        (4, "Open CASCADE Topology V4, (c) Open Cascade"),
    ] {
        let bytes = grouped(&original, version);
        let original_header = format!("\n{}\n", bintools::THE_ASCII_VERSIONS[version as usize]);
        assert!(bytes.starts_with(original_header.as_bytes()));
        let mut legacy = format!("\n{header}\n").into_bytes();
        legacy.extend_from_slice(&bytes[original_header.len()..]);
        let mut restored = Shape::default();
        bintools::read_with_surface_frame_policy(
            &mut restored,
            &mut IoStream::from_bytes(legacy.clone()),
            &ProgressRange::default(),
            Policy::PreserveStoredV1,
        )
        .unwrap();
        assert!(
            grouped(&restored, version)
                .windows(record.len())
                .any(|window| window == record)
        );
        let mut default_read = Shape::default();
        bintools::read(
            &mut default_read,
            &mut IoStream::from_bytes(legacy),
            &ProgressRange::default(),
        )
        .unwrap();
        assert!(
            !grouped(&default_read, version)
                .windows(record.len())
                .any(|window| window == record)
        );
    }
}
#[test]
fn signed_zero_surface_coefficients_survive_table_clear() {
    let axis = Axis3::from_stored_coefficients(
        Point3::new(-0., 0., -0.),
        [-0., 0., 1.],
        [1., -0., 0.],
        [0., 1., -0.],
    )
    .unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(axis)).into_cast();
    let mut table = BinSurfaceSet::new();
    table.add(&surface);
    let bytes = written(|stream| table.write(stream, &ProgressRange::default()).unwrap());
    let mut reader = BinSurfaceSet::with_surface_frame_policy(Policy::PreserveStoredV1);
    for _ in 0..2 {
        reader
            .read(
                &mut IoStream::from_bytes(bytes.clone()),
                &ProgressRange::default(),
            )
            .unwrap();
        assert_eq!(
            written(|stream| reader.write(stream, &ProgressRange::default()).unwrap()),
            bytes
        );
        reader.clear();
    }
}
