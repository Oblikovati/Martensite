//! Endpoint roles use the same initialized real owner as caller inputs.
use super::indexed_tables::{CallerRealTableRole, OwnedCallerTableSlots};
/// A per-subspace endpoint publication; `AdvApp2VarEndpointOutput::Lower`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarEndpointOutput {
    /// Upper truncation-endpoint estimate for a subspace.
    Lower,
    /// Root mean square truncation endpoint for a subspace.
    Upper,
}
impl AdvApp2VarEndpointOutput {
    pub(in crate::adv_app2var) fn index(self) -> usize {
        match self {
            Self::Lower => 0,
            Self::Upper => 1,
        }
    }
}
impl CallerRealTableRole for AdvApp2VarEndpointOutput {
    const LABEL: &'static str = "AdvApp2VarEndpointOutput";
    const POISON_MESSAGE: &'static str =
        "AdvApp2VarEndpointOutput: poisoned=true; expected initialized operation-local real owner";
    fn index(self) -> usize {
        self.index()
    }
}
pub(crate) type OwnedEndpointSlots = OwnedCallerTableSlots<AdvApp2VarEndpointOutput, 2>;
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn endpoint_roles_reuse_one_owner_and_are_send_sync() {
        fn assert_send_sync<Value: Send + Sync>() {}
        assert_send_sync::<AdvApp2VarEndpointOutput>();
        assert_send_sync::<OwnedEndpointSlots>();
        let values = Arc::new(Mutex::new(vec![7.; 8]));
        let pointer = values.lock().unwrap().as_ptr();
        let mut slots = OwnedEndpointSlots::new(Arc::clone(&values), [2, 4], 0);
        slots
            .set_value(AdvApp2VarEndpointOutput::Lower, 0, 3.)
            .unwrap();
        slots
            .set_value(AdvApp2VarEndpointOutput::Upper, 0, 5.)
            .unwrap();
        assert_eq!(values.lock().unwrap()[2], 3.);
        assert_eq!(values.lock().unwrap()[4], 5.);
        assert_eq!(Arc::strong_count(&values), 2);
        assert_eq!(values.lock().unwrap().as_ptr(), pointer);
        assert_eq!(std::mem::size_of::<OwnedEndpointSlots>(), 32);
        assert_eq!(std::mem::size_of::<Option<[usize; 2]>>(), 24);
    }
}
