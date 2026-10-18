//! Caller derivative controls remain distinct from evaluator-local orders.

/// One caller order field, not an evaluator-local derivative argument;
/// `AdvApp2VarCallerOrderField::ContinuityOrder`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarCallerOrderField {
    /// End continuity (`iordre`).
    ContinuityOrder,
    /// Cross derivative being approximated (`ideriv`).
    CrossDerivativeOrder,
}
impl AdvApp2VarCallerOrderField {
    pub(super) fn index(self) -> usize {
        match self {
            Self::ContinuityOrder => 0,
            Self::CrossDerivativeOrder => 1,
        }
    }
}

/// Typed caller integer projection at an iso callback boundary. The two order
/// values are distinct from local U/V derivative arguments; owned frames retain
/// caller alias identity before copying back to this projection.
/// `AdvApp2VarIsoCallerIntegers::new(&mut spaces, &mut [continuity, derivative])`.
pub struct AdvApp2VarIsoCallerIntegers<'a> {
    pub(super) subspace_count: &'a mut i32,
    pub(super) orders: &'a mut [i32; 2],
    pub(super) limits: Option<&'a mut [i32; 2]>,
    pub(super) sampling: Option<&'a mut [i32; 2]>,
}
impl<'a> AdvApp2VarIsoCallerIntegers<'a> {
    /// Borrow operation-local caller fields; `AdvApp2VarIsoCallerIntegers::new(&mut spaces, &mut orders)`.
    pub fn new(subspace_count: &'a mut i32, orders: &'a mut [i32; 2]) -> Self {
        Self {
            subspace_count,
            orders,
            limits: None,
            sampling: None,
        }
    }
    pub(super) fn with_limits(mut self, limits: &'a mut [i32; 2]) -> Self {
        self.limits = Some(limits);
        self
    }
    pub(super) fn with_sampling(mut self, sampling: &'a mut [i32; 2]) -> Self {
        self.sampling = Some(sampling);
        self
    }
    /// Read root-count/Jacobi-degree controls supplied by an iso operation;
    /// `let controls=callers.sampling()` returns `[root_count, degree]`.
    pub fn sampling(&self) -> Option<[i32; 2]> {
        self.sampling.as_deref().copied()
    }
    /// Read caller piece/degree limits when an iso operation provides them;
    /// `let limits = callers.limits()` returns `[maximum_pieces, coefficient_limit]`.
    pub fn limits(&self) -> Option<[i32; 2]> {
        self.limits.as_deref().copied()
    }
    /// Read the actual caller count; `assert_eq!(callers.subspace_count(), 2)`.
    pub fn subspace_count(&self) -> i32 {
        *self.subspace_count
    }
    /// Update the caller count projection; `callers.set_subspace_count(1)`.
    pub fn set_subspace_count(&mut self, value: i32) {
        *self.subspace_count = value;
    }
    /// Read one caller order; `callers.order(AdvApp2VarCallerOrderField::ContinuityOrder)`.
    pub fn order(&self, field: AdvApp2VarCallerOrderField) -> i32 {
        self.orders[field.index()]
    }
    /// Write a caller order projection; `callers.set_order(AdvApp2VarCallerOrderField::ContinuityOrder, 1)`.
    pub fn set_order(&mut self, field: AdvApp2VarCallerOrderField, value: i32) {
        self.orders[field.index()] = value;
    }
}

pub(super) fn missing_caller_order_context() -> martensite_tkernel::standard::Failure {
    martensite_tkernel::standard::Failure::with_message(
        "Standard_DomainError",
        "AdvApp2VarIsoOperationFrame: caller orders=None; expected actual continuity and cross-derivative controls",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn typed_caller_projection_preserves_distinct_fields() {
        let (mut count, mut orders) = (2, [0, 1]);
        let mut callers = AdvApp2VarIsoCallerIntegers::new(&mut count, &mut orders);
        callers.set_order(AdvApp2VarCallerOrderField::ContinuityOrder, 2);
        assert_eq!(
            callers.order(AdvApp2VarCallerOrderField::ContinuityOrder),
            2
        );
        assert_eq!(
            callers.order(AdvApp2VarCallerOrderField::CrossDerivativeOrder),
            1
        );
        assert_eq!(callers.subspace_count(), 2);
        callers.set_subspace_count(1);
        assert_eq!(callers.subspace_count(), 1);
    }
    #[test]
    fn caller_limit_projection_uses_actual_operation_fields() {
        let (mut count, mut orders, mut limits) = (1, [-1, 0], [3, 3]);
        let callers = AdvApp2VarIsoCallerIntegers::new(&mut count, &mut orders);
        assert_eq!(callers.limits(), None);
        assert_eq!(callers.with_limits(&mut limits).limits(), Some([3, 3]));
    }
    #[test]
    fn sampling_projection_borrows_actual_operation_controls() {
        let (mut count, mut orders, mut sampling) = (1, [-1, 0], [3, 3]);
        let callers = AdvApp2VarIsoCallerIntegers::new(&mut count, &mut orders);
        assert_eq!(callers.sampling(), None);
        assert_eq!(
            callers.with_sampling(&mut sampling).sampling(),
            Some([3, 3])
        );
    }
    #[test]
    fn missing_order_context_reports_the_actual_unavailable_field_domain() {
        let failure = missing_caller_order_context();
        assert_eq!(failure.exception_type(), "Standard_DomainError");
        assert!(failure.message().contains(
            "caller orders=None; expected actual continuity and cross-derivative controls"
        ));
    }
    #[test]
    fn caller_projection_and_order_identity_are_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AdvApp2VarIsoCallerIntegers<'static>>();
        assert_send_sync::<AdvApp2VarCallerOrderField>();
    }
}
