//! The circle oracle shares no polynomial products or degree elevation with the proof.
use super::*;

const QUARTER: MetricBezier<'static, 2> = MetricBezier {
    poles: &[[5., 0.], [5., 1.], [4., 1.]],
    weights: Some(&[1., 1., 2.]),
};

#[test]
fn sixteen_dyadic_references_certify_value_and_derivative_error() {
    for index in 0..16 {
        let poles = reference(index, 4);
        let query = query(index, 4, &poles);
        let bound = certify(query, || false).unwrap();
        assert!(bound.position < 5e-6 && bound.derivative < 2e-5);
        assert!(bound.operations > 0 && bound.operations < query.policy.max_operations);
    }
}

#[test]
fn a_coarse_reference_cannot_exceed_the_requested_spatial_allowance() {
    let poles = reference(0, 0);
    let query = query(0, 0, &poles);
    let Err(Refusal::Exceeds { achieved, limits }) = certify(query, || false) else {
        panic!("the full rational quarter must be refined")
    };
    assert_eq!(limits, query.limits);
    assert!(achieved.position > limits[0] && achieved.derivative > limits[1]);
}

#[test]
fn fitter_keeps_the_authored_endpoints_and_one_rounded_value_per_joint() {
    let request = RationalReferenceRequest {
        source: QUARTER,
        level: 4,
        limits: [5e-6, 2e-5],
        direction: [-1., 1.],
        options: Default::default(),
    };
    let spans = fit_rational_bezier_reference(request, || false).unwrap();
    assert_eq!(spans.len(), 16);
    assert_eq!(spans[0].poles[0], QUARTER.poles[0]);
    assert_eq!(spans[15].poles[3], QUARTER.poles[2]);
    for (index, span) in spans.iter().enumerate() {
        let query = query(index as u64, 4, &span.poles);
        assert_eq!(span.interval, query.interval);
        assert!(monotone::certify_projection(query, [-1., 1.], || false).unwrap() > 0.);
    }
    for pair in spans.windows(2) {
        assert_eq!(
            pair[0].poles[3].map(f64::to_bits),
            pair[1].poles[0].map(f64::to_bits)
        );
    }
    assert!(matches!(
        fit_rational_bezier_reference(
            RationalReferenceRequest {
                level: 9,
                ..request
            },
            || false
        ),
        Err(Refusal::Level(9))
    ));
}

#[test]
fn all_quarter_spans_keep_a_strict_forward_projection_at_every_homotopy_time() {
    for index in 0..16 {
        let poles = reference(index, 4);
        let query = query(index, 4, &poles);
        assert!(monotone::certify_projection(query, [-1., 1.], || false).unwrap() > 0.);
    }

    let poles = reference(0, 4);
    let query = query(0, 4, &poles);
    let Err(Refusal::Monotonicity {
        projected_rate_lower,
        error_upper,
    }) = monotone::certify_projection(query, [1., -1.], || false)
    else {
        panic!("the requested projection reverses the authored curve")
    };
    assert!(projected_rate_lower < 0. && error_upper > 0.);
}

#[test]
fn work_and_cancellation_do_not_return_partial_evidence() {
    let poles = reference(0, 4);
    let mut query = query(0, 4, &poles);
    assert!(matches!(
        certify(query, || true),
        Err(Refusal::Arithmetic(BoundedArithmeticFailure::Cancelled))
    ));
    let used = certify(query, || false).unwrap().operations;
    query.policy.max_operations = used - 1;
    assert!(matches!(
        certify(query, || false),
        Err(Refusal::Arithmetic(_))
    ));
}

#[test]
fn malformed_sources_and_policy_retain_actual_operands() {
    let poles = reference(0, 4);
    let mut query = query(0, 4, &poles);
    query.source.weights = None;
    assert!(matches!(
        certify(query, || false),
        Err(Refusal::Shape {
            source: 3,
            weights: None,
            reference: 4
        })
    ));
    query.source.weights = Some(&[1., 0., 1.]);
    assert!(matches!(
        certify(query, || false),
        Err(Refusal::Weight {
            index: 1,
            value: 0.
        })
    ));
    query.policy.fractional_bits = 7;
    assert!(matches!(
        certify(query, || false),
        Err(Refusal::Policy {
            fractional_bits: 7,
            operations: 100_000
        })
    ));
    query.limits = [-1., 2.];
    assert!(matches!(
        certify(query, || false),
        Err(Refusal::Limits {
            position: -1.,
            derivative: 2.
        })
    ));
}

fn query(index: u64, level: u8, reference: &[[f64; 2]]) -> Query<'_> {
    Query {
        source: QUARTER,
        reference,
        interval: CylindricalProfileInterval::new(index, level).unwrap(),
        limits: [5e-6, 2e-5],
        policy: ArithmeticPolicy::default(),
    }
}

fn reference(index: u64, level: u8) -> [[f64; 2]; 4] {
    let width = 1. / f64::from(1_u32 << level);
    let [start, end] = [index as f64 * width, (index + 1) as f64 * width];
    let (a, da) = point_rate(start);
    let (b, db) = point_rate(end);
    [
        a,
        tangent(a, da, width / 3.),
        tangent(b, db, -width / 3.),
        b,
    ]
}

fn tangent(point: [f64; 2], rate: [f64; 2], scale: f64) -> [f64; 2] {
    std::array::from_fn(|axis| point[axis] + rate[axis] * scale)
}

fn point_rate(t: f64) -> ([f64; 2], [f64; 2]) {
    let weight = 1. + t * t;
    let squared = weight * weight;
    (
        [4. + (1. - t * t) / weight, 2. * t / weight],
        [-4. * t / squared, 2. * (1. - t * t) / squared],
    )
}

#[test]
fn residual_refinement_tightens_the_original_cubics_without_adding_reference_spans() {
    let poles = [[4.0625, 0.], [4.0625, 0.03125], [4., 0.03125]];
    let request = RationalReferenceRequest {
        source: MetricBezier {
            poles: &poles,
            weights: Some(&[1., 1., 2.]),
        },
        level: 2,
        limits: [4e-5, 1e-3],
        direction: [-0.0625, 0.03125],
        options: Default::default(),
    };
    let legacy = fit_rational_bezier_reference(request, || false)
        .err()
        .unwrap();
    assert!(matches!(legacy, Refusal::Exceeds { .. }));
    let refined = fit_rational_bezier_reference_with_proof(
        request,
        RationalReferenceProof::BisectResidualV2,
        || false,
    )
    .unwrap();
    let relaxed = fit_rational_bezier_reference(
        RationalReferenceRequest {
            limits: [1e-3; 2],
            ..request
        },
        || false,
    )
    .unwrap();
    assert_eq!(refined.len(), 4);
    for (actual, same) in refined.iter().zip(relaxed) {
        assert_eq!(actual.interval, same.interval);
        assert_eq!(
            actual.poles.map(|p| p.map(f64::to_bits)),
            same.poles.map(|p| p.map(f64::to_bits))
        );
        assert!(actual.bounds.position <= request.limits[0]);
    }
}
