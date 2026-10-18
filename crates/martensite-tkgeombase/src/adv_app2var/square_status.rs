//! Status can be an initialized caller slot without aliasing mutable references.
use super::*;
impl AdvApp2VarSquareOperationFrame {
    /// Bind the caller error code to an owned integer slot;
    /// `let frame=frame.with_owned_status_slot(0)?` permits dimension/status identity.
    pub fn with_owned_status_slot(mut self, slot: usize) -> Result<Self, Failure> {
        self.incidence = scalar_incidence(
            FIELDS
                .map(|field| (FrameSquareField::Caller(field), self.bindings.slot(field)))
                .into_iter()
                .chain([(FrameSquareField::Status, slot)]),
            self.values.len(),
            "AdvApp2VarSquareOperationFrame",
        )?;
        Ok(self)
    }
    /// Read the owned caller status after success or error; `let code=frame.status()?`.
    pub fn status(&self) -> Result<i32, Failure> {
        Ok(self.values[self.status_slot()?])
    }
    pub(in crate::adv_app2var) fn set_status(&mut self, value: i32) -> Result<(), Failure> {
        let slot = self.status_slot()?;
        self.values[slot] = value;
        Ok(())
    }
    pub(in crate::adv_app2var) fn require_owned_status(&self) -> Result<(), Failure> {
        self.status_slot().map(|_| ())
    }
    pub(in crate::adv_app2var) fn require_separate_status(&self) -> Result<(), Failure> {
        if self.status_slot().is_err() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            "AdvApp2VarSquareOperationFrame: owned status configured=true; expected owned-status entrypoint instead of separate mutable error code",
        ))
    }
    fn status_slot(&self) -> Result<usize, Failure> {
        self.incidence.iter().find(|group|group.fields.iter().any(|field|matches!(field,FrameSquareField::Status)))
            .map(|group|group.slot)
            .ok_or_else(||Failure::with_message("Standard_DomainError","AdvApp2VarSquareOperationFrame: owned status configured=false; expected initialized owned status slot"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_binding_keeps_integer_allocation_and_records_metadata() {
        let owner = AdvApp2VarSquareOperationFrame::new(
            vec![3, 3, 3, 2],
            AdvApp2VarSquareIntegerBindings {
                dimension: 0,
                root_counts: [1, 2],
                favorite_iso: 3,
            },
        )
        .unwrap();
        let pointer = owner.values.as_ptr();
        let owner = owner.with_owned_status_slot(0).unwrap();
        assert_eq!(owner.values.as_ptr(), pointer);
        let fields: usize = owner.incidence.iter().map(|group| group.fields.len()).sum();
        assert_eq!((owner.incidence.len(), fields), (4, 5));
        println!(
            "{{\"metric\":\"owned_square_status_metadata\",\"frame_bytes\":{},\"payload_bytes\":{},\"groups\":4,\"field_identities\":5,\"group_capacity_bytes\":{},\"field_capacity_bytes\":{}}}",
            std::mem::size_of_val(&owner),
            owner.values.capacity() * std::mem::size_of::<i32>(),
            owner.incidence.capacity() * std::mem::size_of::<ScalarIncidence<FrameSquareField>>(),
            owner
                .incidence
                .iter()
                .map(|group| group.fields.capacity() * std::mem::size_of::<FrameSquareField>())
                .sum::<usize>()
        );
    }
}
