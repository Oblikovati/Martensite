// SPDX-License-Identifier: GPL-2.0-only
//! A finite Archimedean spiral in a right-handed frame.
mod radial;

use super::curve_support::{AxisFrame, dump_analytic_curve};
use super::eval_failures::unsupported;
use crate::geom::{GeomCurve, GeomCurveD1, GeomCurveD2, GeomCurveD3, GeomGeometry};
use martensite_tkernel::impl_transient;
use martensite_tkernel::precision;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::exceptions::{overflow, range_error};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Axis2, Point3, Transform3, Vector3};
use radial::SpiralRadial;
use std::io;

const CLASS: &str = "GeomEval_PlanarSpiralCurve";

/// Direction of traversal about the placement's positive local Z axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiralRotation {
    /// Positive rotation from local X toward local Y.
    Counterclockwise,
    /// Negative rotation from local X toward negative local Y.
    Clockwise,
}

impl SpiralRotation {
    fn sign(self) -> i32 {
        match self {
            Self::Counterclockwise => 1,
            Self::Clockwise => -1,
        }
    }
}

/// A finite spiral with independent signed radial pitch and rotation sense.
///
/// Evaluation uses radians and the rounded binary64 rate `radial_pitch / TAU`.
/// Construction checks the intended radius over the authored turns and the
/// evaluated radius over the rounded parameter interval. Both are affine, so
/// positive endpoint radii imply positive interior radii.
#[derive(Clone, Debug)]
pub struct GeomEvalPlanarSpiralCurve {
    position: Axis2,
    radial: SpiralRadial,
    rotation: SpiralRotation,
}
impl_transient!(
    GeomEvalPlanarSpiralCurve,
    "GeomEval_PlanarSpiralCurve",
    dyn GeomCurve,
    interfaces: [dyn GeomGeometry, dyn GeomCurve]
);

impl GeomEvalPlanarSpiralCurve {
    /// Create a spiral on `[0, TAU * turns]`. A positive radial pitch increases
    /// radius for either rotation direction. The placement must be a valid
    /// right-handed frame. World coordinates and requested derivative components
    /// are checked when evaluated.
    ///
    /// ```
    /// use martensite_tkg3d::geom_eval::{GeomEvalPlanarSpiralCurve, SpiralRotation};
    /// use martensite_tkmath::gp::Axis2;
    /// let spiral = GeomEvalPlanarSpiralCurve::new(
    ///     Axis2::default(), 4., 2., 1., SpiralRotation::Counterclockwise,
    /// )?;
    /// assert_eq!(spiral.turns(), 1.);
    /// # Ok::<(), martensite_tkernel::standard::Failure>(())
    /// ```
    pub fn new(
        position: Axis2,
        initial_radius: f64,
        radial_pitch: f64,
        turns: f64,
        rotation: SpiralRotation,
    ) -> Result<Self, Failure> {
        let radial = SpiralRadial::new(initial_radius, radial_pitch, turns)?;
        let position = checked_placement(position)?;
        Ok(Self {
            position,
            radial,
            rotation,
        })
    }

    /// Return the right-handed placement. Example `spiral.position()`.
    pub fn position(&self) -> Axis2 {
        self.position
    }

    /// Return the radius at parameter zero. Example `spiral.initial_radius()`.
    pub fn initial_radius(&self) -> f64 {
        self.radial.initial_radius
    }

    /// Return the signed radius change per turn. Example `spiral.radial_pitch()`.
    pub fn radial_pitch(&self) -> f64 {
        self.radial.radial_pitch
    }

    /// Return the authored number of turns. Example `spiral.turns()`.
    pub fn turns(&self) -> f64 {
        self.radial.turns
    }

    /// Return the rotation sense. Example `spiral.rotation()`.
    pub fn rotation(&self) -> SpiralRotation {
        self.rotation
    }

    fn sample(&self, u: f64) -> Result<SpiralSample, Failure> {
        let radius = self.radial.radius_at(u)?;
        let frame = AxisFrame::new(self.position);
        let (sin, cos) = martensite_cmath::sincos(u);
        let sense = self.rotation.sign();
        let sin = f64::from(sense) * sin;
        Ok(SpiralSample {
            parameter: u,
            origin: *frame.origin.coordinates(),
            radial: direction(frame, cos, sin),
            tangent: direction(frame, -sin, cos),
            radius,
            rate: self.radial.rate,
            sense,
        })
    }
}

fn checked_placement(position: Axis2) -> Result<Axis2, Failure> {
    // Legacy Axis2 loading and editing can leave independently stored directions.
    Axis2::from_stored_coefficients(
        position.location(),
        *position.direction().coordinates(),
        *position.x_direction().coordinates(),
        *position.y_direction().coordinates(),
    )
}

fn direction(frame: AxisFrame, cos: f64, sin: f64) -> [f64; 3] {
    let [x, y] = [frame.x_dir.coordinates(), frame.y_dir.coordinates()];
    std::array::from_fn(|i| x[i].mul_add(cos, y[i] * sin))
}

struct SpiralSample {
    parameter: f64,
    origin: [f64; 3],
    radial: [f64; 3],
    tangent: [f64; 3],
    radius: f64,
    rate: f64,
    sense: i32,
}

impl SpiralSample {
    fn point(&self) -> Result<Point3, Failure> {
        let coords = std::array::from_fn(|i| self.radius.mul_add(self.radial[i], self.origin[i]));
        let [x, y, z] = self.checked(coords, 0)?;
        Ok(Point3::new(x, y, z))
    }

    fn derivative(&self, order: i32) -> Result<Vector3, Failure> {
        let current = self.direction_at(order);
        let previous = self.direction_at(order - 1);
        let factor = f64::from(order);
        let scale = factor * self.rate;
        let coords = std::array::from_fn(|i| {
            let radial = self.radius * current[i];
            slope_component(scale, factor, self.rate, previous[i], radial)
        });
        let [x, y, z] = self.checked(coords, order)?;
        Ok(Vector3::new(x, y, z))
    }

    fn direction_at(&self, order: i32) -> [f64; 3] {
        match ((order % 4) * self.sense).rem_euclid(4) {
            0 => self.radial,
            1 => self.tangent,
            2 => self.radial.map(|value| -value),
            _ => self.tangent.map(|value| -value),
        }
    }

    fn checked(&self, coordinates: [f64; 3], order: i32) -> Result<[f64; 3], Failure> {
        if !coordinates.iter().all(|value| value.is_finite()) {
            return Err(overflow(&format!(
                "planar spiral order {order} at parameter {} produced {coordinates:?}, expected finite coordinates",
                self.parameter
            )));
        }
        Ok(coordinates)
    }
}

fn slope_component(scale: f64, order: f64, rate: f64, direction: f64, radial: f64) -> f64 {
    // Scaling first preserves subnormal slopes. Project first if the unprojected scale
    // overflows, since the requested world-space component can still fit.
    if scale.is_finite() {
        return scale.mul_add(direction, radial);
    }
    order.mul_add(rate * direction, radial)
}

impl GeomGeometry for GeomEvalPlanarSpiralCurve {
    fn transform(&self, _transform: Transform3) -> Result<(), Failure> {
        Err(unsupported("GeomEval_PlanarSpiralCurve::Transform", ""))
    }

    fn copy(&self) -> Result<Handle<dyn GeomGeometry>, Failure> {
        Ok(Handle::new(self.clone()).into_upcast())
    }

    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        let fields = [
            ("InitialRadius", self.initial_radius()),
            ("RadialPitch", self.radial_pitch()),
            ("Turns", self.turns()),
            ("Rotation", f64::from(self.rotation().sign())),
        ];
        dump_analytic_curve(stream, depth, CLASS, self.position, &fields)
    }
}

impl GeomCurve for GeomEvalPlanarSpiralCurve {
    martensite_tkg2d::unsupported_curve_reversal!(unsupported, "GeomEval_PlanarSpiralCurve");
    martensite_tkg2d::non_periodic_cn_curve!();

    fn first_parameter(&self) -> f64 {
        0.0
    }

    fn last_parameter(&self) -> f64 {
        self.radial.last
    }

    fn is_closed(&self) -> Result<bool, Failure> {
        if self.radial_pitch() != 0.0 {
            return Ok(false);
        }
        let first = self.eval_d0(self.first_parameter())?;
        let last = self.eval_d0(self.last_parameter())?;
        Ok(first.is_equal(&last, precision::confusion()))
    }

    fn eval_d0(&self, u: f64) -> Result<Point3, Failure> {
        self.sample(u)?.point()
    }

    fn eval_d1(&self, u: f64) -> Result<GeomCurveD1, Failure> {
        let sample = self.sample(u)?;
        Ok(GeomCurveD1 {
            point: sample.point()?,
            d1: sample.derivative(1)?,
        })
    }

    fn eval_d2(&self, u: f64) -> Result<GeomCurveD2, Failure> {
        let sample = self.sample(u)?;
        Ok(GeomCurveD2 {
            point: sample.point()?,
            d1: sample.derivative(1)?,
            d2: sample.derivative(2)?,
        })
    }

    fn eval_d3(&self, u: f64) -> Result<GeomCurveD3, Failure> {
        let sample = self.sample(u)?;
        Ok(GeomCurveD3 {
            point: sample.point()?,
            d1: sample.derivative(1)?,
            d2: sample.derivative(2)?,
            d3: sample.derivative(3)?,
        })
    }

    fn eval_dn(&self, u: f64, n: i32) -> Result<Vector3, Failure> {
        if n < 1 {
            return Err(range_error(&format!(
                "planar spiral derivative order {n} must be at least 1"
            )));
        }
        self.sample(u)?.derivative(n)
    }
}
