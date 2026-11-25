// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4, FRAC_PI_8};

fn unsnapped_circle() -> AdmittedConicSpan {
    AdmittedConicSpan {
        frame: AdmittedConicFrame {
            center: [4., 0.],
            directions: [[1., 0.], [0., 1.]],
            scales: [1.; 2],
        },
        phase: AdmittedConicPhase::Sweep {
            first: 0.,
            sweep: FRAC_PI_4,
            index: 0,
            count: 1,
            parameter: ConicSpanParameter::Symmetric,
        },
        poles: [
            [5., 0.],
            [5., FRAC_PI_8.tan()],
            [4. + FRAC_1_SQRT_2, FRAC_1_SQRT_2],
        ],
        weights: [1., FRAC_PI_8.cos(), 1.],
    }
}

#[test]
fn encloses_standard_circle_and_its_literal_sewing() {
    let original = unsnapped_circle();
    let evidence = certify_admitted_conic_span(original, Default::default(), || false).unwrap();
    assert!(evidence.transport.position > 0. && evidence.transport.position < 1e-13);
    assert!(evidence.transport.derivative > 0. && evidence.transport.derivative < 1e-12);
    assert!(evidence.area_transfer > 0. && evidence.area_transfer < 1e-12);
    assert!(evidence.endpoint_error > 0. && evidence.endpoint_error < 1e-14);

    let mut altered = original;
    altered.poles[2][0] += 0.001;
    let evidence = certify_admitted_conic_span(altered, Default::default(), || false).unwrap();
    assert!(evidence.endpoint_error > 0.000_999 && evidence.endpoint_error < 0.001_001);
    assert!(evidence.transport.position >= evidence.endpoint_error);
}

#[test]
fn phase_frame_and_work_refuse_with_their_own_causes() {
    let mut source = unsnapped_circle();
    source.phase = AdmittedConicPhase::Sweep {
        first: 0.,
        sweep: FRAC_PI_4,
        index: 1,
        count: 1,
        parameter: ConicSpanParameter::FirstEndpoint,
    };
    assert!(matches!(
        certify_admitted_conic_span(source, Default::default(), || false),
        Err(RationalReferenceFailure::ConicPhase {
            index: 1,
            count: 1,
            ..
        })
    ));
    let mut source = unsnapped_circle();
    source.frame.directions.swap(0, 1);
    assert!(matches!(
        certify_admitted_conic_span(source, Default::default(), || false),
        Err(RationalReferenceFailure::ConicFrame(_))
    ));
    let options = CylindricalProfileOptions {
        max_operations: 4,
        ..Default::default()
    };
    assert!(matches!(
        certify_admitted_conic_span(unsnapped_circle(), options, || false),
        Err(RationalReferenceFailure::Arithmetic(
            BoundedArithmeticFailure::WorkLimit { limit: 4, .. }
        ))
    ));
}
