//! Observe only bounded native waits; no timing or fairness guarantee is inferred (#89).
use winsafe::{GetLastError, SetLastError, co::ERROR};

#[test]
fn bounded_native_sleeps_match_wrapper_last_error() {
    for sentinel in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        compare_sleep(super::sec_sleep, 0, sentinel);
        compare_sleep(super::milli_sec_sleep, 0, sentinel);
        compare_sleep(super::milli_sec_sleep, 1, sentinel);
    }
}

fn compare_sleep(original: fn(i32), milliseconds: i32, incoming: ERROR) {
    let expected = capture_sleep_error(original, milliseconds, incoming);
    let observed = capture_sleep_error(
        |argument| winsafe::Sleep(argument as u32),
        milliseconds,
        incoming,
    );
    assert_eq!(observed, expected, "bounded sleep argument {milliseconds}");
}

fn capture_sleep_error(sleep: fn(i32), argument: i32, incoming: ERROR) -> ERROR {
    let previous = GetLastError();
    SetLastError(incoming);
    sleep(argument);
    let observed = GetLastError();
    SetLastError(previous);
    observed
}
