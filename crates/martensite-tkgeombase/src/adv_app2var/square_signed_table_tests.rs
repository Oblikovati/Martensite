//! retained initialized table prefixes and captured component strides share original owners.
use super::*;
use crate::adv_app2var::{
    AdvApp2VarSquareBufferEvaluation as Call, AdvApp2VarSquareIntegerBindings as Bindings,
    AdvApp2VarSquareIntervalAxis as IntervalAxis, AdvApp2VarSquareOperationFrame as Frame,
    AdvApp2VarSquareRootAxis as Axis, AdvApp2VarSquareTable as Table, approx_f2var::SampledSquare,
};
use std::sync::Mutex;
const ROOTS: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
fn integer_owner(transpose: bool) -> Vec<i32> {
    let mut values = vec![-7; 64];
    values[32..36].copy_from_slice(&[
        if transpose { 2 } else { 1 },
        -5,
        3,
        if transpose { 1 } else { 2 },
    ]);
    values
}
fn real_owner() -> Vec<f64> {
    let mut values = vec![7.; 512];
    values[8..12].copy_from_slice(&[0., 1., 0., 1.]);
    values[32..35].copy_from_slice(&ROOTS);
    values[48..51].copy_from_slice(&ROOTS);
    values[61] = 42.;
    values
}
fn request(transpose: bool) -> SampledSquare<'static> {
    SampledSquare {
        ndimen: if transpose { 2 } else { 1 },
        uintfn: [0., 1.],
        vintfn: [0., 1.],
        nbpntu: -5,
        nbpntv: 3,
        urootb: &[],
        vrootb: &[],
        isofav: if transpose { 1 } else { 2 },
    }
}
fn integer_frame(transpose: bool) -> Frame {
    Frame::new(
        integer_owner(transpose),
        Bindings {
            dimension: 32,
            root_counts: [33, 34],
            favorite_iso: 35,
        },
    )
    .unwrap()
    .with_owned_status_slot(if transpose { 33 } else { 36 })
    .unwrap()
}
fn sample_frame() -> AdvApp2VarSquareSampleFrame {
    AdvApp2VarSquareSampleFrame::new(real_owner(), 96, 64)
        .unwrap()
        .with_interval_origin(IntervalAxis::U, 8)
        .unwrap()
        .with_interval_origin(IntervalAxis::V, 10)
        .unwrap()
        .with_root_origin(Axis::U, 32)
        .unwrap()
        .with_root_origin(Axis::V, 48)
        .unwrap()
        .with_table_origins([128, 192, 256, 320])
        .unwrap()
}
fn expected_owners(transpose: bool) -> (Vec<i32>, Vec<f64>) {
    let (mut integers, mut values) = (integer_owner(transpose), real_owner());
    if transpose {
        integers[33] = 0;
        values[96..99].copy_from_slice(&ROOTS.map(|root| 0.5 + 0.5 * root));
        return (integers, values);
    }
    integers[36] = 0;
    values[127] += 42.;
    values[319] -= 42.;
    values[128] += 42.;
    values[127] += 42.;
    values[319] += 42.;
    (integers, values)
}

fn observe_signed_callback(call: &mut Call<'_, '_>, calls: &Mutex<Vec<f64>>) {
    use crate::adv_app2var::AdvApp2VarSquareIntegerField::{Dimension, URootCount};
    assert_eq!((call.integer(Dimension), call.integer(URootCount)), (1, -5));
    let ordinal = calls.lock().unwrap().len();
    assert_eq!(
        call.caller_table_signed(Table::SumSum, -1).unwrap(),
        if ordinal == 0 { 7. } else { 49. }
    );
    calls.lock().unwrap().push(*call.constant_parameter());
    call.set_status(0).unwrap();
}
fn check_published_integers(mut frame: Frame, expected: &[i32]) {
    // Rebinding an inspection role never mutates its initialized integer owner.
    for (index, value) in expected.iter().enumerate() {
        frame = frame.with_owned_status_slot(index).unwrap();
        assert_eq!(frame.status().unwrap(), *value, "integer={index}");
    }
}
fn check_published_tables(samples: &AdvApp2VarSquareSampleFrame, expected: &[f64]) {
    assert_eq!(
        samples.table_signed(Table::SumSum, -1).unwrap(),
        expected[127]
    );
    assert_eq!(samples.table(Table::SumSum, 0).unwrap(), expected[128]);
    for offset in [-129, 384, isize::MIN, isize::MAX] {
        assert!(samples.table_signed(Table::SumSum, offset).is_err());
    }
    for (index, (actual, expected)) in samples.values.iter().zip(expected).enumerate() {
        assert_eq!(actual.to_bits(), expected.to_bits(), "real={index}");
    }
}
fn check_published_request(request: &SampledSquare<'_>, expected: &[i32]) {
    assert_eq!(
        (
            request.ndimen,
            request.nbpntu,
            request.nbpntv,
            request.isofav
        ),
        (expected[32], expected[33], expected[34], expected[35])
    );
}
struct SignedTableCase {
    frame: Frame,
    samples: AdvApp2VarSquareSampleFrame,
    request: SampledSquare<'static>,
    calls: Mutex<Vec<f64>>,
}
impl SignedTableCase {
    fn new(transpose: bool) -> Self {
        Self {
            frame: integer_frame(transpose),
            samples: sample_frame(),
            request: request(transpose),
            calls: Mutex::new(Vec::new()),
        }
    }
    fn run(&mut self) {
        let evaluator = |call: &mut Call<'_, '_>| observe_signed_callback(call, &self.calls);
        self.frame
            .sample_square_owned_tables(&mut self.request, &evaluator, &mut self.samples)
            .unwrap();
    }
    fn verify(self, transpose: bool) {
        let expected_calls = if transpose {
            Vec::new()
        } else {
            ROOTS.map(|root| 0.5 + 0.5 * root).to_vec()
        };
        assert_eq!(self.calls.into_inner().unwrap(), expected_calls);
        let (integers, values) = expected_owners(transpose);
        check_published_request(&self.request, &integers);
        check_published_integers(self.frame, &integers);
        check_published_tables(&self.samples, &values);
    }
}
fn run_signed_table_case(transpose: bool) {
    let mut case = SignedTableCase::new(transpose);
    case.run();
    case.verify(transpose);
}
#[test]
fn initialized_folded_table_prefix_retains_signed_row_stride() {
    run_signed_table_case(false);
}
#[test]
fn initialized_transpose_component_prefix_retains_entry_stride() {
    run_signed_table_case(true);
}
#[path = "square_signed_table_contract_tests.rs"]
mod contracts;
