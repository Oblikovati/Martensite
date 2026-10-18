//! Numeric token encoding shared by buffered replay and the development stdout adapter.
use super::output::Output;
pub use super::reader::Reader;
use martensite_tkernel::standard::Failure;

/// Records a fallible step or its exact raised exception type.
pub fn attempt(output: &Output, block: impl FnOnce() -> Result<(), Failure>) {
    if let Err(error) = block() {
        output.emit(format_args!("{} ", error.exception_type()));
    }
}
/// Encodes a value through its callback, or records the exact exception type.
pub fn show<T>(output: &Output, value: Result<T, Failure>, print: impl FnOnce(T)) {
    match value {
        Ok(value) => print(value),
        Err(error) => output.emit(format_args!("{} ", error.exception_type())),
    }
}
pub fn real(output: &Output, value: f64) {
    output.emit(format_args!("{} ", value.to_bits()));
}
pub fn int(output: &Output, value: i32) {
    output.emit(format_args!("{value} "));
}
pub fn flag(output: &Output, value: bool) {
    int(output, i32::from(value));
}
/// Preserves the single output line for each complete request.
pub fn end(output: &Output) {
    output.emit(format_args!("\n"));
}
