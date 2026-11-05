//! BRepFillOffsetWire probe operations, the Rust twin of
//! the recorded capture (the records are documented there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::tp_shapes;
use martensite_tkbool::brep_fill::{BRepFillOffsetAncestors, BRepFillOffsetWire};
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::JoinType;

/// The join of its enumeration value (0 arc, 1 tangent, 2 intersection).
fn join_of(value: i32) -> JoinType {
    match value {
        1 => JoinType::Tangent,
        2 => JoinType::Intersection,
        _ => JoinType::Arc,
    }
}

/// `bl_sorted`: the sorted indices and orientations of a list.
fn sorted(output: &BufferedTokens, map: &IndexedMapOfShape, l: &List<Shape>) {
    let mut ids: Vec<(i32, i32)> = l
        .iter()
        .map(|s| (map.find_index(s) as i32, s.orientation() as i32))
        .collect();
    ids.sort();
    output.int(ids.len() as i32);
    for (index, orientation) in ids {
        output.int(index);
        output.int(orientation);
    }
}

/// The edges of a shape in explorer order.
fn edges_of(s: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut out = Vec::new();
    let mut ex = Explorer::new(s, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while ex.more() {
        out.push(ex.current().clone());
        ex.next()?;
    }
    Ok(out)
}

/// `bl_ancestors`: the ancestor index in the spine map of each edge of the result.
fn ancestors(output: &BufferedTokens, ow: &mut BRepFillOffsetWire) -> Result<(), Failure> {
    let anc = BRepFillOffsetAncestors::with_offset(ow)?;
    output.flag(anc.is_done());
    let mut sm = IndexedMapOfShape::default();
    topexp::map_shapes(ow.spine(), &mut sm, true, true);
    for e in edges_of(ow.shape())? {
        let e = topods::edge(&e)?;
        let index = if anc.has_ancestor(&e) {
            sm.find_index(anc.ancestor(&e)?) as i32
        } else {
            -1
        };
        output.int(index);
    }
    Ok(())
}

/// The generated shapes and the ancestors of a done offset.
fn print_done(output: &BufferedTokens, program_shapes: &[Shape], ow: &mut BRepFillOffsetWire) {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes(ow.shape(), &mut map, true, true);
    for g in program_shapes {
        output.attempt(|| ow.generated_shapes(g).map(|l| sorted(output, &map, l)));
    }
    let spine_edges = edges_of(ow.spine()).unwrap_or_default();
    for e in &spine_edges {
        output.attempt(|| ow.generated_shapes(e).map(|l| sorted(output, &map, l)));
    }
    output.attempt(|| ancestors(output, ow));
}

/// `bf_offset_wire program face join open offset alt`.
pub fn offset_wire_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let face = at(&t, r.integer()).and_then(|f| topods::face(&f));
    let (join, open) = (join_of(r.integer()), r.flag());
    let (offset, alt) = (r.real(), r.real());
    let mut ow = BRepFillOffsetWire::default();
    let performed = face
        .and_then(|face| ow.init(&face, join, open))
        .and_then(|()| ow.perform(offset, alt));
    if let Err(error) = performed {
        return failed(output, error);
    }
    output.flag(ow.is_done());
    output.int(ow.join_type() as i32);
    output.attempt(|| shape(output, ow.shape()));
    if ow.is_done() {
        print_done(output, &t.shapes, &mut ow);
    }
    output.end();
}
