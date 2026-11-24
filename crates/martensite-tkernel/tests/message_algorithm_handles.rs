//! Shared-handle regression for the algorithm base used by mesh builders.
use martensite_tkernel::{
    impl_transient,
    message::{Algorithm, AlgorithmSource, AlgorithmState, Status},
    standard::{Handle, RuntimeTyped, Transient},
};

struct DerivedMessageAlgorithm {
    state: AlgorithmState,
}
impl DerivedMessageAlgorithm {
    fn new() -> Self {
        Self {
            state: AlgorithmState::with_runtime_type(Self::static_type()),
        }
    }
}
impl_transient!(
    DerivedMessageAlgorithm,
    "Probe_MessageAlgorithm",
    dyn AlgorithmSource,
    interfaces: [dyn AlgorithmSource]
);
impl AlgorithmSource for DerivedMessageAlgorithm {
    fn algorithm_state(&self) -> &AlgorithmState {
        &self.state
    }
}

#[test]
fn algorithm_interface_shares_the_concrete_owners_reporting_state() {
    let derived = Handle::new(DerivedMessageAlgorithm::new());
    let base = derived.upcast::<dyn AlgorithmSource>();
    let root = base.upcast::<dyn Transient>();
    assert_eq!(base.object_address(), derived.object_address());
    base.set_status_with_integer(Status::FAIL2, 17);
    assert!(derived.state.get_status().is_set(Status::FAIL2));
    assert_eq!(base.get_message_numbers(Status::FAIL2).unwrap().len(), 1);
    assert_eq!(
        root.dynamic_type().unwrap().name(),
        "Probe_MessageAlgorithm"
    );
    assert_eq!(
        base.dynamic_type().unwrap().name(),
        "Probe_MessageAlgorithm"
    );
    assert!(!root.cast::<DerivedMessageAlgorithm>().is_null());
    let borrowed = root.view::<dyn AlgorithmSource>();
    assert!(std::ptr::eq(
        borrowed.get().unwrap().algorithm_state(),
        &derived.state
    ));
    assert_eq!(root.ref_count(), 3);
    base.change_status().clear_status(Status::FAIL2);
    assert!(!derived.state.get_status().is_fail());
    assert!(
        derived
            .state
            .get_message_numbers(Status::FAIL2)
            .unwrap()
            .contains(17)
    );
}

#[test]
fn standalone_algorithm_handles_and_self_merging_preserve_state() {
    let algorithm = Handle::new(Algorithm::new());
    let alias = algorithm.clone();
    algorithm.set_status_with_integer(Status::WARN1, 3);
    algorithm.set_status_with_c_str(Status::WARN1, b"first", true);
    algorithm.set_status_with_c_str(Status::WARN1, b"last", true);
    alias.add_status(&algorithm);
    assert_eq!(
        algorithm.get_message_strings(Status::WARN1).unwrap().len(),
        2
    );
    assert_eq!(
        algorithm.get_message_numbers(Status::WARN1).unwrap().len(),
        1
    );
    let root = alias.upcast::<dyn Transient>();
    assert_eq!(root.dynamic_type().unwrap().name(), "Message_Algorithm");
    assert!(!root.cast::<Algorithm>().is_null());
    let base = root.cast::<dyn AlgorithmSource>();
    assert_eq!(base.object_address(), alias.object_address());
    assert!(std::ptr::eq(
        <dyn AlgorithmSource>::static_type(),
        Algorithm::static_type()
    ));
    alias.clear_status();
    assert!(!algorithm.get_status().is_warn());
    assert!(algorithm.get_message_numbers(Status::WARN1).is_none());
}
