// SPDX-License-Identifier: GPL-2.0-only
//! Capture the actual sewn conic pieces consumed by the planar region proof.
use super::exact_quarter::admitted_geometry;
use super::*;
use crate::profile_extrusion::{
    OperationDecline, ProfileCurve, ProfileCurveId, Resolution, SketchProfile,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    AdmittedConicFrame, AdmittedConicPhase, AdmittedConicSpan, ConicSpanParameter,
};

/// Capture the admitted analytic angle and exact literal pieces of a conic use.
/// Example: `let pieces = admitted_conic_spans(profile, curve, resolution)?;`.
pub fn admitted_conic_spans(
    profile: &SketchProfile,
    curve: ProfileCurveId,
    resolution: Resolution,
) -> Result<Option<Vec<AdmittedConicSpan>>, OperationDecline> {
    let Some(closed) = conic_kind(profile, curve) else {
        return Ok(None);
    };
    let geometry = admitted_geometry(profile, curve, resolution)?;
    Ok(capture(geometry, closed))
}

fn conic_kind(profile: &SketchProfile, id: ProfileCurveId) -> Option<bool> {
    match profile.curves.iter().find(|curve| curve.id() == id)? {
        ProfileCurve::Arc { .. } | ProfileCurve::EllipticArc { .. } => Some(false),
        ProfileCurve::Circle { .. } | ProfileCurve::Ellipse { .. } => Some(true),
        _ => None,
    }
}

fn capture(geometry: UseGeometry, closed: bool) -> Option<Vec<AdmittedConicSpan>> {
    let UseGeometry::Arc {
        carrier,
        start_angle,
        sweep,
        start,
        end,
    } = geometry
    else {
        return None;
    };
    let pieces = arc_pieces(carrier, [start_angle, sweep], [start, end]);
    let count = u8::try_from(pieces.len()).ok()?;
    if closed && count != 4 {
        return None;
    }
    let angles = [start_angle, sweep];
    let frame = frame(carrier);
    pieces
        .into_iter()
        .zip(0..count)
        .map(|(piece, index)| capture_piece(piece, frame, angles, [index, count], closed))
        .collect()
}

fn capture_piece(
    piece: BezierPiece,
    frame: AdmittedConicFrame,
    [first, sweep]: [f64; 2],
    [index, count]: [u8; 2],
    closed: bool,
) -> Option<AdmittedConicSpan> {
    let poles = piece.controls.try_into().ok()?;
    let weights = piece.weights?.try_into().ok()?;
    let quarter = weights == [1., 1., 2.];
    let phase = if closed && quarter {
        AdmittedConicPhase::Quarter(index)
    } else if closed {
        return None;
    } else {
        let parameter = if quarter {
            ConicSpanParameter::FirstEndpoint
        } else {
            ConicSpanParameter::Symmetric
        };
        AdmittedConicPhase::Sweep {
            first,
            sweep,
            index,
            count,
            parameter,
        }
    };
    Some(AdmittedConicSpan {
        frame,
        phase,
        poles,
        weights,
    })
}

fn frame(carrier: Conic) -> AdmittedConicFrame {
    match carrier {
        Conic::Circle(circle) => AdmittedConicFrame {
            center: circle.center,
            directions: [[1., 0.], [0., 1.]],
            scales: [circle.radius; 2],
        },
        Conic::Ellipse(ellipse) => AdmittedConicFrame {
            center: ellipse.center,
            directions: [ellipse.axis, ellipse.minor_axis()],
            scales: [ellipse.major, ellipse.minor],
        },
    }
}
