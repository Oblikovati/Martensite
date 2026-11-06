use super::*;

fn alias_pair() -> (TopOpeBRepBuildBuilder, TopOpeBRepBuildBuilder) {
    let mut first = builder();
    let mut second = builder();
    *first.extension_state_mut().created_vertices_mut() =
        BuilderResultArray::with_bounds(0, 1).unwrap();
    *second.extension_state_mut().created_vertices_mut() =
        first.extension_state().created_vertices().clone();
    assert!(
        first
            .extension_state()
            .created_vertices()
            .shares_storage_with(second.extension_state().created_vertices())
    );
    (first, second)
}

#[test]
fn alias_write_while_reading_returns_a_conflict_without_waiting() {
    let (first, mut second) = alias_pair();
    let first_read = first.new_vertex(0).unwrap();
    let second_read = second.new_vertex(0).unwrap();
    assert!(first_read.is_null() && second_read.is_null());
    drop(second_read);
    let failure = second.change_new_vertex(0).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert!(failure.message().contains("active reader or writer"));
    drop(first_read);
    *second.change_new_vertex(0).unwrap() = shape();
    assert!(!first.new_vertex(0).unwrap().is_null());
}

#[test]
fn alias_read_or_write_while_writing_returns_a_conflict_without_waiting() {
    let (mut first, mut second) = alias_pair();
    let write = first.change_new_vertex(0).unwrap();
    assert!(write.is_null());
    assert_eq!(
        second.new_vertex(0).unwrap_err().exception_type(),
        "Standard_ProgramError"
    );
    assert_eq!(
        second.change_new_vertex(1).unwrap_err().exception_type(),
        "Standard_ProgramError"
    );
    drop(write);
    assert!(second.new_vertex(0).unwrap().is_null());
}

#[test]
fn rebinding_one_owner_does_not_invalidate_another_owners_live_guard() {
    let (mut first, second) = alias_pair();
    let read = second.new_vertex(0).unwrap();
    first.extension_state_mut().created_vertices_mut().clear();
    assert!(first.extension_state().created_vertices().is_null());
    assert!(read.is_null());
    assert!(!second.extension_state().created_vertices().is_null());
}

#[test]
fn guarded_array_recovers_poison_without_replacing_storage() {
    let array = BuilderResultArray::<i32>::with_bounds(1, 1).unwrap();
    let alias = array.clone();
    let result = std::panic::catch_unwind(|| {
        let mut values = alias.write_array().unwrap();
        values.set_value(1, 7).unwrap();
        panic!("named test fault while holding a writer");
    });
    assert!(result.is_err());
    assert_eq!(*array.read_array().unwrap().value(1).unwrap(), 7);
    array.write_array().unwrap().set_value(1, 9).unwrap();
    assert_eq!(*alias.read_array().unwrap().value(1).unwrap(), 9);
}

#[test]
fn concurrent_alias_updates_report_contention_and_preserve_every_committed_value() {
    let array = BuilderResultArray::<i32>::with_bounds(0, 0).unwrap();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let alias = array.clone();
            scope.spawn(move || increment_with_explicit_retry(&alias, 500));
        }
    });
    assert_eq!(*array.read_array().unwrap().value(0).unwrap(), 2000);
}

fn increment_with_explicit_retry(array: &BuilderResultArray<i32>, count: usize) {
    for _ in 0..count {
        loop {
            match array.write_array() {
                Ok(mut values) => {
                    *values.change_value(0).unwrap() += 1;
                    break;
                }
                Err(failure) => {
                    assert_eq!(failure.exception_type(), "Standard_ProgramError");
                    std::thread::yield_now();
                }
            }
        }
    }
}
