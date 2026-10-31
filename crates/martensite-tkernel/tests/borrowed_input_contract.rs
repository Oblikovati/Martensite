//! Literal byte fixtures distinguish model outcomes from actual stream cleanup.
#[path = "borrowed_input_contract/support.rs"]
mod support;
use martensite_tkernel::osd::{IStream, SeekDir, StreamAccess, StreamState};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::storage::{
    BorrowedInputUnwind, IoStream, StreamRejoinCause, with_borrowed_input_stream,
};
use std::panic::{AssertUnwindSafe, catch_unwind, panic_any};
use std::sync::{Arc, atomic::Ordering};

#[derive(Debug)]
struct CallbackPanicked;

#[test]
fn initial_count_and_state_are_transferred_without_reset() {
    let (mut caller, _) = support::input(b"abcdef");
    assert_eq!(caller.read(&mut [0; 2]), 2);
    let before = caller.rdbuf();
    let observed =
        with_borrowed_input_stream(&mut caller, |stream| (stream.rdstate(), stream.gcount()))
            .unwrap();
    assert_eq!(observed, (0, 2));
    assert_eq!(caller.gcount(), 2);
    assert!(Arc::ptr_eq(&before, &caller.rdbuf()));
    let mut next = [0];
    caller.read(&mut next);
    assert_eq!(next, *b"c");
}

#[test]
fn unread_lookahead_rejoins_the_same_buffer_and_final_count() {
    let (mut caller, _) = support::input(b"abcdef");
    let value = with_borrowed_input_stream(&mut caller, |stream| {
        let mut value = [0];
        stream.read(&mut value);
        value[0]
    })
    .unwrap();
    assert_eq!(value, b'a');
    assert_eq!(caller.gcount(), 1);
    assert_eq!(caller.rdstate(), StreamState::GOOD);
    let mut next = [0; 2];
    assert_eq!(caller.read(&mut next), 2);
    assert_eq!(next, *b"bc");
}

#[test]
fn short_read_commits_actual_eof_fail_count_without_fake_operations() {
    let (mut caller, _) = support::input(b"abc");
    with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 5]);
    })
    .unwrap();
    assert_eq!(caller.gcount(), 3);
    assert_eq!(caller.rdstate().bits(), 6);
    assert_eq!(caller.read(&mut [9; 1]), 0);
    caller.clear();
    caller.seekg(0, SeekDir::Beg);
    let mut bytes = [0; 3];
    assert_eq!(caller.read(&mut bytes), 3);
    assert_eq!(bytes, *b"abc");
}

#[test]
fn an_initial_failure_is_not_silently_cleared_by_entry() {
    let (mut caller, _) = support::input(b"abc");
    caller.seekg(-1, SeekDir::Beg);
    with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 1]);
    })
    .unwrap();
    assert_eq!(caller.rdstate(), StreamState::FAIL);
    assert_eq!(caller.gcount(), 0);
    caller.clear();
    let mut first = [0];
    caller.read(&mut first);
    assert_eq!(first, *b"a");
}

#[test]
fn a_returned_failure_is_owned_unchanged_after_rejoin() {
    let (mut caller, _) = support::input(b"abcdef");
    let expected = Failure::with_type("Standard_Failure", "owned failure", "owned stack");
    let outcome = with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 2]);
        Err::<(), _>(expected.clone())
    })
    .unwrap();
    assert_eq!(outcome.unwrap_err(), expected);
    assert_eq!(caller.gcount(), 2);
    let mut next = [0];
    caller.read(&mut next);
    assert_eq!(next, *b"c");
}

#[test]
fn rewind_refusal_retains_the_returned_value_and_exact_unread_bytes() {
    let (mut caller, faults) = support::input(b"abcdef");
    let failure = with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 2]);
        faults.refuse_rewind.store(true, Ordering::SeqCst);
        17
    })
    .unwrap_err();
    assert_eq!(failure.outcome, Some(17));
    assert!(matches!(failure.cause, StreamRejoinCause::Seek { .. }));
    assert_eq!(failure.unread_bytes(), Some(b"cdef".as_slice()));
    assert!(caller.bad());
    assert_eq!(caller.gcount(), 2);
    assert!(!failure.observation.position_coherent);
}

#[test]
fn successful_cleanup_resumes_the_original_callback_panic() {
    let (mut caller, _) = support::input(b"abcdef");
    let payload = catch_unwind(AssertUnwindSafe(|| {
        let _ = with_borrowed_input_stream::<()>(&mut caller, |stream| {
            stream.read(&mut [0; 2]);
            panic_any(CallbackPanicked);
        });
    }))
    .unwrap_err();
    assert!(payload.is::<CallbackPanicked>());
    assert_eq!(caller.gcount(), 2);
    let mut next = [0];
    caller.read(&mut next);
    assert_eq!(next, *b"c");
}

#[test]
fn a_second_cleanup_panic_retains_both_owned_panic_causes() {
    let (mut caller, faults) = support::input(b"abcdef");
    let payload = catch_unwind(AssertUnwindSafe(|| {
        let _ = with_borrowed_input_stream::<()>(&mut caller, |stream| {
            stream.read(&mut [0; 2]);
            faults.panic_rewind.store(true, Ordering::SeqCst);
            panic_any(CallbackPanicked);
        });
    }))
    .unwrap_err();
    let retained = payload.downcast_ref::<BorrowedInputUnwind>().unwrap();
    assert!(retained.primary.is::<CallbackPanicked>());
    assert!(
        matches!(&retained.cleanup.cause, StreamRejoinCause::Panicked(cause) if cause.is::<support::RewindPanicked>())
    );
    assert_eq!(retained.cleanup.unread_bytes(), Some(b"cdef".as_slice()));
    assert!(caller.bad());
}

#[test]
fn read_panic_commits_failure_without_claiming_valid_unread_progress() {
    let (mut caller, faults) = support::input(b"abcdef");
    faults.panic_read.store(true, Ordering::SeqCst);
    let payload = catch_unwind(AssertUnwindSafe(|| {
        let _ = with_borrowed_input_stream::<()>(&mut caller, |stream| {
            stream.read(&mut [0; 1]);
        });
    }))
    .unwrap_err();
    let retained = payload.downcast_ref::<BorrowedInputUnwind>().unwrap();
    assert!(retained.primary.is::<support::ReadPanicked>());
    assert!(matches!(
        retained.cleanup.cause,
        StreamRejoinCause::ReadInterrupted
    ));
    assert_eq!(retained.cleanup.unread_bytes(), Some(b"".as_slice()));
    assert!(caller.bad());
}

#[test]
fn reopening_the_borrowed_stream_is_rejected_without_a_rollback_claim() {
    let (mut caller, _) = support::input(b"abcdef");
    let failure = with_borrowed_input_stream(&mut caller, |stream| {
        *stream = IoStream::from_bytes(b"different".to_vec());
        true
    })
    .unwrap_err();
    assert_eq!(failure.outcome, Some(true));
    assert!(matches!(failure.cause, StreamRejoinCause::DeviceReplaced));
    assert_eq!(failure.unread_bytes(), None);
    assert_eq!(failure.observation.gcount, None);
    assert!(caller.bad());
}

#[test]
fn returned_outcomes_need_neither_send_nor_static() {
    let (mut caller, _) = support::input(b"abc");
    let owned = std::rc::Rc::new(String::from("caller-owned"));
    let value = with_borrowed_input_stream(&mut caller, |_| std::rc::Rc::clone(&owned)).unwrap();
    assert!(std::rc::Rc::ptr_eq(&owned, &value));
    let borrowed = with_borrowed_input_stream(&mut caller, |_| owned.as_str()).unwrap();
    assert_eq!(borrowed, "caller-owned");
}

#[test]
fn cleanup_failure_retains_an_actual_returned_error() {
    let (mut caller, faults) = support::input(b"abcdef");
    let failure = with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 2]);
        faults.refuse_rewind.store(true, Ordering::SeqCst);
        Err::<(), _>(std::rc::Rc::new(String::from("decoder refused")))
    })
    .unwrap_err();
    assert_eq!(
        failure.outcome.unwrap().unwrap_err().as_str(),
        "decoder refused"
    );
    assert!(matches!(failure.cause, StreamRejoinCause::Seek { .. }));
    assert!(caller.bad());
}

#[test]
fn caught_read_panic_cannot_be_erased_by_clear_and_a_successful_retry() {
    let (mut caller, faults) = support::input(b"abcdef");
    faults.panic_read.store(true, Ordering::SeqCst);
    let failure = with_borrowed_input_stream(&mut caller, |stream| {
        assert_read_panic(stream);
        stream.clear(0);
        stream.read(&mut [0; 1]);
        stream.gcount()
    })
    .unwrap_err();
    assert!(matches!(failure.cause, StreamRejoinCause::ReadInterrupted));
    assert_eq!(failure.outcome, Some(1));
    assert_eq!(failure.observation.gcount, Some(1));
    assert_eq!(failure.unread_bytes(), Some(b"def".as_slice()));
    assert!(!failure.observation.position_coherent);
    assert!(caller.bad());
}

#[test]
fn a_refused_tell_rewind_cannot_discard_unread_bytes() {
    let (mut caller, faults) = support::input(b"abcdef");
    let failure = with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 2]);
        faults.refuse_rewind.store(true, Ordering::SeqCst);
        let position = stream.tellg();
        stream.clear(0);
        position
    })
    .unwrap_err();
    assert_eq!(failure.outcome, Some(-1));
    assert_eq!(failure.unread_bytes(), Some(b"cdef".as_slice()));
    assert!(matches!(failure.cause, StreamRejoinCause::Seek { .. }));
    assert_eq!(caller.gcount(), 2);
}

#[test]
fn a_refused_seek_rewind_retains_a_decoders_unread_bytes() {
    let (mut caller, faults) = support::input(b"abcdef");
    let failure = with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 2]);
        faults.refuse_rewind.store(true, Ordering::SeqCst);
        stream.seekg(0);
        stream.fail()
    })
    .unwrap_err();
    assert_eq!(failure.outcome, Some(true));
    assert_eq!(failure.unread_bytes(), Some(b"cdef".as_slice()));
    assert!(matches!(failure.cause, StreamRejoinCause::Seek { .. }));
    assert!(caller.bad());
}

#[test]
fn a_caught_positioning_panic_remains_a_sticky_session_failure() {
    let (mut caller, faults) = support::input(b"abcdef");
    let failure = with_borrowed_input_stream(&mut caller, |stream| {
        stream.read(&mut [0; 2]);
        faults.panic_rewind.store(true, Ordering::SeqCst);
        let panic = catch_unwind(AssertUnwindSafe(|| stream.tellg()));
        assert!(panic.unwrap_err().is::<support::RewindPanicked>());
        stream.clear(0);
    })
    .unwrap_err();
    assert!(matches!(
        failure.cause,
        StreamRejoinCause::SeekInterrupted { .. }
    ));
    assert_eq!(failure.unread_bytes(), Some(b"cdef".as_slice()));
    assert!(caller.bad());
}

fn assert_read_panic(stream: &mut IoStream) {
    let panic = catch_unwind(AssertUnwindSafe(|| {
        stream.read(&mut [0; 1]);
    }));
    assert!(panic.unwrap_err().is::<support::ReadPanicked>());
}

#[test]
fn bridge_observation_and_stage_have_shared_thread_bounds() {
    use martensite_tkernel::storage::{StreamBridgeObservation, StreamBridgeStage};
    let observation = StreamBridgeObservation {
        state: 0,
        gcount: Some(2),
        position_coherent: true,
    };
    let shared = Arc::new((StreamBridgeStage::Rejoin, observation));
    let transferred = Arc::clone(&shared);
    let received = std::thread::spawn(move || *transferred).join().unwrap();
    assert_eq!(received, *shared);
}
