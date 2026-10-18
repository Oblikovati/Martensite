//! Owned expected message records and explicit callback failure contracts.
#[path = "fallible_delivery/support.rs"]
mod support;
use martensite_tkernel::message::{Gravity, Messenger, Printer, PrinterHandle};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use std::sync::Arc;
use support::{LegacyRecorder, ReentrantEditor, RefusingPrinter};

#[test]
fn existing_printers_delegate_bytes_with_gravity() {
    let printer = LegacyRecorder::default();
    printer
        .try_send(&AsciiString::new("raw"), Gravity::Trace)
        .unwrap();
    printer
        .try_send_ascii(&AsciiString::from_bytes(b"ascii\0tail"), Gravity::Info)
        .unwrap();
    assert_eq!(
        *printer.records.lock().unwrap(),
        vec![
            (b"raw".to_vec(), Gravity::Trace),
            (b"ascii".to_vec(), Gravity::Info)
        ]
    );
}

#[test]
fn extended_delivery_converts_explicit_code_units_to_utf8() {
    let printer = LegacyRecorder::default();
    let text = ExtendedString::from_utf16(&[0x63, 0x61, 0x66, 0xe9]);
    printer.try_send_extended(&text, Gravity::Warning).unwrap();
    assert_eq!(
        *printer.records.lock().unwrap(),
        vec![(vec![0x63, 0x61, 0x66, 0xc3, 0xa9], Gravity::Warning)]
    );
}

#[test]
fn byte_and_stream_delivery_stop_at_nul() {
    let printer = LegacyRecorder::default();
    printer
        .try_send_c_str(b"bytes\0tail", Gravity::Alarm)
        .unwrap();
    printer
        .try_send_string_stream(b"stream\0tail", Gravity::Fail)
        .unwrap();
    assert_eq!(
        *printer.records.lock().unwrap(),
        vec![
            (b"bytes".to_vec(), Gravity::Alarm),
            (b"stream".to_vec(), Gravity::Fail)
        ]
    );
}

#[test]
fn filtered_callbacks_do_not_attempt_fallible_delivery() {
    let printer = RefusingPrinter::new();
    printer.set_trace_level(Gravity::Fail);
    printer
        .try_send_ascii(&AsciiString::new("x"), Gravity::Info)
        .unwrap();
    printer
        .try_send_extended(&ExtendedString::new("x"), Gravity::Info)
        .unwrap();
    printer.try_send_c_str(b"x", Gravity::Info).unwrap();
    printer.try_send_string_stream(b"x", Gravity::Info).unwrap();
    printer
        .try_send_object(&Handle::default(), Gravity::Fail)
        .unwrap();
    assert!(printer.attempts.lock().unwrap().is_empty());
}

#[test]
fn typed_failure_preserves_payload_and_stops_snapshot() {
    let first = Arc::new(LegacyRecorder::default());
    let refusal = Arc::new(RefusingPrinter::new());
    let last = Arc::new(LegacyRecorder::default());
    let messenger = Messenger::with_printer(first.clone());
    messenger.add_printer(refusal.clone());
    messenger.add_printer(last.clone());
    let failure = messenger.try_send("owned message").unwrap_err();
    assert_eq!(failure, refusal.failure);
    assert_eq!(failure.exception_type(), "DeliveryRefused");
    assert_eq!(failure.stack_string(), "owned stack");
    assert_eq!(first.records.lock().unwrap().len(), 1);
    assert_eq!(refusal.attempts.lock().unwrap().len(), 1);
    assert!(last.records.lock().unwrap().is_empty());
}

#[test]
fn infallible_path_keeps_original_dispatch_after_extension() {
    let refusal = Arc::new(RefusingPrinter::new());
    let last = Arc::new(LegacyRecorder::default());
    let messenger = Messenger::with_printer(refusal.clone());
    messenger.add_printer(last.clone());
    messenger.send("ordinary");
    assert!(refusal.attempts.lock().unwrap().is_empty());
    assert_eq!(refusal.ordinary.records.lock().unwrap().len(), 1);
    assert_eq!(last.records.lock().unwrap().len(), 1);
}

#[test]
fn callback_edits_affect_next_snapshot_and_allow_nested_delivery() {
    let replacement = Arc::new(LegacyRecorder::default());
    let editor = Arc::new(ReentrantEditor::new(replacement.clone()));
    let original = Arc::new(LegacyRecorder::default());
    let messenger = Arc::new(Messenger::with_printer(editor.clone()));
    *editor.target.lock().unwrap() = Arc::downgrade(&messenger);
    messenger.add_printer(original.clone());
    messenger.try_send("outer").unwrap();
    messenger.try_send("next").unwrap();
    support::assert_reentrant_records(&original, &replacement);
}

#[test]
fn messenger_overloads_share_failure_channel_and_object_filtering() {
    let refusal = Arc::new(RefusingPrinter::new());
    let messenger = Messenger::with_printer(refusal.clone());
    support::assert_text_overloads_refuse(&messenger);
    messenger
        .try_send_object(&Handle::default(), Gravity::Fail)
        .unwrap();
    let object = Handle::<StandardTransient>::new(StandardTransient::default());
    assert!(messenger.try_send_object(&object, Gravity::Info).is_err());
    refusal.set_trace_level(Gravity::Fail);
    messenger.try_send_object(&object, Gravity::Info).unwrap();
    assert_eq!(refusal.attempts.lock().unwrap().len(), 5);
}

#[test]
fn independently_owned_messengers_and_failure_copies_stay_isolated() {
    let first = Arc::new(LegacyRecorder::default());
    let second = Arc::new(LegacyRecorder::default());
    let one = Messenger::with_printer(first.clone());
    let two = Messenger::with_printer(second.clone());
    one.try_send("one").unwrap();
    two.try_send("two").unwrap();
    assert_eq!(first.records.lock().unwrap()[0].0, b"one");
    assert_eq!(second.records.lock().unwrap()[0].0, b"two");
    let mut returned = RefusingPrinter::new().failure;
    let retained = returned.clone();
    returned.set_message("changed");
    assert_eq!(
        retained.message(),
        "sink quota 7, expected available capacity"
    );
}

#[test]
fn delivery_public_types_are_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<Messenger>();
    require_send_sync::<PrinterHandle>();
    require_send_sync::<Failure>();
    require_send_sync::<RefusingPrinter>();
}

#[test]
fn empty_snapshot_and_filtered_failure_leave_no_attempts() {
    let refusal = Arc::new(RefusingPrinter::new());
    refusal.set_trace_level(Gravity::Fail);
    let messenger = Messenger::with_printer(refusal.clone());
    messenger.try_send("filtered").unwrap();
    assert!(refusal.attempts.lock().unwrap().is_empty());
    messenger.printers_mut().clear();
    messenger.try_send("empty").unwrap();
    assert!(refusal.attempts.lock().unwrap().is_empty());
    assert!(
        refusal
            .try_send(&AsciiString::new("unfiltered callback"), Gravity::Trace)
            .is_err()
    );
}

#[test]
fn concurrent_callers_keep_owned_delivery_records() {
    let recorder = Arc::new(LegacyRecorder::default());
    let messenger = Arc::new(Messenger::with_printer(recorder.clone()));
    support::send_concurrent_workload(&messenger);
    let mut actual = support::recorded_message_bytes(&recorder);
    let mut expected: Vec<Vec<u8>> = (0..4)
        .flat_map(|w| (0..1000).map(move |m| format!("{w}:{m}").into_bytes()))
        .collect();
    actual.sort();
    expected.sort();
    assert_eq!(actual, expected);
}
