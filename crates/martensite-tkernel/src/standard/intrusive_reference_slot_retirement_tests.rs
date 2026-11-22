use crate::standard::{
    Handle, IntrusiveReferenceSlot, IntrusiveRetirementBorrow, StandardTransient,
};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::time::Duration;

const TEST_DEADLINE: Duration = Duration::from_secs(10);

type DropCallback = Box<dyn FnOnce(&RetiringProbe) + Send + Sync>;

#[derive(Default)]
struct RetiringProbe(Option<DropCallback>);

crate::impl_transient!(RetiringProbe, "Test_RetiringProbe", StandardTransient);

impl Drop for RetiringProbe {
    fn drop(&mut self) {
        if let Some(callback) = self.0.take() {
            callback(self);
        }
    }
}

#[test]
fn displaced_callback_reads_its_own_pointer_without_the_write_lock() {
    let slot = Arc::new(IntrusiveReferenceSlot::default());
    let callback_slot = Arc::downgrade(&slot);
    let observed = Arc::new(AtomicBool::new(false));
    let callback_observation = observed.clone();
    let retiring = Handle::new(RetiringProbe(Some(Box::new(move |retiring| {
        let slot = callback_slot.upgrade().unwrap();
        slot.with_retiring_reference(retiring, |borrow| {
            race_assignments(&slot, &borrow, callback_observation);
        })
        .unwrap();
    }))));

    slot.assign(&retiring).unwrap();
    drop(retiring);
    slot.clear().unwrap();

    assert!(observed.load(Ordering::SeqCst));
    assert!(slot.is_null());
}

fn race_assignments(
    slot: &Arc<IntrusiveReferenceSlot<RetiringProbe>>,
    borrow: &IntrusiveRetirementBorrow<'_, RetiringProbe>,
    observed: Arc<AtomicBool>,
) {
    let (retired, release_drop) = install_blocking_owner(borrow);
    let first = Handle::new(RetiringProbe::default());
    let callback_slot = Arc::downgrade(slot);
    std::thread::scope(|threads| {
        threads.spawn(|| borrow.assign(&first).unwrap());
        retired.recv_timeout(TEST_DEADLINE).unwrap();
        let second = Handle::new(RetiringProbe(Some(Box::new(move |retiring| {
            let slot = callback_slot.upgrade().unwrap();
            let visible = slot.members.try_read().is_ok_and(|members| {
                members.pointer.object_address() == std::ptr::from_ref(retiring).cast()
            });
            observed.store(visible, Ordering::SeqCst);
        }))));
        borrow.assign(&second).unwrap();
        drop(second);
        release_drop.send(()).unwrap();
    });
}

#[test]
fn unwinding_destructor_leaves_an_empty_slot_reusable() {
    let retiring = Handle::new(RetiringProbe(Some(Box::new(|_| panic!("drop failed")))));
    let slot = IntrusiveReferenceSlot::from_owned(retiring);
    let failure = catch_unwind(AssertUnwindSafe(|| slot.clear()));
    assert!(failure.is_err());
    assert!(slot.is_null());

    let next = Handle::new(RetiringProbe::default());
    slot.assign(&next).unwrap();
    assert_eq!(slot.object_address(), next.object_address());
    assert_eq!(next.ref_count(), 2);
}

fn install_blocking_owner(
    borrow: &IntrusiveRetirementBorrow<'_, RetiringProbe>,
) -> (mpsc::Receiver<()>, mpsc::Sender<()>) {
    let (retired, await_retired) = mpsc::channel();
    let (release_drop, await_release) = mpsc::channel();
    let await_release = Mutex::new(await_release);
    let blocker = Handle::new(RetiringProbe(Some(Box::new(move |_| {
        retired.send(()).unwrap();
        await_release
            .lock()
            .unwrap()
            .recv_timeout(TEST_DEADLINE)
            .unwrap();
    }))));
    borrow.assign(&blocker).unwrap();
    drop(blocker);
    (await_retired, release_drop)
}
