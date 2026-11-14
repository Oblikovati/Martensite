//! Ideal measures, retained flux and native estimates have distinct public types.
use super::*;
use martensite_tkbo::body_check::ValidationReport;

/// Certified outward enclosure in the quantity's documented units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedCertifiedRange {
    /// Lower bound.
    pub lower: f64,
    /// Upper bound.
    pub upper: f64,
}
/// Certified nonnegative upper bound in the quantity's documented units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedCertifiedBound {
    /// Outward upper bound.
    pub upper: f64,
}
/// Native quadrature estimate; neither field is promoted to a certified bound.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WrappedNativeVolumeEstimate {
    /// Native reported volume in cubic model units.
    pub value: f64,
    /// Native reported integration error estimate.
    pub integration_error: f64,
}
/// Validation and achieved errors for one selected material component.
#[derive(Clone, Debug)]
pub struct WrappedMemberReport {
    /// Captured developed area with its original evidence classification.
    pub planar_area: martensite_tkbo::measured::Measured,
    /// Exact frozen incidence and retained trim coverage.
    pub coverage: WrappedIncidenceCoverage,
    /// Ordered native stages, retaining sampled and native-search coverage labels.
    pub validation: ValidationReport,
    /// Requested-versus-achieved spatial evidence.
    pub spatial: WrappedSpatialReport,
    /// Ideal, retained-flux and estimated native measures.
    pub measures: WrappedMeasureReport,
}
/// Achieved spatial bounds, with parameter and model units distinguished per field.
#[derive(Clone, Debug)]
pub struct WrappedSpatialReport {
    /// Developed displacement of the ideal profile from the authored profile.
    pub profile_error: WrappedCertifiedBound,
    /// Normal-sweep discrepancy due to stored coefficients and requested-depth rounding.
    pub normal_storage_error: WrappedCertifiedBound,
    /// Coefficient rounding at stored radii, before requested-depth composition.
    pub storage_rounding_error: WrappedCertifiedBound,
    /// Space-edge versus cap-pcurve discrepancy, including fit approximation.
    pub cap_trace_error: WrappedCertifiedBound,
    /// Normalized retained profile versus decoded cap pcurve, in radians.
    pub cap_angular_error: WrappedCertifiedBound,
    /// Normalized retained profile versus decoded cap pcurve height, in model units.
    pub cap_height_error: WrappedCertifiedBound,
    /// Filled native-cap versus ideal-cap Hausdorff bound after homotopy/material gates.
    pub cap_realization_error: WrappedCertifiedBound,
    /// Largest retained effective edge or face tolerance in model units.
    pub native_tolerance: f64,
}
/// Certified ideal measures and boundary flux, alongside the native estimate.
#[derive(Clone, Debug)]
pub struct WrappedMeasureReport {
    /// Volume of the ideal normalized-profile normal extrusion.
    pub ideal_volume: WrappedCertifiedRange,
    /// Signed integral through the retained boundary patches; this is not a closed-body volume claim.
    pub boundary_flux: WrappedCertifiedRange,
    /// Original authored profile's ideal normal volume.
    pub authored_volume: WrappedCertifiedRange,
    /// Certified retained-flux versus ideal-volume discrepancy.
    pub retained_ideal_discrepancy: WrappedCertifiedBound,
    /// Native volume estimate, separate from the certified ideal volume and flux.
    pub native_volume: WrappedNativeVolumeEstimate,
}
/// Counts of checked frozen graph incidences and stored pcurve representations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WrappedIncidenceCoverage {
    /// Bound semantic entities.
    pub bindings: usize,
    /// Checked faces.
    pub faces: usize,
    /// Checked wires.
    pub wires: usize,
    /// Checked edges.
    pub edges: usize,
    /// Checked vertices.
    pub vertices: usize,
    /// Checked directed edge uses.
    pub edge_uses: usize,
    /// Euler characteristic with holes accounted for.
    pub euler_characteristic: i64,
    /// Checked cap pcurves.
    pub cap_pcurves: usize,
    /// Checked ruled-side pcurves.
    pub side_pcurves: usize,
}
/// Outward family totals; integration and authored-profile errors remain separate.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedVolumeBounds {
    /// Sum of absolute retained-flux versus ideal-volume discrepancy bounds.
    pub discrepancy: WrappedCertifiedBound,
    /// Sum of flux-representative versus ideal-volume error bounds.
    pub representative_error: WrappedCertifiedBound,
    /// Sum of certified flux-integration uncertainty bounds.
    pub integration_error: WrappedCertifiedBound,
    /// Sum of authored-profile versus normalized-profile volume discrepancy bounds.
    pub profile_error: WrappedCertifiedBound,
}
/// Requested spatial limit, actual allocations and achieved family volume comparison.
#[derive(Clone, Debug, PartialEq)]
pub struct WrappedBudgetReport {
    /// Certified planar admission coverage for the original complete selection.
    pub planar: martensite_tkbo::profile_extrusion::RegionCertificate,
    /// Requested spatial epsilon.
    pub spatial_error: f64,
    /// Downward-rounded developed-profile allocation.
    pub profile_allowance: f64,
    /// Downward-rounded storage-rounding allocation.
    pub storage_allowance: f64,
    /// Cap-trace allowance, separate from storage rounding.
    pub trace_allowance: f64,
    /// Required nonadjacent and periodic clearance.
    pub required_clearance: f64,
    /// Developed tube enclosing the decoded-cap to normalized-profile homotopy.
    pub cap_homotopy_envelope: WrappedCertifiedBound,
    /// Outward strict nonadjacent separation required by the cap homotopy.
    pub cap_homotopy_clearance: f64,
    /// Original explicit volume comparison policy.
    pub volume_policy: WrappedVolumePolicy,
    /// Achieved outward family comparison bounds.
    pub volume: WrappedVolumeBounds,
}

pub(super) fn member(
    retained: &pipeline::RetainedMember,
    bounds: &publication_budget::MemberBounds,
    planar: &planar::PlanarEvidence,
) -> WrappedMemberReport {
    WrappedMemberReport {
        planar_area: planar.area,
        coverage: incidence_coverage(retained),
        validation: retained.report.clone(),
        spatial: spatial_report(retained, bounds),
        measures: measure_report(retained),
    }
}
fn spatial_report(
    retained: &pipeline::RetainedMember,
    bounds: &publication_budget::MemberBounds,
) -> WrappedSpatialReport {
    WrappedSpatialReport {
        profile_error: bound(bounds.profile),
        normal_storage_error: bound(bounds.storage),
        storage_rounding_error: storage_max(retained, |p| p.certificate.normal_sweep_error_bound()),
        cap_trace_error: bound(bounds.trace),
        cap_angular_error: storage_max(retained, |p| p.cap_profile.angular_position_bound()),
        cap_height_error: storage_max(retained, |p| p.cap_profile.height_position_bound()),
        cap_realization_error: bound(bounds.cap_realization),
        native_tolerance: bounds.trace_tolerance,
    }
}
fn storage_max(
    retained: &pipeline::RetainedMember,
    value: impl Fn(&storage::StoredProof) -> f64,
) -> WrappedCertifiedBound {
    bound(retained.storage.iter().map(value).fold(0., f64::max))
}
fn measure_report(retained: &pipeline::RetainedMember) -> WrappedMeasureReport {
    let comparison = &retained.measures.comparison;
    let normal = comparison.normal_certificate();
    WrappedMeasureReport {
        ideal_volume: range(normal.profile_volume_enclosure()),
        boundary_flux: range(comparison.flux_certificate().enclosure()),
        authored_volume: range(normal.authored_volume_enclosure()),
        retained_ideal_discrepancy: bound(comparison.discrepancy_bound()),
        native_volume: WrappedNativeVolumeEstimate {
            value: retained.volume_estimate,
            integration_error: retained.integration_estimate,
        },
    }
}
pub(super) fn budget(
    evidence: &publication_budget::BudgetEvidence,
    policy: WrappedVolumePolicy,
    planar: martensite_tkbo::profile_extrusion::RegionCertificate,
) -> WrappedBudgetReport {
    let limits = evidence.limits.shares();
    WrappedBudgetReport {
        planar,
        spatial_error: limits.spatial,
        profile_allowance: limits.profile,
        storage_allowance: limits.storage,
        trace_allowance: limits.trace,
        required_clearance: limits.clearance,
        cap_homotopy_envelope: bound(evidence.cap_homotopy.envelope),
        cap_homotopy_clearance: evidence.cap_homotopy.required_clearance,
        volume_policy: policy,
        volume: volume_report(&evidence.volume),
    }
}
fn volume_report(evidence: &publication_budget::FamilyVolumeBounds) -> WrappedVolumeBounds {
    WrappedVolumeBounds {
        discrepancy: bound(evidence.discrepancy),
        representative_error: bound(evidence.representative_error),
        integration_error: bound(evidence.integration_error),
        profile_error: bound(evidence.profile_error),
    }
}

fn incidence_coverage(member: &pipeline::RetainedMember) -> WrappedIncidenceCoverage {
    let graph = &member.coverage;
    WrappedIncidenceCoverage {
        bindings: graph.binding_count,
        faces: graph.faces,
        wires: graph.wires,
        edges: graph.edges,
        vertices: graph.vertices,
        edge_uses: graph.edge_uses,
        euler_characteristic: graph.euler_characteristic,
        cap_pcurves: member.storage.iter().map(|p| p.trims.cap_pcurves).sum(),
        side_pcurves: member.storage.iter().map(|p| p.trims.side_pcurves).sum(),
    }
}
fn bound(upper: f64) -> WrappedCertifiedBound {
    WrappedCertifiedBound { upper }
}
fn range([lower, upper]: [f64; 2]) -> WrappedCertifiedRange {
    WrappedCertifiedRange { lower, upper }
}
