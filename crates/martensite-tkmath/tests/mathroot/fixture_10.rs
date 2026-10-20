#[derive(Default)]
struct RationalCosFunc;
impl Function for RationalCosFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        *theF = theX / (theX + 2.0) - martensite_cmath::cos(theX);
        true
    }
}
impl FunctionWithDerivative for RationalCosFunc {
    fn derivative(&mut self, theX: f64, theDf: &mut f64) -> bool {
        let mut value: f64 = 0.0;
        self.values(theX, &mut value, theDf)
    }
    fn values(&mut self, theX: f64, theF: &mut f64, theDf: &mut f64) -> bool {
        let aDenominator: f64 = theX + 2.0;
        *theF = theX / aDenominator - martensite_cmath::cos(theX);
        *theDf = 2.0 / (aDenominator * aDenominator) + martensite_cmath::sin(theX);
        true
    }
}
