// SPDX-License-Identifier: GPL-2.0-only
//! Split a directed Sketch3D line with an actual selected cap of a native prism or tube.
use crate::fixture::{lid, line, pid, seed};
use martensite_tkbrep::{
    brep::{
        face_input_capture::FaceCaptureLimits,
        planar_attachment::{
            AttachmentSource, FaceAttachment, FaceAttachmentError,
            PlanarAttachmentLocation as Class, capture_planar_attachment,
            capture_planar_split_domain,
        },
    },
    topexp::Explorer,
    topods::{self, Face, Solid, Wire},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    gp::{Axis3, Plane3, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeSweep};
use martensite_tksketch3d::{
    CurveId, CurveRole, Edit, EditFailure, MinimumLength, PointRole, Snapshot,
    split_face::{
        SplitPlanarFaceFailure as Failure, SplitPlanarFaceOutcome as Outcome,
        UnchangedPlanarFaceLine as No,
    },
    split_plane::{SplitLineEdit, SplitLineFailure, SplitLineIds},
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};
use std::num::NonZeroU64;

const STAMP: AttachmentSource = AttachmentSource {
    document: [23; 16],
    body: NonZeroU64::new(31).unwrap(),
    face: NonZeroU64::new(17).unwrap(),
    incarnation: 9,
    revision: 11,
};
const RECTANGLE: [[f64; 2]; 2] = [[-4., 4.], [-4., 4.]];
const HOLE: [[f64; 2]; 2] = [[-1., 1.], [-1., 1.]];

fn polygon([[a, b], [c, d]]: [[f64; 2]; 2], reverse: bool) -> Wire {
    let mut yz = [[a, c], [b, c], [b, d], [a, d]];
    if reverse {
        yz.reverse();
    }
    let mut builder = BRepBuilderAPIMakePolygon::new();
    for [y, z] in yz {
        builder.add_point(Point3::new(0., y, z)).unwrap();
    }
    builder.close().unwrap();
    builder.wire().unwrap()
}

fn native_wall(bounds: [[f64; 2]; 2], hole: Option<[[f64; 2]; 2]>) -> (Solid, Face) {
    let frame = Axis3::from_stored_coefficients(
        Point3::default(),
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
    )
    .unwrap();
    let mut face = BRepBuilderAPIMakeFace::from_plane_wire(
        Plane3::from_position(frame),
        &polygon(bounds, false),
        true,
    )
    .unwrap();
    if let Some(hole) = hole {
        face.add(&polygon(hole, true)).unwrap();
    }
    let mut prism =
        BRepPrimAPIMakePrism::new(&face.face().unwrap(), Vector3::new(2., 0., 0.), false, true)
            .unwrap();
    let first = prism.first_shape().unwrap();
    let solid = topods::solid(prism.shape().unwrap()).unwrap();
    let exact = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .find(|face| face.is_same(&first))
        .unwrap();
    (solid, topods::face(&exact).unwrap())
}

fn proof(bounds: [[f64; 2]; 2], hole: Option<[[f64; 2]; 2]>) -> FaceAttachment {
    let (body, face) = native_wall(bounds, hole);
    capture(&body, &face)
}

fn capture(body: &Solid, face: &Face) -> FaceAttachment {
    capture_planar_split_domain(body, face, STAMP, FaceCaptureLimits::bounded(), || false).unwrap()
}

fn source(points: [[f64; 3]; 2]) -> Snapshot {
    Snapshot::empty(MinimumLength::new(1e-9).unwrap())
        .apply(
            &[
                seed(1, points[0], PointRole::Defining),
                seed(2, points[1], PointRole::Defining),
                seed(80, [8., 9., 10.], PointRole::Defining),
                line(3, 1, 2),
                Edit::SetCurveRole {
                    curve: CurveId::Line(lid(3)),
                    role: CurveRole::Construction,
                },
            ],
            || false,
        )
        .unwrap()
}

fn ids() -> SplitLineIds {
    SplitLineIds {
        cut: pid(4),
        fragments: [lid(7), lid(5)],
    }
}

fn split(source: &Snapshot, domain: &FaceAttachment) -> Result<Outcome, Failure> {
    source.split_line_at_planar_face(lid(3), domain, STAMP, ids(), 1e-16, || false)
}

fn done(result: Outcome, class: Class) -> Box<SplitLineEdit> {
    let Outcome::Split {
        edit,
        face,
        location,
    } = result
    else {
        panic!("expected an atomic selected-face split");
    };
    assert_eq!(face, STAMP);
    assert_eq!(location, class);
    edit
}

fn crossing(y: f64, z: f64) -> [[f64; 3]; 2] {
    [[-2., y, z], [2., y, z]]
}

#[test]
fn owned_x_zero_fixture_splits_both_source_directions_into_two_length_two_halves() {
    let face = proof(RECTANGLE, None);
    for points in [crossing(0., 0.), [[2., 0., 0.], [-2., 0., 0.]]] {
        let input = source(points);
        let out = done(split(&input, &face).unwrap(), Class::Material);
        assert_eq!(out.cut.position, [0.; 3]);
        assert_eq!(out.cut.error_upper, 0.);
        assert_eq!(
            out.fragments.map(|fragment| fragment.line.id),
            ids().fragments
        );
        assert_eq!(out.fragments.map(|fragment| fragment.source), [lid(3); 2]);
        assert_eq!(
            out.fragments.map(|fragment| fragment.line.endpoints),
            [[pid(1), pid(4)], [pid(4), pid(2)]]
        );
        assert_eq!(
            out.fragments.map(|fragment| fragment.source_parameters),
            [[[0.; 2], [0.5; 2]], [[0.5; 2], [1.; 2]]]
        );
        assert!(input.point(pid(80)).is_some());
        assert_eq!(out.snapshot.point(pid(80)), input.point(pid(80)));
        assert!(out.snapshot.line(lid(3)).is_none() && input.line(lid(3)).is_some());
        for (i, fragment) in out.fragments.iter().enumerate() {
            let line = out.snapshot.line(fragment.line.id).unwrap();
            assert_eq!(line.length_estimate, 2.);
            assert_eq!(*line.points[1 - i].coordinates(), [0.; 3]);
            assert_eq!(*line.points[i].coordinates(), points[i]);
            assert_eq!(
                out.snapshot.curve_role(CurveId::Line(fragment.line.id)),
                Some(CurveRole::Construction)
            );
        }
    }
}

#[test]
fn exact_outer_and_hole_rims_are_closed_for_interior_transverse_hits() {
    let face = proof(RECTANGLE, Some(HOLE));
    for (y, class) in [
        (4., Class::OuterBoundary),
        (1., Class::HoleBoundary),
        (1.0_f64.next_up(), Class::Material),
    ] {
        done(split(&source(crossing(y, 0.)), &face).unwrap(), class);
    }
    for (y, class) in [
        (4.0_f64.next_up(), Class::OutsideOuter),
        (1.0_f64.next_down(), Class::HoleInterior),
    ] {
        let input = source(crossing(y, 0.));
        let unusable = SplitLineIds {
            cut: pid(1),
            fragments: [lid(3); 2],
        };
        assert_eq!(
            input.split_line_at_planar_face(lid(3), &face, STAMP, unusable, 0., || false),
            Ok(Outcome::Unchanged(No::TrimmedOut(class)))
        );
        assert!(input.point(pid(4)).is_none());
    }
}

#[test]
fn membership_is_classified_at_exact_one_third_before_rounding_the_world_point() {
    let literal = 1. / 3.;
    let input = source([[-1., 0., 0.], [2., 1., 0.]]);
    let excluded = proof([[0., literal], [-1., 1.]], None);
    assert_eq!(
        split(&input, &excluded),
        Ok(Outcome::Unchanged(No::TrimmedOut(Class::OutsideOuter)))
    );
    let included = proof([[0., literal.next_up()], [-1., 1.]], None);
    let accepted = done(split(&input, &included).unwrap(), Class::Material);
    assert_eq!(accepted.cut.position, [0., literal, 0.]);
    assert_eq!(accepted.cut.parameter, [literal, literal.next_up()]);
    assert!(accepted.cut.error_upper > 0.);
    assert!(
        matches!(input.split_line_at_planar_face(lid(3), &included, STAMP, ids(), 0., || false),
        Err(Failure::Line(SplitLineFailure::PositionError { upper_bound, limit: 0. })) if upper_bound > 0.)
    );
}

#[test]
fn coplanar_contact_distinguishes_hole_empty_touch_and_positive_length_overlap() {
    let face = proof(RECTANGLE, Some(HOLE));
    for (points, expected) in [
        ([[0., -0.5, 0.], [0., 0.5, 0.]], No::Disjoint),
        ([[0., -1., 0.], [0., 1., 0.]], No::CoplanarTouch),
        ([[0., 3., 5.], [0., 5., 3.]], No::CoplanarTouch),
        ([[0., 5., 0.], [0., 6., 0.]], No::Disjoint),
    ] {
        assert_eq!(
            split(&source(points), &face),
            Ok(Outcome::Unchanged(expected))
        );
    }
    for points in [[[0., -2., 0.], [0., 2., 0.]], [[0., 0., 1.], [0., 0.5, 1.]]] {
        assert_eq!(
            split(&source(points), &face),
            Err(Failure::Line(SplitLineFailure::Overlap(lid(3))))
        );
    }
    for points in [[[0., 1., 0.], [2., 1., 0.]], [[2., 1., 0.], [0., 1., 0.]]] {
        let index = usize::from(points[0][0] != 0.);
        assert_eq!(
            split(&source(points), &face),
            Ok(Outcome::Unchanged(No::Endpoint {
                point: [pid(1), pid(2)][index],
                location: Class::HoleBoundary,
            }))
        );
    }
}

#[test]
fn a_selected_face_stamp_and_frozen_capture_refuse_cross_body_and_stale_membership() {
    let (mut body, face) = native_wall(RECTANGLE, None);
    let (_, unrelated) = native_wall(RECTANGLE, None);
    assert!(matches!(
        capture_planar_split_domain(
            &body,
            &unrelated,
            STAMP,
            FaceCaptureLimits::bounded(),
            || false
        ),
        Err(FaceAttachmentError::FaceOccurrence)
    ));
    assert!(matches!(
        capture_planar_split_domain(&body, &face, STAMP, FaceCaptureLimits::bounded(), || true),
        Err(FaceAttachmentError::Cancelled)
    ));
    assert!(matches!(
        capture_planar_attachment(&body, &face, STAMP, FaceCaptureLimits::bounded(), || false),
        Err(FaceAttachmentError::Topology(_))
    ));
    let frozen = capture(&body, &face);
    let input = source(crossing(0., 0.));
    let earlier = split(&input, &frozen).unwrap();
    let mut shift = Transform3::default();
    shift.set_translation(Vector3::new(10., 0., 0.));
    body.set_location(&Location::new(shift), false).unwrap();
    let later = AttachmentSource {
        revision: STAMP.revision + 1,
        ..STAMP
    };
    assert!(matches!(
        capture_planar_split_domain(&body, &face, later, FaceCaptureLimits::bounded(), || false),
        Err(FaceAttachmentError::FaceOccurrence)
    ));
    assert_eq!(split(&input, &frozen).unwrap(), earlier);
    assert!(
        matches!(input.split_line_at_planar_face(lid(3), &frozen, later, ids(), 0., || false),
        Err(Failure::Face(FaceAttachmentError::StaleSource { captured: STAMP, current })) if current == later)
    );
}

#[test]
fn split_uses_the_actual_located_face_not_the_solid_local_plane() {
    let (mut native, original) = native_wall(RECTANGLE, None);
    let mut shift = Transform3::default();
    shift.set_translation(Vector3::new(8., -1., 2.));
    native.set_location(&Location::new(shift), false).unwrap();
    let moved = Explorer::new(&native, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .find(|item| item.is_partner(&original))
        .unwrap();
    let moved = topods::face(&moved).unwrap();
    let domain = capture(&native, &moved);
    let edit = done(
        split(&source([[6., -1., 2.], [10., -1., 2.]]), &domain).unwrap(),
        Class::Material,
    );
    assert_eq!(edit.cut.position, [8., -1., 2.]);
    assert_eq!(edit.cut.parameter, [0.5; 2]);
}

#[test]
fn rejected_ids_and_late_cancel_preserve_all_source_entities() {
    let face = proof(RECTANGLE, None);
    let input = source(crossing(0., 0.));
    let duplicate = SplitLineIds {
        cut: pid(1),
        ..ids()
    };
    assert!(matches!(
        input.split_line_at_planar_face(lid(3), &face, STAMP, duplicate, 0., || false),
        Err(Failure::Line(SplitLineFailure::Edit(
            EditFailure::DuplicateIdentity { .. }
        )))
    ));
    let mut polls = 0;
    input
        .split_line_at_planar_face(lid(3), &face, STAMP, ids(), 0., || {
            polls += 1;
            false
        })
        .unwrap();
    for limit in [1, 5, polls - 2, polls] {
        let mut reached = 0;
        assert_eq!(
            input.split_line_at_planar_face(lid(3), &face, STAMP, ids(), 0., || {
                reached += 1;
                reached >= limit
            }),
            Err(Failure::Line(SplitLineFailure::Cancelled))
        );
    }
    assert_eq!(
        input.owned_ids().collect::<Vec<_>>(),
        source(crossing(0., 0.)).owned_ids().collect::<Vec<_>>()
    );
}

#[test]
fn frozen_split_values_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<FaceAttachment>();
    shared::<Outcome>();
    shared::<Failure>();
    shared::<No>();
}

#[test]
fn fresh_processes_reproduce_outward_cut_and_directed_lineage() {
    let run = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "split_face::emit_planar_split", "--nocapture"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .find(|line| line.starts_with("FACE_SPLIT "))
            .unwrap()
            .to_owned()
    };
    assert_eq!(run(), run());
}

#[test]
fn emit_planar_split() {
    let face = proof(RECTANGLE, None);
    let input = source([[-1., 0., 0.], [2., 1., 0.]]);
    let edit = done(split(&input, &face).unwrap(), Class::Material);
    println!("FACE_SPLIT {:?} {:?}", edit.cut, edit.fragments);
}

#[test]
fn one_frozen_native_domain_replays_the_same_split_in_parallel() {
    use std::{sync::Barrier, thread};

    let face = proof(RECTANGLE, Some(HOLE));
    let input = source([[-1., 2., 0.], [2., 3.5, 0.]]);
    let serial = split(&input, &face).unwrap();
    let Outcome::Split { edit, .. } = &serial else {
        panic!("expected split")
    };
    assert_eq!(edit.cut.position, [0., 2.5, 0.]);
    assert_eq!(edit.cut.error_upper, 0.);
    let barrier = Barrier::new(4);
    thread::scope(|workers| {
        let pending = (0..4)
            .map(|_| {
                workers.spawn(|| {
                    barrier.wait();
                    split(&input, &face).unwrap()
                })
            })
            .collect::<Vec<_>>();
        for worker in pending {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}
