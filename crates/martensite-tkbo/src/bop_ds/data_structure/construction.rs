//! The DS preserves a nullable policy; maps normalize it, segmented arrays do not.
use super::{BaseAllocator, DataMap, DataStructure, DynamicArray, Map};
use martensite_tkernel::ncollection::DefaultHashPolicy;

impl Default for DataStructure {
    fn default() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }
}

impl DataStructure {
    /// Creates empty storage with the supplied nullable allocation policy.
    /// `let ds = DataStructure::with_allocator(None); assert!(ds.allocator().is_none());`
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        // BOPDSDS passes signed zero increments and its raw policy to arrays.
        let normalized = allocator.clone().unwrap_or_else(BaseAllocator::common);
        Self {
            allocator: allocator.clone(),
            arguments: Default::default(),
            ranges: Default::default(),
            lines: Default::default(),
            shape_indices: Default::default(),
            vertex_edges: DataMap::with_allocator(DefaultHashPolicy, 100, normalized.clone()),
            pave_blocks_pool: Default::default(),
            pave_block_common_blocks: Default::default(),
            face_info_pool: Default::default(),
            interf_vv: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_ve: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_vf: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_ee: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_ef: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_ff: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_vz: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_ez: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_fz: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interf_zz: DynamicArray::with_allocator_i32(0, allocator.clone()),
            interference_table: Map::with_allocator(DefaultHashPolicy, 100, normalized.clone()),
            interfered_shapes: Map::with_allocator(DefaultHashPolicy, 100, normalized.clone()),
            same_domain_vertices: DataMap::with_allocator(
                DefaultHashPolicy,
                100,
                normalized.clone(),
            ),
            source_shape_count: Default::default(),
        }
    }

    /// Borrows the original nullable policy, independently of map normalization.
    /// `assert!(DataStructure::with_allocator(None).allocator().is_none());`
    pub fn allocator(&self) -> Option<&BaseAllocator> {
        self.allocator.as_ref()
    }
}
