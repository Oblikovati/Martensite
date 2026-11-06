//! The defaults AP203 requires when the caller gave none: a person, an organization and a date.
//!
//! They are constants, on purpose: a written file does not carry the user's login or the machine's name, and a library has
//! no business reading the account or zone files. A caller who wants real names sets them with
//! `set_default_person_and_organization`. The date is the system clock in UTC, so the offset of zero
//! it states is true whatever the host's zone.
use crate::step_basic::{
    StepBasicAheadOrBehind, StepBasicCalendarDate, StepBasicCoordinatedUniversalTimeOffset,
    StepBasicDateAndTime, StepBasicLocalTime, StepBasicOrganization, StepBasicPerson,
    StepBasicPersonAndOrganization,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use std::time::{SystemTime, UNIX_EPOCH};

/// The identifier, name and first name of the default person and organization.
const DEFAULT_IDENTITY: &str = "Martensite";

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// The stated default time zone: an offset of zero, whatever the host's zone.
fn default_time_zone() -> Handle<StepBasicCoordinatedUniversalTimeOffset> {
    let zone = Handle::new(StepBasicCoordinatedUniversalTimeOffset::new());
    zone.init(0, false, 0, StepBasicAheadOrBehind::AobExact);
    zone
}

/// A moment as the fields of a STEP date and time, in UTC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UtcDateTime {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

/// The UTC date and time of a count of seconds since 1970-01-01, by the civil calendar's 400-year
/// cycle counted from a March 1st, so that a leap day is the last day of its cycle year.
/// Example: `utc_date_time(0)` is 1970-01-01 00:00.
fn utc_date_time(unix_seconds: u64) -> UtcDateTime {
    const DAYS_IN_ERA: u64 = 146_097;
    /// From 0000-03-01 to 1970-01-01.
    const EPOCH_IN_DAYS: u64 = 719_468;
    let days = unix_seconds / 86_400 + EPOCH_IN_DAYS;
    let (era, day_of_era) = (days / DAYS_IN_ERA, days % DAYS_IN_ERA);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    let seconds_of_day = unix_seconds % 86_400;
    UtcDateTime {
        year: year as i32,
        month: month as i32,
        day: (day_of_year - (153 * shifted_month + 2) / 5 + 1) as i32,
        hour: (seconds_of_day / 3600) as i32,
        minute: (seconds_of_day % 3600 / 60) as i32,
    }
}

/// The system clock in UTC.
fn utc_now() -> Result<UtcDateTime, Failure> {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| {
            martensite_tkernel::standard::exceptions::program_error(&format!(
                "the system clock is before 1970-01-01: {error:?}"
            ))
        })?;
    Ok(utc_date_time(since_epoch.as_secs()))
}

/// The present date and time in UTC, the default time zone it states.
pub(super) fn new_date_and_time() -> Result<Handle<StepBasicDateAndTime>, Failure> {
    let date = utc_now()?;
    let calendar = Handle::new(StepBasicCalendarDate::new());
    calendar.init(date.year, date.day, date.month);
    let time = Handle::new(StepBasicLocalTime::new());
    time.init(
        date.hour,
        true,
        date.minute,
        false,
        0.,
        &default_time_zone(),
    );
    let date_time = Handle::new(StepBasicDateAndTime::new());
    date_time.init(&calendar.cast(), &time);
    Ok(date_time)
}

/// The default person and organization.
pub(super) fn new_person_and_organization() -> Handle<StepBasicPersonAndOrganization> {
    let organization = Handle::new(StepBasicOrganization::new());
    organization.init(
        true,
        &text(DEFAULT_IDENTITY),
        &text(DEFAULT_IDENTITY),
        &text(""),
    );
    let person = Handle::new(StepBasicPerson::new());
    person.init(
        &text(DEFAULT_IDENTITY),
        true,
        &text(""),
        true,
        &text(DEFAULT_IDENTITY),
        false,
        &Handle::default(),
        false,
        &Handle::default(),
        false,
        &Handle::default(),
    );
    let both = Handle::new(StepBasicPersonAndOrganization::new());
    both.init(&person, &organization);
    both
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_person_and_organization_are_constants() {
        let both = new_person_and_organization();
        let person = both.the_person();
        let organization = both.the_organization();
        assert_eq!(person.id().string().as_bytes(), b"Martensite");
        assert_eq!(person.first_name().string().as_bytes(), b"Martensite");
        assert_eq!(person.last_name().string().as_bytes(), b"");
        assert_eq!(organization.id().string().as_bytes(), b"Martensite");
        assert_eq!(organization.name().string().as_bytes(), b"Martensite");
    }

    #[test]
    fn the_default_time_zone_is_stated_whatever_the_host() {
        let date = new_date_and_time().unwrap();
        let zone = date.time_component().zone();
        assert_eq!(zone.hour_offset(), 0);
        assert!(!zone.has_minute_offset());
        assert_eq!(zone.sense(), StepBasicAheadOrBehind::AobExact);
    }

    #[test]
    fn a_moment_splits_into_its_utc_date_and_time() {
        let date_time = |year, month, day, hour, minute| UtcDateTime {
            year,
            month,
            day,
            hour,
            minute,
        };
        assert_eq!(utc_date_time(0), date_time(1970, 1, 1, 0, 0));
        assert_eq!(utc_date_time(951_827_696), date_time(2000, 2, 29, 12, 34));
        assert_eq!(
            utc_date_time(1_798_761_599),
            date_time(2026, 12, 31, 23, 59)
        );
        assert_eq!(utc_date_time(4_107_542_400), date_time(2100, 3, 1, 0, 0));
    }

    #[test]
    fn the_default_date_is_now_in_utc() {
        let before = utc_now().unwrap();
        let date = new_date_and_time().unwrap();
        let after = utc_now().unwrap();
        let (calendar, time) = (date.date_component(), date.time_component());
        let written = (time.hour_component(), time.minute_component());
        assert!(
            written == (before.hour, before.minute) || written == (after.hour, after.minute),
            "written {written:?}, expected {before:?} or {after:?}"
        );
        assert!([before.year, after.year].contains(&calendar.year_component()));
    }
}
