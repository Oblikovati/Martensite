//! Shared source/owned sampler workloads; no command interpreter or session.
mod errors;
mod fixtures;
mod pixels;
mod probe;
pub use fixtures::Geometry;
use martensite_raster_core::RasterDisplay;
use martensite_tkernel::standard::Failure;

pub trait SamplingDriver: Sized + 'static {
    fn configured(mode: i32) -> Self;
    fn draw_geometry(
        &self,
        geometry: &Geometry,
        display: &mut RasterDisplay<'_>,
    ) -> Result<(), Failure>;
}
pub fn capture_matrix<S: SamplingDriver>() {
    for (label, geometry, mode) in fixtures::curve_matrix() {
        pixels::capture::<S>(&label, geometry, mode);
    }
    for (label, geometry, mode) in fixtures::iso_matrix() {
        pixels::capture::<S>(&label, geometry, mode);
    }
}

pub use errors::unloaded_geometry_failures;
