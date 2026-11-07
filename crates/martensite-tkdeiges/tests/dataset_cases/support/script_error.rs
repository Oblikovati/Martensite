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

/// Whether a caught unwind is a script error, as the case's `catch` holds one and no other panic.
/// Example: `is_script_error(&*payload)`.
pub fn is_script_error(payload: &(dyn std::any::Any + Send)) -> bool {
    payload.is::<ScriptError>()
}
