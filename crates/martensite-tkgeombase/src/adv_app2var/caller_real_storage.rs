//! Operation-local caller storage shares one allocation only for an owned output.
#[path = "caller_endpoint_storage.rs"]
mod endpoints;
pub use endpoints::AdvApp2VarEndpointOutput;
pub(crate) use endpoints::OwnedEndpointSlots;
#[path = "caller_error_storage.rs"]
mod errors;
#[path = "caller_folded_storage.rs"]
mod folded;
pub use errors::AdvApp2VarErrorOutput;
pub(crate) use errors::OwnedErrorSlots;
#[path = "caller_table_storage.rs"]
mod indexed_tables;
use crate::adv_app2var::approx_f2var::curve_output::CurveCoefficientStorage;
pub use folded::AdvApp2VarFoldedOutput;
pub(crate) use folded::OwnedFoldedSlots;
pub(crate) use indexed_tables::{CallerRealTableRole, OwnedCallerTableSlots};
use martensite_tkernel::standard::Failure;
use std::ops::Range;
use std::sync::{Arc, Mutex};
pub(super) enum CallerRealSlots {
    Owned(Vec<f64>),
    Shared(Arc<Mutex<Vec<f64>>>),
}
/// An operation-local mutable curve region, sharing the caller-real owner.
/// Obtain it with `frame.caller_curve_slots(5,64)?`; pass it as the curve storage
/// of `IsoApproxCallerOutput`. This is mutable operation state, not a published result.
pub struct AdvApp2VarCallerCurveSlots {
    values: Arc<Mutex<Vec<f64>>>,
    region: Range<usize>,
}
impl CallerRealSlots {
    pub(super) fn new(values: Vec<f64>) -> Self {
        Self::Owned(values)
    }
    pub(super) fn len(&self) -> usize {
        self.with_values(|values| values.len())
    }
    pub(super) fn value(&self, index: usize) -> f64 {
        self.with_values(|values| values[index])
    }
    pub(super) fn set_value(&mut self, index: usize, value: f64) {
        self.with_values_mut(|values| values[index] = value);
    }
    pub(super) fn selected_values(&self, slots: &[usize]) -> Vec<f64> {
        self.with_values(|values| slots.iter().map(|slot| values[*slot]).collect())
    }
    pub(super) fn with_values<ResultValue>(
        &self,
        operation: impl FnOnce(&[f64]) -> ResultValue,
    ) -> ResultValue {
        match self {
            Self::Owned(values)=>operation(values),
            Self::Shared(values)=>operation(&values.lock().expect("AdvApp2VarCallerCurveSlots: poisoned=true; expected unpoisoned operation-local values")),
        }
    }
    pub(super) fn with_values_mut<ResultValue>(
        &mut self,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue {
        match self {
            Self::Owned(values)=>operation(values),
            Self::Shared(values)=>operation(&mut values.lock().expect("AdvApp2VarCallerCurveSlots: poisoned=true; expected unpoisoned operation-local values")),
        }
    }
    pub(super) fn shared_values(&mut self) -> Arc<Mutex<Vec<f64>>> {
        if let Self::Shared(values) = self {
            return Arc::clone(values);
        }
        let Self::Owned(values) = std::mem::replace(self, Self::Owned(Vec::new())) else {
            unreachable!()
        };
        let shared = Arc::new(Mutex::new(values));
        *self = Self::Shared(Arc::clone(&shared));
        shared
    }
    pub(super) fn curve_region(&mut self, region: Range<usize>) -> AdvApp2VarCallerCurveSlots {
        let values = self.shared_values();
        AdvApp2VarCallerCurveSlots { values, region }
    }
    pub(super) fn owns_region(
        &self,
        curve: &AdvApp2VarCallerCurveSlots,
        region: &Range<usize>,
    ) -> bool {
        matches!(self,Self::Shared(values) if Arc::ptr_eq(values,&curve.values) && *region==curve.region)
    }
}
impl CurveCoefficientStorage for AdvApp2VarCallerCurveSlots {
    fn with_curve_values_at<ResultValue>(
        &mut self,
        origin: i32,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue {
        let mut values = self.values.lock().expect("AdvApp2VarCallerCurveSlots: poisoned=true; expected unpoisoned operation-local curve values");
        let start = super::checked_real_range(self.region.start, origin, 0, self.region.end)
            .expect("AdvApp2VarCallerCurveSlots: invalid scoped origin; expected initialized physical curve prefix or declared suffix").start;
        operation(&mut values[start..self.region.end])
    }
    fn with_curve_values<ResultValue>(
        &mut self,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue {
        // Only numerical kernels execute in this scope; the guard ends before evaluator dispatch.
        let mut values=self.values.lock().expect("AdvApp2VarCallerCurveSlots: poisoned=true; expected unpoisoned operation-local curve values");
        operation(&mut values[self.region.clone()])
    }
}
pub(super) fn invalid_curve_region(start: usize, length: usize, capacity: usize) -> Failure {
    Failure::with_message(
        "Standard_OutOfRange",
        &format!(
            "AdvApp2VarCallerCurveSlots: start={start}, len={length}; expected nonempty region inside 0..{capacity}"
        ),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar_storage_preserves_bits_and_indexed_write_identity() {
        let mut slots = CallerRealSlots::new(vec![-0., 1.]);
        assert_eq!(slots.len(), 2);
        assert_eq!(slots.value(0).to_bits(), (-0_f64).to_bits());
        slots.set_value(0, 0.5);
        assert_eq!(slots.selected_values(&[1, 0, 1]), [1., 0.5, 1.]);
    }
    #[test]
    fn output_scope_and_caller_access_share_one_allocation_without_escaping_guards() {
        let mut slots = CallerRealSlots::new(vec![7.; 6]);
        let pointer = slots.with_values(|values| values.as_ptr());
        let mut curve = slots.curve_region(1..5);
        assert!(slots.owns_region(&curve, &(1..5)));
        assert_eq!(slots.with_values(|values| values.as_ptr()), pointer);
        curve.with_curve_values(|values| values[0] = 0.25);
        assert_eq!(slots.value(1), 0.25);
        slots.set_value(1, 0.75);
        assert_eq!(curve.with_curve_values(|values| values[0]), 0.75);
        assert_eq!(slots.selected_values(&[0, 5]), [7., 7.]);
        let other = slots.curve_region(2..5);
        assert!(!slots.owns_region(&other, &(1..5)));
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AdvApp2VarCallerCurveSlots>();
    }
    fn measure_access(slots: &mut CallerRealSlots, iterations: usize) -> u128 {
        let started = std::time::Instant::now();
        for index in 0..iterations {
            slots.set_value(0, std::hint::black_box(index as f64));
            std::hint::black_box(slots.value(0));
        }
        started.elapsed().as_nanos()
    }
    #[test]
    fn operation_local_storage_access_costs_are_recorded() {
        let iterations = 10000;
        let mut direct = CallerRealSlots::new(vec![0.; 69]);
        let mut shared = CallerRealSlots::new(vec![0.; 69]);
        let _curve = shared.curve_region(5..69);
        let direct_ns = measure_access(&mut direct, iterations);
        let shared_ns = measure_access(&mut shared, iterations);
        assert_eq!(direct.value(0), 9999.);
        assert_eq!(shared.value(0), 9999.);
        println!(
            "{{\"metric\":\"caller_real_storage_access\",\"iterations\":{iterations},\"direct_ns\":{direct_ns},\"shared_ns\":{shared_ns},\"shared_lock_calls_from_loop_structure\":{},\"slot_payload_bytes\":{},\"mutex_vec_size\":{},\"curve_handle_size\":{},\"storage_enum_size\":{},\"plain_vec_size\":{}}}",
            iterations * 2,
            69 * std::mem::size_of::<f64>(),
            std::mem::size_of::<Mutex<Vec<f64>>>(),
            std::mem::size_of::<AdvApp2VarCallerCurveSlots>(),
            std::mem::size_of::<CallerRealSlots>(),
            std::mem::size_of::<Vec<f64>>()
        );
    }
    #[test]
    fn signed_curve_scopes_reuse_prefix_without_copy_and_record_lock_costs() {
        let mut slots = CallerRealSlots::new(vec![7.; 8]);
        let mut curve = slots.curve_region(3..7);
        let iterations = 10000;
        let started = std::time::Instant::now();
        for index in 0..iterations {
            curve.with_curve_values(|values| values[0] = std::hint::black_box(index as f64));
        }
        let direct_ns = started.elapsed().as_nanos();
        let started = std::time::Instant::now();
        for index in 0..iterations {
            curve.with_curve_values_at(-1, |values| values[0] = std::hint::black_box(index as f64));
        }
        let signed_ns = started.elapsed().as_nanos();
        assert_eq!(curve.with_curve_values_at(-3, |values| values.len()), 7);
        assert_eq!(curve.with_curve_values_at(4, |values| values.len()), 0);
        assert_eq!(
            slots.selected_values(&[0, 1, 2, 3, 4, 7]),
            [7., 7., 9999., 9999., 7., 7.]
        );
        println!(
            "{{\"metric\":\"signed_curve_scope\",\"iterations\":{iterations},\"origin_zero_ns\":{direct_ns},\"signed_ns\":{signed_ns},\"lock_calls_per_loop\":1,\"added_allocations\":0,\"payload_copies\":0}}"
        );
    }
}
