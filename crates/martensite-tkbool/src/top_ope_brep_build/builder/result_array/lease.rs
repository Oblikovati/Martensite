use super::BuilderResultArray;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

impl<T: Send + Sync + 'static> BuilderResultArray<T> {
    fn allocated(&self, operation: &str) -> Result<&RwLock<Array1<'static, T>>, Failure> {
        self.storage.as_deref().ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                &format!(
                    "BuilderResultArray::{operation}: null handle; expected an allocated array"
                ),
            )
        })
    }

    /// Tries a shared lease; `let values = results.read_array()?;`.
    /// A live writer returns a typed failure immediately. The standard guard is
    /// thread-confined (`!Send`); release it before requesting a write through any alias.
    pub fn read_array(&self) -> Result<RwLockReadGuard<'_, Array1<'static, T>>, Failure> {
        match self.allocated("read_array")?.try_read() {
            Ok(guard) => Ok(guard),
            Err(TryLockError::Poisoned(error)) => Ok(error.into_inner()),
            Err(TryLockError::WouldBlock) => Err(conflict("read_array", "writer")),
        }
    }

    /// Tries an exclusive lease; `results.write_array()?.set_value(1, value)?;`.
    /// Any existing lease returns a failure immediately instead of blocking an alias.
    /// The standard guard is thread-confined (`!Send`); drop it before another access.
    pub fn write_array(&self) -> Result<RwLockWriteGuard<'_, Array1<'static, T>>, Failure> {
        match self.allocated("write_array")?.try_write() {
            Ok(guard) => Ok(guard),
            Err(TryLockError::Poisoned(error)) => Ok(error.into_inner()),
            Err(TryLockError::WouldBlock) => Err(conflict("write_array", "reader or writer")),
        }
    }
}

fn conflict(operation: &str, active: &str) -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        &format!(
            "BuilderResultArray::{operation}: active {active}; expected no conflicting array lease"
        ),
    )
}
