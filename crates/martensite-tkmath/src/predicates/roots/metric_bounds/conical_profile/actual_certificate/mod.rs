// SPDX-License-Identifier: GPL-2.0-only
//! Complete numeric coverage for the explicitly selected directed conical proxy.
use super::boundary_chart::ChartBinding;
use super::boundary_snapshot::BoundarySnapshot;
use super::*;
use std::collections::{BTreeMap, BTreeSet};
mod query;
pub use query::*;
mod error;
pub use error::*;
mod gamma;
mod numeric;
pub use gamma::ConicalDirectedGammaEvidence;
pub(in crate::predicates::roots::metric_bounds::conical_profile) use gamma::GammaSpanCapture;
pub use gamma::certify_conical_directed_gamma;
mod snapshot;
pub use snapshot::{
    ConicalActualCapSnapshot, ConicalActualSideSnapshot, ConicalActualUvContourSnapshot,
    ConicalActualUvEdgeSnapshot,
};
mod admission;
mod barriers;
mod binding;
mod caps;
mod capture;
mod evidence;
mod hierarchy;
mod unions;
pub use evidence::{ConicalActualPatchCertificate, ConicalActualPatchCoverage};
mod execute;
pub use execute::certify_conical_actual_patches;
#[cfg(test)]
mod tests;
