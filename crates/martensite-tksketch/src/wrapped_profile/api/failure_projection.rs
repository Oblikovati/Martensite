//! Project typed private causes at the public boundary without parsing diagnostics (#448).
use super::*;
use crate::surface_sketch::HistorySource;
use WrappedFailureClass as Class;
use martensite_tkbo::profile_extrusion::DeclineClass;
struct Context {
    stage: WrappedStage,
    class: Class,
    entities: Vec<HistorySource>,
}
impl Context {
    fn new(class: Class, entities: Vec<HistorySource>) -> Self {
        Self {
            stage: WrappedStage::Construction,
            class,
            entities,
        }
    }
    fn owner(class: Class, region: SurfaceRegionId) -> Self {
        Self::new(class, vec![HistorySource::Region(region)])
    }
    fn optional(class: Class, region: Option<SurfaceRegionId>) -> Self {
        Self::new(
            class,
            region.into_iter().map(HistorySource::Region).collect(),
        )
    }
    fn admission(mut self) -> Self {
        self.stage = WrappedStage::Admission;
        self
    }
}
pub(super) fn family(cause: family::FamilyFailure, selected: &[Request]) -> WrappedNormalFailure {
    let mut context = family_context(&cause);
    add_owners(&mut context.entities, selected.iter().map(|r| r.region));
    error::with_entities(
        context.stage,
        context.class,
        context.entities,
        error::Cause::Family(cause),
    )
}
pub(super) fn selected(
    mut failure: WrappedNormalFailure,
    request: &WrappedNormalRequest,
) -> WrappedNormalFailure {
    add_owners(
        &mut failure.entities,
        request.selections.iter().map(|r| r.region),
    );
    failure.entities.sort();
    failure.entities.dedup();
    failure
}
fn add_owners(entities: &mut Vec<HistorySource>, owners: impl Iterator<Item = SurfaceRegionId>) {
    if entities
        .iter()
        .any(|entity| matches!(entity, HistorySource::Region(_)))
    {
        return;
    }
    entities.extend(owners.map(HistorySource::Region));
}
fn family_context(cause: &family::FamilyFailure) -> Context {
    use family::FamilyFailure as F;
    match cause {
        F::Selection(cause) => selection_context(cause),
        F::Budget(cause) => budget_context(cause),
        F::Member(cause) => member_context(cause),
        F::Cancelled(point) => Context::optional(Class::Cancelled, point.region),
        F::Native { region, .. } => Context::optional(Class::Native, *region),
        F::Encoding(_) => Context::new(Class::Native, vec![]),
        F::Coverage(cause) => coverage_context(cause),
        F::Binding(cause) => clearance_context(cause),
        F::History(cause) => history_context(cause),
    }
}
fn member_context(cause: &Refusal) -> Context {
    let mut context = match &cause.cause {
        RefusalCause::Native(_) => Context::new(Class::Native, vec![]),
        RefusalCause::Coverage(cause) => coverage_context(cause),
        RefusalCause::Cones(cones) => cone_context(cause.region, cones),
        _ => Context::new(Class::Incomplete, vec![]),
    };
    context.entities.push(HistorySource::Region(cause.region));
    context
}
fn selection_context(cause: &selection::SelectionFailure) -> Context {
    use selection::SelectionFailure as S;
    match cause {
        S::Empty | S::Limit { .. } => Context::new(Class::InvalidInput, vec![]).admission(),
        S::Cancelled => Context::new(Class::Cancelled, vec![]),
        S::Admission { region, .. } | S::Context { region, .. } | S::DuplicateRegion(region) => {
            Context::owner(Class::InvalidInput, *region).admission()
        }
        S::DuplicateCurve { curve, owners } => {
            relation_context(HistorySource::Curve(*curve), owners).admission()
        }
        S::DuplicateVertex { vertex, owners } => vertex_context(*vertex, owners),
        S::Region { region, cause } => planar_context(cause, Some(*region)),
        S::Capture(cause) => planar_context(cause, None),
        S::Planning { region, .. } => Context::owner(Class::Incomplete, *region),
        S::Material(cause) => material_context(cause),
        S::Clearance(cause) => clearance_context(cause),
    }
}
fn vertex_context(vertex: CornerSource, owners: &[SurfaceRegionId; 2]) -> Context {
    relation_context(corner_source(vertex), owners).admission()
}

fn relation_context(entity: HistorySource, owners: &[SurfaceRegionId; 2]) -> Context {
    let mut entities = owners
        .iter()
        .copied()
        .map(HistorySource::Region)
        .collect::<Vec<_>>();
    entities.push(entity);
    Context::new(Class::InvalidInput, entities)
}
fn planar_context(cause: &planar::PlanarFailure, region: Option<SurfaceRegionId>) -> Context {
    let mut context = Context::optional(planar_class(cause), region);
    match cause {
        planar::PlanarFailure::SplitSource { curve, .. } => {
            context.entities.push(HistorySource::Curve(*curve))
        }
        planar::PlanarFailure::Capture(cause) => {
            error::decline_entities(&cause.cause, &mut context.entities)
        }
        _ => {}
    }
    context
}
fn planar_class(cause: &planar::PlanarFailure) -> Class {
    match cause {
        planar::PlanarFailure::Capture(cause) => decline_class(cause.class()),
        planar::PlanarFailure::EmptyMaterial
        | planar::PlanarFailure::Selection { .. }
        | planar::PlanarFailure::Join(_) => Class::InvalidInput,
        _ => Class::Incomplete,
    }
}

fn decline_class(class: DeclineClass) -> Class {
    match class {
        DeclineClass::InvalidInput | DeclineClass::InvalidGeometry => Class::InvalidInput,
        DeclineClass::Unsupported => Class::Unsupported,
        DeclineClass::Native => Class::Native,
        DeclineClass::Incomplete | DeclineClass::OverBudget => Class::Incomplete,
    }
}
fn budget_context(cause: &publication_budget::BudgetFailure) -> Context {
    use publication_budget::BudgetFailure as B;
    match cause {
        B::Empty | B::Policy { .. } => Context::new(Class::InvalidInput, vec![]).admission(),
        _ => budget_sources(cause),
    }
}
fn budget_sources(cause: &publication_budget::BudgetFailure) -> Context {
    use publication_budget::BudgetFailure as B;
    match cause {
        B::Owners { actual, expected } => {
            Context::new(Class::Incomplete, region_entities(actual, expected))
        }
        B::Spans {
            region,
            actual,
            expected,
        } => span_context(*region, actual.iter().chain(expected)),
        B::Source(region) | B::Coverage { region, .. } => {
            Context::owner(Class::Incomplete, *region)
        }
        B::Cones { region, cause } => cone_context(*region, cause),
        B::Native { region, .. } => Context::owner(Class::Native, *region),
        B::Limit { region, span, .. } => limit_context(*region, *span),
        _ => Context::new(Class::Incomplete, vec![]),
    }
}
fn region_entities(actual: &[SurfaceRegionId], expected: &[SurfaceRegionId]) -> Vec<HistorySource> {
    actual
        .iter()
        .chain(expected)
        .copied()
        .map(HistorySource::Region)
        .collect()
}
fn span_entities<'a>(spans: impl Iterator<Item = &'a CurveSpan>) -> Vec<HistorySource> {
    spans.map(|span| HistorySource::Curve(span.curve)).collect()
}

fn span_context<'a>(
    region: SurfaceRegionId,
    spans: impl Iterator<Item = &'a CurveSpan>,
) -> Context {
    let mut context = Context::owner(Class::Incomplete, region);
    context
        .entities
        .extend(spans.map(|span| HistorySource::Curve(span.curve)));
    context
}
fn limit_context(region: Option<SurfaceRegionId>, span: Option<CurveSpan>) -> Context {
    let mut entities = region
        .into_iter()
        .map(HistorySource::Region)
        .collect::<Vec<_>>();
    entities.extend(span.map(|span| HistorySource::Curve(span.curve)));
    Context::new(Class::Incomplete, entities)
}
fn material_context(cause: &selection::MaterialFailure) -> Context {
    use selection::MaterialFailure as M;
    match cause {
        M::Admission(cause) => selection_context(cause),
        M::Cancelled => Context::new(Class::Cancelled, vec![]),
        M::Policy(_) => Context::new(Class::InvalidInput, vec![]).admission(),
        M::Source(region) => Context::owner(Class::Incomplete, *region),
        M::Clearance(cause) => clearance_context(cause),
        _ => material_boundary(cause),
    }
}
fn material_boundary(cause: &selection::MaterialFailure) -> Context {
    use selection::MaterialFailure as M;
    match cause {
        M::Winding {
            region, contour, ..
        } => contour_context(Class::Incomplete, *region, *contour),
        M::NonJordan {
            region, contour, ..
        } => contour_context(Class::InvalidInput, *region, *contour),
        M::Overlap {
            source,
            target,
            witness,
            ..
        } => relation_context(HistorySource::Curve(*witness), &[*source, *target]),
        _ => Context::new(Class::Incomplete, vec![]),
    }
}

fn contour_context(class: Class, region: SurfaceRegionId, contour: SurfaceCurveId) -> Context {
    Context::new(
        class,
        vec![HistorySource::Region(region), HistorySource::Curve(contour)],
    )
}
fn clearance_context(cause: &planar_clearance::ClearanceFailure) -> Context {
    use planar_clearance::ClearanceFailure as C;
    let entities = match cause {
        C::SelectionBinding { actual, expected } => region_entities(actual, expected),
        C::DuplicateSource(curve) => vec![HistorySource::Curve(*curve)],
        C::Certificate { spans, .. } => span_entities(spans.iter()),
        C::Binding { expected, actual } => span_entities(expected.iter().chain(actual)),
        _ => vec![],
    };
    Context::new(Class::Incomplete, entities)
}
fn coverage_context(cause: &coverage::CoverageFailure) -> Context {
    let class = if matches!(cause.cause, coverage::CoverageCause::Native(_)) {
        Class::Native
    } else {
        Class::Incomplete
    };
    let mut entities = match cause.owner {
        coverage::CoverageOwner::Binding(key) => vec![binding_source(key)],
        coverage::CoverageOwner::Family => vec![],
    };
    coverage_targets(&cause.cause, &mut entities);
    Context::new(class, entities)
}
fn coverage_targets(cause: &coverage::CoverageCause, entities: &mut Vec<HistorySource>) {
    use coverage::CoverageCause as C;
    match cause {
        C::MissingBinding(key) | C::DuplicateBinding { key, .. } => {
            entities.push(binding_source(*key))
        }
        C::AliasedBindings { first, second } => {
            entities.extend([binding_source(*first), binding_source(*second)])
        }
        _ => {}
    }
}

fn history_context(cause: &history::HistoryFailure) -> Context {
    use history::HistoryFailure as H;
    let entities = match cause {
        H::DuplicateTarget(key) => vec![binding_source(*key)],
        H::Corners { span, .. } => vec![HistorySource::Curve(span.curve)],
        H::Targets { missing, extra } => missing
            .iter()
            .chain(extra)
            .map(|key| binding_source(*key))
            .collect(),
        H::Clearance(cause) => return clearance_context(cause),
        H::SourceBinding => vec![],
    };
    Context::new(Class::Incomplete, entities)
}
pub(super) fn binding_source(key: BindingKey) -> HistorySource {
    match key.source {
        Source::Region(region) => HistorySource::Region(region),
        Source::Span(span) => HistorySource::Curve(span.curve),
        Source::Corner(corner) => corner_source(corner),
    }
}
fn corner_source(corner: CornerSource) -> HistorySource {
    match corner {
        CornerSource::Authored(vertex) => HistorySource::Vertex(vertex),
        CornerSource::Split { curve, .. } => HistorySource::Curve(curve),
    }
}
fn cone_context(region: SurfaceRegionId, cause: &planar_cones::ConeFailure) -> Context {
    use planar_cones::ConeFailure as C;
    let mut context = Context::owner(Class::Incomplete, region);
    match cause {
        C::Membership { span, .. } | C::Interval { span, .. } => {
            context.entities.push(HistorySource::Curve(span.curve))
        }
        C::Binding { spans, .. } | C::Certificate { spans, .. } => {
            context.entities.extend(span_entities(spans.iter()))
        }
        C::CapJoin { spans, owners, .. } => {
            context.entities.extend(span_entities(spans.iter()));
            context
                .entities
                .extend(owners.iter().copied().map(corner_source));
        }
        C::Authored(_) => {}
    }
    context
}
