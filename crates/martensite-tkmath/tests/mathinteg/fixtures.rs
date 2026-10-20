const THE_TOLERANCE: f64 = 1e-10;
fn near(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} != {b}");
}
#[derive(Default)]
struct ConstantFunc;
impl Function for ConstantFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = 5.0;
        true
    }
}
#[derive(Default)]
struct LinearFunc;
impl Function for LinearFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = 2.0 * theX + 1.0;
        true
    }
}
#[derive(Default)]
struct QuadraticFunc;
impl Function for QuadraticFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = theX * theX;
        true
    }
}
#[derive(Default)]
struct SinFunc;
impl Function for SinFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::sin(theX);
        true
    }
}
#[derive(Default)]
struct CosFunc;
impl Function for CosFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::cos(theX);
        true
    }
}
#[derive(Default)]
struct ExpFunc;
impl Function for ExpFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::exp(theX);
        true
    }
}
#[derive(Default)]
struct PolynomialFunc;
impl Function for PolynomialFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        let aX2: f64 = theX * theX;
        let aX3: f64 = aX2 * theX;
        *theF = aX2 * aX3 - 3.0 * aX3 + 2.0 * theX;
        true
    }
}
#[derive(Default)]
struct GaussianFunc;
impl Function for GaussianFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::exp(-theX * theX);
        true
    }
}
#[derive(Default)]
struct ReciprocalFunc;
impl Function for ReciprocalFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = 1.0 / theX;
        true
    }
}
#[derive(Default)]
struct OscillatoryFunc;
impl Function for OscillatoryFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::sin(10.0 * theX);
        true
    }
}
#[derive(Default)]
struct SqrtFunc;
impl Function for SqrtFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = (theX).sqrt();
        true
    }
}
#[derive(Default)]
struct FailingFunc;
impl Function for FailingFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        false
    }
}
#[derive(Default)]
struct NonFiniteFunc;
impl Function for NonFiniteFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = f64::NAN;
        true
    }
}
#[derive(Default)]
struct KinkFunc;
impl Function for KinkFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = (theX - 0.3141592653589793).abs();
        true
    }
}
#[derive(Default)]
struct DampedCosineFunc;
impl Function for DampedCosineFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::exp(-theX) * martensite_cmath::cos(2.0 * theX);
        true
    }
}
#[derive(Default)]
struct ExponentialSineFunc;
impl Function for ExponentialSineFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::exp(martensite_cmath::sin(theX));
        true
    }
}
#[derive(Default)]
struct ShiftedRationalFunc;
impl Function for ShiftedRationalFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = 1.0 / (theX * theX + theX + 1.0);
        true
    }
}
#[derive(Default)]
struct ShiftedGaussianFunc;
impl Function for ShiftedGaussianFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        let aShift: f64 = theX - 0.4;
        *theF = martensite_cmath::exp(-2.3 * aShift * aShift);
        true
    }
}
#[derive(Default)]
struct RationalOscillationFunc;
impl Function for RationalOscillationFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::cos(37.0 * theX) / (1.0 + theX * theX);
        true
    }
}
#[derive(Default)]
struct LogCosineFunc;
impl Function for LogCosineFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = martensite_cmath::log1p(theX * theX) * martensite_cmath::cos(0.5 * theX);
        true
    }
}
#[derive(Default)]
struct SquareRootLogFunc;
impl Function for SquareRootLogFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        let _ = (&theX, &theF);
        *theF = (theX).sqrt() * martensite_cmath::log1p(theX);
        true
    }
}
