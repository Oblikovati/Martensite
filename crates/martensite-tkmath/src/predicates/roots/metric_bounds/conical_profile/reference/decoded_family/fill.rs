// SPDX-License-Identifier: GPL-2.0-only
//! An exact fill check at the decoded endpoint makes the cap homotopy unconditional.
use super::*;
use crate::predicates::roots::{
    RootIsolationOptions,
    bezier_line::{
        BezierWireError, BezierWireLocation, BezierWireQuery, BezierWireSegment,
        classify_bezier_wire,
    },
    metric_bounds::{
        CylindricalPeriodicProbe, CylindricalProbeError, CylindricalProbeFailure,
        CylindricalProbeOptions, CylindricalProbeSense,
    },
};
#[cfg(test)]
mod tests;

const MAX_LOCATIONS: u32 = 8192;

/// Why the decoded cap winding could not establish a disjoint filled family.
#[derive(Debug)]
pub enum ConicalDecodedFillFailure {
    /// Missing decoded cap source or closed contour.
    Binding,
    /// A source boundary witness lies in the target member's material.
    Overlap {
        /// Index in the issuing member order.
        source: usize,
        /// Index in the issuing member order.
        target: usize,
        /// Absolute image number, zero for the unwrapped fill.
        turn: u32,
    },
    /// The inventory exceeded the folded family winding limits.
    WorkLimit,
    /// Cancelled while binding or classifying a winding witness.
    Cancelled,
    /// Rounded periodic seeds could not be certified on the issued boundaries.
    Probe(Box<CylindricalProbeError>),
    /// The exact ray could not be classified.
    Winding(Box<BezierWireError>),
    /// The witness touched a contour or its winding was not Jordan.
    Boundary(BezierWireLocation),
}
impl ConicalDecodedFillFailure {
    pub(super) fn cancelled(&self) -> bool {
        match self {
            Self::Cancelled => true,
            Self::Probe(cause) => matches!(
                &cause.cause,
                CylindricalProbeFailure::Arithmetic(value)
                    if matches!(value.as_ref(), BoundedArithmeticFailure::Cancelled)
            ),
            _ => false,
        }
    }
}

type Checked<T> = Result<T, ConicalDecodedFillFailure>;
type Contour = Vec<Vec<[f64; 2]>>;

pub(super) fn certify(
    joins: &[&ConicalJoinsEvidence],
    periodic: &Periodic,
    options: ConeClearanceOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Result<u32, (ConicalDecodedFillFailure, u32)> {
    let controls: Vec<_> = joins
        .iter()
        .map(|join| contours(join))
        .collect::<Checked<_>>()
        .map_err(unspent)?;
    let members = controls
        .iter()
        .map(|loops| Member::capture(loops))
        .collect::<Checked<Vec<_>>>()
        .map_err(unspent)?;
    let mut work = Work {
        remaining: RootIsolationOptions::new(1e-6),
        locations: 0,
        cancelled,
    };
    let spent = verify_images(&members, periodic, options, &mut work)?;
    verify_unwrapped(&members, &mut work).map_err(|cause| (cause, spent))?;
    Ok(spent)
}

fn unspent(cause: ConicalDecodedFillFailure) -> (ConicalDecodedFillFailure, u32) {
    (cause, 0)
}

struct Member<'a> {
    contours: Vec<Vec<BezierWireSegment<'a>>>,
    bounds: [[f64; 2]; 2],
    witness: [f64; 2],
}

fn contours(joins: &ConicalJoinsEvidence) -> Checked<Vec<Contour>> {
    use ConicalDecodedFillFailure::Binding;
    let mut spans = joins.cap_charts.as_ref().ok_or(Binding)?.reflected_poles();
    Ok(joins
        .contour_lengths
        .iter()
        .map(|&len| spans.by_ref().take(len).collect::<Contour>())
        .collect())
}
impl<'a> Member<'a> {
    fn capture(contours: &'a [Contour]) -> Checked<Self> {
        use ConicalDecodedFillFailure::Binding;
        let outer = contours.first().ok_or(Binding)?;
        let witness = *outer
            .first()
            .and_then(|curve| curve.first())
            .ok_or(Binding)?;
        let bounds = outer.iter().flatten().fold([witness; 2], extend);
        Ok(Self {
            contours: contours.iter().map(segments).collect(),
            bounds,
            witness,
        })
    }
}

fn extend([lower, upper]: [[f64; 2]; 2], point: &[f64; 2]) -> [[f64; 2]; 2] {
    [
        std::array::from_fn(|i| lower[i].min(point[i])),
        std::array::from_fn(|i| upper[i].max(point[i])),
    ]
}

fn segments(contour: &Contour) -> Vec<BezierWireSegment<'_>> {
    contour
        .iter()
        .enumerate()
        .map(|(id, controls)| BezierWireSegment {
            id: id as u64,
            controls,
            weights: None,
            interval: [0., 1.],
            reversed: false,
        })
        .collect()
}

fn verify_images(
    members: &[Member<'_>],
    periodic: &Periodic,
    options: ConeClearanceOptions,
    work: &mut Work<'_>,
) -> Result<u32, (ConicalDecodedFillFailure, u32)> {
    let probe_options = CylindricalProbeOptions {
        fractional_bits: options.fractional_bits,
        max_operations: options.max_operations,
        ..Default::default()
    };
    let count = periodic.images().len() as u64 * (members.len() as u64).pow(2) * 2;
    if count > u64::from(probe_options.max_probes) {
        return Err(unspent(ConicalDecodedFillFailure::WorkLimit));
    }
    let jobs: Vec<_> = image_jobs(members.len(), periodic).collect();
    let requests: Vec<_> = jobs
        .iter()
        .map(|(job, sense)| job.probe(members, *sense))
        .collect();
    let probes = periodic
        .certify_probes_with_work(&requests, probe_options, &mut *work.cancelled)
        .map_err(|error| {
            (
                ConicalDecodedFillFailure::Probe(Box::new(error.cause)),
                error.operations,
            )
        })?;
    let spent = probes.operations();
    jobs.iter()
        .zip(probes.probes())
        .try_for_each(|((job, _), probe)| job.check(members, probe.point(), work))
        .map_err(|cause| (cause, spent))?;
    Ok(spent)
}

fn image_jobs(
    count: usize,
    periodic: &Periodic,
) -> impl Iterator<Item = (Job, CylindricalProbeSense)> + '_ {
    use CylindricalProbeSense::{Add, Subtract};
    periodic.images().iter().flat_map(move |image| {
        let turn = image.translation().turn();
        (0..count).flat_map(move |a| {
            (0..count).flat_map(move |b| {
                [
                    (
                        Job {
                            source: a,
                            target: b,
                            turn,
                        },
                        Subtract,
                    ),
                    (
                        Job {
                            source: b,
                            target: a,
                            turn,
                        },
                        Add,
                    ),
                ]
            })
        })
    })
}

fn verify_unwrapped(members: &[Member<'_>], work: &mut Work<'_>) -> Checked<()> {
    for (source, member) in members.iter().enumerate() {
        for target in 0..members.len() {
            if source != target {
                Job {
                    source,
                    target,
                    turn: 0,
                }
                .check(members, member.witness, work)?;
            }
        }
    }
    Ok(())
}

struct Job {
    source: usize,
    target: usize,
    turn: u32,
}
impl Job {
    fn probe(
        &self,
        members: &[Member<'_>],
        sense: CylindricalProbeSense,
    ) -> CylindricalPeriodicProbe {
        CylindricalPeriodicProbe {
            turn: self.turn,
            point: members[self.source].witness,
            sense,
        }
    }

    fn check(&self, members: &[Member<'_>], point: [f64; 2], work: &mut Work<'_>) -> Checked<()> {
        if members[self.target].contains(point, work)? {
            return Err(ConicalDecodedFillFailure::Overlap {
                source: self.source,
                target: self.target,
                turn: self.turn,
            });
        }
        Ok(())
    }
}
impl Member<'_> {
    fn contains(&self, point: [f64; 2], work: &mut Work<'_>) -> Checked<bool> {
        work.poll()?;
        if (0..2).any(|i| point[i] < self.bounds[0][i] || point[i] > self.bounds[1][i]) {
            return Ok(false);
        }
        if !work.inside(&self.contours[0], point)? {
            return Ok(false);
        }
        for hole in &self.contours[1..] {
            if work.inside(hole, point)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

struct Work<'a> {
    remaining: RootIsolationOptions,
    locations: u32,
    cancelled: &'a mut (dyn FnMut() -> bool + Send),
}
impl Work<'_> {
    fn poll(&mut self) -> Checked<()> {
        if (self.cancelled)() {
            return Err(ConicalDecodedFillFailure::Cancelled);
        }
        Ok(())
    }
    fn inside(&mut self, contour: &[BezierWireSegment<'_>], point: [f64; 2]) -> Checked<bool> {
        self.locations += 1;
        if self.locations > MAX_LOCATIONS {
            return Err(ConicalDecodedFillFailure::WorkLimit);
        }
        let line = [point, [if point[0] == 0. { 1. } else { 0. }, point[1]]];
        let query = BezierWireQuery {
            segments: contour,
            line,
            options: self.remaining,
        };
        let result = self.classify(query)?;
        self.remaining.max_bisections -= result.bisections();
        match result.location() {
            BezierWireLocation::Inside { winding: -1 | 1 } => Ok(true),
            BezierWireLocation::Outside => Ok(false),
            location => Err(ConicalDecodedFillFailure::Boundary(location.clone())),
        }
    }

    fn classify(
        &mut self,
        query: BezierWireQuery<'_>,
    ) -> Checked<crate::predicates::roots::bezier_line::BezierWireResult> {
        let mut observed = false;
        let result = classify_bezier_wire(query, || {
            observed |= (self.cancelled)();
            observed
        });
        result.map_err(|cause| {
            if observed {
                return ConicalDecodedFillFailure::Cancelled;
            }
            ConicalDecodedFillFailure::Winding(Box::new(cause))
        })
    }
}
