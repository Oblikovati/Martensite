//! Indexed planar mesh connectivity with deterministic deletion and compaction order.
use super::{
    BRepMeshEdge, BRepMeshTriangle, BRepMeshVertex, BRepMeshVertexTool, DegreeOfFreedom,
    PairOfIndex,
};
use crate::brep_mesh_data::MeshAllocator;
use martensite_tkernel::{
    impl_transient,
    ncollection::{
        BaseAllocator, DataMap, DefaultHashPolicy, DynamicArray, HashKey, IndexedDataMap, List,
        PackedMap,
    },
    standard::{Failure, Handle, SharedState, StandardTransient},
};

use std::sync::atomic::{AtomicU8, Ordering};

mod compact;
mod dump;
mod elements;
mod links;

// Removing a link changes movability and equality; the endpoint hash remains stable.
// Connectivity's outer SharedState synchronizes all structural changes; AtomicU8 allows
// lock-free in-place movability updates without a per-link RwLock.
#[derive(Debug)]
struct MeshLinkKey {
    first_node: i32,
    last_node: i32,
    movability: AtomicU8,
}
impl MeshLinkKey {
    #[inline]
    fn new(edge: BRepMeshEdge) -> Self {
        Self {
            first_node: edge.first_node(),
            last_node: edge.last_node(),
            movability: AtomicU8::new(edge.movability() as u8),
        }
    }
    #[inline]
    fn movability(&self) -> DegreeOfFreedom {
        match self.movability.load(Ordering::Relaxed) {
            0 => DegreeOfFreedom::Free,
            1 => DegreeOfFreedom::InVolume,
            2 => DegreeOfFreedom::OnSurface,
            3 => DegreeOfFreedom::OnCurve,
            4 => DegreeOfFreedom::Fixed,
            5 => DegreeOfFreedom::Frontier,
            _ => DegreeOfFreedom::Deleted,
        }
    }
    #[inline]
    fn set_movability(&self, movability: DegreeOfFreedom) {
        self.movability.store(movability as u8, Ordering::Relaxed);
    }
    #[inline]
    fn to_edge(&self) -> BRepMeshEdge {
        BRepMeshEdge::new(self.first_node, self.last_node, self.movability())
    }
}
impl PartialEq for MeshLinkKey {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.movability.load(Ordering::Relaxed) == DegreeOfFreedom::Deleted as u8
            || other.movability.load(Ordering::Relaxed) == DegreeOfFreedom::Deleted as u8
        {
            return false;
        }
        (self.first_node == other.first_node && self.last_node == other.last_node)
            || (self.first_node == other.last_node && self.last_node == other.first_node)
    }
}
impl HashKey for MeshLinkKey {
    #[inline]
    fn kernel_hash(&self) -> usize {
        BRepMeshEdge::new(self.first_node, self.last_node, DegreeOfFreedom::Free).kernel_hash()
    }
}
struct DelaunConnectivity {
    node_links: DataMap<i32, List<i32>>,
    links: IndexedDataMap<MeshLinkKey, PairOfIndex>,
    deleted_links: List<i32>,
    elements: DynamicArray<BRepMeshTriangle>,
    // IMeshData::MapOfInteger is TColStd_PackedMapOfInteger: its block/bucket
    // iteration order drives triangle and link enumeration downstream.
    element_domain: PackedMap<i32>,
    link_domain: PackedMap<i32>,
}
/// Shared mesh topology with native one-based indices; `BRepMeshDataStructureOfDelaun::new(&allocator,100)`.
pub struct BRepMeshDataStructureOfDelaun {
    allocator: MeshAllocator,
    nodes: Handle<BRepMeshVertexTool>,
    connectivity: SharedState<DelaunConnectivity>,
}
impl_transient!(
    BRepMeshDataStructureOfDelaun,
    "BRepMesh_DataStructureOfDelaun",
    StandardTransient
);
impl BRepMeshDataStructureOfDelaun {
    /// Reserve native maps and element blocks; `BRepMeshDataStructureOfDelaun::new(&allocator,100)`.
    pub fn new(allocator: &MeshAllocator, reserved_nodes: i32) -> Self {
        Self {
            allocator: allocator.clone(),
            nodes: Handle::new(BRepMeshVertexTool::new(allocator)),
            connectivity: SharedState::new(DelaunConnectivity::new(allocator, reserved_nodes)),
        }
    }
    /// Return allocated node slots, including deletion marks; `mesh.nb_nodes()`.
    pub fn nb_nodes(&self) -> i32 {
        self.nodes.extent()
    }
    /// Retain the allocator identity; `mesh.allocator()`.
    pub fn allocator(&self) -> &MeshAllocator {
        &self.allocator
    }
    /// Retain the live spatial node tool; `mesh.vertex_tool().set_tolerance(1e-7)`.
    pub fn vertex_tool(&self) -> &Handle<BRepMeshVertexTool> {
        &self.nodes
    }
    /// Find a coincident vertex using the spatial inspector; `mesh.index_of_node(&vertex)?`.
    pub fn index_of_node(&self, vertex: &BRepMeshVertex) -> Result<i32, Failure> {
        self.nodes.find_index(vertex)
    }
    /// Read a one-based node slot; `mesh.get_node(1)?`.
    #[inline]
    pub fn get_node(&self, index: i32) -> Result<BRepMeshVertex, Failure> {
        self.nodes.find_key(index)
    }
    /// Read three one-based node slots under a single lock acquisition.
    #[inline]
    pub(crate) fn get_nodes_3(&self, indices: [i32; 3]) -> Result<[BRepMeshVertex; 3], Failure> {
        self.nodes.find_keys_3(indices)
    }
    /// Read the movabilities of three one-based links under a single lock acquisition.
    #[inline]
    pub(crate) fn link_movabilities_3(
        &self,
        edges: [i32; 3],
    ) -> Result<[DegreeOfFreedom; 3], Failure> {
        let state = self.connectivity.read();
        Ok([
            state.links.find_key(edges[0] as usize)?.movability(),
            state.links.find_key(edges[1] as usize)?.movability(),
            state.links.find_key(edges[2] as usize)?.movability(),
        ])
    }
    /// Retain the node's original collection slot; `mesh.get_node_reference(1)?`.
    pub fn get_node_reference(&self, index: i32) -> Result<super::VertexSlotRead, Failure> {
        self.nodes.find_key_reference(index)
    }
    /// Add a node, initializing connectivity only on its first use; `mesh.add_node(&vertex,false)?`.
    pub fn add_node(&self, vertex: &BRepMeshVertex, force: bool) -> Result<i32, Failure> {
        let index = self.nodes.add(vertex, force)?;
        let mut state = self.connectivity.write();
        if !state.node_links.is_bound(&index) {
            state
                .node_links
                .bind(index, List::with_allocator(resource(&self.allocator)));
        }
        Ok(index)
    }
    /// Reject any already indexed replacement, including the current slot; `mesh.substitute_node(1,&vertex)?`.
    pub fn substitute_node(&self, index: i32, vertex: &BRepMeshVertex) -> Result<bool, Failure> {
        if self.nodes.find_index(vertex)? != 0 {
            return Ok(false);
        }
        self.nodes.substitute(index, vertex)?;
        Ok(true)
    }
    /// Mark an unattached free or forced node deleted; `mesh.remove_node(1,false)?`.
    pub fn remove_node(&self, index: i32, force: bool) -> Result<(), Failure> {
        if (force || self.get_node(index)?.movability() == DegreeOfFreedom::Free)
            && self.connectivity.read().node_links.find(&index)?.is_empty()
        {
            self.nodes.delete_vertex(index)?;
        }
        Ok(())
    }
    /// Copy attached links in insertion order; `mesh.links_connected_to(1)?`.
    pub fn links_connected_to(&self, index: i32) -> Result<List<i32>, Failure> {
        Ok(self.connectivity.read().node_links.find(&index)?.clone())
    }
}
impl DelaunConnectivity {
    fn new(allocator: &MeshAllocator, reserved: i32) -> Self {
        let arena = resource(allocator);
        let buckets = reserved.wrapping_mul(3).max(1) as usize;
        Self {
            node_links: DataMap::with_allocator(DefaultHashPolicy, buckets, arena.clone()),
            links: IndexedDataMap::with_allocator(DefaultHashPolicy, buckets, arena.clone()),
            deleted_links: List::with_allocator(arena.clone()),
            elements: DynamicArray::with_allocator(
                reserved.wrapping_mul(2).max(1) as usize,
                Some(arena),
            ),
            element_domain: PackedMap::default(),
            link_domain: PackedMap::default(),
        }
    }
}
fn resource(allocator: &MeshAllocator) -> BaseAllocator {
    allocator
        .get()
        .map(|_| BaseAllocator::from_handle(allocator))
        .unwrap_or_else(BaseAllocator::common)
}
