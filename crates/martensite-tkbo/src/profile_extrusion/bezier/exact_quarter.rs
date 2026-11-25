// SPDX-License-Identifier: GPL-2.0-only
//! Only admit analytic quarters whose affine rational controls are exactly representable.
use super::*;
use crate::profile_extrusion::{
    OperationDecline, OperationKind, OperationStage, ProfileCurve, ProfileCurveId, Resolution,
    SketchProfile, admit_profile,
};

/// A complete exact quarter of an authored circular or elliptic arc.
pub struct ExactConicQuarter {
    /// Original endpoints and their common tangent intersection.
    pub poles: [[f64; 2]; 3],
    /// Native angle at the first endpoint, as a signed multiple of pi/2.
    pub first_quarter: i8,
    /// Whether increasing stereographic parameter increases the native angle.
    pub increasing: bool,
}

/// Reuse admitted arc geometry and certify the literal quarter controls and angle map.
/// A nonquarter or a quarter with rounded control coordinates has no exact result.
/// Example: `let quarter = exact_conic_quarter(&profile, curve_id, resolution)?;`.
pub fn exact_conic_quarter(
    profile: &SketchProfile,
    curve_id: ProfileCurveId,
    resolution: Resolution,
) -> Result<Option<ExactConicQuarter>, OperationDecline> {
    if !profile.curves.iter().any(|curve| {
        matches!(curve,
        ProfileCurve::Arc {id, ..} | ProfileCurve::EllipticArc { id, .. } if *id == curve_id)
    }) {
        return Ok(None);
    }
    Ok(checked_quarter(admitted_geometry(
        profile, curve_id, resolution,
    )?))
}

/// Reuse the closed conic carrier and certify all four exact quarter controls.
/// Each quarter has its own original angle map and shares the endpoint with its neighbor.
/// Example: `let quarters = exact_closed_conic(&profile, curve_id, resolution)?;`.
pub fn exact_closed_conic(
    profile: &SketchProfile,
    curve_id: ProfileCurveId,
    resolution: Resolution,
) -> Result<Option<[ExactConicQuarter; 4]>, OperationDecline> {
    if !profile.curves.iter().any(|curve| {
        matches!(curve,
        ProfileCurve::Circle { id, .. } | ProfileCurve::Ellipse { id, .. } if *id == curve_id)
    }) {
        return Ok(None);
    }
    let UseGeometry::Arc { carrier, .. } = admitted_geometry(profile, curve_id, resolution)? else {
        return Ok(None);
    };
    let pieces: Option<Vec<_>> = (0..4).map(|index| closed_quarter(carrier, index)).collect();
    Ok(pieces.and_then(|pieces| pieces.try_into().ok()))
}

fn admitted_geometry(
    profile: &SketchProfile,
    curve: ProfileCurveId,
    resolution: Resolution,
) -> Result<UseGeometry, OperationDecline> {
    let admitted = admit_profile(profile, resolution).map_err(|cause| OperationDecline {
        operation: OperationKind::BuildPlanarRegions,
        stage: OperationStage::Admission,
        cause,
    })?;
    Ok(use_geometry(
        &admitted,
        LoopUse {
            curve,
            reversed: false,
        },
    ))
}

fn closed_quarter(carrier: Conic, index: i8) -> Option<ExactConicQuarter> {
    let start_angle = f64::from(index) * FRAC_PI_2;
    let end_angle = f64::from(index + 1) * FRAC_PI_2;
    let start = carrier.image(quarter_axis(start_angle)?);
    let end = carrier.image(quarter_axis(end_angle)?);
    checked_quarter(UseGeometry::Arc {
        carrier,
        start_angle,
        sweep: FRAC_PI_2,
        start,
        end,
    })
}

fn checked_quarter(geometry: UseGeometry) -> Option<ExactConicQuarter> {
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
    if (sweep / FRAC_PI_2).abs().round() != 1. {
        return None;
    }
    let angles = [start_angle, start_angle + sweep];
    let first = quarter_axis(angles[0])?;
    let last = quarter_axis(angles[1])?;
    let corner = [first[0] + last[0], first[1] + last[1]];
    let mut exact = exact_quarter_controls(carrier, [first, corner, last])?;
    let lowered = quarter_piece(carrier, angles, [start, end])?;
    let matches = lowered.controls == exact;
    [exact[0], exact[2]] = [start, end];
    matches.then_some(ExactConicQuarter {
        poles: exact,
        first_quarter: (start_angle / FRAC_PI_2).round() as i8,
        increasing: sweep > 0.,
    })
}

fn exact_quarter_controls(carrier: Conic, controls: [[f64; 2]; 3]) -> Option<[[f64; 2]; 3]> {
    let (center, first, second) = match carrier {
        Conic::Circle(circle) => (circle.center, [circle.radius, 0.], [0., circle.radius]),
        Conic::Ellipse(ellipse) => {
            if !ellipse.axis.contains(&0.) || !ellipse.axis.iter().any(|axis| axis.abs() == 1.) {
                return None;
            }
            (
                ellipse.center,
                ellipse.axis.map(|axis| ellipse.major * axis),
                ellipse.minor_axis().map(|axis| ellipse.minor * axis),
            )
        }
    };
    let mut exact = [[0.; 2]; 3];
    for ([u, v], point) in controls.into_iter().zip(&mut exact) {
        for axis in 0..2 {
            point[axis] = exact_add(exact_add(center[axis], first[axis] * u)?, second[axis] * v)?;
        }
    }
    Some(exact)
}

fn exact_add(a: f64, b: f64) -> Option<f64> {
    let sum = a + b;
    let recovered_b = sum - a;
    let recovered_a = sum - recovered_b;
    // TwoSum recovers the binary64 addition residual, including its sign.
    let residual = (a - recovered_a) + (b - recovered_b);
    (sum.is_finite() && residual == 0.).then_some(sum)
}
