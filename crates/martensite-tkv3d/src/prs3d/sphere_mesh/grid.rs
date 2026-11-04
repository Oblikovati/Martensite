use std::f64::consts::PI;
pub(super) struct SphereGrid {
    pub(super) theta_res: i32,
    pub(super) phi_res: i32,
    pub(super) j_start: i32,
    pub(super) j_end: i32,
    pub(super) delta_phi: f64,
    pub(super) delta_theta: f64,
    pub(super) start_theta: f64,
    pub(super) base: i32,
}
/// The native mesh layout of the whole sphere at a resolution (at least 4).
pub(super) fn sphere_grid(res: i32) -> SphereGrid {
    let res = res.max(4);
    let (start_phi, end_phi) = (0.0_f64, 180.0_f64);
    let (local_start, local_end, start_theta, end_theta, mut theta_res) = angular_limits(res);
    let phi_res = res - 2;
    let delta_phi = (end_phi * (PI / 180.0) - start_phi * (PI / 180.0)) / f64::from(res - 1);
    let theta_bands = theta_res;
    if (local_start - local_end).abs() < 360.0 {
        theta_res += 1;
    }
    let local_end = end_theta;
    let spacing = (
        theta_res,
        delta_phi,
        (local_end - start_theta) / f64::from(theta_bands),
    );
    assemble_grid(res, phi_res, local_start, spacing)
}

fn angular_limits(res: i32) -> (f64, f64, f64, f64, i32) {
    let delta = 360.0 / f64::from(res);
    let (piece, pieces) = (-1, 1);
    let (start, end) = (piece * res / pieces, (piece + 1) * res / pieces);
    let local_end = f64::from(end) * delta;
    let local_start = f64::from(start) * delta;
    let theta_res = end - start;
    let start_theta = local_start.min(local_end) * (PI / 180.0);
    let end_theta = local_end.max(local_start) * (PI / 180.0);
    (local_start, local_end, start_theta, end_theta, theta_res)
}

fn assemble_grid(res: i32, phi_res: i32, local_start: f64, spacing: (i32, f64, f64)) -> SphereGrid {
    let (theta_res, delta_phi, delta_theta) = spacing;
    SphereGrid {
        theta_res,
        phi_res,
        j_start: 1,
        j_end: res - 1,
        delta_phi,
        delta_theta,
        start_theta: local_start * PI / 180.0,
        base: phi_res * theta_res,
    }
}
