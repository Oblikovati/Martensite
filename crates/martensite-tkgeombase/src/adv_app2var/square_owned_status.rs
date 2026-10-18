//! The owned status callback observes status/input identity through one frame.
use super::*;
use crate::adv_app2var::approx_f2var::{
    SquareInteger, SquareIntegerStorage, mma2ds1_integer_store,
};
use martensite_tkernel::standard::Failure;
use std::sync::{Mutex, OnceLock};
impl<Arguments> AdvApp2VarSquareEvaluation<'_, '_, Arguments> {
    /// Read actual caller status during evaluation; `let code=call.status()?`.
    pub fn status(&self) -> Result<i32, Failure> {
        self.frame.status()
    }
    /// Write status with immediate incident caller observation; `call.set_status(1)?`.
    pub fn set_status(&mut self, value: i32) -> Result<(), Failure> {
        self.frame.set_status(value)
    }
}
/// Square evaluator with indexed owned status and no separate error reference;
/// `let f=|call:&mut AdvApp2VarSquareEvaluation<'_, '_>, values:&mut[f64]|{values.fill(1.);call.set_status(0).unwrap();};`.
pub trait AdvApp2VarSquareStatusEvaluator {
    /// Evaluate using one owned status/input frame; `f.evaluate_square_status(&mut call,values)`.
    fn evaluate_square_status(
        &self,
        call: &mut AdvApp2VarSquareEvaluation<'_, '_>,
        values: &mut [f64],
    );
}
impl<F: Fn(&mut AdvApp2VarSquareEvaluation<'_, '_>, &mut [f64])> AdvApp2VarSquareStatusEvaluator
    for F
{
    fn evaluate_square_status(
        &self,
        call: &mut AdvApp2VarSquareEvaluation<'_, '_>,
        values: &mut [f64],
    ) {
        self(call, values);
    }
}
struct OwnedStatusEvaluator<'a, E>(&'a E);
impl<E: AdvApp2VarSquareStatusEvaluator> AdvApp2VarSquareEvaluator for OwnedStatusEvaluator<'_, E> {
    fn evaluate_square(
        &self,
        call: &mut AdvApp2VarSquareEvaluation<'_, '_>,
        values: &mut [f64],
        error: &mut i32,
    ) {
        self.0.evaluate_square_status(call, values);
        *error=call.status().expect("AdvApp2VarSquareOperationFrame: owned status missing; expected validated owned-status callback");
    }
}
impl Frame {
    /// Sample through the same square pipeline with initialized owned status;
    /// `frame.sample_square_owned_status(&mut request,&f,&mut tables,samples,parameters)?`.
    pub fn sample_square_owned_status(
        &mut self,
        request: &mut SampledSquare<'_>,
        evaluator: &impl AdvApp2VarSquareStatusEvaluator,
        tables: &mut SquareTables<'_>,
        samples: &mut [f64],
        parameters: &mut [f64],
    ) -> Result<i32, Failure> {
        self.require_owned_status()?;
        self.seed_request(request)?;
        let geometry = *request;
        let state = Mutex::new(OperationFrameState {
            frame: Some(self),
            failure: None,
        });
        let adapter = OwnedStatusEvaluator(evaluator);
        let bridge = SquareCallerBridge {
            state: &state,
            evaluator: &adapter,
            varying: OnceLock::new(),
        };
        let mut integers = OwnedSquareIntegers { state: &state };
        let result = mma2ds1_integer_store(
            geometry,
            &mut integers,
            &bridge,
            tables,
            samples,
            parameters,
        );
        let state = state.into_inner().map_err(|_| poisoned_square())?;
        state
            .frame
            .expect("AdvApp2VarSquareOperationFrame: frame=None; expected restored operation frame")
            .write_request(request);
        state.failure.map_or(Ok(result), Err)
    }
}
pub(super) struct OwnedSquareIntegers<'state, 'frame> {
    pub(super) state: &'state Mutex<OperationFrameState<'frame, Frame>>,
}
impl SquareIntegerStorage for OwnedSquareIntegers<'_, '_> {
    fn integer(&self, field: SquareInteger) -> i32 {
        self.with_resident(|frame|match field {
            SquareInteger::ErrorCode=>frame.status().expect("AdvApp2VarSquareOperationFrame: owned status missing; expected validated integer backend"),
            _=>frame.integer(caller_field(field)),
        })
    }
    fn set_integer(&mut self, field: SquareInteger, value: i32) {
        self.with_resident(|frame|match field {
            SquareInteger::ErrorCode=>frame.set_status(value).expect("AdvApp2VarSquareOperationFrame: owned status missing; expected validated integer backend"),
            _=>frame.set_integer(caller_field(field),value),
        });
    }
}
impl OwnedSquareIntegers<'_, '_> {
    fn with_resident<T>(&self, operation: impl FnOnce(&mut Frame) -> T) -> T {
        let mut state = self.state.lock().expect(
            "AdvApp2VarSquareOperationFrame: poisoned=true; expected unpoisoned integer backend",
        );
        let Some(frame) = state.frame.as_deref_mut() else {
            state.failure = Some(active_square());
            drop(state);
            panic!(
                "AdvApp2VarSquareOperationFrame: callback active=true; expected integer access outside evaluator callback"
            );
        };
        operation(frame)
    }
}
fn caller_field(field: SquareInteger) -> Field {
    match field {
        SquareInteger::Dimension => Field::Dimension,
        SquareInteger::URootCount => Field::URootCount,
        SquareInteger::VRootCount => Field::VRootCount,
        SquareInteger::FavoriteIso => Field::FavoriteIso,
        SquareInteger::ErrorCode => unreachable!("status has its own initialized slot"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    struct CountedSquareIntegers<'state, 'frame> {
        owned: OwnedSquareIntegers<'state, 'frame>,
        calls: Cell<usize>,
    }
    impl SquareIntegerStorage for CountedSquareIntegers<'_, '_> {
        fn integer(&self, field: SquareInteger) -> i32 {
            self.calls.set(self.calls.get() + 1);
            self.owned.integer(field)
        }
        fn set_integer(&mut self, field: SquareInteger, value: i32) {
            self.calls.set(self.calls.get() + 1);
            self.owned.set_integer(field, value);
        }
    }
    #[test]
    fn owned_backend_releases_guards_and_counts_actual_integer_accesses() {
        let mut owner = Frame::new(
            vec![3, 3, 3, 2, -1],
            crate::adv_app2var::AdvApp2VarSquareIntegerBindings {
                dimension: 0,
                root_counts: [1, 2],
                favorite_iso: 3,
            },
        )
        .unwrap()
        .with_owned_status_slot(4)
        .unwrap();
        let roots = [-(3_f64 / 5.).sqrt(), 0., (3_f64 / 5.).sqrt()];
        let request = SampledSquare {
            ndimen: 3,
            nbpntu: 3,
            nbpntv: 3,
            isofav: 2,
            uintfn: [0., 1.],
            vintfn: [0., 1.],
            urootb: &roots,
            vrootb: &roots,
        };
        owner.seed_request(&request).unwrap();
        let state = Mutex::new(OperationFrameState {
            frame: Some(&mut owner),
            failure: None,
        });
        let callbacks = Cell::new(0);
        let evaluator = |call: &mut AdvApp2VarSquareEvaluation<'_, '_>, values: &mut [f64]| {
            assert!(state.try_lock().is_ok());
            values[..9].fill(1.);
            call.set_status(0).unwrap();
            callbacks.set(callbacks.get() + 1);
        };
        let adapter = OwnedStatusEvaluator(&evaluator);
        let bridge = SquareCallerBridge {
            state: &state,
            evaluator: &adapter,
            varying: OnceLock::new(),
        };
        let mut integers = CountedSquareIntegers {
            owned: OwnedSquareIntegers { state: &state },
            calls: Cell::new(0),
        };
        let (mut sums, mut u, mut v, mut both, mut samples, mut parameters) =
            ([0.; 64], [0.; 64], [0.; 64], [0.; 64], [7.; 64], [7.; 64]);
        let mut tables = SquareTables {
            sosotb: &mut sums,
            disotb: &mut u,
            soditb: &mut v,
            diditb: &mut both,
        };
        assert_eq!(
            mma2ds1_integer_store(
                request,
                &mut integers,
                &bridge,
                &mut tables,
                &mut samples,
                &mut parameters
            ),
            0
        );
        assert_eq!(
            &tables.sosotb[..12],
            &[1., 2., 2., 4., 1., 2., 2., 4., 1., 2., 2., 4.]
        );
        assert_eq!(callbacks.get(), 3);
        assert!(integers.calls.get() > 0);
        println!(
            "{{\"metric\":\"owned_square_integer_locks\",\"backend_guard_calls\":{},\"callback_take_restore_guards\":{},\"axis_once_lock_bytes\":{}}}",
            integers.calls.get(),
            2 * callbacks.get(),
            std::mem::size_of::<OnceLock<Field>>()
        );
    }
    #[test]
    fn owned_integer_backend_reentry_latches_failure_without_poisoning() {
        let state = Mutex::new(OperationFrameState::<Frame> {
            frame: None,
            failure: None,
        });
        let integers = OwnedSquareIntegers { state: &state };
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || integers.integer(SquareInteger::Dimension)
            ))
            .is_err()
        );
        let state = state.into_inner().unwrap();
        assert!(
            state
                .failure
                .unwrap()
                .message()
                .contains("callback active=true")
        );
    }
}
