#[derive(Default)]
struct ShiftedSinFunc;
impl Function for ShiftedSinFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        *theF = martensite_cmath::sin(theX) - 0.25;
        true
    }
}
impl FunctionWithDerivative for ShiftedSinFunc {
    fn derivative(&mut self, theX: f64, theDf: &mut f64) -> bool {
        let mut value: f64 = 0.0;
        self.values(theX, &mut value, theDf)
    }
    fn values(&mut self, theX: f64, theF: &mut f64, theDf: &mut f64) -> bool {
        *theF = martensite_cmath::sin(theX) - 0.25;
        *theDf = martensite_cmath::cos(theX);
        true
    }
}
