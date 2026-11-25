//! Final native archive and the closed-domain qualification it satisfied.
use super::*;
use martensite_tkbo::body_check::CheckMode;
use martensite_tkmath::predicates::roots::metric_bounds::CertifiedRotatedFrameSpatialBudget;

/// An attached frustum result whose complete validation finished in exact mode.
///
/// The owned archive does not keep decoded native handles or a standalone tool.
pub struct CertifiedConeFeatureResult {
    pub(super) attempt: Attempt,
    pub(super) bytes: Vec<u8>,
    pub(super) history: CylinderFeatureHistory,
    pub(super) authored: Vec<WrappedHistoryRecord>,
    pub(super) report: CertifiedConeAttachmentReport,
    pub(super) spatial_budget: CertifiedRotatedFrameSpatialBudget,
}

/// Geometry, topology and tolerance coverage bound to one committed attached solid.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct CertifiedConeAttachmentReport {
    /// Number of final faces.
    pub faces: usize,
    /// Number of distinct edges.
    pub edges: usize,
    /// Number of distinct vertices.
    pub vertices: usize,
    /// Number of face boundary loops, including holes.
    pub loops: usize,
    /// Genus of the connected shell.
    pub genus: i64,
    /// Number of retained tool sides checked against their actual 3D edges.
    pub tool_sides: usize,
    /// Number of far-cap edge uses checked against a source-bound numerical residual.
    pub cap_edge_uses: usize,
    /// Number of stock cubic pieces used in the self and pair proof.
    pub stock_trim_pieces: usize,
    /// Number of distinct stock trim-piece pair domains.
    pub trim_piece_pairs: usize,
    /// Number of directed native incident contact paths.
    pub incident_paths: usize,
    /// Number of stock faces that carry proved joins.
    pub stock_contact_faces: usize,
    /// Interval cells visited for contact census and window replay.
    pub contact_interval_visits: usize,
    /// Minimum and maximum loaded thresholds for side, rail, ruling and vertex contacts.
    pub contact_thresholds: [[f64; 4]; 2],
    /// Upper distance from a near contact to the first boundary hit.
    pub prefix_distance_upper: f64,
    /// Upper search distance in the weighted chart for excluding foreign trim.
    pub foreign_clearance_query_upper: f64,
    /// Lower half-Hessian bounds for moving and fixed-point retractions.
    pub contact_convexity_lower: [f64; 2],
    /// Upper continuation distances for own-rail and incident-rail paths.
    pub contact_continuation_upper: [f64; 2],
    /// Maximum loaded tolerance among the admitted final vertices, edges and faces.
    pub maximum_tolerance: f64,
    /// Authored pair domains certified before native tool construction.
    pub authored_pairs: usize,
    /// Native cap pair domains certified before final attachment.
    pub cap_pairs: usize,
    /// Charged exact planar bisections in the tool operation.
    pub pair_bisections: u64,
    /// Examined integration cells in the tool operation.
    pub integration_cells: u32,
    /// Charged exact arithmetic operations in the tool operation.
    pub arithmetic_operations: u32,
    /// Exact operations consumed by the actual native tool-family proof.
    pub family_operations: u32,
    /// Requested spatial limit for the newly installed seam traces.
    pub requested_sew_limit: f64,
    /// Certified full-interval maximum same-parameter gap on the installed traces.
    pub seam_residual_upper: f64,
}

impl CertifiedConeAttachmentReport {
    /// The combinatorial and bounded arithmetic validation mode for every completed stage.
    /// Example: `report.check_mode()`.
    pub fn check_mode(&self) -> CheckMode {
        CheckMode::Exact
    }
}

impl CertifiedConeFeatureResult {
    /// Material change applied at this support revision. Example: `feature.kind()`.
    pub fn kind(&self) -> CylinderFeatureKind {
        self.attempt.kind
    }

    /// The frozen exact geometry and indexed history targets. Example: `feature.encoded_brep()`.
    pub fn encoded_brep(&self) -> &[u8] {
        &self.bytes
    }

    /// Complete support and tool image history. Example: `feature.history().images.len()`.
    pub fn history(&self) -> &CylinderFeatureHistory {
        &self.history
    }

    /// Original authored provenance beneath the retained tool keys. Example: `feature.authored_history()`.
    pub fn authored_history(&self) -> &[WrappedHistoryRecord] {
        &self.authored
    }

    /// Completed check coverage for the published solid. Example: `feature.validation().faces`.
    pub fn validation(&self) -> &CertifiedConeAttachmentReport {
        &self.report
    }

    /// Versioned allocation for the requested physical tool error. Example: `feature.spatial_budget()`.
    pub fn spatial_budget(&self) -> &CertifiedRotatedFrameSpatialBudget {
        &self.spatial_budget
    }

    /// Decode fresh native owners. Example: `let body = feature.export_native()?.body;`.
    pub fn export_native(&self) -> Result<DetachedCylinderFeature, CylinderFeatureFailure> {
        body::decode(
            &self.bytes,
            self.history.images.keys().copied().collect(),
            self.attempt,
        )
    }
}

const _: fn() = || {
    fn shared<T: Send + Sync>() {}
    shared::<CertifiedConeFeatureResult>();
    shared::<CertifiedConeAttachmentReport>();
};
