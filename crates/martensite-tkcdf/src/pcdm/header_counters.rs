//! The two user-info counters use the last valid value and warn on every invalid token.
use super::file_header::{native_driver, query_catching_read_failure};
use martensite_tkernel::{
    message::{Gravity, Messenger},
    standard::Failure,
    storage::{BaseDriver, HeaderData},
    tcollection::{AsciiString, ExtendedString},
};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(super) struct CounterSpec {
    key: &'static [u8],
    fallback: i32,
    warning: &'static str,
}
pub(super) const REFERENCES: CounterSpec = CounterSpec {
    key: b"REFERENCE_COUNTER: ",
    fallback: 0,
    warning: "Warning: could not read the reference counter in ",
};
pub(super) const VERSION: CounterSpec = CounterSpec {
    key: b"MODIFICATION_COUNTER: ",
    fallback: -1,
    warning: "Warning: could not read the version in ",
};

pub(super) fn counter(
    file: &ExtendedString,
    messenger: Option<&Arc<Messenger>>,
    spec: CounterSpec,
) -> Result<i32, Failure> {
    let Some(mut driver) = native_driver(file)? else {
        return Ok(spec.fallback);
    };
    query_catching_read_failure(&mut *driver, file, spec.fallback, |driver, found| {
        read_counter(driver, found, file, messenger, spec)
    })
}

fn read_counter(
    driver: &mut dyn BaseDriver,
    found: &mut i32,
    file: &ExtendedString,
    messenger: Option<&Arc<Messenger>>,
    spec: CounterSpec,
) -> Result<(), Failure> {
    let header = HeaderData::default();
    header.read(driver)?;
    for line in header
        .user_info()
        .iter()
        .filter(|line| line.search(spec.key) != -1)
    {
        apply_counter_line(line, found, file, messenger, spec)?;
    }
    Ok(())
}

fn apply_counter_line(
    line: &AsciiString,
    found: &mut i32,
    file: &ExtendedString,
    messenger: Option<&Arc<Messenger>>,
    spec: CounterSpec,
) -> Result<(), Failure> {
    let token = line.token(b" ", 2);
    if token.is_integer_value() {
        *found = token.integer_value()?;
        return Ok(());
    }
    let mut warning = ExtendedString::new(spec.warning);
    warning.assign_cat(file.clone());
    if let Some(messenger) = messenger {
        messenger.send_extended(&warning, Gravity::Warning);
    }
    Ok(())
}

#[cfg(test)]
#[path = "header_counter_tests.rs"]
mod tests;
