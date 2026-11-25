// SPDX-License-Identifier: GPL-2.0-only
//! One whole-face program takes magnitudes only after each complete loop is integrated.
use super::super::program_trace::ProgramTrace;
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::CertifiedIntegralProgram;
use std::sync::Arc;
pub(in crate::sketch_region_prism::region_certificate::loop_region) fn certify(
    binding: &SegmentBinding<'_>,
    face: FaceUses<'_>,
    trace: &mut ProgramTrace,
    cancelled: &mut impl FnMut() -> bool,
) -> LoopRegionCheck<Arc<CertifiedIntegralProgram>> {
    let loops: Vec<_> = std::iter::once(face.outer)
        .chain(face.holes.iter().copied())
        .collect();
    let uses: Vec<_> = loops.iter().map(|uses| uses.to_vec()).collect();
    let curves: Vec<_> = loops
        .iter()
        .map(|uses| super::curves(binding, uses))
        .collect();
    let views: Vec<_> = curves.iter().map(Vec::as_slice).collect();
    trace
        .face(face.name, uses, &views, cancelled)
        .map_err(|cause| LoopRegionFailure {
            stage: LoopRegionStage::Regions,
            cause: LoopRegionCause::IntegralProgram(Box::new(cause)),
        })
}
