// SPDX-License-Identifier: GPL-2.0-only
//! Immutable, lock-free published 2D geometry (#65).
//!
//! Published handles wrap `Arc<ImmutableCurve2d>` with no `RwLock` and no setters.
//! Edits happen through an operation-local [`Curve2dBuilder`] that publishes a new
//! handle without affecting concurrent readers of prior revisions.
use crate::geom2d::{Geom2dCircle, Geom2dEllipse, Geom2dHyperbola, Geom2dLine, Geom2dParabola};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{
    Circle2, Ellipse2, Hyperbola2, Line2, Parabola2, Point2, Transform2, Vector2,
};
use std::f64::consts::TAU;
use std::sync::Arc;

#[derive(Clone, Debug)]
enum ImmutableCurve2d {
    Line(Line2),
    Circle(Circle2),
    Ellipse(Ellipse2),
    Hyperbola(Hyperbola2),
    Parabola(Parabola2),
    Trimmed {
        basis: Arc<ImmutableCurve2d>,
        first: f64,
        last: f64,
    },
}

/// Immutable, `Send + Sync`, lock-free published 2D curve (#65).
#[derive(Clone, Debug)]
pub struct PublishedCurve2d {
    inner: Arc<ImmutableCurve2d>,
}

impl PublishedCurve2d {
    /// Publishes an immutable 2D line.
    pub fn line(line: Line2) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve2d::Line(line)),
        }
    }

    /// Publishes an immutable 2D circle.
    pub fn circle(circle: Circle2) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve2d::Circle(circle)),
        }
    }

    /// Publishes an immutable 2D ellipse.
    pub fn ellipse(ellipse: Ellipse2) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve2d::Ellipse(ellipse)),
        }
    }

    /// Publishes an immutable 2D hyperbola.
    pub fn hyperbola(hyperbola: Hyperbola2) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve2d::Hyperbola(hyperbola)),
        }
    }

    /// Publishes an immutable 2D parabola.
    pub fn parabola(parabola: Parabola2) -> Self {
        Self {
            inner: Arc::new(ImmutableCurve2d::Parabola(parabola)),
        }
    }

    /// Publishes a trimmed view of an existing published 2D curve without copying.
    pub fn trimmed(&self, first: f64, last: f64) -> Result<Self, Failure> {
        if !first.is_finite() || !last.is_finite() || last <= first {
            return Err(Failure::with_message(
                "Standard_ConstructionError",
                "trimmed curve bounds must be finite with first < last",
            ));
        }
        Ok(Self {
            inner: Arc::new(ImmutableCurve2d::Trimmed {
                basis: Arc::clone(&self.inner),
                first,
                last,
            }),
        })
    }

    /// Snapshots a legacy mutable `Geom2dLine` into an immutable published curve.
    pub fn from_geom_line(curve: &Geom2dLine) -> Self {
        Self::line(curve.lin2d())
    }

    /// Snapshots a legacy mutable `Geom2dCircle` into an immutable published curve.
    pub fn from_geom_circle(curve: &Geom2dCircle) -> Result<Self, Failure> {
        Ok(Self::circle(curve.circ2d()?))
    }

    /// Snapshots a legacy mutable `Geom2dEllipse` into an immutable published curve.
    pub fn from_geom_ellipse(curve: &Geom2dEllipse) -> Result<Self, Failure> {
        Ok(Self::ellipse(curve.elips2d()?))
    }

    /// Snapshots a legacy mutable `Geom2dHyperbola` into an immutable published curve.
    pub fn from_geom_hyperbola(curve: &Geom2dHyperbola) -> Result<Self, Failure> {
        Ok(Self::hyperbola(curve.hypr2d()?))
    }

    /// Snapshots a legacy mutable `Geom2dParabola` into an immutable published curve.
    pub fn from_geom_parabola(curve: &Geom2dParabola) -> Result<Self, Failure> {
        Ok(Self::parabola(curve.parab2d()?))
    }

    /// Lock-free position evaluation `C(u)` (#65).
    pub fn d0(&self, u: f64) -> Point2 {
        eval_d0_2d(&self.inner, u)
    }

    /// Lock-free first derivative evaluation `(C(u), C'(u))` (#65).
    pub fn d1(&self, u: f64) -> (Point2, Vector2) {
        eval_d1_2d(&self.inner, u)
    }

    /// Lock-free second derivative evaluation `(C(u), C'(u), C''(u))` (#65).
    pub fn d2(&self, u: f64) -> (Point2, Vector2, Vector2) {
        eval_d2_2d(&self.inner, u)
    }

    /// First parameter of the curve domain.
    pub fn first_parameter(&self) -> f64 {
        match &*self.inner {
            ImmutableCurve2d::Line(_)
            | ImmutableCurve2d::Hyperbola(_)
            | ImmutableCurve2d::Parabola(_) => -precision::infinite(),
            ImmutableCurve2d::Circle(_) | ImmutableCurve2d::Ellipse(_) => 0.0,
            ImmutableCurve2d::Trimmed { first, .. } => *first,
        }
    }

    /// Last parameter of the curve domain.
    pub fn last_parameter(&self) -> f64 {
        match &*self.inner {
            ImmutableCurve2d::Line(_)
            | ImmutableCurve2d::Hyperbola(_)
            | ImmutableCurve2d::Parabola(_) => precision::infinite(),
            ImmutableCurve2d::Circle(_) | ImmutableCurve2d::Ellipse(_) => TAU,
            ImmutableCurve2d::Trimmed { last, .. } => *last,
        }
    }

    /// Opens an operation-local copy-on-write builder without mutating this handle (#65).
    pub fn to_builder(&self) -> Curve2dBuilder {
        Curve2dBuilder {
            inner: (*self.inner).clone(),
        }
    }
}

/// Operation-local mutable builder producing a new [`PublishedCurve2d`] (#65).
#[derive(Clone, Debug)]
pub struct Curve2dBuilder {
    inner: ImmutableCurve2d,
}

impl Curve2dBuilder {
    /// Applies a 2D transformation to this builder copy.
    pub fn transform(&mut self, trsf: &Transform2) -> &mut Self {
        self.inner = transform_2d(&self.inner, trsf);
        self
    }

    /// Publishes the edited geometry as a new lock-free [`PublishedCurve2d`].
    pub fn publish(self) -> PublishedCurve2d {
        PublishedCurve2d {
            inner: Arc::new(self.inner),
        }
    }
}

fn eval_d0_2d(curve: &ImmutableCurve2d, u: f64) -> Point2 {
    match curve {
        ImmutableCurve2d::Line(c) => elclib::line_value_2d(u, c.position()),
        ImmutableCurve2d::Circle(c) => elclib::circle_value_2d(u, c.position(), c.radius()),
        ImmutableCurve2d::Ellipse(c) => {
            elclib::ellipse_value_2d(u, c.axis(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve2d::Hyperbola(c) => {
            elclib::hyperbola_value_2d(u, c.axis(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve2d::Parabola(c) => elclib::parabola_value_2d(u, c.axis(), c.focal()),
        ImmutableCurve2d::Trimmed { basis, .. } => eval_d0_2d(basis, u),
    }
}

fn eval_d1_2d(curve: &ImmutableCurve2d, u: f64) -> (Point2, Vector2) {
    match curve {
        ImmutableCurve2d::Line(c) => elclib::line_d1_2d(u, c.position()),
        ImmutableCurve2d::Circle(c) => elclib::circle_d1_2d(u, c.position(), c.radius()),
        ImmutableCurve2d::Ellipse(c) => {
            elclib::ellipse_d1_2d(u, c.axis(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve2d::Hyperbola(c) => {
            elclib::hyperbola_d1_2d(u, c.axis(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve2d::Parabola(c) => elclib::parabola_d1_2d(u, c.axis(), c.focal()),
        ImmutableCurve2d::Trimmed { basis, .. } => eval_d1_2d(basis, u),
    }
}

fn eval_d2_2d(curve: &ImmutableCurve2d, u: f64) -> (Point2, Vector2, Vector2) {
    match curve {
        ImmutableCurve2d::Line(c) => {
            let (p, v1) = elclib::line_d1_2d(u, c.position());
            (p, v1, Vector2::new(0.0, 0.0))
        }
        ImmutableCurve2d::Circle(c) => elclib::circle_d2_2d(u, c.position(), c.radius()),
        ImmutableCurve2d::Ellipse(c) => {
            elclib::ellipse_d2_2d(u, c.axis(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve2d::Hyperbola(c) => {
            elclib::hyperbola_d2_2d(u, c.axis(), c.major_radius(), c.minor_radius())
        }
        ImmutableCurve2d::Parabola(c) => elclib::parabola_d2_2d(u, c.axis(), c.focal()),
        ImmutableCurve2d::Trimmed { basis, .. } => eval_d2_2d(basis, u),
    }
}

fn transform_2d(curve: &ImmutableCurve2d, trsf: &Transform2) -> ImmutableCurve2d {
    match curve {
        ImmutableCurve2d::Line(c) => ImmutableCurve2d::Line(c.transformed(*trsf)),
        ImmutableCurve2d::Circle(c) => ImmutableCurve2d::Circle(c.transformed(*trsf)),
        ImmutableCurve2d::Ellipse(c) => ImmutableCurve2d::Ellipse(c.transformed(*trsf)),
        ImmutableCurve2d::Hyperbola(c) => ImmutableCurve2d::Hyperbola(c.transformed(*trsf)),
        ImmutableCurve2d::Parabola(c) => ImmutableCurve2d::Parabola(c.transformed(*trsf)),
        ImmutableCurve2d::Trimmed { basis, first, last } => ImmutableCurve2d::Trimmed {
            basis: Arc::new(transform_2d(basis, trsf)),
            first: *first,
            last: *last,
        },
    }
}
