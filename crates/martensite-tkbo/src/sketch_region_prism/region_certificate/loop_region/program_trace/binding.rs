// SPDX-License-Identifier: GPL-2.0-only
//! Rebinding reconstructs every executed program from the retained actual source bits.
use super::*;
use std::collections::BTreeMap;

type Sources<'a> = BTreeMap<RegionSourceId, &'a IntegralSource>;

impl GraphIntegralEvidence {
    pub(crate) fn matches_issued_sources(&self) -> bool {
        if !self.valid_schedule() {
            return false;
        }
        let mut sources = Sources::new();
        for source in &self.sources {
            sources.entry(source.id).or_insert(source);
        }
        let mut consumed = 0u64;
        for (index, step) in self.steps.iter().enumerate() {
            if !self.matches_step(index, step, consumed, &sources) {
                return false;
            }
            let Some(total) = advance_consumed(step, consumed) else {
                return false;
            };
            consumed = total;
        }
        consumed == self.consumed && consumed <= self.options.max_total_subintervals
    }
    fn matches_step(
        &self,
        index: usize,
        step: &IntegralStep,
        consumed: u64,
        sources: &Sources<'_>,
    ) -> bool {
        match step {
            IntegralStep::Reverse { parent, uses } => self.matches_reverse(index, *parent, uses),
            IntegralStep::Direct { role, loops, proof } => {
                let mut expected = self.options;
                let Some(remaining) = expected.max_total_subintervals.checked_sub(consumed) else {
                    return false;
                };
                expected.max_total_subintervals = remaining;
                if proof.requested_options() != expected {
                    return false;
                }
                matches_program(role, loops, proof, sources)
            }
        }
    }
}
fn matches_program(
    role: &IntegralRole,
    loops: &[Vec<AuthoredLoopUse>],
    proof: &CertifiedIntegralProgram,
    sources: &Sources<'_>,
) -> bool {
    let Some(curves) = loops
        .iter()
        .map(|uses| curves(sources, uses))
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };
    match role {
        IntegralRole::Boundary(_) => curves.len() == 1 && matches_boundary(proof, &curves[0]),
        IntegralRole::Face(_) => {
            matches_face(proof, &curves.iter().map(Vec::as_slice).collect::<Vec<_>>())
        }
    }
}
fn curves<'a>(
    sources: &Sources<'a>,
    uses: &[AuthoredLoopUse],
) -> Option<Vec<OrientedMetricCurve<'a>>> {
    uses.iter()
        .map(|directed| {
            let source = sources.get(&directed.source)?;
            Some(OrientedMetricCurve {
                curve: MetricCurve::Bezier(MetricBezier {
                    poles: &source.poles,
                    weights: source.weights.as_deref(),
                }),
                reversed: directed.reversed,
            })
        })
        .collect()
}
impl GraphIntegralEvidence {
    fn matches_reverse(&self, index: usize, parent: usize, uses: &[AuthoredLoopUse]) -> bool {
        if parent >= index {
            return false;
        }
        let Some(parent_uses) = self.boundary_uses(parent) else {
            return false;
        };
        let reverse = reversed_occurrences(parent_uses);
        uses.len() == reverse.len()
            && (0..uses.len()).any(|start| {
                uses.iter()
                    .enumerate()
                    .all(|(i, use_)| *use_ == reverse[(i + start) % uses.len()])
            })
    }
}

impl GraphIntegralEvidence {
    fn valid_schedule(&self) -> bool {
        let mut boundaries = 0usize;
        let mut faces = 0u64;
        for step in &self.steps {
            match step {
                IntegralStep::Direct {
                    role: IntegralRole::Boundary(index),
                    loops,
                    ..
                } if faces == 0 && *index == boundaries && loops.len() == 1 => boundaries += 1,
                IntegralStep::Reverse { .. } if faces == 0 => boundaries += 1,
                IntegralStep::Direct {
                    role: IntegralRole::Face(name),
                    ..
                } if *name == faces + 1 => faces += 1,
                _ => return false,
            }
        }
        boundaries > 0 && faces > 0
    }
}

impl GraphIntegralEvidence {
    fn boundary_uses(&self, parent: usize) -> Option<&[AuthoredLoopUse]> {
        match &self.steps[parent] {
            IntegralStep::Direct {
                role: IntegralRole::Boundary(_),
                loops,
                ..
            } if loops.len() == 1 => Some(&loops[0]),
            IntegralStep::Reverse { uses, .. } => Some(uses),
            _ => None,
        }
    }
}

fn reversed_occurrences(parent_uses: &[AuthoredLoopUse]) -> Vec<AuthoredLoopUse> {
    parent_uses
        .iter()
        .rev()
        .map(|use_| AuthoredLoopUse {
            source: use_.source,
            reversed: !use_.reversed,
        })
        .collect()
}

fn advance_consumed(step: &IntegralStep, consumed: u64) -> Option<u64> {
    match step {
        IntegralStep::Direct { proof, .. } => {
            consumed.checked_add(proof.integration_subintervals())
        }
        IntegralStep::Reverse { .. } => Some(consumed),
    }
}
