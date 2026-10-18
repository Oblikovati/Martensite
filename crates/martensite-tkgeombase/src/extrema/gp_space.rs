//! The `ThePoint`/`TheVector` parameters of the Extrema class templates: the members
//! of `Point3`/`Vector3` and `Point2`/`Vec2d` that the generic code calls.
use martensite_tkmath::gp::{Point2, Point3, Vector2, Vector3};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A gp point of the Extrema templates (`Point3` or `Point2`).
pub trait GpPoint: Copy + Default + Debug + PartialEq + 'static {
    /// The matching gp vector.
    type Vector: GpVector;
    /// 3 or 2.
    const DIMENSION: usize;
    /// `Coord(index + 1)` (`X()`, `Y()`, `Z()`); `point.coordinate(0)`.
    fn coordinate(&self, index: usize) -> f64;
    /// `SquareDistance(theOther)`; `point.square_distance_to(&other)`.
    fn square_distance_to(&self, other: &Self) -> f64;
    /// `Distance(theOther)`; `point.distance_to(&other)`.
    fn distance_to(&self, other: &Self) -> f64;
    /// `Vector3(*this, theTarget)`; `point.vector_to(&target)`.
    fn vector_to(&self, target: &Self) -> Self::Vector;
}

/// A gp vector of the Extrema templates (`Vector3` or `Vec2d`), with the retained
/// operators.
pub trait GpVector:
    Copy
    + Default
    + Debug
    + 'static
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + Neg<Output = Self>
{
    /// `Dot(theOther)`; `vector.dot_with(&other)`.
    fn dot_with(&self, other: &Self) -> f64;
    /// `Coord(index + 1)` (`X()`, `Y()`, `Z()`); `vector.coordinate(0)`.
    fn coordinate(&self, index: usize) -> f64;
    /// `Magnitude()`; `vector.norm()`.
    fn norm(&self) -> f64;
}

impl GpPoint for Point3 {
    type Vector = Vector3;
    const DIMENSION: usize = 3;
    fn coordinate(&self, index: usize) -> f64 {
        [self.x(), self.y(), self.z()][index]
    }
    fn square_distance_to(&self, other: &Self) -> f64 {
        self.square_distance(other)
    }
    fn distance_to(&self, other: &Self) -> f64 {
        self.distance(other)
    }
    fn vector_to(&self, target: &Self) -> Vector3 {
        Vector3::between(*self, *target)
    }
}

impl GpPoint for Point2 {
    type Vector = Vector2;
    const DIMENSION: usize = 2;
    fn coordinate(&self, index: usize) -> f64 {
        [self.x(), self.y()][index]
    }
    fn square_distance_to(&self, other: &Self) -> f64 {
        self.square_distance(*other)
    }
    fn distance_to(&self, other: &Self) -> f64 {
        self.distance(*other)
    }
    fn vector_to(&self, target: &Self) -> Vector2 {
        Vector2::between(*self, *target)
    }
}

impl GpVector for Vector3 {
    fn dot_with(&self, other: &Self) -> f64 {
        self.dot(*other)
    }
    fn coordinate(&self, index: usize) -> f64 {
        [self.x(), self.y(), self.z()][index]
    }
    fn norm(&self) -> f64 {
        self.magnitude()
    }
}

impl GpVector for Vector2 {
    fn dot_with(&self, other: &Self) -> f64 {
        self.dot(*other)
    }
    fn coordinate(&self, index: usize) -> f64 {
        [self.x(), self.y()][index]
    }
    fn norm(&self) -> f64 {
        self.magnitude()
    }
}
