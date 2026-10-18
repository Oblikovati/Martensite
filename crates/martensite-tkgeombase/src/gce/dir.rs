//! MakeDir and MakeDir2d: construction of Direction3 and Dir2d.
use super::{ErrorType, gce_result};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{
    self, Direction2, Direction3, Point2, Point3, Vector2, Vector3, Xy, Xyz,
};

/// MakeDir: builds a direction from a vector, coordinates or two points;
/// NullVector or ConfusedPoints when the length is at most
/// gp::Resolution().
///
/// `let d = MakeDir::from_coordinates(1.0, 0.0, 0.0)?.value()?;`
#[derive(Clone, Copy, Debug, Default)]
pub struct MakeDir {
    the_result: Direction3,
    the_error: ErrorType,
}
gce_result!(MakeDir, Direction3, "gce_MakeDir");

impl MakeDir {
    /// The maker for a checked length: `length <= gp::Resolution()` gives
    /// `failure`, otherwise `build` normalizes (and raises as Direction3 does).
    fn checked(
        length: f64,
        failure: ErrorType,
        build: impl FnOnce() -> Result<Direction3, Failure>,
    ) -> Result<Self, Failure> {
        if length <= gp::resolution() {
            return Ok(Self {
                the_error: failure,
                ..Self::default()
            });
        }
        Ok(Self {
            the_result: build()?,
            the_error: ErrorType::Done,
        })
    }

    /// MakeDir(V): the direction of `vector`. `MakeDir::from_vector(v)?`.
    pub fn from_vector(vector: Vector3) -> Result<Self, Failure> {
        Self::checked(vector.magnitude(), ErrorType::NullVector, || {
            Direction3::from_vector(vector)
        })
    }

    /// MakeDir(Coord): the direction of `coord`. `MakeDir::from_xyz(xyz)?`.
    pub fn from_xyz(coord: Xyz) -> Result<Self, Failure> {
        Self::checked(coord.modulus(), ErrorType::NullVector, || {
            Direction3::from_xyz(coord)
        })
    }

    /// MakeDir(Xv, Yv, Zv): the direction of the coordinates; the null test
    /// compares the square length with gp::Resolution().
    /// `MakeDir::from_coordinates(1.0, 0.0, 0.0)?`.
    pub fn from_coordinates(xv: f64, yv: f64, zv: f64) -> Result<Self, Failure> {
        let square: f64 = xv * xv + yv * yv + zv * zv;
        Self::checked(square, ErrorType::NullVector, || {
            Direction3::new(xv, yv, zv)
        })
    }

    /// MakeDir(P1, P2): the direction from `p1` to `p2`.
    /// `MakeDir::from_points(a, b)?`.
    pub fn from_points(p1: Point3, p2: Point3) -> Result<Self, Failure> {
        let length: f64 = p1.distance(&p2);
        Self::checked(length, ErrorType::ConfusedPoints, || {
            Direction3::from_xyz(p2.xyz() - p1.xyz())
        })
    }
}

/// MakeDir2d: builds a 2D direction from a vector, coordinates or two
/// points; NullVector or ConfusedPoints when the length is at most
/// gp::Resolution().
///
/// `let d = MakeDir2d::from_coordinates(1.0, 0.0)?.value()?;`
#[derive(Clone, Copy, Debug, Default)]
pub struct MakeDir2d {
    the_result: Direction2,
    the_error: ErrorType,
}
gce_result!(MakeDir2d, Direction2, "gce_MakeDir2d");

impl MakeDir2d {
    /// The maker for a checked length, as [`MakeDir`]'s.
    fn checked(
        length: f64,
        failure: ErrorType,
        build: impl FnOnce() -> Result<Direction2, Failure>,
    ) -> Result<Self, Failure> {
        if length <= gp::resolution() {
            return Ok(Self {
                the_error: failure,
                ..Self::default()
            });
        }
        Ok(Self {
            the_result: build()?,
            the_error: ErrorType::Done,
        })
    }

    /// MakeDir2d(V): the direction of `vector`. `MakeDir2d::from_vector(v)?`.
    pub fn from_vector(vector: Vector2) -> Result<Self, Failure> {
        Self::checked(vector.magnitude(), ErrorType::NullVector, || {
            Direction2::from_vector(vector)
        })
    }

    /// MakeDir2d(Coord): the direction of `coord`. `MakeDir2d::from_xy(xy)?`.
    pub fn from_xy(coord: Xy) -> Result<Self, Failure> {
        Self::checked(coord.modulus(), ErrorType::NullVector, || {
            Direction2::from_xy(coord)
        })
    }

    /// MakeDir2d(Xv, Yv): the direction of the coordinates; the null test
    /// compares the square length with gp::Resolution().
    /// `MakeDir2d::from_coordinates(1.0, 0.0)?`.
    pub fn from_coordinates(xv: f64, yv: f64) -> Result<Self, Failure> {
        Self::checked(xv * xv + yv * yv, ErrorType::NullVector, || {
            Direction2::new(xv, yv)
        })
    }

    /// MakeDir2d(P1, P2): the direction from `p1` to `p2`.
    /// `MakeDir2d::from_points(a, b)?`.
    pub fn from_points(p1: Point2, p2: Point2) -> Result<Self, Failure> {
        let length: f64 = p1.distance(p2);
        Self::checked(length, ErrorType::ConfusedPoints, || {
            Direction2::from_xy(p2.xy() - p1.xy())
        })
    }
}
