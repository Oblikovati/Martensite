//! Geometric primitives with explicit numeric behavior.
mod axis;
pub(crate) mod direction;
mod point;
pub use axis::Axis3;
pub use direction::{Direction3, StandardDirection3};
use martensite_tkernel::standard::Failure;
pub use point::{Point3, PointCoordinatesMut, PointHashPolicy};
pub(crate) fn cross(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}
pub(crate) fn dot(left: [f64; 3], right: [f64; 3]) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}
/// Normalizes a finite nonzero vector without squared-norm overflow or underflow (#550).
pub fn normalize_finite<const N: usize>(coordinates: [f64; N]) -> Option<[f64; N]> {
    if coordinates.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let squared = coordinates.iter().fold(0.0, |acc, &v| acc + v * v);
    if squared == 1.0 {
        return Some(coordinates);
    }
    if squared.is_finite() && squared > f64::MIN_POSITIVE {
        let magnitude = squared.sqrt();
        return Some(coordinates.map(|value| martensite_cmath::quotient(value, magnitude)));
    }
    let scale = coordinates.iter().fold(0.0_f64, |acc, v| acc.max(v.abs()));
    if scale == 0.0 {
        return None;
    }
    let scaled = coordinates.map(|value| martensite_cmath::quotient(value, scale));
    let magnitude = scaled.iter().fold(0.0, |acc, &v| acc + v * v).sqrt();
    Some(scaled.map(|value| martensite_cmath::quotient(value, magnitude)))
}
/// Reflects `value` about the unit line `normal` with the precomputed terms and operation
/// order shared by Direction3::mirror_direction and Vector3::mirror_vector; `reflect([1.0,0.0,0.0],[1.0,2.0,3.0])`.
pub(crate) fn reflect(normal: [f64; 3], value: [f64; 3]) -> [f64; 3] {
    let [a, b, c] = normal;
    let [x, y, z] = value;
    let (m1, m2, m3) = (2.0 * a * b, 2.0 * a * c, 2.0 * b * c);
    [
        ((2.0 * a * a) - 1.0) * x + m1 * y + m2 * z,
        m1 * x + ((2.0 * b * b) - 1.0) * y + m3 * z,
        m2 * x + m3 * y + ((2.0 * c * c) - 1.0) * z,
    ]
}
fn construction(message: &str) -> Failure {
    Failure::with_type("Standard_ConstructionError", message, "")
}
fn domain(message: &str) -> Failure {
    Failure::with_type("Standard_DomainError", message, "")
}
/// The vector-with-null-magnitude exception, derived from `domain failure`: the declared
/// base that lets `Failure::is_kind` answer a catch by base kind.
const VECTOR_WITH_NULL_MAGNITUDE_PARENT: &str = "Standard_DomainError";
/// Raises null-vector-magnitude failure, the Vector3/Vector2 angular-query exception (a
/// domain failure subclass); the one-argument constructor, with the stack
/// capture; `gp::vector_with_null_magnitude("magnitude 0")`.
pub fn vector_with_null_magnitude(message: &str) -> Failure {
    Failure::with_message("gp_VectorWithNullMagnitude", message)
        .with_exception_parent(VECTOR_WITH_NULL_MAGNITUDE_PARENT)
}
/// The `null-vector-magnitude failure(theMessage, theStackTrace)` constructor: an explicit stack,
/// no capture; `gp::vector_with_null_magnitude_with_stack("magnitude 0", "trace")`.
pub fn vector_with_null_magnitude_with_stack(message: &str, stack: &str) -> Failure {
    Failure::with_type("gp_VectorWithNullMagnitude", message, stack)
        .with_exception_parent(VECTOR_WITH_NULL_MAGNITUDE_PARENT)
}
mod placement;

mod transform;
pub use transform::{Transform3, TransformForm, TransformNLerp, UnitPlacementError};
mod xyz;
pub use xyz::Xyz;
mod matrix;
pub use matrix::Matrix3;

mod axis1;
mod axis2;
pub use axis1::Axis1;
pub use axis2::Axis2;
mod vector;
pub use vector::Vector3;
mod general_transform;
pub use general_transform::GeneralTransform3;
mod planar;
pub use planar::{
    Axis2d, Axis22d, Direction2, GeneralTransform2, Matrix2, Point2, StandardDirection2,
    Transform2, Vector2, Xy,
};
mod line;
pub use line::Line3;
pub use planar::{Circle2, Ellipse2, Hyperbola2, Line2, Parabola2};

mod plane;
pub use plane::Plane3;

mod circle;
pub use circle::Circle3;
mod ellipse;
pub use ellipse::Ellipse3;
mod hyperbola;
pub use hyperbola::Hyperbola3;
mod parabola;
pub use parabola::Parabola3;
mod cone;
mod cylinder;
mod sphere;
mod torus;
pub use cone::Cone3;
pub use cylinder::Cylinder3;
pub use sphere::Sphere3;
pub use torus::Torus3;

mod quaternion;
pub use quaternion::{EulerSequence, Quaternion, QuaternionNLerp, QuaternionSLerp};
mod standard_frames;
pub use standard_frames::{
    dx, dx2d, dy, dy2d, dz, origin, origin2d, ox, ox2d, oy, oy2d, oz, resolution, xoy, yoz, zox,
};
