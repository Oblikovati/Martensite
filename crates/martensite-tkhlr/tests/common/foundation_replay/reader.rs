//! Sequential owned request fields for the retained numeric operation driver.
/// Request-local numeric input; no shared state between requests.
pub struct Reader<'v> {
    remaining: std::slice::Iter<'v, f64>,
}
impl<'v> Reader<'v> {
    pub fn new(values: &'v [f64]) -> Self {
        Self {
            remaining: values.iter(),
        }
    }
    pub fn real(&mut self) -> f64 {
        *self.remaining.next().expect("numeric record field")
    }
    pub fn integer(&mut self) -> i32 {
        self.real() as i32
    }
    pub fn flag(&mut self) -> bool {
        self.integer() != 0
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;

    #[test]
    fn independent_readers_keep_truncating_integer_and_flag_semantics() {
        let fields = [1.9, 0.5, -2.];
        let mut first = Reader::new(&fields);
        let mut second = Reader::new(&fields);
        assert_eq!(first.integer(), 1);
        assert!(!first.flag());
        assert_eq!(first.real(), -2.);
        assert_eq!(second.real(), 1.9);
    }
}
