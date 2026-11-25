// SPDX-License-Identifier: GPL-2.0-only
//! Independent affine charts and exact planar trim classification controls.
use martensite_tkmath::{
    gp::{Axis3, Direction3, Line3, Point3},
    predicates::roots::metric_bounds::planar_attachment::{
        CertifiedPlanarAttachment, PlanarAttachmentEdge, PlanarAttachmentError,
        PlanarAttachmentLocation,
    },
};

const PLACEMENT: [[f64; 4]; 3] = [[3., 0., 1., 64.], [0., 1., 0., 0.], [0., 0., 1., 9.]];

fn square(radius: f64, reversed: bool, shift: f64) -> [PlanarAttachmentEdge; 4] {
    let mut corners = [[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]];
    if reversed {
        corners.reverse();
    }
    let corners = corners.map(|[x, y]| [x * radius + shift, y * radius, 0.]);
    std::array::from_fn(|index| {
        let [a, b] = [corners[index], corners[(index + 1) % 4]];
        let [x, y, z] = a;
        let direction = Direction3::new(b[0] - x, b[1] - y, b[2] - z).unwrap();
        PlanarAttachmentEdge {
            line: Line3::new(Point3::new(x, y, z), direction),
            range: [0., 2. * radius],
            vertices: [a, b],
            reversed: false,
            pcurve: None,
        }
    })
}

#[test]
fn exact_ratio_uv_interval_and_inclusive_world_distance() {
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    let chart =
        CertifiedPlanarAttachment::capture(Axis3::default(), PLACEMENT, [&outer, &hole], || false)
            .unwrap();
    let query = chart.classify([65., 2., 10.], 1., || false).unwrap();
    let low = 1. / 3.;
    // binary64 1/3 is strictly below the exact rational value.
    assert_eq!(query.uv, [[low, f64::next_up(low)], [2.; 2]]);
    assert_eq!(query.projected, [[65.; 2], [2.; 2], [9.; 2]]);
    assert_eq!(query.location, PlanarAttachmentLocation::Material);
    assert_eq!(query.distance_bound, 1.);
    assert!(matches!(
        chart.classify([65., 2., 10.], 1.0_f64.next_down(), || false),
        Err(PlanarAttachmentError::OffSupport {
            bound: Some(1.),
            ..
        })
    ));
}

#[test]
fn rings_need_exact_direction_trace_and_a_strictly_nested_hole() {
    let outer = square(4., false, 0.);
    let touching = square(1., true, 3.);
    let same_sense = square(1., false, 0.);
    for invalid in [&touching, &same_sense] {
        assert!(matches!(
            CertifiedPlanarAttachment::capture(
                Axis3::default(),
                PLACEMENT,
                [&outer, invalid],
                || false
            ),
            Err(PlanarAttachmentError::Hole)
        ));
    }
    let mut moved = square(1., true, 0.);
    moved[0].vertices[0][0] = (-1_f64).next_up();
    assert!(matches!(
        CertifiedPlanarAttachment::capture(Axis3::default(), PLACEMENT, [&outer, &moved], || false),
        Err(PlanarAttachmentError::Edge(0))
    ));
}

#[test]
fn cancellation_discards_both_cold_proof_and_warm_classification() {
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    let mut calls = 0;
    assert!(matches!(
        CertifiedPlanarAttachment::capture(Axis3::default(), PLACEMENT, [&outer, &hole], || {
            calls += 1;
            calls == 12
        }),
        Err(PlanarAttachmentError::Cancelled)
    ));
    let chart =
        CertifiedPlanarAttachment::capture(Axis3::default(), PLACEMENT, [&outer, &hole], || false)
            .unwrap();
    let mut calls = 0;
    assert_eq!(
        chart.classify([64., 0., 9.], 0., || {
            calls += 1;
            calls == 8
        }),
        Err(PlanarAttachmentError::Cancelled)
    );
}

#[test]
fn oblique_projection_uses_the_original_nonorthogonal_placed_chart() {
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    let map = [[2., 1., 0., 64.], [1., 3., 0., -8.], [0., 1., 2., 16.]];
    let chart =
        CertifiedPlanarAttachment::capture(Axis3::default(), map, [&outer, &hole], || false)
            .unwrap();
    // (2,1,0) × (1,3,1) = (1,-2,5). The query differs by 1/8 of this normal.
    let query = chart
        .classify([62.125, -9.25, 16.625], 1., || false)
        .unwrap();
    assert_eq!(query.location, PlanarAttachmentLocation::HoleBoundary);
    assert_eq!(query.uv, [[-1.; 2], [0.; 2]]);
    assert_eq!(query.projected, [[62.; 2], [-9.; 2], [16.; 2]]);
    assert_eq!(query.distance_bound, 0.684_653_196_881_457_6);
}

#[test]
fn inconsistent_stored_normal_does_not_admit_the_parameter_plane() {
    use martensite_tkernel::standard::dump::{DumpStream, dump_vector};
    let mut stored = DumpStream::default();
    for (field, coefficients) in [
        ("Location", [0., 0., 0.]),
        ("Direction", [1., 0., 0.]),
        ("XDirection", [1., 0., 0.]),
        ("YDirection", [0., 1., 0.]),
    ] {
        dump_vector(&mut stored, field, &coefficients).unwrap();
    }
    let mut frame = Axis3::default();
    assert!(frame.init_from_json(stored.text(), &mut 1).unwrap());
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    assert!(matches!(
        CertifiedPlanarAttachment::capture(frame, PLACEMENT, [&outer, &hole], || false),
        Err(PlanarAttachmentError::Frame)
    ));
}

#[test]
fn a_valid_indirect_chart_keeps_the_original_uv_directions() {
    let frame = Axis3::from_stored_coefficients(
        Point3::default(),
        [0., 0., -1.],
        [1., 0., 0.],
        [0., 1., 0.],
    )
    .unwrap();
    assert!(!frame.direct().unwrap());
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    let proof =
        CertifiedPlanarAttachment::capture(frame, PLACEMENT, [&outer, &hole], || false).unwrap();
    let query = proof.classify([61., 0., 9.], 0., || false).unwrap();
    assert_eq!(query.location, PlanarAttachmentLocation::HoleBoundary);
    assert_eq!(query.uv, [[-1.; 2], [0.; 2]]);
}

#[test]
fn tilted_support_admits_by_euclidean_distance_and_refuses_the_adjacent_lower_cap() {
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    let map = [[1., 0., 0., 0.], [-1., 0., 1., 0.], [0., 1., 0., 0.]];
    let chart =
        CertifiedPlanarAttachment::capture(Axis3::default(), map, [&outer, &hole], || false)
            .unwrap();
    // The nearest support point (2,-2,0) leaves the exact residual (1,1,0).
    let point = [3., -1., 0.];
    let upper = f64::from_bits(0x3ff6_a09e_667f_3bcd);
    let accepted = chart.classify(point, 1.5, || false).unwrap();
    assert_eq!(accepted.location, PlanarAttachmentLocation::Material);
    assert_eq!(accepted.projected, [[2.; 2], [-2.; 2], [0.; 2]]);
    assert_eq!(accepted.distance_bound, upper);
    assert_eq!(chart.classify(point, upper, || false).unwrap(), accepted);
    assert_eq!(
        chart.classify(point, upper.next_down(), || false),
        Err(PlanarAttachmentError::OffSupport {
            bound: Some(upper),
            limit: upper.next_down(),
        })
    );
}

#[test]
fn native_unit_placement_admits_original_rows_and_rejects_conflicting_shortcuts() {
    use martensite_tkmath::gp::{Axis1, Transform3, TransformForm, UnitPlacementError, Vector3};
    let axis = Axis1::new(Point3::default(), Direction3::Z);
    let rotation = Transform3::rotation(axis, 0.3).unwrap();
    let translation = Transform3::translation_vector(Vector3::new(1., -0., 2.));
    for transform in [Transform3::default(), rotation, translation] {
        let actual = transform.checked_unit_placement_rows().unwrap();
        for (row, entries) in actual.iter().enumerate() {
            for (column, value) in entries.iter().enumerate() {
                let stored = transform.value(row as i32 + 1, column as i32 + 1).unwrap();
                assert_eq!(value.to_bits(), stored.to_bits());
            }
        }
    }
    for form in [
        TransformForm::Identity,
        TransformForm::Translation,
        TransformForm::Scale,
        TransformForm::PointMirror,
    ] {
        let mut forged = rotation;
        forged.set_form(form);
        assert_eq!(
            forged.checked_unit_placement_rows(),
            Err(UnitPlacementError::Shortcut(form))
        );
    }
    let mut forged = translation;
    forged.set_form(TransformForm::Identity);
    assert_eq!(
        forged.checked_unit_placement_rows(),
        Err(UnitPlacementError::Shortcut(TransformForm::Identity))
    );
}

#[test]
fn affine_curve_uses_both_actual_paths_and_bounds_the_whole_distance() {
    use martensite_tkmath::predicates::roots::metric_bounds::planar_attachment::{
        PlanarCurvePath, PlanarCurveProofFailure,
    };
    let [outer, hole] = [square(4., false, 0.), square(1., true, 0.)];
    let chart =
        CertifiedPlanarAttachment::capture(Axis3::default(), PLACEMENT, [&outer, &hole], || false)
            .unwrap();
    let retained_world = [[58., 2., 9.125], [70., 2., 8.5]];
    let proposed_uv = [[-2., 2.], [2., 2.]];
    let evidence = chart
        .certify_straight_curve(retained_world, proposed_uv, 0.5, || false)
        .unwrap();
    assert_eq!(evidence.distance_bound, 0.5);
    assert_eq!(
        evidence.uv_lift.map(|point| point.distance_bound),
        [0.125, 0.5]
    );
    assert_eq!(evidence.world_projection, evidence.uv_lift);
    assert!(matches!(
        chart.certify_straight_curve(retained_world, proposed_uv, 0.5_f64.next_down(), || false),
        Err(PlanarCurveProofFailure::Attachment(
            PlanarAttachmentError::OffSupport { .. }
        ))
    ));
    // Retained 3D runs clear of the hole while the independently supplied UV path crosses;
    // match the actual metric caps first so the interval proof must reject the UV path.
    assert_eq!(
        chart.certify_straight_curve(retained_world, [[-2., 0.], [2., 0.]], 3., || false),
        Err(PlanarCurveProofFailure::Trim {
            path: PlanarCurvePath::UvLift,
            location: PlanarAttachmentLocation::HoleInterior,
        })
    );
}

#[test]
fn directed_segment_uses_cross_product_of_actual_placed_tangents() {
    use martensite_tkmath::predicates::roots::metric_bounds::planar_attachment::PlanarSegmentContact;

    let map = [[1., 0., 0., 0.], [0., 1., 0., 0.], [1., 0., 1., 0.]];
    let outer = square(4., false, 0.);
    let proof =
        CertifiedPlanarAttachment::capture_outer(Axis3::default(), map, &outer, || false).unwrap();
    let PlanarSegmentContact::Crossing { cut, location } = proof
        .classify_segment([[-1., 0., 0.], [2., 0., 0.]], || false)
        .unwrap()
    else {
        panic!("expected transverse hit of z=x");
    };
    let literal = 1.0_f64 / 3.;
    assert_eq!(location, PlanarAttachmentLocation::Material);
    assert_eq!(cut.parameter, [literal, literal.next_up()]);
    assert_eq!(cut.position, [0.; 3]);
    assert_eq!(cut.error_upper, 0.);
}

#[test]
fn single_loop_proof_rejects_the_unbounded_complement_winding() {
    let clockwise = square(4., true, 0.);
    assert!(matches!(
        CertifiedPlanarAttachment::capture_outer(Axis3::default(), PLACEMENT, &clockwise, || false),
        Err(PlanarAttachmentError::Loop)
    ));
}

#[test]
fn shared_outer_only_capture_accepts_the_straight_curve_across_its_interior() {
    let outer = square(4., false, 0.);
    let proof =
        CertifiedPlanarAttachment::capture_outer(Axis3::default(), PLACEMENT, &outer, || false)
            .unwrap();
    let result = proof
        .certify_straight_curve(
            [[-2., 0., 0.], [2., 0., 0.]],
            [[-2., 0.], [2., 0.]],
            0.,
            || false,
        )
        .unwrap();
    assert_eq!(result.distance_bound, 0.);
    assert!(
        result
            .uv_lift
            .into_iter()
            .all(|end| end.location == PlanarAttachmentLocation::Material)
    );
}
