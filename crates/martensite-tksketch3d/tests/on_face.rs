// SPDX-License-Identifier: GPL-2.0-only
mod curve;

use crate::{
    constraints::{length, policy, rid},
    fixture::{lid, line, pid, point, seed},
};
use martensite_tkbrep::{
    brep::{
        BRepTool,
        face_input_capture::FaceCaptureLimits,
        planar_attachment::{
            AttachmentSource, PlanarAttachmentLocation, capture_planar_attachment,
        },
    },
    topexp::Explorer,
    topods::{self, Face, Solid, Wire},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Plane3, Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tksketch3d::{
    PointRole, Snapshot,
    constraints::{
        ConstrainedSketch, PlanarAttachedSketch, PlanarFaceBinding,
        PlanarFaceConstraintFailure as Failure, PlanarFaceRevision, RelationEdit, RelationSpec,
    },
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};
use std::num::NonZeroU64;

fn key(key: u64) -> NonZeroU64 {
    NonZeroU64::new(key).unwrap()
}

fn square(center: f64, radius: f64, reverse: bool) -> Wire {
    let mut corners = [[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]];
    if reverse {
        corners.reverse();
    }
    let mut wire = BRepBuilderAPIMakePolygon::new();
    for [x, y] in corners {
        wire.add_point(point([center + radius * x, radius * y, 0.]))
            .unwrap();
    }
    wire.close().unwrap();
    wire.wire().unwrap()
}

fn native(hole: f64) -> (Solid, Face) {
    let mut face =
        BRepBuilderAPIMakeFace::from_plane_wire(Plane3::default(), &square(0., 4., false), true)
            .unwrap();
    face.add(&square(hole, 1., true)).unwrap();
    let mut prism =
        BRepPrimAPIMakePrism::new(&face.face().unwrap(), Vector3::new(0., 0., 2.), false, true)
            .unwrap();
    let solid = topods::solid(prism.shape().unwrap()).unwrap();
    let face = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .map(|occurrence| topods::face(&occurrence).unwrap())
        .find(|face| {
            face.nb_children() == 2
                && BRepTool::surface_with_location(face)
                    .unwrap()
                    .1
                    .is_identity()
        })
        .unwrap();
    (solid, face)
}

fn proof(revision: u64, hole: f64, salt: u64) -> PlanarFaceBinding {
    let (solid, face) = native(hole);
    captured(&solid, &face, revision, salt)
}

fn captured(solid: &Solid, face: &Face, revision: u64, salt: u64) -> PlanarFaceBinding {
    PlanarFaceBinding::new(
        capture_planar_attachment(
            solid,
            face,
            AttachmentSource {
                document: [42; 16],
                body: key(10),
                face: key(20),
                incarnation: 7,
                revision,
            },
            FaceCaptureLimits::bounded(),
            || false,
        )
        .unwrap(),
        key(salt),
    )
}

fn ordinary(coupled: bool) -> ConstrainedSketch {
    let mut edits = vec![
        seed(1, [-2., 0., 0.], PointRole::Defining),
        seed(2, [0., 0., 4.], PointRole::Defining),
        line(30, 1, 2),
    ];
    let mut relations = vec![RelationEdit::Insert(
        rid(40),
        RelationSpec::FixPoint(pid(2)),
    )];
    if coupled {
        edits.push(seed(3, [-2., 0., 0.], PointRole::SketchMarker));
        relations.push(RelationEdit::Insert(
            rid(41),
            RelationSpec::Coincident([pid(1), pid(3)]),
        ));
    }
    let section = Snapshot::empty(length(0.01))
        .apply(&edits, || false)
        .unwrap();
    ConstrainedSketch::new(section, policy())
        .unwrap()
        .apply(&relations, || false)
        .unwrap()
}

fn attach(base: &ConstrainedSketch, binding: PlanarFaceBinding) -> PlanarAttachedSketch {
    base.attach_point_on_face(rid(90), pid(1), binding, || false)
        .unwrap()
}

fn location(sketch: &PlanarAttachedSketch) -> [f64; 3] {
    *sketch
        .geometry()
        .point(pid(1))
        .unwrap()
        .position
        .coordinates()
}

#[test]
fn same_gesture_admits_hole_boundary_refuses_hole_and_continues() {
    fn shared<T: Send + Sync>() {}
    shared::<PlanarAttachedSketch>();
    shared::<PlanarFaceBinding>();
    shared::<PlanarFaceRevision>();
    shared::<Failure>();
    shared::<martensite_tksketch3d::constraints::PlanarFaceRelation>();
    shared::<martensite_tksketch3d::constraints::PlanarFaceDragUpdate>();
    let binding = proof(1, 0., 70);
    let source = binding.revision();
    let base = ordinary(false);
    let initial = attach(&base, binding.clone());
    assert_eq!(initial.relation().uv, [-2., 0.]);
    let rank = initial.inspect_rank().unwrap();
    assert_eq!(
        (rank.rank, rank.degrees_of_freedom, rank.equations),
        (6, 2, 6)
    );

    let boundary = initial
        .drag_preferred(pid(1), point([-1., 0., 0.]), 1e-8, || source, || false)
        .unwrap();
    assert_eq!(
        boundary.world_projection.location,
        PlanarAttachmentLocation::HoleBoundary
    );
    assert_eq!(boundary.uv_lift, boundary.world_projection);
    assert_eq!(boundary.world_projection.distance_bound, 0.);
    assert_eq!(boundary.world_projection.uv, [[-1.; 2], [0.; 2]]);
    assert_eq!(
        boundary.world_projection.projected,
        [[-1.; 2], [0.; 2], [0.; 2]]
    );
    assert_eq!(location(&boundary.sketch), [-1., 0., 0.]);
    assert_eq!(boundary.sketch.relation().uv, [-1., 0.]);
    let accepted = boundary.sketch;
    assert!(matches!(
        accepted.drag_preferred(pid(1), point([0.; 3]), 1e-8, || source, || false),
        Err(Failure::Trim(PlanarAttachmentLocation::HoleInterior))
    ));
    let corrupt_uv = martensite_tksketch3d::constraints::PlanarFaceRelation {
        uv: [(-1.0_f64).next_up(), 0.],
        ..accepted.relation()
    };
    assert!(matches!(
        accepted.detach(|| false).unwrap().restore_point_on_face(
            corrupt_uv,
            binding.clone(),
            || false
        ),
        Err(Failure::Trim(PlanarAttachmentLocation::HoleInterior))
    ));
    let inside = point([(-1.0_f64).next_up(), 0., 0.]);
    assert!(matches!(
        accepted.drag_preferred(pid(1), inside, 1e-8, || source, || false),
        Err(Failure::Trim(PlanarAttachmentLocation::HoleInterior))
    ));
    assert_eq!(location(&accepted), [-1., 0., 0.]);
    assert_eq!(accepted.relation().source, source);
    assert!(matches!(
        accepted.drag_preferred(pid(1), point([5., 0., 0.]), 1e-8, || source, || false),
        Err(Failure::Trim(PlanarAttachmentLocation::OutsideOuter))
    ));

    let recovered = accepted
        .drag_preferred(pid(1), point([2., 0., 0.]), 1e-8, || source, || false)
        .unwrap();
    assert_eq!(
        recovered.world_projection.location,
        PlanarAttachmentLocation::Material
    );
    for (actual, expected) in location(&recovered.sketch).into_iter().zip([2., 0., 0.]) {
        assert!((actual - expected).abs() <= 1e-9);
    }
    let outer = recovered
        .sketch
        .drag_preferred(pid(1), point([4., 0., 0.]), 1e-8, || source, || false)
        .unwrap();
    assert_eq!(
        outer.world_projection.location,
        PlanarAttachmentLocation::OuterBoundary
    );
    assert_eq!(location(&outer.sketch), [4., 0., 0.]);
    assert_eq!(recovered.rank, rank);
    assert_eq!(
        recovered
            .sketch
            .geometry()
            .line(lid(30))
            .unwrap()
            .line
            .endpoints,
        [pid(1), pid(2)]
    );
    assert_same_reopened_snapshot(&recovered.sketch, binding);
    assert_eq!(location(&initial), [-2., 0., 0.]);
}

fn assert_same_reopened_snapshot(attached: &PlanarAttachedSketch, binding: PlanarFaceBinding) {
    let relations = attached.ordinary_relations().collect::<Vec<_>>();
    let plain =
        ConstrainedSketch::restore(attached.geometry().clone(), &relations, policy(), || false)
            .unwrap();
    let replay = plain
        .restore_point_on_face(attached.relation(), binding, || false)
        .unwrap();
    assert_eq!(replay.geometry(), attached.geometry());
    assert_eq!(replay.relation(), attached.relation());
    assert_eq!(
        replay.inspect_rank().unwrap(),
        attached.inspect_rank().unwrap()
    );
}

#[test]
fn another_cursor_point_moves_attachment_through_the_real_hard_program() {
    let binding = proof(2, 0., 71);
    let source = binding.revision();
    let sketch = attach(&ordinary(true), binding);
    let moved = sketch
        .drag_preferred(pid(3), point([2.5, 0.5, 0.75]), 1e-8, || source, || false)
        .unwrap();
    let actual = location(&moved.sketch);
    assert_eq!((moved.rank.rank, moved.rank.degrees_of_freedom), (9, 2));
    assert_eq!(moved.rank, moved.sketch.inspect_rank().unwrap());
    assert_eq!(
        moved.world_projection.location,
        PlanarAttachmentLocation::Material
    );
    assert_eq!(moved.uv_lift.location, PlanarAttachmentLocation::Material);
    assert_eq!(
        moved.world_projection.projected,
        [[actual[0]; 2], [actual[1]; 2], [0.; 2]]
    );
    assert_eq!(moved.world_projection.distance_bound, actual[2].abs());
    let uv = moved.sketch.relation().uv;
    assert_eq!(moved.uv_lift.uv, [[uv[0]; 2], [uv[1]; 2]]);
    assert_eq!(moved.uv_lift.projected, [[uv[0]; 2], [uv[1]; 2], [0.; 2]]);
    let lift_error = moved.uv_lift.distance_bound;
    assert!(lift_error >= moved.world_projection.distance_bound);
    assert!(lift_error <= policy().length_tolerance.value());
    assert!((actual[0] - 2.5).abs() <= 1e-8);
    assert!((actual[1] - 0.5).abs() <= 1e-8);
    assert!(actual[2].abs() <= 1e-8);
    let linked = moved.sketch.geometry().point(pid(3)).unwrap().position;
    assert!(linked.distance(&point(actual)) <= policy().length_tolerance.value());
    assert_eq!(location(&sketch), [-2., 0., 0.]);
}

#[test]
fn restore_rebind_and_detach_keep_the_named_source_and_prior_geometry_isolated() {
    let base = ordinary(false);
    let binding = proof(1, 0., 70);
    let source = binding.revision();
    let original = attach(&base, binding.clone());
    let stored = original.relation();
    let reopened = base
        .restore_point_on_face(stored, binding.clone(), || false)
        .unwrap();
    assert_eq!(reopened.relation(), stored);
    assert_eq!(reopened.geometry(), original.geometry());
    let signed_zero = martensite_tksketch3d::constraints::PlanarFaceRelation {
        uv: [stored.uv[0], -0.],
        ..stored
    };
    let signed = base
        .restore_point_on_face(signed_zero, binding.clone(), || false)
        .unwrap();
    assert_eq!(signed.relation().uv[1].to_bits(), (-0.0_f64).to_bits());
    let incompatible = martensite_tksketch3d::constraints::PlanarFaceRelation {
        uv: [0., 0.],
        ..stored
    };
    assert!(
        base.restore_point_on_face(incompatible, binding.clone(), || false)
            .is_err()
    );
    assert!(
        base.attach_point_on_face(rid(40), pid(1), binding, || false)
            .is_err()
    );
    let new = proof(2, 2., 71);
    let new_source = new.revision();
    assert!(matches!(
        base.restore_point_on_face(stored, new.clone(), || false),
        Err(Failure::StaleSource { expected, current }) if expected == source && current == new_source
    ));
    assert!(matches!(
        original.rebind(new.clone(), || true),
        Err(Failure::Cancelled)
    ));
    let rebound = original.rebind(new, || false).unwrap();
    assert_eq!(rebound.relation().id, stored.id);
    assert_eq!(rebound.relation().source, new_source);
    assert_eq!(rebound.geometry(), original.geometry());
    assert!(matches!(
        rebound.drag_preferred(pid(1), point([2., 0., 0.]), 1e-8, || new_source, || false),
        Err(Failure::Trim(PlanarAttachmentLocation::HoleInterior))
    ));
    let detached = rebound.detach(|| false).unwrap();
    let loose = detached
        .drag_preferred(pid(1), point([0., 0., 1.]), 1e-8, || false)
        .unwrap();
    assert!(
        loose
            .update
            .sketch
            .geometry()
            .point(pid(1))
            .unwrap()
            .position
            .z()
            > 0.9
    );
    assert_eq!(location(&original), [-2., 0., 0.]);
}

#[test]
fn late_cancellation_and_late_input_salt_changes_never_publish() {
    let binding = proof(1, 0., 70);
    let source = binding.revision();
    let sketch = attach(&ordinary(false), binding);
    let target = point([-2., 2., 0.]);
    let mut count = 0;
    sketch
        .drag_preferred(
            pid(1),
            target,
            1e-8,
            || source,
            || {
                count += 1;
                false
            },
        )
        .unwrap();
    let mut observed = 0;
    assert!(matches!(
        sketch.drag_preferred(
            pid(1),
            target,
            1e-8,
            || source,
            || {
                observed += 1;
                observed == count
            }
        ),
        Err(Failure::Cancelled)
    ));
    let revised = PlanarFaceRevision {
        input_salt: key(99),
        ..source
    };
    let mut reads = 0;
    assert!(matches!(
        sketch.drag_preferred(pid(1), target, 1e-8, || {
            reads += 1;
            if reads == 1 { source } else { revised }
        }, || false),
        Err(Failure::StaleSource { expected, current }) if expected == source && current == revised
    ));
    assert_eq!(reads, 2);
    assert_eq!(location(&sketch), [-2., 0., 0.]);
    assert_eq!(sketch.relation().uv, [-2., 0.]);
}
