use crate::step_rw::*;

#[test]
fn enum_text_round_trip_rw_step_shape_rw_boolean_operator() {
    for value in [
        StepShapeBooleanOperator::BoDifference,
        StepShapeBooleanOperator::BoIntersection,
        StepShapeBooleanOperator::BoUnion,
    ] {
        let mut read = StepShapeBooleanOperator::default();
        assert!(RwStepShapeRwBooleanOperator::convert_to_enum(
            RwStepShapeRwBooleanOperator::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepShapeBooleanOperator::default();
    assert!(!RwStepShapeRwBooleanOperator::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}
