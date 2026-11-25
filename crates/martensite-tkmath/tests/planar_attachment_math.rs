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
    assert_eq!(query.distance_bound, 1.);
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
