//! Numeric assertion assertion semantics and gp shorthands shared by the retained
//! ExtremaPC and ExtremaPC2d suites.
#![allow(non_snake_case, dead_code)]
use martensite_tkmath::gp::{Direction2, Direction3};

/// Numeric assertion `EXPECT_NEAR`: `|a - b| <= tolerance`.
#[track_caller]
pub fn EXPECT_NEAR(a: f64, b: f64, tolerance: f64) {
    assert!(
        (a - b).abs() <= tolerance,
        "EXPECT_NEAR: |{a} - {b}| > {tolerance}"
    );
}

/// Numeric assertion `EXPECT_DOUBLE_EQ`: at most four ULPs apart, never NaN.
#[track_caller]
pub fn EXPECT_DOUBLE_EQ(a: f64, b: f64) {
    assert!(
        !a.is_nan() && !b.is_nan(),
        "EXPECT_DOUBLE_EQ on NaN: {a} vs {b}"
    );
    assert!(
        ordered_bits(a).abs_diff(ordered_bits(b)) <= 4,
        "EXPECT_DOUBLE_EQ: {a} != {b}"
    );
}

/// Numeric assertion `EXPECT_LE`.
#[track_caller]
pub fn EXPECT_LE<T: PartialOrd + std::fmt::Debug>(a: T, b: T) {
    assert!(a <= b, "EXPECT_LE: {a:?} vs {b:?}");
}

/// Numeric assertion `EXPECT_LT`.
#[track_caller]
pub fn EXPECT_LT<T: PartialOrd + std::fmt::Debug>(a: T, b: T) {
    assert!(a < b, "EXPECT_LT: {a:?} vs {b:?}");
}

/// Numeric assertion `EXPECT_GE`.
#[track_caller]
pub fn EXPECT_GE<T: PartialOrd + std::fmt::Debug>(a: T, b: T) {
    assert!(a >= b, "EXPECT_GE: {a:?} vs {b:?}");
}

/// Numeric assertion `EXPECT_GT`.
#[track_caller]
pub fn EXPECT_GT<T: PartialOrd + std::fmt::Debug>(a: T, b: T) {
    assert!(a > b, "EXPECT_GT: {a:?} vs {b:?}");
}

// Maps doubles to integers whose order matches the numeric order (Numeric assertion's
// sign-and-magnitude to biased conversion).
fn ordered_bits(value: f64) -> u64 {
    let bits: u64 = value.to_bits();
    if bits >> 63 != 0 {
        (!bits).wrapping_add(1)
    } else {
        bits | (1 << 63)
    }
}

/// `Dir2d(x, y)`.
pub fn dir2(x: f64, y: f64) -> Direction2 {
    Direction2::new(x, y).unwrap()
}

/// `Direction3(x, y, z)`.
pub fn dir3(x: f64, y: f64, z: f64) -> Direction3 {
    Direction3::new(x, y, z).unwrap()
}

/// The names the retained ExtremaPC suites use.
#[allow(unused_imports)]
pub mod prelude {
    pub use super::{
        EXPECT_DOUBLE_EQ, EXPECT_GE, EXPECT_GT, EXPECT_LE, EXPECT_LT, EXPECT_NEAR, dir2, dir3,
    };
    pub use martensite_tkernel::ncollection::Array1;
    pub use martensite_tkernel::precision;
    pub use martensite_tkernel::standard::Handle;
    pub use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
    pub use martensite_tkg2d::geom2d::{
        Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dOffsetCurve,
    };
    pub use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
    pub use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
    pub use martensite_tkg3d::geom::{
        GeomBSplineCurve, GeomBezierCurve, GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola,
        GeomLine, GeomOffsetCurve, GeomParabola, GeomTrimmedCurve,
    };
    pub use martensite_tkg3d::geom_adaptor::{GeomAdaptorCurve, GeomAdaptorTransformedCurve};
    pub use martensite_tkgeombase::extrema_pc::{
        self as ExtremaPC, Domain1D, ExtremaResult, SearchMode, Status,
    };
    pub use martensite_tkgeombase::extrema_pc2d as ExtremaPC2d;
    pub use martensite_tkmath::elclib;
    pub use martensite_tkmath::gp::{
        Axis1, Axis2, Axis22d, Circle2, Circle3, Direction3, Ellipse2, Ellipse3, Hyperbola2,
        Hyperbola3, Line2, Line3, Parabola2, Parabola3, Point2, Point3, Transform3, Vector2,
        Vector3,
    };
    pub use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    /// The retained default search mode.
    pub const MM: SearchMode = SearchMode::MinMax;
}
