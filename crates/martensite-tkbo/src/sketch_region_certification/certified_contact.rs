// SPDX-License-Identifier: GPL-2.0-only
//! Endpoint-sector contacts and projected roots keep distinct identities and witnesses.
use martensite_tkmath::predicates::roots::{DyadicBound, bezier_pair::*};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ContactProofKey {
    ProjectedRoot(usize),
    Endpoint,
}
pub(crate) enum CertifiedDomainContact<'a> {
    Solver(CertifiedBezierPairContact<'a>),
    Endpoint(CertifiedBezierEndpointContact<'a>),
}
impl CertifiedDomainContact<'_> {
    pub(crate) fn key(&self) -> ContactProofKey {
        match self {
            Self::Solver(contact) => ContactProofKey::ProjectedRoot(contact.candidate_index),
            Self::Endpoint(_) => ContactProofKey::Endpoint,
        }
    }
    pub(crate) fn point(&self) -> [f64; 2] {
        match self {
            Self::Solver(contact) => contact.point,
            Self::Endpoint(contact) => contact.point(),
        }
    }
    pub(crate) fn first_bounds(&self) -> Option<[DyadicBound; 2]> {
        match self {
            Self::Solver(contact) => {
                let (first, last) = contact.parameter.bounds();
                Some([first.clone(), last.clone()])
            }
            Self::Endpoint(contact) => {
                let parameter = DyadicBound::from_f64(f64::from(contact.parameters()[0])).ok()?;
                Some([parameter.clone(), parameter])
            }
        }
    }
    pub(crate) fn exact_endpoints(&self) -> Option<[u8; 2]> {
        match self {
            Self::Solver(contact) => solver_endpoints(contact),
            Self::Endpoint(contact) => Some(contact.parameters()),
        }
    }
    pub(crate) fn projected_multiplicity(&self) -> Option<usize> {
        match self {
            Self::Solver(contact) => Some(contact.parameter.multiplicity()),
            Self::Endpoint(_) => None,
        }
    }
}
fn solver_endpoints(contact: &CertifiedBezierPairContact<'_>) -> Option<[u8; 2]> {
    let (lower, upper) = contact.parameter.bounds();
    let first = [0u8, 1].into_iter().find(|value| {
        lower
            .compare_f64(f64::from(*value))
            .is_ok_and(|o| o.is_eq())
            && upper
                .compare_f64(f64::from(*value))
                .is_ok_and(|o| o.is_eq())
    })?;
    let paired = &contact.exact_paired_parameter;
    let second = [0u8, 1].into_iter().find(|value| {
        paired
            .compare_lower(f64::from(*value))
            .is_ok_and(|o| o.is_eq())
            && paired
                .compare_upper(f64::from(*value))
                .is_ok_and(|o| o.is_eq())
    })?;
    Some([first, second])
}
