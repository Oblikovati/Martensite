// SPDX-License-Identifier: GPL-2.0-only
//! Exact cycle orientation and checked inter-cycle nesting.
use super::*;

pub(super) fn certify_rings(
    pieces: &[Piece<'_>],
    rings: &[Ring],
    work: &mut DirectedCapWork,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Cause> {
    for (i, ring) in rings.iter().enumerate() {
        poll(cancelled)?;
        let sign = green(pieces, ring, i, work, cancelled)?;
        if sign != expected_sign(i) {
            return Err(Cause::Orientation {
                ring: i,
                expected: expected_sign(i),
                actual: sign,
            });
        }
        for (j, other) in rings.iter().enumerate() {
            if i == j {
                continue;
            }
            let inside = !other.bounds.excludes(ring.witness)
                && winding(pieces, other, ring.witness, [i, j], work, cancelled)?;
            let expected = i > 0 && j == 0;
            if inside != expected {
                return Err(Cause::Nesting {
                    ring: i,
                    against: j,
                    expected,
                });
            }
        }
    }
    Ok(())
}
fn expected_sign(index: usize) -> i64 {
    if index == 0 { 1 } else { -1 }
}

fn green(
    pieces: &[Piece<'_>],
    ring: &Ring,
    index: usize,
    work: &mut DirectedCapWork,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<i64, Cause> {
    let curves = ring
        .uses
        .iter()
        .map(|part| OrientedMetricCurve {
            curve: MetricCurve::Bezier(MetricBezier {
                poles: pieces[part.piece].poles,
                weights: None,
            }),
            reversed: part.reversed,
        })
        .collect::<Vec<_>>();
    let integration = RationalIntegralOptions::default();
    let parts = curves
        .chunks(usize::from(integration.max_curves))
        .collect::<Vec<_>>();
    let options = IntegralProgramOptions {
        version: IntegralProgramVersion::V1,
        integration,
        max_components: work.requested.max_components,
        max_total_subintervals: work
            .effective_integration_cells
            .checked_sub(work.integration_cells)
            .ok_or(Cause::Accounting)?,
    };
    let query = BoundaryIntegralProgramQuery {
        components: &parts,
        options,
    };
    let result = certify_boundary_integral_program(query, cancelled);
    let completed = result.as_ref().map_or_else(
        |error| error.integration_subintervals,
        |proof| proof.integration_subintervals(),
    );
    work.integration_cells += completed;
    let proof = result.map_err(|cause| Cause::Integral {
        ring: index,
        cause: Box::new(cause),
    })?;
    let [lo, hi] = proof.enclosure();
    let sign = if lo > 0. {
        1
    } else if hi < 0. {
        -1
    } else {
        0
    };
    Ok(sign)
}

fn winding(
    pieces: &[Piece<'_>],
    ring: &Ring,
    point: Point,
    [owner, against]: [usize; 2],
    work: &mut DirectedCapWork,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<bool, Cause> {
    let occurrences = ring
        .uses
        .iter()
        .map(|part| {
            let piece = &pieces[part.piece];
            CapturedBezierWireOccurrence {
                id: piece.id.0,
                input: CapturedBezierWireInput::capture(piece.id.0, piece.poles, None, [0., 1.]),
                reversed: part.reversed,
            }
        })
        .collect::<Vec<_>>();
    let ray = [point, [point[0] + 1., point[1]]];
    // A fresh store per witness keeps its wire budget independent and exposes failed work.
    let mut store = CapturedBezierWireStore::new(ray, RootIsolationOptions::new(0.001));
    let result = store.classify(&occurrences, cancelled);
    let observed = result
        .as_ref()
        .map_or_else(|failure| failure.work(), |proof| proof.work());
    work.winding_bisections += u64::from(observed.operation_bisections);
    let proof = result.map_err(|cause| Cause::Winding {
        ring: owner,
        against,
        cause: Box::new(cause),
    })?;
    match proof.location() {
        BezierWireLocation::Outside => Ok(false),
        BezierWireLocation::Inside { winding } if *winding == expected_sign(against) => Ok(true),
        other => Err(Cause::Location {
            ring: owner,
            against,
            location: other.clone(),
        }),
    }
}
