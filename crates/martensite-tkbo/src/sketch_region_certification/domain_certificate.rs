// SPDX-License-Identifier: GPL-2.0-only
//! Exact empty domains and solver contacts share consumption, never invented solver metadata.
use martensite_tkmath::predicates::roots::bezier_pair::*;

pub(crate) enum CertifiedPairDomain<'a> {
    Solver(BezierPairCertificate<'a>),
    Geometric(BezierPairGeometricCertificate<'a>),
    Excluded {
        _proof: BezierPairBernsteinExclusion<'a>,
    },
}
impl<'a> CertifiedPairDomain<'a> {
    pub(crate) fn contact_count(&self) -> usize {
        match self {
            Self::Solver(proof) => proof.contact_count(),
            Self::Geometric(proof) => proof.contact_count(),
            Self::Excluded { .. } => 0,
        }
    }
    pub(crate) fn contact(
        &self,
        index: usize,
    ) -> Option<super::certified_contact::CertifiedDomainContact<'a>> {
        match self {
            Self::Solver(proof) => proof
                .contact(index)
                .map(super::certified_contact::CertifiedDomainContact::Solver),
            Self::Geometric(proof) => (index == 0)
                .then(|| proof.contact())
                .flatten()
                .map(super::certified_contact::CertifiedDomainContact::Endpoint),
            Self::Excluded { .. } => None,
        }
    }
    pub(crate) fn common_carrier(&self) -> bool {
        match self {
            Self::Solver(proof) => proof.sign_stage() == BezierPairSignStage::SkippedCommonCarrier,
            Self::Geometric(_) | Self::Excluded { .. } => false,
        }
    }
    pub(crate) fn solver(self) -> Option<BezierPairCertificate<'a>> {
        match self {
            Self::Solver(proof) => Some(proof),
            Self::Geometric(_) | Self::Excluded { .. } => None,
        }
    }
}

impl CertifiedPairDomain<'_> {
    pub(crate) fn measure_native_parameters(
        &self,
        index: usize,
        maps: [martensite_tkmath::predicates::roots::metric_bounds::MetricParameterMap; 2],
        native: [f64; 2],
        limits: [f64; 2],
    ) -> Result<Option<[MeasuredPairParameter; 2]>, PairParameterConstructionFailure> {
        match self {
            Self::Solver(proof) => proof.measure_native_parameters(index, maps, native, limits),
            Self::Geometric(proof) if index == 0 => {
                proof.measure_native_parameters(maps, native, limits)
            }
            _ => Ok(None),
        }
    }
}
