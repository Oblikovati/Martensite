//! Typed family errors map numeric labels back to the original authored owners.
use super::*;
pub(super) fn math(
    inventory: &prepare::directed::Inventory,
    cause: ConicalActualPatchError,
) -> ConeFailure {
    let (region, span) = inventory.owner(&cause);
    ConeFailure::new(
        region,
        span,
        WrappedFailureClass::Incomplete,
        inventory.math_cause(cause),
    )
}
pub(super) fn binding(
    inventory: &prepare::directed::Inventory,
    cause: martensite_tkbo::body_check::ConicalBindingError,
) -> ConeFailure {
    let owner = cause.side.and_then(|id| {
        inventory
            .sides
            .iter()
            .find(|(_, candidate)| **candidate == id)
            .map(|(owner, _)| *owner)
    });
    let region = owner
        .map(|(region, _)| region)
        .or_else(|| cause.member.map(|id| SurfaceRegionId(id.0)));
    ConeFailure::new(
        region,
        owner.map(|(_, span)| span),
        WrappedFailureClass::Incomplete,
        ConeCause::ActualBinding(Box::new(cause)),
    )
}
pub(super) fn charge(
    work: &mut storage::Work,
    operations: u32,
    result: &Result<ConicalActualPatchCertificate, ConicalActualPatchError>,
) -> ConeChecked<()> {
    let owner = work
        .actual_bindings
        .keys()
        .next()
        .copied()
        .ok_or_else(|| global_missing("actual family work owner"))?;
    let charged = work.charge(owner, None, operations);
    work.actual_operations = operations;
    if result.is_err() {
        return Ok(());
    }
    charged
}
