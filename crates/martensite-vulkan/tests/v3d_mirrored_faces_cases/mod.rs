#[allow(dead_code)]
#[path = "../modalg_display_cases/dataset.rs"]
pub(super) mod dataset;
#[path = "../modalg_display_cases/result_view.rs"]
mod result_view;
use result_view::display_result_view;
mod prelude {
    pub(super) use crate::dataset_shape::read_dataset_shape;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use crate::shape_checks::{MassCheck, MassKind, check_mass};
    pub(super) use crate::shape_report_checks::check_shape_structure;
    pub(super) use martensite_tkbrep::topods::Shape;
    pub(super) use std::error::Error;
    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}
#[cfg(test)]
mod cpu;
