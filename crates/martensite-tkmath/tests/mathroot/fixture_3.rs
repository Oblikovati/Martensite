#[derive(Default)]
struct SinFunc;
impl Function for SinFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        *theF = martensite_cmath::sin(theX);
        true
    }
}
impl FunctionWithDerivative for SinFunc {
    fn derivative(&mut self, theX: f64, theDf: &mut f64) -> bool {
        let mut value: f64 = 0.0;
        self.values(theX, &mut value, theDf)
    }
    fn values(&mut self, theX: f64, theF: &mut f64, theDf: &mut f64) -> bool {
        *theF = martensite_cmath::sin(theX);
        *theDf = martensite_cmath::cos(theX);
        true
    }
}
