//! Semantic identities are bound through native sweep images before any serialization.
use super::{
    construction::Section,
    failure::*,
    request::*,
    sweep::{Sweep, SweepNames},
};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeRevol;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Authored parent of a result entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HistorySource {
    /// Whole developed region.
    Region(SurfaceRegionId),
    /// One authored boundary curve.
    Curve(SurfaceCurveId),
    /// One authored corner.
    Vertex(SurfaceVertexId),
}
/// Geometric role relative to the requested, ordered depth endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HistoryRole {
    /// Complete region image.
    Solid,
    /// Region at the first depth.
    FirstDepthFace,
    /// Region at the last depth.
    LastDepthFace,
    /// Boundary swept through the depth interval.
    SideFace,
    /// Boundary at first depth.
    FirstDepthEdge,
    /// Boundary at last depth.
    LastDepthEdge,
    /// Corner swept along its support normal.
    NormalEdge,
    /// Corner at first depth.
    FirstDepthVertex,
    /// Corner at last depth.
    LastDepthVertex,
}
/// Persistent semantic identity; no traversal rank or coordinate ordering participates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HistoryKey {
    /// Caller-authored parent.
    pub source: HistorySource,
    /// Result role.
    pub role: HistoryRole,
}
pub(super) type Bindings = Vec<(HistoryKey, Shape)>;
pub(super) struct Collector<'a> {
    pub maker: &'a mut BRepPrimAPIMakeRevol,
    pub images: Bindings,
}
impl Collector<'_> {
    pub(super) fn generated(
        &mut self,
        source: HistorySource,
        role: HistoryRole,
        shape: &Shape,
    ) -> Checked<()> {
        let images = self
            .maker
            .generated(shape)
            .map_err(|error| native("generated image", error))?;
        let images: Vec<_> = images.iter().cloned().collect();
        let [one] = images.as_slice() else {
            return Err(incomplete(
                "one generated image",
                format!("{source:?}/{role:?}: {} images", images.len()),
            ));
        };
        self.images.push((HistoryKey { source, role }, one.clone()));
        Ok(())
    }
    fn end(
        &mut self,
        source: HistorySource,
        role: HistoryRole,
        shape: &Shape,
        last: bool,
    ) -> Checked<()> {
        let image = if last {
            self.maker.last_shape_of(shape)
        } else {
            self.maker.first_shape_of(shape)
        }
        .map_err(|error| native("angular endpoint image", error))?;
        self.images.push((HistoryKey { source, role }, image));
        Ok(())
    }
}

fn collect_images(
    admitted: &Sweep,
    section: &Section,
    maker: &mut BRepPrimAPIMakeRevol,
    solid: &Shape,
) -> Checked<Bindings> {
    let mut collector = Collector {
        maker,
        images: Vec::new(),
    };
    let key = HistoryKey {
        source: HistorySource::Region(admitted.region),
        role: HistoryRole::Solid,
    };
    collector.images.push((key, solid.clone()));
    match admitted.names {
        SweepNames::Sector {
            corners,
            boundaries,
        } => {
            region_faces(&mut collector, admitted.region, boundaries, section)?;
            curve_images(&mut collector, boundaries, section)?;
            corner_images(&mut collector, corners, section)?;
        }
        SweepNames::FullTurn { rims } => {
            super::full_turn::collect(&mut collector, admitted.region, rims, section, solid)?;
        }
    }
    Ok(collector.images)
}
pub(super) fn face_role(depth: usize) -> HistoryRole {
    [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace][depth]
}
pub(super) fn edge_role(depth: usize) -> HistoryRole {
    [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge][depth]
}
pub(super) fn vertex_role(depth: usize) -> HistoryRole {
    [HistoryRole::FirstDepthVertex, HistoryRole::LastDepthVertex][depth]
}
fn region_faces(
    collector: &mut Collector<'_>,
    region: SurfaceRegionId,
    boundaries: [SurfaceCurveId; 4],
    section: &Section,
) -> Checked<()> {
    depth_faces(collector, region, section)?;
    for (curve, edge) in [(0, 0), (2, 2)] {
        let source = HistorySource::Curve(boundaries[curve]);
        collector.generated(source, HistoryRole::SideFace, &section.edges[edge])?;
    }
    for (curve, last) in [(3, false), (1, true)] {
        let source = HistorySource::Curve(boundaries[curve]);
        collector.end(source, HistoryRole::SideFace, &section.face, last)?;
    }
    Ok(())
}
fn curve_images(
    collector: &mut Collector<'_>,
    boundaries: [SurfaceCurveId; 4],
    section: &Section,
) -> Checked<()> {
    arc_images(collector, [boundaries[0], boundaries[2]], section)?;
    for (curve, last) in [(3, false), (1, true)] {
        let source = HistorySource::Curve(boundaries[curve]);
        let inner = edge_role(section.first_depth);
        let outer = edge_role(1 - section.first_depth);
        collector.end(source, inner, &section.edges[3], last)?;
        collector.end(source, outer, &section.edges[1], last)?;
    }
    Ok(())
}
fn corner_images(
    collector: &mut Collector<'_>,
    corners: [SurfaceVertexId; 4],
    section: &Section,
) -> Checked<()> {
    for (corner, id) in corners.into_iter().enumerate() {
        let source = HistorySource::Vertex(id);
        corner_ends(collector, source, section, corner)?;
    }
    Ok(())
}
pub(super) fn role_kind(role: HistoryRole) -> ShapeEnum {
    match role {
        HistoryRole::Solid => ShapeEnum::Solid,
        HistoryRole::FirstDepthFace | HistoryRole::LastDepthFace | HistoryRole::SideFace => {
            ShapeEnum::Face
        }
        HistoryRole::FirstDepthEdge | HistoryRole::LastDepthEdge | HistoryRole::NormalEdge => {
            ShapeEnum::Edge
        }
        HistoryRole::FirstDepthVertex | HistoryRole::LastDepthVertex => ShapeEnum::Vertex,
    }
}
pub(super) fn validate_membership(solid: &Shape, bindings: &Bindings) -> Checked<()> {
    for (kind, count) in [
        (ShapeEnum::Solid, 1),
        (ShapeEnum::Face, 6),
        (ShapeEnum::Edge, 12),
        (ShapeEnum::Vertex, 8),
    ] {
        validate_kind(solid, bindings, kind, Some(count))?;
    }
    Ok(())
}

pub(super) fn validate_sweep_membership(
    sweep: &Sweep,
    solid: &Shape,
    bindings: &Bindings,
) -> Checked<()> {
    if matches!(sweep.names, SweepNames::Sector { .. }) {
        return validate_membership(solid, bindings);
    }
    for (kind, count) in [
        (ShapeEnum::Solid, Some(1)),
        (ShapeEnum::Face, Some(4)),
        (ShapeEnum::Edge, None),
        (ShapeEnum::Vertex, Some(4)),
    ] {
        validate_kind(solid, bindings, kind, count)?;
    }
    Ok(())
}

// Native sweep images can have their source orientation rather than their solid use.
fn root_face_orientations(solid: &Shape, bindings: &mut Bindings) -> Checked<()> {
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(solid, ShapeEnum::Face, &mut faces)
        .map_err(|error| native("root face occurrences", error))?;
    for (key, shape) in bindings {
        if role_kind(key.role) != ShapeEnum::Face {
            continue;
        }
        let index = faces.find_index(shape);
        *shape = faces
            .find_key(index)
            .map_err(|error| native("root face image", error))?
            .clone();
    }
    Ok(())
}

pub(super) fn collect(
    admitted: &Sweep,
    section: &Section,
    maker: &mut BRepPrimAPIMakeRevol,
    solid: &Shape,
) -> Checked<Bindings> {
    let mut images = collect_images(admitted, section, maker, solid)?;
    images.sort_by_key(|(key, _)| *key);
    root_face_orientations(solid, &mut images)?;
    validate_sweep_membership(admitted, solid, &images)?;
    Ok(images)
}
pub(super) fn depth_faces(
    collector: &mut Collector<'_>,
    region: SurfaceRegionId,
    section: &Section,
) -> Checked<()> {
    let source = HistorySource::Region(region);
    collector.generated(source, face_role(section.first_depth), &section.edges[3])?;
    collector.generated(
        source,
        face_role(1 - section.first_depth),
        &section.edges[1],
    )
}

pub(super) fn arc_images(
    collector: &mut Collector<'_>,
    rims: [SurfaceCurveId; 2],
    section: &Section,
) -> Checked<()> {
    for vertex in 0..4 {
        let rim = rims[vertex / 2];
        let depth = if vertex == 0 || vertex == 3 {
            section.first_depth
        } else {
            1 - section.first_depth
        };
        let source = HistorySource::Curve(rim);
        collector.generated(source, edge_role(depth), &section.vertices[vertex])?;
    }
    Ok(())
}
fn corner_ends(
    collector: &mut Collector<'_>,
    source: HistorySource,
    section: &Section,
    corner: usize,
) -> Checked<()> {
    let last = corner == 1 || corner == 2;
    let (edge, vertices) = if corner < 2 { (0, [0, 1]) } else { (2, [3, 2]) };
    collector.end(source, HistoryRole::NormalEdge, &section.edges[edge], last)?;
    let inner = vertex_role(section.first_depth);
    let outer = vertex_role(1 - section.first_depth);
    collector.end(source, inner, &section.vertices[vertices[0]], last)?;
    collector.end(source, outer, &section.vertices[vertices[1]], last)
}

fn validate_kind(
    solid: &Shape,
    bindings: &Bindings,
    kind: ShapeEnum,
    count: Option<usize>,
) -> Checked<()> {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(solid, kind, &mut map)
        .map_err(|error| native("history membership", error))?;
    let selected: Vec<_> = bindings
        .iter()
        .filter(|(key, _)| role_kind(key.role) == kind)
        .collect();
    let count = count.unwrap_or(selected.len());
    if selected.len() != count || map.extent() != count {
        return Err(incomplete(
            "complete history census",
            format!(
                "{kind:?}: {} bindings, {} entities, expected {count}",
                selected.len(),
                map.extent()
            ),
        ));
    }
    unique_membership(&map, selected)
}
fn unique_membership(map: &IndexedMapOfShape, selected: Vec<&(HistoryKey, Shape)>) -> Checked<()> {
    let mut seen = std::collections::BTreeSet::new();
    for (key, shape) in selected {
        let index = map.find_index(shape);
        if index == 0 || !seen.insert(index) {
            return Err(incomplete(
                "unique history membership",
                format!("{key:?} maps to missing/repeated entity {index}"),
            ));
        }
    }
    Ok(())
}
