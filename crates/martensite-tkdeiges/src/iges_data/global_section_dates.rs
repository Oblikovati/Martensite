//! The dates of the global section, as the file writes them and as they read.
use super::IgesDataGlobalSection;
use martensite_tkernel::{osd::Process, standard::Handle, tcollection::HAsciiString};

/// The system date as year, month, day, hour, minute, second.
pub(super) fn system_date() -> (i32, i32, i32, i32, i32, i32) {
    let mut process = Process::new();
    process.system_date().map_or((0, 0, 0, 0, 0, 0), |date| {
        let (month, day, year, hour, minute, second, _, _) = date.values();
        (year, month, day, hour, minute, second)
    })
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
