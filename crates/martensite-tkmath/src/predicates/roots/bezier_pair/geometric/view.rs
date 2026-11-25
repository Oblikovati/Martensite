// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::{
    certificate_arithmetic::{ExactRatio, RatioInterval},
    metric_bounds::MetricParameterMap,
};
use super::*;
/// Opaque borrowed witness covering the entire actual pair or off-diagonal self domain.
pub struct BezierPairGeometricCertificate<'a> {
    pub(super) source: &'a CapturedBezierPair,
    pub(super) proof: &'a GeometricProof,
}
/// Exactly one geometric endpoint incidence; no algebraic multiplicity is asserted.
pub struct CertifiedBezierEndpointContact<'a> {
    contact: &'a Endpoint,
}
impl CertifiedBezierEndpointContact<'_> {
    /// Native unit-interval endpoint slots; `contact.parameters()==[1,0]`.
    pub fn parameters(&self) -> [u8; 2] {
        self.contact.parameters
    }
    /// Actual shared binary64 point from the first source; `contact.point()`.
    pub fn point(&self) -> [f64; 2] {
        self.contact.point
    }
}
impl<'a> BezierPairGeometricCertificate<'a> {
    /// Read the actually executed geometric method; `proof.kind()`.
    pub fn kind(&self) -> BezierPairGeometricKind {
        match self.proof {
            GeometricProof::Box { .. } => BezierPairGeometricKind::BoxExclusion,
            GeometricProof::Monotone(_) => BezierPairGeometricKind::MonotoneSelf,
            GeometricProof::Endpoint { .. } => BezierPairGeometricKind::EndpointSector,
            GeometricProof::Hull(_) => BezierPairGeometricKind::HullExclusion,
        }
    }
    /// Borrow the sole endpoint incidence, when present; `proof.contact().map(|c|c.parameters())`.
    pub fn contact(&self) -> Option<CertifiedBezierEndpointContact<'a>> {
        let GeometricProof::Endpoint { contact, .. } = self.proof else {
            return None;
        };
        Some(CertifiedBezierEndpointContact { contact })
    }
    /// Complete geometric contact count, including completed empty; `proof.contact_count()`.
    pub fn contact_count(&self) -> usize {
        usize::from(self.contact().is_some())
    }
    /// Measure actual native parameters using the existing exact construction path.
    /// Example: `proof.measure_native_parameters(maps,native,limits)?`.
    pub fn measure_native_parameters(
        &self,
        maps: [MetricParameterMap; 2],
        native: [f64; 2],
        limits: [f64; 2],
    ) -> Result<Option<[MeasuredPairParameter; 2]>, PairParameterConstructionFailure> {
        let Some(contact) = self.contact() else {
            return Ok(None);
        };
        let bounds = contact
            .parameters()
            .map(|p| RatioInterval::point(ExactRatio::from_finite(f64::from(p))));
        super::super::native_parameter_construction::measure_captured_axes(
            bounds.each_ref(),
            self.source.captured_curves(),
            maps,
            native,
            limits,
        )
        .map(Some)
    }
}
