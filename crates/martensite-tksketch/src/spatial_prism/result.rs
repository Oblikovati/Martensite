// SPDX-License-Identifier: GPL-2.0-only
//! Compose TKBO result history with the original wire and detach through exact copy images.
use super::{
    SpatialPrismFailure as Failure, SpatialPrismStage as Stage, failure::native, plane::Lowered,
    poll,
};
use martensite_tkbo::profile_extrusion::{
    AchievedTolerance, Finding, HistoryRecord, HistoryRelation, HistoryRole, HistorySource,
    Measured, OperationResult, PlanarProofWorkReceipt, RegionId, Resolution, ResultEntity,
    SelfIntersectionEvidence, ValidationReport,
};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tksketch3d::{
    LineId, PointId,
    wire::{LineDirection, WireRecipe},
};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

/// Original source of one native prism image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpatialPrismSource {
    /// The selected complete boundary, which supplies the two caps.
    Boundary,
    /// A directed occurrence, uniquely identified by its line within the wire.
    Line(LineId),
    /// An authored vertex of the wire.
    Point(PointId),
}

/// Stable source and role of one result entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpatialPrismHistory {
    /// Original spatial source identity.
    pub source: SpatialPrismSource,
    /// The ordinary prism's history role.
    pub role: HistoryRole,
    /// The retained ordinary prism history relation.
    pub relation: HistoryRelation,
    /// Requested wire traversal sense when the source is a line.
    pub line_direction: Option<LineDirection>,
}

/// One semantic image inside a detached native prism.
pub struct SpatialPrismImage {
    /// Its stable source and role.
    pub history: SpatialPrismHistory,
    /// A shape belonging to the same detached solid.
    pub shape: Shape,
}

/// Fresh native owners for one consumer; geometry may be mutated by that consumer.
pub struct DetachedSpatialPrism {
    /// The detached result body.
    pub solid: Shape,
    /// Complete semantic bindings inside `solid`, in source-role order.
    pub images: Vec<SpatialPrismImage>,
}

/// Complete source-bound ordinary prism with immutable evidence and private native geometry.
#[derive(Debug)]
pub struct SpatialPrismResult {
    swept: OperationResult,
    bindings: Vec<Binding>,
    anchors: [PointId; 3],
    sweep_sign: Ordering,
    region_work: PlanarProofWorkReceipt,
    evidence: SweepEvidence,
}

#[derive(Debug)]
pub(super) struct SweepEvidence {
    pub trace_bound: f64,
    pub trace_limit: f64,
    pub volume: Measured,
}

impl SpatialPrismResult {
    /// Ordered required validation of the body. Example: `result.validation().is_valid()`.
    pub fn validation(&self) -> &ValidationReport {
        self.swept.validation()
    }

    /// Proof behind the self-intersection pass. Example: `result.self_intersection_certificate()`.
    pub fn self_intersection_certificate(&self) -> SelfIntersectionEvidence {
        self.swept.self_intersection_certificate()
    }

    /// Largest stored tolerances of the accepted body. Example: `result.achieved_tolerance().max()`.
    pub fn achieved_tolerance(&self) -> AchievedTolerance {
        self.swept.achieved_tolerance()
    }

    /// Bounded volume of the retained planar sweep. Example: `result.volume().encloses(75.)`.
    pub fn volume(&self) -> Measured {
        self.evidence.volume
    }

    /// Applied ordinary prism resolution. Example: `result.resolution().weld`.
    pub fn resolution(&self) -> Resolution {
        self.swept.resolution()
    }

    /// Ordinary validation findings, including report-mode caps. Example: `result.findings()`.
    pub fn findings(&self) -> &[Finding] {
        self.swept.findings()
    }

    /// Requested source trace limit. Example: `result.source_trace_limit()`.
    pub fn source_trace_limit(&self) -> f64 {
        self.evidence.trace_limit
    }

    /// The three noncollinear authored points defining the exact plane. Example: `result.anchors()`.
    pub fn anchors(&self) -> [PointId; 3] {
        self.anchors
    }

    /// Exact sweep sign relative to the ordered authored plane anchors. Example: `result.sweep_sign()`.
    pub fn sweep_sign(&self) -> Ordering {
        self.sweep_sign
    }

    /// Certified upper bound for the start region and retained sweep boundary. Example: `result.source_trace_bound()`.
    pub fn source_trace_bound(&self) -> f64 {
        self.evidence.trace_bound
    }

    /// Actual bounded native planar certification work. Example: `result.region_proof_work()`.
    pub fn region_proof_work(&self) -> PlanarProofWorkReceipt {
        self.region_work
    }

    /// Complete history, sorted by original source and role. Example: `result.history().len()`.
    pub fn history(&self) -> impl ExactSizeIterator<Item = &SpatialPrismHistory> {
        self.bindings.iter().map(|binding| &binding.history)
    }

    /// Detach the body and its exact copied shape references. Example: `result.export_brep(|| false)?`.
    pub fn export_brep(
        &self,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<DetachedSpatialPrism, Failure> {
        poll(&mut cancelled, Stage::Export)?;
        let mut copier = native(
            BRepBuilderAPICopy::with_shape(self.swept.solid(), true, false),
            Stage::Export,
        )?;
        let solid = native(copier.shape(), Stage::Export)?.clone();
        if solid.is_same(self.swept.solid()) {
            return Err(Failure::History("fresh copied root identity"));
        }
        let images = copied_images(&solid, &copier, &self.swept, &self.bindings, &mut cancelled)?;
        poll(&mut cancelled, Stage::Export)?;
        Ok(DetachedSpatialPrism { solid, images })
    }
}

pub(super) fn new(
    wire: &WireRecipe,
    lowered: Lowered,
    region: RegionId,
    region_work: PlanarProofWorkReceipt,
    evidence: SweepEvidence,
    swept: OperationResult,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<SpatialPrismResult, Failure> {
    let bindings = compose(wire, region, &swept, cancelled)?;
    poll(cancelled, Stage::History)?;
    Ok(SpatialPrismResult {
        swept,
        bindings,
        anchors: lowered.anchors,
        sweep_sign: lowered.sweep_sign,
        region_work,
        evidence,
    })
}

#[derive(Debug)]
struct Binding {
    history: SpatialPrismHistory,
    result: ResultEntity,
}

fn compose(
    wire: &WireRecipe,
    region: RegionId,
    result: &OperationResult,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Binding>, Failure> {
    let lines: BTreeMap<_, _> = wire
        .segments()
        .iter()
        .map(|segment| (segment.source.line.get().get(), segment.source))
        .collect();
    let points: BTreeMap<_, _> = wire
        .segments()
        .iter()
        .map(|segment| {
            let point = segment.endpoints()[0];
            (point.get().get(), point)
        })
        .collect();
    let mut bindings = Vec::new();
    let mut roles = BTreeSet::new();
    for record in &result.history().records {
        poll(cancelled, Stage::History)?;
        let history = source_record(record, region, &lines, &points)?;
        if !roles.insert((history.source, history.role)) {
            return Err(Failure::History(
                "one result image per stable source and role",
            ));
        }
        bindings.push(Binding {
            history,
            result: record.result,
        });
    }
    let total = 2 + 3 * lines.len() + 3 * points.len();
    if !result.history().deleted.is_empty() || bindings.len() != total {
        return Err(Failure::History(
            "complete caps, side faces, edges and vertices",
        ));
    }
    bindings.sort_by_key(|binding| (binding.history.source, binding.history.role));
    Ok(bindings)
}

fn source_record(
    record: &HistoryRecord,
    region: RegionId,
    lines: &BTreeMap<u64, martensite_tksketch3d::wire::DirectedLineUse>,
    points: &BTreeMap<u64, PointId>,
) -> Result<SpatialPrismHistory, Failure> {
    let (source, line_direction) = match (record.source, record.role, record.result) {
        (
            HistorySource::Region(id),
            HistoryRole::StartCap | HistoryRole::EndCap,
            ResultEntity::Face(_),
        ) if id == region => (SpatialPrismSource::Boundary, None),
        (HistorySource::Curve(id), HistoryRole::SideFace, ResultEntity::Face(_))
        | (
            HistorySource::Curve(id),
            HistoryRole::StartEdge | HistoryRole::EndEdge,
            ResultEntity::Edge(_),
        ) => {
            let line = lines
                .get(&id.0)
                .ok_or(Failure::History("an authored line ancestor"))?;
            (SpatialPrismSource::Line(line.line), Some(line.direction))
        }
        (HistorySource::Vertex(id), HistoryRole::LateralEdge, ResultEntity::Edge(_))
        | (
            HistorySource::Vertex(id),
            HistoryRole::StartVertex | HistoryRole::EndVertex,
            ResultEntity::Vertex(_),
        ) => {
            let point = points
                .get(&id.0)
                .ok_or(Failure::History("an authored point ancestor"))?;
            (SpatialPrismSource::Point(*point), None)
        }
        _ => {
            return Err(Failure::History(
                "a result kind and role consistent with the authored source",
            ));
        }
    };
    Ok(SpatialPrismHistory {
        source,
        role: record.role,
        relation: record.relation,
        line_direction,
    })
}

fn copied_images(
    solid: &Shape,
    copier: &BRepBuilderAPICopy,
    result: &OperationResult,
    bindings: &[Binding],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<SpatialPrismImage>, Failure> {
    let mut members = IndexedMapOfShape::default();
    topexp::map_shapes(solid, &mut members, true, true);
    let mut visited = IndexedMapOfShape::default();
    let mut images = Vec::with_capacity(bindings.len());
    for binding in bindings {
        poll(cancelled, Stage::Export)?;
        let original = original_shape(result, binding.result)?;
        let shape = native(copier.modified_shape(original), Stage::Export)?;
        if shape.is_same(original) || !members.contains(&shape) || visited.contains(&shape) {
            return Err(Failure::History(
                "one independent copied member for each original image",
            ));
        }
        visited.add(shape.clone());
        images.push(SpatialPrismImage {
            history: binding.history,
            shape,
        });
    }
    Ok(images)
}

fn original_shape(result: &OperationResult, image: ResultEntity) -> Result<&Shape, Failure> {
    let shape = match image {
        ResultEntity::Face(index) => result.faces().get(index).map(AsRef::as_ref),
        ResultEntity::Edge(index) => result.edges().get(index).map(AsRef::as_ref),
        ResultEntity::Vertex(index) => result.vertices().get(index).map(AsRef::as_ref),
    };
    shape.ok_or(Failure::History(
        "a native member for every indexed prism image",
    ))
}
