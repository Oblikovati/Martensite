//! The public members of `Geom2dBezierCurve` and `GeomBezierCurve` that only lock
//! the shared [`BezierCurveCore`](crate::geom2d::bezier_curve_core::BezierCurveCore)
//! and forward to it: the same list, names and bodies in both classes.

/// The shared public members of the Bezier curve classes; expanded inside the
/// curve's `impl` with its point type;
/// `martensite_tkg2d::bezier_curve_members!(Point3);`.
#[macro_export]
macro_rules! bezier_curve_members {
    ($point:ty) => {
        $crate::spline_pole_members!($point);
        /// Reuses protected defined `init`, bypassing constructor-only validation;
        /// `curve.init(&poles, None)?` resets the evaluation representation on success.
        pub fn init(
            &self,
            poles: &Array1<'_, $point>,
            weights: Option<&Array1<'_, f64>>,
        ) -> Result<(), Failure> {
            self.state.write().init(poles, weights)
        }
        /// Raise the degree (`Increase`); `curve.increase(5)?`.
        pub fn increase(&self, degree: i32) -> Result<(), Failure> {
            self.state.write().increase(degree)
        }
        /// Remove the pole at `index` (`RemovePole`); `curve.remove_pole(2)?`.
        pub fn remove_pole(&self, index: i32) -> Result<(), Failure> {
            self.state.write().remove_pole(index)
        }
        /// Restrict to `[u1, u2]` and reparameterize on `[0, 1]` (`Segment`); `curve.segment(0.25, 0.75)?`.
        pub fn segment(&self, u1: f64, u2: f64) -> Result<(), Failure> {
            self.state.write().segment(u1, u2)
        }
        /// Number of poles minus one (`Degree`); `curve.degree()`.
        pub fn degree(&self) -> i32 {
            self.state.read().degree()
        }
        /// `bsplclib::MaxDegree()` (`MaxDegree`); `curve.max_degree() == 25` (an associated function).
        pub fn max_degree() -> i32 {
            bsplclib::max_degree()
        }
        /// Parametric tolerance for a 2D or 3D tolerance, caching the inverse derivative bound
        /// (`Resolution`); `curve.resolution(1e-7)?`.
        pub fn resolution(&self, tolerance: f64) -> Result<f64, Failure> {
            self.state.write().resolution(tolerance)
        }
        /// The knots `{0, 1}` (`Knots`); `curve.knots()`.
        pub fn knots(&self) -> Array1<'static, f64> {
            $crate::geom2d::spline_data::bezier_knots()
        }
        /// The multiplicities `{Degree+1, Degree+1}` (`Multiplicities`); `curve.multiplicities()?`.
        pub fn multiplicities(&self) -> Result<Array1<'static, i32>, Failure> {
            self.state.read().multiplicities()
        }
        /// The flat knots (`KnotSequence`); `curve.knot_sequence()?`.
        pub fn knot_sequence(&self) -> Result<Array1<'static, f64>, Failure> {
            self.state.read().knot_sequence()
        }
    };
}
