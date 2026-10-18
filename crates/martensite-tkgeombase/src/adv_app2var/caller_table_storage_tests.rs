//! Caller table storage tests behavior and supporting operations.
use super::*;
#[derive(Clone, Copy)]
enum SlotRole {
    First,
    Second,
}
impl CallerRealTableRole for SlotRole {
    const LABEL: &'static str = "generic caller table test";
    const POISON_MESSAGE: &'static str = "generic caller table test poisoned";
    fn index(self) -> usize {
        self as usize
    }
}
#[test]
fn integer_tables_reuse_checked_signed_identity_and_scoped_mutation() {
    let owner = Arc::new(Mutex::new(vec![11_i32, 22, 33]));
    let mut table = OwnedCallerTableSlots::<SlotRole, 2, i32>::new(Arc::clone(&owner), [1, 1], -1);
    assert_eq!(table.value(SlotRole::First, 0).unwrap(), 11);
    table.set_value(SlotRole::First, 1, 44).unwrap();
    assert_eq!(table.value(SlotRole::Second, 1).unwrap(), 44);
    assert_eq!(
        table
            .with_values(SlotRole::First, |values| values.iter().sum::<i32>())
            .unwrap(),
        88
    );
    assert_eq!(*owner.lock().unwrap(), [11, 44, 33]);
}
#[test]
fn default_real_table_keeps_existing_failure_text_and_no_write_on_bad_index() {
    let owner = Arc::new(Mutex::new(vec![11., 22.]));
    let mut table = OwnedCallerTableSlots::<SlotRole, 2>::new(Arc::clone(&owner), [1, 1], 0);
    let failure = table.set_value(SlotRole::First, 2, 44.).unwrap_err();
    assert!(failure.to_string().contains(
        "AdvApp2VarIsoRealFrame: output origin=1, index=2; expected physical real slot in 0..2"
    ));
    assert_eq!(*owner.lock().unwrap(), [11., 22.]);
}
