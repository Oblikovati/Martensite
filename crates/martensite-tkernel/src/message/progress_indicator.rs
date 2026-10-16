//! The thread-safe global progress scale and user hooks.
use super::{ProgressRange, ProgressScope};
use crate::impl_transient;
use crate::standard::StandardTransient;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

/// The methods a progress indicator implementation defines.
///
/// Implement `show` to present the progress and optionally `user_break` to cancel.
/// Hooks take `&self` because they are called through one
/// shared indicator: keep mutable presentation state behind a `Mutex` or atomics.
/// Example: `struct Bar; impl ProgressIndicatorHooks for Bar { fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {} }`.
pub trait ProgressIndicatorHooks: Send + Sync {
    /// Called on every advance, serialized by the
    /// indicator mutex; `position` is the position just stored.
    fn show(&self, scope: &ProgressScope<'_>, position: f64, force: bool);
    /// Polls for a user break: may run concurrently and must not call `show`; default false.
    fn user_break(&self) -> bool {
        false
    }
    /// Called by `ProgressIndicator::start` before the first show.
    fn reset(&self) {}
}

/// State shared by the indicator and every scope bound to it.
pub(super) struct IndicatorShared {
    /// The position as `f64` bits; read without the lock.
    position: AtomicU64,
    /// Serializes increment + show; never held while reading the position.
    increment_lock: Mutex<()>,
    hooks: Box<dyn ProgressIndicatorHooks>,
}
impl IndicatorShared {
    pub(super) fn position(&self) -> f64 {
        f64::from_bits(self.position.load(Ordering::Acquire))
    }
    fn set_position(&self, value: f64) {
        self.position.store(value.to_bits(), Ordering::Release);
    }
    pub(super) fn user_break(&self) -> bool {
        self.hooks.user_break()
    }
    pub(super) fn show(&self, scope: &ProgressScope<'_>, force: bool) {
        self.hooks.show(scope, self.position(), force);
    }
    /// Clamps `position + step` to 1 and shows under the lock.
    pub(super) fn increment(&self, step: f64, scope: &ProgressScope<'_>) {
        // A panicking hook poisons the lock; the guarded data is `()`, so continue.
        let _guard = self
            .increment_lock
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        self.set_position((self.position() + step).min(1.));
        self.show(scope, false);
    }
}

/// Owns the root scope and the global `[0, 1]` position.
///
/// The user's implementation is a boxed [`ProgressIndicatorHooks`]; the root
/// scope lives inside the indicator, so ranges and scopes borrow it and cannot
/// outlive it (enforced by lifetimes).
/// Example: `let range = ProgressIndicator::new(Box::new(hooks)).start();` (keep the indicator bound).
pub struct ProgressIndicator {
    // Shared by handle like other transient objects.
    parent: StandardTransient,
    shared: Arc<IndicatorShared>,
    root: ProgressScope<'static>,
}
impl_transient!(
    ProgressIndicator,
    "Message_ProgressIndicator",
    StandardTransient,
    parent
);
impl ProgressIndicator {
    /// Position 0 and an active unnamed root scope
    /// with max 1; `ProgressIndicator::new(Box::new(hooks))`.
    pub fn new(hooks: Box<dyn ProgressIndicatorHooks>) -> Self {
        let shared = Arc::new(IndicatorShared {
            position: AtomicU64::new(0f64.to_bits()),
            increment_lock: Mutex::new(()),
            hooks,
        });
        let root = ProgressScope::root(Arc::clone(&shared));
        Self {
            parent: StandardTransient::default(),
            shared,
            root,
        }
    }
    /// Resets position and root value to 0, calls `reset` then `show(root, false)`,
    /// and returns the root's `Next()` range covering the whole scale; `indicator.start()`.
    pub fn start(&self) -> ProgressRange<'_> {
        self.shared.set_position(0.);
        self.root.reset_value();
        self.shared.hooks.reset();
        self.shared.show(&self.root, false);
        self.root.next(1.)
    }
    /// Static `Start(handle)`: `start()` for an indicator, an unbound dummy range for
    /// a null handle; `ProgressIndicator::start_optional(None).is_active() == false`.
    pub fn start_optional(progress: Option<&Self>) -> ProgressRange<'_> {
        progress.map_or_else(ProgressRange::default, Self::start)
    }
    /// Returns the total position in `[0, 1]`; `indicator.get_position()`.
    pub fn get_position(&self) -> f64 {
        self.shared.position()
    }
    /// Borrows the unnamed root scope passed to `show` by `start`; `indicator.root_scope()`.
    pub fn root_scope(&self) -> &ProgressScope<'static> {
        &self.root
    }
}
impl Drop for ProgressIndicator {
    fn drop(&mut self) {
        // Disarm the root so closing it never increments a dropped indicator.
        self.root.disarm();
    }
}
