// SPDX-License-Identifier: GPL-2.0-only
//! Name closed rims from maker lineage and seams from their owning faces.
use crate::surface_sketch::{construction::Section, failure::*, history::*, request::*};
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface, topexp, topods, toptools::IndexedMapOfShape,
};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};

pub(in crate::surface_sketch) fn collect(
    collector: &mut Collector<'_>,
    region: SurfaceRegionId,
    rims: [SurfaceCurveId; 2],
    section: &Section,
    solid: &topods::Shape,
) -> Checked<()> {
    depth_faces(collector, region, section)?;
    arc_images(collector, rims, section)?;
    // TKPrim's closed revolution does not always report planar annuli through
    // Generated. Their authored image is unique by both incident rim images.
    faces_at_rims(collector, rims, solid)?;
    anchors(collector)?;
    seams(collector, region, rims, section)
}

fn faces_at_rims(
    collector: &mut Collector<'_>,
    rims: [SurfaceCurveId; 2],
    solid: &topods::Shape,
) -> Checked<()> {
    let mut faces = topexp::Explorer::new(solid, ShapeEnum::Face, ShapeEnum::Shape)
        .map_err(|e| native("full-turn faces", e))?;
    let mut found = Vec::new();
    while faces.more() {
        if let Some(image) = matching_rim_face(faces.current(), rims, &collector.images)? {
            found.push(image);
        }
        faces.next().map_err(|e| native("next full-turn face", e))?;
    }
    collector.images.extend(found);
    Ok(())
}

fn matching_rim_face(
    face: &topods::Shape,
    rims: [SurfaceCurveId; 2],
    bindings: &Bindings,
) -> Checked<Option<(HistoryKey, topods::Shape)>> {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(face, ShapeEnum::Edge, &mut edges)
        .map_err(|e| native("full-turn face edges", e))?;
    for rim in rims {
        let source = HistorySource::Curve(rim);
        let first = lookup(
            bindings,
            HistoryKey {
                source,
                role: edge_role(0),
            },
        )?;
        let last = lookup(
            bindings,
            HistoryKey {
                source,
                role: edge_role(1),
            },
        )?;
        if edges.find_index(first) != 0 && edges.find_index(last) != 0 {
            return Ok(Some((
                HistoryKey {
                    source,
                    role: HistoryRole::SideFace,
                },
                face.clone(),
            )));
        }
    }
    Ok(None)
}

fn anchors(collector: &mut Collector<'_>) -> Checked<()> {
    let anchors = collector
        .images
        .iter()
        .filter_map(|(key, shape)| {
            let role = match (key.source, key.role) {
                (HistorySource::Curve(_), HistoryRole::FirstDepthEdge) => vertex_role(0),
                (HistorySource::Curve(_), HistoryRole::LastDepthEdge) => vertex_role(1),
                _ => return None,
            };
            Some(anchor(key.source, role, shape))
        })
        .collect::<Checked<Vec<_>>>()?;
    collector.images.extend(anchors);
    Ok(())
}

fn anchor(
    source: HistorySource,
    role: HistoryRole,
    shape: &topods::Shape,
) -> Checked<(HistoryKey, topods::Shape)> {
    let edge = topods::edge(shape).map_err(|e| native("full-turn rim edge", e))?;
    let first = topexp::first_vertex(&edge, true);
    let last = topexp::last_vertex(&edge, true);
    if first.is_null() || !first.is_same(&last) {
        return Err(incomplete(
            "closed rim anchor",
            format!("{source:?}: distinct or null ends"),
        ));
    }
    Ok((HistoryKey { source, role }, first.into_shape()))
}

fn seams(
    collector: &mut Collector<'_>,
    region: SurfaceRegionId,
    rims: [SurfaceCurveId; 2],
    section: &Section,
) -> Checked<()> {
    for index in 0..4 {
        let owner = face_key(region, rims, section.first_depth, index);
        let shape = lookup(&collector.images, owner)?;
        let face = topods::face(shape).map_err(|e| native("full-turn seam owner", e))?;
        let surface = BRepAdaptorSurface::with_face(&face, false)
            .map_err(|e| native("full-turn seam surface", e))?;
        if surface
            .is_u_periodic()
            .map_err(|e| native("full-turn seam period", e))?
        {
            let seam = seam_image(collector, &section.edges[index], owner)?;
            collector.images.push((seam_key(owner), seam));
        }
    }
    Ok(())
}

fn seam_image(
    collector: &mut Collector<'_>,
    section_edge: &topods::Edge,
    owner: HistoryKey,
) -> Checked<topods::Shape> {
    let first = collector
        .maker
        .first_shape_of(section_edge)
        .map_err(|e| native("first seam image", e))?;
    let last = collector
        .maker
        .last_shape_of(section_edge)
        .map_err(|e| native("last seam image", e))?;
    if first.is_null() || !first.is_same(&last) {
        return Err(incomplete(
            "one shared seam image",
            format!("{owner:?}: distinct or null ends"),
        ));
    }
    Ok(first)
}

pub(super) fn face_key(
    region: SurfaceRegionId,
    rims: [SurfaceCurveId; 2],
    first_depth: usize,
    index: usize,
) -> HistoryKey {
    let (source, role) = if index.is_multiple_of(2) {
        (HistorySource::Curve(rims[index / 2]), HistoryRole::SideFace)
    } else {
        let depth = if index == 3 {
            first_depth
        } else {
            1 - first_depth
        };
        (HistorySource::Region(region), face_role(depth))
    };
    HistoryKey { source, role }
}

pub(super) fn seam_key(owner: HistoryKey) -> HistoryKey {
    let role = match owner.role {
        HistoryRole::FirstDepthFace => edge_role(0),
        HistoryRole::LastDepthFace => edge_role(1),
        _ => HistoryRole::NormalEdge,
    };
    HistoryKey {
        source: owner.source,
        role,
    }
}

pub(super) fn rim_keys(
    rims: [SurfaceCurveId; 2],
    first_depth: usize,
    index: usize,
) -> [HistoryKey; 2] {
    [index, (index + 1) % 4].map(|corner| {
        let depth = if corner == 0 || corner == 3 {
            first_depth
        } else {
            1 - first_depth
        };
        HistoryKey {
            source: HistorySource::Curve(rims[corner / 2]),
            role: edge_role(depth),
        }
    })
}

pub(super) fn lookup(bindings: &Bindings, key: HistoryKey) -> Checked<&topods::Shape> {
    bindings
        .iter()
        .find(|(candidate, _)| *candidate == key)
        .map(|(_, shape)| shape)
        .ok_or_else(|| incomplete("full-turn semantic binding", format!("missing {key:?}")))
}
