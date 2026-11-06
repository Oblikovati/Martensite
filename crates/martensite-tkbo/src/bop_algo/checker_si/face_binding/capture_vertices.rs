// SPDX-License-Identifier: GPL-2.0-only
//! Read direct endpoint occurrences; no first-face-UV query or mutating range accessor.
use super::{
    capture_curves::{placement, unsupported},
    capture_edges::identity,
    types::*,
};
use martensite_tkbrep::{
    brep::{BRepTVertex, BRepTool},
    topods::{self, Edge, Shape, ShapeIterator, Vertex},
};
use martensite_tkg3d::top_abs::{self, Orientation, ShapeEnum};

pub(super) fn capture(edge: &Edge) -> Result<(Vec<OwnedVertexUse>, Vec<Vertex>), BindingFailure> {
    let forward = topods::edge(&edge.oriented(Orientation::Forward))?;
    let mut owned = Vec::new();
    let mut sources = Vec::new();
    for (position, raw) in ShapeIterator::new(edge, false, false).enumerate() {
        if raw.shape_type()? != ShapeEnum::Vertex {
            return Err(unsupported(
                "edge child",
                format!("{:?}; expected Vertex", raw.shape_type()?),
            ));
        }
        let located = raw.located(&edge.location().multiplied(raw.location())?, false)?;
        let intrinsic = topods::vertex(&located.oriented(raw.orientation()))?;
        let boundary = located.oriented(top_abs::compose(edge.orientation(), raw.orientation()));
        owned.push(vertex(position, &raw, &intrinsic, &boundary, &forward)?);
        sources.push(intrinsic);
    }
    Ok((owned, sources))
}
fn vertex(
    position: usize,
    raw: &Shape,
    intrinsic: &Vertex,
    boundary: &Shape,
    forward: &Edge,
) -> Result<OwnedVertexUse, BindingFailure> {
    let handle = raw.tshape().cast::<BRepTVertex>();
    let source = handle
        .get()
        .ok_or_else(|| unsupported("vertex TShape", "not BRepTVertex"))?;
    let point = source.pnt();
    Ok(OwnedVertexUse {
        raw_placement: placement(raw.location().transformation())?,
        position,
        raw: identity(raw)?,
        intrinsic: identity(intrinsic)?,
        boundary: identity(boundary)?,
        local_point: [point.x(), point.y(), point.z()],
        placement: placement(intrinsic.location().transformation())?,
        raw_tolerance: source.tolerance(),
        effective_tolerance: BRepTool::vertex_tolerance(intrinsic)?,
        point_representation_count: source.points().len(),
        parameter: parameter(intrinsic, forward)?,
    })
}
fn parameter(
    vertex: &Vertex,
    forward: &Edge,
) -> Result<EndpointParameterObservation, BindingFailure> {
    let side = match vertex.orientation() {
        Orientation::Forward => EndpointSide::First,
        Orientation::Reversed => EndpointSide::Last,
        orientation => return Ok(EndpointParameterObservation::NotQueried { orientation }),
    };
    let range = BRepTool::range(forward)?;
    let modified_before = forward.modified()?;
    let returned = BRepTool::find_parameter(vertex, forward);
    Ok(EndpointParameterObservation::TopologicalRangeEnd {
        side,
        recorded_range: [range.0, range.1],
        returned,
        modified_before,
        modified_after: forward.modified()?,
    })
}
pub(super) fn raw_tolerance(shape: &Shape) -> Result<f64, BindingFailure> {
    crate::sketch_region_geometry_capture::capture_raw_entity_tolerance(
        shape,
        "owned endpoint capture",
    )?
    .ok_or_else(|| {
        unsupported(
            "tolerance entity",
            format!("{:?}; expected vertex, edge or face", shape.shape_type()),
        )
    })
}
pub(super) fn relations(vertices: &[Vertex]) -> Vec<OccurrenceRelation> {
    vertices
        .iter()
        .enumerate()
        .flat_map(|(left, a)| {
            vertices
                .iter()
                .enumerate()
                .skip(left + 1)
                .map(move |(right, b)| OccurrenceRelation {
                    left,
                    right,
                    same: a.is_same(b),
                    location_equal: a.location() == b.location(),
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkbrep::{brep::BRepBuilder, topods::Builder};
    use martensite_tkernel::standard::Handle;
    use martensite_tkg3d::geom::{GeomCurve, GeomLine};
    use martensite_tkmath::{
        gp::{Axis1, Point3, Transform3, Vector3},
        toploc::Location,
    };

    fn edge() -> Edge {
        let mut edge = Edge::default();
        let curve = Handle::new(GeomLine::new(Axis1::default())).cast::<dyn GeomCurve>();
        BRepBuilder::default()
            .make_edge_with_curve(&mut edge, &curve, 1e-7)
            .unwrap();
        BRepBuilder::default().range(&edge, 2., 7., false).unwrap();
        edge
    }
    fn vertex() -> Vertex {
        let mut vertex = Vertex::default();
        BRepBuilder::default()
            .make_vertex_at(&mut vertex, Point3::new(1., 2., 3.), 1e-9)
            .unwrap();
        vertex
    }
    fn repeated_edge() -> (Edge, Vertex) {
        let mut edge = edge();
        let vertex = vertex();
        Builder
            .add(&mut edge, &vertex.oriented(Orientation::Forward))
            .unwrap();
        Builder
            .add(&mut edge, &vertex.oriented(Orientation::Reversed))
            .unwrap();
        (edge, vertex)
    }
    #[test]
    fn repeated_endpoint_uses_keep_identity_but_distinct_range_sides() {
        let (edge, _) = repeated_edge();
        let (owned, sources) = capture(&edge).unwrap();
        assert_eq!(owned.len(), 2);
        assert!(sources[0].is_same(&sources[1]));
        assert!(matches!(
            owned[0].parameter,
            EndpointParameterObservation::TopologicalRangeEnd {
                side: EndpointSide::First,
                returned: Ok(Some(2.)),
                ..
            }
        ));
        assert!(matches!(
            owned[1].parameter,
            EndpointParameterObservation::TopologicalRangeEnd {
                side: EndpointSide::Last,
                returned: Ok(Some(7.)),
                ..
            }
        ));
        assert!(relations(&sources)[0].same);
    }
    #[test]
    fn boundary_reversal_never_reverses_the_intrinsic_parameter_query_twice() {
        let (edge, _) = repeated_edge();
        let reversed = topods::edge(&edge.reversed()).unwrap();
        let (normal, _) = capture(&edge).unwrap();
        let (backward, _) = capture(&reversed).unwrap();
        assert_eq!(
            normal[0].intrinsic.orientation,
            backward[0].intrinsic.orientation
        );
        assert_ne!(
            normal[0].boundary.orientation,
            backward[0].boundary.orientation
        );
        assert!(matches!(
            backward[0].parameter,
            EndpointParameterObservation::TopologicalRangeEnd {
                returned: Ok(Some(2.)),
                ..
            }
        ));
    }
    #[test]
    fn raw_and_placed_vertex_operands_remain_separate() {
        let (edge, _) = repeated_edge();
        let mut transform = Transform3::default();
        transform.set_translation(Vector3::new(10., 0., 0.));
        let moved = topods::edge(&edge.located(&Location::new(transform), false).unwrap()).unwrap();
        let (owned, _) = capture(&moved).unwrap();
        assert_eq!(owned[0].local_point, [1., 2., 3.]);
        assert!(owned[0].raw.location_datums.is_empty());
        assert!(!owned[0].intrinsic.location_datums.is_empty());
        assert_eq!(owned[0].placement.origin, [10., 0., 0.]);
        assert_eq!(owned[0].raw_placement.origin, [0.; 3]);
    }
    #[test]
    fn parameter_read_does_not_set_modified_and_raw_tolerance_is_not_floored() {
        let (edge, _) = repeated_edge();
        edge.set_modified(false).unwrap();
        let (owned, _) = capture(&edge).unwrap();
        assert!(!edge.modified().unwrap());
        assert_eq!(owned[0].raw_tolerance, 1e-9);
        assert_eq!(owned[0].effective_tolerance, 1e-7);
        assert!(matches!(
            owned[0].parameter,
            EndpointParameterObservation::TopologicalRangeEnd {
                modified_before: false,
                modified_after: false,
                ..
            }
        ));
    }
    #[test]
    fn owned_vertex_snapshot_does_not_follow_later_point_mutation() {
        let (edge, vertex) = repeated_edge();
        let (owned, _) = capture(&edge).unwrap();
        vertex
            .tshape()
            .cast::<BRepTVertex>()
            .get()
            .unwrap()
            .set_pnt(Point3::new(9., 9., 9.));
        assert_eq!(owned[0].local_point, [1., 2., 3.]);
        assert_eq!(capture(&edge).unwrap().0[0].local_point, [9., 9., 9.]);
    }
    #[test]
    fn internal_use_is_recorded_without_projection_or_parameter_fallback() {
        let mut edge = edge();
        let vertex = vertex();
        Builder
            .add(&mut edge, &vertex.oriented(Orientation::Internal))
            .unwrap();
        let (owned, _) = capture(&edge).unwrap();
        assert!(matches!(
            owned[0].parameter,
            EndpointParameterObservation::NotQueried {
                orientation: Orientation::Internal
            }
        ));
    }
    #[test]
    fn same_vertex_tshape_with_distinct_location_is_not_same_incidence() {
        let mut edge = edge();
        let vertex = vertex();
        let mut transform = Transform3::default();
        transform.set_translation(Vector3::new(1., 0., 0.));
        Builder.add(&mut edge, &vertex).unwrap();
        Builder
            .add(
                &mut edge,
                &vertex.located(&Location::new(transform), false).unwrap(),
            )
            .unwrap();
        let (_, sources) = capture(&edge).unwrap();
        assert!(!relations(&sources)[0].same);
        assert!(!relations(&sources)[0].location_equal);
    }
}
