//

//! `GeometryDrawable`: the discretisation settings every geometric drawable shares
//! and the curve plotting they use.
use martensite_raster_core::RasterDisplay as DrawDisplay;
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dIsoCurve};
use martensite_tkgeombase::gcpnts::UniformDeflection;
use martensite_tkmath::geom_abs::IsoType;
use martensite_tkmath::gp::Point3;
use std::cell::Cell;

mod iso;
mod planar;
mod spatial;

/// Curve sampling state (the draw mode, the
/// number of points per interval and the deflection), independent of drawable identity.
///
/// ```
/// use martensite_geometry_raster::CurveSampling;
/// let drawable = CurveSampling::new(16, 0.01, 1);
/// drawable.set_discretisation(30);
/// assert_eq!((drawable.discretisation(), drawable.draw_mode()), (30, 1));
/// ```
#[derive(Debug)]
pub struct CurveSampling {
    draw_mode: Cell<i32>,
    discret: Cell<i32>,
    deflection: Cell<f64>,
}

impl CurveSampling {
    /// The protected constructor `GeometryDrawable(discret, deflection, DrawMode)`
    /// (the C++ defaults are 0.01 and 0).
    /// Creates independent settings; `CurveSampling::new(30, 0.01, 0)`.
    pub fn new(discret: i32, deflection: f64, draw_mode: i32) -> Self {
        Self {
            draw_mode: Cell::new(draw_mode),
            discret: Cell::new(discret),
            deflection: Cell::new(deflection),
        }
    }

    /// `SetDiscretisation`.
    /// Chooses samples per interval; `sampling.set_discretisation(30)`.
    pub fn set_discretisation(&self, discret: i32) {
        self.discret.set(discret);
    }

    /// `GetDiscretisation`.
    /// Reads the interval count; `let count = sampling.discretisation();`.
    pub fn discretisation(&self) -> i32 {
        self.discret.get()
    }

    /// `SetDeflection`.
    /// Sets model-space chord control; `sampling.set_deflection(0.01)`.
    pub fn set_deflection(&self, deflection: f64) {
        self.deflection.set(deflection);
    }

    /// `GetDeflection`.
    /// Reads chord control; `let limit = sampling.deflection();`.
    pub fn deflection(&self) -> f64 {
        self.deflection.get()
    }

    /// `SetDrawMode`: 1 draws by uniform deflection, 0 by `discret` points per interval.
    /// Selects sampling behavior; `sampling.set_draw_mode(1)`.
    pub fn set_draw_mode(&self, draw_mode: i32) {
        self.draw_mode.set(draw_mode);
    }

    /// `GetDrawMode`.
    /// Reads sampling mode; `let mode = sampling.draw_mode();`.
    pub fn draw_mode(&self) -> i32 {
        self.draw_mode.get()
    }

    /// `DrawCurve2dOn`: plots a 2D curve, by uniform deflection in draw mode 1, else
    /// with `discret` steps on each CN interval (lines get only their end points).
    /// Usage: `sampling.draw_curve2d_on(&curve, &mut display)?;`.
    pub fn draw_curve2d_on(
        &self,
        curve: &dyn Adaptor2dCurve2d,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        planar::draw_curve(self, curve, display)
    }

    /// `DrawCurveOn`: plots a 3D curve, by uniform deflection in draw mode 1, else
    /// adaptively per CN interval (circles and ellipses by equal steps, other curves by
    /// double steps refined while the chord midpoint is off the curve).
    /// Usage: `sampling.draw_curve_on(&curve, &mut display)?;`.
    pub fn draw_curve_on(
        &self,
        curve: &dyn Adaptor3dCurve,
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        spatial::draw_curve(self, curve, display)
    }

    /// `DrawIsoCurveOn`: loads the iso `iso` at `parameter` on `[first, last]` and plots
    /// it; a Bezier or BSpline iso is plotted through a `GeomAdaptorCurve` on it.
    /// Usage: `sampling.draw_iso_curve_on(&iso, kind, u, (a, b), &mut display)?;`.
    pub fn draw_iso_curve_on(
        &self,
        curve: &Adaptor3dIsoCurve,
        iso: IsoType,
        parameter: f64,
        (first, last): (f64, f64),
        display: &mut DrawDisplay<'_>,
    ) -> Result<(), Failure> {
        iso::draw_curve(self, curve, (iso, parameter), (first, last), display)
    }
}

/// Draws the points of a uniform deflection: a move to the first, lines to the others.
fn plot_deflection<P, D>(
    line: &UniformDeflection,
    display: &mut DrawDisplay<'_>,
    mut move_to: P,
    mut draw_to: D,
) -> Result<(), Failure>
where
    P: FnMut(&mut DrawDisplay<'_>, Point3),
    D: FnMut(&mut DrawDisplay<'_>, Point3),
{
    if !line.is_done() {
        return Ok(());
    }
    move_to(display, line.value(1)?);
    plot_deflection_tail(line, display, &mut draw_to)
}
fn plot_deflection_tail<D>(
    line: &UniformDeflection,
    display: &mut DrawDisplay<'_>,
    draw_to: &mut D,
) -> Result<(), Failure>
where
    D: FnMut(&mut DrawDisplay<'_>, Point3),
{
    for index in 2..=line.nb_points()? {
        draw_to(display, line.value(index)?);
    }
    Ok(())
}

pub(crate) use martensite_raster_core::sampling::{bound, interval_bounds};

#[cfg(test)]
mod tests;
