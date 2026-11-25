// SPDX-License-Identifier: GPL-2.0-only
//! Original located cap of a native square tube, with an independent square oracle.
use martensite_tkbrep::{
    brep::{
        BRepBuilder, BRepCurve3D, BRepTool,
        access::edge_tshape,
        face_input_capture::{FaceCaptureIncomplete, FaceCaptureLimits},
        planar_attachment::{
            AttachmentSource, FaceAttachment, FaceAttachmentError as Failure,
            PlanarAttachmentError, PlanarAttachmentLocation as Class, capture_planar_attachment,
        },
    },
    topexp::Explorer,
    topods::{self, Compound, Face, ShapeIterator, Shell, Solid, Wire},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    gp::{
        Axis1, Axis2d, Direction2, Direction3, Plane3, Point2, Point3, Transform3, TransformForm,
        UnitPlacementError, Vector3,
    },
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    },
    brep_check::BRepCheckAnalyzer,
};
use std::num::NonZeroU64;

#[test]
fn planar_attachment_values_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<AttachmentSource>();
    shared::<FaceAttachment>();
    shared::<Failure>();
}

const SOURCE: AttachmentSource = AttachmentSource {
    document: [11; 16],
    body: NonZeroU64::new(20).unwrap(),
    face: NonZeroU64::new(24).unwrap(),
    incarnation: 5,
    revision: 19,
};
const ORIGIN: [f64; 3] = [1_048_576., -262_144., 64.];

fn square(center: [f64; 2], radius: f64, reversed: bool) -> Wire {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    let mut corners = [[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]];
    if reversed {
        corners.reverse();
    }
    for [x, y] in corners {
        polygon
            .add_point(Point3::new(
                center[0] + x * radius,
                center[1] + y * radius,
                0.,
            ))
            .unwrap();
    }
    polygon.close().unwrap();
    polygon.wire().unwrap()
}

fn native_tube(hole_center: [f64; 2]) -> Solid {
    let mut face = BRepBuilderAPIMakeFace::from_plane_wire(
        Plane3::default(),
        &square([0., 0.], 4., false),
        true,
    )
    .unwrap();
    face.add(&square(hole_center, 1., true)).unwrap();
    let mut prism =
        BRepPrimAPIMakePrism::new(&face.face().unwrap(), Vector3::new(0., 0., 2.), false, true)
            .unwrap();
    topods::solid(prism.shape().unwrap()).unwrap()
}

fn translated(solid: &mut Solid, delta: [f64; 3]) {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(delta[0], delta[1], delta[2]));
    solid
        .set_location(&Location::new(transform), false)
        .unwrap();
}

fn base(solid: &Solid, z: f64) -> Face {
    Explorer::new(solid, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .map(|item| topods::face(&item).unwrap())
        .find(|face| {
            face.nb_children() == 2
                && BRepTool::surface_with_location(face)
                    .unwrap()
                    .1
                    .transformation()
                    .value(3, 4)
                    .unwrap()
                    == z
        })
        .unwrap()
}

fn fixture() -> (Solid, Face) {
    let mut solid = native_tube([0., 0.]);
    assert!(
        BRepCheckAnalyzer::new(&solid, true, false, true)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    translated(&mut solid, ORIGIN);
    let cap = base(&solid, ORIGIN[2]);
    (solid, cap)
}

fn capture(solid: &Solid, face: &Face) -> FaceAttachment {
    capture_planar_attachment(solid, face, SOURCE, FaceCaptureLimits::bounded(), || false).unwrap()
}

#[test]
fn native_located_hole_boundary_is_closed_without_absorbing_adjacent_floats() {
    for origin in [[0.; 3], ORIGIN] {
        let mut solid = native_tube([0., 0.]);
        if origin != [0.; 3] {
            translated(&mut solid, origin);
        }
        let proof = capture(&solid, &base(&solid, origin[2]));
        assert_eq!(proof.source(), SOURCE);
        assert_closed_boundary(&proof, origin);
    }
}

fn assert_closed_boundary(proof: &FaceAttachment, origin: [f64; 3]) {
    for (lifted_x, class) in [
        (-5., Class::OutsideOuter),
        (-4., Class::OuterBoundary),
        (-2., Class::Material),
        (-1., Class::HoleBoundary),
        (0., Class::HoleInterior),
        (1., Class::HoleBoundary),
        (2., Class::Material),
        (4., Class::OuterBoundary),
        (5., Class::OutsideOuter),
    ] {
        let point = [origin[0] + lifted_x, origin[1], origin[2]];
        let query = proof.classify(SOURCE, point, 0., || false).unwrap();
        assert_eq!(query.location, class);
        assert_eq!(query.uv, [[lifted_x; 2], [0.; 2]]);
        assert_eq!(query.projected, point.map(|scalar| [scalar; 2]));
        assert_eq!(query.distance_bound, 0.);
    }
    let boundary = origin[0] - 1.;
    for (world_x, class) in [
        (boundary.next_down(), Class::Material),
        (boundary.next_up(), Class::HoleInterior),
    ] {
        let point = [world_x, origin[1], origin[2]];
        assert_eq!(
            proof
                .classify(SOURCE, point, 0., || false)
                .unwrap()
                .location,
            class
        );
    }
}

#[test]
fn source_identity_projection_limit_and_cancellation_are_independent() {
    let (mut solid, cap) = fixture();
    let proof = capture(&solid, &cap);
    let above = [ORIGIN[0] + 2., ORIGIN[1], ORIGIN[2] + 0.25];
    assert_eq!(
        proof
            .classify(SOURCE, above, 0.25, || false)
            .unwrap()
            .distance_bound,
        0.25
    );
    assert!(matches!(
        proof.classify(SOURCE, above, 0.25_f64.next_down(), || false),
        Err(Failure::Numeric(PlanarAttachmentError::OffSupport {
            bound: Some(0.25),
            ..
        }))
    ));
    let current = AttachmentSource {
        revision: SOURCE.revision + 1,
        ..SOURCE
    };
    assert!(matches!(
        proof.classify(current, above, 1., || false),
        Err(Failure::StaleSource { captured: SOURCE, current: explicit }) if explicit == current
    ));
    assert_eq!(
        proof.classify(SOURCE, above, 1., || true),
        Err(Failure::Cancelled)
    );
    assert!(matches!(
        capture_planar_attachment(&solid, &cap, SOURCE, FaceCaptureLimits::bounded(), || true),
        Err(Failure::Cancelled)
    ));
    translated(&mut solid, [0., 0., 100.]);
    assert!(matches!(
        capture_planar_attachment(&solid, &cap, current, FaceCaptureLimits::bounded(), || {
            false
        }),
        Err(Failure::FaceOccurrence)
    ));
    assert_eq!(
        proof
            .classify(SOURCE, above, 1., || false)
            .unwrap()
            .location,
        Class::Material
    );
}

#[test]
fn a_nominal_solid_wrapper_cannot_admit_a_compound_root() {
    let (solid, cap) = fixture();
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    builder.add(&mut compound, &solid).unwrap();
    let counterfeit = Solid::unchecked(compound.into_shape());
    assert!(matches!(
        capture_planar_attachment(
            &counterfeit,
            &cap,
            SOURCE,
            FaceCaptureLimits::bounded(),
            || false
        ),
        Err(Failure::BodyKind(ShapeEnum::Compound))
    ));
}

#[test]
fn an_actual_rotated_translated_nonunit_scale_is_refused_before_projecting() {
    let solid = native_tube([0., 0.]);
    let scale = 1.1_f64;
    let axis = Axis1::new(Point3::default(), Direction3::Z);
    let mut transform = Transform3::rotation(axis, 0.3).unwrap();
    transform.set_scale_factor(scale).unwrap();
    transform.set_translation_part(Vector3::new(7., 11., 13.));
    let moved = solid.moved(&Location::new(transform), false).unwrap();
    let body = topods::solid(&moved).unwrap();
    let cap = base(&body, 13.);
    assert!(matches!(
        capture_planar_attachment(&body, &cap, SOURCE, FaceCaptureLimits::bounded(), || false),
        Err(Failure::Placement(UnitPlacementError::Scale(actual))) if actual.to_bits() == scale.to_bits()
    ));
}

#[test]
fn native_translation_form_cannot_hide_a_nonidentity_rotation_matrix() {
    let solid = native_tube([0., 0.]);
    let axis = Axis1::new(Point3::default(), Direction3::Z);
    let mut transform = Transform3::rotation(axis, 0.3).unwrap();
    transform.set_form(TransformForm::Translation);
    let moved = solid.moved(&Location::new(transform), false).unwrap();
    let body = topods::solid(&moved).unwrap();
    let cap = base(&body, 0.);
    assert!(matches!(
        capture_planar_attachment(&body, &cap, SOURCE, FaceCaptureLimits::bounded(), || false),
        Err(Failure::Placement(UnitPlacementError::Shortcut(
            TransformForm::Translation
        )))
    ));
}

#[test]
fn membership_counts_empty_shells_and_polls_before_visiting_each_one() {
    let (source, cap) = fixture();
    let builder = BRepBuilder::default();
    let mut body = Solid::default();
    builder.make_solid(&mut body);
    for _ in 0..5 {
        let mut empty = Shell::default();
        builder.make_shell(&mut empty);
        builder.add(&mut body, &empty).unwrap();
    }
    let shell = ShapeIterator::new(&source, true, true)
        .value()
        .unwrap()
        .clone();
    builder.add(&mut body, &shell).unwrap();
    let limits = FaceCaptureLimits {
        owners: 3,
        ..FaceCaptureLimits::bounded()
    };
    assert!(matches!(
        capture_planar_attachment(&body, &cap, SOURCE, limits, || false),
        Err(Failure::Capture(FaceCaptureIncomplete::Limit {
            actual: 4,
            maximum: 3,
            dimension: "source body descendants",
        }))
    ));
    let mut calls = 0;
    assert!(matches!(
        capture_planar_attachment(&body, &cap, SOURCE, FaceCaptureLimits::bounded(), || {
            calls += 1;
            calls == 4
        }),
        Err(Failure::Cancelled)
    ));
}

#[test]
fn last_capture_checkpoint_refuses_to_publish_a_complete_proof() {
    let (solid, cap) = fixture();
    let limits = FaceCaptureLimits::bounded();
    let mut checkpoints = 0;
    capture_planar_attachment(&solid, &cap, SOURCE, limits, || {
        checkpoints += 1;
        false
    })
    .unwrap();
    let mut retry = 0;
    assert!(matches!(
        capture_planar_attachment(&solid, &cap, SOURCE, limits, || {
            retry += 1;
            retry == checkpoints
        }),
        Err(Failure::Cancelled)
    ));
}

#[test]
fn null_first_spatial_carrier_does_not_fall_through_to_another_representation() {
    let (solid, cap) = fixture();
    let occurrence = Explorer::new(&cap, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = topods::edge(occurrence.current()).unwrap();
    let empty = Handle::new(BRepCurve3D::new(&Handle::default(), &Location::default()));
    edge_tshape(&edge, "attachment test edge")
        .unwrap()
        .change_curves(|curves| {
            curves.insert(0, empty.cast());
        });
    assert!(
        BRepTool::curve_with_location(&edge)
            .unwrap()
            .curve
            .is_null()
    );
    assert!(matches!(
        capture_planar_attachment(&solid, &cap, SOURCE, FaceCaptureLimits::bounded(), || false),
        Err(Failure::Representation(_))
    ));
}

#[test]
fn wrong_stored_planar_pcurve_cannot_be_replaced_by_a_valid_spatial_line() {
    let (solid, cap) = fixture();
    let original = capture(&solid, &cap);
    let occurrence = Explorer::new(&cap, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    let edge = topods::edge(occurrence.current()).unwrap();
    let corrupt = Handle::new(Geom2dLine::new(Axis2d::new(
        Point2::new(0., 20.),
        Direction2::X,
    )))
    .into_upcast();
    BRepBuilder::default()
        .update_edge_pcurve_on_face(&edge, &corrupt, &cap, 1e-7)
        .unwrap();
    assert!(BRepTool::curve_on_face(&edge, &cap).unwrap().is_stored);
    assert!(matches!(
        capture_planar_attachment(&solid, &cap, SOURCE, FaceCaptureLimits::bounded(), || false),
        Err(Failure::Numeric(PlanarAttachmentError::Edge(_)))
    ));
    let query = original
        .classify(SOURCE, [ORIGIN[0], ORIGIN[1], ORIGIN[2]], 0., || false)
        .unwrap();
    assert_eq!(query.location, Class::HoleInterior);
}

#[test]
fn mutated_actual_vertex_cannot_change_the_frozen_trim_or_seed_a_new_proof() {
    let (solid, cap) = fixture();
    let remembered = capture(&solid, &cap);
    let point = [ORIGIN[0] - 1., ORIGIN[1], ORIGIN[2]];
    let native = Explorer::new(&cap, ShapeEnum::Vertex, ShapeEnum::Shape).unwrap();
    let mut vertex = topods::vertex(native.current()).unwrap();
    vertex.set_location(&Location::default(), false).unwrap();
    BRepBuilder::default()
        .set_vertex_point_and_tolerance(&vertex, Point3::new(99., 99., 99.), 1e-7)
        .unwrap();
    assert_eq!(
        remembered
            .classify(SOURCE, point, 0., || false)
            .unwrap()
            .location,
        Class::HoleBoundary
    );
    assert!(matches!(
        capture_planar_attachment(&solid, &cap, SOURCE, FaceCaptureLimits::bounded(), || false),
        Err(Failure::Numeric(PlanarAttachmentError::Edge(_)))
    ));
}

#[test]
fn detached_classification_is_identical_across_scoped_workers() {
    let (solid, cap) = fixture();
    let proof = capture(&solid, &cap);
    let points =
        [-5., -4., -2., -1., 0., 1., 2., 4., 5.].map(|x| [ORIGIN[0] + x, ORIGIN[1], ORIGIN[2]]);
    let classify = |point| proof.classify(SOURCE, point, 0., || false).unwrap();
    let serial = points.map(classify);
    let parallel = std::thread::scope(|scope| {
        points
            .map(|point| scope.spawn(move || classify(point)))
            .map(|worker| worker.join().unwrap())
    });
    assert_eq!(serial, parallel);
    println!("attachment_receipt={serial:?}");
}
