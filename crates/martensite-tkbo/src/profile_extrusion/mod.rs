// SPDX-License-Identifier: GPL-2.0-only
//! General planar regions and extrusion: the public seam a feature evaluator uses
//! to turn a solved sketch into validated faces and an extruded solid with complete history.
//!
//! - [`build_planar_regions`] converts solved lines, circular and elliptic arcs, circles,
//!   ellipses and clamped B-splines into every bounded face of the curve graph:
//!   nested, touching or sharing edges, each with its outer loop and holes and
//!   a complete curve to edge mapping.
//! - [`extrude`] sweeps one region along a vector into an [`OperationResult`]: the solid,
//!   its complete source to result history, findings, validation and achieved tolerance.
//!
//! [`build_planar_regions_with_work`] accepts a finite shared pair-work choice and
//! retains observed work on failure. Computational limits do not alter geometric accuracy.
//! Every operation is non-destructive and either succeeds completely or returns a typed
//! decline; no partial result exists.
mod admission;
pub(crate) use admission::admit_profile;
mod admit_curves;
mod admitted;
mod bernstein_partition;
mod bezier;
mod build_regions;
mod carriers;

mod decline;
mod ellipse;
mod extrude;
mod integral_evidence;
mod integral_failure;
mod loop_geometry;
mod loops;
mod operation_result;
mod plane_math;
mod profile;
mod proof;
mod proof_pieces;
pub use integral_evidence::PlanarIntegralEvidence;
pub use integral_failure::{PlanarIntegralFailure, PlanarIntegralPolicyFailure};
mod region_area;
mod regions;
mod resolution;
mod solid_validation;
mod spline;
mod sweep_history;
mod sweep_images;
mod topology;
mod validation;
mod work;

pub use crate::body_check::ValidationReport;
pub use crate::measured::{Evidence, IntegrationMethod, Measured, MeasuredPoint};
pub use crate::sketch_region_certification::CensusWorkReceipt as PlanarProofWorkReceipt;
pub use bernstein_partition::{NativeBernsteinSpan, exact_bernstein_partition};
pub use bezier::{
    ExactConicQuarter, admitted_conic_spans, exact_closed_conic, exact_conic_arc_source,
    exact_conic_quarter,
};
pub use build_regions::{
    build_planar_cardinal_quarter_regions_with_work_and_cancel,
    build_planar_line_regions_with_work_and_cancel, build_planar_regions,
    build_planar_regions_with_integration_budget_and_cancel,
    build_planar_regions_with_proof_and_cancel, build_planar_regions_with_work,
    build_planar_regions_with_work_and_cancel,
};
pub use decline::{
    DeclineCause, DeclineClass, OperationDecline, OperationKind, OperationStage,
    ValidationStageKind,
};
pub use extrude::{ExtrudeOptions, extrude};
pub use loops::{LoopUse, ProfileLoop};
pub use operation_result::*;
pub use profile::*;
pub use regions::*;
pub use resolution::*;
pub use validation::{Finding, RegionValidation, SelfIntersectionEvidence, StageStatus};
pub use work::{
    PlanarPairRootFailure, PlanarProofVersion, PlanarProofWorkLimit, PlanarRegionWorkFailure,
    PlanarRegionWorkResult,
};

#[cfg(test)]
mod area_translation_tests;

#[cfg(test)]
mod composite_tests;
