//! Error transport for the date, period and range checks of the foundation APIs.

/// Exception categories used by the first foundation APIs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Reported as `Standard_RangeError`.
    Range,
    /// Reported as `Quantity_PeriodDefinitionError`.
    PeriodDefinition,
    /// Reported as `Quantity_DateDefinitionError`.
    DateDefinition,
    /// An operation would overflow the signed 32-bit seconds representation.
    /// Signed overflow has no defined result to preserve, so it is an error.
    NumericOverflow,
}

/// A typed error carrying its exception category and message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

impl Error {
    pub(crate) const fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }

    /// The exception category, or a defined Rust overflow error.
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The exception message, without the type name.
    pub const fn message(&self) -> &'static str {
        self.message
    }

    /// The exception name reported for this error.
    pub const fn exception_type(&self) -> &'static str {
        match self.kind {
            ErrorKind::Range => "Standard_RangeError",
            ErrorKind::PeriodDefinition => "Quantity_PeriodDefinitionError",
            ErrorKind::DateDefinition => "Quantity_DateDefinitionError",
            ErrorKind::NumericOverflow => "Standard_Overflow",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.exception_type(), self.message)
    }
}

impl std::error::Error for Error {}

impl From<Error> for crate::standard::Failure {
    /// Preserves the exception name and C-string message; [`Error`] carries no stack trace.
    ///
    /// ```
    /// use martensite_tkernel::quantity::Date;
    /// use martensite_tkernel::standard::Failure;
    /// let failure = Failure::from(Date::new(0, 1, 2000, 0, 0, 0, 0, 0).unwrap_err());
    /// assert!(failure.is_kind("Standard_DomainError"));
    /// ```
    fn from(error: Error) -> Self {
        Self::with_message(error.exception_type(), error.message())
    }
}

pub(crate) fn seconds_i32(value: i64) -> Result<i32, Error> {
    i32::try_from(value).map_err(|_| Error::new(ErrorKind::NumericOverflow, "32-bit time overflow"))
}

#[cfg(test)]
mod tests {
    use super::{Error, ErrorKind};
    use crate::standard::Failure;

    #[test]
    fn error_conversion_stops_at_the_native_c_string_terminator() {
        let error = Error::new(ErrorKind::DateDefinition, "before\0after");
        let failure = Failure::from(error);
        assert_eq!(failure.message(), "before");
    }
}
