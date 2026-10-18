//! Evaluator behavior and supporting operations.
use super::{ROOTS, ZeroCase, bits};
use martensite_tkgeombase::adv_app2var::{AdvApp2VarEvaluation, AdvApp2VarEvaluatorFunc2Var};
use std::sync::Mutex;
pub(crate) struct ZeroEvaluator {
    case: ZeroCase,
    calls: Mutex<usize>,
}
impl ZeroEvaluator {
    pub(crate) fn new(case: ZeroCase) -> Self {
        Self {
            case,
            calls: Mutex::new(0),
        }
    }
    pub(crate) fn assert_one_call(&self) {
        assert_eq!(*self.calls.lock().unwrap(), 1);
    }
    fn check_request(&self, request: &AdvApp2VarEvaluation<'_>) {
        assert_eq!(
            (request.dimension, request.favor_iso),
            (self.case.dimension, 1)
        );
        assert_eq!((request.u_order, request.v_order), (0, 0));
        assert_eq!(bits(&request.u_start_end), bits(&[0., 1.]));
        assert_eq!(bits(&request.v_start_end), bits(&[0., 1.]));
        assert_eq!(request.const_param.to_bits(), 0.5_f64.to_bits());
        let mut parameters: Vec<f64> = ROOTS.iter().map(|root| 0.5 * root + 0.5).collect();
        if self.case.order == 0 {
            parameters.insert(0, 0.);
            parameters.push(1.);
        }
        assert_eq!(request.nb_params as usize, parameters.len());
        assert_eq!(
            bits(&request.parameters[..parameters.len()]),
            bits(&parameters)
        );
    }
}
impl AdvApp2VarEvaluatorFunc2Var for ZeroEvaluator {
    fn evaluate(&self, request: &AdvApp2VarEvaluation<'_>, result: &mut [f64], error: &mut i32) {
        self.check_request(request);
        let before = bits(request.parameters);
        let count = (request.dimension * request.nb_params) as usize;
        let untouched = bits(&result[count..]);
        result[..count].fill(0.);
        *error = 0;
        self.check_request(request);
        assert_eq!(bits(request.parameters), before);
        assert_eq!(bits(&result[count..]), untouched);
        *self.calls.lock().unwrap() += 1;
    }
}
