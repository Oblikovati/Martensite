// SPDX-License-Identifier: GPL-2.0-only
//! Checked single-span cylindrical or conical spatial helix and detached native materialization.
mod failure;
mod fit;
mod law;
mod native;

pub use failure::{HelixFailure, HelixNativeInvariant, HelixQuantity, HelixStage};
pub use law::{
    ConstantHelix, ConstantHelixRequest, HelixEvaluation, HelixLead, HelixRadius, HelixResolved,
    HelixRotation,
};
pub use martensite_tkmath::predicates::roots::metric_bounds::{HelixTraceBounds, HelixTraceLimits};
pub use native::{DetachedHelix, FittedHelix, HelixFitOptions, HelixFitReceipt, HelixSpanHistory};
