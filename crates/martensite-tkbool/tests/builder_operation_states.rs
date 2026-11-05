//! Operation predicates follow explicit operand states, including non-selection states.
use martensite_tkbool::top_ope_brep_build::TopOpeBRepBuildBuilder;
use martensite_tkbool::top_ope_brep_ds::{TopOpeBRepDSBuildTool, TopOpeBRepDSHDataStructure};
use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::State;

const STATES: [State; 4] = [State::In, State::Out, State::On, State::Unknown];

fn prepared_operation_builder() -> Result<TopOpeBRepBuildBuilder, Failure> {
    let mut builder = TopOpeBRepBuildBuilder::new(&TopOpeBRepDSBuildTool::new());
    builder.perform(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    Ok(builder)
}

fn empty_operands() -> [Shape; 2] {
    let mut first = Compound::default();
    let mut second = Compound::default();
    Builder.make_compound(&mut first);
    Builder.make_compound(&mut second);
    [first.into_shape(), second.into_shape()]
}

fn assert_operation_predicates(builder: &TopOpeBRepBuildBuilder, first: State, second: State) {
    assert_eq!(builder.opec12(), first == State::Out && second == State::In);
    assert_eq!(builder.opec21(), first == State::In && second == State::Out);
    assert_eq!(builder.opecom(), first == State::In && second == State::In);
    assert_eq!(
        builder.opefus(),
        first == State::Out && second == State::Out
    );
}

#[test]
fn merge_assigns_both_operand_states_for_every_enum_pair() -> Result<(), Failure> {
    let [first, second] = empty_operands();
    let mut builder = prepared_operation_builder()?;
    for first_state in STATES {
        for second_state in STATES {
            builder.merge_shapes(&first, first_state, &second, second_state)?;
            assert_operation_predicates(&builder, first_state, second_state);
            assert_eq!(builder.shape_rank(&first), 0);
            assert_eq!(builder.shape_rank(&second), 0);
        }
    }
    Ok(())
}

#[test]
fn equal_operand_merge_preserves_previously_assigned_states() -> Result<(), Failure> {
    let [first, second] = empty_operands();
    let mut builder = prepared_operation_builder()?;
    for first_state in STATES {
        for second_state in STATES {
            builder.merge_shapes(&first, first_state, &second, second_state)?;
            builder.merge_shapes(&first, second_state, &first, first_state)?;
            assert_operation_predicates(&builder, first_state, second_state);
        }
    }
    Ok(())
}

#[test]
fn preparation_and_clear_retain_states_but_kpart_assignment_replaces_them() -> Result<(), Failure> {
    let [first, second] = empty_operands();
    let mut builder = prepared_operation_builder()?;
    builder.merge_shapes(&first, State::Out, &second, State::In)?;
    builder.perform(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    assert_operation_predicates(&builder, State::Out, State::In);
    builder.clear()?;
    assert_operation_predicates(&builder, State::Out, State::In);
    builder.merge_k_part_states(State::In, State::Out)?;
    assert_operation_predicates(&builder, State::In, State::Out);
    Ok(())
}

#[path = "builder_operation_states/selection.rs"]
mod selection;
