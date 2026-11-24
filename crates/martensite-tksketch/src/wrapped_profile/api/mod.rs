//! Value-only publication boundary for the rational normal-extrusion operation.
use super::*;
mod cone;
pub(super) mod error;
pub use cone::*;
mod export_bindings;
pub(super) mod failure_projection;
pub(super) mod history_projection;
mod history_types;
pub(super) mod input;
mod material;
pub(super) mod placement;
pub(super) mod reports;
mod request;
mod result;
pub use super::operation::{normal_extrude_profile, normal_extrude_profile_with_cancel};
pub use error::{WrappedFailureClass, WrappedNormalFailure, WrappedOperation, WrappedStage};
pub use history_types::*;
pub use material::*;
pub use placement::{
    WrappedChartNormalResult, WrappedChartPlacementReport, WrappedPlacementReport,
    WrappedWorldSpatialReport, normal_extrude_profile_on_chart,
    normal_extrude_profile_on_chart_with_cancel,
};
pub use reports::*;
pub use request::*;
pub use result::*;
type Checked<T> = Result<T, WrappedNormalFailure>;
#[cfg(test)]
mod history_tests;
#[cfg(test)]
pub(super) mod operation_tests;

#[cfg(test)]
mod error_tests;

#[cfg(test)]
mod planar_cancellation_tests;

#[cfg(test)]
mod triangle_clearance_tests;

#[cfg(test)]
mod flux_origin_tests;
