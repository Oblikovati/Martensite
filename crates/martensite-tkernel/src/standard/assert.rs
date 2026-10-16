//! The assertion helpers: report, raise and static assertion.
//!
//! Assertions behave as in a release build: a failed
//! assertion only performs its action, and a raising assertion raises
//! `Standard_ProgramError`. The debug-build report is available as [`standard_assert_report`]
//! with its POSIX text; a Rust caller decides whether to call it.
use super::{Failure, exceptions};
use std::io::{self, Write};

/// The empty statement that completes the macros;
/// `standard_assert_do_nothing()`.
pub fn standard_assert_do_nothing() {}

/// The debug-build assertion report: writes the failed statement, file,
/// line and optional description to `stream` and returns
/// true, which asks the caller to break into the debugger.
/// `standard_assert_report(&mut io::stderr(), file!(), line!() as i32, "x > 0", "")?`.
pub fn standard_assert_report(
    stream: &mut impl Write,
    file: &str,
    line: i32,
    expression: &str,
    description: &str,
) -> io::Result<bool> {
    write!(
        stream,
        "ERROR: statement '{expression}' is not TRUE!\n\nFile: '{file}'\nLine: {line}\n"
    )?;
    if !description.is_empty() {
        writeln!(stream, "Description: {description}")?;
    }
    stream.flush()?;
    Ok(true)
}

/// The exception a raising assertion produces when `holds` is false
/// (`expression` is the asserted text); `standard_assert_raise(x > 0, file!(), "x > 0", "positive")?`.
pub fn standard_assert_raise(
    holds: bool,
    file: &str,
    expression: &str,
    description: &str,
) -> Result<(), Failure> {
    if holds {
        return Ok(());
    }
    Err(exceptions::program_error(&format!(
        "*** ERROR: ASSERT in file '{file}': \n{description} ({expression})"
    )))
}

/// A compile-time assertion: only the `true` specialization has `assert_ok`,
/// so `StandardStaticAssert::<{ N > 0 }>::assert_ok()` fails to compile when `N == 0`.
pub struct StandardStaticAssert<const CONDITION: bool>;
impl StandardStaticAssert<true> {
    /// Compiles only for a true constant condition; `StandardStaticAssert::<true>::assert_ok()`.
    pub fn assert_ok() {}
}
