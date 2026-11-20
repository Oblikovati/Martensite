// SPDX-License-Identifier: GPL-2.0-only
//! Full analytic cap rectangles include filled domains, not sampled boundary vertices.
use super::*;
use crate::predicates::roots::metric_bounds::conical_profile::boundary_chart::tests::domain;
fn fixture(
    body: impl FnOnce(&[ConicalActualCapQuery<'_>], &mut Arithmetic<'_>, &Interval, &Interval),
) {
    let first = [[0., 0.], [0.125, 2.]];
    let last = [[0.25, 0.], [0.375, 2.]];
    let weights = [1., 1.];
    let edges = [edge(1, &first, &weights), edge(2, &last, &weights)];
    let wires = [wire(&edges[..1]), wire(&edges[1..])];
    let queries = [cap_query(1, &wires[..1]), cap_query(2, &wires[1..])];
    let context = domain(0.125);
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 10000,
        },
        &mut never,
    );
    body(&queries, &mut arithmetic, &context.sine, &context.cosine);
}
#[test]
fn positive_filled_radius_and_disjoint_phase_certify_the_whole_rectangles() {
    fixture(|query, arithmetic, sine, cosine| {
        let period = arithmetic.scalar(6.).unwrap();
        let a = cap(&query[0], sine, cosine, &period, false, false, arithmetic).unwrap();
        let b = cap(&query[1], sine, cosine, &period, false, false, arithmetic).unwrap();
        let ten = arithmetic.scalar(10.).unwrap();
        let v = arithmetic.scalar(2.).unwrap();
        let v = Interval {
            lower: 0.into(),
            upper: v.upper,
        };
        let radius = filled_radius(&ten, &v, sine, arithmetic).unwrap();
        assert!(radius.lower >= ten.lower);
        pairs::certify(&[a, b], &period, arithmetic).unwrap();
    });
}
#[test]
fn overlapping_cap_envelopes_are_inconclusive_not_success() {
    fixture(|query, arithmetic, sine, cosine| {
        let period = arithmetic.scalar(6.).unwrap();
        let a = cap(&query[0], sine, cosine, &period, false, false, arithmetic).unwrap();
        let mut other = query[0];
        other.carrier.member = ConicalActualMemberId(2);
        let b = cap(&other, sine, cosine, &period, false, false, arithmetic).unwrap();
        assert!(matches!(
            pairs::certify(&[a, b], &period, arithmetic),
            Err(ConicalActualPatchFailure::CapPair { .. })
        ));
    });
}

#[test]
fn caps_with_identical_phase_and_level_separate_along_common_slant() {
    fixture(|query, arithmetic, sine, cosine| {
        let period = arithmetic.scalar(6.).unwrap();
        let lower = cap(&query[0], sine, cosine, &period, true, false, arithmetic).unwrap();
        let poles = [[0., 4.], [0.125, 6.]];
        let edges = [edge(2, &poles, &[1., 1.])];
        let wires = [wire(&edges)];
        let higher = cap(
            &cap_query(2, &wires),
            sine,
            cosine,
            &period,
            true,
            false,
            arithmetic,
        )
        .unwrap();
        assert_eq!(lower.u, higher.u);
        assert_eq!(lower.level, higher.level);
        pairs::certify(&[lower, higher], &period, arithmetic).unwrap();
    });
}

#[test]
fn family_cap_separation_refuses_shared_slant_endpoints_and_phase_overlap() {
    fixture(|query, arithmetic, sine, cosine| {
        let period = arithmetic.scalar(6.).unwrap();
        let lower = cap(&query[0], sine, cosine, &period, true, false, arithmetic).unwrap();
        let poles = [[0., 2.], [0.125, 4.]];
        let edges = [edge(2, &poles, &[1., 1.])];
        let wires = [wire(&edges)];
        let higher = cap(
            &cap_query(2, &wires),
            sine,
            cosine,
            &period,
            true,
            false,
            arithmetic,
        )
        .unwrap();
        assert!(matches!(
            pairs::certify(&[lower, higher], &period, arithmetic),
            Err(ConicalActualPatchFailure::CapPair {
                caps: [ConicalActualCapId(1), ConicalActualCapId(2)],
                ..
            })
        ));
    });
}

fn edge<'a>(id: u64, poles: &'a [[f64; 2]], weights: &'a [f64]) -> ConicalActualUvEdge<'a> {
    ConicalActualUvEdge {
        side: ConicalActualSideId(id),
        row: 0,
        poles,
        weights,
        range: [0., 1.],
        reversed: false,
    }
}
fn wire<'a>(edges: &'a [ConicalActualUvEdge<'a>]) -> ConicalActualUvContour<'a> {
    ConicalActualUvContour {
        role: ConicalActualContourRole::Outer,
        wire_orientation: 1,
        edges,
    }
}
fn cap_query<'a>(id: u64, contours: &'a [ConicalActualUvContour<'a>]) -> ConicalActualCapQuery<'a> {
    ConicalActualCapQuery {
        carrier: carrier(id),
        face_orientation: if id == 1 { 1 } else { -1 },
        contours,
    }
}
fn carrier(id: u64) -> ConicalActualCapCarrier {
    ConicalActualCapCarrier {
        id: ConicalActualCapId(id),
        member: ConicalActualMemberId(id),
        frame: [[0., 0., 80.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        radius: 10.,
        axial_origin: 80.,
        semi_angle: 0.125,
    }
}

#[test]
fn filled_rectangle_crossing_the_axis_refuses_with_actual_radius_bounds() {
    fixture(|_, arithmetic, sine, cosine| {
        let controls = [[-0.125, -100.], [0.125, 0.]];
        let weights = [1., 1.];
        let edges = [edge(1, &controls, &weights)];
        let wires = [wire(&edges)];
        let query = cap_query(1, &wires);
        let period = arithmetic.scalar(6.).unwrap();
        let failure = cap(&query, sine, cosine, &period, false, false, arithmetic)
            .err()
            .unwrap();
        assert_crossing_radius(failure);
    });
}
fn assert_crossing_radius(failure: ConicalActualPatchFailure) {
    if let ConicalActualPatchFailure::Unresolved {
        quantity,
        lower,
        upper,
        ..
    } = failure
    {
        assert_eq!(quantity, "entire cap rectangle radius");
        assert!(lower.compare_f64(0.).unwrap().is_lt());
        assert!(upper.compare_f64(0.).unwrap().is_gt());
    } else {
        panic!("expected actual axis-crossing radius refusal: {failure:?}");
    }
}
