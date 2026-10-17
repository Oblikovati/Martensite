use super::{LocalTime, system_time_local_time};
use winsafe::co::ERROR;

fn fields(value: LocalTime) -> [i32; 7] {
    [
        value.year,
        value.month,
        value.day,
        value.hour,
        value.minute,
        value.second,
        value.millisecond,
    ]
}

#[test]
fn system_fields_preserve_milliseconds_and_ignore_weekday() {
    for weekday in [0, 6, u16::MAX] {
        let actual = system_time_local_time([2024, 2, weekday, 29, 13, 5, 59, 987]);
        assert_eq!(fields(actual), [2024, 2, 29, 13, 5, 59, 987]);
    }
}

#[test]
fn system_fields_widen_without_normalization() {
    assert_eq!(fields(system_time_local_time([0; 8])), [0; 7]);
    assert_eq!(fields(system_time_local_time([u16::MAX; 8])), [65_535; 7]);
}

#[test]
fn wrapper_system_time_default_initializes_every_field() {
    let clock = winsafe::SYSTEMTIME::default();
    assert_eq!(
        [
            clock.wYear,
            clock.wMonth,
            clock.wDayOfWeek,
            clock.wDay,
            clock.wHour,
            clock.wMinute,
            clock.wSecond,
            clock.wMilliseconds,
        ],
        [0; 8]
    );
}

fn observed_error(call: impl FnOnce(), incoming: ERROR) -> ERROR {
    let saved = winsafe::GetLastError();
    winsafe::SetLastError(incoming);
    call();
    let observed = winsafe::GetLastError();
    winsafe::SetLastError(saved);
    observed
}

#[test]
fn native_local_time_matches_wrapper_last_error_observations() {
    for incoming in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        let original = observed_error(
            || {
                let _ = super::local_time();
            },
            incoming,
        );
        let wrapper = observed_error(
            || {
                let _ = winsafe::GetLocalTime();
            },
            incoming,
        );
        assert_eq!(original, wrapper, "incoming LastError: {incoming:?}");
    }
}
