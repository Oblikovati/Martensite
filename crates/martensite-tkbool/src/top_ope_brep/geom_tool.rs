//! Curves and piecewise linear pcurves of intersection lines.
use super::{TopOpeBRepLineInter, TopOpeBRepTypeLineCurve as Kind};
use crate::{top_ope_brep_ds::TopOpeBRepDSCurve, top_ope_brep_tool::TopOpeBRepToolCurveTool};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    ncollection::Array1,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkmath::gp::{Point2, Point3};

/// Builds geometry from the original analytic or sampled intersection line.
/// Example: `TopOpeBRepGeomTool::make_curve(0., 1., &mut line)?`.
pub struct TopOpeBRepGeomTool;
impl TopOpeBRepGeomTool {
    /// Constructs the curve, preserving the line's analytic bounds; `Self::make_curve(0., 1., &mut line)?`.
    pub fn make_curve(
        min: f64,
        max: f64,
        line: &mut TopOpeBRepLineInter,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        match line.type_line_curve() {
            Kind::Walking => Self::make_bspline1_from_walking3d(line),
            Kind::Line | Kind::Circle | Kind::Ellipse => line.curve(),
            Kind::Parabola | Kind::Hyperbola => line.trimmed_curve(min, max),
            kind => Err(Failure::with_message(
                "Standard_ProgramError",
                &format!(
                    "TopOpeBRepGeomTool::MakePrivateCurves: type {kind:?}; expected walking or conic"
                ),
            )),
        }
    }
    /// Writes the DS curve and walking pcurves, keeping nonwalking outputs untouched; `Self::make_curves(0., 1., &mut line, &s1, &s2, &mut curve, &mut pc1, &mut pc2)?`.
    // Preserve MakeCurves inputs and its three independently mutable outputs.
    #[allow(clippy::too_many_arguments)]
    pub fn make_curves(
        min: f64,
        max: f64,
        line: &mut TopOpeBRepLineInter,
        _first: &Shape,
        _second: &Shape,
        curve: &mut TopOpeBRepDSCurve,
        pc1: &mut Handle<dyn Geom2dCurve>,
        pc2: &mut Handle<dyn Geom2dCurve>,
    ) -> Result<(), Failure> {
        let walking = line.type_line_curve() == Kind::Walking;
        let spatial = Self::make_curve(min, max, line)?;
        if walking {
            set_walking_pcurves(line, curve, pc1, pc2)?;
        }
        curve.define_curve(&spatial, curve.tolerance(), walking);
        curve.set_range(min, max);
        Ok(())
    }
    /// Interpolates sampled spatial points at integer parameters; `Self::make_bspline1_from_walking3d(&mut line)?`.
    pub fn make_bspline1_from_walking3d(
        line: &mut TopOpeBRepLineInter,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let count = line.nb_wpoint()?;
        let mut points = Array1::<Point3>::with_bounds(1, count).map_err(|error| {
            Failure::new(&format!(
                "walking point count {count}: expected positive length; {error:?}"
            ))
        })?;
        for index in 1..=count {
            points
                .set_value(index, line.wpoint(index)?.value()?)
                .map_err(|error| {
                    Failure::new(&format!(
                        "walking point index {index}: expected allocated array slot; {error:?}"
                    ))
                })?;
        }
        TopOpeBRepToolCurveTool::make_bspline1_from_pnt(&points)
    }
    /// Interpolates first or second surface parameters; `Self::make_bspline1_from_walking2d(&mut line, 1)?`.
    pub fn make_bspline1_from_walking2d(
        line: &mut TopOpeBRepLineInter,
        surface: i32,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let count = line.nb_wpoint()?;
        let mut points = Array1::<Point2>::with_bounds(1, count).map_err(|error| {
            Failure::new(&format!(
                "walking point count {count}: expected positive length; {error:?}"
            ))
        })?;
        for index in 1..=count {
            let point = line.wpoint(index)?;
            let value = match surface {
                1 => point.value_on_s1()?,
                2 => point.value_on_s2()?,
                _ => Point2::default(),
            };
            points.set_value(index, value).map_err(|error| {
                Failure::new(&format!(
                    "walking point index {index}: expected allocated array slot; {error:?}"
                ))
            })?;
        }
        TopOpeBRepToolCurveTool::make_bspline1_from_pnt2d(&points)
    }
}

fn set_walking_pcurves(
    line: &mut TopOpeBRepLineInter,
    curve: &TopOpeBRepDSCurve,
    first: &mut Handle<dyn Geom2dCurve>,
    second: &mut Handle<dyn Geom2dCurve>,
) -> Result<(), Failure> {
    *first = TopOpeBRepGeomTool::make_bspline1_from_walking2d(line, 1)?;
    *second = TopOpeBRepGeomTool::make_bspline1_from_walking2d(line, 2)?;
    if !first.is_null() {
        curve.set_curve1(first)?;
    }
    if !second.is_null() {
        curve.set_curve2(second)?;
    }
    Ok(())
}
