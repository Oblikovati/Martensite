// the implementation at .

//! `RealFunction` and `MyGaussFunction`: the integrand of a curve length.
use crate::failure_capture::FailureCapture;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::math::Function;

/// `RealFunction`: a real function of one variable.
///
/// The retained typedef is a function pointer called with a `void*` client data; the
/// Rust closure captures its data, so `Init(F, D)` takes one argument. A raised
/// exception is an `Err`; `let f: RealFunction = Box::new(|x| Ok(x * x));`.
pub type RealFunction<'a> = Box<dyn Fn(f64) -> Result<f64, Failure> + 'a>;

/// `MyGaussFunction`: the `Function` whose value is `F(X)`;
/// `MyGaussFunction::new()`.
///
/// The math integrators take functions that report success as a flag, so an `Err` of
/// `F` is kept, reported as a failed value, and read back with [`Self::take_failure`]
/// by the caller right after the integrator, where the retained exception would surface.
#[derive(Default)]
pub struct MyGaussFunction<'a> {
    my_function: Option<RealFunction<'a>>,
    failure: FailureCapture,
}

impl<'a> MyGaussFunction<'a> {
    /// The empty function (`MyGaussFunction()`); `MyGaussFunction::new()`.
    ///
    /// Natively its function pointer is null and `Value` would call it; here
    /// `value` fails with null-object failure until [`Self::init`] is called.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the function (`Init(F, D)`); `gauss.init(Box::new(|x| Ok(2.0 * x)))`.
    pub fn init(&mut self, function: RealFunction<'a>) {
        self.my_function = Some(function);
    }
    /// `Err` with the first failure raised by `F` since the last call;
    /// `gauss.take_failure()?`.
    pub fn take_failure(&mut self) -> Result<(), Failure> {
        self.failure.take()
    }
    fn evaluate(&self, x: f64) -> Result<f64, Failure> {
        match &self.my_function {
            Some(function) => function(x),
            None => Err(Failure::with_type(
                "Standard_NullObject",
                "CPnts_MyGaussFunction::Value() - null function, expected Init(F, D)",
                "",
            )),
        }
    }
}

impl Function for MyGaussFunction<'_> {
    /// `Value(X, F)`: `F(X)`, always true natively (false only for a raised `F`).
    fn value(&mut self, x: f64, result: &mut f64) -> bool {
        let value: Result<f64, Failure> = self.evaluate(x);
        match self.failure.keep(value) {
            Some(value) => {
                *result = value;
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
#[path = "callback_context_tests.rs"]
mod callback_context_tests;
