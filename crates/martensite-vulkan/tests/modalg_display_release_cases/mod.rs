//! Modelling regressions of the bugs group whose operations only finish in the native
//! release build's time: checked, then shown in a 3D view.
mod argument_lists;
#[path = "../modalg_display_cases/result_view.rs"]
mod result_view;
pub(crate) use argument_lists::{ArgumentListOperation, argument_list_view};
