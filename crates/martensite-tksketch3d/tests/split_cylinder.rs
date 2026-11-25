// SPDX-License-Identifier: GPL-2.0-only
//! Actual selected BRep cylindrical wall and its immutable Sketch3D split lineage.
use crate::fixture::{lid, line, pid, seed};
use martensite_tkbrep::{
    brep::{
        BRepTool,
        cylindrical_split::{
            CylindricalFaceError as FaceError, CylindricalSplitDomain,
            capture_cylindrical_split_domain,
        },
        face_input_capture::{
            FaceCaptureBehavior, FaceCaptureLimits, capture_face_input_with_behavior,
        },
        planar_attachment::{AttachmentSource, FaceAttachmentError},
    },
    topexp::Explorer,
    topods::{self, Face, Solid},
};
use martensite_tkg3d::{geom::GeomCylindricalSurface, top_abs::ShapeEnum};
use martensite_tkmath::{
    gp::{Axis2, Direction3, Point3, StandardDirection3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::{brep_prim::BRepPrimOneAxis, brep_prim_api::BRepPrimAPIMakeCylinder};
use martensite_tksketch3d::{
    CurveId, CurveRole, Edit, EditFailure, MinimumLength, PointRole, Snapshot,
    split_cylinder::{
        CylinderContactSense as Sense, CylinderLinePlace as LinePlace,
        CylinderSegmentIntersection as Hit, CylinderWallPlace as WallPlace,
        SplitCylindricalLineEdit, SplitCylindricalLineFailure as Failure, SplitCylindricalLineIds,
        SplitCylindricalLineOutcome as Outcome,
    },
    split_plane::SplitLineFailure,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::num::NonZeroU64;

const STAMP: AttachmentSource = AttachmentSource {
    document: [17; 16],
    body: NonZeroU64::new(37).unwrap(),
    face: NonZeroU64::new(7).unwrap(),
    incarnation: 3,
    revision: 29,
};
const DIA: [[f64; 3]; 2] = [[-3., 0., 1.], [3., 0., 1.]];
const BOUND: f64 = 1e-8;

fn native(mut maker: BRepPrimAPIMakeCylinder) -> (Solid, Face) {
    let body = topods::solid(maker.shape().unwrap()).unwrap();
    let original = maker.cylinder().lateral_face().unwrap();
    let face = actual_face(&body, &original);
    (body, face)
}

fn actual_face(body: &Solid, original: &Face) -> Face {
    let occurrence = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .find(|face| face.is_partner(original))
        .unwrap();
    topods::face(&occurrence).unwrap()
}

fn capture(body: &Solid, face: &Face) -> CylindricalSplitDomain {
    capture_cylindrical_split_domain(
        body,
        face,
        STAMP,
        FaceCaptureLimits::bounded(),
        BOUND,
        || false,
    )
    .unwrap()
}

fn wall() -> CylindricalSplitDomain {
    let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
    capture(&body, &face)
}

fn source([first, last]: [[f64; 3]; 2]) -> Snapshot {
    Snapshot::empty(MinimumLength::new(1e-9).unwrap())
        .apply(
            &[
                seed(1, first, PointRole::Defining),
                seed(2, last, PointRole::Defining),
                seed(80, [7., 9., 10.], PointRole::Defining),
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

fn ids() -> SplitCylindricalLineIds {
    SplitCylindricalLineIds {
        points: [pid(7), pid(4)],
        fragments: [lid(9), lid(6), lid(5)],
    }
}

fn query(source: &Snapshot, wall: &CylindricalSplitDomain) -> Result<Outcome, Failure> {
    source.split_line_at_cylindrical_face(lid(3), wall, STAMP, ids(), BOUND, || false)
}

fn split(source: &Snapshot, wall: &CylindricalSplitDomain) -> Box<SplitCylindricalLineEdit> {
    let Outcome::Split(edit) = query(source, wall).unwrap() else {
        panic!("expected a completed cylindrical split");
    };
    edit
}

#[test]
fn owned_diameter_splits_both_directions_into_length_two_fragments() {
    let wall = wall();
    let one_third = 1_f64 / 3.;
    let two_thirds = 2_f64 / 3.;
    let intervals = [
        [0.; 2],
        [one_third, one_third.next_up()],
        [two_thirds, two_thirds.next_up()],
        [1.; 2],
    ];
    for (points, sign) in [(DIA, 1.), ([DIA[1], DIA[0]], -1.)] {
        let input = source(points);
        let edit = split(&input, &wall);
        assert_eq!(edit.report.face, STAMP);
        assert_eq!(edit.report.trim_trace.requested_limit, BOUND);
        assert!(edit.report.trim_trace.upper_bound < 1e-14);
        assert_eq!(edit.report.source, lid(3));
        for (index, cut) in edit.cuts.iter().enumerate() {
            assert_eq!(cut.point, ids().points[index]);
            assert_eq!(usize::from(cut.root_ordinal), index);
            assert_eq!(cut.location, WallPlace::Interior);
            assert_eq!(
                cut.position.position,
                [[-sign, 0., 1.], [sign, 0., 1.]][index]
            );
            assert_eq!(cut.position.error_upper, 0.);
            assert_eq!(cut.position.parameter, intervals[index + 1]);
        }
        let chain = [pid(1), ids().points[0], ids().points[1], pid(2)];
        assert_eq!(edit.cuts.len(), 2);
        assert_eq!(edit.fragments.len(), 3);
        for (index, fragment) in edit.fragments.iter().enumerate() {
            assert_eq!(fragment.source, lid(3));
            assert_eq!(fragment.line.id, ids().fragments[index]);
            assert_eq!(fragment.line.endpoints, [chain[index], chain[index + 1]]);
            assert_eq!(
                fragment.source_parameters,
                [intervals[index], intervals[index + 1]]
            );
            assert_eq!(
                edit.snapshot
                    .line(fragment.line.id)
                    .unwrap()
                    .length_estimate,
                2.
            );
            assert_eq!(
                edit.snapshot.curve_role(CurveId::Line(fragment.line.id)),
                Some(CurveRole::Construction)
            );
        }
        assert_eq!(edit.snapshot.point(pid(80)), input.point(pid(80)));
        assert!(edit.snapshot.line(lid(3)).is_none() && input.line(lid(3)).is_some());
    }
}

#[test]
fn tangency_nearest_discriminants_and_both_closed_rims() {
    let wall = wall();
    for (y, expected) in [
        (1.0_f64.next_down(), true),
        (1., false),
        (1.0_f64.next_up(), false),
    ] {
        let input = source([[-3., y, 1.], [3., y, 1.]]);
        match query(&input, &wall).unwrap() {
            Outcome::Split(edit) => {
                assert!(expected);
                assert_eq!(edit.cuts.len(), 2);
                let ideal = (1. - y * y).sqrt();
                assert!((edit.cuts[1].position.position[0] - ideal).abs() < 1e-15);
            }
            Outcome::Unchanged(report) => {
                assert!(!expected);
                if y == 1. {
                    let Hit::Isolated(contacts) = report.contact else {
                        panic!("expected true tangency");
                    };
                    assert_eq!(contacts.len(), 1);
                    assert_eq!(contacts[0].sense, Sense::Tangent);
                    assert_eq!(contacts[0].wall, WallPlace::Interior);
                } else {
                    assert_eq!(report.contact, Hit::Disjoint);
                }
            }
        }
    }
    for (z, expected) in [(0., WallPlace::FirstRim), (2., WallPlace::LastRim)] {
        let input = source([[-3., 0., z], [3., 0., z]]);
        assert!(
            split(&input, &wall)
                .cuts
                .iter()
                .all(|cut| cut.location == expected)
        );
    }
    for (z, expected) in [
        (-f64::EPSILON, WallPlace::Below),
        (2.0_f64.next_up(), WallPlace::Above),
    ] {
        let input = source([[-3., 0., z], [3., 0., z]]);
        let Outcome::Unchanged(report) = query(&input, &wall).unwrap() else {
            panic!("expected exact trim miss");
        };
        let Hit::Isolated(contacts) = report.contact else {
            panic!("expected both infinite-support roots");
        };
        assert!(
            contacts
                .iter()
                .all(|contact| contact.wall == expected && contact.cut.is_none())
        );
    }
}

#[test]
fn exact_algebraic_axial_side_is_checked_before_rounding() {
    let literal = 1. / 3.;
    let input = source([[-3., 0., 0.], [3., 0., 1.]]);
    let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., literal));
    let Outcome::Unchanged(report) = query(&input, &capture(&body, &face)).unwrap() else {
        panic!("the exact first crossing is above its rounded axial height");
    };
    let Hit::Isolated(contacts) = report.contact else {
        panic!("expected support roots");
    };
    assert_eq!(contacts[0].wall, WallPlace::Above);

    let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., literal.next_up()));
    let edit = split(&input, &capture(&body, &face));
    assert_eq!(edit.cuts.len(), 1);
    assert_eq!(edit.cuts[0].position.position, [-1., 0., literal]);
    assert_eq!(
        edit.cuts[0].position.parameter,
        [literal, literal.next_up()]
    );
    assert!(edit.cuts[0].position.error_upper > 0.);
    assert!(matches!(
        input.split_line_at_cylindrical_face(lid(3), &capture(&body, &face), STAMP, ids(), 0., || false),
        Err(Failure::Line(SplitLineFailure::PositionError { upper_bound, limit: 0. })) if upper_bound > 0.
    ));
    assert_eq!(edit.cuts[0].root_ordinal, 0);
    let skipped = source([[-3., 0., 1.], [3., 0., 0.]]);
    let edit = split(&skipped, &capture(&body, &face));
    assert_eq!(edit.cuts.len(), 1);
    assert_eq!(edit.cuts[0].root_ordinal, 1);
    assert_eq!(edit.cuts[0].point, ids().points[0]);
}

#[test]
fn endpoint_and_trimmed_generator_are_explicit_outcomes() {
    let wall = wall();
    let input = source([[-1., 0., 1.], [3., 0., 1.]]);
    let edit = split(&input, &wall);
    let Hit::Isolated(contacts) = &edit.report.contact else {
        panic!("expected existing and interior roots");
    };
    assert_eq!(contacts[0].line, LinePlace::Start);
    assert_eq!(edit.cuts[0].root_ordinal, 1);
    assert_eq!(edit.fragments[0].line.endpoints[0], pid(1));
    assert!(edit.snapshot.point(ids().points[1]).is_none());

    let touches = source([[1., 0., -1.], [1., 0., 0.]]);
    let Outcome::Unchanged(report) = query(&touches, &wall).unwrap() else {
        panic!("expected existing rim endpoint");
    };
    assert_eq!(
        report.contact,
        Hit::GeneratorEndpoint {
            line: LinePlace::End,
            rim: WallPlace::FirstRim
        }
    );
    let overlaps = source([[1., 0., -1.], [1., 0., 2.]]);
    assert_eq!(
        query(&overlaps, &wall),
        Err(Failure::Overlap {
            line: lid(3),
            face: STAMP
        })
    );
}

#[test]
fn seam_relocation_and_actual_occurrence_placement_preserve_signed_lineage() {
    let direction = |axis| Direction3::from_standard(axis);
    let ax2 = Axis2::with_x_direction(
        Point3::default(),
        direction(StandardDirection3::Z),
        direction(StandardDirection3::Y),
    )
    .unwrap();
    let (mut body, face) = native(BRepPrimAPIMakeCylinder::with_axes(ax2, 1., 2.));
    let local = split(&source(DIA), &capture(&body, &face));
    let standard = split(&source(DIA), &wall());
    assert_eq!(local.cuts, standard.cuts);
    assert_eq!(local.fragments, standard.fragments);
    let mut shift = Transform3::default();
    shift.set_translation(Vector3::new(8., -5., 2.));
    body.set_location(&Location::new(shift), false).unwrap();
    let moved = actual_face(&body, &face);
    let input = source([[5., -5., 3.], [11., -5., 3.]]);
    let moved_edit = split(&input, &capture(&body, &moved));
    assert_eq!(moved_edit.fragments, standard.fragments);
    assert_eq!(moved_edit.cuts[0].position.position, [7., -5., 3.]);
    assert_eq!(moved_edit.cuts[1].position.position, [9., -5., 3.]);
}

#[test]
fn frozen_wall_keeps_actual_capture_after_native_carrier_mutation() {
    let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
    let frozen = capture(&body, &face);
    let input = source(DIA);
    let original = query(&input, &frozen).unwrap();
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    let surface = surface.downcast::<GeomCylindricalSurface>();
    surface.get().unwrap().set_radius(1.25).unwrap();
    assert_eq!(query(&input, &frozen).unwrap(), original);
    assert!(matches!(
        capture_cylindrical_split_domain(&body, &face, STAMP, FaceCaptureLimits::bounded(), BOUND, || false),
        Err(FaceError::TraceError { upper_bound, limit: BOUND }) if upper_bound >= 0.25
    ));
    let updated = AttachmentSource {
        revision: STAMP.revision + 1,
        ..STAMP
    };
    assert_eq!(
        input.split_line_at_cylindrical_face(lid(3), &frozen, updated, ids(), BOUND, || false),
        Err(Failure::Face(FaceError::Source(
            FaceAttachmentError::StaleSource {
                captured: STAMP,
                current: updated
            }
        )))
    );
}

#[test]
fn partial_and_unrelated_faces_refuse_and_prior_capture_domains_remain_explicit() {
    let (full, selected) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
    let (other, unrelated) = native(BRepPrimAPIMakeCylinder::with_angle(1., 2., 1.).unwrap());
    let limits = FaceCaptureLimits::bounded();
    assert!(matches!(
        capture_cylindrical_split_domain(&full, &unrelated, STAMP, limits, BOUND, || false),
        Err(FaceError::Source(FaceAttachmentError::FaceOccurrence))
    ));
    assert!(matches!(
        capture_cylindrical_split_domain(&other, &unrelated, STAMP, limits, BOUND, || false),
        Err(FaceError::UnsupportedTrim { .. })
    ));
    for behavior in [
        FaceCaptureBehavior::StoredRepresentationsV1,
        FaceCaptureBehavior::StoredSurfaceQueriesV2,
    ] {
        assert!(capture_face_input_with_behavior(&selected, limits, behavior, || false).is_err());
    }
}

#[test]
fn output_conflict_and_cancellation_refuse_the_entire_batch() {
    let wall = wall();
    let input = source(DIA);
    let invalid = SplitCylindricalLineIds {
        points: [pid(7), pid(7)],
        ..ids()
    };
    assert!(matches!(
        input.split_line_at_cylindrical_face(lid(3), &wall, STAMP, invalid, BOUND, || false),
        Err(Failure::Line(SplitLineFailure::Edit(
            EditFailure::DuplicateIdentity { .. }
        )))
    ));
    let mut total = 0;
    input
        .split_line_at_cylindrical_face(lid(3), &wall, STAMP, ids(), BOUND, || {
            total += 1;
            false
        })
        .unwrap();
    for cutoff in [1, 5, total - 2, total] {
        let mut reached = 0;
        assert_eq!(
            input.split_line_at_cylindrical_face(lid(3), &wall, STAMP, ids(), BOUND, || {
                reached += 1;
                reached >= cutoff
            }),
            Err(Failure::Line(SplitLineFailure::Cancelled))
        );
    }
    assert_eq!(input, source(DIA));
}

#[test]
fn frozen_wall_and_split_are_send_sync_and_deterministic_in_parallel() {
    fn shared<T: Send + Sync>() {}
    shared::<CylindricalSplitDomain>();
    shared::<Outcome>();
    shared::<Failure>();
    let wall = wall();
    let input = source(DIA);
    let expected = query(&input, &wall).unwrap();
    std::thread::scope(|scope| {
        let workers = (0..4).map(|_| {
            scope.spawn(|| {
                for _ in 0..4 {
                    assert_eq!(query(&input, &wall).unwrap(), expected);
                }
            })
        });
        for worker in workers.collect::<Vec<_>>() {
            worker.join().unwrap();
        }
    });
}

#[test]
fn fresh_processes_reproduce_cylindrical_cut_ranges_and_directed_lineage() {
    let run = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "split_cylinder::emit_cylindrical_split",
                "--nocapture",
            ])
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
            .find(|line| line.starts_with("cylindrical-receipt "))
            .unwrap()
            .to_string()
    };
    assert_eq!(run(), run());
}

#[test]
fn emit_cylindrical_split() {
    let edit = split(&source(DIA), &wall());
    println!(
        "cylindrical-receipt {:?} {:?} {:?}",
        edit.report, edit.cuts, edit.fragments
    );
}

mod native_trim;
