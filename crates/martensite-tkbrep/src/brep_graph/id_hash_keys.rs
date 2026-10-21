//! The native hashes of the BRepGraph ids as TKernel map keys.
//!
//! `NCollection_DataMap<BRepGraph_NodeId, ...>` and `NCollection_FlatMap<...>` hash
//! their keys with `NCollection_DefaultHasher`, which is `std::hash` of the id. The
//! layer maps whose iteration order reaches a result use the foundation maps, so
//! the ids hash as natively. CopyRemap keys its
//! item map with the `ItemId` key here.
use crate::brep_graph::{ItemId, ItemUid, KindTag, NodeId, TypedId, Uid};
use martensite_tkernel::ncollection::HashKey;

impl HashKey for NodeId {
    fn kernel_hash(&self) -> usize {
        self.hash_code()
    }
}

// `std::hash<BRepGraph_NodeId::Typed<K>>` (the typed ids of `NCollection_FlatMap<
// BRepGraph_FaceId>` in the mesh cache's dirty collection).
impl<T: KindTag> HashKey for TypedId<T> {
    fn kernel_hash(&self) -> usize {
        self.hash_code()
    }
}

impl HashKey for ItemId {
    fn kernel_hash(&self) -> usize {
        self.hash_code()
    }
}

impl HashKey for Uid {
    fn kernel_hash(&self) -> usize {
        self.hash_value()
    }
}

impl HashKey for ItemUid {
    fn kernel_hash(&self) -> usize {
        self.hash_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep_graph::NodeKind;

    #[test]
    fn keys_hash_with_the_native_id_hashes() {
        let node = NodeId::new(NodeKind::FACE, 3);
        assert_eq!(node.kernel_hash(), node.hash_code());
        let item = ItemId::from(node);
        assert_eq!(item.kernel_hash(), item.hash_code());
        let uid = Uid::new(NodeKind::EDGE, 7);
        assert_eq!(uid.kernel_hash(), uid.hash_value());
        let item_uid = ItemUid::node(NodeKind::EDGE, 7);
        assert_eq!(item_uid.kernel_hash(), item_uid.hash_value());
    }
}
