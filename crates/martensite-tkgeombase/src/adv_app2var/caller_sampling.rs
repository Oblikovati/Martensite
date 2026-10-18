//! Caller sampling controls retain identity separately from evaluator locals.

/// One caller sampling control; `AdvApp2VarCallerSamplingField::RootCount`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarCallerSamplingField {
    /// Number of sample roots (`nbroot`).
    RootCount,
    /// Jacobi expansion degree (`ndgjac`).
    JacobiDegree,
}
impl AdvApp2VarCallerSamplingField {
    pub(super) fn index(self) -> usize {
        match self {
            Self::RootCount => 0,
            Self::JacobiDegree => 1,
        }
    }
}
impl super::AdvApp2VarIsoCallerIntegers<'_> {
    /// Update an actual caller sampling control; unavailable contexts return a
    /// typed domain failure. `callers.set_sampling(AdvApp2VarCallerSamplingField::RootCount, 8)?`.
    pub fn set_sampling(
        &mut self,
        field: AdvApp2VarCallerSamplingField,
        value: i32,
    ) -> Result<(), martensite_tkernel::standard::Failure> {
        let controls = self
            .sampling
            .as_deref_mut()
            .ok_or_else(missing_caller_sampling_context)?;
        controls[field.index()] = value;
        Ok(())
    }
}
pub(super) fn missing_caller_sampling_context() -> martensite_tkernel::standard::Failure {
    martensite_tkernel::standard::Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: caller sampling=None; expected actual root count and Jacobi degree",
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sampling_fields_index_distinct_operation_controls() {
        assert_eq!(AdvApp2VarCallerSamplingField::RootCount.index(), 0);
        assert_eq!(AdvApp2VarCallerSamplingField::JacobiDegree.index(), 1);
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AdvApp2VarCallerSamplingField>();
    }
    #[test]
    fn caller_sampling_setter_requires_and_updates_actual_context() {
        let (mut count, mut orders, mut sampling) = (1, [0, 0], [8, 2]);
        let mut callers = super::super::AdvApp2VarIsoCallerIntegers::new(&mut count, &mut orders);
        assert!(
            callers
                .set_sampling(AdvApp2VarCallerSamplingField::RootCount, 10)
                .is_err()
        );
        let mut callers = callers.with_sampling(&mut sampling);
        callers
            .set_sampling(AdvApp2VarCallerSamplingField::RootCount, 10)
            .unwrap();
        assert_eq!(callers.sampling(), Some([10, 2]));
    }
    #[test]
    fn unavailable_sampling_reports_the_actual_field_domain() {
        let failure = missing_caller_sampling_context();
        assert_eq!(failure.exception_type(), "Standard_DomainError");
        assert!(
            failure
                .message()
                .contains("caller sampling=None; expected actual root count and Jacobi degree")
        );
    }
}
