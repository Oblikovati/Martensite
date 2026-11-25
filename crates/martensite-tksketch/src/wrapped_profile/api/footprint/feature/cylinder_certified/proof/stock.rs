//! Prove clearance to both preserved stock rims and the single admitted periodic cut.
use super::{sides::CapTraces, *};

pub(super) fn close(
    attempt: Attempt,
    input: &Admitted,
    tool: &Tool,
    candidate: &Candidate,
    traces: &CapTraces,
    contact: f64,
) -> Checked<()> {
    let volume = support_window(attempt, input, traces, candidate.height_sign)?;
    require(
        attempt,
        VALIDATION,
        volume.rim_gap > contact && volume.far_gap > contact,
        "entire wall and cap family separated from both original end disks and the exposed cap separated from the stock band",
    )?;
    let [u0, u1] = volume.angle;
    let [min, max] = candidate.stock_sheet;
    let angle_gap = (u0 - min)
        .next_down()
        .min((max - u1).next_down())
        .min(std::f64::consts::PI.next_down());
    let (lower, sweep) = radii(tool);
    let angular_phase =
        ((2. * lower * angle_gap).next_down() / std::f64::consts::PI.next_up()).next_down();
    let angular_phase = candidate
        .recut_clearance
        .map_or(angular_phase, |cut| angular_phase.min(cut));
    let active = (((contact + candidate.native_stock_error).next_up() + volume.near_error)
        .next_up()
        + (2. * sweep).next_up())
    .next_up();
    require(
        attempt,
        VALIDATION,
        lower > 0. && angle_gap > 0. && angular_phase > active,
        "strict chart clearance to the preserved or certified re-cut native seam over the entire contact domain",
    )
}

struct Extents {
    rim_gap: f64,
    far_gap: f64,
    near_error: f64,
    angle: [f64; 2],
}

fn support_window(
    attempt: Attempt,
    input: &Admitted,
    traces: &CapTraces,
    height_sign: f64,
) -> Checked<Extents> {
    let mut angle = [f64::INFINITY, f64::NEG_INFINITY];
    let mut height = angle;
    let mut far_gap = f64::INFINITY;
    let mut near_error = 0_f64;
    for (depth, cap) in traces.iter().enumerate() {
        for trace in cap.values() {
            for &[u, v] in &trace.uv {
                extend(&mut angle, u);
                extend(&mut height, height_sign * v);
            }
            for &[_, _, z] in &trace.poles {
                extend(&mut height, height_sign * z);
            }
            if depth == 0 {
                near_error = near_error.max(trace.proof.spatial_residual_bound());
            } else {
                let gap = match attempt.kind {
                    CylinderFeatureKind::Boss => trace.radius - input.support.radius,
                    CylinderFeatureKind::Pocket => input.support.radius - trace.radius,
                };
                far_gap = far_gap.min(gap.next_down());
            }
        }
    }
    let [z0, z1] = input.axial;
    let rim_gap = (height[0] - z0)
        .next_down()
        .min((z1 - height[1]).next_down());
    require(
        attempt,
        VALIDATION,
        height.into_iter().chain(angle).all(f64::is_finite) && far_gap.is_finite(),
        "complete finite exact cap and wall hulls on the admitted stock",
    )?;
    Ok(Extents {
        rim_gap,
        far_gap,
        near_error,
        angle,
    })
}

fn extend(range: &mut [f64; 2], value: f64) {
    range[0] = range[0].min(value);
    range[1] = range[1].max(value);
}

fn radii(tool: &Tool) -> (f64, f64) {
    tool.spans
        .values()
        .fold((f64::INFINITY, 0_f64), |(radius, sweep), span| {
            let source = &span.storage.certificate;
            let lower = (source.stored_radii()[0] - source.radial_distance_bounds()[0]).next_down();
            (
                radius.min(lower),
                sweep.max(source.normal_sweep_error_bound()),
            )
        })
}
