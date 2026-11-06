use crate::step_rw::*;

#[test]
fn enum_text_round_trip_rw_step_visual_rw_central_or_parallel() {
    for value in [
        StepVisualCentralOrParallel::CopCentral,
        StepVisualCentralOrParallel::CopParallel,
    ] {
        let mut read = StepVisualCentralOrParallel::default();
        assert!(RwStepVisualRwCentralOrParallel::convert_to_enum(
            RwStepVisualRwCentralOrParallel::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepVisualCentralOrParallel::default();
    assert!(!RwStepVisualRwCentralOrParallel::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_visual_rw_surface_side() {
    for value in [
        StepVisualSurfaceSide::SsNegative,
        StepVisualSurfaceSide::SsPositive,
        StepVisualSurfaceSide::SsBoth,
    ] {
        let mut read = StepVisualSurfaceSide::default();
        assert!(RwStepVisualRwSurfaceSide::convert_to_enum(
            RwStepVisualRwSurfaceSide::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepVisualSurfaceSide::default();
    assert!(!RwStepVisualRwSurfaceSide::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}

#[test]
fn enum_text_round_trip_rw_step_visual_rw_text_path() {
    for value in [
        StepVisualTextPath::TpUp,
        StepVisualTextPath::TpRight,
        StepVisualTextPath::TpDown,
        StepVisualTextPath::TpLeft,
    ] {
        let mut read = StepVisualTextPath::default();
        assert!(RwStepVisualRwTextPath::convert_to_enum(
            RwStepVisualRwTextPath::convert_to_string(value),
            &mut read
        ));
        assert_eq!(read, value);
    }
    let mut read = StepVisualTextPath::default();
    assert!(!RwStepVisualRwTextPath::convert_to_enum(
        b".NOT_A_VALUE.",
        &mut read
    ));
}
