use super::*;

fn selected_states(builder: &TopOpeBRepBuildBuilder) -> [State; 2] {
    [
        *builder.operation_state(1).expect("first operand"),
        *builder.operation_state(2).expect("second operand"),
    ]
}

fn state_addresses(builder: &TopOpeBRepBuildBuilder) -> [*const State; 2] {
    [
        builder.operation_state(1).expect("first operand"),
        builder.operation_state(2).expect("second operand"),
    ]
}

#[test]
fn live_operand_mutations_select_operations_without_replacing_the_other_operand()
-> Result<(), Failure> {
    let [first, second] = empty_operands();
    let mut builder = prepared_operation_builder()?;
    builder.merge_shapes(&first, State::In, &second, State::In)?;
    for first_state in STATES {
        *builder.change_operation_state(1).expect("first operand") = first_state;
        assert_eq!(selected_states(&builder)[0], first_state);
        for second_state in STATES {
            *builder.change_operation_state(2).expect("second operand") = second_state;
            assert_eq!(selected_states(&builder), [first_state, second_state]);
            assert_operation_predicates(&builder, first_state, second_state);
        }
    }
    Ok(())
}

#[test]
fn merge_assigns_the_same_storage_and_equal_operands_do_not_write_it() -> Result<(), Failure> {
    let [first, second] = empty_operands();
    let mut builder = prepared_operation_builder()?;
    builder.merge_shapes(&first, State::In, &second, State::Out)?;
    let addresses = state_addresses(&builder);
    for first_state in STATES {
        for second_state in STATES {
            builder.merge_shapes(&first, first_state, &second, second_state)?;
            assert_eq!(selected_states(&builder), [first_state, second_state]);
            assert_eq!(state_addresses(&builder), addresses);
            builder.merge_shapes(&first, second_state, &first, first_state)?;
            assert_eq!(selected_states(&builder), [first_state, second_state]);
        }
    }
    Ok(())
}

#[test]
fn state_borrows_keep_identity_across_mutation_preparation_and_kpart_assignment()
-> Result<(), Failure> {
    let [first, second] = empty_operands();
    let mut builder = prepared_operation_builder()?;
    builder.merge_shapes(&first, State::Out, &second, State::In)?;
    let addresses = state_addresses(&builder);
    let first_mut = builder.change_operation_state(1).expect("first operand") as *mut State;
    let second_mut = builder.change_operation_state(2).expect("second operand") as *mut State;
    assert_eq!(addresses, [first_mut.cast_const(), second_mut.cast_const()]);
    builder.perform(&Handle::new(TopOpeBRepDSHDataStructure::new()))?;
    builder.clear()?;
    assert_eq!(selected_states(&builder), [State::Out, State::In]);
    builder.merge_k_part_states(State::On, State::Unknown)?;
    assert_eq!(selected_states(&builder), [State::On, State::Unknown]);
    assert_eq!(state_addresses(&builder), addresses);
    Ok(())
}

#[test]
fn unsupported_operand_ranks_cannot_alias_or_mutate_either_state() -> Result<(), Failure> {
    let [first, second] = empty_operands();
    let mut builder = prepared_operation_builder()?;
    builder.merge_shapes(&first, State::Out, &second, State::In)?;
    for rank in [i32::MIN, -1, 0, 3, i32::MAX] {
        assert!(builder.operation_state(rank).is_none());
        assert!(builder.change_operation_state(rank).is_none());
        assert_eq!(selected_states(&builder), [State::Out, State::In]);
        assert_operation_predicates(&builder, State::Out, State::In);
    }
    Ok(())
}
