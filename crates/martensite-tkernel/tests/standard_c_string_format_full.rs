//! The glibc printf features the C-string formatter implements on Linux: numbered arguments,
//! wide characters, `%m`, glibc length aliases, `%b`, `long double`, C-locale flags and
//! `%n`, against the reviewed glibc baseline in the crate fixtures.
use martensite_tkernel::standard::c_string::{FormatArg, fprintf_to, printf, sprintf_into};
use std::cell::Cell;
use std::io::ErrorKind;

#[cfg(all(target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]
mod native {
    use super::*;
    use FormatArg::{Int, LongDouble as Ld, Pointer, Real, Text, WideChar, WideText};
    use martensite_tkernel::standard::c_string::sprintf_into_slice;

    const ROWS: &str = include_str!("fixtures/tkernel_c_string_format_full.expected");

    /// `(count, errno, bytes)` of a fixture row; count -1 has no bytes.
    fn row(label: &str) -> (i64, i32, Vec<u8>) {
        let line = ROWS
            .lines()
            .find(|line| line.split(' ').next() == Some(label))
            .unwrap_or_else(|| panic!("fixture row {label}"));
        let mut fields = line.split(' ').skip(1);
        let count = fields.next().unwrap().parse().unwrap();
        let errno = fields.next().unwrap().parse().unwrap();
        let hex = fields.next().unwrap_or("");
        let bytes = (0..hex.len())
            .step_by(2)
            .map(|index| u8::from_str_radix(&hex[index..index + 2], 16).unwrap())
            .collect();
        (count, errno, bytes)
    }

    fn values(label: &str) -> Vec<i64> {
        let line = ROWS.lines().find(|line| line.starts_with(label)).unwrap();
        line.split(' ')
            .skip(1)
            .map(|field| field.parse().unwrap())
            .collect()
    }

    fn wide(text: &str) -> Vec<u32> {
        text.chars().map(u32::from).chain([0]).collect()
    }

    /// Formats into a slice, checking count, bytes, terminator and the untouched tail.
    fn assert_row(label: &str, format: &str, args: &[FormatArg<'_>]) {
        let (count, _, bytes) = row(label);
        let mut output = [0xa5_u8; 1024];
        let result = sprintf_into_slice(&mut output, format.as_bytes(), args);
        if count < 0 {
            let error = result.expect_err(label);
            assert!(
                matches!(
                    error.kind(),
                    ErrorKind::InvalidData | ErrorKind::InvalidInput
                ),
                "{label}"
            );
            assert!(
                output.iter().all(|&byte| byte == 0xa5),
                "{label}: no write on failure"
            );
            return;
        }
        let written = result.unwrap_or_else(|error| panic!("{label}: {error}"));
        assert_eq!(written as i64, count, "{label}");
        assert_eq!(&output[..written], &bytes[..], "{label}");
        assert_eq!(output[written], 0, "{label}");
        assert!(
            output[written + 1..].iter().all(|&byte| byte == 0xa5),
            "{label}"
        );
    }

    #[test]
    fn numbered_arguments_match_native() {
        assert_row("positional", "[%2$s|%1$d|%2$s]", &[Int(7), Text(b"x")]);
        assert_row(
            "positional_star",
            "[%1$*2$d|%1$-*2$d|%3$.*2$f]",
            &[Int(5), Int(4), Real(2.0)],
        );
        let address = std::ptr::without_provenance(0x2a);
        let args = [
            Int(i32::from(b'q')),
            Real(0.5),
            FormatArg::LongLong(-9),
            Pointer(address),
        ];
        assert_row("positional_types", "[%3$lld|%1$c|%2$.2e|%4$p]", &args);
    }

    #[test]
    fn wide_characters_match_native_c_locale() {
        let (wide_text, ab, c, abc, s) =
            (wide("wide"), wide("ab"), wide("c"), wide("abc"), wide("S"));
        let args = [
            WideChar(u32::from('A')),
            WideText(&wide_text),
            WideText(&ab),
            WideText(&c),
            WideText(&abc),
            WideChar(u32::from('Z')),
            WideText(&s),
            WideChar(u32::from('w')),
        ];
        assert_row("wide_ascii", "[%lc|%ls|%5ls|%-4ls|%.2ls|%C|%S|%3lc]", &args);
        assert_row("wide_char_nonascii", "[%lc]", &[WideChar(0xe9)]);
        assert_row("wide_text_nonascii", "[%ls]", &[WideText(&[0xe9, 0])]);
        assert_row(
            "wide_text_cut_before_nonascii",
            "[%.1ls]",
            &[WideText(&[u32::from('a'), 0xe9, 0])],
        );
        assert_row("wide_nul", "[%lc]", &[WideChar(0)]);
    }

    #[allow(unsafe_code)]
    fn set_errno(value: i32) {
        // SAFETY: __errno_location returns the calling thread's live errno slot.
        unsafe { *libc::__errno_location() = value };
    }

    #[test]
    fn errno_text_matches_native() {
        for (label, format) in [
            ("errno_enoent", "[%m|%30m|%-28m|%.5m]"),
            ("errno_eacces", "%m"),
            ("errno_success", "%m"),
            ("errno_unknown", "%m"),
        ] {
            set_errno(row(label).1);
            assert_row(label, format, &[]);
        }
    }

    #[test]
    fn glibc_length_aliases_match_native() {
        let args = [
            FormatArg::LongLong(-5),
            FormatArg::ULongLong(5),
            FormatArg::ULongLong(255),
            FormatArg::LongLong(-6),
            FormatArg::ULongLong(6),
            FormatArg::LongLong(-7),
            FormatArg::ULongLong(7),
            FormatArg::SignedSize(8),
            FormatArg::Size(9),
        ];
        assert_row("intmax", "[%jd|%ju|%jx|%qd|%qu|%Ld|%Lu|%Zd|%Zu]", &args);
        let args = [
            Int(300),
            Int(70000),
            Int(-3),
            FormatArg::Long(-4),
            Int(300),
            FormatArg::Long(-5),
            FormatArg::ULong(255),
            FormatArg::Long(6),
        ];
        assert_row(
            "bit_widths",
            "[%w8d|%w16u|%w32d|%w64d|%wf8u|%wf16d|%wf32x|%wf64d]",
            &args,
        );
        assert_row(
            "float_long_modifier",
            "[%lf|%lF|%le|%lg|%la|%lE|%lG|%lA]",
            &[Real(1.5); 8],
        );
        let args = [
            FormatArg::UInt(5),
            FormatArg::UInt(6),
            FormatArg::UInt(5),
            FormatArg::UInt(6),
            FormatArg::UInt(5),
            FormatArg::UInt(5),
            Int(257),
            FormatArg::ULong(9),
            FormatArg::UInt(0),
        ];
        assert_row("binary", "[%b|%B|%#b|%#B|%08b|%.6b|%hhb|%lb|%#b]", &args);
    }

    #[test]
    fn long_double_matches_native_x87() {
        let args = [
            Ld(1.0),
            Ld(0.1),
            Ld(2.5),
            Ld(1.5),
            Ld(0.0),
            Ld(-3.0),
            Ld(1.0),
            Ld(0.75),
            Ld(6.0),
            Ld(0.5),
        ];
        assert_row(
            "long_double_hex",
            "[%La|%LA|%.3La|%.0La|%La|%La|%#.0La|%20La|%-+12La|%020La]",
            &args,
        );
        let args = [
            Ld(15.9),
            Ld(15.9),
            Ld(15.5),
            Ld(14.5),
            Ld(1.9999),
            Ld(15.5),
            Ld(15.9),
            Ld(1.0 / 3.0),
        ];
        assert_row(
            "long_double_hex_rounding",
            "[%La|%.0La|%.0La|%.0La|%.1La|%#.0La|%.16La|%.2La]",
            &args,
        );
        let args = [
            Ld(5e-324),
            Ld(f64::MAX),
            Ld(f64::MIN_POSITIVE),
            Ld(-0.0),
            Ld(1e-320),
        ];
        assert_row("long_double_hex_extremes", "[%La|%La|%La|%La|%La]", &args);
        let args = [
            Ld(0.1),
            Ld(1e300),
            Ld(1e-5),
            Ld(0.1),
            Ld(12345.678),
            Ld(1e-10),
            Ld(-2.0),
            Ld(3.0),
        ];
        assert_row(
            "long_double_decimal",
            "[%Lf|%Le|%Lg|%.20Lf|%LE|%LG|%+.3Lf|%#Lg]",
            &args,
        );
        let args = [
            Ld(f64::INFINITY),
            Ld(f64::NEG_INFINITY),
            Ld(f64::NAN),
            Ld(-f64::NAN),
            Ld(f64::INFINITY),
        ];
        assert_row("long_double_special", "[%Lf|%LF|%La|%Le|%5Lf]", &args);
    }

    #[test]
    fn flags_and_padding_match_native() {
        let args = [
            Int(1234567),
            Real(1234.5),
            Int(99),
            Real(1234.5),
            FormatArg::UInt(255),
            FormatArg::UInt(7),
        ];
        assert_row("locale_flags", "[%'d|%'f|%Id|%'10.2f|%'x|%I5u]", &args);
        let args = [Int(98), Int(100), Int(102), Int(-3), Int(103), Int(104)];
        assert_row("char_flags", "[%+c|% c|%-3c|%*c|%3c]", &args);
        assert_row(
            "text_flags",
            "[%+s|% s|%-5s|%5.1s]",
            &[Text(b"c"), Text(b"d"), Text(b"f"), Text(b"gh")],
        );
        let address = std::ptr::without_provenance(0x10);
        let mut args = [Pointer(address); 7];
        args[6] = Pointer(std::ptr::null());
        assert_row("pointer_flags", "[%#p|%+p|% p|%020p|%.5p|%-8p|%+p]", &args);
        assert_row("char_nul", "[%c|%5c]", &[Int(0), Int(0)]);
    }

    #[test]
    fn counts_store_the_narrowed_byte_count() {
        let cells: Vec<Cell<i64>> = (0..8).map(|_| Cell::new(-99)).collect();
        let args: Vec<FormatArg<'_>> = cells.iter().map(FormatArg::Count).collect();
        assert_row("count_store", "abc%nde%hhnf%hn%lln%ln%zn%jn%tn!", &args);
        let stored: Vec<i64> = cells.iter().map(Cell::get).collect();
        assert_eq!(stored, values("count_values"));
        let (byte, short) = (Cell::new(0), Cell::new(0));
        assert_row(
            "count_truncate",
            "%300d%hhn%hn",
            &[Int(1), FormatArg::Count(&byte), FormatArg::Count(&short)],
        );
        assert_eq!(
            vec![byte.get(), short.get()],
            values("count_truncate_values")
        );
        let count = Cell::new(0);
        assert_row(
            "count_positional",
            "%2$s%1$n|",
            &[FormatArg::Count(&count), Text(b"xy")],
        );
        assert_eq!(vec![count.get()], values("count_positional_values"));
    }

    #[test]
    fn incomplete_directives_fail_like_native() {
        assert_row("trailing_percent", "abc%", &[]);
        assert_row("trailing_width", "abc%5", &[]);
        assert_row("trailing_length", "abc%l", &[]);
    }
}

fn assert_rejected(format: &[u8], args: &[FormatArg<'_>]) {
    let mut output = b"prior".to_vec();
    let error = sprintf_into(&mut output, format, args).unwrap_err();
    assert_eq!(
        error.kind(),
        ErrorKind::InvalidInput,
        "{}",
        String::from_utf8_lossy(format)
    );
    assert_eq!(output, b"prior");
}

#[test]
fn undefined_directives_are_rejected_before_writing() {
    let count = Cell::new(0);
    assert_rejected(b"%1$d %d", &[FormatArg::Int(1), FormatArg::Int(2)]);
    assert_rejected(b"%1$*d", &[FormatArg::Int(1), FormatArg::Int(2)]);
    assert_rejected(b"%0$d", &[FormatArg::Int(1)]);
    assert_rejected(b"%y", &[]);
    assert_rejected(b"%w12d", &[FormatArg::Int(1)]);
    assert_rejected(b"%hf", &[FormatArg::Real(1.0)]);
    assert_rejected(b"%Lf", &[FormatArg::Real(1.0)]);
    assert_rejected(b"%lf", &[FormatArg::LongDouble(1.0)]);
    assert_rejected(b"%0s", &[FormatArg::Text(b"x")]);
    assert_rejected(b"%hs", &[FormatArg::Text(b"x")]);
    assert_rejected(b"%ls", &[FormatArg::Text(b"x")]);
    assert_rejected(b"%lc", &[FormatArg::Int(65)]);
    assert_rejected(b"%Lc", &[FormatArg::WideChar(65)]);
    assert_rejected(b"%5n", &[FormatArg::Count(&count)]);
    assert_rejected(b"%Ln", &[FormatArg::Count(&count)]);
    assert_rejected(b"%n", &[FormatArg::Int(0)]);
    assert_rejected(b"%jd", &[FormatArg::Long(1)]);
    assert_rejected(b"%2$d", &[FormatArg::Int(1)]);
    assert_rejected(b"%#m", &[]);
    assert_eq!(count.get(), 0);
}

#[test]
fn printf_writes_to_stdout_and_fprintf_to_any_sink() {
    assert_eq!(printf(b"", &[]).unwrap(), 0);
    let mut sink = Vec::new();
    let count = fprintf_to(
        &mut sink,
        b"%2$s=%1$d",
        &[FormatArg::Int(3), FormatArg::Text(b"n")],
    )
    .unwrap();
    assert_eq!((count, sink.as_slice()), (3, b"n=3".as_slice()));
}

/// A sink that fails every write, as a C stream in its error state.
struct FailingSink;
impl std::io::Write for FailingSink {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("stream error"))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn fprintf_reports_the_sink_error() {
    let error = fprintf_to(&mut FailingSink, b"%d", &[FormatArg::Int(1)]).unwrap_err();
    assert_eq!(error.to_string(), "stream error");
}
