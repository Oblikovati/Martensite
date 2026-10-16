//! The argument each defined directive reads, and which flags it may carry.
use super::super::{FormatArg, Length, Spec};
use super::is_integer_length;

/// The typed argument a directive consumes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ArgumentKind {
    Integer,
    Real,
    LongDouble,
    Text,
    Char,
    WideText,
    WideChar,
    Pointer,
    Count,
    Nothing,
}

impl ArgumentKind {
    pub(super) fn expected(self) -> &'static str {
        match self {
            Self::Integer => {
                "the integer FormatArg of the length modifier (Int for hh/h/none or %c, \
                UInt unsigned, Long/ULong for l, LongLong/ULongLong for ll/q/L/j, \
                SignedSize/Size for z, PtrDiff/UnsignedPtrDiff for t)"
            }
            Self::Real => "a Real argument",
            Self::LongDouble => "a LongDouble argument for %L",
            Self::Text => "a Text argument",
            Self::Char => "an Int argument",
            Self::WideText => "a WideText argument for %ls/%S",
            Self::WideChar => "a WideChar argument for %lc/%C",
            Self::Pointer => "a Pointer argument",
            Self::Count => "a Count argument for %n",
            Self::Nothing => "no argument",
        }
    }
}

/// The argument kind of a defined directive, or what a defined one would look like.
pub(super) fn argument_kind(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    if spec.length == Length::Invalid {
        return Err("a wN/wfN width of 8, 16, 32 or 64");
    }
    match spec.conversion {
        b'%' => percent(spec),
        b'd' | b'i' | b'o' | b'u' | b'x' | b'X' | b'b' | b'B' => integer(spec),
        b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => real(spec),
        b's' | b'S' => text(spec),
        b'c' | b'C' => character(spec),
        b'p' => pointer(spec),
        b'n' => count(spec),
        b'm' => errno_text(spec),
        _ => Err("a defined conversion: d i o u x X b B f F e E g G a A c C s S p n m %%"),
    }
}

fn percent(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    let bare = !spec.left
        && !spec.plus
        && !spec.space
        && !spec.alternate
        && !spec.zero
        && !spec.locale_flag
        && spec.width == 0
        && !spec.dynamic_width
        && spec.precision.is_none()
        && !spec.dynamic_precision
        && spec.length == Length::None
        && spec.position.is_none();
    bare.then_some(ArgumentKind::Nothing)
        .ok_or("an unmodified %% directive")
}

/// Every integer modifier, and glibc's `L` (read as `long long`).
fn integer(_spec: &Spec) -> Result<ArgumentKind, &'static str> {
    Ok(ArgumentKind::Integer)
}

/// `l` has no effect on a floating conversion, `L` reads a `long double`; C leaves the
/// integer modifiers undefined there.
fn real(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    match spec.length {
        Length::None | Length::Long => Ok(ArgumentKind::Real),
        Length::LongDouble => Ok(ArgumentKind::LongDouble),
        _ => Err("a floating conversion with no modifier, l, or L"),
    }
}

/// `-`, `+` and space are the only flags C defines for the non-numeric conversions.
fn plain_flags(spec: &Spec) -> bool {
    !spec.alternate && !spec.zero && !spec.locale_flag
}

fn text(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    let wide = spec.conversion == b'S' || spec.length == Length::Long;
    let length_ok =
        spec.length == Length::None || (spec.length == Length::Long && spec.conversion == b's');
    if !plain_flags(spec) || !length_ok {
        return Err("%s/%ls/%S with only the -, + or space flags");
    }
    Ok(if wide {
        ArgumentKind::WideText
    } else {
        ArgumentKind::Text
    })
}

fn character(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    let wide = spec.conversion == b'C' || spec.length == Length::Long;
    let length_ok =
        spec.length == Length::None || (spec.length == Length::Long && spec.conversion == b'c');
    let precision = spec.precision.is_some() || spec.dynamic_precision;
    if !plain_flags(spec) || !length_ok || precision {
        return Err("%c/%lc/%C with only a width and the -, + or space flags");
    }
    Ok(if wide {
        ArgumentKind::WideChar
    } else {
        ArgumentKind::Char
    })
}

/// `%p` text is implementation-defined; glibc lays it out as `%#x` (sign, `0`, precision
/// apply, `#` is implied), so only a length modifier is undefined.
fn pointer(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    if spec.length != Length::None {
        return Err("%p without a length modifier");
    }
    Ok(ArgumentKind::Pointer)
}

fn count(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    let bare = !spec.left
        && !spec.plus
        && !spec.space
        && plain_flags(spec)
        && spec.width == 0
        && !spec.dynamic_width
        && spec.precision.is_none()
        && !spec.dynamic_precision;
    if !bare || !is_integer_length(spec.length) {
        return Err("%n with an integer length modifier only");
    }
    Ok(ArgumentKind::Count)
}

/// glibc `%m`: `strerror(errno)` laid out like `%s`.
// TODO(platform): MSVC printf has no %m; on Windows it is rejected as unsupported.
fn errno_text(spec: &Spec) -> Result<ArgumentKind, &'static str> {
    if cfg!(windows) {
        return Err("a conversion this platform's C library defines (%m is glibc-only)");
    }
    if !plain_flags(spec) || spec.length != Length::None {
        return Err("%m with only a width, a precision and the - flag");
    }
    Ok(ArgumentKind::Nothing)
}

/// The typed argument of an integer directive under its length modifier.
pub(super) fn integer_matches(argument: &FormatArg<'_>, spec: &Spec) -> bool {
    let signed = matches!(spec.conversion, b'd' | b'i');
    match (spec.length, signed) {
        (Length::Char | Length::Short, _) => matches!(argument, FormatArg::Int(_)),
        (Length::None, true) => matches!(argument, FormatArg::Int(_)),
        (Length::None, false) => matches!(argument, FormatArg::UInt(_)),
        (Length::Long, true) => matches!(argument, FormatArg::Long(_)),
        (Length::Long, false) => matches!(argument, FormatArg::ULong(_)),
        (Length::LongLong | Length::IntMax | Length::LongDouble, true) => {
            matches!(argument, FormatArg::LongLong(_))
        }
        (Length::LongLong | Length::IntMax | Length::LongDouble, false) => {
            matches!(argument, FormatArg::ULongLong(_))
        }
        (Length::Size, true) => matches!(argument, FormatArg::SignedSize(_)),
        (Length::Size, false) => matches!(argument, FormatArg::Size(_)),
        (Length::PtrDiff, true) => matches!(argument, FormatArg::PtrDiff(_)),
        (Length::PtrDiff, false) => matches!(argument, FormatArg::UnsignedPtrDiff(_)),
        (Length::Invalid, _) => false,
    }
}
