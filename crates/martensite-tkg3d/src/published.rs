// SPDX-License-Identifier: GPL-2.0-only
//! Immutable, lock-free published 3D curves and surfaces (#65).
//!
//! Published geometry wraps `Arc<Immutable*>` with no `RwLock` (`SharedState`) and no
//! setters reachable through a shared handle. Edits happen through uniquely owned
//! operation-local builders (`Curve3dBuilder`, `SurfaceBuilder`) that publish a new
//! revision while concurrent readers evaluate prior revisions without locking.
use crate::geom::{
    GeomCircle, GeomConicalSurface, GeomCylindricalSurface, GeomEllipse, GeomHyperbola, GeomLine,
    GeomParabola, GeomPlane, GeomSphericalSurface, GeomToroidalSurface,
};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{
    Circle3, Cone3, Cylinder3, Ellipse3, Hyperbola3, Line3, Parabola3, Plane3, Point3, Sphere3,
    Torus3, Transform3, Vector3,
};
use martensite_tkmath::{elclib, elslib};
use std::f64::consts::TAU;
use std::sync::Arc;

#[derive(Clone, Debug)]
enum ImmutableCurve3d {
    Line(Line3),
    Circle(Circle3),
    Ellipse(Ellipse3),
    Hyperbola(Hyperbola3),
    Parabola(Parabola3),
    Trimmed {
        basis: Arc<ImmutableCurve3d>,
        first: f64,
        last: f64,
    },
}

/// Immutable, `Send + Sync`, lock-free published 3D curve (#65).
#[derive(Clone, Debug)]
pub struct PublishedCurve3d {
    inner: Arc<ImmutableCurve3d>,
}

impl PublishedCurve3d {
    /// Publishes an immutable 3D line.
    pub fn line(line: Line3) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve3d::Line(line)),
        }
    }

    /// Publishes an immutable 3D circle.
    pub fn circle(circle: Circle3) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve3d::Circle(circle)),
        }
    }

    /// Publishes an immutable 3D ellipse.
    pub fn ellipse(ellipse: Ellipse3) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve3d::Ellipse(ellipse)),
        }
    }

    /// Publishes an immutable 3D hyperbola.
    pub fn hyperbola(hyperbola: Hyperbola3) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve3d::Hyperbola(hyperbola)),
        }
    }

    /// Publishes an immutable 3D parabola.
    pub fn parabola(parabola: Parabola3) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve3d::Parabola(parabola)),
        }
    }

    /// Publishes a trimmed view of an existing published 3D curve without copying.
    pub fn trimmed(&self, first: f64, last: f64) -> Result<Self, Failure> {
        if !first.is_finite() || !last.is_finite() || last <= first {
            return Err(Failure::with_message(
                "Standard_ConstructionError",
                "trimmed curve bounds must be finite with first < last",
            ));
        }
        Ok(Self {
            inner: Arc::new(ImmutableCurve3d::Trimmed {
                basis: Arc::clone(&self.inner),
                first,
                last,
            }),
        })
    }

    /// Snapshots a legacy mutable `GeomLine` into an immutable published curve.
    pub fn from_geom_line(curve: &GeomLine) -> Self {
        Self::line(curve.lin())
    }

    /// Snapshots a legacy mutable `GeomCircle` into an immutable published curve.
    pub fn from_geom_circle(curve: &GeomCircle) -> Result<Self, Failure> {
        Ok(Self::circle(curve.circ()?))
    }

    /// Snapshots a legacy mutable `GeomEllipse` into an immutable published curve.
    pub fn from_geom_ellipse(curve: &GeomEllipse) -> Result<Self, Failure> {
        Ok(Self::ellipse(curve.elips()?))
    }

    /// Snapshots a legacy mutable `GeomHyperbola` into an immutable published curve.
    pub fn from_geom_hyperbola(curve: &GeomHyperbola) -> Result<Self, Failure> {
        Ok(Self::hyperbola(curve.hypr()?))
    }

    /// Snapshots a legacy mutable `GeomParabola` into an immutable published curve.
    pub fn from_geom_parabola(curve: &GeomParabola) -> Result<Self, Failure> {
        Ok(Self::parabola(curve.parab()?))
    }

    /// Lock-free position evaluation `C(u)` (#65).
    pub fn d0(&self, u: f64) -> Point3 {
        eval_d0_3d(&self.inner, u)
    }

    /// Lock-free first derivative evaluation `(C(u), C'(u))` (#65).
    pub fn d1(&self, u: f64) -> (Point3, Vector3) {
        eval_d1_3d(&self.inner, u)
    }

    /// Lock-free second derivative evaluation `(C(u), C'(u), C''(u))` (#65).
    pub fn d2(&self, u: f64) -> (Point3, Vector3, Vector3) {
        eval_d2_3d(&self.inner, u)
    }

    /// First parameter of the curve domain.
    pub fn first_parameter(&self) -> f64 {
        match &*self.inner {
            ImmutableCurve3d::Line(_)
            | ImmutableCurve3d::Hyperbola(_)
            | ImmutableCurve3d::Parabola(_) => -precision::infinite(),
            ImmutableCurve3d::Circle(_) | ImmutableCurve3d::Ellipse(_) => 0.0,
            ImmutableCurve3d::Trimmed { first, .. } => *first,
        }
    }

    /// Last parameter of the curve domain.
    pub fn last_parameter(&self) -> f64 {
        match &*self.inner {
            ImmutableCurve3d::Line(_)
            | ImmutableCurve3d::Hyperbola(_)
            | ImmutableCurve3d::Parabola(_) => precision::infinite(),
            ImmutableCurve3d::Circle(_) | ImmutableCurve3d::Ellipse(_) => TAU,
            ImmutableCurve3d::Trimmed { last, .. } => *last,
        }
    }

    /// Opens an operation-local copy-on-write builder without mutating this handle (#65).
    pub fn to_builder(&self) -> Curve3dBuilder {
        Curve3dBuilder {
            inner: (*self.inner).clone(),
        }
    }
}

/// Operation-local mutable builder producing a new [`PublishedCurve3d`] (#65).
#[derive(Clone, Debug)]
pub struct Curve3dBuilder {
    inner: ImmutableCurve3d,
}

impl Curve3dBuilder {
    /// Applies a 3D transformation to this builder copy.
    pub fn transform(&mut self, trsf: &Transform3) -> Result<&mut Self, Failure> {
        self.inner = transform_3d(&self.inner, trsf)?;
        Ok(self)
    }

    /// Publishes the edited geometry as a new lock-free [`PublishedCurve3d`].
    pub fn publish(self) -> PublishedCurve3d {
        PublishedCurve3d {
            inner: Arc::new(self.inner),
        }
    }
}

#[derive(Clone, Debug)]
enum ImmutableSurface {
    Plane(Plane3),
    Cylinder(Cylinder3),
    Cone(Cone3),
    Sphere(Sphere3),
    Torus(Torus3),
}

/// Immutable, `Send + Sync`, lock-free published 3D surface (#65).
#[derive(Clone, Debug)]
pub struct PublishedSurface {
    inner: Arc<ImmutableSurface>,
}

impl PublishedSurface {
    /// Publishes an immutable plane.
    pub fn plane(plane: Plane3) -> Self {
        Self {
            inner: Arc::new(ImmutableSurface::Plane(plane)),
        }
    }

    /// Publishes an immutable cylinder.
    pub fn cylinder(cylinder: Cylinder3) -> Self {
        Self {
            inner: Arc::new(ImmutableSurface::Cylinder(cylinder)),
        }
    }

    /// Publishes an immutable cone.
    pub fn cone(cone: Cone3) -> Self {
        Self {
            inner: Arc::new(ImmutableSurface::Cone(cone)),
        }
    }

    /// Publishes an immutable sphere.
    pub fn sphere(sphere: Sphere3) -> Self {
        Self {
            inner: Arc::new(ImmutableSurface::Sphere(sphere)),
        }
    }

    /// Publishes an immutable torus.
    pub fn torus(torus: Torus3) -> Self {
        Self {
            inner: Arc::new(ImmutableSurface::Torus(torus)),
        }
    }

    /// Snapshots a legacy mutable `GeomPlane`.
    pub fn from_geom_plane(surface: &GeomPlane) -> Self {
        Self::plane(surface.pln())
    }

    /// Snapshots a legacy mutable `GeomCylindricalSurface`.
    pub fn from_geom_cylinder(surface: &GeomCylindricalSurface) -> Result<Self, Failure> {
        Ok(Self::cylinder(surface.cylinder()?))
    }

    /// Snapshots a legacy mutable `GeomConicalSurface`.
    pub fn from_geom_cone(surface: &GeomConicalSurface) -> Result<Self, Failure> {
        Ok(Self::cone(surface.cone()?))
    }

    /// Snapshots a legacy mutable `GeomSphericalSurface`.
    pub fn from_geom_sphere(surface: &GeomSphericalSurface) -> Result<Self, Failure> {
        Ok(Self::sphere(surface.sphere()?))
    }

    /// Snapshots a legacy mutable `GeomToroidalSurface`.
    pub fn from_geom_torus(surface: &GeomToroidalSurface) -> Result<Self, Failure> {
        Ok(Self::torus(surface.torus()?))
    }

    /// Lock-free surface point evaluation `S(u, v)` (#65).
    pub fn d0(&self, u: f64, v: f64) -> Point3 {
        match &*self.inner {
            ImmutableSurface::Plane(s) => elslib::plane_value(u, v, s.position()),
            ImmutableSurface::Cylinder(s) => elslib::cylinder_value(u, v, s.position(), s.radius()),
            ImmutableSurface::Cone(s) => {
                elslib::cone_value(u, v, s.position(), s.ref_radius(), s.semi_angle())
            }
            ImmutableSurface::Sphere(s) => elslib::sphere_value(u, v, s.position(), s.radius()),
            ImmutableSurface::Torus(s) => {
                elslib::torus_value(u, v, s.position(), s.major_radius(), s.minor_radius())
            }
        }
    }

    /// Lock-free first partial derivatives `(S(u, v), dS/du, dS/dv)` (#65).
    pub fn d1(&self, u: f64, v: f64) -> (Point3, Vector3, Vector3) {
        match &*self.inner {
            ImmutableSurface::Plane(s) => elslib::plane_d1(u, v, s.position()),
            ImmutableSurface::Cylinder(s) => elslib::cylinder_d1(u, v, s.position(), s.radius()),
            ImmutableSurface::Cone(s) => {
                elslib::cone_d1(u, v, s.position(), s.ref_radius(), s.semi_angle())
            }
            ImmutableSurface::Sphere(s) => elslib::sphere_d1(u, v, s.position(), s.radius()),
            ImmutableSurface::Torus(s) => {
                elslib::torus_d1(u, v, s.position(), s.major_radius(), s.minor_radius())
            }
        }
    }

    /// Opens an operation-local copy-on-write builder without mutating this handle (#65).
    pub fn to_builder(&self) -> SurfaceBuilder {
        SurfaceBuilder {
            inner: (*self.inner).clone(),
        }
    }
}

/// Operation-local mutable builder producing a new [`PublishedSurface`] (#65).
#[derive(Clone, Debug)]
pub struct SurfaceBuilder {
    inner: ImmutableSurface,
}

impl SurfaceBuilder {
    /// Applies a 3D transformation to this builder copy.
    pub fn transform(&mut self, trsf: &Transform3) -> &mut Self {
        self.inner = match &self.inner {
            ImmutableSurface::Plane(s) => ImmutableSurface::Plane(s.transformed(*trsf)),
            ImmutableSurface::Cylinder(s) => ImmutableSurface::Cylinder(s.transformed(*trsf)),
            ImmutableSurface::Cone(s) => ImmutableSurface::Cone(s.transformed(*trsf)),
            ImmutableSurface::Sphere(s) => ImmutableSurface::Sphere(s.transformed(*trsf)),
            ImmutableSurface::Torus(s) => ImmutableSurface::Torus(s.transformed(*trsf)),
        };
        self
    }

    /// Publishes the edited surface as a new lock-free [`PublishedSurface`].
    pub fn publish(self) -> PublishedSurface {
        PublishedSurface {
            inner: Arc::new(self.inner),
        }
    }
}

fn eval_d0_3d(curve: &ImmutableCurve3d, u: f64) -> Point3 {
    match curve {
        ImmutableCurve3d::Line(c) => elclib::line_value(u, c.position()),
        ImmutableCurve3d::Circle(c) => elclib::circle_value(u, c.position(), c.radius()),
        ImmutableCurve3d::Ellipse(c) => {
            elclib::ellipse_value(u, c.position(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve3d::Hyperbola(c) => {
            elclib::hyperbola_value(u, c.position(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve3d::Parabola(c) => elclib::parabola_value(u, c.position(), c.focal()),
        ImmutableCurve3d::Trimmed { basis, .. } => eval_d0_3d(basis, u),
    }
}

fn eval_d1_3d(curve: &ImmutableCurve3d, u: f64) -> (Point3, Vector3) {
    match curve {
        ImmutableCurve3d::Line(c) => elclib::line_d1(u, c.position()),
        ImmutableCurve3d::Circle(c) => elclib::circle_d1(u, c.position(), c.radius()),
        ImmutableCurve3d::Ellipse(c) => {
            elclib::ellipse_d1(u, c.position(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve3d::Hyperbola(c) => {
            elclib::hyperbola_d1(u, c.position(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve3d::Parabola(c) => elclib::parabola_d1(u, c.position(), c.focal()),
        ImmutableCurve3d::Trimmed { basis, .. } => eval_d1_3d(basis, u),
    }
}

fn eval_d2_3d(curve: &ImmutableCurve3d, u: f64) -> (Point3, Vector3, Vector3) {
    match curve {
        ImmutableCurve3d::Line(c) => {
            let (p, v1) = elclib::line_d1(u, c.position());
            (p, v1, Vector3::new(0.0, 0.0, 0.0))
        }
        ImmutableCurve3d::Circle(c) => elclib::circle_d2(u, c.position(), c.radius()),
        ImmutableCurve3d::Ellipse(c) => {
            elclib::ellipse_d2(u, c.position(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve3d::Hyperbola(c) => {
            elclib::hyperbola_d2(u, c.position(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve3d::Parabola(c) => elclib::parabola_d2(u, c.position(), c.focal()),
        ImmutableCurve3d::Trimmed { basis, .. } => eval_d2_3d(basis, u),
    }
}

fn transform_3d(curve: &ImmutableCurve3d, trsf: &Transform3) -> Result<ImmutableCurve3d, Failure> {
    Ok(match curve {
        ImmutableCurve3d::Line(c) => ImmutableCurve3d::Line(c.transformed(*trsf)),
        ImmutableCurve3d::Circle(c) => ImmutableCurve3d::Circle(c.transformed(*trsf)?),
        ImmutableCurve3d::Ellipse(c) => ImmutableCurve3d::Ellipse(c.transformed(*trsf)?),
        ImmutableCurve3d::Hyperbola(c) => ImmutableCurve3d::Hyperbola(c.transformed(*trsf)?),
        ImmutableCurve3d::Parabola(c) => ImmutableCurve3d::Parabola(c.transformed(*trsf)?),
        ImmutableCurve3d::Trimmed { basis, first, last } => ImmutableCurve3d::Trimmed {
            basis: Arc::new(transform_3d(basis, trsf)?),
            first: *first,
            last: *last,
        },
    })
}
