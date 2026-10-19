//! Owned indexed square samples compose with actual caller integer/status slots.
use super::owned_status::OwnedSquareIntegers;
use super::*;
use crate::adv_app2var::{
    AdvApp2VarSquareSampleFrame,
    approx_f2var::{BorrowedSquareTables, SquareTableStorage, mma2ds1_real_store},
};
/// Square evaluator with indexed shared samples and owned caller status;
/// `let f=|call:&mut AdvApp2VarSquareBufferEvaluation<'_, '_>|{call.buffers().set_result(0,1.);call.set_status(0).unwrap();};`.
pub trait AdvApp2VarSquareBufferEvaluator {
    /// Evaluate actual shared slots; `f.evaluate_square_buffers(&mut call)`.
    fn evaluate_square_buffers(&self, call: &mut AdvApp2VarSquareBufferEvaluation<'_, '_>);
}
impl<F: Fn(&mut AdvApp2VarSquareBufferEvaluation<'_, '_>)> AdvApp2VarSquareBufferEvaluator for F {
    fn evaluate_square_buffers(&self, call: &mut AdvApp2VarSquareBufferEvaluation<'_, '_>) {
        self(call);
    }
}
impl Frame {
    /// Sample using an owned parameter/sample allocation and actual owned status;
    /// `frame.sample_square_owned_samples(&mut request,&f,&mut tables,&mut samples)?`.
    pub fn sample_square_owned_samples(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareBufferEvaluator,
        tables: &mut SquareTables<'_>,
        samples: &mut AdvApp2VarSquareSampleFrame,
    ) -> Result<i32, Failure> {
        samples.require_unbound_intervals()?;
        samples.require_unbound_tables()?;
        samples.require_unbound_roots()?;
        self.sample_square_bound_samples(request, evaluator, tables, samples)
    }
    /// Sample actual owned roots, requiring empty borrowed root fields on bound axes;
    /// `frame.sample_square_owned_roots(&mut request,&f,&mut tables,&mut samples)?`.
    pub fn sample_square_owned_roots(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareBufferEvaluator,
        tables: &mut SquareTables<'_>,
        samples: &mut AdvApp2VarSquareSampleFrame,
    ) -> Result<i32, Failure> {
        samples.require_unbound_intervals()?;
        samples.require_unbound_tables()?;
        samples.validate_owned_root_inputs(request)?;
        self.sample_square_bound_samples(request, evaluator, tables, samples)
    }
    /// Sample tables in the same initialized allocation as parameters, roots or samples;
    /// `frame.sample_square_owned_tables(&mut request,&f,&mut samples)?`.
    pub fn sample_square_owned_tables(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareBufferEvaluator,
        samples: &mut AdvApp2VarSquareSampleFrame,
    ) -> Result<i32, Failure> {
        samples.validate_interval_seed(request)?;
        let mut tables = samples.table_bindings();
        tables.start(super::super::AdvApp2VarSquareTable::SumSum)?;
        if samples.root_bindings().has_roots() {
            samples.validate_owned_root_inputs(request)?;
        }
        self.sample_square_bound_storage(request, evaluator, &mut tables, samples)
    }
    fn sample_square_bound_samples(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareBufferEvaluator,
        tables: &mut SquareTables<'_>,
        samples: &mut AdvApp2VarSquareSampleFrame,
    ) -> Result<i32, Failure> {
        let mut tables = BorrowedSquareTables(tables);
        self.sample_square_bound_storage(request, evaluator, &mut tables, samples)
    }
    fn sample_square_bound_storage(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareBufferEvaluator,
        tables: &mut dyn SquareTableStorage,
        samples: &mut AdvApp2VarSquareSampleFrame,
    ) -> Result<i32, Failure> {
        self.require_owned_status()?;
        self.seed_request(request)?;
        let geometry = *request;
        let state = Mutex::new(OperationFrameState {
            frame: Some(self),
            failure: None,
        });
        let bridge = IndexedSquareCallerBridge {
            state: &state,
            evaluator,
            varying: OnceLock::new(),
            roots: samples.root_bindings(),
            tables: samples.table_bindings(),
            inputs: samples.input_bindings(),
        };
        let mut integers = OwnedSquareIntegers { state: &state };
        let result = mma2ds1_real_store(
            geometry,
            &mut integers,
            &bridge,
            tables,
            &mut samples.buffers(),
            bridge.inputs,
        );
        let state = state.into_inner().map_err(|_| poisoned_square())?;
        state
            .frame
            .expect("AdvApp2VarSquareOperationFrame: frame=None; expected restored operation frame")
            .write_request(request);
        samples.write_intervals(request);
        state.failure.map_or(Ok(result), Err)
    }
}
struct IndexedSquareCallerBridge<'state, 'frame, 'eval, E> {
    state: &'state Mutex<OperationFrameState<'frame, Frame>>,
    evaluator: &'eval E,
    varying: OnceLock<Field>,
    roots: SquareRootBindings,
    tables: super::super::square_tables::SquareTableBindings,
    inputs: crate::adv_app2var::square_intervals::SquareInputBindings,
}
impl<E: AdvApp2VarSquareBufferEvaluator> AdvApp2VarEvaluatorFunc2Var
    for IndexedSquareCallerBridge<'_, '_, '_, E>
{
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], error: &mut i32) {
        *error = 1;
    }
    fn evaluate_square_buffers(&self, call: &mut AdvApp2VarBufferEvaluation<'_>, error: &mut i32) {
        evaluate_square_frame(
            self.state,
            &self.varying,
            call,
            error,
            |frame, call, varying, error| {
                let mut view = AdvApp2VarSquareBufferEvaluation {
                    frame,
                    call,
                    varying,
                    callback_lifetime: std::marker::PhantomData,
                    roots: self.roots,
                    tables: self.tables,
                    intervals: self.inputs.intervals,
                };
                self.evaluator.evaluate_square_buffers(&mut view);
                *error=view.status().expect("AdvApp2VarSquareOperationFrame: owned status missing; expected validated indexed callback");
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn indexed_callback_holds_no_state_guard_and_restores_actual_frame_on_unwind() {
        let mut owner = Frame::new(
            vec![1, 3, 3, 2],
            crate::adv_app2var::AdvApp2VarSquareIntegerBindings {
                dimension: 0,
                root_counts: [1, 2],
                favorite_iso: 3,
            },
        )
        .unwrap()
        .with_owned_status_slot(0)
        .unwrap();
        let state = Mutex::new(OperationFrameState {
            frame: Some(&mut owner),
            failure: None,
        });
        let evaluator = |call: &mut AdvApp2VarSquareBufferEvaluation<'_, '_>| {
            assert!(state.try_lock().is_ok());
            call.buffers().set_result(0, 0.25);
            assert_eq!(call.buffers().parameter(0), 0.25);
            call.set_status(2).unwrap();
            assert_eq!(call.argument(Argument::Dimension), 2);
            panic!("indexed square callback restoration fixture");
        };
        let bridge = IndexedSquareCallerBridge {
            state: &state,
            evaluator: &evaluator,
            varying: OnceLock::new(),
            roots: SquareRootBindings::default(),
            tables: crate::adv_app2var::square_tables::SquareTableBindings::default(),
            inputs: crate::adv_app2var::square_intervals::SquareInputBindings::default(),
        };
        let (mut dimension, mut iso, mut count, mut order) = (1, 2, 3, 0);
        let (mut u, mut v, mut constant, mut values) = ([0., 1.], [0., 1.], 0.5, [7.; 64]);
        let mut call = AdvApp2VarBufferEvaluation {
            dimension: &mut dimension,
            favor_iso: &mut iso,
            nb_params: &mut count,
            u_start_end: &mut u,
            v_start_end: &mut v,
            const_param: &mut constant,
            orders: crate::adv_app2var::AdvApp2VarDerivativeOrders::Shared(&mut order),
            buffers: AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 64, 0, 64),
        };
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || bridge.evaluate_square_buffers(&mut call, &mut 0)
            ))
            .is_err()
        );
        assert_eq!(call.buffers.parameter(0), 0.25);
        let state = state.into_inner().unwrap();
        assert!(state.failure.is_none());
        assert_eq!(state.frame.unwrap().status().unwrap(), 2);
    }
}
