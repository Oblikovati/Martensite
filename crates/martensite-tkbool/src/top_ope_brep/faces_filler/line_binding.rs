//! Current-line views keep borrowed records and shared array slots aliased safely.
use super::*;
use crate::top_ope_brep_ds::TopOpeBRepDSTransition;
use martensite_tkernel::ncollection::{Array1, HArray1};
use std::{
    ops::{Deref, DerefMut},
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

pub(super) enum BoundLine<'a> {
    Borrowed(&'a mut TopOpeBRepLineInter),
    Shared {
        lines: Handle<HArray1<TopOpeBRepLineInter>>,
        /// Validated native array index of the retained slot.
        index: i32,
    },
}
/// A thread-confined borrowed or locked current-line view; `filler.p_line_inter_dummy()?.unwrap().index()`.
pub enum TopOpeBRepFacesFillerLine<'a> {
    /// An externally bound record whose borrow remains valid for the filler.
    Borrowed(&'a TopOpeBRepLineInter),
    /// A shared-array record; the read guard keeps its slot stable.
    Shared {
        /// Shared read lock retaining all line slots.
        array: RwLockReadGuard<'a, Array1<'static, TopOpeBRepLineInter>>,
        /// Validated native array index of the retained slot.
        index: i32,
    },
}
impl Deref for TopOpeBRepFacesFillerLine<'_> {
    type Target = TopOpeBRepLineInter;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(line) => line,
            Self::Shared { array, index } => {
                array.value(*index).expect("validated current-line index")
            }
        }
    }
}
/// A thread-confined mutable current-line view; `filler.change_line_inter_dummy()?.unwrap().set_index(7)`.
pub enum TopOpeBRepFacesFillerLineMut<'a> {
    /// Externally retained mutable record.
    Borrowed(&'a mut TopOpeBRepLineInter),
    /// Shared array locked for the lifetime of the mutable view.
    Shared {
        /// Shared write lock retaining all line slots.
        array: RwLockWriteGuard<'a, Array1<'static, TopOpeBRepLineInter>>,
        /// Validated native slot index.
        index: i32,
    },
}
impl Deref for TopOpeBRepFacesFillerLineMut<'_> {
    type Target = TopOpeBRepLineInter;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(line) => line,
            Self::Shared { array, index } => {
                array.value(*index).expect("validated current-line index")
            }
        }
    }
}
impl DerefMut for TopOpeBRepFacesFillerLineMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Borrowed(line) => line,
            Self::Shared { array, index } => array
                .change_value(*index)
                .expect("validated current-line index"),
        }
    }
}
/// A transition reference whose guard retains its current line; `filler.face_face_transition(1)?.before()`.
pub struct TopOpeBRepFacesFillerTransition<'a> {
    line: TopOpeBRepFacesFillerLine<'a>,
    side: i32,
}
impl Deref for TopOpeBRepFacesFillerTransition<'_> {
    type Target = TopOpeBRepDSTransition;
    fn deref(&self) -> &Self::Target {
        self.line
            .face_face_transition(self.side)
            .expect("validated operand side")
    }
}
impl BoundLine<'_> {
    pub(super) fn check(
        &mut self,
        filler: &TopOpeBRepFacesFiller<'_>,
        input: &mut TopOpeBRepLineInter,
    ) -> Result<bool, Failure> {
        match self {
            Self::Borrowed(line) => filler.check_line_with_retained(input, Some(line)),
            Self::Shared { .. } => Err(super::line_array::shared_line_requires_guard()),
        }
    }
    fn view(&self) -> Result<TopOpeBRepFacesFillerLine<'_>, Failure> {
        match self {
            Self::Borrowed(line) => Ok(TopOpeBRepFacesFillerLine::Borrowed(line)),
            Self::Shared { lines, index, .. } => {
                let array = lines.get().ok_or_else(missing_line)?.array1();
                array.value(*index).map_err(|error| {
                    Failure::with_message("Standard_OutOfRange", &error.to_string())
                })?;
                Ok(TopOpeBRepFacesFillerLine::Shared {
                    array,
                    index: *index,
                })
            }
        }
    }
}
impl<'a> TopOpeBRepFacesFiller<'a> {
    /// Retains an external current line without a raw pointer; `filler.bind_line(&mut line)?`.
    pub fn bind_line(&mut self, line: &'a mut TopOpeBRepLineInter) -> Result<(), Failure> {
        let result = self.load_line(line);
        self.bound_line = Some(BoundLine::Borrowed(line));
        result
    }
    /// Reads the aliased current record, or None before binding; `filler.p_line_inter_dummy()?`.
    pub fn p_line_inter_dummy(&self) -> Result<Option<TopOpeBRepFacesFillerLine<'_>>, Failure> {
        self.bound_line.as_ref().map(BoundLine::view).transpose()
    }
    /// Mutates the aliased current record, or None before binding; `filler.change_line_inter_dummy()?.unwrap().set_ok(false)`.
    pub fn change_line_inter_dummy(
        &mut self,
    ) -> Result<Option<TopOpeBRepFacesFillerLineMut<'_>>, Failure> {
        let Some(bound) = self.bound_line.as_mut() else {
            return Ok(None);
        };
        match bound {
            BoundLine::Borrowed(line) => Ok(Some(TopOpeBRepFacesFillerLineMut::Borrowed(line))),
            BoundLine::Shared { lines, index, .. } => {
                let array = lines.get().ok_or_else(missing_line)?.change_array1();
                array.value(*index).map_err(|error| {
                    Failure::with_message("Standard_OutOfRange", &error.to_string())
                })?;
                Ok(Some(TopOpeBRepFacesFillerLineMut::Shared {
                    array,
                    index: *index,
                }))
            }
        }
    }
    /// Reads the retained current line's transition; `filler.face_face_transition(2)?`.
    pub fn face_face_transition(
        &self,
        side: i32,
    ) -> Result<TopOpeBRepFacesFillerTransition<'_>, Failure> {
        let line = self.p_line_inter_dummy()?.ok_or_else(missing_line)?;
        line.face_face_transition(side)?;
        Ok(TopOpeBRepFacesFillerTransition { line, side })
    }
    /// Mutates the retained shared data structure, or None before storage binding; `filler.p_data_structure_dummy().unwrap().nb_points()`.
    pub fn p_data_structure_dummy(
        &self,
    ) -> Option<RwLockWriteGuard<'_, TopOpeBRepDSDataStructure>> {
        self.hds.get().map(TopOpeBRepDSHDataStructure::change_ds)
    }
    pub(super) fn bind_array_line(
        &mut self,
        lines: Handle<HArray1<TopOpeBRepLineInter>>,
        index: i32,
    ) {
        self.bound_line = Some(BoundLine::Shared { lines, index });
    }
}
fn missing_line() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "FacesFiller: current line=None; expected a bound current line",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn operation_builder_can_move_between_threads() {
        fn require_send<T: Send>() {}
        require_send::<TopOpeBRepFacesFiller<'static>>();
    }
}
