//! Exact binder chain teardown, including the native retained-middle mutation.
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, RuntimeType, RuntimeTyped, StandardTransient},
};
use martensite_tkxsbase::transfer::{TransferBinder, TransferBinderBase};
use std::sync::{Arc, Mutex};

type ReleaseTrace = Arc<Mutex<Vec<usize>>>;
struct CountedBinder {
    transient: StandardTransient,
    base: TransferBinderBase,
    number: usize,
    trace: ReleaseTrace,
}
impl_transient!(CountedBinder, "CountedBinder", dyn TransferBinder,
    transient, interfaces: [dyn TransferBinder]);
impl TransferBinder for CountedBinder {
    fn binder_base(&self) -> &TransferBinderBase {
        &self.base
    }
    fn result_type(&self) -> &'static RuntimeType {
        StandardTransient::static_type()
    }
    fn result_type_name(&self) -> String {
        "owned".into()
    }
}
impl Drop for CountedBinder {
    fn drop(&mut self) {
        self.trace.lock().unwrap().push(self.number);
    }
}
fn binder(number: usize, trace: &ReleaseTrace) -> Handle<dyn TransferBinder> {
    Handle::new(CountedBinder {
        transient: StandardTransient::default(),
        base: TransferBinderBase::default(),
        number,
        trace: trace.clone(),
    })
    .cast()
}
fn chain(
    length: usize,
    retained_index: Option<usize>,
    trace: &ReleaseTrace,
) -> (Handle<dyn TransferBinder>, Handle<dyn TransferBinder>) {
    let head = binder(0, trace);
    let mut retained = Handle::default();
    for number in 1..length {
        let current = binder(number, trace);
        head.add_result(&current);
        if Some(number) == retained_index {
            retained = current;
        }
    }
    (head, retained)
}
#[test]
fn unshared_chain_drops_once_in_forward_order_on_a_small_stack() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let trace = ReleaseTrace::default();
            let (head, retained) = chain(20_000, None, &trace);
            assert!(retained.is_null());
            drop(head);
            assert_eq!(*trace.lock().unwrap(), (0..20_000).collect::<Vec<_>>());
        })
        .unwrap()
        .join()
        .unwrap();
}
#[test]
fn external_terminal_owner_delays_only_the_terminal_binder() {
    let trace = ReleaseTrace::default();
    let (head, retained) = chain(4, Some(3), &trace);
    drop(head);
    assert_eq!(*trace.lock().unwrap(), [0, 1, 2]);
    assert!(!retained.is_null() && retained.next_result().is_null());
    drop(retained);
    assert_eq!(*trace.lock().unwrap(), [0, 1, 2, 3]);
}
fn assert_retained_middle(index: usize, before: &[usize], after: &[usize]) {
    let trace = ReleaseTrace::default();
    let (head, retained) = chain(4, Some(index), &trace);
    assert!(!retained.next_result().is_null());
    drop(head);
    assert_eq!(*trace.lock().unwrap(), before);
    // The source tests NEXT's count, so the retained current binder loses its tail.
    assert!(retained.next_result().is_null());
    drop(retained);
    assert_eq!(*trace.lock().unwrap(), after);
}
#[test]
fn retained_middle_preserves_native_next_count_teardown_policy() {
    assert_retained_middle(1, &[0, 2, 3], &[0, 2, 3, 1]);
    assert_retained_middle(2, &[0, 1, 3], &[0, 1, 3, 2]);
}
