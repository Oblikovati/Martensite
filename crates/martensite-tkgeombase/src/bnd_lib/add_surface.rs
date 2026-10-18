//! `AddSurface`: add the box of a surface adaptor to a box, through
//! Surface.
use crate::geom_bnd_lib::Surface;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkmath::bnd::BoundingBox3;

/// `AddSurface`: the box of an `Adaptor3dSurface` added to a `BoundingBox3`.
pub struct AddSurface;

// `[FirstU, LastU, FirstV, LastV]` of the adaptor.
fn natural_patch(surface: &dyn Adaptor3dSurface) -> Result<[f64; 4], Failure> {
    Ok([
        surface.first_u_parameter()?,
        surface.last_u_parameter()?,
        surface.first_v_parameter()?,
        surface.last_v_parameter()?,
    ])
}

impl AddSurface {
    /// `Add(S, Tol, B)`: the adaptor bounds; `AddSurface::add(&adaptor, 0.0, &mut bounds)?`.
    pub fn add(
        surface: &dyn Adaptor3dSurface,
        tolerance: f64,
        bounds: &mut BoundingBox3,
    ) -> Result<(), Failure> {
        Self::add_range(surface, natural_patch(surface)?, tolerance, bounds)
    }
    /// `Add(S, UMin, UMax, VMin, VMax, Tol, B)`: `Surface(S).Add(...)`;
    /// `AddSurface::add_range(&adaptor, [0.0, 1.0, 0.0, 1.0], 0.0, &mut bounds)?`.
    pub fn add_range(
        surface: &dyn Adaptor3dSurface,
        patch: [f64; 4],
        tolerance: f64,
        bounds: &mut BoundingBox3,
    ) -> Result<(), Failure> {
        Surface::from_adaptor(surface)?.add_range(patch, tolerance, bounds)
    }
    /// `AddOptimal(S, Tol, B)`; `AddSurface::add_optimal(&adaptor, 0.0, &mut bounds)?`.
    pub fn add_optimal(
        surface: &dyn Adaptor3dSurface,
        tolerance: f64,
        bounds: &mut BoundingBox3,
    ) -> Result<(), Failure> {
        Self::add_optimal_range(surface, natural_patch(surface)?, tolerance, bounds)
    }
    /// `AddOptimal(S, UMin, UMax, VMin, VMax, Tol, B)`: `Surface(S).AddOptimal(...)`;
    /// `AddSurface::add_optimal_range(&adaptor, [0.0, 1.0, 0.0, 1.0], 0.0, &mut bounds)?`.
    pub fn add_optimal_range(
        surface: &dyn Adaptor3dSurface,
        patch: [f64; 4],
        tolerance: f64,
        bounds: &mut BoundingBox3,
    ) -> Result<(), Failure> {
        Surface::from_adaptor(surface)?.add_optimal_range(patch, tolerance, bounds)
    }
}
