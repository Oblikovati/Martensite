/// Sequential reader over one request's numeric fields.
pub struct Reader<'v> {
    values: &'v [f64],
    next: usize,
}
impl<'v> Reader<'v> {
    pub fn new(values: &'v [f64]) -> Self {
        Self { values, next: 0 }
    }
    pub fn real(&mut self) -> f64 {
        self.next += 1;
        self.values[self.next - 1]
    }
    pub fn integer(&mut self) -> i32 {
        self.real() as i32
    }
    pub fn flag(&mut self) -> bool {
        self.integer() != 0
    }
    pub fn reals(&mut self) -> Vec<f64> {
        let n = self.integer();
        (0..n).map(|_| self.real()).collect()
    }
}
