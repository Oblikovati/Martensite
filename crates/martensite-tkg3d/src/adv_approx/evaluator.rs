//! `AdvApproxEvaluatorFunction`: the function an approximation samples.
//!
//! The retained virtual `Evaluate(int* Dimension, double StartEnd[2], double* Parameter,
//! int* DerivativeRequest, double* Result, int* ErrorCode)` passes every argument
//! through a pointer. The value-style method keeps ordinary evaluators concise;
//! `evaluate_native` exposes mutable input storage for callbacks whose writes affect
//! subsequent samples. Its default adapter writes the returned value to ErrorCode.

/// A function of one parameter with values in a space of `dimension` reals.
///
/// Closures with the matching signature implement it;
/// `let mut f = |_: i32, _: [f64; 2], t: f64, _: i32, r: &mut [f64]| { r[0] = t; 0 };`.
pub trait AdvApproxEvaluatorFunction {
    /// Writes the derivative of order `derivative_request` at `parameter` into
    /// `result[..dimension]`; `start_end` is the interval being approximated.
    /// Returns the error code, 0 on success (`Evaluate`);
    /// `let code: i32 = f.evaluate(1, [0.0, 1.0], 0.5, 0, &mut values);`.
    fn evaluate(
        &mut self,
        dimension: i32,
        start_end: [f64; 2],
        parameter: f64,
        derivative_request: i32,
        result: &mut [f64],
    ) -> i32;

    /// Receives the retained mutable callback protocol, including its error output.
    /// Existing value-style implementations use the default adapter;
    /// `fn evaluate_native(&mut self, call: &mut AdvApproxEvaluation<'_>)` can
    /// override it when callback mutations must affect later samples.
    fn evaluate_native(&mut self, call: &mut AdvApproxEvaluation<'_>) {
        *call.error_code = self.evaluate(
            *call.dimension,
            *call.start_end,
            *call.parameter,
            *call.derivative_request,
            call.result,
        );
    }
}

/// Borrowed inputs and outputs of retained `Evaluate`.
pub struct AdvApproxEvaluation<'a> {
    /// Mutable retained `Dimension`; result storage retains its original safe slice length.
    pub dimension: &'a mut i32,
    /// retained mutable interval pair, reused by `AdvApproxSimpleApprox::Perform`.
    pub start_end: &'a mut [f64; 2],
    /// retained parameter, whose lifetime follows the caller's local variable.
    pub parameter: &'a mut f64,
    /// retained derivative request, whose lifetime follows the caller's local variable.
    pub derivative_request: &'a mut i32,
    /// Result storage allocated for the original total dimension.
    pub result: &'a mut [f64],
    /// retained `ErrorCode` output; nonzero stops the current approximation.
    pub error_code: &'a mut i32,
}

impl<F> AdvApproxEvaluatorFunction for F
where
    F: FnMut(i32, [f64; 2], f64, i32, &mut [f64]) -> i32,
{
    fn evaluate(
        &mut self,
        dimension: i32,
        start_end: [f64; 2],
        parameter: f64,
        derivative_request: i32,
        result: &mut [f64],
    ) -> i32 {
        self(dimension, start_end, parameter, derivative_request, result)
    }
}
