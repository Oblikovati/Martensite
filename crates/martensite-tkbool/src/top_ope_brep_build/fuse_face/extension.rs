//! Mutable extension access to the retained edge and vertex classification lists.
use super::TopOpeBRepBuildFuseFace;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;

/// Exclusive operation-local list access; `state.internal_edges_mut().clear()`.
///
/// This view borrows the existing lists. It does not clone their nodes, alter
/// fusion flags, or promise isolation of geometry shared by retained shapes.
pub struct FuseFaceExtensionState<'a> {
    record: &'a mut TopOpeBRepBuildFuseFace,
}

impl TopOpeBRepBuildFuseFace {
    /// Borrows the retained classification lists; `let state = fusion.extension_state();`.
    pub fn extension_state(&mut self) -> FuseFaceExtensionState<'_> {
        FuseFaceExtensionState { record: self }
    }
}

impl FuseFaceExtensionState<'_> {
    /// Edits the retained internal edges; `state.internal_edges_mut().clear()`.
    pub fn internal_edges_mut(&mut self) -> &mut List<Shape> {
        &mut self.record.my_lie
    }

    /// Edits the retained external edges; `state.external_edges_mut().clear()`.
    pub fn external_edges_mut(&mut self) -> &mut List<Shape> {
        &mut self.record.my_lee
    }

    /// Edits the retained modified edges; `state.modified_edges_mut().clear()`.
    pub fn modified_edges_mut(&mut self) -> &mut List<Shape> {
        &mut self.record.my_lme
    }

    /// Edits the retained internal vertices; `state.internal_vertices_mut().clear()`.
    pub fn internal_vertices_mut(&mut self) -> &mut List<Shape> {
        &mut self.record.my_liv
    }

    /// Edits the retained external vertices; `state.external_vertices_mut().clear()`.
    pub fn external_vertices_mut(&mut self) -> &mut List<Shape> {
        &mut self.record.my_lev
    }

    /// Edits the retained modified vertices; `state.modified_vertices_mut().clear()`.
    pub fn modified_vertices_mut(&mut self) -> &mut List<Shape> {
        &mut self.record.my_lmv
    }
}
