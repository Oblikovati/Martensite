// SPDX-License-Identifier: GPL-2.0-only
//! Exact pair enumeration and source-keyed endpoint checking.
use super::*;

pub(super) fn certify_pairs(
    pieces: &[Piece<'_>],
    expected: &Expected,
    resolution: Resolution,
    work: &mut DirectedCapWork,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Cause> {
    let mut roots = RootIsolationOptions::new(0.001);
    roots.max_bisections = work.requested.pair_bisections;
    let mut signs = BezierPairSignOptions::derived();
    signs.version = 2;
    let mut remaining = BezierPairPolicy {
        version: 2,
        roots,
        signs,
        requested_world_error: resolution.weld,
    };
    let captured = pieces
        .iter()
        .map(|piece| {
            CapturedBezierCurve::capture(BezierPairCurve {
                controls: piece.poles,
                weights: None,
                interval: [0., 1.],
            })
            .map_err(|cause| Cause::Pair {
                curves: [piece.id; 2],
                detail: DirectedCapPairCause::Source(cause),
            })
        })
        .collect::<Result<Vec<_>, Cause>>()?;
    let bounds = pieces
        .iter()
        .map(|piece| Bounds::of(piece.poles))
        .collect::<Vec<_>>();
    let mut joints = 0;
    for i in 0..pieces.len() {
        poll(cancelled)?;
        let solo = CapturedBezierPair::from_captured_self(&captured[i]);
        compare_pair(
            solo,
            [pieces[i].id; 2],
            &[],
            &mut remaining,
            work,
            cancelled,
        )?;
        work.pair_domains += 1;
        for j in i + 1..pieces.len() {
            if bounds[i].separated(bounds[j]) {
                work.pair_domains += 1;
                continue;
            }
            let query = CapturedBezierPair::from_captured_pair(&captured[i], &captured[j]);
            let wanted = expected.get(&(i, j)).map_or(&[][..], Vec::as_slice);
            compare_pair(
                query,
                [pieces[i].id, pieces[j].id],
                wanted,
                &mut remaining,
                work,
                cancelled,
            )?;
            work.pair_domains += 1;
            joints += wanted.len();
        }
    }
    if joints != pieces.len() {
        return Err(Cause::Topology(rejected(
            "known cap exact pair census omitted an authored join",
        )));
    }
    Ok(())
}

fn compare_pair(
    query: CapturedBezierPair,
    ids: [ProfileCurveId; 2],
    wanted: &[[u8; 2]],
    remaining: &mut BezierPairPolicy,
    work: &mut DirectedCapWork,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Cause> {
    let geometric = query.into_geometric_report(*remaining, &mut *cancelled);
    let actual = if let Some(report) = geometric.complete() {
        report
            .contact()
            .into_iter()
            .map(|contact| contact.parameters())
            .collect::<Vec<_>>()
    } else {
        if geometric.completion() != BezierPairGeometricCompletion::Inconclusive {
            return Err(Cause::Pair {
                curves: ids,
                detail: DirectedCapPairCause::Geometric(Box::new(geometric)),
            });
        }
        let report = geometric.source().certify(*remaining, cancelled);
        work.pair.consumed_pair_bisections += u64::from(report.work().completed);
        remaining.roots.max_bisections = remaining
            .roots
            .max_bisections
            .checked_sub(report.work().completed)
            .ok_or(Cause::Accounting)?;
        let Some(proof) = report.complete() else {
            return Err(Cause::Pair {
                curves: ids,
                detail: DirectedCapPairCause::Algebraic(Box::new(report)),
            });
        };
        work.carrier_pairs += usize::from(proof.common_carrier().is_some());
        (0..proof.contact_count())
            .map(|i| {
                let contact = proof.contact(i).ok_or(Cause::Accounting)?;
                let point = contact.point;
                endpoint(contact).ok_or(Cause::Pair {
                    curves: ids,
                    detail: DirectedCapPairCause::Interior { point },
                })
            })
            .collect::<Result<Vec<_>, Cause>>()?
    };
    if actual.len() != wanted.len() || wanted.iter().any(|pair| !actual.contains(pair)) {
        return Err(Cause::Pair {
            curves: ids,
            detail: DirectedCapPairCause::Contacts {
                expected: wanted.to_vec(),
                actual,
            },
        });
    }
    Ok(())
}

fn endpoint(contact: CertifiedBezierPairContact<'_>) -> Option<[u8; 2]> {
    let (lo, hi) = contact.parameter.bounds();
    let one = [0, 1]
        .into_iter()
        .find(|&x| lo == hi && lo.compare_f64(f64::from(x)).is_ok_and(|r| r.is_eq()))?;
    let paired = contact.exact_paired_parameter;
    let two = [0, 1].into_iter().find(|&x| {
        paired.compare_lower(f64::from(x)).is_ok_and(|r| r.is_eq())
            && paired.compare_upper(f64::from(x)).is_ok_and(|r| r.is_eq())
    })?;
    Some([one, two])
}
