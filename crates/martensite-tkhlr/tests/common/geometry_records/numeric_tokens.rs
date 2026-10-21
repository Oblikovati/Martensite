#![allow(dead_code, missing_docs)] // Each test uses the subset it prints.
//! The output tokens of the retained numeric records.
use martensite_tkernel::standard::Failure;

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
/// Sequential reader over one request's numeric fields.
pub struct Reader<'v> {
    values: &'v [f64],
    next: usize,
}
impl<'v> Reader<'v> {
    pub fn new(values: &'v [f64]) -> Self {
        Self { values, next: 0 }
    }
    pub fn real(&mut self) -> f64 {
        self.next += 1;
        self.values[self.next - 1]
    }
    pub fn integer(&mut self) -> i32 {
        self.real() as i32
    }
    pub fn flag(&mut self) -> bool {
        self.integer() != 0
    }
    pub fn reals(&mut self) -> Vec<f64> {
        let n = self.integer();
        (0..n).map(|_| self.real()).collect()
    }
}
