// SPDX-License-Identifier: GPL-2.0-only
//! Exact supplied-representation geometry; no mapped tolerance algorithm is replaced.
mod arithmetic;
mod curve_identity;
pub use curve_identity::{CurveIdentityDecision, MetricCurveChain, compare_curve_chains};
mod affine_coordinate;
mod affine_support;
#[cfg(test)]
mod curve_identity_tests;
mod mapped;
pub use affine_coordinate::exact_affine_coordinate;
mod cylindrical_generators;
mod placed_plane;
pub mod planar_attachment;
pub use cylindrical_generators::{
    CylindricalGeneratorChart, CylindricalGeneratorError, CylindricalPullKind,
};
pub use placed_plane::{PlacedPlane, exact_placed_line_ends};
mod parameter;
mod prism_jacobian;
pub use prism_jacobian::affine_prism_jacobian;
#[cfg(test)]
mod prism_jacobian_tests;
pub use mapped::{
    TransportedResidualQuery, TransportedResidualReport, certify_transported_residual,
    certify_transported_residual_on_bounds,
};
pub use parameter::MetricParameterMap;
mod coefficients;
mod cylindrical_clearance;
pub use cylindrical_clearance::{
    CertifiedCylindricalClearance, CertifiedCylindricalImage,
    CertifiedCylindricalPeriodicClearance, CertifiedCylindricalPeriodicSlab,
    CylindricalClearanceDomain, CylindricalClearanceError, CylindricalClearanceFailure,
    CylindricalClearanceOptions, CylindricalClearanceQuery, CylindricalClearanceScope,
    CylindricalClearanceSource, CylindricalClearanceWorkError, CylindricalPeriodicTranslation,
    certify_cylindrical_clearance, certify_cylindrical_clearance_with_work,
};
pub use cylindrical_clearance::{
    CertifiedCylindricalProbe, CertifiedCylindricalProbes, CylindricalPeriodicProbe,
    CylindricalProbeError, CylindricalProbeFailure, CylindricalProbeOptions, CylindricalProbeSense,
    CylindricalProbeWorkError,
};
mod developed_period;
pub use developed_period::{
    DevelopedConeRay, DevelopedConeRayError, DevelopedPeriod, DevelopedPeriodError, DevelopedScale,
    certify_developed_cone_ray, certify_developed_period, enclose_developed_cone_angle,
};
mod cylindrical_measure;
pub use cylindrical_measure::{
    CertifiedCylindricalMeasureComparison, CylindricalMeasureComparisonError,
    CylindricalMeasureComparisonFailure, CylindricalMeasureComparisonQuery,
    compare_cylindrical_measures,
};
pub use cylindrical_measure::{
    CertifiedCylindricalNormalMeasure, CylindricalMeasureError, CylindricalMeasureEvidence,
    CylindricalMeasureFailure, CylindricalMeasureLimit, CylindricalMeasureOptions,
    CylindricalMeasureQuery, certify_cylindrical_normal_measure,
};
mod cylindrical_cone;
/// Original cylindrical profile source shared by local and collection certificates.
pub use cylindrical_cone::CylindricalConeSource as CylindricalProfileSource;
pub use cylindrical_cone::{
    CertifiedCylindricalCone, CylindricalConeBehavior, CylindricalConeError,
    CylindricalConeFailure, CylindricalConeOptions, CylindricalConeQuery, CylindricalConeSource,
    certify_cylindrical_cone,
};
mod cylindrical_depths;
pub use cylindrical_depths::{
    CertifiedCylindricalDepths, CylindricalDepthFailure, certify_cylindrical_depths,
};
mod conical_development;
mod conical_material;
pub use conical_development::{
    ConicalDevelopment, ConicalDevelopmentFailure, ConicalFootprint, ConicalTrimBand,
};
mod conical_profile;
mod cylindrical_profile;
mod cylindrical_storage;
pub use super::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;
pub use conical_material::{
    ConicalDevelopedMaterialError, ConicalDevelopedMaterialFailure, ConicalMaterialError,
    ConicalMaterialEvidence, ConicalMaterialQuery, ConicalMaterialSpan, certify_conical_material,
    certify_conical_material_with_developed_boundaries,
    certify_conical_material_with_developed_boundaries_and_refinement,
};
pub use conical_profile::{
    ConeClearanceOptions, ConeRequest, ConeRequestError, ConeRequestReceipt,
    ConicalAtlasReferenceDomain, ConicalAtlasReferenceQuery, ConicalAuthoredContour,
    ConicalAuthoredFlux, ConicalAuthoredFluxError, ConicalAuthoredFluxMethod,
    ConicalAuthoredFluxQuery, ConicalAuthoredFluxStage, ConicalBoundaryError,
    ConicalBoundaryEvidence, ConicalBoundaryFailure, ConicalBoundaryFailureKind,
    ConicalBoundaryProofFailure, ConicalBoundaryQuery, ConicalBoundaryStage,
    ConicalCapClearanceRelation, ConicalCapFamilyHomotopyError, ConicalCapHomotopyEvidence,
    ConicalCapHomotopyFailure, ConicalCapJoinSpan, ConicalCapJoinsQuery, ConicalCapNormalError,
    ConicalCapNormalEvidence, ConicalCapVolumeError, ConicalCapVolumeEvidence,
    ConicalCapVolumeFailure, ConicalChartDomainBound, ConicalChartDomainError,
    ConicalChartDomainEvidence, ConicalChartDomainFailure, ConicalChartDomainQuery,
    ConicalChartDomainRequirement, ConicalChartDomainVersion, ConicalChartPlan,
    ConicalChartPlanError, ConicalChartPlanFailure, ConicalChartPlanQuery,
    ConicalChartReferenceQuery, ConicalDecodedCapEvidence, ConicalDecodedFamilyError,
    ConicalDecodedFamilyFailure, ConicalDecodedFillFailure, ConicalDepthFailure,
    ConicalJoinFailureKind, ConicalJoinProofFailure, ConicalJoinSpan, ConicalJoinsEvidence,
    ConicalJoinsFailure, ConicalJoinsQuery, ConicalOriginalFailure, ConicalOriginalSpan,
    ConicalPrimitiveRefinement, ConicalReferenceError, ConicalReferenceEvidence,
    ConicalReferenceFailure, ConicalReferenceMember, ConicalReferenceQuery, ConicalSectorEvidence,
    ConicalSectorFailure, ConicalSectorQuery, ConicalSectorWedge, ConicalSourceFailure,
    ConicalStorageError, ConicalStorageEvidence, ConicalStorageFailure, ConicalStorageQuery,
    ConicalSubspanChartError, certify_cone_request, certify_conical_atlas_reference,
    certify_conical_authored_flux, certify_conical_authored_flux_with_method,
    certify_conical_boundary, certify_conical_boundary_in_chart,
    certify_conical_cap_family_homotopy, certify_conical_cap_joins, certify_conical_chart_domain,
    certify_conical_chart_domain_with_version, certify_conical_chart_reference,
    certify_conical_decoded_family, certify_conical_decoded_family_with_fill,
    certify_conical_joins, certify_conical_reference, certify_conical_sector,
    certify_conical_storage, certify_conical_subspan_chart, plan_conical_subspan_charts,
};
pub use conical_profile::{
    ConicalActualAdjacent, ConicalActualCapCarrier, ConicalActualCapId, ConicalActualCapQuery,
    ConicalActualCapSnapshot, ConicalActualContourRole, ConicalActualMemberId,
    ConicalActualPatchCertificate, ConicalActualPatchCoverage, ConicalActualPatchError,
    ConicalActualPatchFailure, ConicalActualPatchQuery, ConicalActualPatchStage,
    ConicalActualPeriodicCaps, ConicalActualPeriodicMember, ConicalActualSideId,
    ConicalActualSideQuery, ConicalActualSideSnapshot, ConicalActualUvContour,
    ConicalActualUvContourSnapshot, ConicalActualUvEdge, ConicalActualUvEdgeSnapshot,
    ConicalDirectedGammaEvidence, ConicalDirectedGammaQuery, ConicalDirectedGammaSpan,
    certify_conical_actual_patches, certify_conical_actual_patches_with_periodic_caps,
    certify_conical_directed_gamma,
};
pub use conical_profile::{
    ConicalBoundaryZRowError, ConicalBoundaryZRowEvidence, ConicalBoundaryZRowFailure,
    ConicalBoundaryZRowTransition, certify_conical_boundary_in_chart_with_refinement,
    certify_conical_boundary_z_rows, certify_conical_chart_reference_with_refinement,
    certify_conical_joins_in_chart_with_refinement,
    certify_conical_trim_pair_in_chart_with_refinement,
    certify_conical_trim_pair_with_developed_boundary_and_refinement,
};
pub use cylindrical_profile::{
    CertifiedConicalTrim, CertifiedConicalTrimPair, CertifiedCylindricalLift,
    CertifiedCylindricalProfile, CertifiedCylindricalTrim, ConicalTrimCarrier, ConicalTrimError,
    ConicalTrimPairError, ConicalTrimPairQuery, ConicalTrimQuery, CylindricalChartPlan,
    CylindricalFitChart, CylindricalFitPlan, CylindricalFitQuery, CylindricalLiftError,
    CylindricalProfileBehavior, CylindricalProfileError, CylindricalProfileFailure,
    CylindricalProfileInterval, CylindricalProfileOptions, CylindricalProfileQuery,
    CylindricalTrimError, CylindricalTrimQuery, CylindricalTrimWorkspace, certify_conical_trim,
    certify_conical_trim_pair, certify_conical_trim_ranges, certify_cylindrical_lift,
    certify_cylindrical_profile, certify_cylindrical_trim, certify_cylindrical_trim_ranges,
    plan_cylindrical_charts, plan_cylindrical_fit,
};
mod inequality;
mod proof;
mod types;
use super::{DyadicBound, error::poll};
pub use cylindrical_storage::{
    CertifiedCylindricalContactPrefix, CertifiedCylindricalRadialSweep, CertifiedCylindricalSlice,
    CertifiedCylindricalStorage, CylindricalContactPrefixError, CylindricalContactPrefixFailure,
    CylindricalContactPrefixQuery, CylindricalRadialSense, CylindricalRadialSweepError,
    CylindricalRadialSweepFailure, CylindricalRadialSweepQuery, CylindricalSliceQuery,
    CylindricalSliceRefusal, CylindricalStorageError, CylindricalStorageFailure,
    CylindricalStorageLimits, CylindricalStorageOptions, CylindricalStorageQuery,
    CylindricalStorageResidual, certify_cylindrical_contact_prefix,
    certify_cylindrical_radial_sweep, certify_cylindrical_slice_direction,
    certify_cylindrical_slice_with_joins, certify_cylindrical_storage,
};
use num_bigint::BigInt;
pub use types::*;

/// Certify the full declared same-parameter residual without mutating representations.
/// Example: `certify_planar_residual(query, || false)?` yields a proven squared bound.
/// The bound is not a sampled estimate or necessarily the tight maximum.
pub fn certify_planar_residual(
    query: PlanarResidualQuery<'_>,
    mut cancelled: impl FnMut() -> bool,
) -> Result<ResidualDecision, ResidualError> {
    certify_query(query, None, &mut cancelled)
}
/// Certify an exact subinterval inside the original declared representation domain.
/// Example: `certify_planar_residual_on_bounds(query, root_bounds, || false)?`.
/// Use original unsplit source parameters; this proves metric conformance, not contact identity.
pub fn certify_planar_residual_on_bounds(
    query: PlanarResidualQuery<'_>,
    bounds: [DyadicBound; 2],
    mut cancelled: impl FnMut() -> bool,
) -> Result<ResidualDecision, ResidualError> {
    certify_query(query, Some(bounds), &mut cancelled)
}
fn certify_query(
    query: PlanarResidualQuery<'_>,
    exact_bounds: Option<[DyadicBound; 2]>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<ResidualDecision, ResidualError> {
    poll(cancelled)?;
    validate_query(&query)?;
    let (declared, width) = super::checked_policy_interval(query.interval, query.options)?;
    let bounds = exact_bounds.unwrap_or_else(|| declared.clone());
    admit_subinterval(&bounds, &declared, query.interval)?;
    let (coefficients, zero, square) = inequality::threshold(&query, cancelled)?;
    let upper = DyadicBound::new(if zero { BigInt::from(0) } else { square }, -2148);
    let domain = proof::MetricProofDomain {
        bounds,
        width,
        options: query.options,
        upper,
    };
    proof::decide(coefficients, domain, cancelled)
}
fn admit_subinterval(
    bounds: &[DyadicBound; 2],
    declared: &[DyadicBound; 2],
    original: [f64; 2],
) -> Result<(), ResidualError> {
    if bounds[0] > bounds[1] || bounds[0] < declared[0] || bounds[1] > declared[1] {
        return Err(ResidualError::ExactDomain {
            supplied: bounds.clone(),
            declared: original,
        });
    }
    Ok(())
}
fn validate_query(query: &PlanarResidualQuery<'_>) -> Result<(), ResidualError> {
    let [first, last] = query.interval;
    let bezier = matches!(query.curve, MetricCurve::Bezier(_))
        || matches!(query.pcurve, MetricCurve::Bezier(_));
    if !first.is_finite()
        || !last.is_finite()
        || first > last
        || (bezier && (first < 0. || last > 1.))
    {
        return Err(ResidualError::Domain(query.interval));
    }
    if !query.spatial_limit.is_finite() || query.spatial_limit < 0. {
        return Err(ResidualError::Value {
            role: "spatial_limit",
            index: 0,
            value: query.spatial_limit,
        });
    }
    Ok(())
}
#[cfg(test)]
mod tests;

#[cfg(test)]
mod parameter_tests;

#[cfg(test)]
mod transported_contact_bounds_tests;

#[cfg(test)]
mod integral_baseline_tests;

mod polynomial_integrals;
pub use polynomial_integrals::{
    CertifiedPolynomialMeasure, CertifiedPolynomialPlaneMeasure, OrientedMetricCurve,
    PolynomialIntegralError, PolynomialIntegralEvidence, PolynomialIntegralFailure,
    PolynomialIntegralOperation, PolynomialIntegralOptions, PolynomialPlaneError,
    PolynomialPlaneEvidence, PolynomialPlaneFailure, certify_polynomial_boundary_integral,
    certify_polynomial_plane_measure, certify_polynomial_sweep_integral,
};

mod rational_integrals;
mod rational_reference;
pub use rational_integrals::{
    AnalyticConicalFluxError, AnalyticConicalFluxFailure, AnalyticConicalFluxQuery,
    AnalyticConicalFluxSpan, BoundaryIntegralProgramQuery, CertifiedAnalyticConicalFlux,
    CertifiedConicalReferenceVolume, CertifiedCylindricalFlux, CertifiedIntegralProgram,
    CertifiedRationalMeasure, ConicalReferenceVolumeError, ConicalReferenceVolumeFailure,
    ConicalReferenceVolumeQuery, ConicalReferenceVolumeSpan, ConicalReferenceVolumeTerm,
    CylindricalFluxError, CylindricalFluxFailure, CylindricalFluxOptions, CylindricalFluxQuery,
    CylindricalFluxSpan, CylindricalFluxWork, FaceIntegralProgramQuery, IntegralProgramError,
    IntegralProgramFailure, IntegralProgramLoop, IntegralProgramOptions, IntegralProgramVersion,
    RATIONAL_TAYLOR_ORDER, RationalIntegralError, RationalIntegralEvidence,
    RationalIntegralFailure, RationalIntegralOptions, certify_analytic_conical_boundary_flux,
    certify_boundary_integral_program, certify_conical_reference_volume,
    certify_cylindrical_boundary_flux, certify_face_integral_program,
    certify_rational_boundary_integral, certify_rational_face_area,
    certify_rational_sweep_integral,
};
pub use rational_reference::{
    AdmittedConicFrame, AdmittedConicPhase, AdmittedConicSpan, CertifiedAdmittedConicSpan,
    CertifiedNativeSplineSpan, CertifiedRationalConicSegment, ConicSpanParameter,
    NativeSplineFailure, NativeSplineSource, RationalConicSegment, RationalConicSegmentFailure,
    RationalReferenceBounds, RationalReferenceFailure, RationalReferenceProof,
    RationalReferenceRequest, RationalReferenceSpan, certify_admitted_conic_span,
    fit_rational_bezier_reference, fit_rational_bezier_reference_with_proof, lower_native_bspline,
    lower_rational_conic_segment, partition_homogeneous_bspline,
};

mod polynomial_side;
pub use polynomial_side::{
    CertifiedPolynomialSideIntegral, PolynomialSideError, PolynomialSideEvidence,
    PolynomialSideFailure, PolynomialSideOptions, PolynomialSideWork,
    certify_polynomial_side_integral, validate_polynomial_side_options,
};

#[cfg(test)]
mod regularity_baseline_tests;

mod bezier_regularity;
pub use bezier_regularity::{
    BezierRegularityCause, BezierRegularityCompletion, BezierRegularityDecision,
    BezierRegularityFailure, BezierRegularityOptions, BezierRegularityQuery,
    BezierRegularityReport, BezierRegularitySource, BezierRegularityStage,
    certify_bezier_regularity,
};

mod enclosure_sum;
pub use enclosure_sum::{
    CertifiedEnclosureSum, EnclosureSumCause, EnclosureSumError, EnclosureSumEvidence,
    EnclosureSumOperation, EnclosureSumOptions, sum_nonnegative_enclosures,
};

mod frame_transport;
pub use frame_transport::{
    CertifiedFrameSpatialBudget, CertifiedFrameTransport, CertifiedRotatedFrameSpatialBudget,
    CertifiedRotatedFrameTransport, FrameRotationError, FrameRotationFailure, FrameRotationReport,
    FrameSpatialInput, FrameTransportError, FrameTransportField, FrameTransportInput,
    FrameTransportQuantity, certify_frame_transport, certify_frame_transport_at_angle,
};

#[cfg(test)]
mod cone_thread_contracts;

pub use conical_profile::{ConicalChartJoinsError, certify_conical_joins_in_chart};

pub use conical_profile::{
    ConicalChartTrimPairError, ConicalChartTrimPairFailure, ConicalDevelopedBoundaryEvidence,
    ConicalDevelopedBoundarySpan, certify_conical_trim_pair_in_chart,
    certify_conical_trim_pair_with_developed_boundary,
};

/// Checked separation of decoded ruled faces and cap trims from a coaxial stock carrier.
///
/// See [`conical_separation::certify_separation`] for admitted frames and caller obligations.
pub mod conical_separation;

mod segment_plane;
pub use segment_plane::{
    SegmentPlaneCut, SegmentPlaneFailure, SegmentPlaneIntersection, SegmentWorkPlane,
    intersect_segment_plane,
};

mod cardinal_bend;
pub use cardinal_bend::{
    BINARY64_QUARTER_ANGLE_ERROR, CardinalBendFailure, CardinalBendPlacement, CardinalLegCut,
    cardinal_bend_placement,
};
mod directed_line;
pub use directed_line::{
    DirectedLineContact, DirectedLineHit, DirectedLineIncidenceFailure, DirectedLineParameter,
    DirectedLinePlacement, LineEvidenceQuantity, LineParameterLocation, directed_line_contact,
};
