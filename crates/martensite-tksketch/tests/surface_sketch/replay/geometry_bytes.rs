//! A test-local native encoding binds the exported geometry to each semantic key.
use martensite_tkbrep::bintools::{BinShapeSet, BinShapeSetBase};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tksketch::surface_sketch::DetachedNormalBody;
use std::fmt::Write;

pub(super) fn signature(body: &DetachedNormalBody) -> String {
    let mut set = shape_table(body);
    let mut stream = IoStream::from_bytes(Vec::new());
    set.write(&mut stream, &ProgressRange::default()).unwrap();
    set.write_shape(&body.solid, &mut stream).unwrap();
    let mut keys = String::new();
    for (key, shape) in &body.entities {
        write!(&mut keys, "{key:?}|").unwrap();
        set.write_shape(shape, &mut stream).unwrap();
    }
    format!("{keys}{}", encoded_hex(&mut stream))
}
fn shape_table(body: &DetachedNormalBody) -> BinShapeSet {
    let mut set = BinShapeSet::new();
    set.set_with_triangles(false);
    set.set_with_normals(false);
    set.add(&body.solid).unwrap();
    for (_, shape) in &body.entities {
        set.add(shape).unwrap();
    }
    set
}
fn encoded_hex(stream: &mut IoStream) -> String {
    let length = usize::try_from(stream.tellp()).unwrap();
    let mut bytes = vec![0; length];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    let mut encoded = String::with_capacity(2 * length);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").unwrap();
    }
    encoded
}
