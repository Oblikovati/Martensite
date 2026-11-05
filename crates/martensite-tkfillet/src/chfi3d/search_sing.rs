//! `ChFi3dSearchSing`: the function whose roots are the parameters where two curves
//! (two sections of a fillet) are closest, `F(x) = (C2(x) - C1(x)). (C2'(x) - C1'(x))`.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkmath::gp::Vector3;
use martensite_tkmath::math::{Function, FunctionWithDerivative};

/// `ChFi3dSearchSing`: half the derivative of the squared distance between the points
/// of the same parameter on two curves, for `math_FunctionRoot`. A curve failure ends
/// the evaluation with false and is kept for `take_failure` (natively it propagates
/// out of the solver).
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkfillet::chfi3d::ChFi3dSearchSing;
/// use martensite_tkg3d::geom::{GeomCurve, GeomLine};
/// use martensite_tkmath::gp::{Axis1, Direction3, Point3};
/// use martensite_tkmath::math::Function;
/// let line = |y: f64| -> Handle<dyn GeomCurve> {
///     Handle::new(GeomLine::new(Axis1::new(Point3::new(0.0, y, 0.0), Direction3::new(1.0, 0.0, 0.0).unwrap()))).cast()
/// };
/// let mut f = ChFi3dSearchSing::new(&line(0.0), &line(1.0));
/// let mut value = 1.0;
/// assert!(f.value(0.5, &mut value));
/// assert_eq!(value, 0.0);
/// ```
pub struct ChFi3dSearchSing {
    c1: Handle<dyn GeomCurve>,
    c2: Handle<dyn GeomCurve>,
    failure: Option<Failure>,
}

impl ChFi3dSearchSing {
    /// `ChFi3dSearchSing(C1, C2)`; `ChFi3dSearchSing::new(&c1, &c2)`.
    pub fn new(c1: &Handle<dyn GeomCurve>, c2: &Handle<dyn GeomCurve>) -> Self {
        Self {
            c1: c1.clone(),
            c2: c2.clone(),
            failure: None,
        }
    }

    /// The first curve failure met by an evaluation, if any; `f.take_failure()`.
    pub fn take_failure(&mut self) -> Option<Failure> {
        self.failure.take()
    }

    /// `(V, V', V'')` of `V = C2 - C1` at `x`, the second derivative only when asked.
    fn difference(&self, x: f64, second: bool) -> Result<(Vector3, Vector3, Vector3), Failure> {
        if !second {
            let ((p1, v1), (p2, v2)) = (self.c1.d1(x)?, self.c2.d1(x)?);
            return Ok((Vector3::between(p1, p2), v2 - v1, Vector3::default()));
        }
        let ((p1, v1, w1), (p2, v2, w2)) = (self.c1.d2(x)?, self.c2.d2(x)?);
        Ok((Vector3::between(p1, p2), v2 - v1, w2 - w1))
    }

    fn keep(&mut self, failure: Failure) -> bool {
        self.failure.get_or_insert(failure);
        false
    }
}

impl Function for ChFi3dSearchSing {
    /// `Value(X, F)`: `F = V . V'`.
    fn value(&mut self, x: f64, f: &mut f64) -> bool {
        match self.difference(x, false) {
            Ok((v, v_prim, _)) => {
                *f = v.dot(v_prim);
                true
            }
            Err(failure) => self.keep(failure),
        }
    }
}

impl FunctionWithDerivative for ChFi3dSearchSing {
    /// `Derivative(X, D)`: `D = |V'|^2 + V . V''`.
    fn derivative(&mut self, x: f64, d: &mut f64) -> bool {
        match self.difference(x, true) {
            Ok((v, v_prim, v_second)) => {
                *d = v_prim.square_magnitude() + v.dot(v_second);
                true
            }
            Err(failure) => self.keep(failure),
        }
    }

    /// `Values(X, F, D)`: both from one second-order evaluation.
    fn values(&mut self, x: f64, f: &mut f64, d: &mut f64) -> bool {
        match self.difference(x, true) {
            Ok((v, v_prim, v_second)) => {
                *f = v.dot(v_prim);
                *d = v_prim.square_magnitude() + v.dot(v_second);
                true
            }
            Err(failure) => self.keep(failure),
        }
    }
}
