// SPDX-License-Identifier: GPL-2.0-only
//! Step 3: the certified sign of every orbit's Green integral.
//! An orbit that is the exact reverse of an earlier one takes the negated sign.
use super::super::binding::SegmentBinding;
use super::super::measure::curves;
use super::super::program_trace::ProgramTrace;
use super::super::types::*;
use super::GraphRegionRequest;
use super::components::Graph;
use super::darts::Dart;
use martensite_tkmath::predicates::roots::metric_bounds::certify_rational_boundary_integral;
use std::collections::BTreeMap;
mod program;

/// `+1` or `-1` per orbit; an enclosure containing zero is refused.
pub(super) fn signs(
    request: &GraphRegionRequest<'_, '_>,
    binding: &SegmentBinding<'_>,
    graph: &Graph,
    cancelled: &mut impl FnMut() -> bool,
) -> LoopRegionCheck<Vec<i8>> {
    signs_with_program(request, binding, graph, cancelled, None)
}
pub(super) fn signs_with_program(
    request: &GraphRegionRequest<'_, '_>,
    binding: &SegmentBinding<'_>,
    graph: &Graph,
    cancelled: &mut impl FnMut() -> bool,
    trace: Option<&mut ProgramTrace>,
) -> LoopRegionCheck<Vec<i8>> {
    let mut state = program::OrientationState::new(trace);
    let mut signs = Vec::with_capacity(graph.orbits.len());
    for (index, orbit) in graph.orbits.iter().enumerate() {
        signs.push(state.sign(request, binding, index, orbit, cancelled)?);
    }
    Ok(signs)
}

fn canonical(darts: impl Iterator<Item = Dart>) -> Vec<Dart> {
    let mut darts: Vec<Dart> = darts.collect();
    darts.sort_unstable();
    darts
}

fn certified_sign(
    request: &GraphRegionRequest<'_, '_>,
    binding: &SegmentBinding<'_>,
    index: usize,
    orbit: &[Dart],
    cancelled: &mut impl FnMut() -> bool,
) -> LoopRegionCheck<i8> {
    let uses: Vec<AuthoredLoopUse> = orbit.iter().map(|d| d.authored(binding)).collect();
    let occurrences = curves(binding, &uses);
    let mut stopped = false;
    let integral = certify_rational_boundary_integral(&occurrences, request.options.area, || {
        stopped = stopped || cancelled();
        stopped
    });
    let measure = completed_integral(integral, stopped, index)?;
    strict_sign(measure.enclosure(), index)
}
fn strict_sign(enclosure: [f64; 2], index: usize) -> LoopRegionCheck<i8> {
    match enclosure {
        [lower, _] if lower > 0. => Ok(1),
        [_, upper] if upper < 0. => Ok(-1),
        enclosure => refuse(
            LoopRegionStage::Faces,
            LoopRegionCause::OrbitOrientation {
                orbit: index,
                enclosure,
            },
        ),
    }
}

fn completed_integral(
    integral: Result<
        martensite_tkmath::predicates::roots::metric_bounds::CertifiedRationalMeasure,
        martensite_tkmath::predicates::roots::metric_bounds::RationalIntegralError,
    >,
    stopped: bool,
    index: usize,
) -> LoopRegionCheck<martensite_tkmath::predicates::roots::metric_bounds::CertifiedRationalMeasure>
{
    let stage = LoopRegionStage::Faces;
    match integral {
        Ok(measure) => Ok(measure),
        Err(_) if stopped => refuse(stage, LoopRegionCause::Cancelled),
        Err(error) => {
            let cause = LoopRegionCause::Area {
                region: index as u64 + 1,
                curve_index: error.curve_index,
                cause: error.to_string(),
            };
            refuse(stage, cause)
        }
    }
}
