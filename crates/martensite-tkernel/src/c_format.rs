//! Shared C-style formatting engine used by the message helpers and the safe C-string wrappers.
//!
//! Message argument formats are user data (`%-10.3f`, `%05d`, `%s`) rendered by this
//! engine. Message time filling and OSD timing output also depend on its C-locale layout.
//!
//! It implements the glibc C-locale `printf` family on
//! Linux: numbered (`n$`) and sequential arguments, literal and dynamic width/precision,
//! every C conversion plus glibc's `%m`, `%b`, `%C`/`%S` and `q`/`Z`/`wN` aliases, wide
//! characters, `long double`, and `%n`. The public wrappers reject directives whose
//! behaviour C leaves undefined before rendering.
mod arguments;
mod errno_text;
mod float;
mod long_double;
mod nonfinite;
mod parse;
mod text;
pub(crate) use nonfinite::nonfinite_text;
mod hexadecimal;
mod integer;
mod pointer;
mod validation;
use arguments::Arguments;
pub(crate) use errno_text::{current_errno, os_error_text};
use std::cell::Cell;

/// One typed argument for the supported C `printf` conversions.
#[derive(Clone, Copy, Debug)]
pub enum FormatArg<'a> {
    /// An `int` conversion, dynamic width/precision, or `%c` argument.
    Int(i32),
    /// An `unsigned int` argument.
    UInt(u32),
    /// A platform C `long` argument, with the target's own width.
    Long(std::ffi::c_long),
    /// A platform C `unsigned long` argument, with the target's own width.
    ULong(std::ffi::c_ulong),
    /// A C `long long` or `intmax_t` argument (`%lld`, `%qd`, `%Ld`, `%jd`).
    LongLong(i64),
    /// A C `unsigned long long` or `uintmax_t` argument (`%llu`, `%qu`, `%ju`).
    ULongLong(u64),
    /// The signed integer type corresponding to C `size_t`, for `%zd` and `%zi`.
    SignedSize(isize),
    /// A C `size_t` argument, for `%zu`, `%zo`, `%zx`, and `%zX`.
    Size(usize),
    /// A C `ptrdiff_t` argument, for `%td` and `%ti`.
    PtrDiff(isize),
    /// The unsigned integer type corresponding to C `ptrdiff_t`.
    UnsignedPtrDiff(usize),
    /// A `double` argument.
    Real(f64),
    /// A C `long double` argument for `%Lf`/`%Le`/`%Lg`/`%La`. Rust has no extended type,
    /// so the value is a binary64 one, which every `long double` format holds exactly.
    LongDouble(f64),
    /// A C `%p` address, formatted without accessing the pointed-to storage.
    Pointer(*const std::ffi::c_void),
    /// A byte string used as a C string, truncated at the first NUL.
    Text(&'a [u8]),
    /// A `wint_t` for `%lc`/`%C`, as its code unit.
    WideChar(u32),
    /// A `const wchar_t*` for `%ls`/`%S`, as code units read up to the first zero unit.
    WideText(&'a [u32]),
    /// The object a `%n` directive stores the count of bytes written so far into, after
    /// the C narrowing of its length modifier (`%hhn` keeps the low 8 bits, signed).
    Count(&'a Cell<i64>),
}
pub(crate) use FormatArg as CArg;

/// A format directive outside the defined argument and conversion set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FormatError {
    pub(crate) offset: usize,
    pub(crate) conversion: u8,
    pub(crate) expected: &'static str,
}

/// A wide character with no C-locale multibyte form (C `EILSEQ`): code `unit` of the
/// directive at byte `offset`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Unencodable {
    pub(crate) offset: usize,
    pub(crate) unit: u32,
}

/// A C length modifier, with glibc's aliases folded in.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Length {
    /// `hh` (also `w8`, `wf8`).
    Char,
    /// `h` (also `w16`).
    Short,
    /// No modifier (also `w32`).
    #[default]
    None,
    /// `l` (also `w64` and the glibc `wf16`/`wf32`/`wf64` fast types).
    Long,
    /// `ll` or `q`.
    LongLong,
    /// `z` or `Z`.
    Size,
    /// `t`.
    PtrDiff,
    /// `j`.
    IntMax,
    /// `L`: `long double` for floating conversions, `long long` for integers in glibc.
    LongDouble,
    /// A `wN`/`wfN` with an unsupported width.
    Invalid,
}

/// Parsed flags, width, precision and length modifier of one conversion.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Spec {
    pub(crate) position: Option<usize>,
    pub(crate) left: bool,
    pub(crate) plus: bool,
    pub(crate) space: bool,
    pub(crate) alternate: bool,
    pub(crate) zero: bool,
    pub(crate) locale_flag: bool,
    pub(crate) width: usize,
    pub(crate) dynamic_width: bool,
    pub(crate) width_position: Option<usize>,
    pub(crate) precision: Option<usize>,
    pub(crate) dynamic_precision: bool,
    pub(crate) precision_position: Option<usize>,
    pub(crate) length: Length,
    pub(crate) conversion: u8,
}

impl Spec {
    /// True for directives that read no argument (`%%`, `%m`).
    pub(crate) fn takes_no_argument(&self) -> bool {
        matches!(self.conversion, b'%' | b'm')
    }
}

/// Formats `format` with `args` like C `sprintf`; `c_sprintf(b"%02d", &[CArg::Int(5)]) == b"05"`.
///
/// Rendering stops at an unencodable wide character, keeping the bytes before it.
pub(crate) fn c_sprintf(format: &[u8], args: &[CArg<'_>]) -> Vec<u8> {
    let mut output = Vec::new();
    let _ = render_into(format, args, current_errno(), &mut output);
    output
}

/// Renders every directive into `output`; `errno` is the value `%m` describes.
pub(crate) fn render_into(
    format: &[u8],
    args: &[CArg<'_>],
    errno: i32,
    output: &mut Vec<u8>,
) -> Result<(), Unencodable> {
    let mut arguments = Arguments::new(args);
    let mut index = 0;
    while index < format.len() {
        if format[index] != b'%' {
            output.push(format[index]);
            index += 1;
            continue;
        }
        let (spec, end) = parse::parse_spec(format, index + 1);
        let spec = arguments.resolve_dimensions(spec);
        let argument = arguments.value(&spec);
        let directive = Directive {
            text: &format[index..end],
            offset: index,
            errno,
        };
        render(&spec, argument, &directive, output)?;
        index = end;
    }
    Ok(())
}

pub(crate) use parse::parse_spec;
pub(crate) use validation::{FormatFault, checked_sprintf};

/// The source bytes and position of one directive, and the `errno` `%m` reads.
struct Directive<'f> {
    text: &'f [u8],
    offset: usize,
    errno: i32,
}

fn render(
    spec: &Spec,
    argument: Option<&CArg<'_>>,
    directive: &Directive<'_>,
    output: &mut Vec<u8>,
) -> Result<(), Unencodable> {
    let body = match spec.conversion {
        b'%' => vec![b'%'],
        b'd' | b'i' | b'o' | b'u' | b'x' | b'X' | b'b' | b'B' => integer::format(spec, argument),
        b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => float::format(spec, argument),
        b's' | b'S' | b'c' | b'C' => return text::render(spec, argument, directive.offset, output),
        b'p' => pointer::format(spec, argument),
        b'n' => {
            text::store_count(spec, argument, output.len());
            Vec::new()
        }
        b'm' => text::errno_message(spec, directive.errno),
        // An invalid conversion is undefined behavior in C; glibc echoes the directive.
        _ => directive.text.to_vec(),
    };
    output.extend_from_slice(&body);
    Ok(())
}

/// Joins sign/prefix and digits, padding to the width with spaces or (numeric) zeros.
pub(crate) fn pad(spec: &Spec, prefix: Vec<u8>, digits: Vec<u8>, zero_allowed: bool) -> Vec<u8> {
    let length = prefix.len() + digits.len();
    let fill = spec.width.saturating_sub(length);
    if spec.left {
        return [prefix, digits, vec![b' '; fill]].concat();
    }
    if spec.zero && zero_allowed {
        return [prefix, vec![b'0'; fill], digits].concat();
    }
    [vec![b' '; fill], prefix, digits].concat()
}

/// The `+`/space sign prefix of a signed conversion.
pub(crate) fn sign_prefix(spec: &Spec, negative: bool) -> Vec<u8> {
    match (negative, spec.plus, spec.space) {
        (true, _, _) => b"-".to_vec(),
        (false, true, _) => b"+".to_vec(),
        (false, false, true) => b" ".to_vec(),
        _ => Vec::new(),
    }
}

/// `printf("%.*e", precision, value)`, as real-to-text conversion uses it; `format_e(1.5, 2) == "1.50e+00"`.
pub(crate) fn format_e(value: f64, precision: usize) -> String {
    format_real(&format!("%.{precision}e"), value)
}

/// `printf("%.*g", precision, value)`, the default `ostream << double` spelling at
/// `precision(N)`; `format_g(5.5, 12) == "5.5"`.
pub(crate) fn format_g(value: f64, precision: usize) -> String {
    format_real(&format!("%.{precision}g"), value)
}

fn format_real(format: &str, value: f64) -> String {
    let bytes: Vec<u8> = c_sprintf(format.as_bytes(), &[CArg::Real(value)]);
    String::from_utf8(bytes).expect("C float formatting is ASCII")
}

#[cfg(test)]
mod real_tests {
    use super::*;
    #[test]
    fn c_spellings_of_finite_and_special_values() {
        assert_eq!(format_e(1.5, 2), "1.50e+00");
        assert_eq!(format_e(-1e-300, 1), "-1.0e-300");
        assert_eq!(format_e(-0.0, 0), "-0e+00");
        assert_eq!(format_e(f64::NAN, 17), "nan");
        assert_eq!(format_e(f64::NEG_INFINITY, 17), "-inf");
        assert_eq!(format_g(5.5, 12), "5.5");
        assert_eq!(format_g(0.0, 12), "0");
        assert_eq!(format_g(1e12, 12), "1e+12");
        assert_eq!(format_g(123456789012.0, 12), "123456789012");
        assert_eq!(format_g(0.0001, 12), "0.0001");
        assert_eq!(format_g(0.00001, 12), "1e-05");
        assert_eq!(format_g(0.125, 2), "0.12");
    }
}
