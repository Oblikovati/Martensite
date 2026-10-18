//! One transpose scratch zone serves both separate and in-place callers.
#[derive(Clone, Copy)]
pub(crate) struct TransposeShape {
    pub(crate) input_stride: i32,
    pub(crate) rows: i32,
    pub(crate) columns: i32,
    pub(crate) output_stride: i32,
}
pub(crate) use crate::adv_app2var::sys_base::numeric_workspace::{
    HeapNumericWorkspace as HeapTransposeWorkspace, NumericWorkspace as TransposeWorkspace,
};
trait TransposeSlots {
    fn input(&self, index: usize) -> f64;
    fn copy_output(&mut self, values: &[f64]);
}
struct SeparateTransposeSlots<'a> {
    input: &'a [f64],
    output: &'a mut [f64],
}
impl TransposeSlots for SeparateTransposeSlots<'_> {
    fn input(&self, index: usize) -> f64 {
        self.input[index]
    }
    fn copy_output(&mut self, values: &[f64]) {
        self.output[..values.len()].copy_from_slice(values);
    }
}
struct InPlaceTransposeSlots<'a>(&'a mut [f64]);
impl TransposeSlots for InPlaceTransposeSlots<'_> {
    fn input(&self, index: usize) -> f64 {
        self.0[index]
    }
    fn copy_output(&mut self, values: &[f64]) {
        self.0[..values.len()].copy_from_slice(values);
    }
}
pub(crate) fn transpose_separate(
    shape: TransposeShape,
    input: &[f64],
    output: &mut [f64],
    sizes: &mut [i32; 2],
    code: &mut i32,
) {
    let mut slots = SeparateTransposeSlots { input, output };
    transpose_slots(shape, &mut slots, sizes, code, &mut HeapTransposeWorkspace);
}
pub(crate) fn transpose_in_place(
    shape: TransposeShape,
    table: &mut [f64],
    sizes: &mut [i32; 2],
    code: &mut i32,
    workspace: &mut dyn TransposeWorkspace,
) {
    transpose_slots(
        shape,
        &mut InPlaceTransposeSlots(table),
        sizes,
        code,
        workspace,
    );
}
fn transpose_slots(
    shape: TransposeShape,
    slots: &mut dyn TransposeSlots,
    sizes: &mut [i32; 2],
    code: &mut i32,
    workspace: &mut dyn TransposeWorkspace,
) {
    *code = 0;
    if shape.rows > shape.input_stride || shape.columns > shape.output_stride {
        *code = 1;
        return;
    }
    let Ok(mut buffer) = workspace.allocate((shape.output_stride * shape.rows).max(0) as usize)
    else {
        *code = 2;
        return;
    };
    fill_transposed(shape, slots, &mut buffer);
    slots.copy_output(&buffer);
    *sizes = [shape.columns, shape.rows];
    if workspace.release(buffer).is_err() {
        *code = 2;
    }
}
fn fill_transposed(shape: TransposeShape, slots: &dyn TransposeSlots, buffer: &mut [f64]) {
    for row in 0..shape.rows {
        for column in 0..shape.columns {
            buffer[(row * shape.output_stride + column) as usize] =
                slots.input((row + column * shape.input_stride) as usize);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn separate_and_in_place_share_the_same_rectangular_transpose() {
        let shape = TransposeShape {
            input_stride: 2,
            rows: 2,
            columns: 3,
            output_stride: 3,
        };
        let mut separate = [7.; 6];
        let mut sizes = [-1, -2];
        let mut code = 5;
        transpose_separate(
            shape,
            &[1., 2., 3., 4., 5., 6.],
            &mut separate,
            &mut sizes,
            &mut code,
        );
        assert_eq!(
            (separate, sizes, code),
            ([1., 3., 5., 2., 4., 6.], [3, 2], 0)
        );
        let mut inplace = [1., 2., 3., 4., 5., 6.];
        transpose_in_place(
            shape,
            &mut inplace,
            &mut sizes,
            &mut code,
            &mut HeapTransposeWorkspace,
        );
        assert_eq!(inplace, separate);
    }
}

#[cfg(test)]
mod fallible_tests {
    use super::*;
    #[test]
    fn impossible_workspace_capacity_returns_failure_without_panicking() {
        let result = std::panic::catch_unwind(|| HeapTransposeWorkspace.allocate(usize::MAX));
        assert!(
            result.is_ok(),
            "expected fallible reserve, not capacity panic"
        );
        assert!(result.unwrap().is_err());
    }
}

#[cfg(test)]
#[path = "transpose_workspace_tests.rs"]
pub(crate) mod fault_tests;
