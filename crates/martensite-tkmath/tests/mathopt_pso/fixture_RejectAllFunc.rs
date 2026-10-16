#[derive(Default)]
struct RejectAllFunc;
impl MultipleVarFunction for RejectAllFunc {
    fn variables(&self) -> i32 {
        panic!("the callback has no variable count")
    }
    fn value(&mut self, theX: &Vector<'_>, theF: &mut f64) -> bool {
        false
    }
}
