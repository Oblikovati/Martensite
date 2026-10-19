#[derive(Default)]
struct AckleyFunc;
impl MultipleVarFunction for AckleyFunc {
    fn variables(&self) -> i32 {
        panic!("the callback has no variable count")
    }
    fn value(&mut self, theX: &Vector<'_>, theF: &mut f64) -> bool {
        let aN: usize = theX.length() as usize;
        let aPI: f64 = std::f64::consts::PI;
        let mut aSum1: f64 = 0.0;
        let mut aSum2: f64 = 0.0;

        for i in 0..theX.length() as usize {
            aSum1 += *theX.at(i).unwrap() * *theX.at(i).unwrap();
            aSum2 += martensite_cmath::cos(2.0 * aPI * *theX.at(i).unwrap());
        }

        *theF = -20.0 * martensite_cmath::exp(-0.2 * (aSum1 / aN as f64).sqrt())
            - martensite_cmath::exp(aSum2 / aN as f64)
            + 20.0
            + std::f64::consts::E;
        true
    }
}
