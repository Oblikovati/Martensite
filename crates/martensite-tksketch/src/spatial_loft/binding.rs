// SPDX-License-Identifier: GPL-2.0-only
//! Bind both original section identities to the same retained ruled band.
use super::{
    LoftFailure as Failure, LoftHistory as History, LoftImage, LoftLine, LoftOptions, LoftPoint,
    LoftSectionId, LoftStage as Stage, admission::AdmittedPair, failure::native, poll,
};
use crate::spatial_prism::{
    SpatialPrismFailure,
    volume::{FluxMode, certify_affine_flux},
};
use martensite_tkbo::profile_extrusion::{AchievedTolerance, Finding, Measured, ToleranceCap};
use martensite_tkbrep::{
    brep::{BRepTool, access::vertex_tshape},
    topexp::{self, Explorer},
    topods::{self, Edge, Face, Shape, Vertex},
    toptools::{IndexedDataMapOfShapeListOfShape, IndexedMapOfShape},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{gp::Point3, predicates::placed_point_distance_upper_bound};
use martensite_tkoffset::brep_offset_api::{CorrespondingRuledPair, CorrespondingRuledSection};
use martensite_tksketch3d::wire::DetachedWire;

#[derive(Debug)]
pub(super) struct Evidence {
    pub volume: Measured,
    pub trace_bound: f64,
    pub achieved: AchievedTolerance,
    pub findings: Vec<Finding>,
    pub images: Vec<LoftImage>,
}

struct Section {
    id: LoftSectionId,
    cap: Face,
    lines: Vec<(LoftLine, Edge)>,
    points: Vec<(LoftPoint, Vertex)>,
    vertex_trace: f64,
}

pub(super) fn bind(
    native_pair: &CorrespondingRuledPair,
    wires: &[DetachedWire; 2],
    pair: &AdmittedPair<'_>,
    options: LoftOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Evidence, Failure> {
    let limit = options.source_trace_limit;
    let [first, last] = [0, 1].map(|index| {
        section(
            pair.sections[index].id,
            &native_pair.sections[index],
            &wires[index],
            &pair.planes[index].points,
            limit,
            cancelled,
        )
    });
    let (first, last) = (first?, last?);
    let mut images = section_images(&first);
    images.extend(section_images(&last));
    images.extend(band_images(&native_pair.solid, [&first, &last], cancelled)?);
    check_coverage(&native_pair.solid, &images)?;
    let (volume, trace_bound) = measure(&native_pair.solid, [&first, &last], limit, cancelled)?;
    let achieved = achieved(&native_pair.solid)?;
    let findings = match options.resolution.cap {
        ToleranceCap::Report(limit) if achieved.max() > limit => {
            vec![Finding::ToleranceAboveReportLimit {
                achieved: achieved.max(),
                limit,
            }]
        }
        _ => Vec::new(),
    };
    Ok(Evidence {
        volume,
        trace_bound,
        achieved,
        findings,
        images,
    })
}

fn section(
    id: LoftSectionId,
    retained: &CorrespondingRuledSection,
    wire: &DetachedWire,
    authored: &[(martensite_tksketch3d::PointId, [f64; 3])],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Section, Failure> {
    let lines = wire
        .edges
        .iter()
        .map(|edge| {
            poll(cancelled, Stage::Binding)?;
            let directed = LoftLine {
                section: id,
                directed: edge.source,
            };
            let mapped = native(
                topods::edge(image(&retained.edges, &edge.edge)?),
                Stage::Binding,
            )?;
            Ok((directed, mapped))
        })
        .collect::<Result<_, Failure>>()?;
    let mut points = Vec::with_capacity(authored.len());
    let mut vertex_trace = 0.0_f64;
    for &(point, xyz) in authored {
        poll(cancelled, Stage::Binding)?;
        let point = LoftPoint { section: id, point };
        let (actual, bound) = bind_point(point, xyz, retained, wire, limit)?;
        vertex_trace = vertex_trace.max(bound);
        points.push((point, actual));
    }
    Ok(Section {
        id,
        cap: retained.cap.clone(),
        lines,
        points,
        vertex_trace,
    })
}

fn image<'a>(bindings: &'a [(Shape, Shape)], source: &Shape) -> Result<&'a Shape, Failure> {
    let mut found = bindings.iter().filter(|(input, _)| input.is_same(source));
    match (found.next(), found.next()) {
        (Some((_, image)), None) => Ok(image),
        _ => Err(Failure::Binding(
            "one retained image for each original section member",
        )),
    }
}

fn bind_point(
    key: LoftPoint,
    xyz: [f64; 3],
    retained: &CorrespondingRuledSection,
    wire: &DetachedWire,
    limit: f64,
) -> Result<(Vertex, f64), Failure> {
    let source = wire
        .points
        .iter()
        .find(|input| input.point == key.point)
        .ok_or(Failure::Binding(
            "one materialized original point for each section vertex",
        ))?;
    let actual = native(
        topods::vertex(image(&retained.vertices, &source.vertex)?),
        Stage::Binding,
    )?;
    let local = native(
        vertex_tshape(&actual, "ruled section vertex"),
        Stage::Binding,
    )?
    .pnt();
    let [x, y, z] = xyz;
    let bound = placed_point_distance_upper_bound(
        local,
        actual.location().transformation(),
        Point3::new(x, y, z),
    );
    if !bound.is_finite() || bound > limit {
        return Err(Failure::SectionVertexTrace {
            section: key.section,
            point: key.point,
            bound,
            limit,
        });
    }
    Ok((actual, bound))
}

fn section_images(section: &Section) -> Vec<LoftImage> {
    let cap = LoftImage {
        history: History::Cap(section.id),
        shape: section.cap.to_shape(),
    };
    let edges = section.lines.iter().map(|(key, edge)| LoftImage {
        history: History::SectionEdge(*key),
        shape: edge.to_shape(),
    });
    let vertices = section.points.iter().map(|(key, vertex)| LoftImage {
        history: History::SectionVertex(*key),
        shape: vertex.to_shape(),
    });
    std::iter::once(cap).chain(edges).chain(vertices).collect()
}

fn band_images(
    solid: &Shape,
    [first, last]: [&Section; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<LoftImage>, Failure> {
    let edge_faces = ancestors(solid, ShapeEnum::Edge, ShapeEnum::Face)?;
    let vertex_edges = ancestors(solid, ShapeEnum::Vertex, ShapeEnum::Edge)?;
    let mut rails = Vec::with_capacity(first.points.len());
    for ((a, av), (b, bv)) in first.points.iter().zip(&last.points) {
        poll(cancelled, Stage::Binding)?;
        let rail = unique_common(
            &vertex_edges,
            av,
            bv,
            "one rail joining two corresponding section vertices",
        )?;
        rails.push(LoftImage {
            history: History::Rail([*a, *b]),
            shape: rail,
        });
    }
    let mut images = Vec::with_capacity(first.lines.len() + rails.len());
    for (index, ((a, ae), (b, be))) in first.lines.iter().zip(&last.lines).enumerate() {
        poll(cancelled, Stage::Binding)?;
        let side = unique_common(
            &edge_faces,
            ae,
            be,
            "one side shared by two corresponding section edges",
        )?;
        require_rails(
            &side,
            &rails[index].shape,
            &rails[(index + 1) % rails.len()].shape,
        )?;
        images.push(LoftImage {
            history: History::Side([*a, *b]),
            shape: side,
        });
    }
    images.extend(rails);
    Ok(images)
}

fn ancestors(
    solid: &Shape,
    child: ShapeEnum,
    parent: ShapeEnum,
) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
    let mut map = IndexedDataMapOfShapeListOfShape::default();
    native(
        topexp::map_shapes_and_ancestors(solid, child, parent, &mut map),
        Stage::Binding,
    )?;
    Ok(map)
}

fn unique_common(
    ancestors: &IndexedDataMapOfShapeListOfShape,
    first: &Shape,
    last: &Shape,
    expected: &'static str,
) -> Result<Shape, Failure> {
    let starts = native(ancestors.find_from_key(first), Stage::Binding)?;
    let ends = native(ancestors.find_from_key(last), Stage::Binding)?;
    let mut found = IndexedMapOfShape::default();
    for shape in starts
        .iter()
        .filter(|shape| ends.iter().any(|end| end.is_same(shape)))
    {
        found.add(shape.clone());
    }
    if found.extent() != 1 {
        return Err(Failure::Binding(expected));
    }
    Ok(native(found.find_key(1), Stage::Binding)?.clone())
}

fn require_rails(side: &Shape, first: &Shape, last: &Shape) -> Result<(), Failure> {
    let mut boundary = IndexedMapOfShape::default();
    native(
        topexp::map_shapes_of_type(side, ShapeEnum::Edge, &mut boundary),
        Stage::Binding,
    )?;
    if boundary.extent() != 4 || !boundary.contains(first) || !boundary.contains(last) {
        return Err(Failure::Binding(
            "two cap edges and two actual corresponding rails on each side",
        ));
    }
    Ok(())
}

fn check_coverage(solid: &Shape, images: &[LoftImage]) -> Result<(), Failure> {
    let mut expected = IndexedMapOfShape::default();
    for kind in [ShapeEnum::Face, ShapeEnum::Edge, ShapeEnum::Vertex] {
        native(
            topexp::map_shapes_of_type(solid, kind, &mut expected),
            Stage::Binding,
        )?;
    }
    let mut actual = IndexedMapOfShape::default();
    for image in images {
        if !expected.contains(&image.shape) || actual.contains(&image.shape) {
            return Err(Failure::Binding(
                "one distinct lineage per native face, edge and vertex",
            ));
        }
        actual.add(image.shape.clone());
    }
    if actual.extent() != expected.extent() {
        return Err(Failure::Binding("complete face, edge and vertex coverage"));
    }
    Ok(())
}

fn measure(
    solid: &Shape,
    [first, last]: [&Section; 2],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Measured, f64), Failure> {
    let reference = *native(BRepTool::pnt(&first.points[0].1), Stage::Binding)?.coordinates();
    let explorer = native(
        Explorer::new(solid, ShapeEnum::Face, ShapeEnum::Shape),
        Stage::Binding,
    )?;
    let faces = explorer
        .map(|face| native(topods::face(&face), Stage::Binding))
        .collect::<Result<Vec<_>, _>>()?;
    let vertex_trace = first.vertex_trace.max(last.vertex_trace);
    certify_affine_flux(
        &faces,
        reference,
        vertex_trace,
        limit,
        FluxMode::Ruled,
        cancelled,
    )
    .map_err(|cause| match cause {
        SpatialPrismFailure::Cancelled(_) => Failure::Cancelled(Stage::Binding),
        source => Failure::OutputProof(source),
    })
}

fn achieved(solid: &Shape) -> Result<AchievedTolerance, Failure> {
    let tolerance = |kind| native(BRepTool::max_tolerance(solid, kind), Stage::Validation);
    Ok(AchievedTolerance {
        face: tolerance(ShapeEnum::Face)?,
        edge: tolerance(ShapeEnum::Edge)?,
        vertex: tolerance(ShapeEnum::Vertex)?,
    })
}
