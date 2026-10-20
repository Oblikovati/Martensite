use super::fake_input::FakeProbeInput;
use martensite_tkcdf::pcdm::PcdmReadWriterBase;
use martensite_tkernel::{standard::Handle, storage::Data};

#[test]
fn unknown_input_consumes_at_most_seven_bytes_and_preserves_storage_identity() {
    for length in 0..=12 {
        let mut input = FakeProbeInput::unknown(length);
        let original = Handle::new(Data::default());
        let mut storage = original.clone();
        assert!(PcdmReadWriterBase::file_format_stream(&mut input, &mut storage).is_empty());
        assert_eq!(input.consumed, length.min(7));
        assert_eq!(input.calls, (length + 1).min(7));
        assert!(storage == original);
    }
}

#[test]
fn default_exception_mask_stops_on_the_first_device_failure() {
    for fail_at in 0..7 {
        let mut input = FakeProbeInput::unknown(12);
        input.fail_at = Some(fail_at);
        let mut storage = Handle::default();
        assert!(PcdmReadWriterBase::file_format_stream(&mut input, &mut storage).is_empty());
        assert_eq!((input.consumed, input.calls), (fail_at, fail_at + 1));
        assert!(storage.is_null());
    }
}
