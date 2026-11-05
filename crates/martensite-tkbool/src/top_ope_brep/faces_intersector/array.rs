//! The mutable record guard preserves shared-array edits without escaping a lock.
use super::TopOpeBRepFacesIntersector;
use crate::top_ope_brep::TopOpeBRepLineInter;
use martensite_tkernel::{
    ncollection::{Array1, HArray1},
    standard::{Failure, Handle},
};
use std::{
    ops::{Deref, DerefMut},
    sync::RwLockWriteGuard,
};

/// This guard is thread-confined while its write lock is held.
/// A mutable record in the shared line array; `let mut line = pair.current_line()?;`.
pub struct TopOpeBRepFacesLineMut<'a> {
    guard: RwLockWriteGuard<'a, Array1<'static, TopOpeBRepLineInter>>,
    index: i32,
}
impl Deref for TopOpeBRepFacesLineMut<'_> {
    type Target = TopOpeBRepLineInter;
    fn deref(&self) -> &Self::Target {
        self.guard.value(self.index).expect("validated line index")
    }
}
impl DerefMut for TopOpeBRepFacesLineMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard
            .change_value(self.index)
            .expect("validated line index")
    }
}
impl TopOpeBRepFacesIntersector {
    /// Shares the prepared indexed array; `let shared=pair.lines();`.
    pub fn lines(&self) -> Handle<HArray1<TopOpeBRepLineInter>> {
        self.lines.clone()
    }

    /// Borrows the current line under the shared-array write lock; `pair.current_line()?`.
    pub fn current_line(&self) -> Result<TopOpeBRepFacesLineMut<'_>, Failure> {
        self.change_line(self.line_index)
    }

    /// Borrows a line at the native array index; `pair.change_line(1)?`.
    pub fn change_line(&self, index: i32) -> Result<TopOpeBRepFacesLineMut<'_>, Failure> {
        let array = self.lines.get().ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                "TopOpeBRepFacesIntersector::ChangeLine: lines=null; expected prepared array",
            )
        })?;
        let guard = array.change_array1();
        guard.value(index).map_err(|error|Failure::with_message("Standard_OutOfRange",&format!("TopOpeBRepFacesIntersector::ChangeLine: index={index}; expected {}..={}: {error}",guard.lower(),guard.upper())))?;
        Ok(TopOpeBRepFacesLineMut { guard, index })
    }
}
