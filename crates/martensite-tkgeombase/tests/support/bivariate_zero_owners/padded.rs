//! Logical zero payloads retain independently checked original-owner guards.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Padded<T> {
    pub(super) values: Vec<T>,
    length: usize,
}
impl<T: Clone> Padded<T> {
    pub(super) fn new(length: usize, sentinel: T) -> Self {
        Self {
            values: vec![sentinel; 64 + length + 16],
            length,
        }
    }
    pub(super) fn payload(&self) -> &[T] {
        &self.values[64..64 + self.length]
    }
    pub(super) fn payload_mut(&mut self) -> &mut [T] {
        &mut self.values[64..64 + self.length]
    }
}
pub(crate) fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}
pub(super) fn expect_real_owner(owner: &Padded<f64>, payload: &[f64], sentinel: f64) {
    let mut expected = Padded::new(payload.len(), sentinel);
    expected.payload_mut().copy_from_slice(payload);
    assert_eq!(bits(&owner.values), bits(&expected.values));
}
