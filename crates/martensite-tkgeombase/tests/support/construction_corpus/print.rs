//! Development-only stdout tokens for retained numeric probes; independent of toolkit-private source.
#[path = "reader.rs"]
mod reader;
use martensite_tkernel::standard::Failure;
#[allow(unused_imports)]
// Some probes consume only encoders, others also consume the field reader.
pub use reader::Reader;

/// Prints a fallible value or the raised exception type (the fallible result capture macro).
pub fn attempt(output: &Output, block: impl FnOnce() -> Result<(), Failure>) {
    if let Err(error) = block() {
        output.write(format_args!("{} ", error.exception_type()));
    }
}
/// Prints a fallible value with `print`, or the raised exception type.
pub fn show<T>(output: &Output, value: Result<T, Failure>, print: impl FnOnce(T)) {
    match value {
        Ok(value) => print(value),
        Err(error) => output.write(format_args!("{} ", error.exception_type())),
    }
}
pub fn real(output: &Output, value: f64) {
    output.write(format_args!("{} ", value.to_bits()));
}
pub fn int(output: &Output, value: i32) {
    output.write(format_args!("{value} "));
}
pub fn flag(output: &Output, value: bool) {
    output.write(format_args!("{} ", i32::from(value)));
}
/// Ends the output line of one request.
pub fn end(output: &Output) {
    output.write(format_args!("\n"));
}

/// One request's explicit output, independent of stdout and other operations.
#[derive(Default)]
pub struct Output(std::sync::Mutex<String>);
impl Output {
    pub fn write(&self, args: std::fmt::Arguments<'_>) {
        use std::fmt::Write;
        self.0
            .lock()
            .expect("output mutex")
            .write_fmt(args)
            .expect("string output");
    }
    pub fn finish(self) -> String {
        self.0.into_inner().expect("output mutex")
    }
}
