//! Timer lifetime checks use owned counters, without sleeps or shared named timers.
use martensite_tkernel::standard::Handle;
use martensite_tkxsbase::moni_tool::{MoniToolTimer, MoniToolTimerSentry};

#[test]
fn nested_scope_drops_stop_exactly_one_timer_level_each() {
    let timer = Handle::new(MoniToolTimer::new());
    let outer = MoniToolTimerSentry::new(&timer);
    let inner = MoniToolTimerSentry::new(&timer);
    assert_eq!((timer.is_running(), timer.count()), (2, 2));
    drop(inner);
    assert_eq!((timer.is_running(), timer.count()), (1, 2));
    drop(outer);
    assert_eq!((timer.is_running(), timer.count()), (0, 2));
}

#[test]
fn stopped_sentry_drop_preserves_another_active_scope() {
    let timer = Handle::new(MoniToolTimer::new());
    let outer = MoniToolTimerSentry::new(&timer);
    let mut inner = MoniToolTimerSentry::new(&timer);
    inner.stop();
    assert!(inner.timer().is_null());
    drop(inner);
    assert_eq!((timer.is_running(), timer.count()), (1, 2));
    drop(outer);
    assert_eq!((timer.is_running(), timer.count()), (0, 2));
}
