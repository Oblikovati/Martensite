//! The queries of `BlendPoint` (BlendPoint): each raises StandardDomainError when
//! the point has no such support or, for a tangent, is a tangency point.
use super::BlendPoint;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Point3, Vector2, Vector3};

/// The anonymous `StandardDomainError()` of the native queries.
fn domain() -> Failure {
    Failure::with_message("Standard_DomainError", "")
}

/// `value` when `available`, else StandardDomainError.
fn when<T>(available: bool, value: T) -> Result<T, Failure> {
    if available { Ok(value) } else { Err(domain()) }
}

impl BlendPoint {
    /// `Parameter()`: the parameter on the guide line; `point.parameter()`.
    pub fn parameter(&self) -> f64 {
        self.prm
    }

    /// `IsTangencyPoint()`: true when the tangents could not be computed;
    /// `point.is_tangency_point()`.
    pub fn is_tangency_point(&self) -> bool {
        self.istgt
    }

    /// `PointOnS1()`; `point.point_on_s1()`.
    pub fn point_on_s1(&self) -> Point3 {
        self.pt1
    }

    /// `PointOnS2()`; `point.point_on_s2()`.
    pub fn point_on_s2(&self) -> Point3 {
        self.pt2
    }

    /// `ParametersOnS1(U, V)`: StandardDomainError without a first surface;
    /// `let (u, v) = point.parameters_on_s1()?;`.
    pub fn parameters_on_s1(&self) -> Result<(f64, f64), Failure> {
        when(self.hass1, (self.u1, self.v1))
    }

    /// `ParametersOnS2(U, V)`: StandardDomainError without a second surface;
    /// `let (u, v) = point.parameters_on_s2()?;`.
    pub fn parameters_on_s2(&self) -> Result<(f64, f64), Failure> {
        when(self.hass2, (self.u2, self.v2))
    }

    /// `TangentOnS1()`: StandardDomainError on a tangency point; `point.tangent_on_s1()?`.
    pub fn tangent_on_s1(&self) -> Result<Vector3, Failure> {
        when(!self.istgt, self.tg1)
    }

    /// `TangentOnS2()`: StandardDomainError on a tangency point; `point.tangent_on_s2()?`.
    pub fn tangent_on_s2(&self) -> Result<Vector3, Failure> {
        when(!self.istgt, self.tg2)
    }

    /// `Tangent2dOnS1()`: StandardDomainError on a tangency point or without a first
    /// surface; `point.tangent2d_on_s1()?`.
    pub fn tangent2d_on_s1(&self) -> Result<Vector2, Failure> {
        when(
            !self.istgt && self.hass1,
            Vector2::new(self.utg12d, self.vtg12d),
        )
    }

    /// `Tangent2dOnS2()`: StandardDomainError on a tangency point or without a second
    /// surface; `point.tangent2d_on_s2()?`.
    pub fn tangent2d_on_s2(&self) -> Result<Vector2, Failure> {
        when(
            !self.istgt && self.hass2,
            Vector2::new(self.utg22d, self.vtg22d),
        )
    }

    /// `PointOnS()`: the point on the surface of a surface-curve point; `point.point_on_s()`.
    pub fn point_on_s(&self) -> Point3 {
        self.pt1
    }

    /// `PointOnC()`: the point on the curve of a surface-curve point; `point.point_on_c()`.
    pub fn point_on_c(&self) -> Point3 {
        self.pt2
    }

    /// `ParametersOnS(U, V)`: StandardDomainError without a surface;
    /// `let (u, v) = point.parameters_on_s()?;`.
    pub fn parameters_on_s(&self) -> Result<(f64, f64), Failure> {
        when(self.hass1, (self.u1, self.v1))
    }

    /// `ParameterOnC()`: StandardDomainError without a curve; `point.parameter_on_c()?`.
    pub fn parameter_on_c(&self) -> Result<f64, Failure> {
        when(self.hasc2, self.pc2)
    }

    /// `TangentOnS()`: StandardDomainError on a tangency point or without a surface;
    /// `point.tangent_on_s()?`.
    pub fn tangent_on_s(&self) -> Result<Vector3, Failure> {
        when(!self.istgt && self.hass1, self.tg1)
    }

    /// `TangentOnC()`: StandardDomainError on a tangency point; `point.tangent_on_c()?`.
    pub fn tangent_on_c(&self) -> Result<Vector3, Failure> {
        when(!self.istgt, self.tg2)
    }

    /// `Tangent2d()`: StandardDomainError on a tangency point or without a surface;
    /// `point.tangent2d()?`.
    pub fn tangent2d(&self) -> Result<Vector2, Failure> {
        when(
            !self.istgt && self.hass1,
            Vector2::new(self.utg12d, self.vtg12d),
        )
    }

    /// `PointOnC1()`; `point.point_on_c1()`.
    pub fn point_on_c1(&self) -> Point3 {
        self.pt1
    }

    /// `PointOnC2()`; `point.point_on_c2()`.
    pub fn point_on_c2(&self) -> Point3 {
        self.pt2
    }

    /// `ParameterOnC1()`: StandardDomainError without a first curve;
    /// `point.parameter_on_c1()?`.
    pub fn parameter_on_c1(&self) -> Result<f64, Failure> {
        when(self.hasc1, self.pc1)
    }

    /// `ParameterOnC2()`: StandardDomainError without a second curve;
    /// `point.parameter_on_c2()?`.
    pub fn parameter_on_c2(&self) -> Result<f64, Failure> {
        when(self.hasc2, self.pc2)
    }

    /// `TangentOnC1()`: StandardDomainError on a tangency point or without a first
    /// surface (as natively, the check is on the surface flag); `point.tangent_on_c1()?`.
    pub fn tangent_on_c1(&self) -> Result<Vector3, Failure> {
        when(!self.istgt && self.hass1, self.tg1)
    }

    /// `TangentOnC2()`: StandardDomainError on a tangency point; `point.tangent_on_c2()?`.
    pub fn tangent_on_c2(&self) -> Result<Vector3, Failure> {
        when(!self.istgt, self.tg2)
    }
}
