//! Display only an operation result whose ordered checks, history and measurements completed.
mod blind_bores;
#[cfg(test)]
mod boolean_ancestry_controls;
#[cfg(test)]
mod boolean_controls;
mod boolean_coverage;
mod boolean_history;
mod boolean_incidence;
mod boolean_measurements;
mod boolean_outcome;
#[cfg(test)]
mod boolean_receipt_controls;
mod boolean_validation;

use crate::common_operation as bop_options;
use crate::source_bop_alerts as bop_alerts;
pub(crate) use blind_bores::five_blind_bores;

mod prelude {
    pub(super) use super::boolean_outcome::{BooleanRequest, execute_boolean};
    pub(super) use crate::case_view::viewer_session::DisplayRequest;
    pub(super) use crate::session_run::{SessionRun, banner};
    pub(super) use martensite_tkbo::bop_algo::Operation;
    pub(super) use martensite_tkbrep::topods::Shape;
    pub(super) use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
    pub(super) use std::error::Error;
    pub(super) type Outcome = Result<(), Box<dyn Error>>;
}
