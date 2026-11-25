//! Feed the same issued cap winding and decoded rows to native intersection validation.
use super::*;

pub(super) struct Member<'a> {
    member: ConicalActualMemberId,
    joins: &'a ConicalJoinsEvidence,
    sides: Vec<ConicalActualSideId>,
}
pub(super) fn capture<'a>(
    prepared: &[Prepared],
    members: &'a ActualMembers<'_>,
) -> ConeChecked<Vec<Member<'a>>> {
    prepared
        .iter()
        .map(|prepared| {
            let region = prepared.source.region;
            let owner = members
                .get(&region)
                .ok_or_else(|| missing(region, "periodic native geometry owner"))?;
            let curves = owner
                .material
                .periodic_curves
                .ok_or_else(|| missing(region, "periodic native contour source keys"))?;
            Ok(Member {
                member: ConicalActualMemberId(region.0),
                joins: owner.joins,
                sides: curves.iter().map(|(_, side)| *side).collect(),
            })
        })
        .collect()
}

pub(super) fn certify(
    query: ConicalActualPatchQuery<'_>,
    members: &[Member<'_>],
    reference: &ConicalReferenceEvidence,
    cancelled: &mut CancelPoll<'_>,
) -> Result<ConicalActualPatchCertificate, ConicalActualPatchError> {
    let members = members
        .iter()
        .map(|member| ConicalActualPeriodicMember {
            member: member.member,
            joins: member.joins,
            sides: &member.sides,
        })
        .collect::<Vec<_>>();
    let authority = ConicalActualPeriodicCaps {
        reference,
        members: &members,
    };
    certify_conical_actual_patches_with_periodic_caps(query, authority, cancelled)
}
