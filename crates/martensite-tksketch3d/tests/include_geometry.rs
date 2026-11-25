// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, pid, point};
use martensite_tkbrep::{
    brep::{BRepTEdge, BRepTVertex, BRepTool, line_edge_capture::LineEdgeCaptureError},
    topexp::{self, Explorer},
    topods::{self, Edge, Vertex},
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::{
    gp::{Direction3, Line3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tksketch3d::{
    CurveId, CurveRole, Edit, EditFailure, MinimumLength, Snapshot,
    constraints::{ConstrainedSketch, ConstraintFailure},
    include_geometry::*,
    split_plane::{SegmentWorkPlane, SplitLineFailure, SplitLineIds},
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use std::num::NonZeroU64;

struct NativeFixture {
    edge: Edge,
    vertices: [Vertex; 2],
}

impl NativeFixture {
    fn located_line(translation: [f64; 3], reversed: bool) -> Self {
        let line = Line3::new(point([2., 0., 0.]), Direction3::Z);
        let edge = BRepBuilderAPIMakeEdge::from_lin_params(line, 1., 3.)
            .unwrap()
            .edge()
            .unwrap();
        let [x, y, z] = translation;
        let place = Location::new(Transform3::translation_vector(Vector3::new(x, y, z)));
        let edge = topods::edge(&edge.moved(&place, false).unwrap()).unwrap();
        let mut result = Self::new(edge);
        if reversed {
            result.edge = topods::edge(&result.edge.reversed()).unwrap();
        }
        result
    }

    fn new(edge: Edge) -> Self {
        let (a, b) = topexp::vertices(&edge, false);
        Self {
            edge,
            vertices: [a, b],
        }
    }

    fn resolve(&self, version: IncludedLineVersion) -> IncludedLineResolution<'_> {
        IncludedLineResolution::Unique(IncludedLineObservation {
            version,
            edge: &self.edge,
            vertices: [0, 1].map(|index| IncludedVertexObservation {
                name: vertex_name(index as u64 + 11),
                vertex: &self.vertices[index],
            }),
        })
    }
}

fn vertex_name(value: u64) -> IncludedVertexName {
    IncludedVertexName(NonZeroU64::new(value).unwrap())
}

fn version(path: u64, revision: u64) -> IncludedLineVersion {
    IncludedLineVersion {
        key: IncludedLineKey {
            document: *b"included-line-01",
            path: NonZeroU64::new(path).unwrap(),
            occurrence: NonZeroU64::new(7).unwrap(),
        },
        revision: IncludedSourceRevision {
            incarnation: 6,
            revision,
        },
    }
}

fn ids(offset: u64) -> IncludedLineIds {
    IncludedLineIds {
        line: lid(offset + 3),
        endpoints: [pid(offset + 1), pid(offset + 2)],
    }
}
fn empty() -> Snapshot {
    Snapshot::empty(MinimumLength::new(1e-6).unwrap())
}

#[test]
fn include_reversed_located_trim_and_independent_nominated_endpoint_names() {
    let source = NativeFixture::located_line([5., 2., 10.], true);
    let revision = version(20, 1);
    let sketch = empty()
        .include_native_line(ids(0), revision, source.resolve(revision), || false)
        .unwrap();
    let accepted = sketch.included_line(lid(3)).unwrap();
    assert_eq!(accepted.source, revision);
    assert_eq!(accepted.native.orientation, Orientation::Reversed);
    assert_eq!(accepted.native.range, [1., 3.]);
    assert_eq!(accepted.vertices, [vertex_name(12), vertex_name(11)]);
    assert_eq!(
        sketch.line(lid(3)).unwrap().points,
        [point([7., 2., 13.]), point([7., 2., 11.])]
    );
    assert_eq!(
        sketch.included_point_source(pid(1)).unwrap().vertex,
        vertex_name(12)
    );
    assert!(sketch.frozen_source(pid(1)).is_none());

    let IncludedLineResolution::Unique(mut reordered) = source.resolve(revision) else {
        panic!("fixture resolves uniquely")
    };
    reordered.vertices.swap(0, 1);
    assert_eq!(
        empty()
            .include_native_line(
                ids(0),
                revision,
                IncludedLineResolution::Unique(reordered),
                || false
            )
            .unwrap(),
        sketch
    );
    assert_immutable_candidate(&sketch, &source, revision);
}

fn assert_immutable_candidate(
    sketch: &Snapshot,
    source: &NativeFixture,
    version: IncludedLineVersion,
) {
    let frozen = sketch.clone();
    let vertex = source.vertices[0].tshape().downcast::<BRepTVertex>();
    vertex.get().unwrap().set_pnt(point([9., 2., 1.]));
    assert_eq!(*sketch, frozen);
    assert!(matches!(
        empty().include_native_line(ids(0), version, source.resolve(version), || false),
        Err(IncludeGeometryError::Capture { source, cause })
            if source == version && matches!(*cause, LineEdgeCaptureError::Endpoint { .. })
    ));
}

#[test]
fn rebind_changes_only_the_same_source_revision_and_preserves_named_ids() {
    let a = NativeFixture::located_line([0.; 3], false);
    let other = NativeFixture::located_line([10., 0., 0.], false);
    let old = version(20, 1);
    let unrelated = version(21, 1);
    let sketch = empty()
        .include_native_line(ids(0), old, a.resolve(old), || false)
        .unwrap()
        .include_native_line(ids(3), unrelated, other.resolve(unrelated), || false)
        .unwrap()
        .apply(
            &[Edit::SetCurveRole {
                curve: CurveId::Line(lid(3)),
                role: CurveRole::Construction,
            }],
            || false,
        )
        .unwrap();
    let newer = NativeFixture::located_line([5., 2., 10.], true);
    let next = version(20, 2);
    assert!(sketch.included_line(lid(3)).unwrap().is_affected_by(next));
    assert!(!sketch.included_line(lid(6)).unwrap().is_affected_by(next));
    let request = IncludedLineRebind {
        line: lid(3),
        previous: old,
        next,
    };
    let rebound = sketch
        .rebind_included_line(request, newer.resolve(next), || false)
        .unwrap();
    assert_eq!(
        rebound.line(lid(3)).unwrap().line.endpoints,
        [pid(2), pid(1)]
    );
    assert_eq!(
        rebound.point(pid(1)).unwrap().position,
        point([7., 2., 11.])
    );
    assert_eq!(
        rebound.point(pid(2)).unwrap().position,
        point([7., 2., 13.])
    );
    assert_eq!(
        rebound.included_point_source(pid(1)).unwrap().vertex,
        vertex_name(11)
    );
    assert_eq!(rebound.included_line(lid(6)), sketch.included_line(lid(6)));
    assert_eq!(
        rebound.curve_role(CurveId::Line(lid(3))),
        Some(CurveRole::Construction)
    );
    assert!(!rebound.included_line(lid(3)).unwrap().is_affected_by(old));
    assert!(matches!(
        rebound.rebind_included_line(request, newer.resolve(next), || false),
        Err(IncludeGeometryError::StaleLink { .. })
    ));
}

#[test]
fn refusal_of_wrong_missing_and_ambiguous_rebind_leaves_the_link_unchanged() {
    let source = NativeFixture::located_line([0.; 3], false);
    let old = version(20, 1);
    let next = version(20, 2);
    let before = empty()
        .include_native_line(ids(0), old, source.resolve(old), || false)
        .unwrap();
    let request = IncludedLineRebind {
        line: lid(3),
        previous: old,
        next,
    };
    for (resolution, error) in [
        (
            IncludedLineResolution::Missing,
            IncludeGeometryError::Missing(next),
        ),
        (
            IncludedLineResolution::Ambiguous,
            IncludeGeometryError::Ambiguous(next),
        ),
        (
            source.resolve(old),
            IncludeGeometryError::WrongRevision {
                expected: next,
                actual: old,
            },
        ),
        (
            source.resolve(version(21, 2)),
            IncludeGeometryError::WrongSource {
                expected: next.key,
                actual: version(21, 2).key,
            },
        ),
    ] {
        assert_eq!(
            before.rebind_included_line(request, resolution, || false),
            Err(error)
        );
    }
    assert_eq!(
        before.rebind_included_line(request, source.resolve(next), || true),
        Err(IncludeGeometryError::Cancelled)
    );
    let IncludedLineResolution::Unique(mut renamed) = source.resolve(next) else {
        panic!("fixture resolves uniquely")
    };
    renamed.vertices[0].name = vertex_name(13);
    assert!(matches!(
        before.rebind_included_line(request, IncludedLineResolution::Unique(renamed), || false),
        Err(IncludeGeometryError::ChangedVertices { .. })
    ));
    assert_eq!(before.included_line(lid(3)).unwrap().source, old);
}

#[test]
fn break_link_retains_the_geometry_and_makes_both_endpoints_editable() {
    let source = NativeFixture::located_line([0.; 3], false);
    let revision = version(1, 1);
    let sketch = empty()
        .include_native_line(ids(0), revision, source.resolve(revision), || false)
        .unwrap();
    let displacement = Edit::MovePoint {
        point: pid(1),
        position: point([4., 0., 1.]),
    };
    assert!(matches!(
        sketch.apply(&[displacement], || false),
        Err(EditFailure::IncludedPointMove { .. })
    ));
    assert_eq!(
        sketch.apply(&[Edit::DetachFrozenPoint(pid(1))], || false),
        Err(EditFailure::IncludedLineEdit(lid(3)))
    );
    let (detached, old) = sketch.detach_included_line(lid(3), || false).unwrap();
    assert_eq!(Some(old), sketch.included_line(lid(3)));
    assert_eq!(detached.line(lid(3)), sketch.line(lid(3)));
    assert_eq!(
        detached.owned_ids().collect::<Vec<_>>(),
        sketch.owned_ids().collect::<Vec<_>>()
    );
    assert!(detached.included_point_source(pid(2)).is_none());
    assert!(detached.apply(&[displacement], || false).is_ok());
    let removed = sketch.apply(&[Edit::RemoveLine(lid(3))], || false).unwrap();
    assert_eq!(removed.owned_ids().count(), 0);
    assert!(removed.included_line(lid(3)).is_none());
}

#[test]
fn normal_solver_and_split_respect_linked_geometry() {
    let source = NativeFixture::located_line([0.; 3], false);
    let revision = version(1, 1);
    let sketch = empty()
        .include_native_line(ids(0), revision, source.resolve(revision), || false)
        .unwrap();
    let constrained = ConstrainedSketch::new(sketch.clone(), crate::constraints::policy()).unwrap();
    assert_eq!(constrained.inspect_rank().unwrap().frozen_coordinates, 6);
    assert!(
        matches!(constrained.drag(pid(1), point([2.,0.,0.]), || false), Err(ConstraintFailure::IncludedDrag(id, _)) if id == pid(1))
    );
    let plane = SegmentWorkPlane::new([0., 0., 2.], [0., 0., 1.]).unwrap();
    let split = SplitLineIds {
        cut: pid(4),
        fragments: [lid(5), lid(6)],
    };
    assert!(
        matches!(sketch.split_line_at_plane(lid(3), plane, split, 0., || false), Err(SplitLineFailure::Edit(EditFailure::IncludedLineEdit(id))) if id == lid(3))
    );
}

#[test]
fn include_an_actual_solid_edge_with_other_face_representations() {
    let mut solid = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    let shape = solid.shape().unwrap();
    let first = point([0., 0., 0.]);
    let last = point([0., 0., 4.]);
    let target = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .map(|candidate| NativeFixture::new(topods::edge(&candidate).unwrap()))
        .find(|capture| {
            let positions = capture
                .vertices
                .each_ref()
                .map(|vertex| BRepTool::pnt(vertex).unwrap());
            positions == [first, last] || positions == [last, first]
        })
        .unwrap();
    let tshape = target.edge.tshape().downcast::<BRepTEdge>();
    assert!(tshape.get().unwrap().curves().len() >= 3);
    let revision = version(23, 5);
    let accepted = empty()
        .include_native_line(ids(0), revision, target.resolve(revision), || false)
        .unwrap();
    let endpoints = accepted.line(lid(3)).unwrap().points;
    assert!(endpoints == [first, last] || endpoints == [last, first]);
}

#[test]
fn concurrent_includes_are_equal_and_cancellation_leaves_no_partial_link() {
    let source = NativeFixture::located_line([3., 4., 5.], false);
    let revision = version(23, 5);
    let seed = empty();
    let mut polls = 0;
    let expected = seed
        .include_native_line(ids(0), revision, source.resolve(revision), || {
            polls += 1;
            false
        })
        .unwrap();
    for stop in 1..=polls {
        let mut current = 0;
        let cancelled =
            seed.include_native_line(ids(0), revision, source.resolve(revision), || {
                current += 1;
                current == stop
            });
        assert_eq!(cancelled, Err(IncludeGeometryError::Cancelled));
    }
    assert_eq!(seed.owned_ids().count(), 0);

    std::thread::scope(|workers| {
        let jobs = [0, 1, 2].map(|_| {
            workers.spawn(|| {
                seed.include_native_line(ids(0), revision, source.resolve(revision), || false)
                    .unwrap()
            })
        });
        for job in jobs {
            assert_eq!(job.join().unwrap(), expected);
        }
    });
}
