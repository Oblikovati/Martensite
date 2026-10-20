//! Boundary normalization is restricted to intrinsic driver construction paths.
use super::native_stream_failure;
use martensite_tkernel::standard::Failure;

#[test]
fn intrinsic_stream_payload_normalization_preserves_class_and_raw_stack() {
    let inputs = [
        (
            "Storage_StreamTypeMismatchError",
            b"FSD_BinaryFile::GetInteger: expected more bytes in the file".as_slice(),
        ),
        (
            "Storage_StreamReadError",
            b"FSD_BinaryFile::ReadString: the stream ended inside a string".as_slice(),
        ),
        (
            "Storage_StreamReadError",
            b"FSD_BinaryFile::ReadExtendedString: the stream ended inside a string".as_slice(),
        ),
    ];
    for (class, message) in inputs {
        let error = native_stream_failure(Failure::with_type_bytes(class, message, b"stack\xff"));
        assert_eq!(error.exception_type(), class);
        assert!(error.what().is_empty());
        assert_eq!(error.stack_bytes(), b"stack\xff");
    }
}

#[test]
fn arbitrary_same_class_device_payload_is_never_normalized() {
    let original = Failure::with_type_bytes(
        "Storage_StreamReadError",
        b"FSD_BinaryFile::ReadString: callback\xff",
        b"owned\xffstack",
    );
    assert_eq!(native_stream_failure(original.clone()), original);
}

#[test]
fn identical_message_on_a_different_typed_cause_is_never_normalized() {
    let original = Failure::with_type_bytes(
        "Standard_DomainError",
        b"FSD_BinaryFile::GetInteger: expected more bytes in the file",
        b"stack",
    );
    assert_eq!(native_stream_failure(original.clone()), original);
}
