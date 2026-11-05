//! Opaque one-operation binding and observations; no pointer is dereferenced or published.
use super::{WireSplitter, WireSplitterHooks};
use crate::bop_algo::ProgressCancellation;
use martensite_tkernel::standard::Failure;
use std::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};

const WIRE_FRESH: u8 = 0;
const WIRE_BOUND: u8 = 1;
const WIRE_PREPARED: u8 = 2;
const WIRE_CONSUMED: u8 = 3;

/// Owns one binding/cancellation lifetime; fields cannot certify successful geometry.
/// Example: `let mut operation = WireOperationContext::new();` for a prepared helper.
/// A context cannot be cloned, reset, rebound or reused after its helper is consumed.
pub struct WireOperationContext {
    receiver_address: AtomicUsize,
    phase: AtomicU8,
    invalid_binding: AtomicBool,
    observed: ProgressCancellation,
}

impl Default for WireOperationContext {
    fn default() -> Self {
        // Default must create fresh operation-local observations, never shared state.
        Self::new()
    }
}

impl WireOperationContext {
    /// Start without a receiver, scope or report effect; `WireOperationContext::new()`.
    pub fn new() -> Self {
        Self {
            receiver_address: AtomicUsize::new(0),
            phase: AtomicU8::new(WIRE_FRESH),
            invalid_binding: AtomicBool::new(false),
            observed: ProgressCancellation::default(),
        }
    }

    /// Delegate on the actual bound receiver; `operation.check_data_default(&mut wrapper)?`.
    pub fn check_data_default<H: WireSplitterHooks + ?Sized>(
        &self,
        hooks: &mut H,
    ) -> Result<(), Failure> {
        self.require_phase(WIRE_BOUND)?;
        self.guard_result(Ok(()))?;
        self.checked_hooks(hooks)?.check_data();
        Ok(())
    }

    pub(super) fn bind_hooks<'a, H: WireSplitterHooks + ?Sized>(
        &self,
        hooks: &'a mut H,
    ) -> Result<&'a mut WireSplitter, Failure> {
        let receiver = hooks.wire_splitter();
        self.bind_direct(receiver)?;
        Ok(receiver)
    }

    pub(super) fn checked_hooks<'a, H: WireSplitterHooks + ?Sized>(
        &self,
        hooks: &'a mut H,
    ) -> Result<&'a mut WireSplitter, Failure> {
        let receiver = hooks.wire_splitter();
        self.check_direct(receiver)?;
        Ok(receiver)
    }

    pub(super) fn bind_direct(&self, receiver: &WireSplitter) -> Result<(), Failure> {
        self.require_phase(WIRE_FRESH)?;
        self.guard_result(Ok(()))?;
        self.receiver_address
            .store(receiver_address(receiver), Ordering::Release);
        self.phase.store(WIRE_BOUND, Ordering::Release);
        Ok(())
    }

    pub(super) fn check_direct(&self, receiver: &WireSplitter) -> Result<(), Failure> {
        let bound = self.receiver_address.load(Ordering::Acquire);
        if bound != 0 && bound == receiver_address(receiver) {
            return Ok(());
        }
        self.invalid_binding.store(true, Ordering::Release);
        Err(context_failure(
            "changed or unbound receiver",
            self.phase.load(Ordering::Acquire),
        ))
    }

    pub(super) fn mark_prepared(&self) -> Result<(), Failure> {
        self.require_phase(WIRE_BOUND)?;
        self.guard_result(Ok(()))?;
        self.phase.store(WIRE_PREPARED, Ordering::Release);
        Ok(())
    }

    pub(super) fn consume_helper(&self, receiver: &WireSplitter) -> Result<(), Failure> {
        self.require_phase(WIRE_PREPARED)?;
        self.check_direct(receiver)?;
        self.guard_result(Ok(()))?;
        self.phase.store(WIRE_CONSUMED, Ordering::Release);
        Ok(())
    }

    pub(super) fn guard_result(&self, result: Result<(), Failure>) -> Result<(), Failure> {
        if result.is_err() || !self.invalid_binding.load(Ordering::Acquire) {
            return result;
        }
        Err(context_failure(
            "observed receiver mismatch",
            self.phase.load(Ordering::Acquire),
        ))
    }

    pub(super) fn observation(&self) -> &ProgressCancellation {
        // Classification reuses the observed stop without polling caller hooks again.
        &self.observed
    }

    fn require_phase(&self, expected: u8) -> Result<(), Failure> {
        let phase = self.phase.load(Ordering::Acquire);
        if phase == expected {
            return Ok(());
        }
        self.invalid_binding.store(true, Ordering::Release);
        Err(context_failure("unavailable operation phase", phase))
    }
}

fn receiver_address(receiver: &WireSplitter) -> usize {
    // This is a transient borrow-binding token, never geometry identity or serialized provenance.
    std::ptr::from_ref(receiver).addr()
}

fn context_failure(reason: &str, phase: u8) -> Failure {
    let message = format!(
        "WireSplitter {reason}, phase={phase}; expected the same receiver in one fresh operation"
    );
    Failure::with_type("Standard_ProgramError", &message, "")
}
