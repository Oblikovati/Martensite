// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkmath::predicates::roots::metric_bounds::{
    BoundaryIntegralProgramQuery, CertifiedIntegralProgram, FaceIntegralProgramQuery,
    IntegralProgramLoop, OrientedMetricCurve,
};

pub(super) fn components<'a>(
    curves: &'a [OrientedMetricCurve<'a>],
    maximum: u16,
) -> Vec<&'a [OrientedMetricCurve<'a>]> {
    if maximum == 0 {
        // Keep the invalid policy intact for Math admission rather than panicking in chunks(0).
        return vec![curves];
    }
    curves.chunks(usize::from(maximum)).collect()
}
pub(crate) fn matches_boundary(
    proof: &CertifiedIntegralProgram,
    curves: &[OrientedMetricCurve<'_>],
) -> bool {
    let options = proof.requested_options();
    let parts = components(curves, options.integration.max_curves);
    proof.matches_boundary(BoundaryIntegralProgramQuery {
        components: &parts,
        options,
    })
}
pub(crate) fn matches_face(
    proof: &CertifiedIntegralProgram,
    loops: &[&[OrientedMetricCurve<'_>]],
) -> bool {
    let options = proof.requested_options();
    let parts: Vec<_> = loops
        .iter()
        .map(|curves| components(curves, options.integration.max_curves))
        .collect();
    let loops: Vec<_> = parts
        .iter()
        .map(|components| IntegralProgramLoop { components })
        .collect();
    proof.matches_face(FaceIntegralProgramQuery {
        loops: &loops,
        options,
    })
}
