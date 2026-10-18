//! The local frame and the failures that every Geom2dEval curve shares; the
//! numeric helpers of the anonymous namespaces (`powInt`, `trigShiftByQuarterTurns`...)
//! live in `eval_basis`, which GeomEval shares.
use crate::geom2d::errors;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis2d, Point2, Vector2, Xy};

/// The `anO`, `aXD` and `aYD = (-aXD.Y(), aXD.X())` that every analytic eval curve
/// reads from its `Axis2d`; `LocalFrame::new(axis).point(1.0, 0.0)`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalFrame {
    pub(crate) origin: Xy,
    pub(crate) x_dir: Xy,
    pub(crate) y_dir: Xy,
}
impl LocalFrame {
    /// The frame of `position`; `LocalFrame::new(Axis2d::default())`.
    pub(crate) fn new(position: Axis2d) -> Self {
        let x_dir: Xy = position.direction().xy();
        Self {
            origin: position.location().xy(),
            x_dir,
            y_dir: Xy::new(-x_dir.y(), x_dir.x()),
        }
    }
    /// `Point2(anO + a * aXD + b * aYD)`, summed left to right; `frame.point(r * c, r * s)`.
    pub(crate) fn point(&self, a: f64, b: f64) -> Point2 {
        Point2::from(self.origin + a * self.x_dir + b * self.y_dir)
    }
    /// `Vector2(a * aXD + b * aYD)`; `frame.vector(a, b)`.
    pub(crate) fn vector(&self, a: f64, b: f64) -> Vector2 {
        Vector2::from(a * self.x_dir + b * self.y_dir)
    }
}

/// a not-implemented failure of the members an eval curve refuses (`Reverse`,
/// `ReversedParameter`, `Transform`); `unsupported("Geom2dEvalSineWaveCurve::Reverse", "")`.
pub(crate) fn unsupported(member: &str, detail: &str) -> Failure {
    errors::not_implemented(&format!(
        "{member}{detail}: eval curves keep their defining parameterization; expected no reversal or transformation"
    ))
}

/// a range-error failure of `EvalDN` below order 1; `check_derivative_order("X::EvalDN", 0)?`.
pub(crate) fn check_derivative_order(member: &str, n: i32) -> Result<(), Failure> {
    if n < 1 {
        return Err(errors::range(&format!(
            "{member}: N = {n}; expected N >= 1"
        )));
    }
    Ok(())
}

/// `EvalD0`..`EvalD3` of an analytic eval curve whose `terms(u)` value evaluates the
/// point and the first three derivatives in the curve's `LocalFrame`; the frame is
/// built first, then the terms, then the point before each derivative, as the defined
/// members run; `local_frame_evaluators!(SpiralTerms);` inside `impl Geom2dCurve`.
macro_rules! local_frame_evaluators {
    ($terms:ty) => {
        fn eval_d0(&self, u: f64) -> Result<Point2, Failure> {
            Ok(self.terms(u).point(&LocalFrame::new(self.position)))
        }
        fn eval_d1(&self, u: f64) -> Result<Geom2dCurveD1, Failure> {
            let frame = LocalFrame::new(self.position);
            let terms: $terms = self.terms(u);
            let (point, d1) = (terms.point(&frame), terms.d1(&frame));
            Ok(Geom2dCurveD1 { point, d1 })
        }
        fn eval_d2(&self, u: f64) -> Result<Geom2dCurveD2, Failure> {
            let frame = LocalFrame::new(self.position);
            let terms: $terms = self.terms(u);
            let (point, d1, d2) = (terms.point(&frame), terms.d1(&frame), terms.d2(&frame));
            Ok(Geom2dCurveD2 { point, d1, d2 })
        }
        fn eval_d3(&self, u: f64) -> Result<Geom2dCurveD3, Failure> {
            let frame = LocalFrame::new(self.position);
            let terms: $terms = self.terms(u);
            let (point, d1, d2) = (terms.point(&frame), terms.d1(&frame), terms.d2(&frame));
            let d3: Vector2 = terms.d3(&frame);
            Ok(Geom2dCurveD3 { point, d1, d2, d3 })
        }
    };
}
pub(crate) use local_frame_evaluators;

/// The constructor and accessors of the sine-wave eval curves, whose fields are
/// `position`, `amplitude`, `omega` and `phase`: a construction-error failure unless
/// `amplitude > 0` and `omega > 0`, checked in that order; `errors`, `CLASS` and
/// `a transient failure` must be in scope;
/// `martensite_tkg2d::sine_wave_members!(Axis2);`.
#[macro_export]
macro_rules! sine_wave_members {
    ($axis:ty) => {
        $crate::sine_wave_members!($axis, |name: &str, value: f64| errors::construction(
            &format!("{CLASS}: {name} = {value}; expected > 0")
        ));
    };
    ($axis:ty, $failure:expr) => {
        /// Sine wave (`Geom2dEvalSineWaveCurve` or `GeomEvalSineWaveCurve(Position,
        /// Amplitude, Omega, Phase = 0)`); a construction-error failure unless
        /// `amplitude > 0` and `omega > 0`; `Self::new(axis, 2.0, 3.0, 0.0)?`.
        pub fn new(
            position: $axis,
            amplitude: f64,
            omega: f64,
            phase: f64,
        ) -> Result<Self, Failure> {
            if amplitude <= 0.0 {
                return Err(($failure)("amplitude", amplitude));
            }
            if omega <= 0.0 {
                return Err(($failure)("omega", omega));
            }
            Ok(Self {
                parent: StandardTransient::default(),
                position,
                amplitude,
                omega,
                phase,
            })
        }
        // The defined Copy() reruns the constructor on valid stored values.
        fn copied(&self) -> Self {
            Self {
                parent: StandardTransient::default(),
                ..*self
            }
        }
        /// The local coordinate system (`Position`); `curve.position()`.
        pub fn position(&self) -> $axis {
            self.position
        }
        /// The amplitude (`Amplitude`); `curve.amplitude()`.
        pub fn amplitude(&self) -> f64 {
            self.amplitude
        }
        /// The angular frequency (`Omega`); `curve.omega()`.
        pub fn omega(&self) -> f64 {
            self.omega
        }
        /// The phase shift (`Phase`); `curve.phase()`.
        pub fn phase(&self) -> f64 {
            self.phase
        }
        // `omega * U + phase`, the sine argument.
        fn argument(&self, u: f64) -> f64 {
            self.omega * u + self.phase
        }
    };
}
