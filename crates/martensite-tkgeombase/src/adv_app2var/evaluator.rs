// .

//! `EvaluatorFunc2Var`: the function of two parameters an approximation
//! samples.
//!
//! The retained virtual `Evaluate` passes every argument through a pointer. Legacy
//! evaluators can use the read-only `evaluate` view; `evaluate_native` exposes the
//! mutable argument values to callbacks that need them. `evaluate_slots` also
//! exposes derivative-argument alias identity without duplicate mutable borrows.
//! `evaluate_iso_callers` carries actual caller controls separately from local
//! U/V derivative arguments; changing a caller order does not replace those locals.
//! `operator()` forwards to
//! the same retained method. The trait takes `&self` like the retained `const` member.

/// A function `F(U, V)` with values in a space of `dimension` reals.
///
/// Closures with the matching signature implement it;
/// `let f = |_: &AdvApp2VarEvaluation<'_>, r: &mut [f64], e: &mut i32| { r.fill(0.0); *e = 0; };`.
pub trait AdvApp2VarEvaluatorFunc2Var {
    /// Evaluates the derivative `(u_order, v_order)` of `F` on the iso of constant
    /// parameter `const_param` (U when `favor_iso` is 1, V otherwise) at the
    /// `nb_params` values `parameters` of the other parameter, into
    /// `result[dimension * nb_params]`; a positive `error_code` stops the kernel
    /// (`Evaluate`). `f.evaluate(&request, &mut values, &mut code)`.
    fn evaluate(
        &self,
        request: &AdvApp2VarEvaluation<'_>,
        result: &mut [f64],
        error_code: &mut i32,
    );

    /// Read the operation-owned caller dimension array after evaluation. Override
    /// when the evaluator captures mutable dimensions; otherwise the request's
    /// borrowed dimensions remain the input without allocation.
    /// `let dimensions = f.caller_subspace_dimensions(&[2]);`.
    fn caller_subspace_dimensions<'a>(
        &'a self,
        fallback: &'a [i32],
    ) -> std::borrow::Cow<'a, [i32]> {
        std::borrow::Cow::Borrowed(fallback)
    }

    /// Read the operation-owned requested approximation limits after evaluation.
    /// Existing evaluators keep the request's borrowed per-subspace limits;
    /// mutable owners override this projection without changing achieved errors.
    /// `let limits = f.caller_subspace_tolerances(&[1e-6]);`.
    fn caller_subspace_tolerances<'a>(
        &'a self,
        fallback: &'a [f64],
    ) -> std::borrow::Cow<'a, [f64]> {
        std::borrow::Cow::Borrowed(fallback)
    }

    /// Read original square caller counts in U/V order at live source read sites;
    /// `let counts=f.caller_square_root_counts();` is `None` for ordinary evaluators.
    fn caller_square_root_counts(&self) -> Option<[i32; 2]> {
        None
    }

    /// Read caller domain bounds after initial output clearing, before the
    /// operation copies its piece bounds. Ordinary evaluators retain the input;
    /// `let bounds = f.caller_iso_domain([0.,1.,0.,1.]);`.
    fn caller_iso_domain(&self, fallback: [f64; 4]) -> [f64; 4] {
        fallback
    }

    /// Read the operation-owned iso roots before generating parameters and after
    /// evaluation. Mutable owners supply their actual current abscissae; existing
    /// evaluators borrow the request input without allocation.
    /// `let roots = f.caller_iso_roots(&[-0.5, 0.5]);`.
    fn caller_iso_roots<'a>(&'a self, fallback: &'a [f64]) -> std::borrow::Cow<'a, [f64]> {
        std::borrow::Cow::Borrowed(fallback)
    }

    /// Evaluates with distinct mutable argument references. For immediate observation
    /// of aliased derivative arguments, override [`Self::evaluate_slots`]. Existing implementations
    /// keep their read-only behavior through this default adapter.
    /// `f.evaluate_native(&mut call, &mut values, &mut code)`.
    fn evaluate_native(
        &self,
        call: &mut AdvApp2VarEvaluationMut<'_>,
        result: &mut [f64],
        error_code: &mut i32,
    ) {
        self.evaluate(&call.snapshot(), result, error_code);
    }
    /// Evaluate through identity-preserving derivative slots. Override this method
    /// when the callback must observe a U write immediately through an aliased V
    /// argument. The default retains the existing independent-reference adapter;
    /// `f.evaluate_slots(&mut call, &mut values, &mut code)`.
    fn evaluate_slots(
        &self,
        call: &mut super::AdvApp2VarEvaluationSlots<'_>,
        result: &mut [f64],
        error_code: &mut i32,
    ) {
        call.evaluate_legacy(self, result, error_code);
    }
    /// Evaluate with the caller's subspace-count context, which retained callbacks
    /// may capture even though it is not an Evaluate argument. Existing adapters
    /// retain their behavior; `f.evaluate_iso_slots(&mut call, &mut spaces, values, error)`.
    fn evaluate_iso_slots(
        &self,
        call: &mut super::AdvApp2VarEvaluationSlots<'_>,
        _subspace_count: &mut i32,
        result: &mut [f64],
        error_code: &mut i32,
    ) {
        self.evaluate_slots(call, result, error_code);
    }
    /// Evaluate square samples through indexed parameter/result storage. The
    /// default preserves existing disjoint slices and shared derivative identity;
    /// `f.evaluate_square_buffers(&mut call,error)`.
    fn evaluate_square_buffers(
        &self,
        call: &mut super::AdvApp2VarBufferEvaluation<'_>,
        error: &mut i32,
    ) {
        call.evaluate_square_legacy(self, error);
    }
    /// Evaluate with indexed operation buffers, preserving immediate observation
    /// when parameter and result regions overlap. The default retains the original
    /// disjoint callback views; override this hook for shared-buffer operations.
    /// `f.evaluate_iso_buffers(&mut call, &mut callers, &mut error)`.
    fn evaluate_iso_buffers(
        &self,
        call: &mut super::AdvApp2VarBufferEvaluation<'_>,
        callers: &mut super::AdvApp2VarIsoCallerIntegers<'_>,
        error_code: &mut i32,
    ) {
        call.evaluate_legacy(self, callers, error_code);
    }
    /// Evaluate with actual caller integer controls, separate from callback-local
    /// derivative arguments. The default preserves the earlier count adapter;
    /// `f.evaluate_iso_callers(&mut call, &mut callers, values, error)`.
    fn evaluate_iso_callers(
        &self,
        call: &mut super::AdvApp2VarEvaluationSlots<'_>,
        callers: &mut super::AdvApp2VarIsoCallerIntegers<'_>,
        result: &mut [f64],
        error_code: &mut i32,
    ) {
        self.evaluate_iso_slots(call, callers.subspace_count, result, error_code);
    }
}

/// Mutable arguments passed to one retained `Evaluate` call.
pub struct AdvApp2VarEvaluationMut<'a> {
    /// `theDimension`: number of reals per value.
    pub dimension: &'a mut i32,
    /// `theUStartEnd`: first and last parameters in U.
    pub u_start_end: &'a mut [f64; 2],
    /// `theVStartEnd`: first and last parameters in V.
    pub v_start_end: &'a mut [f64; 2],
    /// `theFavorIso`: 1 for constant U, 2 for constant V.
    pub favor_iso: &'a mut i32,
    /// `theConstParam`: the value of the constant parameter.
    pub const_param: &'a mut f64,
    /// `theNbParams`: number of parameters.
    pub nb_params: &'a mut i32,
    /// `theParameters`: mutable parameter storage beginning at the retained pointer.
    pub parameters: &'a mut [f64],
    /// `theUOrder`: derivative order in U.
    pub u_order: &'a mut i32,
    /// `theVOrder`: derivative order in V.
    pub v_order: &'a mut i32,
}

impl AdvApp2VarEvaluationMut<'_> {
    fn snapshot(&self) -> AdvApp2VarEvaluation<'_> {
        AdvApp2VarEvaluation {
            dimension: *self.dimension,
            u_start_end: *self.u_start_end,
            v_start_end: *self.v_start_end,
            favor_iso: *self.favor_iso,
            const_param: *self.const_param,
            nb_params: *self.nb_params,
            parameters: self.parameters,
            u_order: *self.u_order,
            v_order: *self.v_order,
        }
    }
}

/// The inputs of one `Evaluate` call, in the retained argument order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AdvApp2VarEvaluation<'a> {
    /// `theDimension`: number of reals per value.
    pub dimension: i32,
    /// `theUStartEnd`: first and last parameters in U.
    pub u_start_end: [f64; 2],
    /// `theVStartEnd`: first and last parameters in V.
    pub v_start_end: [f64; 2],
    /// `theFavorIso`: 1 when U is constant, 2 when V is.
    pub favor_iso: i32,
    /// `theConstParam`: the value of the constant parameter.
    pub const_param: f64,
    /// `theNbParams`: the number of parameter values.
    pub nb_params: i32,
    /// `theParameters`: the values of the other parameter.
    pub parameters: &'a [f64],
    /// `theUOrder`: derivative order in U.
    pub u_order: i32,
    /// `theVOrder`: derivative order in V.
    pub v_order: i32,
}

impl<F> AdvApp2VarEvaluatorFunc2Var for F
where
    F: Fn(&AdvApp2VarEvaluation<'_>, &mut [f64], &mut i32),
{
    fn evaluate(
        &self,
        request: &AdvApp2VarEvaluation<'_>,
        result: &mut [f64],
        error_code: &mut i32,
    ) {
        self(request, result, error_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_caller_dimensions_borrow_the_existing_request_input() {
        let evaluator = |_: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32| {};
        let dimensions = [2, 3];
        let actual = evaluator.caller_subspace_dimensions(&dimensions);
        assert!(matches!(actual, std::borrow::Cow::Borrowed(_)));
        assert_eq!(actual.as_ref(), dimensions);
    }

    #[test]
    fn default_caller_tolerances_borrow_the_existing_requested_limits() {
        let evaluator = |_: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32| {};
        let tolerances = [1e-6, 2e-6];
        let actual = evaluator.caller_subspace_tolerances(&tolerances);
        assert!(matches!(actual, std::borrow::Cow::Borrowed(_)));
        assert_eq!(actual.as_ref(), tolerances);
    }

    #[test]
    fn default_square_caller_counts_are_unconfigured() {
        let evaluator = |_: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32| {};
        assert_eq!(evaluator.caller_square_root_counts(), None);
    }

    #[test]
    fn default_caller_domain_retains_the_existing_request_bounds() {
        let evaluator = |_: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32| {};
        let bounds = [0., 1., 2., 3.];
        assert_eq!(evaluator.caller_iso_domain(bounds), bounds);
    }

    #[test]
    fn default_caller_roots_borrow_the_existing_request_abscissae() {
        let evaluator = |_: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32| {};
        let roots = [-0.5, 0.5];
        let actual = evaluator.caller_iso_roots(&roots);
        assert!(matches!(actual, std::borrow::Cow::Borrowed(_)));
        assert_eq!(actual.as_ref(), roots);
    }

    #[test]
    fn closures_are_evaluators_and_may_leave_the_error_code() {
        let f = |request: &AdvApp2VarEvaluation<'_>, result: &mut [f64], _: &mut i32| {
            for (value, t) in result.iter_mut().zip(request.parameters) {
                *value = request.const_param + t;
            }
        };
        let request = AdvApp2VarEvaluation {
            dimension: 1,
            u_start_end: [0.0, 1.0],
            v_start_end: [0.0, 1.0],
            favor_iso: 1,
            const_param: 0.5,
            nb_params: 2,
            parameters: &[0.25, 0.75],
            u_order: 0,
            v_order: 0,
        };
        let (mut result, mut code) = ([0.0; 2], -4);
        f.evaluate(&request, &mut result, &mut code);
        assert_eq!((result, code), ([0.75, 1.25], -4));
    }
}
