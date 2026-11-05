//! Derived split caches are separate from the base builder's regularization maps.
use super::TopOpeBRepBuildBuilder1;
use crate::top_ope_brep_build::builder::DataMapOfShapeListOfShape;
use martensite_tkernel::standard::Failure;

/// A shared view of a derived builder's extension caches.
/// `builder.split_extension_state()?.face_splits().is_empty()` reads the face cache.
pub struct Builder1SplitState<'a> {
    face_splits: &'a DataMapOfShapeListOfShape,
    edge_splits: &'a DataMapOfShapeListOfShape,
}

/// An exclusive operation-local view of the derived extension caches.
/// `builder.split_extension_state_mut()?.edge_splits_mut().clear(true)` clears one cache.
pub struct Builder1SplitStateMut<'a> {
    face_splits: &'a mut DataMapOfShapeListOfShape,
    edge_splits: &'a mut DataMapOfShapeListOfShape,
}

impl TopOpeBRepBuildBuilder1 {
    /// Borrows derived caches, not inherited caches; `let state = builder.split_extension_state()?;`.
    pub fn split_extension_state(&self) -> Result<Builder1SplitState<'_>, Failure> {
        let derived = self.b1d()?;
        Ok(Builder1SplitState {
            face_splits: &derived.my_f_splits,
            edge_splits: &derived.my_e_splits,
        })
    }

    /// Exclusively borrows derived caches; `let state = builder.split_extension_state_mut()?;`.
    pub fn split_extension_state_mut(&mut self) -> Result<Builder1SplitStateMut<'_>, Failure> {
        let derived = self.b1d_mut()?;
        Ok(Builder1SplitStateMut {
            face_splits: &mut derived.my_f_splits,
            edge_splits: &mut derived.my_e_splits,
        })
    }
}

impl Builder1SplitState<'_> {
    /// Reads the retained derived face cache; `assert!(state.face_splits().is_empty())`.
    pub fn face_splits(&self) -> &DataMapOfShapeListOfShape {
        self.face_splits
    }

    /// Reads the retained derived edge cache; `assert!(state.edge_splits().is_empty())`.
    pub fn edge_splits(&self) -> &DataMapOfShapeListOfShape {
        self.edge_splits
    }
}

impl Builder1SplitStateMut<'_> {
    /// Edits only the derived face cache; `state.face_splits_mut().clear(true)`.
    pub fn face_splits_mut(&mut self) -> &mut DataMapOfShapeListOfShape {
        self.face_splits
    }

    /// Edits only the derived edge cache; `state.edge_splits_mut().clear(true)`.
    pub fn edge_splits_mut(&mut self) -> &mut DataMapOfShapeListOfShape {
        self.edge_splits
    }
}
