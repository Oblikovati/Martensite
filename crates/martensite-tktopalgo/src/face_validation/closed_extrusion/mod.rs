//! Exact sufficient checks on complete closed-quadratic extrusion strips.
mod boundary;
mod boundary_report;
mod budget;
mod carrier;
mod decode;
mod incidence;
mod metrics;
mod open_support_inputs;
mod operation;
pub use open_support_inputs::{
    OpenSupportInputFailure, OpenSupportInputReport, OpenSupportInputRole, PlacedOpenSupportInputs,
    capture_placed_open_support_inputs,
};
mod placements;
mod report;
mod support_proof;
mod types;
pub use boundary_report::ClosedFaceBoundaryReport;
pub use operation::certify_closed_extrusion_face;
pub use report::{ClosedFaceCertificate, ClosedFaceReport};
pub use types::*;
