#[derive(Default)]
struct CoshLikeFunc;
impl Function for CoshLikeFunc {
    fn value(&mut self, theX: f64, theF: &mut f64) -> bool {
        *theF = martensite_cmath::exp(theX) + martensite_cmath::exp(-theX);
        true
    }
}
