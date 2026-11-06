//! `InterfaceHGraph`: an `InterfaceGraph` shared through a handle.
use super::{
    InterfaceGTool, InterfaceGeneralLib, InterfaceGraph, InterfaceInterfaceModel, InterfaceProtocol,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient},
};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

/// `InterfaceHGraph`; the graph is lent through its lock (`Graph`, `CGraph`).
///
/// ```
/// use martensite_tkxsbase::interface::InterfaceHGraph;
/// fn size(hgraph: &InterfaceHGraph) -> i32 { hgraph.graph().size() }
/// ```
pub struct InterfaceHGraph {
    transient: StandardTransient,
    thegraph: SharedState<InterfaceGraph>,
}
impl_transient!(
    InterfaceHGraph,
    "InterfaceHGraph",
    StandardTransient,
    transient
);

impl InterfaceHGraph {
    /// `InterfaceHGraph(agraph)`: a copy of the graph.
    pub fn from_graph(agraph: &InterfaceGraph) -> Self {
        Self::wrap(agraph.clone())
    }
    /// `InterfaceHGraph(amodel, lib, theModeStats = true)`.
    pub fn with_lib(
        amodel: &Handle<dyn InterfaceInterfaceModel>,
        lib: &InterfaceGeneralLib,
        mode_stats: bool,
    ) -> Result<Self, Failure> {
        Ok(Self::wrap(InterfaceGraph::with_lib(
            amodel, lib, mode_stats,
        )?))
    }
    /// `InterfaceHGraph(amodel, protocol, theModeStats = true)`.
    pub fn with_protocol(
        amodel: &Handle<dyn InterfaceInterfaceModel>,
        protocol: &Handle<dyn InterfaceProtocol>,
        mode_stats: bool,
    ) -> Result<Self, Failure> {
        Ok(Self::wrap(InterfaceGraph::with_protocol(
            amodel, protocol, mode_stats,
        )?))
    }
    /// `InterfaceHGraph(amodel, gtool, theModeStats = true)`.
    pub fn with_gtool(
        amodel: &Handle<dyn InterfaceInterfaceModel>,
        gtool: &Handle<InterfaceGTool>,
        mode_stats: bool,
    ) -> Result<Self, Failure> {
        Ok(Self::wrap(InterfaceGraph::with_gtool(
            amodel, gtool, mode_stats,
        )?))
    }
    /// `InterfaceHGraph(amodel, theModeStats = true)`.
    pub fn new(
        amodel: &Handle<dyn InterfaceInterfaceModel>,
        mode_stats: bool,
    ) -> Result<Self, Failure> {
        Ok(Self::wrap(InterfaceGraph::new(amodel, mode_stats)?))
    }
    fn wrap(graph: InterfaceGraph) -> Self {
        Self {
            transient: StandardTransient::default(),
            thegraph: SharedState::new(graph),
        }
    }
    /// `Graph`: the graph, read only (a guard to drop before editing it).
    pub fn graph(&self) -> RwLockReadGuard<'_, InterfaceGraph> {
        self.thegraph.read()
    }
    /// `CGraph`: the graph, modifiable.
    pub fn c_graph(&self) -> RwLockWriteGuard<'_, InterfaceGraph> {
        self.thegraph.write()
    }
}
