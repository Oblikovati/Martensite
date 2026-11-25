//! Cone receipts retain their conditional meaning and separate requested work from consumption.
use super::*;
use martensite_tkbo::body_check::ValidationReport;

/// Observed native checks and source-bound bounds for one cone member.
#[derive(Clone, Debug)]
pub struct WrappedConeMemberReport {
    /// Authored planar material, retaining its measurement classification.
    pub planar_area: martensite_tkbo::measured::Measured,
    /// Actual frozen incidence and counted retained pcurves.
    pub coverage: WrappedIncidenceCoverage,
    /// Ordered native checks with their original coverage modes.
    pub validation: ValidationReport,
    /// Achieved authored-boundary versus stored-boundary discrepancy in model units.
    pub boundary_error: WrappedCertifiedBound,
    /// Achieved space-edge versus analytic-cap occurrence discrepancy in model units.
    pub cap_trace_error: WrappedCertifiedBound,
    /// Largest observed edge or face tolerance in model units.
    pub native_tolerance: f64,
    /// Maximum certified decoded first/last depth-edge discrepancy from the reference edges.
    pub depth_storage_error: [WrappedCertifiedBound; 2],
    /// Maximum certified full-depth ruled-side discrepancy from the reference surface.
    pub ruled_storage_error: WrappedCertifiedBound,
    /// Effective downward-rounded storage allocation applied to every retained span.
    pub storage_allowance: f64,
    /// Authored normal-volume enclosure, conditional on the admitted normal map.
    pub authored_volume: WrappedCertifiedRange,
    /// Signed occurrence flux of `F(p)=(p-origin)/3`; this does not assert geometric closure.
    pub boundary_flux: WrappedCertifiedRange,
    /// Certified volume enclosure of the executed common-chart q/L reference body.
    pub reference_volume: WrappedCertifiedRange,
    /// Separate authored, reference-body and conditional proxy-flux comparison bounds.
    pub volume: WrappedConeVolumeBounds,
    /// Native quadrature estimate, separate from certified occurrence flux.
    pub native_volume: WrappedNativeVolumeEstimate,
    /// Actual member storage, material and measure arithmetic, excluding family source preparation.
    pub arithmetic_operations: u32,
    /// Actual bounded arithmetic operations consumed by the retained join certificate.
    pub join_arithmetic_operations: u32,
    /// Executed compact UV material admission and outward filled-cap bound.
    pub material: WrappedConeMaterialReport,
}

/// Actual material-domain checks, separate from original developed-selection admission.
#[derive(Clone, Debug)]
pub struct WrappedConeMaterialReport {
    /// Actual proof engine used for retained polynomial UV material admission.
    pub planar_proof: martensite_tkbo::profile_extrusion::PlanarProofVersion,
    /// Census of the retained polynomial UV material domain.
    pub planar: martensite_tkbo::profile_extrusion::RegionCertificate,
    /// Requested, effective and consumed UV-domain proof work.
    pub planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    /// Actual scalar admission resolution for the retained `[V,U]` chart coordinates.
    /// This is not a physical separation or spatial-error bound.
    pub chart_admission_tolerance: f64,
    /// Outward authored versus retained filled-cap discrepancy in model units.
    pub cap_realization_error: WrappedCertifiedBound,
}

/// Original policies, actual proof work and achieved outward bounds for the cone family.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedConeBudgetReport {
    /// Original planar admission census.
    pub planar: martensite_tkbo::profile_extrusion::RegionCertificate,
    /// Actual original developed-selection admission work, before retained UV-domain checks.
    pub source_planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    /// Requested, effective and consumed planar work across original and retained UV domains.
    pub planar_work: martensite_tkbo::profile_extrusion::PlanarProofWorkReceipt,
    /// Actually charged arithmetic operations across selection and all retained members.
    pub arithmetic_operations: u32,
    /// Integration cells charged for the authored and retained planar programs, reference volume and proxy flux.
    pub integration_subintervals: u32,
    /// Original spatial limit in model units.
    pub requested_spatial_error: f64,
    /// Maximum composed boundary and trace discrepancy, including native tolerance, in model units.
    pub achieved_spatial_error: WrappedCertifiedBound,
    /// Original-to-reference developed-profile displacement.
    pub reference_developed_error: WrappedCertifiedBound,
    /// Original-to-reference filled normal-body discrepancy.
    pub reference_filled_error: WrappedCertifiedBound,
    /// Decoded edge and full-depth ruling discrepancy from the reference representation.
    pub reference_storage_error: WrappedCertifiedBound,
    /// Separate reference filled-cap versus analytic proxy-cap discrepancy.
    pub cap_reference_proxy_error: WrappedCertifiedBound,
    /// Original explicit volume-comparison policy.
    pub volume_policy: WrappedVolumePolicy,
    /// Actual outward family comparison bounds.
    pub volume: WrappedConeVolumeBounds,
}

/// Outward cone volume comparisons for one member or the summed family.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedConeVolumeBounds {
    /// Authored-volume versus retained occurrence-flux discrepancy, summed for a family.
    pub discrepancy: WrappedCertifiedBound,
    /// Retained occurrence-flux integration uncertainty, summed for a family.
    pub integration_error: WrappedCertifiedBound,
    /// Authored-volume versus reference-body volume enclosure discrepancy, summed for a family.
    pub authored_reference_discrepancy: WrappedCertifiedBound,
    /// Conditional proxy occurrence-flux versus reference-volume discrepancy, summed for a family.
    pub proxy_reference_discrepancy: WrappedCertifiedBound,
    /// Reference integration uncertainty, already included in comparisons and summed for a family.
    pub reference_integration_error: WrappedCertifiedBound,
}
