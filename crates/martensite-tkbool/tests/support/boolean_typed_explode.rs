//! Compatibility import of the one shared topology traversal implementation.
#[path = "typed_exploration.rs"]
mod implementation;
#[allow(unused_imports)] // Each existing source host selects one or both traversal operations.
pub use implementation::{all_distinct, first_distinct};
