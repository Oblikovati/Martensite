//! Caller domain reads follow each cut store, before any evaluator callback.
// retained  the implementation6948–6965 writes cuts before iso2 fixed-axis capture.
use super::curve::IsoApproxRequest;
use super::curve_integers::CurveIntegerOutputs;
use crate::adv_app2var::AdvApp2VarEvaluatorFunc2Var;
pub(super) fn initialize_cut_parameters<Integers: CurveIntegerOutputs>(
    request: &mut IsoApproxRequest<'_>,
    evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    cuts: &mut [f64],
    bounds: &mut [f64; 4],
    integers: &mut Integers,
) -> bool {
    match request.isofav {
        1 => capture_iso_one(request, evaluator, cuts, bounds, integers),
        2 => capture_iso_two(request, evaluator, cuts, bounds, integers),
        _ => {
            integers.set_status(1, request);
            return false;
        }
    }
    true
}
fn capture_iso_one<Integers: CurveIntegerOutputs>(
    request: &mut IsoApproxRequest<'_>,
    evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    cuts: &mut [f64],
    bounds: &mut [f64; 4],
    integers: &mut Integers,
) {
    let domain = evaluator.caller_iso_domain(request.uvfonc);
    bounds[..2].copy_from_slice(&domain[..2]);
    integers.set_cut_parameter(
        cuts,
        0,
        evaluator.caller_iso_domain(request.uvfonc)[2],
        request,
    );
    integers.set_cut_parameter(
        cuts,
        1,
        evaluator.caller_iso_domain(request.uvfonc)[3],
        request,
    );
}
fn capture_iso_two<Integers: CurveIntegerOutputs>(
    request: &mut IsoApproxRequest<'_>,
    evaluator: &dyn AdvApp2VarEvaluatorFunc2Var,
    cuts: &mut [f64],
    bounds: &mut [f64; 4],
    integers: &mut Integers,
) {
    integers.set_cut_parameter(
        cuts,
        0,
        evaluator.caller_iso_domain(request.uvfonc)[0],
        request,
    );
    integers.set_cut_parameter(
        cuts,
        1,
        evaluator.caller_iso_domain(request.uvfonc)[1],
        request,
    );
    let domain = evaluator.caller_iso_domain(request.uvfonc);
    bounds[2..].copy_from_slice(&domain[2..]);
}
