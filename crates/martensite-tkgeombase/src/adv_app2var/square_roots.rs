//! Logical root origins are independent of their actual initialized allocation slots.
use martensite_tkernel::standard::Failure;
/// An original caller root axis; `AdvApp2VarSquareRootAxis::U`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarSquareRootAxis {
    /// Original U roots, even when sampling a transposed square.
    U,
    /// Original V roots.
    V,
}
use AdvApp2VarSquareRootAxis as Axis;
#[derive(Clone, Copy, Default)]
pub(crate) struct SquareRootBindings {
    origins: [Option<usize>; 2],
}
impl SquareRootBindings {
    pub(crate) fn bind(&mut self, axis: Axis, start: usize) {
        self.origins[axis_index(axis)] = Some(start);
    }
    pub(crate) fn origin(&self, axis: Axis) -> Option<usize> {
        self.origins[axis_index(axis)]
    }
    pub(crate) fn has_roots(&self) -> bool {
        self.origins.iter().any(Option::is_some)
    }
    pub(crate) fn read(
        &self,
        axis: Axis,
        index: isize,
        borrowed: &[f64],
        buffers: &super::AdvApp2VarEvaluationBuffers<'_>,
    ) -> f64 {
        if self.origin(axis).is_none() {
            return borrowed[usize::try_from(index).expect("AdvApp2VarSquareSampleFrame: negative borrowed root offset; expected an owned origin including initialized prefix")];
        }
        let capacity=buffers.allocation_capacity().expect("AdvApp2VarSquareSampleFrame: mapped root without shared allocation; expected initialized owned source");
        let slot=self.shifted_slot(axis,index,capacity).expect("AdvApp2VarSquareSampleFrame: root index outside initialized allocation; expected source access within owned slots");
        buffers.allocation_value(slot).expect(
            "AdvApp2VarSquareSampleFrame: root slot missing; expected initialized owned scalar",
        )
    }
    fn shifted_slot(self, axis: Axis, index: isize, capacity: usize) -> Result<usize, Failure> {
        let start=self.origin(axis).ok_or_else(||Failure::with_message("Standard_DomainError",&format!("AdvApp2VarSquareSampleFrame: {axis:?} roots configured=false; expected owned caller root origin")))?;
        start.checked_add_signed(index).filter(|slot|*slot<capacity)
            .ok_or_else(||Failure::with_message("Standard_OutOfRange",&format!("AdvApp2VarSquareSampleFrame: {axis:?} root offset={index}; expected initialized physical slot in0..{capacity} from origin={start}")))
    }
    pub(crate) fn slot(self, axis: Axis, index: usize, capacity: usize) -> Result<usize, Failure> {
        let start=self.origin(axis).ok_or_else(||Failure::with_message("Standard_DomainError",&format!("AdvApp2VarSquareSampleFrame: {axis:?} roots configured=false; expected owned caller root origin")))?;
        start.checked_add(index).filter(|slot|*slot<capacity)
            .ok_or_else(||Failure::with_message("Standard_OutOfRange",&format!("AdvApp2VarSquareSampleFrame: {axis:?} root index={index}; expected element in 0..{}",capacity.saturating_sub(start))))
    }
}
fn axis_index(axis: Axis) -> usize {
    match axis {
        Axis::U => 0,
        Axis::V => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_root_offsets_cover_initialized_prefix_and_reject_physical_overflow() {
        let mut roots = SquareRootBindings::default();
        assert!(roots.shifted_slot(Axis::U, -1, 4).is_err());
        roots.bind(Axis::U, 2);
        assert_eq!(roots.shifted_slot(Axis::U, -2, 4).unwrap(), 0);
        assert_eq!(roots.shifted_slot(Axis::U, 0, 4).unwrap(), 2);
        assert_eq!(roots.shifted_slot(Axis::U, 1, 4).unwrap(), 3);
        for offset in [-3, 2, isize::MIN, isize::MAX] {
            assert!(roots.shifted_slot(Axis::U, offset, 4).is_err());
        }
        assert_eq!(roots.slot(Axis::U, 1, 4).unwrap(), 3);
        assert!(roots.slot(Axis::U, usize::MAX, 4).is_err());
        let mut values = [0.25, 0.5, 0.75, 1.];
        let buffers = super::super::AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 4, 0, 4);
        assert_eq!(roots.read(Axis::U, -2, &[], &buffers), 0.25);
        assert_eq!(roots.read(Axis::V, 0, &[0.125], &buffers), 0.125);
    }
}
