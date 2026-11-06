//! Shared full numerical programs are independent of historical image gates.
#[allow(dead_code)]
#[path = "../modalg_display_release_cases/argument_lists.rs"]
pub(super) mod argument_lists;
#[path = "../modalg_display_cases/result_view.rs"]
mod result_view;
pub(crate) use argument_lists::{ArgumentListOperation, argument_list_view};
#[cfg(test)]
mod cpu;
