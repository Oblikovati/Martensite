//! Binding mismatches remain failures even if a later accessor returns the original object.
use super::{fixture, support::*};
use martensite_tkbo::bop_algo::{
    ProgressCompletion, WireOperationContext, WireSplitter, WireSplitterHooks,
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, RuntimeTyped},
};

struct AlternatingWireHooks {
    first: WireSplitter,
    second: WireSplitter,
    gets: usize,
    swallow: bool,
    switch_at: usize,
}

impl AlternatingWireHooks {
    fn new(swallow: bool) -> Self {
        let second = fixture::empty_splitter();
        add_error(&second, false);
        Self {
            first: fixture::empty_splitter(),
            second,
            gets: 0,
            swallow,
            switch_at: 2,
        }
    }
}

impl WireSplitterHooks for AlternatingWireHooks {
    fn wire_splitter(&mut self) -> &mut WireSplitter {
        self.gets += 1;
        if self.gets == self.switch_at {
            return &mut self.second;
        }
        &mut self.first
    }

    fn check_data(&mut self, operation: &WireOperationContext) -> Result<(), Failure> {
        let result = operation.check_data_default(self);
        if self.swallow {
            return Ok(());
        }
        result
    }
}

#[test]
fn alternating_a_b_a_default_delegation_refuses_before_touching_b() {
    for swallow in [false, true] {
        let mut hooks = AlternatingWireHooks::new(swallow);
        let outcome = hooks.perform_with_progress(&ProgressRange::default());
        require_kernel_failure(outcome, "Standard_ProgramError");
        assert_eq!(hooks.gets, 3);
        assert!(hooks.first.context().is_null());
        assert!(hooks.second.context().is_null());
        assert!(hooks.first.wes().unwrap().shapes().is_empty());
        assert!(
            hooks
                .second
                .options()
                .has_error(martensite_tkbo::bop_algo::AlertTooFewArguments::static_type())
        );
    }
}

#[test]
fn fresh_context_cannot_delegate_on_an_unbound_receiver() {
    let operation = WireOperationContext::new();
    let mut hooks = AuthoredWireHooks::new(fixture::empty_splitter(), WireCheckPolicy::Delegate);
    let cause = operation.check_data_default(&mut hooks).unwrap_err();
    assert_eq!(cause.exception_type(), "Standard_ProgramError");
    assert!(hooks.receiver.context().is_null());
    assert!(!hooks.receiver.options().has_errors());
}

#[test]
fn prepared_foreign_context_cannot_authorize_another_receiver() {
    let mut first = fixture::empty_splitter();
    let mut operation = WireOperationContext::new();
    drop(first.prepare_wires_extension(&mut operation).unwrap());
    let mut other = fixture::empty_splitter();
    let failure = other.prepare_wires_extension(&mut operation).err().unwrap();
    require_kernel_failure(*failure, "Standard_ProgramError");
    assert!(other.context().is_null());
    assert!(other.wes().unwrap().shapes().is_empty());
}

#[test]
fn consumed_context_rejects_default_delegation_and_repreparation() {
    let mut hooks = AuthoredWireHooks::new(fixture::empty_splitter(), WireCheckPolicy::Delegate);
    let mut operation = WireOperationContext::new();
    let prepared = hooks
        .receiver
        .prepare_wires_extension(&mut operation)
        .unwrap();
    assert_eq!(
        prepared.make_wires(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
    assert!(operation.check_data_default(&mut hooks).is_err());
    let failure = hooks
        .receiver
        .prepare_wires_extension(&mut operation)
        .err()
        .unwrap();
    require_kernel_failure(*failure, "Standard_ProgramError");
}

#[test]
fn final_classifier_rejects_late_replacement_after_real_helper_work() {
    let mut hooks = AlternatingWireHooks::new(false);
    hooks.switch_at = 4;
    let outcome = hooks.perform_with_progress(&ProgressRange::default());
    require_kernel_failure(outcome, "Standard_ProgramError");
    assert!(!hooks.first.context().is_null());
    assert!(hooks.second.context().is_null());
    assert!(
        hooks
            .second
            .options()
            .has_error(martensite_tkbo::bop_algo::AlertTooFewArguments::static_type())
    );
}

#[test]
fn unbound_default_error_cannot_be_erased_by_later_preparation() {
    let mut operation = WireOperationContext::new();
    let mut hooks = AuthoredWireHooks::new(fixture::empty_splitter(), WireCheckPolicy::Delegate);
    assert!(operation.check_data_default(&mut hooks).is_err());
    let result = hooks
        .receiver
        .prepare_wires_extension(&mut operation)
        .err()
        .unwrap();
    require_kernel_failure(*result, "Standard_ProgramError");
    assert!(hooks.receiver.context().is_null());
}
