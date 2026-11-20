// SPDX-License-Identifier: GPL-2.0-only
//! A numeric certificate is issued only after independent complete native rebinding.
use super::*;
use std::{collections::BTreeSet, sync::Arc};
mod capture;
mod matching;
mod snapshot;

/// Complete mathematical family authority with owned actual native snapshots.
/// No native handle, digest-only match or caller validity flag is retained.
pub struct CertifiedConicalBody {
    certificate: Arc<ConicalActualPatchCertificate>,
    census: ConicalNativeCensus,
    members: Vec<(ConicalActualMemberId, Vec<u64>)>,
}
/// Bind the complete issued numeric proof to independently captured actual geometry.
/// Example: `let body = bind_conical_actual_family(&native, &query, proof)?;`.
pub fn bind_conical_actual_family(
    actual: &ConicalNativeGeometry,
    query: &ConicalActualPatchQuery<'_>,
    certificate: Arc<ConicalActualPatchCertificate>,
) -> Result<CertifiedConicalBody, ConicalBindingError> {
    if !certificate.matches(query) {
        return Err(error(
            None,
            None,
            None,
            native::rejected("complete numeric certificate source mismatch"),
        ));
    }
    matching::validate(actual, query)?;
    Ok(CertifiedConicalBody {
        certificate,
        census: actual.census(),
        members: actual
            .members
            .iter()
            .map(|member| (member.id, snapshot::member(member)))
            .collect(),
    })
}
impl CertifiedConicalBody {
    /// Replay every native member before family publication.
    /// Example: `body.rebind_family(&members)?;`.
    pub fn rebind_family(
        &self,
        members: &[ConicalNativeMember],
    ) -> Result<(), ConicalBindingError> {
        let actual = capture(members)?;
        let found: Vec<_> = actual
            .members
            .iter()
            .map(|member| (member.id, snapshot::member(member)))
            .collect();
        if found != self.members {
            return Err(error(
                None,
                None,
                None,
                native::rejected("complete native family source mismatch"),
            ));
        }
        Ok(())
    }
    pub(super) fn rebind_member(
        &self,
        member: &ConicalNativeMember,
    ) -> Result<(), ConicalBindingError> {
        let actual = capture(std::slice::from_ref(member))?;
        let found = snapshot::member(&actual.members[0]);
        if !self
            .members
            .iter()
            .any(|(id, bits)| *id == member.id && *bits == found)
        {
            return Err(error(
                Some(member.id),
                None,
                None,
                native::rejected("ordered member source mismatch"),
            ));
        }
        Ok(())
    }
    /// Exact native inventory independently bound to the mathematical receipt.
    /// Example: `assert_eq!(body.native_census().members, 2);`.
    pub fn native_census(&self) -> ConicalNativeCensus {
        self.census
    }
    /// Borrow the complete issued mathematical family receipt.
    /// Example: `body.certificate().operations()`.
    pub fn certificate(&self) -> &ConicalActualPatchCertificate {
        &self.certificate
    }
}
pub(super) fn capture(
    members: &[ConicalNativeMember],
) -> Result<ConicalNativeGeometry, ConicalBindingError> {
    let actual = capture::family(members)?;
    for (binding, member) in members.iter().map(|binding| {
        let member = actual.members.iter().find(|member| member.id == binding.id);
        (binding, member)
    }) {
        let member = member.ok_or_else(|| {
            error(
                Some(binding.id),
                None,
                None,
                native::rejected("captured source member missing"),
            )
        })?;
        native::complete_entities(&binding.shape, &member.sides)
            .map_err(|cause| error(Some(binding.id), None, None, cause))?;
    }
    Ok(actual)
}
pub(super) fn error(
    member: Option<ConicalActualMemberId>,
    side: Option<ConicalActualSideId>,
    cap: Option<ConicalActualCapId>,
    cause: Failure,
) -> ConicalBindingError {
    ConicalBindingError {
        member,
        side,
        cap,
        cause: Box::new(cause),
    }
}
