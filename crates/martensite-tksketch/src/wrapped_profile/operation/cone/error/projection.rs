//! Cone projection reuses typed decline/entities and separates admission from numerical work.
use super::*;
use crate::surface_sketch::wrapped_profile::api::failure_projection as shared;
use WrappedFailureClass as Class;
mod integrals;
mod numerical;
pub(super) fn context(failure: &ConeFailure) -> (Class, Vec<HistorySource>) {
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
        Support(_) | ProofOptions(_) | Depths(_) | SourceAnchor { .. } => None,
        Scalar { .. } | Limit { .. } | Missing(_) | Native(_) | Encoding(_) => None,
        Storage(_) | Candidate(_) | Coefficients(_) | Boundary(_) => None,
        Radius(_) | Sector(_) | Joins(_) | TrimPair(_) | Material(_) => None,
        Reference(_) | ReferenceVolume(_) | Flux(_) | Clearance(_) => None,
        Authored(_) | Cancelled(_) | Binding { .. } => None,
    }
}
fn admission_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        Support(_) => Some(Class::Unsupported),
        ProofOptions(_) | Depths(_) => Some(Class::InvalidInput),
        Native(_) | Encoding(_) => Some(Class::Native),
        Storage(cause) => Some(cause.public_class()),
        Cancelled(_) => Some(Class::Cancelled),
        SourceAnchor { .. } | Missing(_) => Some(Class::Incomplete),
        Scalar { .. } | Limit { .. } | Binding { .. } => None,
        Budget(_) | Planar(_) | NativeQualification(_) | Coverage(_) | History(_) => None,
        Candidate(_) | Coefficients(_) | Boundary(_) | Radius(_) => None,
        Sector(_) | Joins(_) | TrimPair(_) | Material(_) => None,
        Reference(_) | ReferenceVolume(_) | Flux(_) | Clearance(_) | Authored(_) => None,
    }
}
fn mathematical_class(cause: &ConeCause) -> Option<Class> {
    match cause {
        Candidate(cause) => Some(numerical::candidate(cause)),
        Coefficients(cause) => Some(numerical::coefficients(cause)),
        Boundary(cause) => Some(numerical::boundary(cause)),
        Radius(cause) => Some(numerical::depth(cause.cause())),
        Sector(cause) => Some(numerical::sector(cause)),
        Joins(cause) => Some(numerical::joins(cause)),
        TrimPair(cause) => Some(numerical::profile(&cause.cause.cause)),
        Material(cause) => Some(numerical::profile(&cause.cause)),
        Reference(cause) => Some(numerical::reference(&cause.cause)),
        ReferenceVolume(cause) => Some(integrals::reference(&cause.cause)),
        Flux(cause) => Some(integrals::flux(&cause.cause)),
        Clearance(cause) => Some(integrals::clearance(&cause.cause)),
        Authored(cause) => Some(integrals::authored(cause)),
        Support(_) | ProofOptions(_) | Depths(_) | SourceAnchor { .. } => None,
        Scalar { .. } | Limit { .. } | Missing(_) | Native(_) | Encoding(_) => None,
        NativeQualification(_) | Storage(_) | Coverage(_) | History(_) => None,
        Budget(_) | Planar(_) | Cancelled(_) | Binding { .. } => None,
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
