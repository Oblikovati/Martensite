//! Actual leaf failure wins secondary bookkeeping refusal without dropping its work.
use super::*;
fn original_failure() -> AnalyticConicalFluxError {
    AnalyticConicalFluxError {
        requested_options: Default::default(),
        requested_arithmetic: Default::default(),
        span_count: 1,
        operations: 7,
        integration_subintervals: 2,
        cause: AnalyticConicalFluxFailure::Arithmetic {
            span: Some(0),
            depth: None,
            cause: Box::new(BoundedArithmeticFailure::WorkLimit {
                completed: 6,
                limit: 6,
            }),
        },
    }
}
#[test]
fn failed_flux_charges_once_and_retains_primary_cause_when_cell_charge_refuses() {
    let mut work = storage::Work::new(ConeClearanceOptions {
        fractional_bits: 128,
        max_operations: 100,
    });
    let options = CylindricalFluxOptions {
        max_total_subintervals: 1,
        ..Default::default()
    };
    let failure = charged(
        Err(original_failure()),
        SurfaceRegionId(1),
        &[],
        options,
        &mut work,
    )
    .err()
    .unwrap();
    assert_eq!(work.consumed, 7);
    assert_primary(failure);
}

fn assert_primary(failure: ConeFailure) {
    if let ConeCause::Flux(cause) = failure.cause {
        assert_eq!(cause.operations, 7);
        assert_eq!(cause.integration_subintervals, 2);
        assert!(matches!(
            cause.cause,
            AnalyticConicalFluxFailure::Arithmetic { span: Some(0), .. }
        ));
    } else {
        panic!("original flux error was replaced: {failure:?}");
    }
}

#[test]
fn successful_flux_charge_refusal_keeps_the_actual_computed_work() {
    let proof = real_flux();
    let operations = proof.arithmetic_operations();
    let mut work = storage::Work::new(ConeClearanceOptions {
        fractional_bits: 128,
        max_operations: 1,
    });
    let failure = charged(
        Ok(proof),
        SurfaceRegionId(1),
        &[],
        Default::default(),
        &mut work,
    )
    .err()
    .unwrap();
    assert!(matches!(failure.cause,ConeCause::Limit{actual,limit:1,..} if actual==operations));
    assert_eq!(work.consumed, 0);
}
fn real_flux() -> CertifiedAnalyticConicalFlux {
    let spans = [real_span()];
    let query = AnalyticConicalFluxQuery {
        spans: &spans,
        carriers: [ConicalTrimCarrier {
            radius: 2.,
            axial_origin: 1.,
            semi_angle: 0.5,
        }; 2],
        options: CylindricalFluxOptions {
            behavior_version: 2,
            ..Default::default()
        },
        arithmetic: Default::default(),
    };
    certify_analytic_conical_boundary_flux(query, || false).unwrap()
}

fn real_span() -> AnalyticConicalFluxSpan<'static> {
    const A: [[f64; 3]; 2] = [[0., 0., 1.], [1., 0., 1.]];
    const B: [[f64; 3]; 2] = [[0., 1., 1.], [1., 1., 1.]];
    const UV: [[f64; 2]; 2] = [[0.; 2]; 2];
    AnalyticConicalFluxSpan {
        boundaries: [&A, &B].map(|poles| MetricBezier {
            poles,
            weights: None,
        }),
        cap_pcurves: [&UV, &UV],
        side_reversed: false,
        cap_reversed: [false; 2],
    }
}
