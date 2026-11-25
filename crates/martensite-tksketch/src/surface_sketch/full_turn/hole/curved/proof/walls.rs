// SPDX-License-Identifier: GPL-2.0-only
//! Transfer full ruled-wall certificates by exact surface and trim coefficient ownership.
use super::*;
use martensite_tkbrep::bintools::{BinCurve2dSet, BinCurveSet, BinOStream, BinSurfaceSet};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::{standard::Failure, storage::IoStream};
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomBezierSurface};

pub(super) fn verify(
    bindings: &binding::Bindings,
    tool: &DetachedWrappedBodies,
    sources: &[binding::FittedCut; 2],
) -> Checked<()> {
    for (key, source) in tool
        .entities
        .iter()
        .filter(|(key, _)| key.role == HistoryRole::SideFace)
    {
        let actual = binding::resolve(bindings, PeriodicImageKey::WrappedRetained(*key))?;
        wall(bindings, tool, sources, source, actual)?;
    }
    Ok(())
}

fn wall(
    bindings: &binding::Bindings,
    tool: &DetachedWrappedBodies,
    sources: &[binding::FittedCut; 2],
    source: &topods::Shape,
    actual: &topods::Shape,
) -> Checked<()> {
    let before = topods::face(source).map_err(|e| native("fitted ruled wall", e))?;
    let after = topods::face(actual).map_err(|e| native("decoded ruled wall", e))?;
    one_wire(&after)?;
    if before.orientation() == after.orientation() || surface(&before)? != surface(&after)? {
        return Err(incomplete(
            "reversed cut wall with its original certified surface",
            format!("{:?} to {:?}", before.orientation(), after.orientation()),
        ));
    }
    let actual_edges = coedges(&after)?;
    let mut expected = 0;
    for edge in coedges(&before)? {
        let key = source_edge(tool, &edge)?;
        let images = edge_images(key, sources);
        for image in &images {
            let [result] = occurrences(&actual_edges, binding::resolve(bindings, *image)?)?;
            verify_edge(&before, &edge, &after, &result)?;
        }
        expected += images.len();
    }
    if expected != actual_edges.len() {
        return Err(incomplete(
            "complete curved wall boundary",
            format!("{} vs {expected}", actual_edges.len()),
        ));
    }
    Ok(())
}

fn source_edge(tool: &DetachedWrappedBodies, edge: &topods::Edge) -> Checked<WrappedHistoryKey> {
    let mut exact = tool
        .entities
        .iter()
        .filter(|(_, source)| source.is_same(edge))
        .map(|(key, _)| *key);
    let key = exact
        .next()
        .ok_or_else(|| incomplete("original certified wall edge", "missing source"))?;
    if exact.next().is_some() {
        return Err(incomplete(
            "unique certified wall edge source",
            format!("{key:?}"),
        ));
    }
    Ok(key)
}

fn edge_images(key: WrappedHistoryKey, sources: &[binding::FittedCut; 2]) -> Vec<PeriodicImageKey> {
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        if let Some(source) = sources.iter().find(|source| source.edge(depth) == key) {
            return source.pieces(depth).to_vec();
        }
    }
    vec![PeriodicImageKey::WrappedRetained(key)]
}

fn verify_edge(
    source: &topods::Face,
    edge: &topods::Edge,
    actual: &topods::Face,
    result: &topods::Edge,
) -> Checked<()> {
    let (before, range) = geometry(edge)?;
    let (after, span) = geometry(result)?;
    if before != after || span[0] < range[0] || span[1] > range[1] || span[1] <= span[0] {
        return Err(incomplete(
            "same certified wall-edge geometry with restricted range",
            format!("{range:?} -> {span:?}"),
        ));
    }
    let old_trim = trim(source, edge, range)?;
    let new_trim = trim(actual, result, span)?;
    if old_trim != new_trim {
        return Err(incomplete(
            "unchanged ruled-wall trim polynomial",
            format!("{range:?} -> {span:?}"),
        ));
    }
    Ok(())
}

fn trim(face: &topods::Face, edge: &topods::Edge, range: [f64; 2]) -> Checked<Vec<u8>> {
    let stored =
        BRepTool::curve_on_face(edge, face).map_err(|e| native("curved wall pcurve", e))?;
    if !stored.is_stored
        || [stored.first, stored.last] != range
        || stored
            .pcurve
            .downcast_ref::<Geom2dBezierCurve>()
            .is_none_or(|curve| curve.has_eval_representation())
    {
        return Err(incomplete(
            "literal ruled-wall same-parameter trim",
            format!("{range:?}"),
        ));
    }
    bits(|out| BinCurve2dSet::write_curve2d(&stored.pcurve, out))
}

fn surface(face: &topods::Face) -> Checked<Vec<u8>> {
    let (surface, location) =
        BRepTool::surface_with_location(face).map_err(|e| native("curved wall surface", e))?;
    if !location.is_identity()
        || surface
            .downcast_ref::<GeomBezierSurface>()
            .is_none_or(|value| value.has_eval_representation())
    {
        return Err(incomplete(
            "canonical literal ruled Bezier surface",
            "different support representation",
        ));
    }
    bits(|out| BinSurfaceSet::write_surface(&surface, out))
}

fn geometry(edge: &topods::Edge) -> Checked<(Vec<u8>, [f64; 2])> {
    let (curve, first, last) =
        BRepTool::curve(edge).map_err(|e| native("curved wall edge geometry", e))?;
    if curve
        .downcast_ref::<GeomBezierCurve>()
        .is_none_or(|value| value.has_eval_representation())
    {
        return Err(incomplete(
            "literal rational wall-edge curve",
            format!("{first}..{last}"),
        ));
    }
    let data = bits(|out| BinCurveSet::write_curve(&curve, out))?;
    Ok((data, [first, last]))
}

fn bits(writer: impl FnOnce(&mut BinOStream<'_>) -> Result<(), Failure>) -> Checked<Vec<u8>> {
    let mut stream = IoStream::from_bytes(Vec::new());
    writer(&mut BinOStream::new(&mut stream))
        .map_err(|e| native("capture native trim coefficients", e))?;
    let size = usize::try_from(stream.tellp())
        .map_err(|e| incomplete("binary geometry length", e.to_string()))?;
    let mut bytes = vec![0; size];
    stream.seekg(0).read(&mut bytes);
    if stream.fail() {
        return Err(incomplete(
            "captured native trim coefficient bytes",
            format!("{size} bytes"),
        ));
    }
    Ok(bytes)
}

pub(super) fn one_wire(shape: &topods::Shape) -> Checked<()> {
    let mut wires = IndexedMapOfShape::default();
    martensite_tkbrep::topexp::map_shapes_of_type(shape, ShapeEnum::Wire, &mut wires)
        .map_err(|e| native("curved result wire membership", e))?;
    if wires.extent() != 1 {
        return Err(incomplete(
            "one complete curved result wire",
            format!("{}", wires.extent()),
        ));
    }
    Ok(())
}
