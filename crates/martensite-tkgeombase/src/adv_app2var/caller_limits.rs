//! Caller allocation limits retain identity separately from evaluator locals.

/// One caller limit; `AdvApp2VarCallerLimitField::MaximumPieces`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarCallerLimitField {
    /// Maximum curve pieces (`nbcrmx`).
    MaximumPieces,
    /// Maximum coefficients per component (`ncflim`).
    CoefficientLimit,
}
impl AdvApp2VarCallerLimitField {
    pub(super) fn index(self) -> usize {
        match self {
            Self::MaximumPieces => 0,
            Self::CoefficientLimit => 1,
        }
    }
}
pub(super) fn missing_caller_limit_context() -> martensite_tkernel::standard::Failure {
    martensite_tkernel::standard::Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: caller limits=None; expected actual piece and coefficient limits",
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn limit_fields_index_distinct_operation_controls() {
        assert_eq!(AdvApp2VarCallerLimitField::MaximumPieces.index(), 0);
        assert_eq!(AdvApp2VarCallerLimitField::CoefficientLimit.index(), 1);
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AdvApp2VarCallerLimitField>();
    }
    #[test]
    fn unavailable_limits_report_the_actual_field_domain() {
        let failure = missing_caller_limit_context();
        assert_eq!(failure.exception_type(), "Standard_DomainError");
        assert!(
            failure
                .message()
                .contains("caller limits=None; expected actual piece and coefficient limits")
        );
    }
}
