//! Behavior four preserves raw representation and resolves only declared incidence neighborhoods.
mod operation;
mod report;
pub(super) mod support;
mod types;
pub use operation::certify_tolerance_resolved_extrusion_face;
pub use report::ToleranceResolvedFaceCertificate;
pub use types::*;
