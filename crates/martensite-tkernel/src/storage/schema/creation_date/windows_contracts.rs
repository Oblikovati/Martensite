use super::{CreationDateSource, windows_calendar_date};
use winsafe::co::ERROR;

struct FixedCreationDate((u32, u32, i32));

impl CreationDateSource for FixedCreationDate {
    fn local_date(&self) -> (u32, u32, i32) {
        self.0
    }
}

#[test]
fn date_mapping_ignores_weekday_and_clock_fields() {
    for ignored in [0, 7, u16::MAX] {
        let fields = [2024, 2, ignored, 29, ignored, ignored, ignored, ignored];
        assert_eq!(windows_calendar_date(fields), (2, 29, 2024));
    }
}

#[test]
fn date_mapping_widens_without_normalization() {
    assert_eq!(windows_calendar_date([0; 8]), (0, 0, 0));
    assert_eq!(
        windows_calendar_date([u16::MAX; 8]),
        (65_535, 65_535, 65_535)
    );
}

#[test]
fn schema_date_format_keeps_minimum_width_without_truncating() {
    for (parts, expected) in [
        ((2, 29, 2024), "02/29/2024"),
        ((0, 0, 0), "00/00/0"),
        ((65_535, 65_535, 65_535), "65535/65535/65535"),
    ] {
        assert_eq!(
            FixedCreationDate(parts).creation_date().as_bytes(),
            expected.as_bytes()
        );
    }
}

#[test]
fn replacement_record_starts_with_all_fields_zero() {
    let now = winsafe::SYSTEMTIME::default();
    assert_eq!(
        [
            now.wYear,
            now.wMonth,
            now.wDayOfWeek,
            now.wDay,
            now.wHour,
            now.wMinute,
            now.wSecond,
            now.wMilliseconds,
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
fn native_schema_date_matches_wrapper_last_error() {
    for incoming in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        let original = observed_error(
            || {
                let _ = super::native_local_date();
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
