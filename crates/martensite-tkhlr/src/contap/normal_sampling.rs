//! Shared native normal-scale updates, retaining partial state on failures.
use super::{ContapHContTool, ContapSurfProps};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkmath::gp::Point3;
pub(super) trait NormalSampler {
    fn sample_count(&mut self, surface: &Adaptor3dSurfaceHandle) -> Result<i32, Failure>;
    fn sample_point(
        &self,
        surface: &Adaptor3dSurfaceHandle,
        index: i32,
    ) -> Result<(f64, f64), Failure>;
}
struct HistoricalSampler;
impl NormalSampler for HistoricalSampler {
    fn sample_count(&mut self, surface: &Adaptor3dSurfaceHandle) -> Result<i32, Failure> {
        ContapHContTool::nb_sample_points(surface)
    }
    fn sample_point(
        &self,
        surface: &Adaptor3dSurfaceHandle,
        index: i32,
    ) -> Result<(f64, f64), Failure> {
        ContapHContTool::sample_point(surface, index)
    }
}
pub(super) fn sample_normal_scale(
    surface: &Adaptor3dSurfaceHandle,
    mean: &mut f64,
    point: &mut Point3,
) -> Result<(), Failure> {
    sample_normal_scale_using(&mut HistoricalSampler, surface, mean, point)
}
pub(super) fn sample_normal_scale_using(
    sampler: &mut impl NormalSampler,
    surface: &Adaptor3dSurfaceHandle,
    mean: &mut f64,
    point: &mut Point3,
) -> Result<(), Failure> {
    let count = sampler.sample_count(surface)?;
    if count <= 0 {
        return Ok(());
    }
    *mean = 0.;
    for index in 1..=count {
        let (u, v) = sampler.sample_point(surface, index)?;
        let (sample, normal) = ContapSurfProps::normale(surface, u, v)?;
        *point = sample;
        *mean += normal.magnitude();
    }
    *mean /= f64::from(count);
    Ok(())
}
