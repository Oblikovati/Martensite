use super::{LegacyRecorder, Messenger};
use martensite_tkernel::message::Gravity;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};

pub fn assert_reentrant_records(original: &LegacyRecorder, replacement: &LegacyRecorder) {
    assert_eq!(
        *original.records.lock().unwrap(),
        vec![(b"outer".to_vec(), Gravity::Warning)]
    );
    assert_eq!(
        *replacement.records.lock().unwrap(),
        vec![
            (b"inner".to_vec(), Gravity::Warning),
            (b"next".to_vec(), Gravity::Warning)
        ]
    );
}

pub fn assert_text_overloads_refuse(messenger: &Messenger) {
    assert!(messenger.try_send_c_str(b"c", Gravity::Info).is_err());
    assert!(
        messenger
            .try_send_string_stream(b"s", Gravity::Info)
            .is_err()
    );
    assert!(
        messenger
            .try_send_ascii(&AsciiString::new("a"), Gravity::Info)
            .is_err()
    );
    assert!(
        messenger
            .try_send_extended(&ExtendedString::new("e"), Gravity::Info)
            .is_err()
    );
}

pub fn recorded_message_bytes(recorder: &LegacyRecorder) -> Vec<Vec<u8>> {
    recorder
        .records
        .lock()
        .unwrap()
        .iter()
        .map(|r| r.0.clone())
        .collect()
}
