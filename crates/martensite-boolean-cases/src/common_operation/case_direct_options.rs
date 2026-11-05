//! Shared original direct-Boolean configuration; execution stays caller-owned.
use martensite_tkbo::bop_algo::{BOPAlgoBOP, Glue, Operation};

/// Apply original direct command options before caller-owned execution; `direct_options(&mut operation, Operation::Fuse, precision::confusion())`.
pub fn direct_options(operation: &mut BOPAlgoBOP, kind: Operation, fuzzy: f64) {
    operation.set_operation(kind);
    operation.set_glue(Glue::Off);
    operation.options_mut().set_fuzzy_value(fuzzy);
    operation.set_non_destructive(false);
    operation.options_mut().set_run_parallel(false);
    operation.options_mut().set_use_obb(false);
    operation.set_check_inverted(true);
    operation.set_to_fill_history(true);
}
