// SPDX-License-Identifier: GPL-2.0-only
//! Reuse literal coefficient enclosures while preserving each rail's independent proof.
use super::*;

/// Operation-local workspace for the decoded UV trace shared by cylinder cap rails.
/// A new radius or spatial curve still receives a full independent residual check.
/// Example: `let proof = workspace.certify(query, || false)?;`.
#[derive(Default)]
pub struct CylindricalTrimWorkspace {
    last: Option<BoundTrace>,
}

struct BoundTrace {
    pcurve: Vec<[u64; 2]>,
    fractional_bits: u16,
    capture: CapturedTrace,
}

impl CylindricalTrimWorkspace {
    /// Certify a rail, reusing only identical UV controls at identical precision.
    /// Example: `let far_proof = workspace.certify(far_query, || false)?;`.
    pub fn certify(
        &mut self,
        query: CylindricalTrimQuery<'_>,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalTrim, CylindricalTrimError> {
        issue(query, &mut cancelled, Some(self))
    }

    pub(super) fn matching(&self, query: CylindricalTrimQuery<'_>) -> Option<&CapturedTrace> {
        self.last.as_ref().and_then(|previous| {
            (previous.fractional_bits == query.options.fractional_bits
                && previous.pcurve.iter().copied().eq(uv_bits(query.pcurve)))
            .then_some(&previous.capture)
        })
    }

    pub(super) fn remember(&mut self, query: CylindricalTrimQuery<'_>, capture: CapturedTrace) {
        self.last = Some(BoundTrace {
            pcurve: uv_bits(query.pcurve).collect(),
            fractional_bits: query.options.fractional_bits,
            capture,
        });
    }
}

fn uv_bits(pcurve: &[[f64; 2]]) -> impl Iterator<Item = [u64; 2]> + '_ {
    pcurve.iter().map(|point| point.map(f64::to_bits))
}

pub(super) struct CapturedTrace {
    height: Vec<Interval>,
    trig: [Vec<Interval>; 2],
    uv_steps: u32,
    trig_steps: u32,
}

pub(super) fn residual(
    query: CylindricalTrimQuery<'_>,
    ranges: [[f64; 2]; 2],
    previous: Option<&CapturedTrace>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([f64; 3], Option<CapturedTrace>)> {
    if let Some(previous) = previous {
        return Ok((previous.replay_residual(query, arithmetic)?, None));
    }
    let (bounds, capture) = capture_residual(query, ranges, arithmetic)?;
    Ok((bounds, Some(capture)))
}

fn capture_residual(
    query: CylindricalTrimQuery<'_>,
    [space, chart]: [[f64; 2]; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<([f64; 3], CapturedTrace)> {
    let start = arithmetic.operations();
    let [angles, height] = narrowed(capture_planar(query.pcurve, arithmetic)?, chart, arithmetic)?;
    let uv_steps = arithmetic.operations() - start;
    let stored = narrowed(source::stored(query.curve, arithmetic)?, space, arithmetic)?;
    let weight = proof::minimum_weight(&stored);
    let start = arithmetic.operations();
    let trig = trigonometric::enclose(&angles, arithmetic)?;
    let trig_steps = arithmetic.operations() - start;
    let bounds = physical_residual(query.radius, &stored, &weight, &height, &trig, arithmetic)?;
    Ok((
        bounds,
        CapturedTrace {
            height,
            trig,
            uv_steps,
            trig_steps,
        },
    ))
}

impl CapturedTrace {
    fn replay_residual(
        &self,
        query: CylindricalTrimQuery<'_>,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Checked<[f64; 3]> {
        // Preserve the original UV/rail/trig step order and each new rail's own ledger.
        replay_steps(self.uv_steps, arithmetic)?;
        let stored = source::stored(query.curve, arithmetic)?;
        let weight = proof::minimum_weight(&stored);
        replay_steps(self.trig_steps, arithmetic)?;
        physical_residual(
            query.radius,
            &stored,
            &weight,
            &self.height,
            &self.trig,
            arithmetic,
        )
    }
}

fn replay_steps(count: u32, arithmetic: &mut Arithmetic<'_>) -> Checked<()> {
    for _ in 0..count {
        arithmetic.certificate_step()?;
    }
    Ok(())
}

fn physical_residual(
    radius: f64,
    stored: &[Vec<Interval>; 4],
    weight: &Interval,
    height: &[Interval],
    trig: &[Vec<Interval>; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[f64; 3]> {
    let radius = arithmetic.scalar(radius)?;
    let xy = proof::xy_residual(&radius, stored, trig, weight, arithmetic)?;
    let height = proof::height_residual(height, stored, weight, arithmetic)?;
    let total = arithmetic.add(&xy, &height)?;
    Ok([
        arithmetic.upper_f64(xy.upper)?,
        arithmetic.upper_f64(height.upper)?,
        arithmetic.upper_f64(total.upper)?,
    ])
}

#[cfg(test)]
#[path = "trace_tests.rs"]
mod tests;
