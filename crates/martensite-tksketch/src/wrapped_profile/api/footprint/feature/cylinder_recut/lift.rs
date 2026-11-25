//! Keep the authored angular lift and move only the native full-turn cut.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    FrameTransportInput, certify_frame_transport_at_angle,
};
use std::f64::consts::{PI, TAU};

#[derive(Debug)]
pub(super) struct Cut {
    pub(super) range: [f64; 2],
    pub(super) residual: f64,
    pub(super) clear_distance: f64,
}

impl Cut {
    pub(super) fn opposite(
        attempt: Attempt,
        hull: [f64; 2],
        radius: f64,
        stock_error: f64,
        clearance: f64,
    ) -> FeatureChecked<Self> {
        let range = if hull[0] > PI {
            [PI, 3. * PI]
        } else {
            [-PI, PI]
        };
        if !hull.iter().all(|u| u.is_finite())
            || hull[0] > hull[1]
            || sum_with_residual(range[0], TAU) != (range[1], 0.)
        {
            return Err(refuse(
                attempt,
                "ordered finite angular hull in a full-turn sheet",
            ));
        }
        let low_phase_error = angle_error(attempt, range[0])?;
        let high_phase_error = angle_error(attempt, range[1])?;
        let phase_bound = low_phase_error.max(high_phase_error);
        let residual = ((radius * phase_bound).next_up() + stock_error).next_up();
        let gap = (hull[0] - range[0])
            .next_down()
            .min((range[1] - hull[1]).next_down());
        // At most one radian is needed here. On [0,1], 2 sin(angle/2) >= angle/2.
        let stable_gap = (gap - phase_bound).next_down().min(1.);
        let clear_distance = (radius * (0.5 * stable_gap).next_down()).next_down();
        if !residual.is_finite()
            || !clear_distance.is_finite()
            || clear_distance <= (clearance + residual).next_up()
        {
            return Err(refuse(
                attempt,
                "strict clearance from both images of the replacement seam",
            ));
        }
        Ok(Self {
            range,
            residual,
            clear_distance,
        })
    }
}

fn angle_error(attempt: Attempt, angle: f64) -> FeatureChecked<f64> {
    let canonical = FrameTransportInput {
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        stored: [[-1., 0., 0.], [0., -1., 0.], [0., 0., 1.]],
    };
    let proof = certify_frame_transport_at_angle(canonical, angle)
        .map_err(|_| refuse(attempt, "certified trigonometry at the opposite generator"))?;
    let rotation = proof.rotation();
    if rotation.cosine_bounds[1] >= -0.5 {
        return Err(refuse(
            attempt,
            "stored cut angle adjacent to the opposite generator",
        ));
    }
    // In this angular sector the distance to the nearest odd multiple of pi is
    // at most twice the absolute sine. The trig proof used the exact stored angle.
    let sine = rotation.sine_bounds[0]
        .abs()
        .max(rotation.sine_bounds[1].abs());
    Ok((2. * sine).next_up())
}
