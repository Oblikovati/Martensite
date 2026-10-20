//! `ExtElCS`: the extrema between an elementary curve and an elementary
//! surface. The line-cone, line-torus, circle-cone and circle-torus overloads raise
//! NotImplemented natively.
mod circles;
mod lines;

use super::ext_el_ss::handle_value;
use super::failures::{extremum_index, not_done, not_implemented};
use super::pon::{POnCurv, POnSurf};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::elclib;
use martensite_tkmath::elslib;
use martensite_tkmath::gp::{
    Axis2, Circle3, Cone3, Direction3, Hyperbola3, Line3, Plane3, Point3, Torus3,
};

/// `ExtElCS`. The result arrays are handles natively: a Perform keeps the
/// arrays it does not reallocate, and `Points` on a result whose arrays were never
/// allocated raises null-object failure (a null dereference natively);
/// `ExtElCS::line_plane(line, plane).is_parallel()?`.
#[derive(Clone, Debug, Default)]
pub struct ExtElCS {
    done: bool,
    nb_ext: i32,
    is_par: bool,
    sq_dist: Option<Vec<f64>>,
    points1: Option<Vec<POnCurv>>,
    points2: Option<Vec<POnSurf>>,
}

// The `ExtElCS(C, S)` constructor of one `Perform` overload.
macro_rules! cs_constructor {
    ($(#[$doc:meta])* $ctor:ident, $perform:ident, $curve:ty, $surface:ty) => {
        $(#[$doc])*
        pub fn $ctor(curve: $curve, surface: $surface) -> Result<Self, Failure> {
            let mut extrema: Self = Self::new();
            extrema.$perform(&curve, &surface)?;
            Ok(extrema)
        }
    };
}

impl ExtElCS {
    /// `ExtElCS()`: not done; `ExtElCS::new().is_done() == false`.
    pub fn new() -> Self {
        Self::default()
    }
    // Allocates the three result arrays for `count` extrema.
    fn allocate(&mut self, count: usize) {
        self.sq_dist = Some(vec![0.0; count]);
        self.points1 = Some(vec![POnCurv::default(); count]);
        self.points2 = Some(vec![POnSurf::default(); count]);
    }
    // `SetValue(i, ...)` on the three arrays (1-based).
    fn set_extremum(&mut self, i: usize, sq_dist: f64, on_curve: POnCurv, on_surface: POnSurf) {
        if let Some(values) = self.sq_dist.as_mut() {
            values[i - 1] = sq_dist;
        }
        if let Some(values) = self.points1.as_mut() {
            values[i - 1] = on_curve;
        }
        if let Some(values) = self.points2.as_mut() {
            values[i - 1] = on_surface;
        }
    }
    // The parallel result: one square distance in a fresh array of one value.
    fn set_parallel(&mut self, sq_dist: f64) {
        self.sq_dist = Some(vec![sq_dist]);
        self.is_par = true;
        self.nb_ext = 1;
    }
    /// `ExtElCS(Lin, Pln)`; `ExtElCS::line_plane(line, plane)`.
    pub fn line_plane(line: Line3, plane: Plane3) -> Self {
        let mut extrema: Self = Self::new();
        extrema.perform_line_plane(&line, &plane);
        extrema
    }
    /// `Perform(Lin, Pln)`: always done; a line normal to the plane axis (within
    /// Precision::Angular()) is parallel with its square distance, other lines have no
    /// extremum; `extrema.perform_line_plane(&line, &plane)`.
    pub fn perform_line_plane(&mut self, line: &Line3, plane: &Plane3) {
        self.done = true;
        self.is_par = false;
        self.nb_ext = 0;
        let normal: Direction3 = plane.axis().direction();
        if line.direction().is_normal(normal, precision::angular()) {
            self.set_parallel(plane.square_distance_line(*line));
        }
    }
    cs_constructor!(
        /// `ExtElCS(Lin, Cone)`: NotImplemented.
        line_cone, perform_line_cone, Line3, Cone3);
    /// `Perform(Lin, Cone)`: NotImplemented, the state unchanged.
    pub fn perform_line_cone(&mut self, _line: &Line3, _cone: &Cone3) -> Result<(), Failure> {
        Err(not_implemented("Extrema_ExtElCS::Perform(gp_Lin, gp_Cone)"))
    }
    cs_constructor!(
        /// `ExtElCS(Lin, Torus)`: NotImplemented.
        line_torus, perform_line_torus, Line3, Torus3);
    /// `Perform(Lin, Torus)`: NotImplemented, the state unchanged.
    pub fn perform_line_torus(&mut self, _line: &Line3, _torus: &Torus3) -> Result<(), Failure> {
        Err(not_implemented(
            "Extrema_ExtElCS::Perform(gp_Lin, gp_Torus)",
        ))
    }
    cs_constructor!(
        /// `ExtElCS(Circ, Cone)`: NotImplemented.
        circle_cone, perform_circle_cone, Circle3, Cone3);
    /// `Perform(Circ, Cone)`: NotImplemented, the state unchanged.
    pub fn perform_circle_cone(&mut self, _circle: &Circle3, _cone: &Cone3) -> Result<(), Failure> {
        Err(not_implemented(
            "Extrema_ExtElCS::Perform(gp_Circ, gp_Cone)",
        ))
    }
    cs_constructor!(
        /// `ExtElCS(Circ, Torus)`: NotImplemented.
        circle_torus, perform_circle_torus, Circle3, Torus3);
    /// `Perform(Circ, Torus)`: NotImplemented, the state unchanged.
    pub fn perform_circle_torus(
        &mut self,
        _circle: &Circle3,
        _torus: &Torus3,
    ) -> Result<(), Failure> {
        Err(not_implemented(
            "Extrema_ExtElCS::Perform(gp_Circ, gp_Torus)",
        ))
    }
}

impl ExtElCS {
    /// `ExtElCS(Hypr, Pln)`; `ExtElCS::hyperbola_plane(hyperbola, plane)`.
    pub fn hyperbola_plane(hyperbola: Hyperbola3, plane: Plane3) -> Self {
        let mut extrema: Self = Self::new();
        extrema.perform_hyperbola_plane(&hyperbola, &plane);
        extrema
    }
    /// `Perform(Hypr, Pln)`: always done; a hyperbola parallel to the plane is
    /// parallel with the square distance of its centre; otherwise the point where the
    /// hyperbola is parallel to the plane, `T = -ln((A + B) / (B - A)) / 2`, when
    /// `|B| > |A|`; `extrema.perform_hyperbola_plane(&hyperbola, &plane)`.
    // The negated test keeps the retained NaN semantics.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    pub fn perform_hyperbola_plane(&mut self, hyperbola: &Hyperbola3, plane: &Plane3) {
        self.done = true;
        self.is_par = false;
        self.nb_ext = 0;
        let position: Axis2 = hyperbola.position();
        let n_pln: Direction3 = plane.axis().direction();
        if position
            .direction()
            .is_parallel(n_pln, precision::angular())
        {
            self.set_parallel(plane.square_distance(hyperbola.location()));
            return;
        }
        let a: f64 = hyperbola.minor_radius() * n_pln.dot(position.y_direction());
        let b: f64 = hyperbola.major_radius() * n_pln.dot(position.x_direction());
        if !(b.abs() > a.abs()) {
            return;
        }
        let t: f64 = -0.5 * martensite_cmath::log((a + b) / (b - a));
        let (major, minor) = (hyperbola.major_radius(), hyperbola.minor_radius());
        let ph: Point3 = elclib::hyperbola_value(t, position, major, minor);
        self.points1 = Some(vec![POnCurv::new(t, ph)]);
        self.sq_dist = Some(vec![plane.square_distance(ph)]);
        let (u, v) = elslib::plane_parameters(plane.position(), ph);
        let pp: Point3 = elslib::plane_value(u, v, plane.position());
        self.points2 = Some(vec![POnSurf::new(u, v, pp)]);
        self.nb_ext = 1;
    }
}

impl ExtElCS {
    /// `IsDone()`; `extrema.is_done()`.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// `IsParallel()`: not-done failure when not done; `extrema.is_parallel()?`.
    pub fn is_parallel(&self) -> Result<bool, Failure> {
        if !self.done {
            return Err(not_done("Extrema_ExtElCS::IsParallel()"));
        }
        Ok(self.is_par)
    }
    /// `NbExt()`: not-done failure when not done; `extrema.nb_ext()?`.
    pub fn nb_ext(&self) -> Result<i32, Failure> {
        if !self.done {
            return Err(not_done("Extrema_ExtElCS::NbExt()"));
        }
        Ok(self.nb_ext)
    }
    /// `SquareDistance(N)` (the retained default is 1); `extrema.square_distance(1)?`.
    pub fn square_distance(&self, n: i32) -> Result<f64, Failure> {
        let member: &str = "Extrema_ExtElCS::SquareDistance()";
        extremum_index(member, n, || self.nb_ext())?;
        handle_value(member, self.sq_dist.as_deref(), n)
    }
    /// `Points(N, P1, P2)`, returned as `(P1 on the curve, P2 on the surface)`;
    /// `extrema.points(1)?`.
    pub fn points(&self, n: i32) -> Result<(POnCurv, POnSurf), Failure> {
        let member: &str = "Extrema_ExtElCS::Points()";
        extremum_index(member, n, || self.nb_ext())?;
        let first: POnCurv = handle_value(member, self.points1.as_deref(), n)?;
        Ok((first, handle_value(member, self.points2.as_deref(), n)?))
    }
}

/// The parameters and point of `point` on a cylinder, cone or sphere
/// (`ElSLib::Parameters` then the given point).
fn surface_point<S: elslib::ElementarySurface>(surface: &S, point: Point3) -> POnSurf {
    let (u, v) = elslib::parameters(surface, point);
    POnSurf::new(u, v, point)
}

/// The smallest square distance of a line-line or line-circle extrema, starting
/// from the first one.
fn minimal_square_distance(
    count: i32,
    value: impl Fn(i32) -> Result<f64, Failure>,
) -> Result<f64, Failure> {
    let mut distance: f64 = value(1)?;
    for i in 2..=count {
        let candidate: f64 = value(i)?;
        if candidate < distance {
            distance = candidate;
        }
    }
    Ok(distance)
}

/// The `(sqrt(d) - R)²` of the parallel cases.
fn parallel_square_distance(sq_dist: f64, radius: f64) -> f64 {
    let distance: f64 = sq_dist.sqrt() - radius;
    distance * distance
}
