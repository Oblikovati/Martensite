//! Prove ordered material cycles from freshly decoded stock pcurves.
mod leaves;
use super::{Image, SurfaceRegionId, UvInput};
use leaves::*;
use martensite_tkbo::profile_extrusion::{
    ProfileCurve, ProfileCurveId, ProfileVertexId, Resolution,
};
use martensite_tkmath::predicates::roots::{
    RootIsolationOptions, bezier_line::*, bezier_pair::*, metric_bounds::*,
};
use std::collections::{BTreeMap, BTreeSet};

type Point = [f64; 2];
type RegionKey = (SurfaceRegionId, Image);
pub(super) const PIECE_LIMIT: usize = 2048;
const PAIR_LIMIT: u16 = 8192;
const AREA_LIMIT: u64 = 100_000;

#[derive(Debug)]
pub(in super::super) enum Failure {
    Cancelled,
    Input(String),
    Pair([ProfileCurveId; 2], String),
    Orientation(RegionKey, String),
    Containment(RegionKey, String),
}
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(f, "known UV proof cancelled"),
            Self::Input(reason) => write!(f, "source incidence: {reason}"),
            Self::Pair(ids, reason) => write!(f, "pair {ids:?}: {reason}"),
            Self::Orientation(id, reason) => write!(f, "orientation {id:?}: {reason}"),
            Self::Containment(id, reason) => write!(f, "containment {id:?}: {reason}"),
        }
    }
}

#[derive(Default)]
pub(super) struct Stats {
    logical: usize,
    residual: usize,
    pair_work: u16,
    loops: usize,
    exact_windings: usize,
    winding_work: u32,
    integration: u64,
}
impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "logical={} residual={} pair_work={} loops={} exact_windings={} winding_work={} integration={}",
            self.logical,
            self.residual,
            self.pair_work,
            self.loops,
            self.exact_windings,
            self.winding_work,
            self.integration
        )
    }
}

pub(super) struct Receipt {
    pub(super) stats: Stats,
}

impl Receipt {
    pub(super) fn issue(
        input: &UvInput,
        resolution: Resolution,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, Failure> {
        let capture = Program::capture(input, &mut cancelled)?;
        let mut stats = Stats {
            loops: capture.rings.len(),
            ..Default::default()
        };
        capture.prove_pairs(resolution.weld, &mut stats, &mut cancelled)?;
        capture.prove_orientation(&mut stats, &mut cancelled)?;
        capture.prove_containment(&mut stats, &mut cancelled)?;
        Ok(Self { stats })
    }

    pub(super) fn pair_domains(&self) -> usize {
        self.stats.logical
    }
}

struct Piece<'p> {
    id: ProfileCurveId,
    ends: [ProfileVertexId; 2],
    poles: &'p [Point; 4],
    bounds: Bounds,
}
impl<'p> Piece<'p> {
    fn capture(curve: &'p ProfileCurve) -> Result<(Self, &'p [f64]), Failure> {
        let ProfileCurve::Spline {
            id,
            start,
            end,
            degree: 3,
            poles,
            weights: None,
            knots,
        } = curve
        else {
            return Err(Failure::Input(
                "expected a full-domain unweighted cubic".into(),
            ));
        };
        let poles: &'p [Point; 4] = poles
            .as_slice()
            .try_into()
            .map_err(|_| Failure::Input(format!("piece {id:?}: expected four controls")))?;
        let full_domain = [0.; 4].into_iter().chain([1.; 4]).map(f64::to_bits);
        if !knots.iter().map(|v| v.to_bits()).eq(full_domain)
            || !poles.iter().flatten().all(|v| v.is_finite())
            || start == end
        {
            return Err(Failure::Input(format!(
                "piece {id:?}: expected finite full-domain nonclosed cubic"
            )));
        }
        Ok((
            Self {
                id: *id,
                ends: [*start, *end],
                poles,
                bounds: Bounds::capture(poles),
            },
            knots,
        ))
    }
}

struct Ring {
    key: RegionKey,
    parts: Vec<usize>,
    bounds: Bounds,
    witness: Point,
}
impl Ring {
    fn expected_sign(&self) -> i64 {
        if self.key.1 == Image::SupportRemainder {
            -1
        } else {
            1
        }
    }
    fn expected_container(&self) -> Option<RegionKey> {
        (self.key.1 != Image::SupportRemainder).then_some((self.key.0, Image::SupportRemainder))
    }
}

struct Program<'p> {
    pieces: Vec<Piece<'p>>,
    rings: Vec<Ring>,
    expected: BTreeMap<[usize; 2], Vec<[u8; 2]>>,
}
impl<'p> Program<'p> {
    fn capture(input: &'p UvInput, cancelled: &mut impl FnMut() -> bool) -> Result<Self, Failure> {
        let pieces = input
            .profile
            .curves
            .iter()
            .map(|curve| Piece::capture(curve).map(|(piece, _)| piece))
            .collect::<Result<Vec<_>, _>>()?;
        let by_id: BTreeMap<_, _> = pieces
            .iter()
            .enumerate()
            .map(|(i, piece)| (piece.id, i))
            .collect();
        if pieces.is_empty() || pieces.len() != by_id.len() || pieces.len() > PIECE_LIMIT {
            return Err(Failure::Input(format!(
                "expected 1..=2048 pieces with distinct IDs, got {} / {}",
                pieces.len(),
                by_id.len()
            )));
        }
        let mut sources = BTreeSet::new();
        let mut vertices = BTreeSet::new();
        let mut keys = BTreeSet::new();
        let mut expected = BTreeMap::<[usize; 2], Vec<[u8; 2]>>::new();
        let mut rings = Vec::new();
        for item in &input.loops {
            poll(cancelled)?;
            let key = (item.region, item.owner);
            if !keys.insert((item.region, item.owner == Image::SupportRemainder))
                || !matches!(item.owner, Image::SupportRemainder)
                    && item.owner != Image::SupportCounterIsland(item.region)
            {
                return Err(Failure::Input(format!(
                    "invalid or repeated native wire owner {key:?}"
                )));
            }
            let parts = item
                .curves
                .iter()
                .map(|id| {
                    by_id.get(id).copied().ok_or_else(|| {
                        Failure::Input(format!("wire {key:?} missing source {id:?}"))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if parts.len() < 2 {
                return Err(Failure::Input(format!("expected multi-piece wire {key:?}")));
            }
            let first = &pieces[parts[0]];
            let mut bounds = first.bounds;
            for (&a, &b) in parts.iter().zip(parts.iter().cycle().skip(1)) {
                let (a_piece, b_piece) = (&pieces[a], &pieces[b]);
                if !sources.insert(a)
                    || !vertices.insert(a_piece.ends[0])
                    || a_piece.ends[1] != b_piece.ends[0]
                    || bits(a_piece.poles[3]) != bits(b_piece.poles[0])
                {
                    return Err(Failure::Input(format!(
                        "invalid ordered incidence for {key:?}, {:?} to {:?}",
                        a_piece.id, b_piece.id
                    )));
                }
                let (pair, endpoint) = if a < b {
                    ([a, b], [1, 0])
                } else {
                    ([b, a], [0, 1])
                };
                expected.entry(pair).or_default().push(endpoint);
                bounds = bounds.union(a_piece.bounds);
            }
            rings.push(Ring {
                key,
                parts,
                bounds,
                witness: first.poles[0],
            });
        }
        if sources.len() != pieces.len() {
            return Err(Failure::Input(format!(
                "wire coverage {} of {}",
                sources.len(),
                pieces.len()
            )));
        }
        Ok(Self {
            pieces,
            rings,
            expected,
        })
    }

    fn prove_pairs(
        &self,
        world: f64,
        stats: &mut Stats,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        let n = self.pieces.len();
        let pairs = sweep(&self.pieces, cancelled)?;
        stats.logical = n * (n + 1) / 2;
        stats.residual = pairs.len() + n;
        let captured = self
            .pieces
            .iter()
            .map(|piece| {
                CapturedBezierCurve::capture(BezierPairCurve {
                    controls: piece.poles,
                    weights: None,
                    interval: [0., 1.],
                })
                .map_err(|err| Failure::Pair([piece.id; 2], format!("capture {err:?}")))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut requested = policy(world);
        let mut pairs = pairs.into_iter().peekable();
        let mut joined = 0;
        for a in 0..n {
            let solo = CapturedBezierPair::from_captured_self(&captured[a]);
            self.check_pair([a, a], solo, &mut requested, cancelled)?;
            while let Some(&[first, b]) = pairs.peek() {
                if first != a {
                    break;
                }
                pairs.next();
                let query = CapturedBezierPair::from_captured_pair(&captured[a], &captured[b]);
                joined += self.check_pair([a, b], query, &mut requested, cancelled)?;
            }
        }
        if joined != n {
            return Err(Failure::Input(format!(
                "certified {joined} of {n} native joins"
            )));
        }
        stats.pair_work = PAIR_LIMIT - requested.roots.max_bisections;
        Ok(())
    }

    fn check_pair(
        &self,
        pair: [usize; 2],
        query: CapturedBezierPair,
        policy: &mut BezierPairPolicy,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<usize, Failure> {
        let ids = pair.map(|i| self.pieces[i].id);
        poll(cancelled)?;
        let mut actual =
            contacts(query, policy, cancelled).map_err(|why| Failure::Pair(ids, why))?;
        let mut expected = self.expected.get(&pair).cloned().unwrap_or_default();
        actual.sort_unstable();
        expected.sort_unstable();
        if actual != expected {
            return Err(Failure::Pair(
                ids,
                format!("expected endpoints {expected:?}, observed {actual:?}"),
            ));
        }
        Ok(actual.len())
    }

    fn prove_orientation(
        &self,
        stats: &mut Stats,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        let integration = RationalIntegralOptions::default();
        for ring in &self.rings {
            let curves = ring
                .parts
                .iter()
                .map(|&part| OrientedMetricCurve {
                    curve: MetricCurve::Bezier(MetricBezier {
                        poles: self.pieces[part].poles,
                        weights: None,
                    }),
                    reversed: false,
                })
                .collect::<Vec<_>>();
            let components = curves
                .chunks(usize::from(integration.max_curves))
                .collect::<Vec<_>>();
            let options = IntegralProgramOptions {
                version: IntegralProgramVersion::V1,
                integration,
                max_components: 64,
                max_total_subintervals: AREA_LIMIT - stats.integration,
            };
            let query = BoundaryIntegralProgramQuery {
                components: &components,
                options,
            };
            let proof = certify_boundary_integral_program(query, &mut *cancelled)
                .map_err(|error| Failure::Orientation(ring.key, error.to_string()))?;
            let [low, high] = proof.enclosure();
            let sign = if low > 0. {
                1
            } else if high < 0. {
                -1
            } else {
                0
            };
            if sign != ring.expected_sign() {
                return Err(Failure::Orientation(
                    ring.key,
                    format!(
                        "expected sign {}, got enclosure {low}..{high}",
                        ring.expected_sign()
                    ),
                ));
            }
            stats.integration += proof.integration_subintervals();
        }
        Ok(())
    }

    fn prove_containment(
        &self,
        stats: &mut Stats,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), Failure> {
        for ring in &self.rings {
            poll(cancelled)?;
            let mut actual = Vec::new();
            for candidate in &self.rings {
                if ring.key == candidate.key || candidate.bounds.excludes(ring.witness) {
                    continue;
                }
                let proof = self.winding(candidate, ring.witness, cancelled)?;
                stats.exact_windings += 1;
                stats.winding_work += u32::from(proof.bisections());
                match proof.location() {
                    BezierWireLocation::Outside => (),
                    BezierWireLocation::Inside { winding }
                        if *winding == candidate.expected_sign() =>
                    {
                        actual.push(candidate.key)
                    }
                    other => {
                        return Err(Failure::Containment(
                            ring.key,
                            format!(
                                "candidate {:?}, expected off-wire zero or signed unit, got {other:?}",
                                candidate.key
                            ),
                        ));
                    }
                }
            }
            let expected = ring.expected_container();
            if !actual.iter().copied().eq(expected) {
                return Err(Failure::Containment(
                    ring.key,
                    format!("expected {expected:?}, observed {actual:?}"),
                ));
            }
        }
        Ok(())
    }

    fn winding(
        &self,
        candidate: &Ring,
        point: Point,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<BezierWireResult, Failure> {
        let segments = candidate
            .parts
            .iter()
            .map(|&part| {
                let piece = &self.pieces[part];
                BezierWireSegment {
                    id: piece.id.0,
                    controls: piece.poles,
                    weights: None,
                    interval: [0., 1.],
                    reversed: false,
                }
            })
            .collect::<Vec<_>>();
        let query = BezierWireQuery {
            segments: &segments,
            line: [point, [point[0] + 1., point[1]]],
            options: RootIsolationOptions::new(0.001),
        };
        classify_bezier_wire(query, cancelled)
            .map_err(|cause| Failure::Containment(candidate.key, cause.to_string()))
    }
}
