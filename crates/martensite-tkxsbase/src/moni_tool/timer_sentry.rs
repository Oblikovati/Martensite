//! `MoniToolTimerSentry`: starts a timer when created and stops it when dropped.
use super::MoniToolTimer;
use martensite_tkernel::standard::Handle;

/// `MoniToolTimerSentry`; the destructor (`Drop`) stops the timer unless `Stop` did.
///
/// ```
/// use martensite_tkxsbase::moni_tool::{MoniToolTimer, MoniToolTimerSentry};
/// use martensite_tkernel::standard::Handle;
/// let timer = Handle::new(MoniToolTimer::new());
/// {
///     let _sentry = MoniToolTimerSentry::new(&timer);
///     assert_eq!(timer.is_running(), 1);
/// }
/// assert_eq!(timer.is_running(), 0);
/// ```
pub struct MoniToolTimerSentry {
    my_timer: Handle<MoniToolTimer>,
}

impl MoniToolTimerSentry {
    /// `MoniToolTimerSentry(cname)`: starts the timer of the dictionary named `cname`.
    pub fn from_name(cname: &str) -> Self {
        Self::new(&MoniToolTimer::timer_named(cname))
    }
    /// `MoniToolTimerSentry(timer)`: starts `timer` (a null handle starts nothing; the
    /// native constructor dereferences it).
    pub fn new(timer: &Handle<MoniToolTimer>) -> Self {
        if let Some(live) = timer.get() {
            live.start();
        }
        Self {
            my_timer: timer.clone(),
        }
    }
    /// `Timer`.
    pub fn timer(&self) -> Handle<MoniToolTimer> {
        self.my_timer.clone()
    }
    /// `Stop`: stops the timer and forgets it.
    pub fn stop(&mut self) {
        if let Some(live) = self.my_timer.get() {
            live.stop();
        }
        self.my_timer.nullify();
    }
}

impl Drop for MoniToolTimerSentry {
    /// `~MoniToolTimerSentry`: stops the timer unless already stopped.
    fn drop(&mut self) {
        if let Some(live) = self.my_timer.get() {
            live.stop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_stop_forgets_the_timer() {
        let mut sentry = MoniToolTimerSentry::from_name("_test_sentry");
        let timer = sentry.timer();
        assert_eq!(timer.is_running(), 1);
        sentry.stop();
        assert!(sentry.timer().is_null());
        drop(sentry);
        assert_eq!(timer.is_running(), 0);
        assert_eq!(timer.count(), 1);
    }
}
