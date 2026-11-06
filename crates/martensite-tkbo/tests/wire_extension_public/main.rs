//! Ordinary normal-library consumers of operation-bound wire extension behavior.
mod bop_algo {
    pub use martensite_tkbo::bop_algo::{WireEdgeSet, WireSplitter};
}
// Reuse only the authored input/identity factory, which contains no registered tests.
#[path = "../../src/bop_algo/wire_splitter/progress_fixture.rs"]
mod fixture;
mod helper_contracts;
mod hook_contracts;
mod receiver_contracts;
mod scope_contracts;
mod support;
