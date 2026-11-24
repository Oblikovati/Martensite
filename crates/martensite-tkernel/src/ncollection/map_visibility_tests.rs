// Map and tree contracts run with access to TKernel-private methods.
#[path = "../../tests/base_map_indexed_comparisons.rs"]
mod base_map_indexed_comparisons;

#[path = "../../tests/base_map_indexed_iterator_contracts.rs"]
mod base_map_indexed_iterator_contracts;

#[path = "../../tests/base_map_indexed_proxy_comparisons.rs"]
mod base_map_indexed_proxy_comparisons;

#[path = "../../tests/base_map_indexed_proxy_results.rs"]
mod base_map_indexed_proxy_results;

#[path = "../../tests/data_map_method_contracts.rs"]
mod data_map_method_contracts;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/data_map_storage_contracts.rs"]
mod data_map_storage_contracts;

#[path = "../../tests/data_map_supplemental.rs"]
mod data_map_supplemental;

#[allow(clippy::module_inception)]
#[path = "../../tests/double_map_canonical_contracts.rs"]
mod double_map_canonical_contracts;

#[path = "../../tests/double_map_method_contracts.rs"]
mod double_map_method_contracts;

#[path = "../../tests/double_map_supplemental.rs"]
mod double_map_supplemental;

#[path = "../../tests/flat_data_map_method_contracts.rs"]
mod flat_data_map_method_contracts;

#[path = "../../tests/flat_map_method_contracts.rs"]
mod flat_map_method_contracts;

#[path = "../../tests/flat_map_supplemental.rs"]
mod flat_map_supplemental;

#[path = "../../tests/indexed_data_map_borrowed_add.rs"]
mod indexed_data_map_borrowed_add;

#[path = "../../tests/indexed_data_map_canonical.rs"]
mod indexed_data_map_canonical;

#[path = "../../tests/indexed_data_map_method_contracts.rs"]
mod indexed_data_map_method_contracts;

#[path = "../../tests/indexed_data_supplemental.rs"]
mod indexed_data_supplemental;

#[path = "../../tests/indexed_map_borrowed_add.rs"]
mod indexed_map_borrowed_add;

#[path = "../../tests/indexed_map_canonical.rs"]
mod indexed_map_canonical;

#[path = "../../tests/indexed_map_method_contracts.rs"]
mod indexed_map_method_contracts;

#[path = "../../tests/indexed_map_supplemental.rs"]
mod indexed_map_supplemental;

#[allow(clippy::duplicate_mod)]
#[path = "../../tests/map_canonical_storage_contracts.rs"]
mod map_canonical_storage_contracts;

#[path = "../../tests/map_method_contracts.rs"]
mod map_method_contracts;

#[path = "../../tests/map_supplemental.rs"]
mod map_supplemental;

#[path = "../../tests/ncollection_data_map.rs"]
mod ncollection_data_map;

#[path = "../../tests/ncollection_data_map_assignment.rs"]
mod ncollection_data_map_assignment;

#[path = "../../tests/ncollection_data_map_bind_ref.rs"]
mod ncollection_data_map_bind_ref;

#[path = "../../tests/ncollection_data_map_bind_ref_allocation.rs"]
mod ncollection_data_map_bind_ref_allocation;

#[path = "../../tests/ncollection_double_map.rs"]
mod ncollection_double_map;

#[path = "../../tests/ncollection_flat_copy_contracts.rs"]
mod ncollection_flat_copy_contracts;

#[path = "../../tests/ncollection_flat_data_map.rs"]
mod ncollection_flat_data_map;

#[path = "../../tests/ncollection_flat_map.rs"]
mod ncollection_flat_map;

#[path = "../../tests/ncollection_flat_maps_api_behavior.rs"]
mod ncollection_flat_maps_api_behavior;

#[path = "../../tests/ncollection_flat_move_contracts.rs"]
mod ncollection_flat_move_contracts;

#[path = "../../tests/ncollection_indexed_data_map.rs"]
mod ncollection_indexed_data_map;

#[path = "../../tests/ncollection_indexed_map.rs"]
mod ncollection_indexed_map;

#[path = "../../tests/ncollection_kdtree.rs"]
mod ncollection_kdtree;

#[path = "../../tests/ncollection_kdtree_supplemental.rs"]
mod ncollection_kdtree_supplemental;

#[path = "../../tests/ncollection_map.rs"]
mod ncollection_map;

#[path = "../../tests/ncollection_map_borrowed_add.rs"]
mod ncollection_map_borrowed_add;

#[path = "../../tests/ncollection_ordered_data_map.rs"]
mod ncollection_ordered_data_map;

#[path = "../../tests/ncollection_ordered_map.rs"]
mod ncollection_ordered_map;

#[path = "../../tests/ncollection_packed_map.rs"]
mod ncollection_packed_map;

#[path = "../../tests/ordered_data_map_method_contracts.rs"]
mod ordered_data_map_method_contracts;

#[path = "../../tests/ordered_map_destruction.rs"]
mod ordered_map_destruction;

#[path = "../../tests/ordered_map_generic_bounds.rs"]
mod ordered_map_generic_bounds;

#[path = "../../tests/ordered_map_method_contracts.rs"]
mod ordered_map_method_contracts;

#[path = "../../tests/ordered_map_supplemental.rs"]
mod ordered_map_supplemental;

#[path = "../../tests/packed_map_aliases.rs"]
mod packed_map_aliases;

#[path = "../../tests/packed_map_method_contracts.rs"]
mod packed_map_method_contracts;

#[path = "../../tests/packed_map_node_contracts.rs"]
mod packed_map_node_contracts;

#[path = "../../tests/packed_map_protected_helpers.rs"]
mod packed_map_protected_helpers;

#[path = "../../tests/packed_map_supplemental.rs"]
mod packed_map_supplemental;

#[path = "../../tests/ncollection_flat_hash_contracts.rs"]
mod ncollection_flat_hash_contracts;

#[path = "../../tests/ncollection_flat_key_construction.rs"]
mod ncollection_flat_key_construction;

#[path = "../../tests/ordered_policy_construction.rs"]
mod ordered_policy_construction;

#[path = "../../tests/ordered_policy_exchange.rs"]
mod ordered_policy_exchange;

#[path = "../../tests/ordered_base_bridge.rs"]
mod ordered_base_bridge;

#[path = "../../tests/ordered_borrowed_emplace.rs"]
mod ordered_borrowed_emplace;

#[path = "../../tests/ordered_canonical_storage.rs"]
mod ordered_canonical_storage;

#[path = "../../tests/ordered_emplace_construction.rs"]
mod ordered_emplace_construction;

#[path = "../../tests/packed_map_rows/mod.rs"]
mod packed_map_rows;

#[path = "../../tests/support/native_node_counts.rs"]
mod native_node_counts;

mod private_examples {
    use crate::ncollection::{DataMap, DoubleMap, IndexedDataMap, IndexedMap, Map};
    use crate::standard::Failure;

    #[test]
    fn data_map_self_assignment_keeps_binding() {
        let mut map = DataMap::<i32, i32>::default();
        map.bind(1, 7);
        assert_eq!(map.assign_self().find(&1), Ok(&7));
    }

    #[test]
    fn double_map_buckets_expose_primary_chain() -> Result<(), Failure> {
        let mut map = DoubleMap::<i32, i32>::default();
        assert!(map.buckets().is_none());
        map.bind(1, 5).unwrap();
        let table = map.buckets().unwrap();
        assert_eq!(table.len(), 102);
        assert_eq!(table.chain(2)?.collect::<Vec<_>>(), [(&1, &5)]);
        Ok(())
    }

    #[test]
    fn indexed_data_map_borrowed_add_preserves_existing_value() {
        let mut map = IndexedDataMap::<i32, i32>::default();
        assert_eq!(map.add_ref(&1, &42), 1);
        assert_eq!(map.add_ref(&1, &99), 1);
        assert_eq!(map.find_from_index(1), Ok(&42));
    }

    #[test]
    fn indexed_data_map_buckets_expose_key_and_value() -> Result<(), Failure> {
        let mut map = IndexedDataMap::<i32, i32>::default();
        map.add(7, 70);
        let table = map.buckets().unwrap();
        assert_eq!(table.chain(8)?.collect::<Vec<_>>(), [(&7, &70)]);
        Ok(())
    }

    #[test]
    fn indexed_map_buckets_expose_key_chain() -> Result<(), Failure> {
        let mut map = IndexedMap::<i32>::default();
        assert!(map.buckets().is_none());
        map.add(7);
        let table = map.buckets().unwrap();
        assert_eq!(
            table.chain(8)?.map(|(key, _)| *key).collect::<Vec<_>>(),
            [7]
        );
        Ok(())
    }

    #[test]
    fn indexed_map_borrowed_add_reuses_index() {
        let mut map = IndexedMap::<i32>::default();
        assert_eq!(map.add_ref(&42), 1);
        assert_eq!(map.add_ref(&42), 1);
    }

    #[test]
    fn map_borrowed_add_skips_duplicate() {
        let mut keys = Map::<i32>::default();
        assert!(keys.add_ref(&7));
        assert!(!keys.add_ref(&7));
    }

    #[test]
    fn map_borrowed_add_returns_stored_key() {
        let mut keys = Map::<i32>::default();
        assert_eq!(keys.added_ref(&7), &7);
        assert_eq!(keys.added_ref(&7), &7);
    }
}
