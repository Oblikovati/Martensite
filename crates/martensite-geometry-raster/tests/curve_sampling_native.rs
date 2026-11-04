//! Owned sampling preserves the pinned mathematical curve and iso workloads.
#![cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
#[path = "sampling_capture/mod.rs"]
mod sampling_capture;
use martensite_geometry_raster::CurveSampling;
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::standard::Failure;
use sampling_capture::{Geometry, SamplingDriver};
impl SamplingDriver for CurveSampling {
    fn configured(mode: i32) -> Self {
        Self::new(30, 0.01, mode)
    }
    fn draw_geometry(
        &self,
        geometry: &Geometry,
        display: &mut RasterDisplay<'_>,
    ) -> Result<(), Failure> {
        match geometry {
            Geometry::Spatial(curve) => self.draw_curve_on(curve, display),
            Geometry::Planar(curve) => self.draw_curve2d_on(curve.as_ref(), display),
            Geometry::Iso(curve, kind, parameter, range) => {
                self.draw_iso_curve_on(curve, *kind, *parameter, *range, display)
            }
        }
    }
}
#[test]
fn curve_and_surface_iso_sampling_capture_matrix() {
    sampling_capture::capture_matrix::<CurveSampling>();
}

#[test]
fn unloaded_sampler_geometry_keeps_typed_failures() {
    sampling_capture::unloaded_geometry_failures::<CurveSampling>();
}
