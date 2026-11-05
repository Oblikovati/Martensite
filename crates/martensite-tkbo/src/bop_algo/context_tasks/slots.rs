//! Context indices are the launcher's real worker indices; no global/thread-local map.
use martensite_tkernel::{osd::Launcher, standard::Failure};
use std::sync::OnceLock;

/// Borrow the supplied main context and own each lazily created worker context.
/// Example: `slots.at(launcher.upper_thread_index(), &factory)` returns the original main.
pub(super) struct ContextTaskSlots<'a, C> {
    main: &'a C,
    lower: i32,
    caller: i32,
    workers: Vec<OnceLock<C>>,
}

impl<'a, C> ContextTaskSlots<'a, C> {
    /// Retain the actual launcher bounds rather than assuming a worker count.
    /// Example: `ContextTaskSlots::new(&context, &launcher)`.
    pub(super) fn new(main: &'a C, launcher: &Launcher<'_>) -> Self {
        Self {
            main,
            lower: launcher.lower_thread_index(),
            caller: launcher.upper_thread_index(),
            workers: (0..launcher.nb_threads())
                .map(|_| OnceLock::new())
                .collect(),
        }
    }

    /// Resolve one exact worker index without cloning the main context.
    /// Example: `slots.at(worker_index, &Context::default)?`.
    pub(super) fn at(&self, index: i32, make: &impl Fn() -> C) -> Result<&C, Failure> {
        let Some(relative) = index.checked_sub(self.lower).filter(|value| *value >= 0) else {
            return Err(self.invalid_index(index));
        };
        let Some(slot) = self.workers.get(relative as usize) else {
            return Err(self.invalid_index(index));
        };
        if index == self.caller {
            return Ok(self.main);
        }
        Ok(slot.get_or_init(make))
    }

    fn invalid_index(&self, index: i32) -> Failure {
        Failure::out_of_range(&format!(
            "worker_index={index}; expected launcher index {}..={}",
            self.lower, self.caller
        ))
    }
}
