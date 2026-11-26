use super::require_restored_png;
use std::path::Path;

#[test]
fn restored_control_accepts_only_byte_identical_image() {
    let original = Path::new("original.png");
    let restored = Path::new("restored.png");
    assert!(require_restored_png(original, restored, b"png-bytes", b"png-bytes").is_ok());
    let error = require_restored_png(original, restored, b"png-bytes", b"changed").unwrap_err();
    let message = error.to_string();
    assert!(message.contains("restored.png"));
    assert!(message.contains("original.png"));
}
