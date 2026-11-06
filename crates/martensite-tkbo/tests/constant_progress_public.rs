//! Public constant-weight policies preserve caller budgets, array bounds and untouched slots.
use martensite_tkbo::bop_algo::{
    BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoPaveFiller, BOPAlgoRemoveFeatures, BOPAlgoSection,
    Operation, PISteps,
};
use martensite_tkernel::ncollection::Array1;
use std::{fmt::Debug, str::FromStr};

const OPERATIONS: [Operation; 6] = [
    Operation::Common,
    Operation::Fuse,
    Operation::Cut,
    Operation::Cut21,
    Operation::Section,
    Operation::Unknown,
];

const CASES: [&str; 12] = [
    include_str!("constant_progress_expected/cases0.expected"),
    include_str!("constant_progress_expected/cases1.expected"),
    include_str!("constant_progress_expected/cases2.expected"),
    include_str!("constant_progress_expected/cases3.expected"),
    include_str!("constant_progress_expected/cases4.expected"),
    include_str!("constant_progress_expected/cases5.expected"),
    include_str!("constant_progress_expected/cases6.expected"),
    include_str!("constant_progress_expected/cases7.expected"),
    include_str!("constant_progress_expected/cases8.expected"),
    include_str!("constant_progress_expected/cases9.expected"),
    include_str!("constant_progress_expected/cases10.expected"),
    include_str!("constant_progress_expected/cases11.expected"),
];

struct ConstantCase {
    kind: u8,
    flag: bool,
    operation: Operation,
    whole: f64,
    lower: i32,
    upper: i32,
    expected: Vec<u64>,
}

fn fixture_value<T: FromStr>(field: &str) -> T
where
    T::Err: Debug,
{
    field.parse().unwrap_or_else(|cause| {
        panic!(
            "field={field:?}, cause={cause:?}; expected {}",
            std::any::type_name::<T>()
        )
    })
}

fn fixture_case(line: &str) -> ConstantCase {
    let fields: Vec<_> = line.split('|').collect();
    assert_eq!(fields.len(), 7, "case={line:?}; expected seven fields");
    ConstantCase {
        kind: fixture_value(fields[0]),
        flag: fixture_value::<u8>(fields[1]) == 1,
        operation: OPERATIONS[fixture_value::<usize>(fields[2])],
        whole: f64::from_bits(fixture_value(fields[3])),
        lower: fixture_value(fields[4]),
        upper: fixture_value(fields[5]),
        expected: fields[6].split(',').map(fixture_value).collect(),
    }
}

fn filler_constants(case: &ConstantCase, steps: &mut PISteps) {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_non_destructive(case.flag);
    filler.fill_pi_constants(case.whole, steps);
    assert_eq!(filler.non_destructive(), case.flag);
    assert!(!filler.options().has_errors());
}

fn builder_constants(case: &ConstantCase, steps: &mut PISteps) {
    let mut builder = BOPAlgoBuilder::new();
    builder.set_to_fill_history(case.flag);
    builder.fill_pi_constants(case.whole, steps);
    assert_eq!(builder.has_history(), case.flag);
    assert!(builder.shape().is_null() && !builder.options().has_errors());
}

fn boolean_constants(case: &ConstantCase, steps: &mut PISteps) {
    let mut boolean = BOPAlgoBOP::new();
    boolean.set_to_fill_history(case.flag);
    boolean.set_operation(case.operation);
    boolean.fill_pi_constants(case.whole, steps);
    assert_eq!(
        (boolean.has_history(), boolean.operation()),
        (case.flag, case.operation)
    );
    assert!(boolean.shape().is_null() && !boolean.options().has_errors());
}

fn section_constants(case: &ConstantCase, steps: &mut PISteps) {
    let mut section = BOPAlgoSection::new();
    section.set_to_fill_history(case.flag);
    section.fill_pi_constants(case.whole, steps);
    assert_eq!(section.has_history(), case.flag);
    assert!(section.shape().is_null() && !section.options().has_errors());
}

fn removal_constants(case: &ConstantCase, steps: &mut PISteps) {
    let mut removal = BOPAlgoRemoveFeatures::new();
    removal.set_to_fill_history(case.flag);
    removal.fill_pi_constants(case.whole, steps);
    assert_eq!(removal.has_history(), case.flag);
    assert!(removal.shape().is_null() && !removal.options().has_errors());
}

fn apply_receiver_constants(case: &ConstantCase, steps: &mut PISteps) {
    match case.kind {
        0 => filler_constants(case, steps),
        1 => builder_constants(case, steps),
        2 => boolean_constants(case, steps),
        3 => section_constants(case, steps),
        4 => removal_constants(case, steps),
        kind => panic!("kind={kind}; expected receiver kind 0..=4"),
    }
}

#[test]
fn all_five_callbacks_have_their_actual_immutable_receiver_type() {
    type ConstantCallback<T> = for<'a, 'b> fn(&'a T, f64, &'b mut PISteps);
    let _: ConstantCallback<BOPAlgoPaveFiller> = BOPAlgoPaveFiller::fill_pi_constants;
    let _: ConstantCallback<BOPAlgoBuilder> = BOPAlgoBuilder::fill_pi_constants;
    let _: ConstantCallback<BOPAlgoBOP> = BOPAlgoBOP::fill_pi_constants;
    let _: ConstantCallback<BOPAlgoSection> = BOPAlgoSection::fill_pi_constants;
    let _: ConstantCallback<BOPAlgoRemoveFeatures> = BOPAlgoRemoveFeatures::fill_pi_constants;
}

#[test]
fn all_native_budget_flag_operation_and_shifted_array_cases_match_every_slot_bit() {
    let mut count = 0;
    for line in CASES.iter().flat_map(|chunk| chunk.lines()) {
        check_case_slots(line);
        count += 1;
    }
    assert_eq!(count, 2400);
}

fn check_case_slots(line: &str) {
    let case = fixture_case(line);
    let mut steps = PISteps::new(1).unwrap();
    *steps.change_steps() = Array1::with_bounds(case.lower, case.upper).unwrap();
    steps.change_steps().init(-7.25);
    apply_receiver_constants(&case, &mut steps);
    let actual: Vec<_> = steps
        .steps()
        .as_slice()
        .iter()
        .map(|value| value.to_bits())
        .collect();
    assert_eq!(actual, case.expected, "case={line}");
    assert_eq!(
        (steps.steps().lower(), steps.steps().upper()),
        (case.lower, case.upper)
    );
}

#[test]
fn filler_reserves_twenty_of_two_thousand_without_touching_variable_slots() {
    let mut steps = PISteps::new(14).unwrap();
    steps.change_steps().init(-7.25);
    BOPAlgoPaveFiller::new().fill_pi_constants(2000., &mut steps);
    assert_eq!(steps.get_step(0), 20.);
    assert!(
        steps.steps().as_slice()[1..]
            .iter()
            .all(|value| *value == -7.25)
    );
}

#[test]
fn builder_reserves_one_hundred_for_history_and_sixty_for_post_treatment() {
    let mut steps = PISteps::new(11).unwrap();
    let mut builder = BOPAlgoBuilder::new();
    builder.set_to_fill_history(true);
    builder.fill_pi_constants(2000., &mut steps);
    assert_eq!(
        (steps.get_step(8), steps.get_step(9), steps.get_step(10)),
        (100., 60., 0.)
    );
}

#[test]
fn boolean_fuse_reserves_two_hundred_and_other_operations_one_hundred() {
    let mut boolean = BOPAlgoBOP::new();
    for operation in OPERATIONS {
        let mut steps = PISteps::new(11).unwrap();
        boolean.set_operation(operation);
        boolean.fill_pi_constants(2000., &mut steps);
        let expected = if operation == Operation::Fuse {
            200.
        } else {
            100.
        };
        assert_eq!(steps.get_step(10), expected);
    }
}

#[test]
fn section_uses_its_own_slots_and_larger_history_fraction() {
    let mut section = BOPAlgoSection::new();
    section.set_to_fill_history(true);
    let mut steps = PISteps::new(5).unwrap();
    section.fill_pi_constants(2000., &mut steps);
    assert_eq!(steps.steps().as_slice(), &[0., 0., 0., 200., 100.]);
}

#[test]
fn feature_removal_reserves_all_four_slots_even_without_history() {
    let mut removal = BOPAlgoRemoveFeatures::new();
    removal.set_to_fill_history(false);
    let mut steps = PISteps::new(4).unwrap();
    removal.fill_pi_constants(2000., &mut steps);
    assert_eq!(steps.steps().as_slice(), &[100., 1600., 100., 200.]);
}

#[test]
fn fractional_budget_uses_multiply_then_divide_in_boolean_and_section_callbacks() {
    let mut boolean = BOPAlgoBOP::new();
    boolean.set_operation(Operation::Fuse);
    let mut steps = PISteps::new(11).unwrap();
    boolean.fill_pi_constants(3., &mut steps);
    assert_eq!(steps.get_step(10).to_bits(), 0.3_f64.to_bits());
    let mut section = BOPAlgoSection::new();
    section.set_to_fill_history(true);
    section.fill_pi_constants(3., &mut steps);
    assert_eq!(steps.get_step(3).to_bits(), 0.3_f64.to_bits());
    assert_eq!(steps.get_step(4).to_bits(), 0.15_f64.to_bits());
}
