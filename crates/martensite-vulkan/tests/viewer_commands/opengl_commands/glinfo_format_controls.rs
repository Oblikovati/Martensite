use super::*;
#[test]
fn diagnostic_entry_preserves_short_and_long_real_values() {
    assert_eq!(
        format_entry("GPU", "actual device", 80),
        "  GPU: actual device"
    );
    assert_eq!(
        format_entry("GPU", "actual  device family", 15),
        "  GPU:\n    actual\n    device\n    family"
    );
    assert_eq!(format_entry("LongKey", "", 1), "  LongKey:");
}
