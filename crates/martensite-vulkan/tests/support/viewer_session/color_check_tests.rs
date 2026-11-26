use super::*;

#[test]
fn channel_tolerance_uses_integer_division() {
    assert!(!channel_off(100_000, 50_000));
    assert!(channel_off(100_000, 200_001));
    assert!(channel_off(0, 1));
    assert!(!channel_off(0, 0));
}
