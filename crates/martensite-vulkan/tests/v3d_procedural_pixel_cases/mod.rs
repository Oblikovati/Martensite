//! Procedural objects exercise their production presentations and actual framebuffer queries.
mod erased_affinity;
pub(crate) use erased_affinity::erased_two_view_affinity;
mod array_cases;
mod arrays;
pub(crate) use array_cases::{four_point_pixels, patch_vertex_colors};
