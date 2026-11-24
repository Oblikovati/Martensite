//! Default handle key comparison over full string spans.
use crate::ncollection::{DefaultHashPolicy, HashPolicy};
use crate::standard::Handle;
use crate::tcollection::{AsciiString, ExtendedString, HAsciiString, HExtendedString};

#[test]
fn default_string_handle_keys_compare_beyond_embedded_nuls() {
    let first = Handle::new(HAsciiString::from(AsciiString::from_raw(b"a\0b")));
    let second = Handle::new(HAsciiString::from(AsciiString::from_raw(b"a\0c")));
    assert!(first.is_same_state(&second));
    assert!(!DefaultHashPolicy.equal(&first, &second));
    let first = Handle::new(HExtendedString::from(ExtendedString::from_raw(&[
        65, 0, 66,
    ])));
    let second = Handle::new(HExtendedString::from(ExtendedString::from_raw(&[
        65, 0, 67,
    ])));
    assert!(!DefaultHashPolicy.equal(&first, &second));
}
