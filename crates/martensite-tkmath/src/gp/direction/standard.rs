//! Direction3::D, the axis-aligned 3D directions, and the frame helpers that combine them.
use super::Direction3;
use StandardDirection3::*;
/// Direction3::D; `Direction3::from_standard(StandardDirection3::Z)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StandardDirection3 {
    /// Positive X axis `(1, 0, 0)`.
    X,
    /// Positive Y axis `(0, 1, 0)`.
    Y,
    /// Positive Z axis `(0, 0, 1)`.
    Z,
    /// Negative X axis `(-1, 0, 0)`.
    NegX,
    /// Negative Y axis `(0, -1, 0)`.
    NegY,
    /// Negative Z axis `(0, 0, -1)`.
    NegZ,
}
impl StandardDirection3 {
    /// Axis2/Axis3::get_perpendicular_x_dir: the X direction of a frame with this main
    /// direction; `StandardDirection3::Z.perpendicular_x()`.
    pub const fn perpendicular_x(self) -> Self {
        match self {
            X | NegX => Y,
            Y | NegY => Z,
            _ => X,
        }
    }
    /// Axis2/Axis3::get_perpendicular_y_dir: main x perpendicular X; `StandardDirection3::Z.perpendicular_y()`.
    pub const fn perpendicular_y(self) -> Self {
        match self {
            Z => Y,
            NegZ => NegY,
            X => Z,
            NegX => NegZ,
            Y => X,
            NegY => NegX,
        }
    }
    /// Axis2/Axis3::cross_standard_dir: `self x other`. Parallel pairs have no product and
    /// return the fallback, so such a frame is not orthogonal;
    /// `StandardDirection3::Z.cross(StandardDirection3::X)`.
    pub const fn cross(self, other: Self) -> Self {
        match (self, other) {
            (X, Y) | (Y, NegX) | (NegX, NegY) | (NegY, X) => Z,
            (X, NegY) | (Y, X) | (NegX, Y) | (NegY, NegX) => NegZ,
            (Y, Z) | (Z, NegY) | (NegY, NegZ) | (NegZ, Y) => X,
            (Y, NegZ) | (Z, Y) | (NegY, Z) | (NegZ, NegY) => NegX,
            (X, NegZ) | (Z, X) | (NegX, Z) | (NegZ, NegX) => Y,
            (X, Z) | (Z, NegX) | (NegX, NegZ) | (NegZ, X) => NegY,
            _ => self.parallel_fallback(),
        }
    }
    /// The value each `crossStandardDir` branch returns for a parallel second argument.
    const fn parallel_fallback(self) -> Self {
        match self {
            X => Z,
            Y => X,
            Z => Y,
            NegX => NegZ,
            NegY => NegX,
            NegZ => NegY,
        }
    }
}
impl Direction3 {
    /// Direction3(Direction3::D): exact unit components; `Direction3::from_standard(StandardDirection3::NegZ)`.
    pub const fn from_standard(direction: StandardDirection3) -> Self {
        let coordinates = match direction {
            X => [1.0, 0.0, 0.0],
            Y => [0.0, 1.0, 0.0],
            Z => [0.0, 0.0, 1.0],
            NegX => [-1.0, 0.0, 0.0],
            NegY => [0.0, -1.0, 0.0],
            NegZ => [0.0, 0.0, -1.0],
        };
        Self { coordinates }
    }
}
impl From<StandardDirection3> for Direction3 {
    fn from(direction: StandardDirection3) -> Self {
        Self::from_standard(direction)
    }
}
