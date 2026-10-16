//! GeomAbs enumerations shared by curve, surface and approximation algorithms.
//!
//! Discriminants keep the declaration order, so `as i32` yields the
//! integer stored for the corresponding enumerator.

/// Continuity of a curve or surface (`Shape`); `Shape::C2 as i32 == 4`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(i32)]
pub enum Shape {
    /// Only geometric continuity of position.
    C0,
    /// Tangent directions are continuous.
    G1,
    /// First derivatives are continuous.
    C1,
    /// Curvature is continuous.
    G2,
    /// Second derivatives are continuous.
    C2,
    /// Third derivatives are continuous.
    C3,
    /// Infinitely continuous.
    CN,
}

/// Kind of curve behind an adaptor (`CurveType`); `CurveType::Line as i32 == 0`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum CurveType {
    /// Straight line.
    Line,
    /// Circle.
    Circle,
    /// Ellipse.
    Ellipse,
    /// Hyperbola.
    Hyperbola,
    /// Parabola.
    Parabola,
    /// Bezier curve.
    BezierCurve,
    /// B-spline curve.
    BSplineCurve,
    /// Offset curve.
    OffsetCurve,
    /// Any other curve.
    OtherCurve,
}

/// Kind of surface behind an adaptor (`SurfaceType`); `SurfaceType::Torus as i32 == 4`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum SurfaceType {
    /// Plane.
    Plane,
    /// Cylinder.
    Cylinder,
    /// Cone.
    Cone,
    /// Sphere.
    Sphere,
    /// Torus.
    Torus,
    /// Bezier surface.
    BezierSurface,
    /// B-spline surface.
    BSplineSurface,
    /// Surface of revolution.
    SurfaceOfRevolution,
    /// Surface of linear extrusion.
    SurfaceOfExtrusion,
    /// Offset surface.
    OffsetSurface,
    /// Any other surface.
    OtherSurface,
}

/// Isoparametric direction (`IsoType`); `IsoType::IsoV as i32 == 1`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum IsoType {
    /// Iso-U curve.
    IsoU,
    /// Iso-V curve.
    IsoV,
    /// Not an isoparametric curve.
    NoneIso,
}

/// Offset corner treatment (`JoinType`); `JoinType::Intersection as i32 == 2`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum JoinType {
    /// Corners are filled with arcs.
    Arc,
    /// Corners are filled with tangent extensions.
    Tangent,
    /// Corners are filled by intersecting the offset elements.
    Intersection,
}

/// Knot distribution of a B-spline (`BSplKnotDistribution`); `BSplKnotDistribution::Uniform as i32 == 1`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum BSplKnotDistribution {
    /// Arbitrary knot spacing.
    NonUniform,
    /// Uniform knots with uniform multiplicities.
    Uniform,
    /// Uniform interior with multiplicity degree+1 end knots.
    QuasiUniform,
    /// Every knot has multiplicity degree.
    PiecewiseBezier,
}
