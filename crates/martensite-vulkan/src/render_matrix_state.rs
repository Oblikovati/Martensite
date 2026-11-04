//! One transformation's matrix stack (`GlMatrixState`: the current matrix and
//! the copies saved by `push`, restored last-in first-out by `pop`.
use crate::{RenderError, error::rejected};
use martensite_tkernel::ncollection::{ElementCast, Mat4};

/// The current matrix (identity at first) and its saved copies;
/// `RenderMatrixState::<f32>::default().current().is_identity()`.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderMatrixState<T> {
    current: Mat4<T>,
    stack: Vec<Mat4<T>>,
    /// Native `myStack.Length()`: the slots ever filled; `pop` keeps them.
    slots: usize,
}

impl<T: Copy + From<u8>> Default for RenderMatrixState<T> {
    fn default() -> Self {
        Self {
            current: Mat4::identity(),
            stack: Vec::new(),
            slots: 0,
        }
    }
}

impl<T: Copy + From<u8>> RenderMatrixState<T> {
    /// Save a copy of the current matrix; `state.push()`.
    pub fn push(&mut self) {
        self.stack.push(self.current);
        self.slots = self.slots.max(self.stack.len());
    }

    /// Restore the most recently saved matrix; an empty stack is rejected and leaves the
    /// current matrix unchanged (native asserts and returns); `state.pop()?`.
    pub fn pop(&mut self) -> Result<(), RenderError> {
        let saved = self
            .stack
            .pop()
            .ok_or_else(|| rejected("an empty matrix stack", "a matrix saved by push"))?;
        self.current = saved;
        Ok(())
    }

    /// The current matrix; `state.current()`.
    pub fn current(&self) -> &Mat4<T> {
        &self.current
    }

    /// The current matrix for in-place edits (`ChangeCurrent`); `state.change_current()`.
    pub fn change_current(&mut self) -> &mut Mat4<T> {
        &mut self.current
    }

    /// Replace the current matrix; `state.set_current(&matrix)`.
    pub fn set_current(&mut self, matrix: &Mat4<T>) {
        self.current = *matrix;
    }

    /// Replace the current matrix by an element-wise conversion of another scalar type (the
    /// templated `SetCurrent`); `state.set_current_converted(&double_matrix)`.
    pub fn set_current_converted<U: Copy + ElementCast<T>>(&mut self, matrix: &Mat4<U>) {
        self.current = Mat4::from_other(matrix);
    }

    /// Reset the current matrix to the identity, keeping the saved ones; `state.set_identity()`.
    pub fn set_identity(&mut self) {
        self.current = Mat4::identity();
    }

    /// The number of saved matrices; `state.depth()`.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}

impl<T: Copy + From<u8> + Into<f64>> RenderMatrixState<T> {
    /// `DumpJson`: the current matrix row by row, the filled stack slots and the stack head
    /// (-1 when empty); `state.dump_json(&mut stream, -1)?`.
    pub fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        _depth: i32,
    ) -> std::io::Result<()> {
        let values: Vec<f64> = (0..4)
            .flat_map(|row| (0..4).map(move |column| (row, column)))
            .map(|(row, column)| self.current.get_value(row, column).into())
            .collect();
        crate::driver_dump::field_real_values(stream, "myCurrent", &values)?;
        martensite_tkservice::json_dump::field_integer(
            stream,
            "myStack.Length()",
            self.slots as i32,
        )?;
        martensite_tkservice::json_dump::field_integer(
            stream,
            "myStackHead",
            self.stack.len() as i32 - 1,
        )
    }
}

#[cfg(test)]
#[path = "render_matrix_state_tests.rs"]
mod tests;
