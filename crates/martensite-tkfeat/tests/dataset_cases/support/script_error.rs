//! A script error: it stops the script where it happens, end scripts included, and the harness
//! writes one exception line that the parse rules then read.
use super::rules::EXCEPTION_PREFIX;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

/// The unwinding payload of a script error; any other panic stays a test failure.
#[derive(Debug)]
struct ScriptError(String);

/// Stops the script as an error in a command does; only inside [`run_script`].
/// Example: `script_error("File a.brep could not be found")`.
pub fn script_error(message: &str) -> ! {
    // No panic hook runs: this is a normal outcome of a script, not a test failure.
    resume_unwind(Box::new(ScriptError(message.to_owned())))
}

/// Runs a whole script; after a script error, writes the exception line and returns normally.
/// Example: `run_script(|| script_error("stop"))`.
pub fn run_script(script: impl FnOnce()) {
    let Err(payload) = catch_unwind(AssertUnwindSafe(script)) else {
        return;
    };
    match payload.downcast::<ScriptError>() {
        Ok(error) => println!("{EXCEPTION_PREFIX}{}", error.0),
        Err(other) => resume_unwind(other),
    }
}

/// Runs a command as the script's `catch` does: `true` after a script error, which stops only the
/// command; any other panic stays a test failure. Example: `catch_script_error(|| script_error("x"))`.
pub fn catch_script_error(command: impl FnOnce()) -> bool {
    let Err(payload) = catch_unwind(AssertUnwindSafe(command)) else {
        return false;
    };
    if payload.downcast_ref::<ScriptError>().is_some() {
        return true;
    }
    resume_unwind(payload)
}
