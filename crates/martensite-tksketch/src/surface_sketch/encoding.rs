//! One immutable native shape table, followed by explicit semantic shape references.
use super::{
    failure::*,
    history::{Bindings, HistoryKey},
};
use martensite_tkbrep::{
    bintools::{BinShapeSet, BinShapeSetBase},
    topods::Shape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};

pub(super) fn encode(solid: &Shape, bindings: &Bindings) -> Checked<Vec<u8>> {
    let mut set = shape_table(solid, bindings)?;
    let mut stream = IoStream::from_bytes(Vec::new());
    set.write(&mut stream, &ProgressRange::default())
        .map_err(|error| native("encode table", error))?;
    set.write_shape(solid, &mut stream)
        .map_err(|error| native("encode root reference", error))?;
    for (_, shape) in bindings {
        set.write_shape(shape, &mut stream)
            .map_err(|error| native("encode semantic reference", error))?;
    }
    stream_bytes(&mut stream)
}
fn stream_bytes(stream: &mut IoStream) -> Checked<Vec<u8>> {
    let size = usize::try_from(stream.tellp()).map_err(|_| {
        incomplete(
            "encoded stream length",
            "negative or unrepresentable length",
        )
    })?;
    let mut bytes = vec![0; size];
    stream.seekg(0).read(&mut bytes);
    if stream.fail() {
        return Err(incomplete(
            "encoded stream",
            "stream failed while capturing bytes",
        ));
    }
    Ok(bytes)
}
pub(super) fn decode(bytes: &[u8], keys: &[HistoryKey]) -> Checked<(Shape, Bindings)> {
    let mut stream = IoStream::from_bytes(bytes.to_vec());
    let mut set = BinShapeSet::new();
    set.read(&mut stream, &ProgressRange::default())
        .map_err(|error| native("decode table", error))?;
    let solid = reference(&set, &mut stream)?;
    let bindings = keys
        .iter()
        .map(|key| Ok((*key, reference(&set, &mut stream)?)))
        .collect::<Checked<_>>()?;
    if stream.fail() {
        return Err(incomplete(
            "decoded stream",
            "incomplete root or semantic references",
        ));
    }
    Ok((solid, bindings))
}
fn reference(set: &BinShapeSet, stream: &mut IoStream) -> Checked<Shape> {
    let mut shape = Shape::default();
    set.read_subs(&mut shape, stream, set.nb_shapes())
        .map_err(|error| native("decode semantic reference", error))?;
    if shape.is_null() {
        return Err(incomplete("decoded semantic reference", "null shape"));
    }
    Ok(shape)
}

fn shape_table(solid: &Shape, bindings: &Bindings) -> Checked<BinShapeSet> {
    let mut set = BinShapeSet::new();
    set.set_with_triangles(false);
    set.set_with_normals(false);
    set.add(solid)
        .map_err(|error| native("encode solid", error))?;
    for (_, shape) in bindings {
        set.add(shape)
            .map_err(|error| native("encode binding", error))?;
    }
    Ok(set)
}
