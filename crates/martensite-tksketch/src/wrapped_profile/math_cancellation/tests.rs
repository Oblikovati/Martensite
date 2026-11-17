//! Cancellation and exhausted work remain distinct through each mathematical wrapper.
use super::*;

#[test]
fn arithmetic_cancelled_is_preserved_through_wrappers() {
    for cause in arithmetic_wrappers(BoundedArithmeticFailure::Cancelled) {
        assert!(cause.is_cancelled());
    }
}

#[test]
fn arithmetic_work_exhaustion_is_not_cancellation() {
    let refusal = BoundedArithmeticFailure::WorkLimit {
        completed: 7,
        limit: 7,
    };
    for cause in arithmetic_wrappers(refusal) {
        assert!(!cause.is_cancelled());
    }
}

fn arithmetic_wrappers(cause: BoundedArithmeticFailure) -> Vec<Box<dyn MathCancellation>> {
    let mut result: Vec<Box<dyn MathCancellation>> = vec![Box::new(cause.clone())];
    result.extend(profile_wrappers(&cause));
    result.extend(measure_wrappers(&cause));
    result
}

fn measure_wrappers(cause: &BoundedArithmeticFailure) -> Vec<Box<dyn MathCancellation>> {
    let failure = CylindricalMeasureFailure::Arithmetic(Box::new(cause.clone()));
    vec![
        Box::new(failure.clone()),
        Box::new(CylindricalMeasureError {
            requested_options: Default::default(),
            requested_limit: CylindricalMeasureLimit::Report,
            requested_depths: [-1., 2.],
            source_count: 1,
            cause: failure,
        }),
    ]
}

fn profile_wrappers(c: &BoundedArithmeticFailure) -> Vec<Box<dyn MathCancellation>> {
    vec![
        Box::new(CylindricalProfileFailure::Arithmetic(Box::new(c.clone()))),
        Box::new(CylindricalClearanceFailure::Arithmetic(Box::new(c.clone()))),
        Box::new(CylindricalProbeFailure::Arithmetic(Box::new(c.clone()))),
        Box::new(CylindricalConeFailure::Arithmetic(Box::new(c.clone()))),
        Box::new(CylindricalMeasureComparisonFailure::Arithmetic(Box::new(
            c.clone(),
        ))),
    ]
}

#[test]
fn root_cancelled_is_preserved_through_integral_wrappers() {
    for cause in root_wrappers(RootIsolationError::Cancelled) {
        assert!(cause.is_cancelled());
    }
}

#[test]
fn root_limit_is_preserved_as_refusal() {
    for cause in root_wrappers(RootIsolationError::SubdivisionLimit { limit: 4 }) {
        assert!(!cause.is_cancelled());
    }
}

fn root_wrappers(c: RootIsolationError) -> Vec<Box<dyn MathCancellation>> {
    let mut result: Vec<Box<dyn MathCancellation>> = vec![
        Box::new(c.clone()),
        Box::new(BezierRayError::Direction(c.clone())),
        Box::new(BezierWireError::Segment {
            id: 9,
            cause: BezierRayError::Endpoint(c.clone()),
        }),
        Box::new(ResidualError::Roots(c.clone())),
    ];
    result.extend(integral_wrappers(c));
    result
}

fn integral_wrappers(c: RootIsolationError) -> Vec<Box<dyn MathCancellation>> {
    vec![
        Box::new(PolynomialIntegralFailure::Metric(Box::new(
            ResidualError::Roots(c.clone()),
        ))),
        Box::new(RationalIntegralFailure::Metric(Box::new(
            ResidualError::Roots(c.clone()),
        ))),
        Box::new(CylindricalFluxFailure::Geometry {
            span: 2,
            cause: Box::new(ResidualError::Roots(c)),
        }),
    ]
}

#[test]
fn admission_errors_are_not_cancelled() {
    let causes: &[&dyn MathCancellation] = &[
        &CylindricalDepthFailure::DepthOrder { depths: [1., 0.] },
        &FrameTransportError::ZeroAxis { axis: [0.; 3] },
        &CylindricalStorageFailure::WorkLimit {
            completed: 3,
            limit: 3,
        },
    ];
    for cause in causes {
        assert!(!cause.is_cancelled());
    }
}
