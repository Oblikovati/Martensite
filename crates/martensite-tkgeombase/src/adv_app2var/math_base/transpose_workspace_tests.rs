//! Transpose workspace tests behavior and supporting operations.
use super::*;
#[derive(Default)]
pub(crate) struct FaultingTransposeWorkspace {
    pub(crate) allocation_failure: Option<usize>,
    pub(crate) release_failure: Option<usize>,
    pub(crate) allocations: usize,
    pub(crate) releases: usize,
}
impl TransposeWorkspace for FaultingTransposeWorkspace {
    fn allocate(&mut self, length: usize) -> Result<Vec<f64>, ()> {
        self.allocations += 1;
        if self.allocation_failure == Some(self.allocations) {
            return Err(());
        }
        HeapTransposeWorkspace.allocate(length)
    }
    fn release(&mut self, buffer: Vec<f64>) -> Result<(), ()> {
        self.releases += 1;
        drop(buffer);
        if self.release_failure == Some(self.releases) {
            return Err(());
        }
        Ok(())
    }
}
fn rectangular_shape() -> TransposeShape {
    TransposeShape {
        input_stride: 2,
        rows: 2,
        columns: 3,
        output_stride: 3,
    }
}
#[test]
fn allocation_failure_preserves_output_and_sizes_and_skips_release() {
    let mut workspace = FaultingTransposeWorkspace {
        allocation_failure: Some(1),
        ..Default::default()
    };
    let (mut table, mut sizes, mut code) = ([1., 2., 3., 4., 5., 6.], [-1, -2], 9);
    transpose_in_place(
        rectangular_shape(),
        &mut table,
        &mut sizes,
        &mut code,
        &mut workspace,
    );
    assert_eq!(
        (table, sizes, code),
        ([1., 2., 3., 4., 5., 6.], [-1, -2], 2)
    );
    assert_eq!((workspace.allocations, workspace.releases), (1, 0));
}
#[test]
fn release_failure_follows_output_and_size_publication() {
    let mut workspace = FaultingTransposeWorkspace {
        release_failure: Some(1),
        ..Default::default()
    };
    let (mut table, mut sizes, mut code) = ([1., 2., 3., 4., 5., 6.], [-1, -2], 9);
    transpose_in_place(
        rectangular_shape(),
        &mut table,
        &mut sizes,
        &mut code,
        &mut workspace,
    );
    assert_eq!((table, sizes, code), ([1., 3., 5., 2., 4., 6.], [3, 2], 2));
    assert_eq!((workspace.allocations, workspace.releases), (1, 1));
}
#[test]
fn invalid_dimensions_clear_status_without_allocation_or_publication() {
    let mut workspace = FaultingTransposeWorkspace::default();
    let mut table = [7.; 6];
    let mut sizes = [-1, -2];
    let mut code = 9;
    let shape = TransposeShape {
        rows: 3,
        ..rectangular_shape()
    };
    transpose_in_place(shape, &mut table, &mut sizes, &mut code, &mut workspace);
    assert_eq!((table, sizes, code), ([7.; 6], [-1, -2], 1));
    assert_eq!((workspace.allocations, workspace.releases), (0, 0));
}
#[test]
fn separate_storage_has_the_same_failure_publication_order() {
    for allocation in [true, false] {
        let mut workspace = FaultingTransposeWorkspace {
            allocation_failure: allocation.then_some(1),
            release_failure: (!allocation).then_some(1),
            ..Default::default()
        };
        let input = [1., 2., 3., 4., 5., 6.];
        let mut output = [7.; 6];
        let mut slots = SeparateTransposeSlots {
            input: &input,
            output: &mut output,
        };
        let mut sizes = [-1, -2];
        let mut code = 9;
        transpose_slots(
            rectangular_shape(),
            &mut slots,
            &mut sizes,
            &mut code,
            &mut workspace,
        );
        assert_eq!(code, 2);
        assert_eq!(
            output,
            if allocation {
                [7.; 6]
            } else {
                [1., 3., 5., 2., 4., 6.]
            }
        );
        assert_eq!(sizes, if allocation { [-1, -2] } else { [3, 2] });
    }
}
#[test]
fn successful_heap_workspace_initializes_padding_and_releases_normally() {
    let allocated = HeapTransposeWorkspace.allocate(4).unwrap();
    assert_eq!(allocated, [0.; 4]);
    assert!(HeapTransposeWorkspace.release(allocated).is_ok());
}
