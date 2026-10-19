//! Endpoint reads and stores use one table interface before ownership is selected.
pub(crate) use crate::adv_app2var::AdvApp2VarEndpointOutput as ConstraintEnd;
use crate::adv_app2var::AdvApp2VarEvaluationBuffers;
use crate::adv_app2var::real_slots::OwnedEndpointSlots;
enum EndpointStorage<'a> {
    Borrowed([&'a mut [f64]; 2]),
    Owned(OwnedEndpointSlots),
}
pub(crate) struct EndpointPieceTables<'a> {
    storage: EndpointStorage<'a>,
}
impl<'a> EndpointPieceTables<'a> {
    pub(crate) fn new(lower: &'a mut [f64], upper: &'a mut [f64]) -> Self {
        Self {
            storage: EndpointStorage::Borrowed([lower, upper]),
        }
    }
    pub(crate) fn owned(slots: OwnedEndpointSlots) -> Self {
        Self {
            storage: EndpointStorage::Owned(slots),
        }
    }
    pub(crate) fn value(&self, end: ConstraintEnd, index: i32) -> f64 {
        match &self.storage {
            EndpointStorage::Borrowed(values) => values[end.index()][index as usize],
            EndpointStorage::Owned(slots) => slots.value(end, index).expect(
                "AdvApp2VarEndpointOutput: invalid read; expected initialized endpoint slot",
            ),
        }
    }
    pub(crate) fn set_value(&mut self, end: ConstraintEnd, index: i32, value: f64) {
        match &mut self.storage {
            EndpointStorage::Borrowed(values) => values[end.index()][index as usize] = value,
            EndpointStorage::Owned(slots) => slots.set_value(end, index, value).expect(
                "AdvApp2VarEndpointOutput: invalid write; expected initialized endpoint slot",
            ),
        }
    }
    pub(crate) fn with_values<ResultValue>(
        &mut self,
        end: ConstraintEnd,
        start: i32,
        operation: impl FnOnce(&mut [f64]) -> ResultValue,
    ) -> ResultValue {
        match &mut self.storage {
            EndpointStorage::Borrowed(values) => operation(&mut values[end.index()][start as usize..]),
            EndpointStorage::Owned(slots) => slots.with_values(end, |values| operation(&mut values[start as usize..])).expect("AdvApp2VarEndpointOutput: invalid pure scope; expected initialized endpoint suffix"),
        }
    }
    pub(crate) fn callback_buffers<'view>(
        &'view mut self,
        end: ConstraintEnd,
        start: i32,
        parameters: &'view mut [f64],
        legacy_parameters: usize,
    ) -> AdvApp2VarEvaluationBuffers<'view> {
        match &mut self.storage {
            EndpointStorage::Borrowed(values) => AdvApp2VarEvaluationBuffers::separate(
                parameters,
                &mut values[end.index()][start as usize..],
                legacy_parameters,
            ),
            EndpointStorage::Owned(slots) => {
                AdvApp2VarEvaluationBuffers::owned_endpoint(parameters, slots, end, start)
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_endpoint_access_preserves_roles_bits_and_pure_kernel_views() {
        let (mut lower, mut upper) = ([1., 2., 7.], [3., 4., 11.]);
        let mut tables = EndpointPieceTables::new(&mut lower, &mut upper);
        tables.set_value(ConstraintEnd::Lower, 1, -0.);
        assert_eq!(
            tables.value(ConstraintEnd::Lower, 1).to_bits(),
            (-0_f64).to_bits()
        );
        assert_eq!(tables.value(ConstraintEnd::Upper, 1), 4.);
        assert_eq!(
            tables.with_values(ConstraintEnd::Upper, 1, |tail| {
                tail[0] = 5.;
                tail.len()
            }),
            2
        );
        assert_eq!(
            lower.map(f64::to_bits),
            [1_f64.to_bits(), (-0_f64).to_bits(), 7_f64.to_bits()]
        );
        assert_eq!(upper, [3., 5., 11.]);
    }
    #[test]
    fn borrowed_endpoint_callback_results_are_the_actual_role_suffix() {
        let (mut lower, mut upper, mut parameters) = ([7.; 4], [11.; 4], [0.5]);
        let mut tables = EndpointPieceTables::new(&mut lower, &mut upper);
        let mut buffers = tables.callback_buffers(ConstraintEnd::Upper, 1, &mut parameters, 1);
        assert_eq!(buffers.parameter(0), 0.5);
        buffers.set_parameter(0, 0.75);
        assert_eq!(buffers.result(0), 11.);
        buffers.set_result(0, 3.);
        assert_eq!(upper, [11., 3., 11., 11.]);
        assert_eq!(lower, [7.; 4]);
        assert_eq!(parameters, [0.75]);
    }
    #[test]
    fn borrowed_endpoint_metadata_is_send_sync_and_keeps_borrowed_allocations() {
        fn assert_send_sync<Value: Send + Sync>() {}
        assert_send_sync::<ConstraintEnd>();
        assert_send_sync::<EndpointPieceTables<'static>>();
        let (mut lower, mut upper) = ([7.; 2], [11.; 2]);
        let pointers = [lower.as_ptr(), upper.as_ptr()];
        let tables = EndpointPieceTables::new(&mut lower, &mut upper);
        let EndpointStorage::Borrowed(values) = &tables.storage else {
            panic!("expected borrowed endpoint fixture");
        };
        assert_eq!(values[0].as_ptr(), pointers[0]);
        assert_eq!(values[1].as_ptr(), pointers[1]);
    }
}
