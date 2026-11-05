//! Nullable shared result arrays retain bounds and reject conflicting live leases.
mod cell;
mod lease;
pub use cell::{BuilderResultRead, BuilderResultWrite};
pub(super) use cell::{read_cell, write_cell};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use std::sync::{Arc, RwLock};

/// An operation-local shared bounded array; cloning shares its storage, not a copy.
///
/// `let results = BuilderResultArray::<i32>::with_bounds(-2, 3)?;`
/// Shared geometry reachable from elements retains its existing ownership rules.
/// This mutable construction state is not an immutable published operation result.
#[derive(Debug)]
pub struct BuilderResultArray<T: Send + Sync + 'static> {
    storage: Option<Arc<RwLock<Array1<'static, T>>>>,
}

impl<T: Send + Sync + 'static> Default for BuilderResultArray<T> {
    fn default() -> Self {
        Self { storage: None }
    }
}

impl<T: Send + Sync + 'static> Clone for BuilderResultArray<T> {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
        }
    }
}

impl<T: Send + Sync + 'static> BuilderResultArray<T> {
    /// Wraps owned bounds/storage without element copies; `Self::from_array(Array1::default())`.
    pub fn from_array(values: Array1<'static, T>) -> Self {
        Self {
            storage: Some(Arc::new(RwLock::new(values))),
        }
    }

    /// Distinguishes a null handle from an allocated empty array; `Self::default().is_null()`.
    pub fn is_null(&self) -> bool {
        self.storage.is_none()
    }

    /// Releases this owner's handle without changing aliases; `results.clear()`.
    pub fn clear(&mut self) {
        self.storage = None;
    }

    /// Compares shared array identity, including null handles; `first.shares_storage_with(&alias)`.
    pub fn shares_storage_with(&self, other: &Self) -> bool {
        match (&self.storage, &other.storage) {
            (Some(first), Some(second)) => Arc::ptr_eq(first, second),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: Default + Send + Sync + 'static> BuilderResultArray<T> {
    /// Allocates the inclusive bounds, preserving negative indices; `Self::with_bounds(-3, -1)?`.
    pub fn with_bounds(lower: i32, upper: i32) -> Result<Self, Failure> {
        Array1::with_bounds(lower, upper).map(Self::from_array).map_err(|error| {
            Failure::with_message("Standard_RangeError", &format!(
                "BuilderResultArray bounds [{lower}, {upper}]: {error}; expected a valid inclusive range"
            ))
        })
    }
}
