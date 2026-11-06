// Ranged drawing retained from GeometryBSplineCurve.
//! Exact span and neighborhood traversal, including existing endpoint choices.
use martensite_raster_core::RasterDisplay as DrawDisplay;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
/// The minimum number of points of each span of the ranged drawing.
const MIN_SPAN_POINTS: i32 = 30;
/// The control polygon around pole `index`: every pole for 0, the first or last leg
/// at the ends, the two legs through the pole otherwise.
pub(super) fn draw_pole_neighbourhood(
    display: &mut DrawDisplay<'_>,
    bspline: &GeomBSplineCurve,
    index: i32,
) -> Result<(), Failure> {
    let count = bspline.nb_poles();
    let legs = pole_legs(index, count);
    let mut first = true;
    for i in legs {
        let pole = bspline.pole(i)?;
        if first {
            display.move_to_3d(pole);
            first = false;
        } else {
            display.draw_to_3d(pole);
        }
    }
    Ok(())
}

fn pole_legs(index: i32, count: i32) -> std::ops::RangeInclusive<i32> {
    match index {
        0 => 1..=count,
        1 => 1..=2,
        i if i == count => count - 1..=count,
        i => i - 1..=i + 1,
    }
}

/// The curve part of the ranged drawing: the parts before the first and after the
/// last knot of the range, then the knot spans, each with a number of points
/// proportional to its length (at least [`MIN_SPAN_POINTS`]).
pub(super) struct RangeSpans<'a> {
    pub(super) bspline: &'a GeomBSplineCurve,
    pub(super) discret: i32,
}

impl RangeSpans<'_> {
    /// Draws `[u1, u2]`; `(i1, j2)` are the `LocateU` indices of `u1` and `u2`.
    pub(super) fn draw(
        &self,
        display: &mut DrawDisplay<'_>,
        (u1, u2): (f64, f64),
        (i1, j2): (i32, i32),
    ) -> Result<(), Failure> {
        let (mut ka, mut kb) = (
            self.bspline.first_u_knot_index()?,
            self.bspline.last_u_knot_index()?,
        );
        let total = (self.bspline.knot(ka)?, self.bspline.knot(kb)?);
        let uk1 = self.lower_edge(display, &mut ka, i1, u1, total)?;
        let uk2 = self.upper_edge(display, &mut kb, j2, u2, total)?;
        self.draw_knots(display, (ka, kb), (uk1, uk2), total)
    }

    fn lower_edge(
        &self,
        display: &mut DrawDisplay<'_>,
        ka: &mut i32,
        i1: i32,
        u1: f64,
        total: (f64, f64),
    ) -> Result<f64, Failure> {
        if i1 > *ka {
            *ka = i1;
            Ok(u1)
        } else {
            self.plot(display, (u1, total.0), total)?;
            Ok(total.0)
        }
    }
    fn upper_edge(
        &self,
        display: &mut DrawDisplay<'_>,
        kb: &mut i32,
        j2: i32,
        u2: f64,
        total: (f64, f64),
    ) -> Result<f64, Failure> {
        if j2 < *kb {
            *kb = j2;
            Ok(u2)
        } else {
            self.plot(display, (total.1, u2), total)?;
            Ok(total.1)
        }
    }
    fn draw_knots(
        &self,
        display: &mut DrawDisplay<'_>,
        (ka, kb): (i32, i32),
        (uk1, uk2): (f64, f64),
        total: (f64, f64),
    ) -> Result<(), Failure> {
        for k in ka..kb {
            let (ua, ub) = self.span_limits(k, (ka, kb), (uk1, uk2))?;
            self.plot(display, (ua, ub), total)?;
        }
        Ok(())
    }

    // Keep first-span precedence even when the range selects only one span:
    //  pins the existing endpoint behavior before this structural split.
    fn span_limits(
        &self,
        k: i32,
        (ka, kb): (i32, i32),
        (uk1, uk2): (f64, f64),
    ) -> Result<(f64, f64), Failure> {
        let ua = if k == ka { uk1 } else { self.bspline.knot(k)? };
        let ub = if k == ka {
            self.bspline.knot(k + 1)?
        } else if k == kb - 1 {
            uk2
        } else {
            self.bspline.knot(k + 1)?
        };
        Ok((ua, ub))
    }

    /// One span `[ua, ub]`: `|Discret * (ua - ub) / (Ustart - Uend)|` points (at least
    /// 30), a move to `ua`, `NbPoints - 2` steps, then a line to `ub`.
    fn plot(
        &self,
        display: &mut DrawDisplay<'_>,
        (ua, ub): (f64, f64),
        (u_start, u_end): (f64, f64),
    ) -> Result<(), Failure> {
        let ratio = f64::from(self.discret) * (ua - ub) / (u_start - u_end);
        let nb_points = (ratio.abs() as i32).max(MIN_SPAN_POINTS);
        let du = (ub - ua) / f64::from(nb_points);
        let mut u = ua;
        display.move_to_3d(self.bspline.value(u)?);
        for _ in 1..=nb_points - 2 {
            u += du;
            display.draw_to_3d(self.bspline.value(u)?);
        }
        display.draw_to_3d(self.bspline.value(ub)?);
        Ok(())
    }
}
