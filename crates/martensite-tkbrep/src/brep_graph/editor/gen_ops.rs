//! `EditorView::GenOps`: the generic operations. The removals are in `gen_remove`,
//! `gen_subgraph` and `gen_cleanup`.
use super::guards::mutate_ref;
use super::helpers::require_settable;
use crate::brep_graph::layer_history::{HistoryKind, LayerHistory};
use crate::brep_graph::mut_guard::MutGuard;
use crate::brep_graph::*;
use crate::brep_graph_inc::ParityOrientation;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::toploc::Location;

/// `EditorView::GenOps`.
#[derive(Debug)]
pub struct GenOps<'g> {
    pub(super) graph: &'g mut BRepGraph,
}

impl<'g> GenOps<'g> {
    /// `MutChildRef(theChildRef)`: the scoped guard of a child reference (generic over
    /// the child kind, so it lives on `Gen()`).
    pub fn mutate_child_ref(
        self,
        reference: ChildRefId,
    ) -> Result<MutGuard<'g, ChildRefTag>, Failure> {
        mutate_ref(self.graph, reference)
    }

    /// `SetChildRefLocalLocation(theChildRef, theLoc)`: a locked reference raises
    /// Standard_ProgramError; a reference out of range panics (undefined natively).
    pub fn set_child_ref_local_location(
        &mut self,
        reference: ChildRefId,
        location: &Location,
    ) -> Result<(), Failure> {
        require_settable(self.graph, reference)?;
        self.graph
            .inc_storage_mut()
            .change_child_ref(reference)
            .local_location = location.clone();
        self.graph.mark_ref_modified(reference.into());
        Ok(())
    }

    /// `SetChildRefLocalLocation(theMut, theLoc)`.
    pub fn set_child_ref_local_location_guarded(
        guard: &mut MutGuard<'_, ChildRefTag>,
        location: &Location,
    ) {
        guard.internal().local_location = location.clone();
    }

    /// `SetChildRefOrientation(theChildRef, theOrientation)`.
    pub fn set_child_ref_orientation(
        &mut self,
        reference: ChildRefId,
        orientation: ParityOrientation,
    ) -> Result<(), Failure> {
        require_settable(self.graph, reference)?;
        self.graph
            .inc_storage_mut()
            .change_child_ref(reference)
            .orientation = orientation;
        self.graph.mark_ref_modified(reference.into());
        Ok(())
    }

    /// `SetChildRefOrientation(theMut, theOrientation)`.
    pub fn set_child_ref_orientation_guarded(
        guard: &mut MutGuard<'_, ChildRefTag>,
        orientation: ParityOrientation,
    ) {
        guard.internal().orientation = orientation;
    }

    /// `SetChildRefChildNodeId(theChildRef, theChild)`: retargets the reference and
    /// rebinds the compound-parent relations of both children.
    pub fn set_child_ref_child_node_id(
        &mut self,
        reference: ChildRefId,
        child: NodeId,
    ) -> Result<(), Failure> {
        require_settable(self.graph, reference)?;
        let storage = self.graph.inc_storage_mut();
        let old = storage.child_ref(reference).child_node_id;
        if old == child {
            return Ok(());
        }
        storage.change_child_ref(reference).child_node_id = child;
        storage.rebind_child_ref(reference, old, child);
        self.graph.mark_ref_modified(reference.into());
        Ok(())
    }

    /// `SetChildRefChildNodeId(theMut, theChild)`.
    pub fn set_child_ref_child_node_id_guarded(
        guard: &mut MutGuard<'_, ChildRefTag>,
        child: NodeId,
    ) {
        let old = guard.child_node_id;
        if old == child {
            return;
        }
        guard.internal().child_node_id = child;
        let reference = guard.id();
        guard
            .graph_mut()
            .inc_storage_mut()
            .rebind_child_ref(reference, old, child);
    }

    /// `ApplyModification(theTarget, theModifier, theOpLabel)`: runs the modifier on the
    /// graph, records one Modified history event from the target to the replacements
    /// it returns (in the history layer, registered on demand), then invalidates the
    /// target's subgraph.
    ///
    /// ```
    /// use martensite_tkbrep::brep_graph::{BRepGraph, NodeId};
    /// use martensite_tkernel::tcollection::AsciiString;
    /// let mut graph = BRepGraph::new();
    /// let shell = graph.editor().shells().add();
    /// let label = AsciiString::from("noop");
    /// graph.editor().generic()
    ///     .apply_modification(shell.into(), |_, target| Ok(vec![target]), &label)
    ///     .unwrap();
    /// ```
    pub fn apply_modification<R>(
        &mut self,
        target: NodeId,
        modifier: impl FnOnce(&mut BRepGraph, NodeId) -> Result<R, Failure>,
        label: &AsciiString,
    ) -> Result<(), Failure>
    where
        R: IntoIterator<Item = NodeId>,
    {
        // A modifier that fails (throws natively) leaves no history record.
        let replacements: Vec<NodeId> = modifier(self.graph, target)?.into_iter().collect();
        self.apply_modification_impl(target, &replacements, label)
    }

    /// `applyModificationImpl(theTarget, theReplacements, theOpLabel)` (private
    /// natively). A null history handle (another layer class under its GUID, a null
    /// dereference natively) raises Standard_NullObject.
    fn apply_modification_impl(
        &mut self,
        target: NodeId,
        replacements: &[NodeId],
        label: &AsciiString,
    ) -> Result<(), Failure> {
        let history = self.graph.layer_registry().ensure::<LayerHistory>()?;
        let Some(history) = history.get() else {
            return Err(Failure::with_type(
                "Standard_NullObject",
                &format!("ApplyModification: no LayerHistory under its GUID for target {target:?}"),
                "",
            ));
        };
        history.record(
            self.graph,
            label,
            target,
            replacements,
            HistoryKind::Modified,
        );
        self.graph.invalidate_subgraph_impl(target);
        Ok(())
    }
}
