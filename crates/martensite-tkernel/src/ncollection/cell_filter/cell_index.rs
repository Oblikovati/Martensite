//! A cell index uses ten inline integers; lookup copies must not alias stored keys.
#![forbid(unsafe_code)]
use super::{CellGeometry, indices::cell_coordinate};
use crate::ncollection::Array1;
use allocator_api2::boxed::Box as AllocBox;
use std::fmt;
mod storage;
/// Ten inline integers avoid allocation for common dimensions.
/// Inline storage has no heap owner: Rust moves must not retain a self pointer.
#[repr(C)]
pub(super) struct CellIndex {
    inline: [i32; 10],
    // The thin owner preserves the compact key layout; the vector owns retained empty storage.
    heap: Option<AllocBox<Vec<i32>>>,
    length: usize,
}
impl CellIndex {
    pub(super) fn from_point<I: CellGeometry>(
        point: &I::CellPoint,
        sizes: &Array1<'_, f64>,
    ) -> Self {
        let mut index = Self::allocate(sizes.len());
        for (axis, size) in sizes.as_slice().iter().enumerate() {
            index.as_mut_slice()[axis] = cell_coordinate(I::cell_coordinate(axis, point) / size);
        }
        index
    }
    pub(super) fn from_slice(source: &[i32]) -> Self {
        let mut index = Self::allocate(source.len());
        index.as_mut_slice().copy_from_slice(source);
        index
    }
    pub(super) fn as_mut_slice(&mut self) -> &mut [i32] {
        match &mut self.heap {
            Some(heap) => &mut heap[..self.length],
            None => &mut self.inline[..self.length],
        }
    }
    pub(super) fn as_slice(&self) -> &[i32] {
        match &self.heap {
            Some(heap) => &heap[..self.length],
            None => &self.inline[..self.length],
        }
    }
    pub(super) fn data_ptr(&self) -> *const i32 {
        self.as_slice().as_ptr()
    }
}
impl fmt::Debug for CellIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(formatter)
    }
}
impl Clone for CellIndex {
    fn clone(&self) -> Self {
        // A cell copies coordinates into its own ten-inline-element storage.
        Self::from_slice(self.as_slice())
    }
}
#[cfg(test)]
mod migration_tests;
#[cfg(test)]
mod native_tests;
