//! Original sample addresses can refer to initialized parameter-prefix storage.
use super::*;
use crate::adv_app2var::evaluator::AdvApp2VarEvaluatorFunc2Var;
use crate::adv_app2var::{
    AdvApp2VarBufferEvaluation, AdvApp2VarCallerSamplingField, AdvApp2VarEvaluation,
    AdvApp2VarIsoCallerIntegers,
};

struct InitializedSamplePrefix;
impl AdvApp2VarEvaluatorFunc2Var for InitializedSamplePrefix {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("original-owner sample prefix requires indexed callback storage")
    }
    fn evaluate_iso_buffers(
        &self,
        call: &mut AdvApp2VarBufferEvaluation<'_>,
        callers: &mut AdvApp2VarIsoCallerIntegers<'_>,
        error: &mut i32,
    ) {
        assert_eq!((*call.dimension, *call.nb_params), (1, 8));
        assert_eq!(call.buffers.allocation_value(9).unwrap(), 1.);
        call.buffers.set_allocation_value(10, 3.).unwrap();
        for index in 0..8 {
            call.buffers.set_result(index, 1.);
        }
        callers
            .set_sampling(AdvApp2VarCallerSamplingField::RootCount, -5)
            .unwrap();
        *error = 0;
    }
}
fn prefix_request(iso: i32, roots: &[f64]) -> IsoApproxRequest<'_> {
    IsoApproxRequest {
        ndimen: 1,
        nbsesp: 1,
        ndimse: &[1],
        uvfonc: [0., 1., 0., 1.],
        tconst: 0.5,
        isofav: iso,
        nbroot: 8,
        rootlg: roots,
        iordre: -1,
        ideriv: 0,
        ndgjac: 2,
        nbcrmx: 1,
        ncflim: 3,
        epsapr: &[1e-6],
    }
}
#[test]
fn initialized_parameter_prefix_is_reachable_from_live_sample_indices() {
    for iso in [1, 2] {
        verify_prefix_operation(iso);
    }
}
struct PrefixOutputs {
    tables: [[f64; 64]; 5],
    counts: [i32; 6],
    cuts: [f64; 12],
    maximum: [f64; 12],
    mean: [f64; 12],
}
impl PrefixOutputs {
    fn new() -> Self {
        Self {
            tables: [[7.; 64]; 5],
            counts: [-7; 6],
            cuts: [7.; 12],
            maximum: [7.; 12],
            mean: [7.; 12],
        }
    }
    fn output(&mut self) -> IsoApproxOutput<'_> {
        let [curve, sums, differences, lower, upper] = &mut self.tables;
        IsoApproxOutput {
            ncoeff: &mut self.counts,
            courbe: curve,
            nbcrbe: -7,
            somtab: sums,
            diftab: differences,
            contr1: lower,
            contr2: upper,
            tabdec: &mut self.cuts,
            errmax: &mut self.maximum,
            errmoy: &mut self.mean,
        }
    }
    fn assert_untouched_outputs(&self) {
        assert_prefix_tables(&self.tables);
        self.assert_untouched_controls();
    }
    fn assert_untouched_controls(&self) {
        assert_eq!(self.counts, [-7; 6]);
        assert_eq!(&self.cuts[..2], &[0., 1.]);
        assert_eq!(&self.cuts[2..], &[7.; 10]);
        assert_eq!((self.maximum, self.mean), ([7.; 12], [7.; 12]));
    }
}
fn verify_prefix_operation(iso: i32) {
    let (mut roots, mut other) = ([0.; 8], [0.; 8]);
    mma2roo_(8, 8, &mut roots, &mut other);
    let mut request = prefix_request(iso, &roots);
    let mut outputs = PrefixOutputs::new();
    let mut output = outputs.output();
    let mut error = -7;
    mma2fnc_mut_(
        &mut request,
        &InitializedSamplePrefix,
        &mut output,
        &mut error,
    );
    assert_eq!((request.nbroot, output.nbcrbe, error), (-5, 0, 1));
    outputs.assert_untouched_outputs();
}
fn assert_prefix_tables(tables: &[[f64; 64]; 5]) {
    let mut expected = [[7.; 64]; 5];
    expected[0][..3].fill(0.);
    // Captured four pairs use live positive indices -1,0,1,2. Samples -1/0
    // are the initialized right parameter endpoint1 and callback-written3.
    expected[1][..5].copy_from_slice(&[0., 2., 4., 2., 2.]);
    expected[2][..5].copy_from_slice(&[0., 0., 2., 0., 0.]);
    assert_eq!(tables, &expected);
}

#[path = "curve_negative_entry_tests.rs"]
mod negative_entry_tests;
