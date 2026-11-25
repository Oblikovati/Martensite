//! Bridge projection retains caller-relative depth and the public original trace cause.
use super::*;
#[test]
fn chart_trace_projection_keeps_invalid_input_and_original_source() {
    let failure = projected(trace(CylindricalProfileFailure::Value {
        role: "actual chart trim",
        index: 3,
        value: f64::NAN,
    }));
    assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
    let error = find_source::<ConicalChartTrimPairError>(&failure).unwrap();
    assert_eq!(error.depth, Some(1));
    assert_eq!(error.operations, 17);
    assert!(find_source::<ConicalTrimError>(&failure).is_some());
}
#[test]
fn chart_trace_cancellation_and_binding_have_distinct_sources_and_classes() {
    let cancelled = projected(trace(CylindricalProfileFailure::Arithmetic(Box::new(
        BoundedArithmeticFailure::Cancelled,
    ))));
    assert_eq!(cancelled.class(), WrappedFailureClass::Cancelled);
    let binding = projected(ConeCause::ChartTrimPair(Box::new(
        ConicalChartTrimPairError {
            depth: None,
            operations: 0,
            cause: Box::new(ConicalChartTrimPairFailure::Binding {
                field: "actual decoded weights",
                actual: vec![2],
                expected: vec![1],
            }),
        },
    )));
    assert_eq!(binding.class(), WrappedFailureClass::Incomplete);
    let error = find_source::<ConicalChartTrimPairError>(&binding).unwrap();
    assert_eq!(error.depth, None);
    assert!(find_source::<ConicalTrimError>(&binding).is_none());
}
fn trace(cause: CylindricalProfileFailure) -> ConeCause {
    let trace = ConicalTrimError {
        carrier: ConicalTrimCarrier {
            radius: 1.,
            axial_origin: 0.,
            semi_angle: 0.7,
        },
        requested_spatial_limit: 1e-4,
        requested_options: Default::default(),
        cause,
    };
    ConeCause::ChartTrimPair(Box::new(ConicalChartTrimPairError {
        depth: Some(1),
        operations: 17,
        cause: Box::new(ConicalChartTrimPairFailure::Trace(trace)),
    }))
}
