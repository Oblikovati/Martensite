//! Call the existing platform signal API rather than changing only a test-local requested flag.
use martensite_tkernel::osd::{SignalHost, SignalMode, SignalState};
pub(super) fn disable_fpe(state: &SignalState, host: &dyn SignalHost) {
    let stack_length = state.signal_stack_trace_length();
    state.set_signal(host, SignalMode::Set, false);
    state.set_signal_stack_trace_length(stack_length);
    println!(
        "Signal mode: {}\nCatch FPE: {}\nStack Trace Length: {stack_length}",
        format!("{:?}", state.signal_mode()).to_lowercase(),
        u8::from(host.to_catch_floating_signals())
    );
}
