//! The dates of the global section, as the file writes them and as they read.
use super::IgesDataGlobalSection;
use martensite_tkernel::{standard::Handle, tcollection::HAsciiString};
use std::time::{SystemTime, UNIX_EPOCH};

/// The system clock in UTC as year, month, day, hour, minute, second, so that a file does not say
/// where it was written. A clock before 1970 gives zeros, the form of an unknown date.
pub(crate) fn system_date() -> (i32, i32, i32, i32, i32, i32) {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or((0, 0, 0, 0, 0, 0), |since| utc_fields(since.as_secs()))
}

/// Year, month, day, hour, minute, second of a count of seconds since 1970-01-01, by the civil
/// calendar's 400-year cycle counted from a March 1st, so that a leap day ends its cycle year.
/// Example: `utc_fields(0)` is `(1970, 1, 1, 0, 0, 0)`.
fn utc_fields(unix_seconds: u64) -> (i32, i32, i32, i32, i32, i32) {
    const DAYS_IN_ERA: u64 = 146_097;
    /// From 0000-03-01 to 1970-01-01.
    const EPOCH_IN_DAYS: u64 = 719_468;
    let days = unix_seconds / 86_400 + EPOCH_IN_DAYS;
    let (era, of_era) = (days / DAYS_IN_ERA, days % DAYS_IN_ERA);
    let year_of_era = (of_era - of_era / 1460 + of_era / 36_524 - of_era / 146_096) / 365;
    let of_year = of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let from_march = (5 * of_year + 2) / 153;
    let month = if from_march < 10 {
        from_march + 3
    } else {
        from_march - 9
    };
    let of_day = unix_seconds % 86_400;
    (
        (year_of_era + era * 400 + u64::from(month <= 2)) as i32,
        month as i32,
        (of_year - (153 * from_march + 2) / 5 + 1) as i32,
        (of_day / 3600) as i32,
        (of_day % 3600 / 60) as i32,
        (of_day % 60) as i32,
    )
}

impl IgesDataGlobalSection {
    /// A date as text. `mode` 0 gives `YYMMDD.HHNNSS`, -1 `YYYYMMDD.HHNNSS`, 1 the readable
    /// `YYYY-MM-DD:HH-NN-SS`; a year of 0 takes the system date.
    pub fn new_date_string(
        annee: i32,
        mois: i32,
        jour: i32,
        heure: i32,
        minute: i32,
        seconde: i32,
        mode: i32,
    ) -> Handle<HAsciiString> {
        let (anne, moi, jou, heur, minut, second) = if annee == 0 {
            system_date()
        } else {
            (annee, mois, jour, heure, minute, seconde)
        };
        let text = match mode {
            0 | -1 => iges_date(anne, moi, jou, heur, minut, second, mode),
            1 => {
                format!("{anne:04}-{moi:02}-{jou:02}:{heur:02}-{minut:02}-{second:02}").into_bytes()
            }
            _ => Vec::new(),
        };
        Handle::new(HAsciiString::new(text))
    }
    /// A date of the file form in another form; `date` itself when it is not of the form.
    pub fn new_date_string_from(date: &Handle<HAsciiString>, mode: i32) -> Handle<HAsciiString> {
        let Some(text) = date.get() else {
            return date.clone();
        };
        let content = text.string();
        let digits = content.as_bytes();
        let i0 = match digits.len() {
            15 => 2,
            13 => 0,
            _ => return date.clone(),
        };
        let at = |rank: usize| i32::from(digits[rank - 1]) - 48;
        if digits[i0 + 6] != b'.' {
            return date.clone();
        }
        let mut anne = at(i0 + 1) * 10 + at(i0 + 2);
        if i0 == 0 {
            anne += 1900;
            if anne < 1980 {
                anne += 100;
            }
        } else {
            anne += at(1) * 1000 + at(2) * 100;
        }
        Self::new_date_string(
            anne,
            at(i0 + 3) * 10 + at(i0 + 4),
            at(i0 + 5) * 10 + at(i0 + 6),
            at(i0 + 8) * 10 + at(i0 + 9),
            at(i0 + 10) * 10 + at(i0 + 11),
            at(i0 + 12) * 10 + at(i0 + 13),
            mode,
        )
    }
}

/// The `YYMMDD.HHNNSS` or `YYYYMMDD.HHNNSS` form of a date. A year below ten is written by adding
/// ten and overwriting the first digit, so that its leading zero stays.
fn iges_date(
    anne: i32,
    moi: i32,
    jou: i32,
    heur: i32,
    minut: i32,
    second: i32,
    mode: i32,
) -> Vec<u8> {
    let mut an = anne % 100;
    let mut dizaine = an >= 10;
    if !dizaine {
        an += 10;
    }
    if mode < 0 {
        an = anne;
        dizaine = true;
    }
    let date1 = an * 10000 + moi * 100 + jou;
    let date2 = (heur + 100) * 10000 + minut * 100 + second;
    let mut text = format!("{date1}{date2}").into_bytes();
    let point = if mode == 0 { 6 } else { 8 };
    if let Some(slot) = text.get_mut(point) {
        *slot = b'.';
    }
    if !dizaine {
        text[0] = b'0';
    }
    text
}

#[cfg(test)]
mod tests {
    use super::utc_fields;

    #[test]
    fn seconds_since_1970_are_a_civil_date_in_utc() {
        assert_eq!(utc_fields(0), (1970, 1, 1, 0, 0, 0));
        assert_eq!(utc_fields(951_827_696), (2000, 2, 29, 12, 34, 56));
        assert_eq!(utc_fields(1_798_761_599), (2026, 12, 31, 23, 59, 59));
        assert_eq!(utc_fields(4_107_542_400), (2100, 3, 1, 0, 0, 0));
    }
}
