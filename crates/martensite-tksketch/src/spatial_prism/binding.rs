// SPDX-License-Identifier: GPL-2.0-only
//! Bind certified outer-loop identity and each actual affine region edge to authored XYZ.
use super::{
    SpatialPrismFailure as Failure, SpatialPrismStage as Stage, failure::native, plane::vertex_id,
    poll,
};
use martensite_tkbo::profile_extrusion::{
    LoopUse, PlanarRegion, PlanarRegionSet, ProfileCurveId, ProfileVertexId,
};
use martensite_tkbrep::{
    brep::{BRepTool, access::vertex_tshape},
    topexp::{self, Explorer},
    topods::{Edge, Vertex},
};
use martensite_tkg3d::{
    geom::{GeomLine, GeomPlane},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    gp::{Direction3, Point3},
    predicates::{
        compare_scalar_triple_product_3d, placed_affine_line_trace_bound,
        placed_point_distance_upper_bound,
    },
};
use martensite_tksketch3d::{
    LineId,
    wire::{LineDirection, WireRecipe, WireSegment},
};
use std::collections::BTreeMap;

pub(super) fn bind<'a>(
    wire: &WireRecipe,
    regions: &'a PlanarRegionSet,
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(&'a PlanarRegion, f64), Failure> {
    poll(cancelled, Stage::Binding)?;
    let [region] = regions.regions() else {
        return Err(mismatch(None, "exactly one selected region"));
    };
    let segments = wire.segments();
    if !region.holes().is_empty() || !equivalent_loop(&region.outer().uses, segments, cancelled)? {
        return Err(mismatch(
            None,
            "the complete directed closed loop without holes",
        ));
    }
    let mut bound = 0.0_f64;
    let edges: BTreeMap<_, _> = region
        .edges()
        .iter()
        .map(|(id, edge)| (*id, edge))
        .collect();
    let vertices: BTreeMap<_, _> = region
        .vertices()
        .iter()
        .map(|(id, vertex)| (*id, vertex))
        .collect();
    if edges.len() != segments.len() || vertices.len() != segments.len() {
        return Err(mismatch(
            None,
            "one owned native edge and vertex per authored member",
        ));
    }
    bind_native_wire(region, &edges, cancelled)?;
    for segment in segments {
        poll(cancelled, Stage::Binding)?;
        bound = bound.max(bind_line(segment, &edges, &vertices, limit)?);
    }
    Ok((region, bound))
}

pub(super) fn bind_native_wire(
    region: &PlanarRegion,
    edges: &BTreeMap<ProfileCurveId, &Edge>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Failure> {
    let mut wires = native(
        Explorer::new(region.face(), ShapeEnum::Wire, ShapeEnum::Shape),
        Stage::Binding,
    )?;
    if !wires.more() {
        return Err(mismatch(None, "the actual outer wire"));
    }
    let outer = wires.current().clone();
    native(Explorer::next(&mut wires), Stage::Binding)?;
    if wires.more() || !native(BRepTool::is_closed(&outer), Stage::Binding)? {
        return Err(mismatch(None, "a single closed native wire"));
    }
    let mut actual = native(
        Explorer::new(&outer, ShapeEnum::Edge, ShapeEnum::Shape),
        Stage::Binding,
    )?;
    for used in &region.outer().uses {
        poll(cancelled, Stage::Binding)?;
        let orientation = [Orientation::Forward, Orientation::Reversed][usize::from(used.reversed)];
        if !actual.more()
            || actual.current().orientation() != orientation
            || !edges
                .get(&used.curve)
                .is_some_and(|edge| edge.is_same(actual.current()))
        {
            return Err(mismatch(
                None,
                "the ordered native boundary edge identities",
            ));
        }
        native(Explorer::next(&mut actual), Stage::Binding)?;
    }
    if actual.more() {
        return Err(mismatch(None, "the complete native edge count"));
    }
    Ok(())
}

fn equivalent_loop(
    actual: &[LoopUse],
    source: &[WireSegment],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<bool, Failure> {
    if actual.len() != source.len() || actual.is_empty() {
        return Ok(false);
    }
    let Some(start) = actual
        .iter()
        .position(|item| item.curve == curve_id(&source[0]))
    else {
        return Ok(false);
    };
    let reversed = actual[start].reversed ^ is_reversed(&source[0]);
    for (offset, expected) in source.iter().enumerate() {
        poll(cancelled, Stage::Binding)?;
        let index = (if reversed {
            start + actual.len() - offset
        } else {
            start + offset
        }) % actual.len();
        if actual[index].curve != curve_id(expected)
            || (actual[index].reversed ^ reversed) != is_reversed(expected)
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn bind_line(
    segment: &WireSegment,
    edges: &BTreeMap<ProfileCurveId, &Edge>,
    vertices: &BTreeMap<ProfileVertexId, &Vertex>,
    limit: f64,
) -> Result<f64, Failure> {
    let source = Some(segment.source.line);
    let edge = edges
        .get(&curve_id(segment))
        .ok_or_else(|| mismatch(source, "its authored edge binding"))?;
    let point_bound = vertex_trace(segment, edge, vertices)?;
    let curve_bound = affine_trace(segment, edge)?;
    let bound = point_bound.max(curve_bound);
    if !bound.is_finite() || bound > limit {
        return Err(Failure::SourceTrace {
            line: segment.source.line,
            bound,
            limit,
        });
    }
    Ok(bound)
}

fn vertex_trace(
    segment: &WireSegment,
    edge: &Edge,
    vertices: &BTreeMap<ProfileVertexId, &Vertex>,
) -> Result<f64, Failure> {
    let (first, last) = topexp::vertices(edge, false);
    let ids = segment.authored.line.endpoints.map(vertex_id);
    let mut bound = 0.0_f64;
    for ((id, actual), xyz) in ids
        .into_iter()
        .zip([first, last])
        .zip(segment.authored.points)
    {
        if !vertices
            .get(&id)
            .is_some_and(|expected| expected.is_same(&actual))
        {
            return Err(mismatch(
                Some(segment.source.line),
                "the ordered authored vertex identities",
            ));
        }
        let point = native(
            vertex_tshape(&actual, "spatial prism profile vertex"),
            Stage::Binding,
        )?
        .pnt();
        bound = bound.max(placed_point_distance_upper_bound(
            point,
            actual.location().transformation(),
            xyz,
        ));
    }
    Ok(bound)
}

fn affine_trace(segment: &WireSegment, edge: &Edge) -> Result<f64, Failure> {
    let line = segment.source.line;
    let stored = native(BRepTool::curve_with_location(edge), Stage::Binding)?;
    let Some(carrier) = stored.curve.downcast_ref::<GeomLine>() else {
        return Err(mismatch(Some(line), "an analytic native line"));
    };
    if stored.first >= stored.last {
        return Err(mismatch(Some(line), "an increasing native line trim"));
    }
    Ok(placed_affine_line_trace_bound(
        carrier.lin(),
        [stored.first, stored.last],
        stored.location.transformation(),
        segment.authored.points,
    ))
}

pub(super) fn require_sweep_sense(
    region: &PlanarRegion,
    direction: [f64; 3],
    expected: std::cmp::Ordering,
) -> Result<(), Failure> {
    let (surface, location) = native(
        BRepTool::surface_with_location(region.face()),
        Stage::Binding,
    )?;
    let Some(plane) = surface.downcast_ref::<GeomPlane>() else {
        return Err(mismatch(None, "a native planar carrier"));
    };
    let frame = plane.pln().position();
    if frame.location() != Point3::default()
        || frame.x_direction().coordinates() != Direction3::X.coordinates()
        || frame.y_direction().coordinates() != Direction3::Y.coordinates()
    {
        return Err(mismatch(None, "a placed canonical XY carrier"));
    }
    let transform = location.transformation().vectorial_part();
    let x = *native(transform.column(1), Stage::Binding)?.coordinates();
    let y = *native(transform.column(2), Stage::Binding)?.coordinates();
    let sign =
        compare_scalar_triple_product_3d(x, y, direction).map_err(|source| Failure::Predicate {
            point: None,
            source,
        })?;
    if sign != expected {
        return Err(mismatch(None, "the exact authored sweep direction sense"));
    }
    Ok(())
}

fn curve_id(segment: &WireSegment) -> ProfileCurveId {
    ProfileCurveId(segment.source.line.get().get())
}

fn is_reversed(segment: &WireSegment) -> bool {
    segment.source.direction == LineDirection::Reversed
}

pub(super) fn mismatch(line: Option<LineId>, expected: &'static str) -> Failure {
    Failure::RegionBinding { line, expected }
}
