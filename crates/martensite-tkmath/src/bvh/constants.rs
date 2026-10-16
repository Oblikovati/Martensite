//! BVH constants: tree depth, leaf sizes and bin counts.

/// `MAX_TREE_DEPTH`: optimal depth, in sync with traversal stacks.
pub const MAX_TREE_DEPTH: i32 = 32;
/// `LEAF_NODE_SIZE_SINGLE`: for upper-level trees of complex nodes.
pub const LEAF_NODE_SIZE_SINGLE: i32 = 1;
/// `LEAF_NODE_SIZE_AVERAGE`: 4 primitives per leaf.
pub const LEAF_NODE_SIZE_AVERAGE: i32 = 4;
/// `LEAF_NODE_SIZE_DEFAULT`: 5 primitives per leaf.
pub const LEAF_NODE_SIZE_DEFAULT: i32 = 5;
/// `LEAF_NODE_SIZE_SMALL`: 8 primitives per leaf (e.g. triangles).
pub const LEAF_NODE_SIZE_SMALL: i32 = 8;
/// `NB_BINS_OPTIMAL`: bin count of the binned builder.
pub const NB_BINS_OPTIMAL: usize = 32;
/// `NB_BINS_BEST`: maximum useful bin count of the binned builder.
pub const NB_BINS_BEST: usize = 48;
/// `bvh::THE_NODE_MIN_SIZE`: minimum node size to split (a `double`).
pub const THE_NODE_MIN_SIZE: f64 = 1e-5;
