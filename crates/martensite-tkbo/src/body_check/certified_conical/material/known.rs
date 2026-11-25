// SPDX-License-Identifier: GPL-2.0-only
//! Exact selected nonperiodic material on caller-identified directed cycles.
use super::*;
use crate::profile_extrusion::{LoopUse, PlanarProofWorkReceipt, ProfilePlane, ProfileVertexId};
use martensite_tkmath::predicates::roots::{
    RootIsolationOptions, bezier_line::*, bezier_pair::*, metric_bounds::*,
};
use std::collections::BTreeSet;

mod contract;
mod pairs;
mod snapshot;
#[cfg(test)]
mod tests;
mod wires;
use self::{
    pairs::certify_pairs,
    snapshot::{snapshot, source_bits},
    wires::certify_rings,
};
use DirectedCapCause as Cause;
pub use contract::{
    DirectedCapCause, DirectedCapFailure, DirectedCapPairCause, DirectedCapPolicy, DirectedCapWork,
};

pub(super) struct Seal {
    pub bits: Arc<[u64]>,
    pub work: DirectedCapWork,
}

pub(super) fn preflight(
    profile: &SketchProfile,
    outer: &ProfileLoop,
    holes: &[ProfileLoop],
    policy: DirectedCapPolicy,
) -> Result<(), Cause> {
    let limits = RationalIntegralOptions::default();
    let maximum = u64::from(limits.max_curves) * u64::from(limits.max_subintervals);
    if !(1..=64).contains(&policy.max_components) {
        return Err(Cause::Policy {
            field: "max_components",
            actual: u64::from(policy.max_components),
            minimum: 1,
            maximum: 64,
        });
    }
    if policy.requested_integration_cells > maximum {
        return Err(Cause::Policy {
            field: "max_total_subintervals",
            actual: policy.requested_integration_cells,
            minimum: 0,
            maximum,
        });
    }
    let count = profile.curves.len();
    if !(1..=2048).contains(&count) {
        return Err(Cause::Shape {
            field: "curves",
            actual: count,
            expected: 2048,
        });
    }
    if profile.vertices.len() != count {
        return Err(Cause::Shape {
            field: "vertices",
            actual: profile.vertices.len(),
            expected: count,
        });
    }
    if holes.len() >= usize::from(policy.max_components) {
        return Err(Cause::Components {
            actual: holes.len() + 1,
            limit: policy.max_components,
        });
    }
    let mut uses = 0usize;
    let mut components = 0usize;
    for wire in std::iter::once(outer).chain(holes) {
        let n = wire.uses.len();
        if !(2..=count).contains(&n) {
            return Err(Cause::Shape {
                field: "ring uses",
                actual: n,
                expected: count,
            });
        }
        uses += n;
        components += n.div_ceil(usize::from(limits.max_curves));
    }
    if components > usize::from(policy.max_components) {
        return Err(Cause::Components {
            actual: components,
            limit: policy.max_components,
        });
    }
    if uses != count {
        return Err(Cause::Shape {
            field: "complete face uses",
            actual: uses,
            expected: count,
        });
    }
    for curve in &profile.curves {
        let ProfileCurve::Spline {
            degree: 1..=3,
            poles,
            weights: None,
            knots,
            ..
        } = curve
        else {
            return Err(Cause::Topology(rejected(
                "expected full-range unweighted Bezier degree 1..=3",
            )));
        };
        if poles.len() > 4 || knots.len() > 8 {
            return Err(Cause::Topology(rejected(
                "polynomial curve storage exceeds degree-three limits",
            )));
        }
    }
    Ok(())
}

type Point = [f64; 2];
type Expected = BTreeMap<(usize, usize), Vec<[u8; 2]>>;
struct Piece<'a> {
    id: ProfileCurveId,
    ends: [ProfileVertexId; 2],
    poles: &'a [Point],
}
struct Use {
    piece: usize,
    reversed: bool,
}
struct Ring {
    uses: Vec<Use>,
    bounds: Bounds,
    witness: Point,
}

pub(super) fn seal(
    profile: &SketchProfile,
    outer: &ProfileLoop,
    holes: &[ProfileLoop],
    resolution: Resolution,
    policy: DirectedCapPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Seal, DirectedCapFailure> {
    let pieces = pieces(profile).map_err(|err| unobserved(Cause::Topology(err)))?;
    let (rings, expected) =
        rings(&pieces, outer, holes).map_err(|err| unobserved(Cause::Topology(err)))?;
    let mut work = DirectedCapWork::start(policy);
    work.loops = rings.len();
    let run = certify_pairs(&pieces, &expected, resolution, &mut work, cancelled)
        .and_then(|()| certify_rings(&pieces, &rings, &mut work, cancelled));
    run.map_err(|cause| DirectedCapFailure {
        cause: Box::new(cause),
        observed: Some(work),
        cancelled: false,
    })?;
    Ok(Seal {
        bits: snapshot(profile, outer, holes, resolution, work).into(),
        work,
    })
}

pub(super) fn unobserved(cause: Cause) -> DirectedCapFailure {
    DirectedCapFailure {
        cause: Box::new(cause),
        observed: None,
        cancelled: false,
    }
}

pub(super) fn source_matches(profile: &SketchProfile, original: &SketchProfile) -> bool {
    profile == original && source_bits(profile) == source_bits(original)
}

fn pieces(profile: &SketchProfile) -> Result<Vec<Piece<'_>>, Failure> {
    if profile.plane != ProfilePlane::world_xy()
        || profile.curves.is_empty()
        || profile.curves.len() > 2048
    {
        return Err(rejected("known cap requires world XY and 1..=2048 curves"));
    }
    let vertices: BTreeMap<_, _> = profile
        .vertices
        .iter()
        .map(|v| (v.id, v.point.map(f64::to_bits)))
        .collect();
    if vertices.len() != profile.vertices.len() || vertices.len() != profile.curves.len() {
        return Err(rejected(
            "known cap requires one distinct vertex per source curve",
        ));
    }
    let mut ids = BTreeSet::new();
    profile
        .curves
        .iter()
        .map(|curve| piece(curve, &vertices, &mut ids))
        .collect()
}

fn piece<'a>(
    curve: &'a ProfileCurve,
    vertices: &BTreeMap<ProfileVertexId, [u64; 2]>,
    ids: &mut BTreeSet<ProfileCurveId>,
) -> Result<Piece<'a>, Failure> {
    let ProfileCurve::Spline {
        id,
        start,
        end,
        degree,
        poles,
        weights: None,
        knots,
    } = curve
    else {
        return Err(rejected("known cap requires unweighted polynomial splines"));
    };
    let n = *degree as usize + 1;
    let unit = std::iter::repeat_n(0f64.to_bits(), n).chain(std::iter::repeat_n(1f64.to_bits(), n));
    if !(2..=4).contains(&n)
        || poles.len() != n
        || !knots.iter().map(|x| x.to_bits()).eq(unit)
        || !poles.iter().flatten().all(|p| p.is_finite())
        || !ids.insert(*id)
        || start == end
        || vertices.get(start) != Some(&poles[0].map(f64::to_bits))
        || vertices.get(end) != Some(&poles[n - 1].map(f64::to_bits))
    {
        return Err(rejected(&format!(
            "known cap curve={id:?} requires exact endpoints, unique ID and unit degree-1..3 domain"
        )));
    }
    Ok(Piece {
        id: *id,
        ends: [*start, *end],
        poles,
    })
}

fn rings(
    pieces: &[Piece<'_>],
    outer: &ProfileLoop,
    holes: &[ProfileLoop],
) -> Result<(Vec<Ring>, Expected), Failure> {
    let by_id: BTreeMap<_, _> = pieces
        .iter()
        .enumerate()
        .map(|(i, piece)| (piece.id, i))
        .collect();
    let mut visited = BTreeSet::new();
    let mut vertices = BTreeSet::new();
    let mut expected = Expected::new();
    let mut rings = Vec::new();
    for wire in std::iter::once(outer).chain(holes) {
        let uses = wire
            .uses
            .iter()
            .map(|LoopUse { curve, reversed }| {
                let piece = by_id
                    .get(curve)
                    .copied()
                    .ok_or_else(|| rejected(&format!("known cap missing curve={curve:?}")))?;
                Ok(Use {
                    piece,
                    reversed: *reversed,
                })
            })
            .collect::<Result<Vec<_>, Failure>>()?;
        if uses.len() < 2 {
            return Err(rejected("known cap requires a multi-curve ring"));
        }
        let mut bounds = Bounds::empty();
        for (a, b) in uses.iter().zip(uses.iter().cycle().skip(1)) {
            let one = &pieces[a.piece];
            let two = &pieces[b.piece];
            let ai = usize::from(!a.reversed);
            let bi = usize::from(b.reversed);
            if !visited.insert(a.piece)
                || !vertices.insert(one.ends[1 - ai])
                || one.ends[ai] != two.ends[bi]
            {
                return Err(rejected(&format!(
                    "known cap open or repeated source {:?}/{:?}",
                    one.id, two.id
                )));
            }
            let (i, j, parameters) = if a.piece < b.piece {
                (a.piece, b.piece, [ai as u8, bi as u8])
            } else {
                (b.piece, a.piece, [bi as u8, ai as u8])
            };
            expected.entry((i, j)).or_default().push(parameters);
            bounds.include(one.poles);
        }
        let first = &pieces[uses[0].piece];
        let witness = if uses[0].reversed {
            first.poles[first.poles.len() - 1]
        } else {
            first.poles[0]
        };
        rings.push(Ring {
            uses,
            bounds,
            witness,
        });
    }
    if visited.len() != pieces.len() {
        return Err(rejected("known cap directed cycles must cover every curve"));
    }
    Ok((rings, expected))
}

#[derive(Clone, Copy)]
struct Bounds {
    lo: Point,
    hi: Point,
}
impl Bounds {
    fn empty() -> Self {
        Self {
            lo: [f64::INFINITY; 2],
            hi: [f64::NEG_INFINITY; 2],
        }
    }
    fn of(poles: &[Point]) -> Self {
        let mut result = Self::empty();
        result.include(poles);
        result
    }
    fn include(&mut self, poles: &[Point]) {
        for pole in poles {
            for (i, x) in pole.iter().copied().enumerate() {
                self.lo[i] = self.lo[i].min(x);
                self.hi[i] = self.hi[i].max(x);
            }
        }
    }
    fn excludes(self, point: Point) -> bool {
        (0..2).any(|i| point[i] < self.lo[i] || point[i] > self.hi[i])
    }
    fn separated(self, other: Self) -> bool {
        (0..2).any(|i| self.hi[i] < other.lo[i] || other.hi[i] < self.lo[i])
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), Cause> {
    if cancelled() {
        Err(Cause::Cancelled)
    } else {
        Ok(())
    }
}
