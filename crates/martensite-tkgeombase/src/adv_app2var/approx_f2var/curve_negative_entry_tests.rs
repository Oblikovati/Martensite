//! Negative entry loop bounds retain source skipping and original failure output.
use super::*;
struct EmptyNegativeSampling;
impl AdvApp2VarEvaluatorFunc2Var for EmptyNegativeSampling {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("negative count callback uses original indexed owner")
    }
    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        _: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        assert_eq!((*call.dimension, *call.nb_params), (1, -1));
        assert_eq!(call.buffers.allocation_value(0).unwrap(), 1.);
        *error = 0;
    }
}
#[test]
fn negative_entry_root_bound_skips_the_parameter_root_loop() {
    for (iso, upper) in [(1, 5.), (2, 2.)] {
        let mut parameters = [7.; 6];
        let mut error = 9;
        super::super::curve_steps::mma1nop_(
            -1,
            &[],
            &[0., 2., 3., 5.],
            iso,
            &mut parameters,
            &mut error,
        );
        assert_eq!(parameters, [upper, 7., 7., 7., 7., 7.]);
        assert_eq!(error, 9);
    }
}
#[test]
fn negative_entry_rule_reaches_projection_failure_without_sampling() {
    for iso in [1, 2] {
        verify_negative_entry(iso);
    }
}
fn verify_negative_entry(iso: i32) {
    let mut request = prefix_request(iso, &[]);
    request.nbroot = -1;
    let mut outputs = PrefixOutputs::new();
    let mut output = outputs.output();
    let mut error = -7;
    mma2fnc_mut_(
        &mut request,
        &EmptyNegativeSampling,
        &mut output,
        &mut error,
    );
    assert_eq!((request.nbroot, output.nbcrbe, error), (-1, 0, 1));
    let mut expected = [[7.; 64]; 5];
    expected[0][..3].fill(0.);
    expected[1][0] = 0.;
    expected[2][0] = 0.;
    assert_eq!(outputs.tables, expected);
    outputs.assert_untouched_controls();
}
