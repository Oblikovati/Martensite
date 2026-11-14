//! Both binary shape formats propagate the independent location policy (#451).
use super::fakes::written;
use super::stored_location_transforms::{bits, oblique};
use martensite_tkbrep::bintools::{
    self, BinShapeReader, BinShapeSet, BinShapeSetBase, BinShapeWriter,
    LocationTransformReadPolicy as Policy, SurfaceFrameReadPolicy as SurfacePolicy, WriteOptions,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Shape, Vertex};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkmath::{
    gp::{Point3, Transform3},
    toploc::Location,
};
fn placed(transform: Transform3) -> Shape {
    let mut vertex = Vertex::default();
    BRepBuilder::default()
        .make_vertex_at(&mut vertex, Point3::new(2., 3., 4.), 1e-7)
        .unwrap();
    vertex.located(&Location::new(transform), false).unwrap()
}
#[test]
fn reference_shapes_retain_location_policy_across_clear_and_cached_references() {
    for indirect in [false, true] {
        let original = oblique(indirect);
        let bytes = reference_bytes(&placed(original));
        let mut reader = BinShapeReader::with_read_policies(
            SurfacePolicy::LegacyReplay,
            Policy::PreserveStoredRigidV1,
        );
        for _ in 0..2 {
            check_references(&mut reader, bytes.clone(), original);
            reader.clear();
        }
    }
}
fn reference_bytes(shape: &Shape) -> Vec<u8> {
    written(|stream| {
        let mut writer = BinShapeWriter::new();
        writer.write_shape(shape, stream).unwrap();
        writer.write_shape(shape, stream).unwrap();
    })
}
fn check_references(reader: &mut BinShapeReader, bytes: Vec<u8>, original: Transform3) {
    let mut input = IoStream::from_bytes(bytes);
    for _ in 0..2 {
        let mut restored = Shape::default();
        reader.read_shape(&mut input, &mut restored).unwrap();
        assert_eq!(bits(&restored.location().transformation()), bits(&original));
    }
}
fn grouped_decode(bytes: Vec<u8>, policy: Policy) -> Shape {
    let mut reader = BinShapeSet::with_read_policies(SurfacePolicy::LegacyReplay, policy);
    let mut input = IoStream::from_bytes(bytes);
    let progress = ProgressRange::default();
    reader.read(&mut input, &progress).unwrap();
    let count = reader.nb_shapes();
    let mut shape = Shape::default();
    reader.read_subs(&mut shape, &mut input, count).unwrap();
    shape
}
#[test]
fn grouped_shape_reader_preserves_checked_oblique_location_coefficients() {
    for indirect in [false, true] {
        let original = oblique(indirect);
        let bytes = grouped_bytes(&placed(original));
        let restored = grouped_decode(bytes.clone(), Policy::PreserveStoredRigidV1);
        assert_eq!(bits(&restored.location().transformation()), bits(&original));
        check_legacy_grouped(bytes);
    }
}
fn grouped_bytes(shape: &Shape) -> Vec<u8> {
    written(|stream| {
        bintools::write(
            shape,
            stream,
            WriteOptions::default(),
            &ProgressRange::default(),
        )
        .unwrap()
    })
}
fn check_legacy_grouped(bytes: Vec<u8>) {
    let legacy = grouped_decode(bytes.clone(), Policy::LegacyReplay);
    let mut default = Shape::default();
    bintools::read(
        &mut default,
        &mut IoStream::from_bytes(bytes),
        &ProgressRange::default(),
    )
    .unwrap();
    assert_eq!(
        bits(&legacy.location().transformation()),
        bits(&default.location().transformation())
    );
}
