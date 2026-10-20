//! Owned contour sampling bounds, separate from historical process-wide sampling.
use super::h_cont_tool::sampling::range::{self, SamplingBounds};
use super::normal_sampling::NormalSampler;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;

/// Operation-local sampling bounds. Pass one context through a surface or arc setup.
///
/// ```
/// use martensite_tkhlr::contap::ContapSamplingContext;
/// let context = ContapSamplingContext::new();
/// assert_eq!(context.bounds(), [0.; 4]);
/// ```
/// This owns only sampling bounds. It does not certify ownership or thread safety
/// of surface adaptors, geometry handles, or operations that retain those handles.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContapSamplingContext {
    bounds: [f64; 4],
}

impl ContapSamplingContext {
    /// Start with zero bounds; `let context = ContapSamplingContext::new();`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inspect retained U-first, V-first, U-last, V-last bounds, including partial failure state.
    /// `assert_eq!(ContapSamplingContext::new().bounds(), [0.; 4]);`.
    pub fn bounds(&self) -> [f64; 4] {
        self.bounds
    }

    /// Capture the surface bounds locally and return the sampling count.
    /// `let count = context.nb_sample_points(&surface)?;`.
    /// Failed callbacks retain earlier writes; no historical global state is touched.
    pub fn nb_sample_points(&mut self, surface: &Adaptor3dSurfaceHandle) -> Result<i32, Failure> {
        range::sample_count(surface, self)
    }

    /// Sample the retained local range with the surface's existing grid rule.
    /// `let (u, v) = context.sample_point(&surface, 1)?;`.
    /// As with historical sampling, callers capture the intended range before sampling.
    pub fn sample_point(
        &self,
        surface: &Adaptor3dSurfaceHandle,
        index: i32,
    ) -> Result<(f64, f64), Failure> {
        range::sample_point(surface, index, self)
    }
}

impl SamplingBounds for ContapSamplingContext {
    fn sampled_range(&self) -> [f64; 4] {
        self.bounds
    }
    fn store_bound(&mut self, index: usize, value: f64) {
        self.bounds[index] = value;
    }
    fn replace_bounds(&mut self, bounds: [f64; 4]) {
        self.bounds = bounds;
    }
}

impl NormalSampler for ContapSamplingContext {
    fn sample_count(&mut self, surface: &Adaptor3dSurfaceHandle) -> Result<i32, Failure> {
        self.nb_sample_points(surface)
    }
    fn sample_point(
        &self,
        surface: &Adaptor3dSurfaceHandle,
        index: i32,
    ) -> Result<(f64, f64), Failure> {
        self.sample_point(surface, index)
    }
}
