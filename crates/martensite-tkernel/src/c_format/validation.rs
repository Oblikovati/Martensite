//! Validation for the safe public formatting surface: every directive must have a defined
//! C/glibc meaning and a matching typed argument before anything is rendered.
//!
//! C17 7.21.6.1 leaves undefined a length modifier or flag a conversion does not list
//! (`%#s`, `%0c`, `%hp`, `%Lc`), a precision on `%c`/`%n`, an unknown conversion, and
//! mixing numbered with sequential arguments (POSIX); those return an error instead of
//! glibc's incidental output.
use super::arguments::Arguments;
use super::{FormatArg, FormatError, Length, Spec, Unencodable, current_errno, parse_spec};
mod kinds;
use kinds::{ArgumentKind, argument_kind};

/// Why a format cannot be rendered: an undefined directive or an unencodable wide unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormatFault {
    Directive(FormatError),
    Unencodable(Unencodable),
}

/// Checks every directive, then renders; `%m` reads the caller's `errno` at entry.
pub(crate) fn checked_sprintf(
    format: &[u8],
    args: &[FormatArg<'_>],
) -> Result<Vec<u8>, FormatFault> {
    let errno = current_errno();
    validate_format(format, args).map_err(FormatFault::Directive)?;
    let mut output = Vec::new();
    super::render_into(format, args, errno, &mut output).map_err(FormatFault::Unencodable)?;
    Ok(output)
}

/// Whether the format uses numbered (`n$`) or sequential arguments; never both.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Numbering {
    Unknown,
    Numbered,
    Sequential,
}

fn validate_format(format: &[u8], args: &[FormatArg<'_>]) -> Result<(), FormatError> {
    let mut arguments = Arguments::new(args);
    let mut numbering = Numbering::Unknown;
    let mut index: usize = 0;
    while index < format.len() {
        if format[index] != b'%' {
            index += 1;
            continue;
        }
        let (spec, end) = parse_spec(format, index + 1);
        let check = Check {
            spec,
            offset: index,
        };
        numbering = check.numbering(numbering)?;
        check.validate(&mut arguments)?;
        index = end;
    }
    Ok(())
}

/// One parsed directive and its byte offset in the format.
struct Check {
    spec: Spec,
    offset: usize,
}

impl Check {
    fn error(&self, expected: &'static str) -> FormatError {
        FormatError {
            offset: self.offset,
            conversion: self.spec.conversion,
            expected,
        }
    }

    /// Positions of the directive's argument reads, in C order: width, precision, value.
    fn reads(&self) -> Vec<Option<usize>> {
        let spec = &self.spec;
        let mut reads = Vec::new();
        if spec.dynamic_width {
            reads.push(spec.width_position);
        }
        if spec.dynamic_precision {
            reads.push(spec.precision_position);
        }
        if !spec.takes_no_argument() {
            reads.push(spec.position);
        }
        reads
    }

    fn numbering(&self, current: Numbering) -> Result<Numbering, FormatError> {
        let mut numbering = current;
        for read in self.reads() {
            let this = if read.is_some() {
                Numbering::Numbered
            } else {
                Numbering::Sequential
            };
            if numbering != Numbering::Unknown && numbering != this {
                return Err(self.error("numbered (n$) or sequential arguments, not both"));
            }
            numbering = this;
        }
        if self.spec.position == Some(0) || self.reads().contains(&Some(0)) {
            return Err(self.error("argument positions counted from 1$"));
        }
        Ok(numbering)
    }

    fn validate(&self, arguments: &mut Arguments<'_, '_>) -> Result<(), FormatError> {
        let kind = argument_kind(&self.spec).map_err(|expected| self.error(expected))?;
        self.validate_dimension(arguments, self.spec.dynamic_width, self.spec.width_position)?;
        self.validate_dimension(
            arguments,
            self.spec.dynamic_precision,
            self.spec.precision_position,
        )?;
        if self.spec.takes_no_argument() {
            return Ok(());
        }
        let argument = arguments.take(self.spec.position);
        if argument.is_some_and(|argument| kind.accepts(argument, &self.spec)) {
            return Ok(());
        }
        Err(self.error(kind.expected()))
    }

    fn validate_dimension(
        &self,
        arguments: &mut Arguments<'_, '_>,
        dynamic: bool,
        position: Option<usize>,
    ) -> Result<(), FormatError> {
        if !dynamic || matches!(arguments.take(position), Some(FormatArg::Int(_))) {
            return Ok(());
        }
        Err(self.error("an Int for a dynamic width or precision"))
    }
}

impl ArgumentKind {
    fn accepts(self, argument: &FormatArg<'_>, spec: &Spec) -> bool {
        match self {
            Self::Integer => kinds::integer_matches(argument, spec),
            Self::Real => matches!(argument, FormatArg::Real(_)),
            Self::LongDouble => matches!(argument, FormatArg::LongDouble(_)),
            Self::Text => matches!(argument, FormatArg::Text(_)),
            Self::Char => matches!(argument, FormatArg::Int(_)),
            Self::WideText => matches!(argument, FormatArg::WideText(_)),
            Self::WideChar => matches!(argument, FormatArg::WideChar(_)),
            Self::Pointer => matches!(argument, FormatArg::Pointer(_)),
            Self::Count => matches!(argument, FormatArg::Count(_)),
            Self::Nothing => true,
        }
    }
}

/// True when `length` is one `%n` may carry (any integer modifier).
pub(super) fn is_integer_length(length: Length) -> bool {
    !matches!(length, Length::LongDouble | Length::Invalid)
}
