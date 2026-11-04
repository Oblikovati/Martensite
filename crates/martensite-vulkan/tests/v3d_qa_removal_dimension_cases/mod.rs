//! Full QA command bodies preserve anonymous ownership, update order and shared aspects.
#[cfg(test)]
mod cpu;
mod deferred_removal;
mod primitive_removal;
mod shared_arrow_shapes;
mod shared_arrows;
pub(crate) use deferred_removal::deferred_selected_removal;
pub(crate) use primitive_removal::remove_four_primitives;
pub(crate) use shared_arrows::shared_dimension_arrows;
type Outcome = Result<(), Box<dyn std::error::Error>>;
