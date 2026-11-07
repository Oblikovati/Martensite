//! Actual histories compose inside the construction arena; refusals retain topology-free observations.
use super::boolean_outcome::{BooleanIssue, BooleanRequest, RefusalClass, kernel_issue, refused};
use martensite_tkbo::bop_algo::BOPAlgoBOP;
use martensite_tkbrep::{brep_tools::History, topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert};

/// Private local handles are never advertised as immutable published geometry.
pub(super) struct LocalRelation {
    pub parent: Shape,
    pub modified: Vec<Shape>,
    pub generated: Vec<Shape>,
    pub deleted: bool,
    pub native_supported: bool,
    binding: &'static str,
    invocation: usize,
    operand: usize,
}

pub(super) struct LocalHistory {
    native: Handle<History>,
    pub relations: Vec<LocalRelation>,
    steps: Vec<&'static str>,
}

/// Debug identities describe this process's actual objects; they are not naming keys.
#[derive(Clone, Debug)]
pub(super) struct RelationObservation {
    pub binding: &'static str,
    pub invocation: usize,
    pub operand: usize,
    pub parent: String,
    pub modified: Vec<String>,
    pub generated: Vec<String>,
    pub deleted: bool,
    pub native_supported: bool,
}

/// No Shape, native history handle or other geometry can escape through a refusal.
#[derive(Clone, Debug)]
pub(super) struct HistoryObservation {
    pub relations: Vec<RelationObservation>,
    pub steps: Vec<&'static str>,
}

impl LocalHistory {
    pub(super) fn observation(&self) -> HistoryObservation {
        HistoryObservation {
            relations: self.relations.iter().map(observe_relation).collect(),
            steps: self.steps.clone(),
        }
    }
    pub(super) fn tool_origins(&self, request: BooleanRequest) -> Vec<HistoryOrigin> {
        self.relations
            .iter()
            .filter(|relation| relation.operand == 1 && relation.invocation == request.ordinal)
            .map(origin_from_relation)
            .collect()
    }
}

pub(super) struct HistoryOrigin {
    parent: Shape,
    binding: &'static str,
    invocation: usize,
    operand: usize,
}

pub(super) fn observe(
    request: BooleanRequest,
    operation: &mut BOPAlgoBOP,
    operands: &[&Shape],
) -> Result<LocalHistory, BooleanIssue> {
    require_history(request, operation)?;
    let native = operation
        .history()
        .ok_or_else(|| missing_history(request))?;
    let mut origins = Vec::new();
    for (operand, shape) in operands.iter().enumerate() {
        for parent in parents(shape).map_err(|cause| kernel_issue(request, "History", cause))? {
            origins.push(HistoryOrigin {
                parent,
                binding: request.operands[operand],
                invocation: request.ordinal,
                operand,
            });
        }
    }
    materialize(request, native, origins, vec!["BOP"])
}

pub(super) fn compose(
    request: BooleanRequest,
    previous: LocalHistory,
    next: LocalHistory,
    extra: Vec<HistoryOrigin>,
) -> Result<LocalHistory, BooleanIssue> {
    previous
        .native
        .merge_handle(&next.native)
        .map_err(|cause| history_incomplete(request, cause))?;
    let mut origins: Vec<HistoryOrigin> = previous
        .relations
        .iter()
        .map(origin_from_relation)
        .collect();
    origins.extend(extra);
    let mut steps = previous.steps;
    steps.extend(next.steps);
    materialize(request, previous.native, origins, steps)
}

pub(super) fn observe_nurbs(
    request: BooleanRequest,
    input: &Shape,
    conversion: &mut BRepBuilderAPINurbsConvert,
) -> Result<LocalHistory, BooleanIssue> {
    let native = Handle::new(History::new());
    let mut origins = Vec::new();
    for parent in parents(input).map_err(|cause| history_incomplete(request, cause))? {
        record_nurbs_parent(&native, conversion, &parent)
            .map_err(|cause| history_incomplete(request, cause))?;
        origins.push(HistoryOrigin {
            parent,
            binding: "nurbs_input",
            invocation: request.ordinal,
            operand: 0,
        });
    }
    materialize(request, native, origins, vec!["NurbsConvert"])
}

// This observes the concrete converter's actual queries; no generic algorithm adapter is copied.
fn record_nurbs_parent(
    history: &History,
    conversion: &mut BRepBuilderAPINurbsConvert,
    parent: &Shape,
) -> Result<(), Failure> {
    if !History::is_supported_type(parent)? {
        return Ok(());
    }
    if conversion.is_deleted(parent)? {
        history.remove(parent)?;
    }
    let modified: Vec<Shape> = conversion.modified(parent)?.iter().cloned().collect();
    for image in modified {
        history.add_modified(parent, &image)?;
    }
    let generated: Vec<Shape> = conversion.generated(parent)?.iter().cloned().collect();
    for image in generated {
        history.add_generated(parent, &image)?;
    }
    Ok(())
}

fn materialize(
    request: BooleanRequest,
    native: Handle<History>,
    origins: Vec<HistoryOrigin>,
    steps: Vec<&'static str>,
) -> Result<LocalHistory, BooleanIssue> {
    let relations = origins
        .into_iter()
        .map(|origin| relation(&native, origin))
        .collect::<Result<Vec<LocalRelation>, Failure>>()
        .map_err(|cause| history_incomplete(request, cause))?;
    Ok(LocalHistory {
        native,
        relations,
        steps,
    })
}

fn parents(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut identities = IndexedMapOfShape::default();
    topexp::map_shapes(shape, &mut identities, true, true);
    (1..=identities.extent())
        .map(|index| identities.find_key(index).cloned())
        .collect()
}

fn relation(history: &History, origin: HistoryOrigin) -> Result<LocalRelation, Failure> {
    let modified = history.modified(&origin.parent)?;
    let generated = history.generated(&origin.parent)?;
    let deleted = history.is_removed(&origin.parent)?;
    let native_supported = History::is_supported_type(&origin.parent)?;
    Ok(LocalRelation {
        parent: origin.parent,
        binding: origin.binding,
        invocation: origin.invocation,
        operand: origin.operand,
        modified,
        generated,
        deleted,
        native_supported,
    })
}

fn origin_from_relation(relation: &LocalRelation) -> HistoryOrigin {
    HistoryOrigin {
        parent: relation.parent.clone(),
        binding: relation.binding,
        invocation: relation.invocation,
        operand: relation.operand,
    }
}

fn observe_relation(relation: &LocalRelation) -> RelationObservation {
    RelationObservation {
        binding: relation.binding,
        invocation: relation.invocation,
        operand: relation.operand,
        parent: format!("{:?}", relation.parent),
        modified: relation
            .modified
            .iter()
            .map(|shape| format!("{shape:?}"))
            .collect(),
        generated: relation
            .generated
            .iter()
            .map(|shape| format!("{shape:?}"))
            .collect(),
        deleted: relation.deleted,
        native_supported: relation.native_supported,
    }
}

fn missing_history(request: BooleanRequest) -> BooleanIssue {
    refused(
        request,
        "History",
        "history=None".into(),
        "actual performed history before any lazy getter",
        RefusalClass::Incomplete,
    )
}

fn require_history(request: BooleanRequest, operation: &BOPAlgoBOP) -> Result<(), BooleanIssue> {
    if operation.has_filled_history() {
        return Ok(());
    }
    Err(missing_history(request))
}

fn history_incomplete(request: BooleanRequest, cause: Failure) -> BooleanIssue {
    let mut issue = kernel_issue(request, "History", cause);
    issue.class = RefusalClass::Incomplete;
    issue
}
