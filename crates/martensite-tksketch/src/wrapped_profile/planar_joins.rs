//! Exact zero-margin join exclusions; positive metric margins remain a separate proof.
use super::*;
use martensite_tkmath::predicates::{NonFiniteCoordinate, OrientationSign, orient2d};

#[derive(Debug)]
pub(in super::super) enum JoinFailure {
    Coordinate(NonFiniteCoordinate),
    Stationary {
        curve: SurfaceCurveId,
        endpoint: usize,
        point: [f64; 2],
    },
    TangentReturn {
        vertex: CornerSource,
        curves: [SurfaceCurveId; 2],
        controls: [[f64; 2]; 3],
    },
}
impl std::fmt::Display for JoinFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coordinate(cause) => write!(f, "join coordinate: {cause}"),
            Self::Stationary {
                curve,
                endpoint,
                point,
            } => stationary_message(f, *curve, *endpoint, *point),
            Self::TangentReturn {
                vertex,
                curves,
                controls,
            } => write!(
                f,
                "vertex={vertex:?}, curves={curves:?}, branch controls={controls:?}; expected outward branches with a nonzero opening or regular through-join"
            ),
        }
    }
}

pub(super) fn admit(request: &DevelopedRegionRequest) -> Result<(), JoinFailure> {
    for contour in &request.loops {
        for (i, piece) in contour.iter().enumerate() {
            let next = &contour[(i + 1) % contour.len()];
            join(piece, next)?;
        }
    }
    Ok(())
}

fn join(piece: &AuthoredPiece, next: &AuthoredPiece) -> Result<(), JoinFailure> {
    let count = piece.poles.len();
    let before = piece.poles[count - 2];
    let vertex = piece.poles[count - 1];
    let after = next.poles[1];
    nonstationary(piece.span.curve, 1, before, vertex)?;
    nonstationary(next.span.curve, 0, after, vertex)?;
    if orient2d(before, vertex, after).map_err(JoinFailure::Coordinate)? != OrientationSign::Zero {
        return Ok(());
    }
    if same_ray(before, vertex, after) {
        return Err(JoinFailure::TangentReturn {
            vertex: piece.end,
            curves: [piece.span.curve, next.span.curve],
            controls: [before, vertex, after],
        });
    }
    Ok(())
}

fn nonstationary(
    curve: SurfaceCurveId,
    endpoint: usize,
    adjacent: [f64; 2],
    point: [f64; 2],
) -> Result<(), JoinFailure> {
    if adjacent == point {
        return Err(JoinFailure::Stationary {
            curve,
            endpoint,
            point,
        });
    }
    Ok(())
}

fn same_ray(before: [f64; 2], vertex: [f64; 2], after: [f64; 2]) -> bool {
    let axis = usize::from(before[0] == vertex[0]);
    (before[axis] < vertex[axis]) == (after[axis] < vertex[axis])
}

fn stationary_message(
    f: &mut std::fmt::Formatter<'_>,
    curve: SurfaceCurveId,
    endpoint: usize,
    point: [f64; 2],
) -> std::fmt::Result {
    write!(
        f,
        "curve={curve:?}, endpoint={endpoint}, repeated adjacent pole={point:?}; expected nonzero endpoint derivative"
    )
}
