//! Error roles use the same initialized real owner as caller inputs.
use super::indexed_tables::{CallerRealTableRole, OwnedCallerTableSlots};
/// A per-subspace error publication; `AdvApp2VarErrorOutput::Maximum`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarErrorOutput {
    /// Upper truncation-error estimate for a subspace.
    Maximum,
    /// Root mean square truncation error for a subspace.
    RootMeanSquare,
}
impl AdvApp2VarErrorOutput {
    pub(in crate::adv_app2var) fn index(self) -> usize {
        match self {
            Self::Maximum => 0,
            Self::RootMeanSquare => 1,
        }
    }
}
impl CallerRealTableRole for AdvApp2VarErrorOutput {
    const LABEL: &'static str = "AdvApp2VarErrorOutput";
    const POISON_MESSAGE: &'static str =
        "AdvApp2VarErrorOutput: poisoned=true; expected initialized operation-local real owner";
    fn index(self) -> usize {
        self.index()
    }
}
pub(crate) type OwnedErrorSlots = OwnedCallerTableSlots<AdvApp2VarErrorOutput, 2>;
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[test]
    fn error_roles_reuse_one_owner_and_are_send_sync() {
        fn assert_send_sync<Value: Send + Sync>() {}
        assert_send_sync::<AdvApp2VarErrorOutput>();
        assert_send_sync::<OwnedErrorSlots>();
        let values = Arc::new(Mutex::new(vec![7.; 8]));
        let pointer = values.lock().unwrap().as_ptr();
        let mut slots = OwnedErrorSlots::new(Arc::clone(&values), [2, 4], 0);
        slots
            .set_value(AdvApp2VarErrorOutput::Maximum, 0, 3.)
            .unwrap();
        slots
            .set_value(AdvApp2VarErrorOutput::RootMeanSquare, 0, 5.)
            .unwrap();
        assert_eq!(values.lock().unwrap()[2], 3.);
        assert_eq!(values.lock().unwrap()[4], 5.);
        assert_eq!(Arc::strong_count(&values), 2);
        assert_eq!(values.lock().unwrap().as_ptr(), pointer);
        assert_eq!(std::mem::size_of::<OwnedErrorSlots>(), 32);
        assert_eq!(std::mem::size_of::<Option<[usize; 2]>>(), 24);
    }
}
