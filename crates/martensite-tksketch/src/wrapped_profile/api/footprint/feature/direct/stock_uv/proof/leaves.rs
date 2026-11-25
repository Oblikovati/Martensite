//! Isolate residual curve pair and winding domains.
use super::*;

pub(super) fn policy(world: f64) -> BezierPairPolicy {
    let mut signs = BezierPairSignOptions::derived();
    signs.version = 2;
    let mut roots = RootIsolationOptions::new(0.001);
    roots.max_bisections = PAIR_LIMIT;
    BezierPairPolicy {
        version: 2,
        roots,
        signs,
        requested_world_error: world,
    }
}

pub(super) fn contacts(
    query: CapturedBezierPair,
    policy: &mut BezierPairPolicy,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<[u8; 2]>, String> {
    let geometric = query.into_geometric_report(*policy, &mut *cancelled);
    if let Some(proof) = geometric.complete() {
        return Ok(proof
            .contact()
            .into_iter()
            .map(|contact| contact.parameters())
            .collect());
    }
    if geometric.completion() != BezierPairGeometricCompletion::Inconclusive {
        return Err(format!("geometric proof {:?}", geometric.failure()));
    }
    let report = geometric.source().certify(*policy, cancelled);
    policy.roots.max_bisections = policy
        .roots
        .max_bisections
        .checked_sub(report.work().completed)
        .ok_or("inconsistent pair bisections")?;
    let proof = report
        .complete()
        .ok_or_else(|| format!("algebraic proof {:?}", report.failure()))?;
    (0..proof.contact_count())
        .map(|i| {
            let contact = proof.contact(i).ok_or("missing exact contact")?;
            let (low, high) = contact.parameter.bounds();
            let a = [0u8, 1].into_iter().find(|&v| {
                low == high && low.compare_f64(f64::from(v)).is_ok_and(|cmp| cmp.is_eq())
            });
            let exact = contact.exact_paired_parameter;
            let b = [0u8, 1].into_iter().find(|&v| {
                exact
                    .compare_lower(f64::from(v))
                    .is_ok_and(|cmp| cmp.is_eq())
                    && exact
                        .compare_upper(f64::from(v))
                        .is_ok_and(|cmp| cmp.is_eq())
            });
            a.zip(b)
                .map(|(a, b)| [a, b])
                .ok_or_else(|| format!("interior or uncertified endpoint {:?}", contact.point))
        })
        .collect()
}

pub(super) fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), Failure> {
    if cancelled() {
        Err(Failure::Cancelled)
    } else {
        Ok(())
    }
}
pub(super) fn bits(point: Point) -> [u64; 2] {
    point.map(f64::to_bits)
}

#[derive(Clone, Copy)]
pub(super) struct Bounds {
    pub(super) lo: Point,
    pub(super) hi: Point,
}
impl Bounds {
    pub(super) fn capture(poles: &[Point; 4]) -> Self {
        Self {
            lo: std::array::from_fn(|a| poles.iter().map(|p| p[a]).fold(f64::INFINITY, f64::min)),
            hi: std::array::from_fn(|a| {
                poles.iter().map(|p| p[a]).fold(f64::NEG_INFINITY, f64::max)
            }),
        }
    }
    pub(super) fn union(self, other: Self) -> Self {
        Self {
            lo: std::array::from_fn(|a| self.lo[a].min(other.lo[a])),
            hi: std::array::from_fn(|a| self.hi[a].max(other.hi[a])),
        }
    }
    pub(super) fn excludes(self, point: Point) -> bool {
        (0..2).any(|a| point[a] < self.lo[a] || point[a] > self.hi[a])
    }
    pub(super) fn separated(self, other: Self, axis: usize) -> bool {
        self.hi[axis] < other.lo[axis] || other.hi[axis] < self.lo[axis]
    }
}

pub(super) fn sweep(
    pieces: &[Piece<'_>],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<[usize; 2]>, Failure> {
    let mut order: Vec<_> = (0..pieces.len()).collect();
    order.sort_by(|&a, &b| {
        pieces[a].bounds.lo[0]
            .total_cmp(&pieces[b].bounds.lo[0])
            .then(a.cmp(&b))
    });
    let mut active = Vec::<usize>::new();
    let mut pairs = Vec::new();
    for i in order {
        poll(cancelled)?;
        let current = pieces[i].bounds;
        active.retain(|&j| pieces[j].bounds.hi[0] >= current.lo[0]);
        for &j in &active {
            if !current.separated(pieces[j].bounds, 1) {
                pairs.push([i.min(j), i.max(j)]);
            }
        }
        active.push(i);
    }
    pairs.sort_unstable();
    Ok(pairs)
}
