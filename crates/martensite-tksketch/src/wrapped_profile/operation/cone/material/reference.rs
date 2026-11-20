//! The accepted exact q/L normal reference is separate from its analytic native cap proxy.
use super::super::super::stages::OwnedFamily;
use super::*;
mod inputs;
mod query;
struct Inputs<'a> {
    spans: Vec<Vec<ConicalJoinSpan<'a>>>,
    lengths: Vec<Vec<usize>>,
    sector: Vec<&'a [[f64; 2]]>,
    joins: Vec<&'a ConicalJoinsEvidence>,
    atlas: Vec<(&'a ConicalChartDomainEvidence, std::ops::Range<usize>)>,
}
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn certify(
    request: &WrappedConeNormalRequest,
    family: &OwnedFamily<Selection, Geometry>,
    work: &mut storage::Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalReferenceEvidence> {
    let inputs = inputs::capture(family)?;
    let members = inputs.members();
    let query = inputs.query(
        &members,
        family,
        request.options.spatial_error,
        work.remaining(),
    );
    let result = query.certify(cancelled);
    let region = first_region(family)?;
    let operations = result.as_ref().map_or_else(
        |cause| cause.operations,
        ConicalReferenceEvidence::operations,
    );
    let charged = work.charge(region, None, operations);
    finish(result, charged, family)
}
fn finish(
    result: Result<ConicalReferenceEvidence, ConicalReferenceError>,
    charged: ConeChecked<()>,
    family: &OwnedFamily<Selection, Geometry>,
) -> ConeChecked<ConicalReferenceEvidence> {
    match (result, charged) {
        (Err(cause), _) => Err(failure(family, cause)),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(proof), Ok(())) => Ok(proof),
    }
}
pub(super) fn validate(family: &ConeBudgeted) -> ConeChecked<()> {
    let inputs = inputs::capture(&family.family)?;
    let members = inputs.members();
    let proof = &family.budget.reference;
    let query = inputs.query(
        &members,
        &family.family,
        proof.requested_spatial_limit(),
        proof.options(),
    );
    if !query.matches(proof) {
        return Err(mismatch(
            first_region(&family.family)?,
            None,
            "executed q/L reference body source binding",
        ));
    }
    Ok(())
}
impl<'a> Inputs<'a> {
    fn members(&self) -> Vec<ConicalReferenceMember<'_>> {
        self.spans
            .iter()
            .zip(&self.lengths)
            .zip(&self.joins)
            .map(|((spans, lengths), joins)| ConicalReferenceMember {
                spans,
                contour_lengths: lengths,
                joins,
            })
            .collect()
    }
}
fn first_region(family: &OwnedFamily<Selection, Geometry>) -> ConeChecked<SurfaceRegionId> {
    family
        .selection
        .prepared
        .first()
        .map(|prepared| prepared.source.region)
        .ok_or_else(|| {
            ConeFailure::new(
                None,
                None,
                WrappedFailureClass::Incomplete,
                ConeCause::Scalar {
                    field: "reference family member count",
                    actual: 0.,
                    expected: "at least one selected source-bound material",
                },
            )
        })
}
fn failure(family: &OwnedFamily<Selection, Geometry>, cause: ConicalReferenceError) -> ConeFailure {
    let offender = cause.offender.and_then(|[member, span]| {
        family.selection.prepared.get(member).and_then(|prepared| {
            prepared
                .source
                .loops
                .iter()
                .flatten()
                .nth(span)
                .map(|piece| (prepared.source.region, piece.span))
        })
    });
    ConeFailure::new(
        offender.map(|owner| owner.0),
        offender.map(|owner| owner.1),
        WrappedFailureClass::Incomplete,
        ConeCause::Reference(Box::new(cause)),
    )
}
