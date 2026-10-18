//! Owned caller table origins select sequential regions of the sample allocation.
use super::AdvApp2VarEvaluationBuffers;
use super::approx_f2var::{SquareTableRole, SquareTableStorage};
use martensite_tkernel::standard::Failure;
/// An original caller square symmetry table; `AdvApp2VarSquareTable::SumSum`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarSquareTable {
    /// Sum in both parameters.
    SumSum,
    /// Difference in U and sum in V.
    DifferenceSum,
    /// Sum in U and difference in V.
    SumDifference,
    /// Difference in both parameters.
    DifferenceDifference,
}
impl AdvApp2VarSquareTable {
    pub(super) fn index(self) -> usize {
        match self {
            Self::SumSum => 0,
            Self::DifferenceSum => 1,
            Self::SumDifference => 2,
            Self::DifferenceDifference => 3,
        }
    }
}
#[derive(Clone, Copy, Default)]
pub(super) struct SquareTableBindings {
    origins: Option<[usize; 4]>,
}
impl SquareTableBindings {
    pub(super) fn new(origins: [usize; 4]) -> Self {
        Self {
            origins: Some(origins),
        }
    }
    pub(super) fn is_bound(self) -> bool {
        self.origins.is_some()
    }
    pub(super) fn start(self, role: AdvApp2VarSquareTable) -> Result<usize, Failure> {
        self.origins.map(|origins|origins[role.index()]).ok_or_else(||Failure::with_message("Standard_DomainError","AdvApp2VarSquareSampleFrame: owned tables configured=false; expected four owned table origins"))
    }
    fn shifted_origin(
        self,
        role: AdvApp2VarSquareTable,
        offset: isize,
        capacity: usize,
    ) -> Result<usize, Failure> {
        let origin = self.start(role)?;
        origin.checked_add_signed(offset).filter(|slot| *slot <= capacity).ok_or_else(|| Failure::with_message(
            "Standard_OutOfRange", &format!("AdvApp2VarSquareSampleFrame: {role:?} offset={offset} from origin={origin}; expected physical origin in 0..={capacity}"),
        ))
    }
    pub(super) fn signed_slot(
        self,
        role: AdvApp2VarSquareTable,
        offset: isize,
        capacity: usize,
    ) -> Result<usize, Failure> {
        let slot = self.shifted_origin(role, offset, capacity)?;
        if slot < capacity {
            return Ok(slot);
        }
        Err(Failure::with_message(
            "Standard_OutOfRange",
            &format!(
                "AdvApp2VarSquareSampleFrame: {role:?} offset={offset} gives physical slot={slot}; expected initialized element in 0..{capacity}"
            ),
        ))
    }
    pub(super) fn slot(
        self,
        role: AdvApp2VarSquareTable,
        index: usize,
        capacity: usize,
    ) -> Result<usize, Failure> {
        let start = self.start(role)?;
        start.checked_add(index).filter(|slot|*slot<capacity).ok_or_else(||Failure::with_message("Standard_OutOfRange",&format!("AdvApp2VarSquareSampleFrame: {role:?} index={index}; expected element in 0..{}",capacity.saturating_sub(start))))
    }
}
impl SquareTableStorage for SquareTableBindings {
    fn table_at<'a>(
        &'a self,
        role: SquareTableRole,
        offset: isize,
        buffers: &'a AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a [f64] {
        let capacity = buffers.allocation_capacity().expect("AdvApp2VarSquareSampleFrame: table storage is separate; expected owned initialized allocation");
        let start = self.shifted_origin(public_role(role),offset,capacity).expect(
            "AdvApp2VarSquareSampleFrame: table offset outside original owner; expected validated physical origin",
        );
        buffers.allocation_suffix(start).expect("AdvApp2VarSquareSampleFrame: owned table outside allocation; expected initialized suffix")
    }
    fn table_at_mut<'a>(
        &'a mut self,
        role: SquareTableRole,
        offset: isize,
        buffers: &'a mut AdvApp2VarEvaluationBuffers<'_>,
    ) -> &'a mut [f64] {
        let capacity = buffers.allocation_capacity().expect("AdvApp2VarSquareSampleFrame: table storage is separate; expected owned initialized allocation");
        let start = self.shifted_origin(public_role(role),offset,capacity).expect(
            "AdvApp2VarSquareSampleFrame: table offset outside original owner; expected validated physical origin",
        );
        buffers.allocation_suffix_mut(start).expect("AdvApp2VarSquareSampleFrame: owned table outside allocation; expected initialized suffix")
    }
}
fn public_role(role: SquareTableRole) -> AdvApp2VarSquareTable {
    match role {
        SquareTableRole::Soso => AdvApp2VarSquareTable::SumSum,
        SquareTableRole::Diso => AdvApp2VarSquareTable::DifferenceSum,
        SquareTableRole::Sodi => AdvApp2VarSquareTable::SumDifference,
        SquareTableRole::Didi => AdvApp2VarSquareTable::DifferenceDifference,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn owned_table_scopes_keep_prefix_identity_and_payload() {
        let mut values = [0., 1., 2., 3., 4., 5.];
        let mut buffers = AdvApp2VarEvaluationBuffers::shared(&mut values, 0, 0, 0, 6);
        let mut tables = SquareTableBindings::new([3; 4]);
        assert_eq!(
            tables.table_at(SquareTableRole::Soso, -2, &buffers),
            &[1., 2., 3., 4., 5.]
        );
        tables.table_at_mut(SquareTableRole::Didi, -2, &mut buffers)[0] = 9.;
        assert_eq!(tables.table_at(SquareTableRole::Soso, -2, &buffers)[0], 9.);
        assert_eq!(tables.table(SquareTableRole::Soso, &buffers)[0], 3.);
        assert!(
            tables
                .table_at(SquareTableRole::Soso, 3, &buffers)
                .is_empty()
        );
        assert_eq!(buffers.allocation_capacity().unwrap(), 6);
        assert_eq!(
            tables
                .table_at(SquareTableRole::Soso, -2, &buffers)
                .as_ptr(),
            buffers.allocation_suffix(1).unwrap().as_ptr()
        );
        println!(
            "{{\"metric\":\"signed_square_table_scope\",\"descriptor_bytes\":{},\"payload_copies\":0,\"new_scope_allocations\":0,\"scope_locks\":0}}",
            std::mem::size_of::<SquareTableBindings>()
        );
    }
    #[test]
    fn table_origins_check_physical_prefix_and_one_past_bounds() {
        let tables = SquareTableBindings::new([3; 4]);
        let role = AdvApp2VarSquareTable::SumSum;
        assert_eq!(tables.shifted_origin(role, -3, 6).unwrap(), 0);
        assert_eq!(tables.shifted_origin(role, 3, 6).unwrap(), 6);
        for offset in [-4, 4, isize::MIN, isize::MAX] {
            assert!(tables.shifted_origin(role, offset, 6).is_err());
        }
        assert!(tables.slot(role, usize::MAX, 6).is_err());
        assert!(
            SquareTableBindings::default()
                .shifted_origin(role, 0, 6)
                .is_err()
        );
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SquareTableBindings>();
    }
}
