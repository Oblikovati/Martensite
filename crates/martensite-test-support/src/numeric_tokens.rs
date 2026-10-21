//! Development-only stdout tokens for retained numeric probes; independent of toolkit-private source.
#[path = "numeric_tokens/reader.rs"]
mod reader;
use martensite_tkernel::standard::Failure;
#[allow(unused_imports)]
// Some probes consume only encoders, others also consume the field reader.
pub use reader::Reader;

/// Prints a fallible value or the raised exception type (the G2_TRY macro).
pub fn attempt(block: impl FnOnce() -> Result<(), Failure>) {
    if let Err(error) = block() {
        print!("{} ", error.exception_type());
    }
}
/// Prints a fallible value with `print`, or the raised exception type.
pub fn show<T>(value: Result<T, Failure>, print: impl FnOnce(T)) {
    match value {
        Ok(value) => print(value),
        Err(error) => print!("{} ", error.exception_type()),
    }
}
pub fn real(value: f64) {
    print!("{} ", value.to_bits());
}
pub fn int(value: i32) {
    print!("{value} ");
}
pub fn flag(value: bool) {
    print!("{} ", i32::from(value));
}
/// Ends the output line of one request.
pub fn end() {
    println!();
}
