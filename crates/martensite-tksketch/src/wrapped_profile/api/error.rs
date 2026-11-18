//! Typed public operation failures retain the original owned cause.
use super::*;
use crate::surface_sketch::HistorySource;

/// Operation that failed before returning a complete value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WrappedOperation {
    /// Build and validate a new frozen family.
    NormalExtrusion,
    /// Decode a fresh native representation of a frozen result.
    DetachedExport,
}
/// Public operation boundary at which work was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WrappedStage {
    /// Support, depth or policy admission.
    Admission,
    /// Authored profile resolution.
    Input,
    /// Geometry, certificates and ordered native validation.
    Construction,
    /// Authored history publication.
    History,
    /// Frozen physical placement and its world-space evidence.
    Placement,
    /// Final immutable result handoff.
    Publication,
    /// Detached native decoding.
    Export,
}
/// Failure category, separate from the complete retained cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WrappedFailureClass {
    /// Input did not satisfy its declared shape or finite-domain constraints.
    InvalidInput,
    /// The requested geometry representation is not implemented.
    Unsupported,
    /// A required computation or certificate did not complete successfully.
    Incomplete,
    /// Cooperative cancellation prevented handoff.
    Cancelled,
    /// Native serialization or decoding failed.
    Native,
}
// Bug #547: shared cylinder error formatting must not retain concrete cone code.
pub(in crate::surface_sketch::wrapped_profile) trait ConeFailureCause:
    std::error::Error + Send + Sync
{
    fn planar_work_failure(
        &self,
    ) -> Option<&martensite_tkbo::profile_extrusion::PlanarRegionWorkFailure>;
}
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) enum Cause {
    Support(crate::surface_sketch::CylinderSupport),
    Depths([f64; 2]),
    Input(profile_input::ProfileInputFailure),
    Family(family::FamilyFailure),
    Cone(Box<dyn ConeFailureCause>),
    History(history_projection::HistoryMapFailure),
    Placement(Box<placement::PlacementFailure>),
    Export(Failure),
    ExportBindings(export_bindings::ExportBindingFailure),
    Cancelled,
}
/// No successful or partial geometry accompanies this typed refusal.
#[derive(Debug)]
pub struct WrappedNormalFailure {
    pub(super) operation: WrappedOperation,
    pub(super) stage: WrappedStage,
    pub(super) class: WrappedFailureClass,
    pub(super) entities: Vec<HistorySource>,
    pub(super) cause: Box<Cause>,
}
impl WrappedNormalFailure {
    /// Retained planar work refusal, when planar capture failed; `failure.planar_work_failure()`.
    pub fn planar_work_failure(
        &self,
    ) -> Option<&martensite_tkbo::profile_extrusion::PlanarRegionWorkFailure> {
        if let Cause::Cone(cause) = self.cause.as_ref() {
            return cause.planar_work_failure();
        }
        let Cause::Family(family) = self.cause.as_ref() else {
            return None;
        };
        let family::FamilyFailure::Selection(selection) = family else {
            return None;
        };
        let planar = match selection.as_ref() {
            selection::SelectionFailure::Capture(cause) => cause,
            selection::SelectionFailure::Region { cause, .. } => cause,
            _ => return None,
        };
        match planar.as_ref() {
            planar::PlanarFailure::WorkCapture(cause) => Some(cause),
            _ => None,
        }
    }
    /// Failed operation; `failure.operation()`.
    pub fn operation(&self) -> WrappedOperation {
        self.operation
    }
    /// Refusing boundary; `failure.stage()`.
    pub fn stage(&self) -> WrappedStage {
        self.stage
    }
    /// Typed failure category; `failure.class()`.
    pub fn class(&self) -> WrappedFailureClass {
        self.class
    }
    /// Known affected authored entities; `failure.entities()`.
    pub fn entities(&self) -> &[HistorySource] {
        &self.entities
    }
}
impl std::fmt::Display for WrappedNormalFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} stage={:?} class={:?} entities={:?}: {}",
            self.operation, self.stage, self.class, self.entities, self.cause
        )
    }
}
impl std::error::Error for WrappedNormalFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
impl std::fmt::Display for Cause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Support(actual) => write!(f, "support={actual:?}; expected zero chart origin"),
            Self::Depths(actual) => write!(
                f,
                "depths={actual:?}; expected increasing finite signed normal depths"
            ),
            Self::Input(cause) => write!(f, "{cause}"),
            Self::Family(cause) => write!(f, "{cause}"),
            Self::Cone(cause) => write!(f, "{cause}"),
            Self::History(cause) => write!(f, "{cause}"),
            Self::Placement(cause) => write!(f, "{cause}"),
            Self::Export(cause) => write!(f, "native export: {cause:?}"),
            Self::ExportBindings(cause) => write!(f, "{cause}"),
            Self::Cancelled => write!(f, "cancelled; expected completed result before handoff"),
        }
    }
}
impl std::error::Error for Cause {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Export(cause) => Some(cause),
            Self::ExportBindings(cause) => Some(cause),
            Self::Input(cause) => Some(cause),
            Self::Family(cause) => Some(cause),
            Self::Cone(cause) => Some(cause.as_ref()),
            Self::History(cause) => Some(cause),
            Self::Placement(cause) => Some(cause.as_ref()),
            _ => None,
        }
    }
}
impl std::error::Error for profile_input::ProfileInputFailure {}
impl std::error::Error for family::FamilyFailure {}
impl std::error::Error for history_projection::HistoryMapFailure {}

pub(in crate::surface_sketch::wrapped_profile) fn failure(
    stage: WrappedStage,
    class: WrappedFailureClass,
    region: Option<SurfaceRegionId>,
    cause: Cause,
) -> WrappedNormalFailure {
    let mut entities = region.into_iter().map(HistorySource::Region).collect();
    let class = annotate(&cause, class, &mut entities);
    with_entities(stage, class, entities, cause)
}
pub(in crate::surface_sketch::wrapped_profile) fn with_entities(
    stage: WrappedStage,
    class: WrappedFailureClass,
    mut entities: Vec<HistorySource>,
    cause: Cause,
) -> WrappedNormalFailure {
    entities.sort();
    entities.dedup();
    WrappedNormalFailure {
        operation: WrappedOperation::NormalExtrusion,
        stage,
        class,
        entities,
        cause: Box::new(cause),
    }
}

pub(super) fn cancelled(stage: WrappedStage) -> WrappedNormalFailure {
    failure(
        stage,
        WrappedFailureClass::Cancelled,
        None,
        Cause::Cancelled,
    )
}

use martensite_tkbo::profile_extrusion::DeclineCause;

pub(super) fn annotate(
    cause: &Cause,
    class: WrappedFailureClass,
    entities: &mut Vec<HistorySource>,
) -> WrappedFailureClass {
    match cause {
        Cause::Input(cause) => input_context(cause, class, entities),
        Cause::History(cause) => {
            history_projection_entities(cause, entities);
            class
        }
        _ => class,
    }
}

pub(in crate::surface_sketch::wrapped_profile) fn input_context(
    cause: &profile_input::ProfileInputFailure,
    class: WrappedFailureClass,
    entities: &mut Vec<HistorySource>,
) -> WrappedFailureClass {
    use profile_input::ProfileInputFailure as I;
    input_entities(cause, entities);
    if matches!(cause, I::Unsupported(_)) {
        return WrappedFailureClass::Unsupported;
    }
    class
}

fn input_entities(cause: &profile_input::ProfileInputFailure, entities: &mut Vec<HistorySource>) {
    use profile_input::ProfileInputFailure as I;
    match cause {
        I::DuplicateVertex(vertex) => {
            entities.push(HistorySource::Vertex(SurfaceVertexId(vertex.0)))
        }
        I::MissingVertex { curve, vertex } | I::Endpoint { curve, vertex, .. } => {
            entities.extend([
                HistorySource::Curve(SurfaceCurveId(curve.0)),
                HistorySource::Vertex(SurfaceVertexId(vertex.0)),
            ])
        }
        I::EmptyLoop { region, .. } => entities.push(HistorySource::Region(*region)),
        _ => input_curve(cause, entities),
    }
}

fn input_curve(cause: &profile_input::ProfileInputFailure, entities: &mut Vec<HistorySource>) {
    use profile_input::ProfileInputFailure as I;
    let curve = match cause {
        I::DuplicateCurve(curve)
        | I::MissingCurve(curve)
        | I::ReusedCurve(curve)
        | I::Unsupported(curve)
        | I::Spline { curve, .. }
        | I::Weight { curve, .. }
        | I::Knot { curve, .. }
        | I::NonFinite { curve, .. } => SurfaceCurveId(curve.0),
        I::UnknownSource(curve) | I::Point { curve, .. } => *curve,
        I::Interval(span) => span.curve,
        _ => return,
    };
    entities.push(HistorySource::Curve(curve));
}

pub(in crate::surface_sketch::wrapped_profile) fn history_projection_entities(
    cause: &history_projection::HistoryMapFailure,
    entities: &mut Vec<HistorySource>,
) {
    use history_projection::HistoryMapFailure as H;
    match cause {
        H::Owner(owner) => entities.push(HistorySource::Region(*owner)),
        H::Parameter { owner, cause } => {
            entities.push(HistorySource::Region(*owner));
            input_context(cause, WrappedFailureClass::Incomplete, entities);
        }
        H::Binding { key, .. } => entities.push(failure_projection::binding_source(*key)),
        H::Duplicate(key) => entities.push(public_source(key.source)),
        H::Count { .. } => {}
    }
}

pub(super) fn decline_entities(cause: &DeclineCause, entities: &mut Vec<HistorySource>) {
    entities
        .extend(decline_curve(cause).map(|curve| HistorySource::Curve(SurfaceCurveId(curve.0))));
    entities.extend(
        decline_vertex(cause).map(|vertex| HistorySource::Vertex(SurfaceVertexId(vertex.0))),
    );
    match cause {
        DeclineCause::SelfIntersecting { curves, .. } => entities.extend(
            curves
                .iter()
                .map(|c| HistorySource::Curve(SurfaceCurveId(c.0))),
        ),
        DeclineCause::CoincidentVertices { vertices, .. } => entities.extend(
            vertices
                .iter()
                .map(|v| HistorySource::Vertex(SurfaceVertexId(v.0))),
        ),
        _ => {}
    }
}

fn decline_curve(
    cause: &DeclineCause,
) -> Option<martensite_tkbo::profile_extrusion::ProfileCurveId> {
    use DeclineCause as D;
    match cause {
        D::NonFiniteCurve { curve }
        | D::DuplicateCurve { curve }
        | D::UnknownVertex { curve, .. }
        | D::UnsupportedCurve { curve, .. }
        | D::DegenerateCurve { curve, .. }
        | D::InvalidCurve { curve, .. }
        | D::EndOffCurve { curve, .. }
        | D::ArcRadiusMismatch { curve, .. }
        | D::BridgeCurve { curve } => Some(*curve),
        _ => None,
    }
}

fn decline_vertex(
    cause: &DeclineCause,
) -> Option<martensite_tkbo::profile_extrusion::ProfileVertexId> {
    use DeclineCause as D;
    match cause {
        D::NonFiniteVertex { vertex }
        | D::DuplicateVertex { vertex }
        | D::UnknownVertex { vertex, .. }
        | D::EndOffCurve { vertex, .. }
        | D::OpenProfile { vertex, .. }
        | D::BranchingProfile { vertex, .. }
        | D::TangentBranches { vertex }
        | D::PinchedRegion { vertex, .. } => Some(*vertex),
        _ => None,
    }
}

fn public_source(source: WrappedEntitySource) -> HistorySource {
    match source {
        WrappedEntitySource::Region(region) => HistorySource::Region(region),
        WrappedEntitySource::Curve { curve, .. }
        | WrappedEntitySource::SplitVertex { curve, .. } => HistorySource::Curve(curve),
        WrappedEntitySource::Vertex(vertex) => HistorySource::Vertex(vertex),
    }
}
