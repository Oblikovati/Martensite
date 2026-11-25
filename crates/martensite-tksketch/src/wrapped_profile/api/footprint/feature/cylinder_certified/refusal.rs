// SPDX-License-Identifier: GPL-2.0-only
//! Project numeric proof refusals without losing the original error or authored incidence.
use super::*;
use crate::surface_sketch::WrappedFailureClass as Class;
use crate::surface_sketch::wrapped_profile::{
    api::{error, export_bindings, failure_projection as shared, history_projection},
    history, planar_clearance, planar_cones, storage,
};
use martensite_tkmath::predicates::{
    NonFiniteCoordinate,
    roots::metric_bounds::{
        BoundedArithmeticFailure as Arithmetic, CylindricalClearanceError,
        CylindricalClearanceFailure, CylindricalConeFailure, CylindricalContactPrefixError,
        CylindricalContactPrefixFailure, CylindricalProfileError, CylindricalProfileFailure,
        CylindricalSliceRefusal, CylindricalStorageError, CylindricalStorageFailure,
        CylindricalTrimError,
    },
};
use std::error::Error;
mod chains;
#[cfg(test)]
mod tests;

type Context = (Class, Vec<HistorySource>);

pub(super) trait ProofFailure: Error + Send + Sync + 'static {
    fn context(&self) -> Context;
}

pub(super) fn project(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    query: &'static str,
    subjects: impl IntoIterator<Item = HistorySource>,
    cause: impl ProofFailure,
    observed_cancel: bool,
) -> CylinderFeatureFailure {
    let (class, mut entities) = cause.context();
    let class = if observed_cancel {
        Class::Cancelled
    } else {
        class
    };
    entities.extend(subjects);
    entities.sort();
    entities.dedup();
    attempt.fail(
        stage,
        CylinderFeatureCause::NumericalProof {
            query,
            class,
            entities,
            cause: Box::new(cause),
        },
    )
}

pub(super) fn cardinality(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    query: &'static str,
    [actual, expected]: [usize; 2],
    entities: impl IntoIterator<Item = HistorySource>,
) -> CylinderFeatureFailure {
    let mut entities: Vec<_> = entities.into_iter().collect();
    entities.sort();
    entities.dedup();
    attempt.fail(
        stage,
        CylinderFeatureCause::Cardinality {
            query,
            actual,
            expected,
            entities,
        },
    )
}

impl ProofFailure for selection::SelectionFailure {
    fn context(&self) -> Context {
        let context = shared::selection_context(self);
        let class = match self {
            Self::Clearance(cause) => cause.context().0,
            Self::Planning { cause, .. } => profile_class(&cause.source_cause().cause),
            _ => context.class,
        };
        let mut entities = context.entities;
        if let Self::Planning { cause, .. } = self {
            entities.push(HistorySource::Curve(cause.curve()));
        }
        (class, entities)
    }
}

impl ProofFailure for storage::StorageFailure {
    fn context(&self) -> Context {
        let current = self.public_class();
        let class = if self.is_cancelled() {
            Class::Cancelled
        } else if let Some(cause) = self.source() {
            storage_source_class(cause).unwrap_or(current)
        } else {
            current
        };
        (class, vec![shared::binding_source(self.key)])
    }
}

fn storage_source_class(cause: &(dyn Error + 'static)) -> Option<Class> {
    if let Some(cause) = cause.downcast_ref::<CylindricalStorageError>() {
        return Some(storage_class(&cause.cause));
    }
    if let Some(cause) = cause.downcast_ref::<CylindricalProfileError>() {
        return Some(profile_class(&cause.cause));
    }
    if let Some(cause) = cause.downcast_ref::<CylindricalTrimError>() {
        return Some(profile_class(&cause.cause));
    }
    cause
        .downcast_ref::<storage::StorageFailure>()
        .map(|cause| cause.context().0)
}

impl ProofFailure for planar_cones::ConeFailure {
    fn context(&self) -> Context {
        use CylindricalConeFailure as Proof;
        use planar_cones::ConeFailure as C;
        let class = match self {
            C::Authored(cause) => cause.context().0,
            C::Interval { cause, .. } => profile_class(cause),
            C::Certificate { cause, .. } => match &cause.cause {
                Proof::Policy(_) | Proof::SourceCount(_) => Class::InvalidInput,
                Proof::Arithmetic(cause) => arithmetic_class(cause),
                _ => Class::Incomplete,
            },
            _ => Class::Incomplete,
        };
        (class, shared::cone_entities(self))
    }
}

impl ProofFailure for planar_clearance::ClearanceFailure {
    fn context(&self) -> Context {
        use planar_clearance::ClearanceFailure as C;
        let class = match self {
            C::Authored(cause) => cause.context().0,
            C::Sources(cause) => cause.context().0,
            C::Certificate { cause, .. } => clearance_class(cause),
            C::EmptySelection | C::DuplicateSource(_) => Class::InvalidInput,
            _ => Class::Incomplete,
        };
        (class, shared::clearance_context(self).entities)
    }
}

fn clearance_class(mut cause: &CylindricalClearanceError) -> Class {
    use CylindricalClearanceFailure as C;
    loop {
        match &cause.cause {
            C::PeriodicImage { cause: nested, .. } => cause = nested,
            C::Arithmetic(cause) => return arithmetic_class(cause),
            C::Policy(_)
            | C::InputCount { .. }
            | C::Value { .. }
            | C::SourceShape { .. }
            | C::PeriodicImagePolicy { .. } => return Class::InvalidInput,
            _ => return Class::Incomplete,
        }
    }
}

impl ProofFailure for history::HistoryFailure {
    fn context(&self) -> Context {
        let context = shared::history_context(self);
        (context.class, context.entities)
    }
}

impl ProofFailure for history_projection::HistoryMapFailure {
    fn context(&self) -> Context {
        let mut entities = Vec::new();
        error::history_projection_entities(self, &mut entities);
        (Class::Incomplete, entities)
    }
}

impl ProofFailure for export_bindings::ExportBindingFailure {
    fn context(&self) -> Context {
        use export_bindings::ExportBindingFailure as F;
        let entity = match self {
            F::DuplicateBinding(key) | F::MissingBinding(key) => Some(shared::binding_source(*key)),
            F::DuplicateName(key) => Some(history_entity(*key)),
            F::Count { .. } => None,
        };
        (Class::Incomplete, entity.into_iter().collect())
    }
}

impl ProofFailure for NonFiniteCoordinate {
    fn context(&self) -> Context {
        (Class::InvalidInput, Vec::new())
    }
}

impl ProofFailure for CylindricalTrimError {
    fn context(&self) -> Context {
        (profile_class(&self.cause), Vec::new())
    }
}

impl ProofFailure for CylindricalContactPrefixError {
    fn context(&self) -> Context {
        use CylindricalContactPrefixFailure as C;
        let class = match &self.cause {
            C::Admission(cause) => storage_class(cause),
            C::Arithmetic(cause) => arithmetic_class(cause),
            C::Radius(_) | C::ContactAllowance(_) | C::MaximumDepth(_) => Class::InvalidInput,
            _ => Class::Incomplete,
        };
        (class, Vec::new())
    }
}

impl ProofFailure for CylindricalSliceRefusal {
    fn context(&self) -> Context {
        use CylindricalSliceRefusal as C;
        let class = match self {
            C::Admission(cause) => storage_class(cause),
            C::Arithmetic(cause) => arithmetic_class(cause),
            C::Chart { .. } | C::RailCount(_) | C::RailControl { .. } => Class::InvalidInput,
            _ => Class::Incomplete,
        };
        (class, Vec::new())
    }
}

fn arithmetic_class(cause: &Arithmetic) -> Class {
    match cause {
        Arithmetic::Cancelled => Class::Cancelled,
        Arithmetic::NonFiniteScalar { .. } => Class::InvalidInput,
        _ => Class::Incomplete,
    }
}

fn profile_class(cause: &CylindricalProfileFailure) -> Class {
    use CylindricalProfileFailure as C;
    match cause {
        C::Policy(_)
        | C::FitPolicy { .. }
        | C::Interval { .. }
        | C::Count { .. }
        | C::Value { .. }
        | C::Weights { .. } => Class::InvalidInput,
        C::Arithmetic(cause) => arithmetic_class(cause),
        _ => Class::Incomplete,
    }
}

fn storage_class(cause: &CylindricalStorageFailure) -> Class {
    use CylindricalStorageFailure as C;
    match cause {
        C::Cancelled => Class::Cancelled,
        C::Policy(_) | C::Value { .. } | C::PoleCount { .. } | C::Degree { .. } => {
            Class::InvalidInput
        }
        _ => Class::Incomplete,
    }
}
