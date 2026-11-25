//! New material reuse errors retain exact source operands and the original numerical cause.
use super::*;
fn failure(cause: ConicalDevelopedMaterialFailure) -> WrappedNormalFailure {
    let span = CurveSpan {
        curve: SurfaceCurveId(81),
        level: 3,
        index: 5,
    };
    let error = ConicalDevelopedMaterialError {
        span: Some(2),
        operations: 17,
        cause: Box::new(cause),
    };
    ConeFailure::new(
        Some(SurfaceRegionId(9)),
        Some(span),
        WrappedFailureClass::Incomplete,
        ConeCause::DevelopedMaterial(Box::new(error)),
    )
    .public()
}
#[test]
fn developed_binding_preserves_actual_expected_operands_and_authored_owner() {
    let failure = failure(ConicalDevelopedMaterialFailure::Binding {
        field: "actual retained UV",
        actual: vec![2],
        expected: vec![1],
    });
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    let cause = find_source::<ConicalDevelopedMaterialError>(&failure).unwrap();
    assert_eq!((cause.span, cause.operations), (Some(2), 17));
    assert!(
        matches!(cause.cause.as_ref(), ConicalDevelopedMaterialFailure::Binding { field: "actual retained UV", actual, expected } if actual == &[2] && expected == &[1])
    );
    assert!(
        failure
            .entities()
            .contains(&HistorySource::Curve(SurfaceCurveId(81)))
    );
}
#[test]
fn developed_numerical_invalid_input_remains_typed_and_distinct_from_binding() {
    let failure = failure(ConicalDevelopedMaterialFailure::Numerical(
        CylindricalProfileFailure::Value {
            role: "developed material",
            index: 2,
            value: f64::NAN,
        },
    ));
    assert_eq!(failure.class(), WrappedFailureClass::InvalidInput);
    assert!(find_source::<ConicalDevelopedMaterialError>(&failure).is_some());
    assert!(find_source::<CylindricalProfileFailure>(&failure).is_some());
}
#[test]
fn developed_numerical_cancellation_keeps_work_and_original_source() {
    let failure = failure(ConicalDevelopedMaterialFailure::Numerical(
        CylindricalProfileFailure::Arithmetic(Box::new(BoundedArithmeticFailure::Cancelled)),
    ));
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    let cause = find_source::<ConicalDevelopedMaterialError>(&failure).unwrap();
    assert_eq!(cause.operations, 17);
    assert!(find_source::<CylindricalProfileFailure>(&failure).is_some());
}
