//! One sampling algorithm over either historical or owned bounds storage.
use super::super::ContapHContTool;
use martensite_tkernel::standard::{
    Failure,
    real::{real_first, real_last},
};
use martensite_tkg3d::adaptor3d::{Adaptor3dSurface, h_surface_tool::Adaptor3dSurfaceHandle};
use martensite_tkmath::geom_abs::SurfaceType;

pub(in crate::contap) trait SamplingBounds {
    fn sampled_range(&self) -> [f64; 4];
    fn store_bound(&mut self, index: usize, value: f64);
    fn replace_bounds(&mut self, bounds: [f64; 4]);
}
pub(super) fn live_surface(
    surface: &Adaptor3dSurfaceHandle,
) -> Result<&dyn Adaptor3dSurface, Failure> {
    surface.get().ok_or_else(|| {
        Failure::with_message(
            "Standard_NullObject",
            "Contap_HContTool: null surface; expected live Adaptor3d_Surface",
        )
    })
}
fn capture_bounds(
    surface: &Adaptor3dSurfaceHandle,
    storage: &mut impl SamplingBounds,
) -> Result<[f64; 4], Failure> {
    let surface = live_surface(surface)?;
    // Store each result immediately: native failures retain already-written statics.
    storage.store_bound(0, surface.first_u_parameter()?);
    storage.store_bound(2, surface.last_u_parameter()?);
    storage.store_bound(1, surface.first_v_parameter()?);
    storage.store_bound(3, surface.last_v_parameter()?);
    let [u0, v0, u1, v1] = storage.sampled_range();
    let (u0, u1) = finite_range(u0, u1);
    let (v0, v1) = finite_range(v0, v1);
    let bounds = [u0, v0, u1, v1];
    storage.replace_bounds(bounds);
    Ok(bounds)
}
/// A range made finite for sampling: an infinite side is 2e5 from the other one, both
/// infinite sides give `[-1e5, 1e5]`.
fn finite_range(mut inf: f64, mut sup: f64) -> (f64, f64) {
    if sup < inf {
        (inf, sup) = (sup, inf);
    }
    if inf == real_first() && sup == real_last() {
        return (-1.0e5, 1.0e5);
    }
    if inf == real_first() {
        inf = sup - 2.0e5;
    } else if sup == real_last() {
        sup = inf + 2.0e5;
    }
    (inf, sup)
}

/// The B-spline sample count: knots times degree, at least 2.
pub(super) fn spline_samples(nb_knots: i32, degree: i32) -> i32 {
    let nbs = nb_knots.wrapping_mul(degree);
    if nbs < 2 { 2 } else { nbs }
}

pub(in crate::contap) fn sample_count(
    s: &Adaptor3dSurfaceHandle,
    storage: &mut impl SamplingBounds,
) -> Result<i32, Failure> {
    let [uinf, vinf, usup, vsup] = capture_bounds(s, storage)?;
    if s.get_type()? != SurfaceType::BSplineSurface {
        return Ok(5);
    }
    let m = (ContapHContTool::nb_samples_u(s, uinf, usup)? / 3)
        .wrapping_mul(ContapHContTool::nb_samples_v(s, vinf, vsup)? / 3);
    Ok(if m > 5 { m } else { 5 })
}
pub(in crate::contap) fn sample_point(
    s: &Adaptor3dSurfaceHandle,
    index: i32,
    storage: &impl SamplingBounds,
) -> Result<(f64, f64), Failure> {
    let kind = live_surface(s)?.get_type()?;
    let [uinf, vinf, usup, vsup] = storage.sampled_range();
    if kind == SurfaceType::BSplineSurface
        && let Some(point) = spline_grid_point(s, index, [uinf, vinf, usup, vsup])?
    {
        return Ok(point);
    }
    Ok(match index {
        1 => (0.75 * uinf + 0.25 * usup, 0.75 * vinf + 0.25 * vsup),
        2 => (0.75 * uinf + 0.25 * usup, 0.25 * vinf + 0.75 * vsup),
        3 => (0.25 * uinf + 0.75 * usup, 0.75 * vinf + 0.25 * vsup),
        4 => (0.25 * uinf + 0.75 * usup, 0.25 * vinf + 0.75 * vsup),
        _ => (0.5 * (uinf + usup), 0.5 * (vinf + vsup)),
    })
}
fn spline_grid_point(
    surface: &Adaptor3dSurfaceHandle,
    index: i32,
    bounds: [f64; 4],
) -> Result<Option<(f64, f64)>, Failure> {
    let [uinf, vinf, usup, vsup] = bounds;
    let nu = ContapHContTool::nb_samples_u(surface, uinf, usup)? / 3;
    let nv = ContapHContTool::nb_samples_v(surface, vinf, vsup)? / 3;
    if nu.wrapping_mul(nv) <= 5 {
        return Ok(None);
    }
    let iu = index.wrapping_sub(1) / nu;
    let iv = index.wrapping_sub(1) - iu * nu;
    let u = uinf + ((usup - uinf) / f64::from(nu + 1)) * f64::from(iu + 1);
    let v = vinf + ((vsup - vinf) / f64::from(nv + 2)) * f64::from(iv + 1);
    Ok(Some((u, v)))
}
