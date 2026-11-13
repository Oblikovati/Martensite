//! Canonical dyadic intervals are chosen before native construction, never by retries.
use super::*;

pub(super) fn prepare(request: &Request) -> Request {
    let mut prepared = request.clone();
    for _ in 0..request.fit_level {
        prepared.loops = prepared
            .loops
            .iter()
            .map(|contour| contour.iter().flat_map(split_piece).collect())
            .collect();
    }
    prepared
}

fn split_piece(piece: &AuthoredPiece) -> [AuthoredPiece; 2] {
    let level = piece.span.level + 1;
    let first = piece.span.index * 2;
    let joint = CornerSource::Split {
        curve: piece.span.curve,
        numerator: first + 1,
        level,
    };
    let [left, right] = split_poles(&piece.poles);
    let make = |index, start, end, poles| AuthoredPiece {
        span: CurveSpan {
            curve: piece.span.curve,
            level,
            index,
        },
        start,
        end,
        poles,
    };
    [
        make(first, piece.start, joint, left),
        make(first + 1, joint, piece.end, right),
    ]
}

fn split_poles(poles: &[[f64; 2]]) -> [Vec<[f64; 2]>; 2] {
    let mut work = poles.to_vec();
    let mut left = vec![work[0]];
    let mut right = vec![work[work.len() - 1]];
    for count in (1..work.len()).rev() {
        for i in 0..count {
            work[i] = std::array::from_fn(|k| work[i][k] / 2. + work[i + 1][k] / 2.);
        }
        left.push(work[0]);
        right.push(work[count - 1]);
    }
    right.reverse();
    [left, right]
}
