//! Independent contracts for checked input through its public interface.
#[path = "borrowed_input/fakes.rs"]
mod fakes;
#[path = "borrowed_input/mask_cases.rs"]
mod mask_cases;
#[path = "borrowed_input/operation_cases.rs"]
mod operation_cases;
#[path = "borrowed_input/primitive_cases.rs"]
mod primitive_cases;
use martensite_tkernel::storage::checked_input;

#[path = "borrowed_input/additional_cases.rs"]
mod additional_cases;

#[path = "borrowed_input/state_matrix.rs"]
mod state_matrix;

#[path = "borrowed_input/interface_cases.rs"]
mod interface_cases;
