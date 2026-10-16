//! A clear token belongs to one invocation, not every future clear of the same sequence.
use martensite_tkernel::ncollection::{Sequence, SequenceClearProgress};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

struct CountedClearValue(Arc<AtomicUsize>);
impl Drop for CountedClearValue {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn stale_empty_token_cannot_finish_a_later_nonempty_clear() {
    let mut sequence = Sequence::<i32>::default();
    let stale = sequence.begin_clear_progress();
    sequence.clear(None);
    sequence.append(7);
    let mut current = sequence.begin_clear_progress();
    let error = sequence.finish_clear_progress(stale).expect_err(
        "a cancelled empty token must not finish a later clear and abandon its live nodes",
    );
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert_eq!(sequence.len(), 1);
    assert_eq!(sequence.visit_clear_node(&mut current).unwrap(), Some(&7));
    drop(sequence.detach_clear_node(&mut current).unwrap());
    sequence.finish_clear_progress(current).unwrap();
    assert!(sequence.is_empty());
}

#[test]
fn rejected_stale_finish_does_not_abandon_owned_values() {
    let dropped = Arc::new(AtomicUsize::new(0));
    let mut sequence = Sequence::<CountedClearValue>::default();
    let stale = sequence.begin_clear_progress();
    sequence.clear(None);
    sequence.append(CountedClearValue(dropped.clone()));
    let _current = sequence.begin_clear_progress();
    let result = sequence.finish_clear_progress(stale);
    drop(sequence);
    assert_eq!(
        dropped.load(Ordering::Relaxed),
        1,
        "the live value must be destroyed once, not abandoned"
    );
    assert!(result.is_err());
}

#[test]
fn stale_visit_and_detach_cannot_touch_the_current_invocation() {
    let mut sequence = Sequence::<i32>::default();
    let mut stale = sequence.begin_clear_progress();
    sequence.clear(None);
    sequence.append(9);
    let mut current = sequence.begin_clear_progress();
    assert!(sequence.visit_clear_node(&mut stale).is_err());
    assert!(sequence.detach_clear_node(&mut stale).is_err());
    assert_eq!(sequence.len(), 1);
    retire_current_value(&mut sequence, &mut current, 9);
    sequence.finish_clear_progress(current).unwrap();
    assert!(sequence.is_empty());
}

fn retire_current_value(
    sequence: &mut Sequence<i32>,
    progress: &mut SequenceClearProgress,
    expected: i32,
) {
    assert_eq!(
        sequence.visit_clear_node(progress).unwrap(),
        Some(&expected)
    );
    drop(sequence.detach_clear_node(progress).unwrap());
}

#[test]
fn cancelled_tokens_remain_invalid_across_repeated_empty_clears() {
    let mut sequence = Sequence::<i32>::default();
    for _ in 0..8 {
        let mut stale = sequence.begin_clear_progress();
        sequence.clear(None);
        assert!(sequence.visit_clear_node(&mut stale).is_err());
        let mut current = sequence.begin_clear_progress();
        assert!(sequence.finish_clear_progress(stale).is_err());
        assert!(sequence.visit_clear_node(&mut current).unwrap().is_none());
        sequence.finish_clear_progress(current).unwrap();
        assert!(sequence.is_empty());
    }
}

#[test]
fn cancelled_nonempty_token_cannot_retire_a_later_invocations_node() {
    let mut sequence: Sequence<i32> = [1, 2].into_iter().collect();
    let mut stale = sequence.begin_clear_progress();
    assert_eq!(sequence.visit_clear_node(&mut stale).unwrap(), Some(&1));
    sequence.clear(None);
    sequence.append(3);
    let mut current = sequence.begin_clear_progress();
    assert!(sequence.detach_clear_node(&mut stale).is_err());
    assert!(sequence.finish_clear_progress(stale).is_err());
    assert_eq!(sequence.len(), 1);
    retire_current_value(&mut sequence, &mut current, 3);
    sequence.finish_clear_progress(current).unwrap();
}

#[test]
fn current_token_can_complete_after_the_sequence_moves() {
    let mut sequence: Sequence<i32> = [4, 5].into_iter().collect();
    let mut current = sequence.begin_clear_progress();
    let mut moved = sequence;
    retire_current_value(&mut moved, &mut current, 4);
    retire_current_value(&mut moved, &mut current, 5);
    moved.finish_clear_progress(current).unwrap();
    assert!(moved.is_empty());
}
