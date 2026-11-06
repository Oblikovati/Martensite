//! The verification of the global section of a model against the norm.
use super::{IgesDataBasicEditor, IgesDataGlobalSection, iges_msg};
use martensite_tkernel::{standard::Handle, tcollection::HAsciiString};
use martensite_tkxsbase::interface::InterfaceCheck;

fn send_fail(check: &InterfaceCheck, key: &str) {
    check.send_fail(&iges_msg(key));
}

/// Whether a character can be a delimiter of the file.
fn bad_delimiter(del: u8) -> bool {
    del <= 32 || matches!(del, 43 | 45 | 46 | 48..=57 | 68 | 69 | 72) || del >= 127
}

pub(super) fn verify_header(h: &IgesDataGlobalSection, check: &InterfaceCheck) {
    if h.separator() == h.end_mark() {
        send_fail(check, "XSTEP_40");
    }
    if bad_delimiter(h.separator()) {
        send_fail(check, "XSTEP_41");
    }
    if bad_delimiter(h.end_mark()) {
        send_fail(check, "XSTEP_42");
    }
    for (value, key) in [
        (h.max_power10_single(), "XSTEP_43"),
        (h.max_digits_single(), "XSTEP_44"),
        (h.max_power10_double(), "XSTEP_45"),
        (h.max_digits_double(), "XSTEP_46"),
    ] {
        if value <= 0 {
            send_fail(check, key);
        }
    }
    if h.scale() <= 0.0 {
        send_fail(check, "XSTEP_47");
    }
    verify_unit(h, check);
    verify_date(&h.date(), check, 18);
    if h.resolution() <= 0.0 {
        send_fail(check, "XSTEP_52");
    }
    if h.iges_version() < 1 || h.iges_version() > IgesDataBasicEditor::iges_version_max() {
        check.send_warning(&iges_msg("XSTEP_53"));
    }
    if h.drafting_standard() < 0 || h.drafting_standard() > IgesDataBasicEditor::drafting_max() {
        check.send_warning(&iges_msg("XSTEP_54"));
    }
    if h.iges_version() >= 9 {
        if h.has_last_change_date() {
            verify_date(&h.last_change_date(), check, 25);
        } else {
            check.send_warning(&iges_msg("XSTEP_55"));
        }
    }
}

fn verify_unit(h: &IgesDataGlobalSection, check: &InterfaceCheck) {
    let unf = h.unit_flag();
    if !(1..=11).contains(&unf) {
        send_fail(check, "XSTEP_48");
    }
    let name = h.unit_name();
    let Some(live) = name.get() else {
        if unf == 3 {
            send_fail(check, "XSTEP_50");
        }
        return;
    };
    let unm = live.string();
    let unm = unm.as_bytes();
    let unok = match unf {
        1 => unm == b"IN" || unm == b"INCH",
        2 => unm == b"MM",
        3 => true,
        4 => unm == b"FT",
        5 => unm == b"MI",
        6 => unm == b"M",
        7 => unm == b"KM",
        8 => unm == b"MIL",
        9 => unm == b"UM",
        10 => unm == b"CM",
        11 => unm == b"UIN",
        _ => {
            send_fail(check, "XSTEP_48");
            true
        }
    };
    if !unok {
        send_fail(check, "XSTEP_51");
    }
}

/// Checks the form of a date parameter, numbered `param` in the section.
fn verify_date(date: &Handle<HAsciiString>, check: &InterfaceCheck, param: i32) {
    let mut msg = iges_msg("XSTEP_57");
    let Some(live) = date.get() else {
        check.send_fail(&msg);
        return;
    };
    let _no_more_format_is_fine = msg.arg_integer(param);
    if date_is_wrong(live) {
        check.send_fail(&msg);
    }
}

fn date_is_wrong(date: &HAsciiString) -> bool {
    let content = date.string();
    let text = content.as_bytes();
    let (short, long) = (text.len() == 13, text.len() == 15);
    if (!short && !long) || !content.is_real_value(true) {
        return true;
    }
    let at = |rank: usize| text[rank - 1];
    // The fields of the time and date, `shift` further for the long form of the year.
    let shift = if long { 2 } else { 0 };
    let over = |rank: usize, tens: u8, units: u8| {
        at(rank + shift) > tens || (at(rank + shift) == tens && at(rank + shift + 1) > units)
    };
    over(3, b'1', b'2')
        || over(5, b'3', b'1')
        || at(7 + shift) != b'.'
        || at(10 + shift) > b'5'
        || at(12 + shift) > b'5'
        || over(8, b'2', b'3')
}

#[cfg(test)]
mod tests {
    use super::{HAsciiString, Handle, InterfaceCheck, verify_date};

    fn fails_of(text: &str) -> i32 {
        let check = InterfaceCheck::new();
        verify_date(&Handle::new(HAsciiString::new(text)), &check, 18);
        check.nb_fails()
    }

    #[test]
    fn dates_of_both_forms() {
        assert_eq!(fails_of("190102.120000"), 0);
        assert_eq!(fails_of("20190102.120000"), 0);
        assert_eq!(fails_of("191302.120000"), 1);
        assert_eq!(fails_of("190132.120000"), 1);
        assert_eq!(fails_of("190102.240000"), 1);
        assert_eq!(fails_of("190102.126000"), 1);
        assert_eq!(fails_of("20191302.120000"), 1);
        assert_eq!(fails_of("19010"), 1);
    }
}
