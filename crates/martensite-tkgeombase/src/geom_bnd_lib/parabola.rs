//! `Parabola` and `Parabola2d`: boxes of parabola arcs.
use super::live;
use super::ranges::{OpenSides, add_branch_range};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dParabola};
use martensite_tkg3d::geom::{GeomCurve, GeomParabola};
use martensite_tkmath::bnd::{BoundingBox2, BoundingBox3};
use martensite_tkmath::elclib::{self, ElementaryCurve};
use martensite_tkmath::gp::{Parabola2, Parabola3};

/// The parabola box of `[u1, u2]` (also BndLib's `Add(Parab)`, `Add(Parab2d)` and
/// `Add(Hypr2d)`, which share the finite rule): U1, U2 and the vertex when the arc
/// crosses 0; `add_parabola_range(&mut bounds, &parabola, (-4.0, 4.0), "...")?`.
pub fn add_parabola_range<C, B>(
    bounds: &mut B,
    curve: &C,
    (u1, u2): (f64, f64),
    message: &str,
) -> Result<(), Failure>
where
    C: ElementaryCurve<Point = <B as super::space::SampleBounds>::Point>,
    B: OpenSides,
{
    let value = |u: f64| elclib::value(u, curve);
    let mut finite = |b: &mut B| {
        b.add_sample(value(u2));
        if u1 * u2 < 0.0 {
            b.add_sample(value(0.0));
        }
    };
    add_branch_range(bounds, (u1, u2), &value, &mut finite, message)
}

/// `Parabola`: the box of a `Parabola` arc; `parab_box` takes a
/// `Parab`.
pub struct Parabola {
    geom: Handle<GeomParabola>,
}

impl Parabola {
    /// The evaluator of `parabola`; `Parabola::new(handle)`.
    pub fn new(parabola: Handle<GeomParabola>) -> Self {
        Self { geom: parabola }
    }
    /// `Geometry()`; `evaluator.geometry()`.
    pub fn geometry(&self) -> &Handle<GeomParabola> {
        &self.geom
    }
    /// `Box(U1, U2, Tol)`; `evaluator.bounding_box_range(-4.0, 4.0, 0.0)?`.
    pub fn bounding_box_range(
        &self,
        u1: f64,
        u2: f64,
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        Self::parab_box(
            &live(&self.geom, "GeomBndLib_Parabola")?.parab()?,
            u1,
            u2,
            tolerance,
        )
    }
    /// `Box(Tol)`: the whole (infinite) parabola; `evaluator.bounding_box(0.0)?`.
    pub fn bounding_box(&self, tolerance: f64) -> Result<BoundingBox3, Failure> {
        let curve = live(&self.geom, "GeomBndLib_Parabola")?;
        self.bounding_box_range(curve.first_parameter(), curve.last_parameter(), tolerance)
    }
    /// `BoxOptimal(U1, U2, Tol)`: the analytic box;
    /// `evaluator.bounding_box_optimal_range(-4.0, 4.0, 0.0)?`.
    pub fn bounding_box_optimal_range(
        &self,
        u1: f64,
        u2: f64,
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        self.bounding_box_range(u1, u2, tolerance)
    }
    /// `Box(Parab, U1, U2, Tol)`: infinite ends open every min or max side; two
    /// infinite ends of the same sign raise Failure;
    /// `Parabola::parab_box(&parabola, -4.0, 4.0, 0.0)?`.
    pub fn parab_box(
        parabola: &Parabola3,
        u1: f64,
        u2: f64,
        tolerance: f64,
    ) -> Result<BoundingBox3, Failure> {
        let mut bounds: BoundingBox3 = BoundingBox3::default();
        let message = "GeomBndLib_Parabola::Box - bad parameter";
        add_parabola_range(&mut bounds, parabola, (u1, u2), message)?;
        bounds.enlarge(tolerance);
        Ok(bounds)
    }
}

/// `Parabola2d`: the box of a `Parabola` arc; `parab_box` takes a
/// `Parab2d`.
pub struct Parabola2d {
    geom: Handle<Geom2dParabola>,
}

impl Parabola2d {
    /// The evaluator of `parabola`; `Parabola2d::new(handle)`.
    pub fn new(parabola: Handle<Geom2dParabola>) -> Self {
        Self { geom: parabola }
    }
    /// `Geometry()`; `evaluator.geometry()`.
    pub fn geometry(&self) -> &Handle<Geom2dParabola> {
        &self.geom
    }
    /// `Box(U1, U2, Tol)`; `evaluator.bounding_box_range(-4.0, 4.0, 0.0)?`.
    pub fn bounding_box_range(
        &self,
        u1: f64,
        u2: f64,
        tolerance: f64,
    ) -> Result<BoundingBox2, Failure> {
        Self::parab_box(
            &live(&self.geom, "GeomBndLib_Parabola2d")?.parab2d()?,
            u1,
            u2,
            tolerance,
        )
    }
    /// `Box(Tol)`: the whole parabola; `evaluator.bounding_box(0.0)?`.
    pub fn bounding_box(&self, tolerance: f64) -> Result<BoundingBox2, Failure> {
        let curve = live(&self.geom, "GeomBndLib_Parabola2d")?;
        self.bounding_box_range(curve.first_parameter(), curve.last_parameter(), tolerance)
    }
    /// `BoxOptimal(U1, U2, Tol)`: the analytic box;
    /// `evaluator.bounding_box_optimal_range(-4.0, 4.0, 0.0)?`.
    pub fn bounding_box_optimal_range(
        &self,
        u1: f64,
        u2: f64,
        tolerance: f64,
    ) -> Result<BoundingBox2, Failure> {
        self.bounding_box_range(u1, u2, tolerance)
    }
    /// `Box(Parab2d, U1, U2, Tol)`: as [`Parabola::parab_box`] in 2D;
    /// `Parabola2d::parab_box(&parabola, -4.0, 4.0, 0.0)?`.
    pub fn parab_box(
        parabola: &Parabola2,
        u1: f64,
        u2: f64,
        tolerance: f64,
    ) -> Result<BoundingBox2, Failure> {
        let mut bounds: BoundingBox2 = BoundingBox2::default();
        let message = "GeomBndLib_Parabola2d::Box - bad parameter";
        add_parabola_range(&mut bounds, parabola, (u1, u2), message)?;
        bounds.enlarge(tolerance);
        Ok(bounds)
    }
}
