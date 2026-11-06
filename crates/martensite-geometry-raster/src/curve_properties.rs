//

//! The steps that `GeometryCurve::DrawOn` and `GeometryCurve2d::DrawOn` share:
//! the finite range of an infinite curve, the orientation arrow and the curvature
//! radii.
use crate::CurveStyle;
use martensite_raster_core::sampling::bound;
use martensite_raster_core::{MarkerShape as DrawMarkerShape, RasterDisplay as DrawDisplay};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::geom_lprop::{CLProps, CLProps2d};
use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};

/// `GeometryCurveLimit`: an infinite curve is drawn until its ends are this far
/// apart.
pub const CURVE_LIMIT: f64 = 400.0;

/// `Resolution` of the curvature display: smaller curvatures are not drawn.
const CURVATURE_RESOLUTION: f64 = 1.0e-9;

/// A point of the plane or of space as the curve drawings use it.
pub trait PlotPoint: Copy {
    /// `Distance`.
    /// Usage: `let length = point.distance_to(other);`.
    fn distance_to(self, other: Self) -> f64;
    /// `RasterDisplay::MoveTo`.
    /// Usage: `point.move_on(&mut display);`.
    fn move_on(self, display: &mut DrawDisplay<'_>);
    /// `RasterDisplay::DrawTo`.
    /// Usage: `point.draw_on(&mut display);`.
    fn draw_on(self, display: &mut DrawDisplay<'_>);
    /// `P1.Translated(ratio * gpVec(P1, P2))`.
    /// Usage: `let radius = point.toward(centre, fraction);`.
    fn toward(self, target: Self, ratio: f64) -> Self;
    /// `RasterDisplay::Project`; a 2D point is projected as `(x, y, 0)`.
    /// Usage: `let screen = point.project_on(&display);`.
    fn project_on(self, display: &DrawDisplay<'_>) -> Point2;
    /// `RasterDisplay::DrawMarker(P, shape, size)` with a pixel size.
    /// Usage: `point.mark_on(&mut display, marker, 5);`.
    fn mark_on(self, display: &mut DrawDisplay<'_>, shape: DrawMarkerShape, size: i32);
}

impl PlotPoint for Point3 {
    fn distance_to(self, other: Self) -> f64 {
        self.distance(&other)
    }
    fn move_on(self, display: &mut DrawDisplay<'_>) {
        display.move_to_3d(self);
    }
    fn draw_on(self, display: &mut DrawDisplay<'_>) {
        display.draw_to_3d(self);
    }
    fn toward(self, target: Self, ratio: f64) -> Self {
        self.translated(Vector3::between(self, target) * ratio)
    }
    fn project_on(self, display: &DrawDisplay<'_>) -> Point2 {
        display.project(self)
    }
    fn mark_on(self, display: &mut DrawDisplay<'_>, shape: DrawMarkerShape, size: i32) {
        display.draw_marker_3d(self, shape, size);
    }
}

impl PlotPoint for Point2 {
    fn distance_to(self, other: Self) -> f64 {
        self.distance(other)
    }
    fn move_on(self, display: &mut DrawDisplay<'_>) {
        display.move_to(self);
    }
    fn draw_on(self, display: &mut DrawDisplay<'_>) {
        display.draw_to(self);
    }
    fn toward(self, target: Self, ratio: f64) -> Self {
        self.translated(Vector2::between(self, target) * ratio)
    }
    fn project_on(self, display: &DrawDisplay<'_>) -> Point2 {
        display.project(Point3::new(self.x(), self.y(), 0.0))
    }
    fn mark_on(self, display: &mut DrawDisplay<'_>, shape: DrawMarkerShape, size: i32) {
        display.draw_marker(self, shape, size);
    }
}

/// The parameter range drawn for a curve on `[first, last]`: an infinite end is
/// replaced by doubling a range around the finite end (or around 0) until the end
/// points are [`CURVE_LIMIT`] apart.
///
/// Native quirk kept: a curve whose points never get that far apart loops forever.
/// A NaN distance ends the loop, as the C++ `while (d < limit)` does.
/// Usage: `finite_range((first, last), |t| curve.d0(t))?;`.
pub fn finite_range<P: PlotPoint>(
    (first, last): (f64, f64),
    d0: impl Fn(f64) -> Result<P, Failure>,
) -> Result<(f64, f64), Failure> {
    let first_inf = precision::is_negative_infinite(first);
    let last_inf = precision::is_positive_infinite(last);
    if first_inf && last_inf {
        return symmetric_range(&d0);
    }
    if first_inf {
        return lower_infinite_range(last, &d0);
    }
    if last_inf {
        return upper_infinite_range(first, &d0);
    }
    Ok((first, last))
}
fn symmetric_range<P: PlotPoint>(
    d0: &impl Fn(f64) -> Result<P, Failure>,
) -> Result<(f64, f64), Failure> {
    let mut delta = 1.0;
    loop {
        delta *= 2.0;
        let (first, last) = (-delta, delta);
        if reached_limit(d0(first)?.distance_to(d0(last)?)) {
            return Ok((first, last));
        }
    }
}
fn lower_infinite_range<P: PlotPoint>(
    last: f64,
    d0: &impl Fn(f64) -> Result<P, Failure>,
) -> Result<(f64, f64), Failure> {
    let mut delta = 1.0;
    let end = d0(last)?;
    loop {
        delta *= 2.0;
        let first = last - delta;
        if reached_limit(d0(first)?.distance_to(end)) {
            return Ok((first, last));
        }
    }
}
fn upper_infinite_range<P: PlotPoint>(
    first: f64,
    d0: &impl Fn(f64) -> Result<P, Failure>,
) -> Result<(f64, f64), Failure> {
    let mut delta = 1.0;
    let start = d0(first)?;
    loop {
        delta *= 2.0;
        let last = first + delta;
        if reached_limit(start.distance_to(d0(last)?)) {
            return Ok((first, last));
        }
    }
}

/// The end test of the range doubling: `!(distance < CURVE_LIMIT)`, true for NaN.
fn reached_limit(distance: f64) -> bool {
    distance >= CURVE_LIMIT || distance.is_nan()
}

pub use martensite_raster_core::sampling::draw_orientation_arrow;

/// The local properties a curvature display reads at one parameter.
pub trait CurvatureProps {
    /// `gpPnt` or `gpPnt2d`.
    type Point: PlotPoint;
    /// `SetParameter(t)` then `IsTangentDefined()` and `|Curvature()|`; `None` when
    /// the tangent is not defined.
    /// Usage: `let curvature = props.curvature_at(parameter)?;`.
    fn curvature_at(&mut self, t: f64) -> Result<Option<f64>, Failure>;
    /// `CentreOfCurvature` at the last parameter.
    /// Usage: `let centre = props.centre()?;`.
    fn centre(&mut self) -> Result<Self::Point, Failure>;
}

macro_rules! curvature_props {
    ($props:ty, $point:ty) => {
        impl CurvatureProps for $props {
            type Point = $point;
            fn curvature_at(&mut self, t: f64) -> Result<Option<f64>, Failure> {
                self.set_parameter(t)?;
                if !self.is_tangent_defined()? {
                    return Ok(None);
                }
                Ok(Some(self.curvature()?.abs()))
            }
            fn centre(&mut self) -> Result<$point, Failure> {
                self.centre_of_curvature()
            }
        }
    };
}
curvature_props!(CLProps, Point3);
curvature_props!(CLProps2d, Point2);

/// What the curvature display of one curve draws with.
pub struct CurvatureDisplay {
    /// The number of samples per CN interval (`GetDiscretisation`).
    samples: i32,
    /// `radiusmax`: longer radii are scaled down to this length.
    radius_max: f64,
    /// `radiusratio`: the drawn part of each radius.
    radius_ratio: f64,
}

impl CurvatureDisplay {
    /// The display of a curve with `samples` points per interval and `style`.
    /// Snapshots radius settings for a draw; `CurvatureDisplay::of(30, &style)`.
    pub fn of(samples: i32, style: &CurveStyle) -> Self {
        Self {
            samples,
            radius_max: style.radius_max(),
            radius_ratio: style.radius_ratio(),
        }
    }

    /// The `GeomLProp_CLProps(curv, 2, 1e-9)` resolution, below which a curvature is
    /// not drawn.
    /// Reads the metric property threshold; `let epsilon = radii.resolution();`.
    pub const fn resolution(&self) -> f64 {
        CURVATURE_RESOLUTION
    }

    /// The curvature radii: at `samples` parameters of each CN interval, a segment from
    /// the curve point toward the centre of curvature, of `radius_ratio` times the
    /// radius (capped at `radius_max`).
    /// Usage: `radii.draw(&mut display, &bounds, &mut props, |t| curve.d0(t))?;`.
    pub fn draw<P: CurvatureProps>(
        &self,
        display: &mut DrawDisplay<'_>,
        bounds: &Array1<'static, f64>,
        props: &mut P,
        d0: impl Fn(f64) -> Result<P::Point, Failure>,
    ) -> Result<(), Failure> {
        for interval in 1..bounds.len() as i32 {
            let mut t = bound(bounds, interval)?;
            let step = (bound(bounds, interval + 1)? - t) / f64::from(self.samples);
            for _ in 1..=self.samples {
                self.draw_radius(display, props, &d0, t)?;
                t += step;
            }
        }
        Ok(())
    }

    /// One radius at `t`, when the tangent is defined and the curvature is not null.
    fn draw_radius<P: CurvatureProps>(
        &self,
        display: &mut DrawDisplay<'_>,
        props: &mut P,
        d0: &impl Fn(f64) -> Result<P::Point, Failure>,
        t: f64,
    ) -> Result<(), Failure> {
        let Some(curvature) = props.curvature_at(t)? else {
            return Ok(());
        };
        if curvature <= CURVATURE_RESOLUTION {
            return Ok(());
        }
        let point = d0(t)?;
        point.move_on(display);
        let ratio = self.scaled_radius_ratio(curvature);
        point.toward(props.centre()?, ratio).draw_on(display);
        Ok(())
    }
    fn scaled_radius_ratio(&self, curvature: f64) -> f64 {
        let radius = 1. / curvature;
        let mut ratio = if radius > self.radius_max {
            self.radius_max / radius
        } else {
            1.0
        };
        ratio *= self.radius_ratio;
        ratio
    }
}
