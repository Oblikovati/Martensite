//! A final Arc release runs native Rust destruction once and cannot revive weak references.
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Handle, Transient, WeakHandle};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

struct DropProbe {
    drops: Arc<AtomicUsize>,
    weak: OnceLock<WeakHandle<DropProbe>>,
}
impl_transient!(DropProbe, "DropProbe", dyn Transient);

impl Drop for DropProbe {
    fn drop(&mut self) {
        assert!(self.weak.get().unwrap().upgrade().is_null());
        self.drops.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn last_owner_can_be_of_any_view_and_weak_does_not_retain_the_object() {
    let drops = Arc::new(AtomicUsize::new(0));
    let owner = Handle::new(DropProbe {
        drops: drops.clone(),
        weak: OnceLock::new(),
    });
    let weak = owner.downgrade();
    owner.weak.set(weak.clone()).unwrap();
    let root = owner.upcast::<dyn Transient>();
    assert_eq!(root.ref_count(), 2);
    drop(owner);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(weak.ref_count(), 1);
    drop(root);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    assert!(!weak.is_null());
    assert!(weak.upgrade().is_null());
}
