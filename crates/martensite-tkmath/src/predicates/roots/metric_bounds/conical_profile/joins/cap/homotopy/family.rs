// SPDX-License-Identifier: GPL-2.0-only
//! All selected contours share the same clearance and periodic image relation.
use super::*;

/// An atlas member or the global cap relation could not be bound.
#[derive(Debug)]
pub struct ConicalCapFamilyHomotopyError {
    /// Index in the supplied member order when an individual member fails.
    pub member: Option<usize>,
    /// The failed join or clearance obligation.
    pub cause: ConicalCapHomotopyFailure,
}
impl std::fmt::Display for ConicalCapFamilyHomotopyError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(out, "cap family member={:?}, {}", self.member, self.cause)
    }
}
impl std::error::Error for ConicalCapFamilyHomotopyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}

/// Bind both clearance relations against every member in contour order, then certify the
/// simultaneous interpolation. The composing operation checks filled material at an endpoint.
/// Call with `certify_conical_cap_family_homotopy(&joins, &within, &periodic)?`.
pub fn certify_conical_cap_family_homotopy<'a>(
    joins: &[&'a ConicalJoinsEvidence],
    clearance: &CertifiedCylindricalClearance,
    periodic: &CertifiedCylindricalPeriodicClearance,
) -> Result<Vec<ConicalCapHomotopyEvidence<'a>>, ConicalCapFamilyHomotopyError> {
    let members = bind_members(joins)?;
    let margins =
        certify_margins(members.iter().copied(), clearance, periodic).map_err(global_failure)?;
    Ok(members
        .into_iter()
        .map(|(cap, _)| ConicalCapHomotopyEvidence {
            cap,
            margins: margins.clone(),
        })
        .collect())
}

fn bind_members<'a>(
    joins: &[&'a ConicalJoinsEvidence],
) -> Result<Vec<CapMember<'a>>, ConicalCapFamilyHomotopyError> {
    if joins.is_empty() {
        return Err(global_failure(ConicalCapHomotopyFailure::Joins));
    }
    let mut members: Vec<CapMember<'a>> = Vec::with_capacity(joins.len());
    for (index, joins) in joins.iter().enumerate() {
        let member = cap_member(joins).map_err(|cause| member_failure(index, cause))?;
        if members
            .first()
            .is_some_and(|(first, _)| !same_datum(first, member.0))
        {
            return Err(member_failure(
                index,
                ConicalCapHomotopyFailure::Binding(ConicalCapClearanceRelation::Nonadjacent),
            ));
        }
        members.push(member);
    }
    Ok(members)
}

fn same_datum(first: &CapBinding, member: &CapBinding) -> bool {
    first.reference == member.reference && first.support == member.support
}

fn member_failure(
    member: usize,
    cause: ConicalCapHomotopyFailure,
) -> ConicalCapFamilyHomotopyError {
    ConicalCapFamilyHomotopyError {
        member: Some(member),
        cause,
    }
}

fn global_failure(cause: ConicalCapHomotopyFailure) -> ConicalCapFamilyHomotopyError {
    ConicalCapFamilyHomotopyError {
        member: None,
        cause,
    }
}
