use super::*;

#[test]
fn sequence_switch_accepts_unset_or_literal_one() {
    assert!(validate_sequence_flag(None).is_ok());
    assert!(validate_sequence_flag(Some(OsStr::new("1"))).is_ok());
}

#[test]
fn sequence_switch_rejects_other_values() {
    for value in ["", "0", "true", "2"] {
        let error = validate_sequence_flag(Some(OsStr::new(value))).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(error.to_string().contains(value));
    }
}
