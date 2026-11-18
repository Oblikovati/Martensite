// SPDX-License-Identifier: GPL-2.0-only
//! Exact empty domains and solver contacts share consumption, never invented solver metadata.
use martensite_tkmath::predicates::roots::bezier_pair::*;

pub(crate) enum CertifiedPairDomain<'a> {
    Solver(BezierPairCertificate<'a>),
    Excluded {
        _proof: BezierPairBernsteinExclusion<'a>,
    },
}
impl<'a> CertifiedPairDomain<'a> {
    pub(crate) fn contact_count(&self) -> usize {
        match self {
            Self::Solver(proof) => proof.contact_count(),
            Self::Excluded { .. } => 0,
        }
    }
    pub(crate) fn contact(&self, index: usize) -> Option<CertifiedBezierPairContact<'a>> {
        match self {
            Self::Solver(proof) => proof.contact(index),
            Self::Excluded { .. } => None,
        }
    }
    pub(crate) fn common_carrier(&self) -> bool {
        match self {
            Self::Solver(proof) => proof.sign_stage() == BezierPairSignStage::SkippedCommonCarrier,
            Self::Excluded { .. } => false,
        }
    }
    pub(crate) fn solver(self) -> Option<BezierPairCertificate<'a>> {
        match self {
            Self::Solver(proof) => Some(proof),
            Self::Excluded { .. } => None,
        }
    }
}
