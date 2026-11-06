//! `transient replacements`: fields that hold a measure as any transient, so either a
//! measure with unit or a representation item combined with one.
use martensite_tkdestep::step_basic::*;
use martensite_tkdestep::step_data::{StepDataLogical, StepDataSelectMember, StepDataSelectType};
use martensite_tkdestep::step_dim_tol::{
    StepDimTolGeometricTolerance, StepDimTolGeometricToleranceTarget,
};
use martensite_tkdestep::step_repr::*;
use martensite_tkdestep::step_shape::*;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;

const TOLERANCE: f64 = 1e-7;

fn measure_with_unit(value: f64) -> Handle<StepBasicMeasureWithUnit> {
    let measure = Handle::new(StepBasicMeasureWithUnit::new());
    let member = Handle::new(StepBasicMeasureValueMember::new());
    member.set_name("POSITIVE_LENGTH_MEASURE");
    member.set_real(value);
    measure.set_value_component_member(&member);
    let si_unit = Handle::new(StepBasicSiUnit::new());
    si_unit.init(
        false,
        StepBasicSiPrefix::SpMilli,
        StepBasicSiUnitName::SunMetre,
    );
    let mut unit = StepBasicUnit::default();
    unit.set_value(&si_unit.cast()).unwrap();
    measure.set_unit_component(&unit);
    measure
}

fn repr_item_and_measure(value: f64) -> Handle<StepReprReprItemAndMeasureWithUnit> {
    let item = Handle::new(StepReprReprItemAndMeasureWithUnit::new());
    item.set_measure_with_unit(&measure_with_unit(value));
    item.set_name(&text("TestReprItem"));
    item
}

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The two states every case checks: the field holds what was set, as its dynamic type.
fn expect_measure_holds(field: &Handle<StandardTransient>, value: f64) {
    assert!(field.is_some());
    assert!(field.is_kind(StepBasicMeasureWithUnit::static_type()));
    let measure = field.cast::<StepBasicMeasureWithUnit>();
    assert!(measure.is_some());
    assert!((measure.value_component() - value).abs() < TOLERANCE);
}

fn expect_repr_item_holds(field: &Handle<StandardTransient>, value: f64) {
    assert!(field.is_some());
    assert!(field.is_kind(StepReprReprItemAndMeasureWithUnit::static_type()));
    let item = field.cast::<StepReprReprItemAndMeasureWithUnit>();
    assert!(item.is_some());
    let extracted = item.get_measure_with_unit();
    assert!((extracted.value_component() - value).abs() < TOLERANCE);
}

#[test]
fn conversion_based_unit_works_with_both_types() {
    let exponents = Handle::new(StepBasicDimensionalExponents::new());
    exponents.init(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let unit = StepBasicConversionBasedUnit::new();
    unit.init(
        &exponents,
        &text("TestName"),
        &measure_with_unit(5.0).cast(),
    );
    expect_measure_holds(&unit.conversion_factor(), 5.0);
    unit.set_conversion_factor(&repr_item_and_measure(10.0).cast());
    expect_repr_item_holds(&unit.conversion_factor(), 10.0);
}

#[test]
fn geometric_tolerance_works_with_both_types() {
    let tolerance = StepDimTolGeometricTolerance::new();
    let target = StepDimTolGeometricToleranceTarget::default();
    tolerance.init_geometric_tolerance_target(
        &text("TestName"),
        &text("TestDescription"),
        &measure_with_unit(5.0).cast(),
        &target,
    );
    expect_measure_holds(&tolerance.magnitude(), 5.0);
    tolerance.set_magnitude(&repr_item_and_measure(10.0).cast());
    expect_repr_item_holds(&tolerance.magnitude(), 10.0);
}

#[test]
fn make_from_usage_option_works_with_both_types() {
    let definition = Handle::new(StepBasicProductDefinition::new());
    let usage = StepReprMakeFromUsageOption::new();
    usage
        .init(
            &text("TestName"),
            &text("TestName"),
            true,
            &text("TestDescription"),
            &definition,
            &definition,
            1,
            &text("TestDescription"),
            &measure_with_unit(5.0).cast(),
        )
        .unwrap();
    expect_measure_holds(&usage.quantity(), 5.0);
    usage.set_quantity(&repr_item_and_measure(10.0).cast());
    expect_repr_item_holds(&usage.quantity(), 10.0);
}

#[test]
fn parallel_offset_works_with_both_types() {
    let offset = StepReprParallelOffset::new();
    offset.init(
        &text("TestName"),
        &text("TestDescription"),
        &Handle::new(StepReprProductDefinitionShape::new()),
        StepDataLogical::LTrue,
        &measure_with_unit(5.0).cast(),
    );
    expect_measure_holds(&offset.offset(), 5.0);
    offset.set_offset(&repr_item_and_measure(10.0).cast());
    expect_repr_item_holds(&offset.offset(), 10.0);
}

#[test]
fn quantified_assembly_component_usage_works_with_both_types() {
    let definition = Handle::new(StepBasicProductDefinition::new());
    let usage = StepReprQuantifiedAssemblyComponentUsage::new();
    usage
        .init(
            &text("TestName"),
            &text("TestName"),
            true,
            &text("TestDescription"),
            &definition,
            &definition,
            true,
            &text("TestName"),
            &measure_with_unit(5.0).cast(),
        )
        .unwrap();
    expect_measure_holds(&usage.quantity(), 5.0);
    usage.set_quantity(&repr_item_and_measure(10.0).cast());
    expect_repr_item_holds(&usage.quantity(), 10.0);
}

#[test]
fn measure_qualification_works_with_both_types() {
    let qualifiers = Handle::new(HArray1::<StepShapeValueQualifier>::with_bounds(1, 1).unwrap());
    let qualification = StepShapeMeasureQualification::new();
    qualification.init(
        &text("TestName"),
        &text("TestDescription"),
        &measure_with_unit(5.0).cast(),
        &qualifiers,
    );
    expect_measure_holds(&qualification.qualified_measure(), 5.0);
    qualification.set_qualified_measure(&repr_item_and_measure(10.0).cast());
    expect_repr_item_holds(&qualification.qualified_measure(), 10.0);
}

/// The helper the test recreates from the assembly reader.
fn get_measure_with_unit(measure: &Handle<StandardTransient>) -> Handle<StepBasicMeasureWithUnit> {
    if measure.is_null() {
        return Handle::default();
    }
    if measure.is_kind(StepBasicMeasureWithUnit::static_type()) {
        return measure.cast();
    }
    if measure.is_kind(StepReprReprItemAndMeasureWithUnit::static_type()) {
        return measure
            .cast::<StepReprReprItemAndMeasureWithUnit>()
            .get_measure_with_unit();
    }
    Handle::default()
}

#[test]
fn get_measure_with_unit_extracts_correctly() {
    assert!(get_measure_with_unit(&Handle::default()).is_null());
    let direct = get_measure_with_unit(&measure_with_unit(5.0).cast());
    assert!((direct.value_component() - 5.0).abs() < TOLERANCE);
    let through_item = get_measure_with_unit(&repr_item_and_measure(10.0).cast());
    assert!((through_item.value_component() - 10.0).abs() < TOLERANCE);
    assert!(get_measure_with_unit(&text("TestName").cast()).is_null());
}
