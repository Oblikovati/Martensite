//! Pin the existing abandonment contract while #372 records its resource cost.
use super::PendingEmplaceValues;
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
};

struct OwnedGapProbe(Rc<Cell<usize>>);
impl Drop for OwnedGapProbe {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn pending_constructor_unwind_abandons_owned_gap_resources() {
    let drops = Rc::new(Cell::new(0));
    let failure = catch_unwind(AssertUnwindSafe(|| {
        let mut pending = PendingEmplaceValues::with_capacity(2);
        pending.push(OwnedGapProbe(drops.clone()));
        pending.push(OwnedGapProbe(drops.clone()));
        panic!("NamedGapConstructorFailure: expected replay abandonment");
    }));
    assert!(failure.is_err());
    assert_eq!(drops.get(), 0);
    // Unlike a destructor-only counter, retained strong owners prove actual abandonment.
    assert_eq!(Rc::strong_count(&drops), 3);
}

#[test]
fn committed_gap_values_keep_normal_owned_destruction() {
    let drops = Rc::new(Cell::new(0));
    let mut pending = PendingEmplaceValues::with_capacity(2);
    pending.push(OwnedGapProbe(drops.clone()));
    pending.push(OwnedGapProbe(drops.clone()));
    let values = pending.release();
    assert_eq!(drops.get(), 0);
    drop(values);
    assert_eq!(drops.get(), 2);
    assert_eq!(Rc::strong_count(&drops), 1);
}
