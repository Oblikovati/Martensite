//! One stage order for source-bound cylinder and cone operations.
use super::{stages::*, *};
use family::{Control, FamilyStage};
type Family<S> = OwnedFamily<<S as SupportStages>::Selection, <S as SupportStages>::Geometry>;
type BudgetedSupport<S> = Budgeted<
    <S as SupportStages>::Selection,
    <S as SupportStages>::Geometry,
    <S as SupportStages>::Budget,
>;
pub(super) fn budgeted<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    cancelled: &mut (dyn FnMut(family::Checkpoint) -> bool + Send),
) -> Result<BudgetedSupport<S>, S::Error> {
    let limits = support.limits(originals)?;
    let mut control = Control::new(cancelled);
    let mut context = support.context();
    let family = construct_with_context(support, originals, &mut control, &mut context)?;
    let budget = control.run_for::<_, S::Error>(FamilyStage::Budget, None, |poll| {
        support.budget(originals, &family, limits, &mut context, poll)
    })?;
    control.run_for::<_, S::Error>(FamilyStage::Completion, None, |_| {
        Ok(Budgeted { family, budget })
    })
}
pub(super) fn construct_with_context<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    control: &mut Control<'_>,
    context: &mut S::Context,
) -> Result<Family<S>, S::Error> {
    let selection = control.run_for::<_, S::Error>(FamilyStage::Selection, None, |poll| {
        support.select(originals, context, poll)
    })?;
    let frozen = super::frozen::construct(support, S::prepared(&selection), control)?;
    qualify(support, originals, selection, frozen, context, control)
}
fn qualify<S: SupportStages>(
    support: &S,
    originals: &[S::Original],
    selection: S::Selection,
    frozen: family::freeze::Frozen,
    context: &mut S::Context,
    control: &mut Control<'_>,
) -> Result<Family<S>, S::Error> {
    let (bytes, keys, root, bindings) = frozen;
    let (coverage, views) = control.run_for::<_, S::Error>(FamilyStage::Coverage, None, |_| {
        coverage::certify_developed_family(S::prepared(&selection), &root, &bindings)
            .map_err(S::coverage)
    })?;
    let history = control.run_for::<_, S::Error>(FamilyStage::History, None, |_| {
        support.history(originals, &selection, &keys)
    })?;
    let members = super::members::qualify(support, originals, &selection, views, context, control)?;
    Ok(OwnedFamily {
        bytes,
        keys,
        members,
        selection,
        coverage,
        history,
    })
}
