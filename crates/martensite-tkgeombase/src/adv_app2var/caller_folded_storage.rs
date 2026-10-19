//! Operation-local folded views preserve actual shared real identities per store.
#[cfg(test)]
use super::*;
/// One folded sample table; `AdvApp2VarFoldedOutput::SymmetricSum`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarFoldedOutput {
    /// Sum of samples at opposite parameters.
    SymmetricSum,
    /// Difference of samples at opposite parameters.
    AntisymmetricDifference,
}
impl AdvApp2VarFoldedOutput {
    pub(in crate::adv_app2var) fn index(self) -> usize {
        match self {
            Self::SymmetricSum => 0,
            Self::AntisymmetricDifference => 1,
        }
    }
}
pub(crate) type OwnedFoldedSlots =
    super::indexed_tables::OwnedCallerTableSlots<AdvApp2VarFoldedOutput, 2>;
impl super::indexed_tables::CallerRealTableRole for AdvApp2VarFoldedOutput {
    const LABEL: &'static str = "AdvApp2VarFoldedOutput";
    const POISON_MESSAGE: &'static str =
        "AdvApp2VarFoldedOutput: poisoned=true; expected initialized operation-local real owner";
    fn index(self) -> usize {
        self.index()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use AdvApp2VarFoldedOutput::{AntisymmetricDifference as Difference, SymmetricSum as Sum};
    #[test]
    fn indexed_folded_owner_preserves_signed_origins_and_immediate_identity() {
        let values = Arc::new(Mutex::new(vec![7.; 8]));
        let mut slots = OwnedFoldedSlots::new(Arc::clone(&values), [3, 3], -1);
        slots.set_value(Sum, 0, -0.).unwrap();
        assert_eq!(
            slots.value(Difference, 0).unwrap().to_bits(),
            (-0_f64).to_bits()
        );
        slots
            .with_values(Difference, |tail| {
                assert_eq!(tail.len(), 6);
                tail[1] = 9.;
            })
            .unwrap();
        assert_eq!(slots.value(Sum, 1).unwrap(), 9.);
        assert_eq!(values.lock().unwrap()[2].to_bits(), (-0_f64).to_bits());
        assert!(slots.value(Sum, -3).is_err());
        assert!(slots.set_value(Sum, 6, 0.).is_err());
    }
    #[test]
    fn folded_ranges_check_overflow_and_allow_an_empty_onepast_kernel() {
        let values = Arc::new(Mutex::new(vec![7.; 8]));
        let mut past = OwnedFoldedSlots::new(Arc::clone(&values), [7, 7], 1);
        assert_eq!(past.with_values(Sum, |tail| tail.len()).unwrap(), 0);
        assert!(past.value(Sum, 0).is_err());
        let mut overflow = OwnedFoldedSlots::new(values, [0, 0], i32::MAX);
        assert!(overflow.value(Sum, 1).is_err());
        assert!(overflow.set_value(Difference, 1, 0.).is_err());
        assert!(overflow.with_values(Sum, |_| ()).is_err());
    }
    #[test]
    fn folded_metadata_reuses_the_owner_and_is_send_sync() {
        fn assert_send_sync<Value: Send + Sync>() {}
        assert_send_sync::<OwnedFoldedSlots>();
        assert_send_sync::<AdvApp2VarFoldedOutput>();
        let values = Arc::new(Mutex::new(vec![7.; 8]));
        let pointer = values.lock().unwrap().as_ptr();
        let slots = OwnedFoldedSlots::new(Arc::clone(&values), [2, 4], 0);
        assert_eq!(Arc::strong_count(&values), 2);
        assert_eq!(slots.values.lock().unwrap().as_ptr(), pointer);
        assert_eq!(std::mem::size_of::<OwnedFoldedSlots>(), 32);
        assert_eq!(std::mem::size_of::<Option<[usize; 2]>>(), 24);
    }
}
