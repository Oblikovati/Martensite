//! Integer capture and output stages share the real numerical pipeline.
use martensite_tkernel::standard::Failure;
pub(super) trait TruncationIntegerAccess {
    fn stride(&self) -> Result<i32, Failure>;
    fn dimension(&self) -> Result<i32, Failure>;
    fn count(&self) -> Result<i32, Failure>;
    fn order(&self) -> Result<i32, Failure>;
    fn retained(&self) -> Result<i32, Failure>;
    fn set_retained(&mut self, value: i32) -> Result<(), Failure>;
}
pub(super) struct BorrowedTruncationIntegers<'a> {
    inputs: [i32; 4],
    retained: &'a mut i32,
}
impl<'a> BorrowedTruncationIntegers<'a> {
    pub(super) fn new(
        stride: i32,
        dimension: i32,
        count: i32,
        order: i32,
        retained: &'a mut i32,
    ) -> Self {
        Self {
            inputs: [stride, dimension, count, order],
            retained,
        }
    }
}
impl TruncationIntegerAccess for BorrowedTruncationIntegers<'_> {
    fn stride(&self) -> Result<i32, Failure> {
        Ok(self.inputs[0])
    }
    fn dimension(&self) -> Result<i32, Failure> {
        Ok(self.inputs[1])
    }
    fn count(&self) -> Result<i32, Failure> {
        Ok(self.inputs[2])
    }
    fn order(&self) -> Result<i32, Failure> {
        Ok(self.inputs[3])
    }
    fn retained(&self) -> Result<i32, Failure> {
        Ok(*self.retained)
    }
    fn set_retained(&mut self, value: i32) -> Result<(), Failure> {
        *self.retained = value;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_integer_stage_preserves_fixed_inputs_and_output_reference() {
        let mut output = -7;
        let mut integers = BorrowedTruncationIntegers::new(8, 2, 8, -1, &mut output);
        assert_eq!(integers.stride().unwrap(), 8);
        assert_eq!(integers.dimension().unwrap(), 2);
        assert_eq!(integers.count().unwrap(), 8);
        assert_eq!(integers.order().unwrap(), -1);
        integers.set_retained(2).unwrap();
        assert_eq!(integers.retained().unwrap(), 2);
        assert_eq!(output, 2);
    }
}
