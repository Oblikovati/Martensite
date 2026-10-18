//! Sampling workspace tests behavior and supporting operations.
use super::*;
use crate::adv_app2var::math_base::transpose_workspace::fault_tests::FaultingTransposeWorkspace;
use crate::adv_app2var::{
    AdvApp2VarSquareIntegerBindings, AdvApp2VarSquareIntegerField as Field,
    AdvApp2VarSquareOperationFrame,
};
use std::cell::Cell;
struct BoundFrameIntegers(AdvApp2VarSquareOperationFrame);
impl SquareIntegerStorage for BoundFrameIntegers {
    fn integer(&self, field: SquareInteger) -> i32 {
        match field {
            SquareInteger::ErrorCode => self.0.status().unwrap(),
            SquareInteger::Dimension => self.0.integer(Field::Dimension),
            SquareInteger::URootCount => self.0.integer(Field::URootCount),
            SquareInteger::VRootCount => self.0.integer(Field::VRootCount),
            SquareInteger::FavoriteIso => self.0.integer(Field::FavoriteIso),
        }
    }
    fn set_integer(&mut self, field: SquareInteger, value: i32) {
        match field {
            SquareInteger::ErrorCode => self.0.set_status(value).unwrap(),
            SquareInteger::Dimension => self.0.set_integer(Field::Dimension, value),
            SquareInteger::URootCount => self.0.set_integer(Field::URootCount, value),
            SquareInteger::VRootCount => self.0.set_integer(Field::VRootCount, value),
            SquareInteger::FavoriteIso => self.0.set_integer(Field::FavoriteIso, value),
        }
    }
}
fn caller_integers(status_slot: usize) -> BoundFrameIntegers {
    BoundFrameIntegers(
        AdvApp2VarSquareOperationFrame::new(
            vec![1, 3, 5, 1, 7],
            AdvApp2VarSquareIntegerBindings {
                dimension: 0,
                root_counts: [1, 2],
                favorite_iso: 3,
            },
        )
        .unwrap()
        .with_owned_status_slot(status_slot)
        .unwrap(),
    )
}
fn square_request() -> SampledSquare<'static> {
    SampledSquare {
        ndimen: 1,
        uintfn: [0., 1.],
        vintfn: [0., 1.],
        nbpntu: 3,
        nbpntv: 5,
        urootb: &[-0.5, 0., 0.5],
        vrootb: &[-0.75, -0.25, 0., 0.25, 0.75],
        isofav: 1,
    }
}
fn execute_failure(
    workspace: &mut FaultingTransposeWorkspace,
    integers: &mut BoundFrameIntegers,
    callback_error: i32,
) -> ([f64; 64], usize) {
    let mut soso = [7.; 64];
    soso[..6].copy_from_slice(&[1., 2., 3., 4., 5., 6.]);
    let (mut diso, mut sodi, mut didi) = ([11.; 64], [13.; 64], [17.; 64]);
    let mut tables = SquareTables {
        sosotb: &mut soso,
        disotb: &mut diso,
        soditb: &mut sodi,
        diditb: &mut didi,
    };
    let mut storage = BorrowedSquareTables(&mut tables);
    let (mut params, mut results) = ([0.; 64], [0.; 64]);
    let mut buffers = AdvApp2VarEvaluationBuffers::separate(&mut params, &mut results, 64);
    let calls = Cell::new(0);
    let evaluator =
        |_: &crate::adv_app2var::AdvApp2VarEvaluation<'_>, values: &mut [f64], code: &mut i32| {
            calls.set(calls.get() + 1);
            values.fill(1.);
            *code = callback_error;
        };
    mma2ds1_real_store_with_workspace(
        square_request(),
        integers,
        &evaluator,
        &mut storage,
        &mut buffers,
        SquareInputBindings::default(),
        workspace,
    );
    (soso, calls.get())
}
#[test]
fn failures_at_each_of_eight_transposes_stop_and_translate_code_two() {
    for failure in 1..=8 {
        for release in [false, true] {
            let mut workspace = FaultingTransposeWorkspace {
                allocation_failure: (!release).then_some(failure),
                release_failure: release.then_some(failure),
                ..Default::default()
            };
            let mut integers = caller_integers(4);
            let (_, calls) = execute_failure(&mut workspace, &mut integers, 0);
            assert_eq!(integers.0.status().unwrap(), 102);
            assert_eq!(workspace.allocations, failure);
            assert_eq!(
                workspace.releases,
                if release { failure } else { failure - 1 }
            );
            assert_eq!(calls, if failure <= 4 { 0 } else { 3 });
        }
    }
}
#[test]
fn first_failure_distinguishes_unpublished_from_published_transpose() {
    for release in [false, true] {
        let mut workspace = FaultingTransposeWorkspace {
            allocation_failure: (!release).then_some(1),
            release_failure: release.then_some(1),
            ..Default::default()
        };
        let mut integers = caller_integers(4);
        let (soso, calls) = execute_failure(&mut workspace, &mut integers, 0);
        assert_eq!(calls, 0);
        assert_eq!(
            &soso[..6],
            if release {
                &[1., 3., 5., 2., 4., 6.]
            } else {
                &[1., 2., 3., 4., 5., 6.]
            }
        );
        assert_eq!(integers.0.status().unwrap(), 102);
    }
}
#[test]
fn final_transpose_failure_updates_incident_dimension_then_outer_error() {
    for release in [false, true] {
        let mut workspace = FaultingTransposeWorkspace {
            allocation_failure: (!release).then_some(1),
            release_failure: release.then_some(1),
            ..Default::default()
        };
        let mut integers = caller_integers(0);
        let (_, calls) = execute_failure(&mut workspace, &mut integers, 7);
        assert_eq!(calls, 1);
        assert_eq!(
            (
                integers.0.status().unwrap(),
                integers.0.integer(Field::Dimension)
            ),
            (102, 102)
        );
        assert_eq!(workspace.allocations, 1);
    }
}
