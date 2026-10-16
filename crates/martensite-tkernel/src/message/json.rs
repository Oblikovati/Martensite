//! The JSON dump helpers used by the message `dump_json` members.
//!
//! Field names are the already-converted dump spellings (`Name`,
//! `CompositAlerts`, `Alert<n>`).
use crate::standard::dump::{
    DumpOutput, DumpStream, add_values_separator, dump_integer, dump_vector,
};

/// Pointer text for dumps: empty for null, else the `std::ostream`-style
/// spelling with leading zeros stripped from position 2 and `0x` prepended.
///
/// Kept on purpose: the scan starts at the `x` of `0x...`, which is not `'0'`, so the
/// result keeps it: `0x7ffd` becomes `0xx7ffd`.
pub(crate) fn pointer_info(pointer: *const ()) -> String {
    if pointer.is_null() {
        return String::new();
    }
    let spelled = format!("{pointer:p}");
    match spelled.bytes().skip(1).position(|byte| byte != b'0') {
        Some(offset) => format!("0x{}", &spelled[offset + 1..]),
        None => spelled,
    }
}

fn written(output: &mut DumpStream, bytes: &[u8]) {
    // DumpStream writes into memory and cannot fail.
    output.write_fragment(bytes).expect("in-memory dump stream");
}

/// Opens a class dump: `"className": "<name>"`.
pub(crate) fn class_begin(output: &mut DumpStream, class_name: &str) {
    field_string(output, "className", class_name.as_bytes());
}

/// A string field: `"<name>": "<value>"`.
pub(crate) fn field_string(output: &mut DumpStream, name: &str, value: &[u8]) {
    add_values_separator(output).expect("in-memory dump stream");
    written(output, format!("\"{name}\": \"").as_bytes());
    written(output, value);
    written(output, b"\"");
}

/// A numeric field holding an integer (booleans print as 0/1).
pub(crate) fn field_integer(output: &mut DumpStream, name: &str, value: i32) {
    dump_integer(output, name, value).expect("in-memory dump stream");
}

/// A vector field: `"<name>": [v1, v2]` with `std::ostream` default precision.
pub(crate) fn field_reals(output: &mut DumpStream, name: &str, values: &[f64]) {
    dump_vector(output, name, values).expect("in-memory dump stream");
}

/// A child object dumped into its own stream, skipped at
/// depth 0; the child receives `depth - 1`.
pub(crate) fn field_dumped(
    output: &mut DumpStream,
    depth: i32,
    name: &str,
    child: impl FnOnce(&mut DumpStream, i32),
) {
    if depth == 0 {
        return;
    }
    let mut field = DumpStream::default();
    child(&mut field, depth - 1);
    stream_value(output, name, field.bytes());
}

/// A pre-dumped stream value: `"<name>": {<text>}`.
pub(crate) fn stream_value(output: &mut DumpStream, name: &str, text: &[u8]) {
    // The dump text is read as a C string: it ends at the first NUL.
    let text = text.split(|&byte| byte == 0).next().unwrap_or_default();
    add_values_separator(output).expect("in-memory dump stream");
    written(output, format!("\"{name}\": {{").as_bytes());
    written(output, text);
    written(output, b"}");
}

/// A base-class dump: separator, then the base dump at `depth - 1`; skipped at depth 0.
pub(crate) fn base_class(
    output: &mut DumpStream,
    depth: i32,
    base: impl FnOnce(&mut DumpStream, i32),
) {
    if depth == 0 {
        return;
    }
    add_values_separator(output).expect("in-memory dump stream");
    base(output, depth - 1);
}
