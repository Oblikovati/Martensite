//! Nonowning vertex-slot aliases preserve in-place changes without holding a collection lock.
use super::{BRepMeshVertex, MeshVertexCollection};
use martensite_tkernel::standard::{Failure, Handle, HandleObserver, WeakHandle};

/// Read a live vertex slot while its collection and initialized slot remain valid.
///
/// This operation-local alias does not change the collection's intrusive reference count.
/// `let retained = tool.find_key_reference(1)?; let current = retained.snapshot()?;`.
#[derive(Clone, Debug)]
pub struct VertexSlotRead {
    collection: WeakHandle<MeshVertexCollection>,
    index: i32,
}

impl VertexSlotRead {
    pub(super) fn new(
        collection: &Handle<MeshVertexCollection>,
        index: i32,
    ) -> Result<Self, Failure> {
        let reference = Self {
            collection: collection.downgrade(),
            index,
        };
        reference.snapshot()?;
        Ok(reference)
    }

    /// Observe the current value rather than the value at acquisition; `reference.snapshot()?`.
    pub fn snapshot(&self) -> Result<BRepMeshVertex, Failure> {
        let collection = self.lease()?;
        super::inspector::read_initialized_vertex(&collection, self.index)
    }

    /// Compare logical stored-slot identity; `assert!(first.same_slot(&second))`.
    pub fn same_slot(&self, other: &Self) -> bool {
        self.collection == other.collection && self.index == other.index
    }

    fn lease(&self) -> Result<HandleObserver<MeshVertexCollection>, Failure> {
        let lease = self.collection.observe();
        if lease.ref_count() == 0 {
            return Err(Failure::with_message(
                "Standard_NullObject",
                &format!(
                    "vertex slot {} has expired storage; expected a live collection owner",
                    self.index
                ),
            ));
        }
        Ok(lease)
    }
}

/// Mutate the stored vertex without reindexing spatial cells, as direct slot access does.
///
/// This is an operation-local alias; `inspector.get_vertex_reference(1)?.replace(vertex)?;`.
#[derive(Clone, Debug)]
pub struct VertexSlotWrite(VertexSlotRead);

impl VertexSlotWrite {
    pub(super) fn new(
        collection: &Handle<MeshVertexCollection>,
        index: i32,
    ) -> Result<Self, Failure> {
        VertexSlotRead::new(collection, index).map(Self)
    }

    /// Keep a const view of this same slot; `let observer = reference.read_only();`.
    pub fn read_only(&self) -> VertexSlotRead {
        self.0.clone()
    }

    /// Read the current vertex; `let current = reference.snapshot()?;`.
    pub fn snapshot(&self) -> Result<BRepMeshVertex, Failure> {
        self.0.snapshot()
    }

    /// Replace the vertex through its original slot; `reference.replace(vertex)?;`.
    pub fn replace(&self, vertex: BRepMeshVertex) -> Result<(), Failure> {
        let collection = self.0.lease()?;
        super::inspector::write_initialized_vertex(&collection, self.0.index, vertex)
    }
}
