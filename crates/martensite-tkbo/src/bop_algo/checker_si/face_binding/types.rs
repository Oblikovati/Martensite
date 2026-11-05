// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::predicates::roots::{
    DyadicBound,
    metric_bounds::{MetricEmbedding, ResidualDecision, ResidualError},
};

#[derive(Clone, Debug)]
pub(crate) struct AffinePlacement {
    pub linear: [[f64; 3]; 3],
    pub origin: [f64; 3],
    pub scale: f64,
}
impl AffinePlacement {
    pub(super) fn embedding(&self) -> MetricEmbedding<3> {
        MetricEmbedding {
            linear: self.linear,
            origin: self.origin,
            scale: self.scale,
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) struct OwnedSpline {
    pub degree: i32,
    pub poles: Vec<[f64; 3]>,
    pub weights: Vec<f64>,
    pub knots: Vec<f64>,
    pub multiplicities: Vec<i32>,
    pub periodic: bool,
    pub domain: [f64; 2],
}
#[derive(Clone, Debug)]
pub(crate) struct OwnedLine<const DIM: usize> {
    pub origin: [f64; DIM],
    pub direction: [f64; DIM],
}
#[derive(Clone, Debug)]
pub(crate) enum OwnedSpaceCurve {
    Line(OwnedLine<3>),
    Spline(OwnedSpline),
}
/// Local positions address occurrences; actual wrapper identity is retained independently.
#[derive(Clone, Debug)]
pub(crate) struct ShapeOccurrenceIdentity {
    pub tshape_token: usize,
    pub location_datums: Vec<(usize, i32)>,
    pub orientation: Orientation,
}
#[derive(Clone, Debug)]
pub(crate) struct EdgeOccurrenceContext {
    pub face: ShapeOccurrenceIdentity,
    pub wire: ShapeOccurrenceIdentity,
    pub edge: ShapeOccurrenceIdentity,
    pub wire_position: usize,
    pub edge_position: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EndpointSide {
    First,
    Last,
}
#[derive(Clone, Debug)]
pub(crate) enum EndpointParameterObservation {
    TopologicalRangeEnd {
        side: EndpointSide,
        recorded_range: [f64; 2],
        returned: Result<Option<f64>, Failure>,
        modified_before: bool,
        modified_after: bool,
    },
    NotQueried {
        orientation: Orientation,
    },
}
#[derive(Clone, Debug)]
pub(crate) struct OwnedVertexUse {
    pub raw_placement: AffinePlacement,
    pub position: usize,
    pub raw: ShapeOccurrenceIdentity,
    pub intrinsic: ShapeOccurrenceIdentity,
    pub boundary: ShapeOccurrenceIdentity,
    pub local_point: [f64; 3],
    pub placement: AffinePlacement,
    pub raw_tolerance: f64,
    pub effective_tolerance: f64,
    pub point_representation_count: usize,
    pub parameter: EndpointParameterObservation,
}
#[derive(Clone, Debug)]
pub(crate) struct OwnedEdgeUse {
    pub vertices: Vec<OwnedVertexUse>,
    pub raw_tolerance: f64,
    pub context: EdgeOccurrenceContext,
    pub canonical_orientation: Orientation,
    pub pcurve: OwnedLine<2>,
    pub range: [f64; 2],
    pub curve: OwnedSpaceCurve,
    pub curve_range: [f64; 2],
    pub placement: AffinePlacement,
    pub tolerance: f64,
    pub stored_branch: u8,
    pub geometric_seam: bool,
}
#[derive(Clone, Debug)]
pub(crate) struct OccurrenceRelation {
    pub left: usize,
    pub right: usize,
    pub same: bool,
    pub location_equal: bool,
}
/// Invocation tokens are provenance only, never persistent identity or mutation revisions.
#[derive(Clone, Debug)]
pub(crate) struct FaceInvocationIdentity {
    pub tshape_token: usize,
    pub location_datums: Vec<(usize, i32)>,
}
#[derive(Clone, Debug)]
pub(crate) struct OwnedFacePacket {
    pub raw_face_tolerance: f64,
    pub vertex_relations: Vec<OccurrenceRelation>,
    pub invocation: FaceInvocationIdentity,
    pub basis: OwnedSpline,
    pub direction: [f64; 3],
    pub support_placement: AffinePlacement,
    pub uv_bounds: [f64; 4],
    pub face_orientation: Orientation,
    pub face_tolerance: f64,
    pub wire_count: usize,
    pub edges: Vec<OwnedEdgeUse>,
    pub relations: Vec<OccurrenceRelation>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BoundaryRole {
    UFirst,
    ULast,
    VFirst,
    VLast,
}
#[derive(Debug)]
pub(crate) struct EdgeConformance {
    pub occurrence: usize,
    pub existing_spatial_limit: f64,
    pub proven_squared_upper: DyadicBound,
    pub coefficient_count: usize,
    pub bound_queries: usize,
    pub existing_squared_limit: DyadicBound,
}
#[derive(Debug)]
pub(crate) struct StripPrerequisites {
    pub roles: [BoundaryRole; 4],
    pub conformance: Vec<EdgeConformance>,
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BindingIncomplete {
    ReleasedSupportCertificateNotBound,
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SourceAtomicity {
    Unestablished,
}
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TopologyObservation {
    EdgeOccurrencesOnlyVerticesUnbound,
    OwnedVertexEndpointIncidenceBound,
}
#[derive(Debug)]
pub(crate) struct FaceBindingObservation {
    pub vertex_binding: VertexBindingObservation,
    pub prerequisites: Result<StripPrerequisites, BindingFailure>,
    pub completion: BindingIncomplete,
    pub source_atomicity: SourceAtomicity,
    pub topology_observation: TopologyObservation,
}
#[derive(Debug)]
pub(crate) enum BindingFailure {
    VertexEndpoint {
        edge: usize,
        vertex: usize,
        check: &'static str,
        cause: Box<BindingFailure>,
    },
    Native(Failure),
    Unsupported {
        role: &'static str,
        actual: String,
    },
    Count {
        role: &'static str,
        actual: usize,
        expected: usize,
    },
    Domain {
        role: &'static str,
        actual: Vec<f64>,
    },
    Exact(ResidualError),
    Conformance {
        occurrence: usize,
        limit: f64,
        decision: ResidualDecision,
    },
    Cancelled,
}
impl From<Failure> for BindingFailure {
    fn from(value: Failure) -> Self {
        Self::Native(value)
    }
}
impl From<ResidualError> for BindingFailure {
    fn from(value: ResidualError) -> Self {
        Self::Exact(value)
    }
}

#[derive(Debug)]
pub(crate) struct EndpointBound {
    pub requested_spatial_limit: f64,
    pub requested_squared_limit: DyadicBound,
    pub certified_squared_upper: DyadicBound,
    pub threshold_queries: usize,
    pub root_bisections: u64,
}
#[derive(Debug)]
pub(crate) struct VertexEndpointCertificate {
    pub edge_address: usize,
    pub vertex_address: usize,
    pub source_vertex_position: usize,
    pub parameter: f64,
    pub side: EndpointSide,
    pub corner: usize,
    pub curve: EndpointBound,
    pub support: EndpointBound,
}
#[derive(Debug)]
pub(crate) struct OwnedVertexBinding {
    pub endpoints: Vec<VertexEndpointCertificate>,
    pub identity_pairs_checked: usize,
}
#[derive(Debug)]
pub(crate) enum VertexBindingObservation {
    NotAttempted,
    Incomplete(BindingFailure),
    Bound(OwnedVertexBinding),
}
pub(super) struct EndpointUse<'a> {
    pub edge_address: usize,
    pub vertex_address: usize,
    pub vertex: &'a OwnedVertexUse,
    pub parameter: f64,
    pub side: EndpointSide,
    pub corner: usize,
}
