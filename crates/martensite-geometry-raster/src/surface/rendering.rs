// Generic surface drawing retained from GeometrySurface.
//! Geometry-only rendering borrows actual sampling and surface controls.
use super::SurfaceStyle;
use crate::{
    CurveSampling, curve_properties::finite_range, geometry_access::checked_geometry as geometry,
};
use martensite_raster_core::RasterDisplay as DrawDisplay;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dIsoCurve, Adaptor3dSurface},
    geom::{GeomSurface, SurfaceBounds},
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkmath::{geom_abs::IsoType, gp::Point3};
/// Live surface geometry is shared; the rendering record is Send but not Sync.
/// ```compile_fail
/// use martensite_geometry_raster::SurfaceRendering;
/// fn shared<T: Sync>() {}
/// shared::<SurfaceRendering>();
/// ```
pub struct SurfaceRendering {
    surface: Handle<dyn GeomSurface>,
    // Live rendering stays confined even when geometry handles are shared.
    thread_confined: std::marker::PhantomData<std::cell::Cell<()>>,
}
impl SurfaceRendering {
    /// Retains the live surface; `SurfaceRendering::new(surface)`.
    pub fn new(surface: Handle<dyn GeomSurface>) -> Self {
        Self {
            surface,
            thread_confined: std::marker::PhantomData,
        }
    }
    /// Shares the retained geometry; `let surface = rendering.surface();`.
    pub fn surface(&self) -> Handle<dyn GeomSurface> {
        self.surface.clone()
    }
    /// Borrows without increasing reference count; `rendering.surface_ref()`.
    pub fn surface_ref(&self) -> &Handle<dyn GeomSurface> {
        &self.surface
    }
    /// Draws isos, finite sides and orientation in their existing order; `rendering.draw_on(sampling,style,display,true)?`.
    pub fn draw_on(
        &self,
        sampling: &CurveSampling,
        style: &SurfaceStyle,
        display: &mut DrawDisplay<'_>,
        with_isos: bool,
    ) -> Result<(), Failure> {
        let (declared, bounds, iso) = self.iso_geometry()?;
        if with_isos {
            display.set_color(style.isos_color());
            draw_isos(sampling, style, &iso, bounds, display)?;
        }
        display.set_color(style.bounds_color());
        draw_boundaries(sampling, &iso, bounds, declared, display)?;
        draw_orientation(sampling, &iso, bounds, display)
    }
    fn iso_geometry(&self) -> Result<(SurfaceBounds, SurfaceBounds, Adaptor3dIsoCurve), Failure> {
        let surface = geometry(&self.surface)?;
        let declared = surface.bounds();
        let bounds = finite_bounds(surface, declared)?;
        let adaptor = GeomAdaptorSurface::default();
        adaptor.load_bounds(&self.surface, bounds)?;
        let iso =
            Adaptor3dIsoCurve::with_surface(&Handle::new(adaptor).cast::<dyn Adaptor3dSurface>());
        Ok((declared, bounds, iso))
    }
}
fn draw_isos(
    sampling: &CurveSampling,
    style: &SurfaceStyle,
    iso: &Adaptor3dIsoCurve,
    (u1, u2, v1, v2): SurfaceBounds,
    display: &mut DrawDisplay<'_>,
) -> Result<(), Failure> {
    let (nb_u, nb_v) = style.nb_isos();
    let u = (IsoType::IsoU, (u1, u2), (v1, v2), nb_u);
    let v = (IsoType::IsoV, (v1, v2), (u1, u2), nb_v);
    draw_direction_isos(sampling, iso, u, display)?;
    draw_direction_isos(sampling, iso, v, display)
}
type IsoDirection = (IsoType, (f64, f64), (f64, f64), i32);
fn draw_direction_isos(
    sampling: &CurveSampling,
    iso: &Adaptor3dIsoCurve,
    (kind, (first, last), range, count): IsoDirection,
    display: &mut DrawDisplay<'_>,
) -> Result<(), Failure> {
    let delta = (last - first) / f64::from(count + 1);
    let mut parameter = first;
    for _ in 1..=count {
        parameter += delta;
        sampling.draw_iso_curve_on(iso, kind, parameter, range, display)?;
    }
    Ok(())
}
type SurfaceSide = (bool, IsoType, f64, (f64, f64));
fn boundary_sides((u1, u2, v1, v2): SurfaceBounds, declared: SurfaceBounds) -> [SurfaceSide; 4] {
    let (u1_inf, u2_inf, v1_inf, v2_inf) = infinite_sides(declared);
    [
        (u1_inf, IsoType::IsoU, u1, (v1, v2)),
        (u2_inf, IsoType::IsoU, u2, (v1, v2)),
        (v1_inf, IsoType::IsoV, v1, (u1, u2)),
        (v2_inf, IsoType::IsoV, v2, (u1, u2)),
    ]
}
fn draw_boundaries(
    sampling: &CurveSampling,
    iso: &Adaptor3dIsoCurve,
    bounds: SurfaceBounds,
    declared: SurfaceBounds,
    display: &mut DrawDisplay<'_>,
) -> Result<(), Failure> {
    for (infinite, kind, parameter, range) in boundary_sides(bounds, declared) {
        if !infinite {
            sampling.draw_iso_curve_on(iso, kind, parameter, range, display)?;
        }
    }
    Ok(())
}
fn draw_orientation(
    sampling: &CurveSampling,
    iso: &Adaptor3dIsoCurve,
    (u1, u2, v1, v2): SurfaceBounds,
    display: &mut DrawDisplay<'_>,
) -> Result<(), Failure> {
    let mark = (v1, v1 + (v2 - v1) / 10.);
    sampling.draw_iso_curve_on(iso, IsoType::IsoU, u1 + (u2 - u1) / 10., mark, display)
}
/// Which of `(U1, U2, V1, V2)` are infinite.
fn infinite_sides((u1, u2, v1, v2): SurfaceBounds) -> (bool, bool, bool, bool) {
    (
        precision::is_negative_infinite(u1),
        precision::is_positive_infinite(u2),
        precision::is_negative_infinite(v1),
        precision::is_positive_infinite(v2),
    )
}

/// The bounds drawn for a surface: an infinite U range is made finite along the iso
/// V at the middle (or at the finite V end, or 0), then an infinite V range along
/// the iso U at the middle of the (finite) U range.
fn finite_bounds(
    surface: &dyn GeomSurface,
    declared: SurfaceBounds,
) -> Result<SurfaceBounds, Failure> {
    let (u1, u2, v1, v2) = declared;
    let (_, _, v1_inf, v2_inf) = infinite_sides(declared);
    let v = match (v1_inf, v2_inf) {
        (true, true) => 0.0,
        (true, false) => v2,
        (false, true) => v1,
        (false, false) => (v1 + v2) / 2.,
    };
    let (u1, u2) = finite_range::<Point3>((u1, u2), |u| surface.d0(u, v))?;
    let u = (u1 + u2) / 2.;
    let (v1, v2) = finite_range::<Point3>((v1, v2), |v| surface.d0(u, v))?;
    Ok((u1, u2, v1, v2))
}

#[cfg(test)]
#[path = "../../tests/surface_support/bounds_cases.rs"]
mod bounds_cases;
