//! Independent container input, publication and typed failure contracts.
#[path = "binary_container_replay/fixture.rs"]
mod baseline;
#[path = "checked_binary_container/failures.rs"]
mod failures;
#[path = "borrowed_input/fakes.rs"]
mod fakes;
#[path = "checked_binary_container/fixture.rs"]
mod fixture;
#[path = "checked_binary_container/process_contract.rs"]
mod process_contract;
#[path = "checked_binary_container/success.rs"]
mod success;
#[path = "checked_binary_container/support.rs"]
mod support;
#[path = "checked_binary_container/position_range.rs"]
mod position_range;
