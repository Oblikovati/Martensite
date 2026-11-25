//! Cone projection reuses typed decline/entities and separates admission from numerical work.
use super::*;
use crate::surface_sketch::wrapped_profile::api::failure_projection as shared;
use WrappedFailureClass as Class;
mod directed;
mod integrals;
mod numerical;
pub(super) fn context(failure: &ConeFailure) -> (Class, Vec<HistorySource>) {
    if let ActualPair { cause, entities } = &failure.cause {
        return (directed::actual(&cause.cause), entities.clone());
    }
    if let Some(context) = shared_context(failure) {
        return (context.class, context.entities);
    }
    let class = admission_class(&failure.cause).or_else(|| mathematical_class(&failure.cause));
    (class.unwrap_or(failure.class), vec![])
}
fn shared_context(failure: &ConeFailure) -> Option<shared::Context> {
    match &failure.cause {
        Budget(cause) => Some(budget_context(cause)),
        Planar(cause) => Some(shared::planar_context(cause, failure.region)),
        NativeQualification(cause) => Some(shared::member_context(cause)),
        Coverage(cause) => Some(shared::coverage_context(cause)),
        History(cause) => Some(shared::history_context(cause)),
        Support(_)
        | ProofOptions(_)
        | InputBoundProofOptions { .. }
        | ExecutionProof(_)
        | Depths(_)
        | SourceAnchor { .. } => None,
        Scalar { .. } | Limit { .. } | Missing(_) | Native(_) | Encoding(_) => None,
        Storage(_) | Candidate(_) | Coefficients(_) | Boundary(_) => None,
        Radius(_) | Sector(_) | Domain(_) | Joins(_) | ChartJoins(_) | TrimPair(_)
        | ChartTrimPair(_) | Material(_) | DevelopedMaterial(_) => None,
        Reference(_) | ReferenceVolume(_) | Flux(_) | Clearance(_) => None,
        Authored(_) | Cancelled(_) | Binding { .. } | ActualBinding(_) | ActualValidation(_) => {
            None
        }
        ActualPatch(_) | ActualPair { .. } | DirectedRows(_) => None,
        ChartPlan(_) => None,
        DirectedCap(_) => None,
        Atlas(cause) => match cause.as_ref() {
            material::atlas::Refusal::Material(cause) => {
                let mut context = shared::material_context(cause);
                // Projected witness keys identify decoded spans, not authored curves.
                context
                    .entities
                    .retain(|source| matches!(source, HistorySource::Region(_)));
                Some(context)
            }
            material::atlas::Refusal::Reference {
                overlap: Some(owners),
                ..
            } => {
                let mut entities = owners.map(HistorySource::Region).to_vec();
                entities.sort();
                entities.dedup();
                Some(shared::Context {
                    stage: failure.stage,
                    class: Class::InvalidInput,
                    entities,
                })
            }
            _ => None,
        },
    }
}
fn admission_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        Support(_) | ExecutionProof(_) => Some(Class::Unsupported),
        ProofOptions(_) | InputBoundProofOptions { .. } | Depths(_) => Some(Class::InvalidInput),
        Native(_) | Encoding(_) => Some(Class::Native),
        ActualBinding(_) | ActualValidation(_) => Some(Class::Incomplete),
        Storage(cause) => Some(cause.public_class()),
        Cancelled(_) => Some(Class::Cancelled),
        DirectedCap(failure) => {
            use martensite_tkbo::body_check::DirectedCapCause;
            Some(match failure.cause.as_ref() {
                DirectedCapCause::Policy { .. }
                | DirectedCapCause::Shape { .. }
                | DirectedCapCause::Profile(_) => Class::InvalidInput,
                _ => Class::Incomplete,
            })
        }
        SourceAnchor { .. } | Missing(_) => Some(Class::Incomplete),
        Scalar { .. } | Limit { .. } | Binding { .. } => None,
        Budget(_) | Planar(_) | NativeQualification(_) | Coverage(_) | History(_) => None,
        Candidate(_) | Coefficients(_) | Boundary(_) | Radius(_) => None,
        Sector(_) | Domain(_) | Joins(_) | ChartJoins(_) | TrimPair(_) | ChartTrimPair(_)
        | Material(_) | DevelopedMaterial(_) => None,
        Reference(_) | ReferenceVolume(_) | Flux(_) | Clearance(_) | Authored(_) => None,
        ActualPatch(_) | ActualPair { .. } | DirectedRows(_) => None,
        ChartPlan(_) | Atlas(_) => None,
    }
}
fn mathematical_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        ActualPatch(_) | ActualPair { .. } | DirectedRows(_) => actual_class(cause),
        Candidate(cause) => Some(numerical::candidate(cause)),
        Coefficients(cause) => Some(numerical::coefficients(cause)),
        Boundary(cause) => Some(numerical::boundary(cause)),
        Radius(cause) => Some(numerical::depth(cause.cause())),
        Sector(cause) => Some(numerical::sector(cause)),
        Domain(cause) => Some(numerical::domain(&cause.cause)),
        ChartPlan(cause) => Some(chart_plan_class(&cause.cause)),
        Atlas(_) => None,
        Joins(_) | ChartJoins(_) => join_class(cause),
        TrimPair(_) | ChartTrimPair(_) => trim_class(cause),
        Material(_) | DevelopedMaterial(_) => material_class(cause),
        Reference(cause) => Some(numerical::reference(&cause.cause)),
        ReferenceVolume(_) | Flux(_) | Clearance(_) | Authored(_) => integral_class(cause),
        Support(_)
        | ProofOptions(_)
        | InputBoundProofOptions { .. }
        | ExecutionProof(_)
        | Depths(_)
        | SourceAnchor { .. } => None,
        Scalar { .. } | Limit { .. } | Missing(_) | Native(_) | Encoding(_) => None,
        NativeQualification(_) | Storage(_) | Coverage(_) | History(_) => None,
        ActualBinding(_) | ActualValidation(_) => None,
        Budget(_) | Planar(_) | Cancelled(_) | Binding { .. } => None,
        DirectedCap(_) => None,
    }
}

fn chart_plan_class(cause: &ConicalChartPlanFailure) -> Class {
    match cause {
        ConicalChartPlanFailure::Domain(cause) => numerical::domain(cause),
        ConicalChartPlanFailure::Limits { .. } => Class::InvalidInput,
        ConicalChartPlanFailure::Level { .. } | ConicalChartPlanFailure::Capacity { .. } => {
            Class::Incomplete
        }
    }
}

fn budget_context(cause: &publication_budget::BudgetFailure) -> shared::Context {
    let (stage, class, entities) = shared::budget(cause);
    shared::Context {
        stage,
        class,
        entities,
    }
}

fn join_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        Joins(cause) => Some(numerical::joins(cause)),
        ChartJoins(cause) => Some(numerical::joins(&cause.cause)),
        _ => None,
    }
}

fn trim_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        TrimPair(cause) => Some(numerical::profile(&cause.cause.cause)),
        ChartTrimPair(cause) => Some(chart_trim_class(&cause.cause)),
        _ => None,
    }
}
fn chart_trim_class(cause: &ConicalChartTrimPairFailure) -> Class {
    match cause {
        ConicalChartTrimPairFailure::Binding { .. } => Class::Incomplete,
        ConicalChartTrimPairFailure::Trace(cause) => numerical::profile(&cause.cause),
    }
}

fn material_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        Material(cause) => Some(numerical::profile(&cause.cause)),
        DevelopedMaterial(cause) => Some(match cause.cause.as_ref() {
            ConicalDevelopedMaterialFailure::Binding { .. } => Class::Incomplete,
            ConicalDevelopedMaterialFailure::Numerical(cause) => numerical::profile(cause),
        }),
        _ => None,
    }
}

fn integral_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        ReferenceVolume(cause) => Some(integrals::reference(&cause.cause)),
        Flux(cause) => Some(integrals::flux(&cause.cause)),
        Clearance(cause) => Some(integrals::clearance(&cause.cause)),
        Authored(cause) => Some(integrals::authored(cause)),
        _ => None,
    }
}

fn actual_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        ActualPatch(cause) | ActualPair { cause, .. } => Some(directed::actual(&cause.cause)),
        DirectedRows(cause) => Some(directed::rows(&cause.cause)),
        _ => None,
    }
}
