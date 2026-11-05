use super::BuilderResultArray;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

/// A read lease for one result cell; `let vertex = builder.new_vertex(1)?;`.
/// This guard is `!Send` because its standard lock must be released on its acquiring thread.
/// Drop it before writing through any array alias; conflicts return a failure, never wait.
#[derive(Debug)]
pub struct BuilderResultRead<'a, T: Send + Sync + 'static> {
    values: RwLockReadGuard<'a, Array1<'static, T>>,
    offset: usize,
}

/// An exclusive result-cell lease; `*builder.change_new_vertex(1)? = replacement;`.
/// This guard is `!Send`; drop it before any other access through an array alias.
#[derive(Debug)]
pub struct BuilderResultWrite<'a, T: Send + Sync + 'static> {
    values: RwLockWriteGuard<'a, Array1<'static, T>>,
    offset: usize,
}

pub(in crate::top_ope_brep_build::builder) fn read_cell<'a, T: Send + Sync + 'static>(
    array: &'a BuilderResultArray<T>,
    index: i32,
    operation: &str,
) -> Result<BuilderResultRead<'a, T>, Failure> {
    if array.is_null() {
        return Err(index_error(operation, index));
    }
    let values = array.read_array()?;
    let offset = cell_offset(&values, index, operation)?;
    Ok(BuilderResultRead { values, offset })
}

pub(in crate::top_ope_brep_build::builder) fn write_cell<'a, T: Send + Sync + 'static>(
    array: &'a BuilderResultArray<T>,
    index: i32,
    operation: &str,
) -> Result<BuilderResultWrite<'a, T>, Failure> {
    if array.is_null() {
        return Err(index_error(operation, index));
    }
    let values = array.write_array()?;
    let offset = cell_offset(&values, index, operation)?;
    Ok(BuilderResultWrite { values, offset })
}

fn cell_offset<T>(values: &Array1<'_, T>, index: i32, operation: &str) -> Result<usize, Failure> {
    values
        .value(index)
        .map_err(|_| index_error(operation, index))?;
    // Bounds cannot change while the returned cell retains this array lease.
    let offset = i64::from(index) - i64::from(values.lower());
    Ok(usize::try_from(offset).expect("validated cell offset fits the array extent"))
}

fn index_error(operation: &str, index: i32) -> Failure {
    Failure::with_type(
        "Standard_OutOfRange",
        &format!("{operation}: index {index} out of range"),
        "NCollection_Array1",
    )
}

impl<T: Send + Sync + 'static> Deref for BuilderResultRead<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.values.as_slice()[self.offset]
    }
}
impl<T: Send + Sync + 'static> Deref for BuilderResultWrite<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.values.as_slice()[self.offset]
    }
}
impl<T: Send + Sync + 'static> DerefMut for BuilderResultWrite<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.values.as_mut_slice()[self.offset]
    }
}
