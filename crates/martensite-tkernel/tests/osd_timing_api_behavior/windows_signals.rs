//! Native console events stay inside disposable, hidden consoles (#334, #88, #340).
#![allow(unsafe_code)]
use martensite_tkernel::osd::{Osd, SignalMode};
use std::time::{Duration, Instant};
use windows_sys::Win32::System::Console::{
    CTRL_BREAK_EVENT, CTRL_C_EVENT, GenerateConsoleCtrlEvent, GetConsoleProcessList,
    SetConsoleCtrlHandler,
};
use winsafe::{co, prelude::*};

const CHILD_MARKER: &str = "martensite-private-signal-console-child";
const CHILD_SUCCESS: u32 = 67;

fn private_console_test(name: &str, check: fn()) {
    if std::env::args().any(|argument| argument == CHILD_MARKER) {
        check();
        std::process::exit(CHILD_SUCCESS as i32);
    }
    let child = private_console_child(name);
    assert_private_child_exit(name, &child.hProcess);
}

fn private_console_child(name: &str) -> winsafe::guard::CloseHandlePiGuard {
    let executable = std::env::current_exe().unwrap();
    let command = format!(
        "\"{}\" --exact windows_signals::{name} --skip {CHILD_MARKER} --test-threads=1",
        executable.display()
    );
    winsafe::CreateProcess(
        None,
        Some(&command),
        None,
        None,
        false,
        co::CREATE::NEW_CONSOLE,
        &[],
        None,
        &mut hidden_console_startup(),
    )
    .unwrap()
}

fn hidden_console_startup() -> winsafe::STARTUPINFO<'static, 'static> {
    let mut startup = winsafe::STARTUPINFO::default();
    startup.dwFlags = co::STARTF::USESHOWWINDOW;
    startup.set_wShowWindow(co::SW::HIDE);
    startup
}

fn assert_private_child_exit(name: &str, process: &winsafe::HPROCESS) {
    let waited = process.WaitForSingleObject(Some(15_000)).unwrap();
    if waited != co::WAIT::OBJECT_0 {
        process.TerminateProcess(1).unwrap();
        process.WaitForSingleObject(Some(5_000)).unwrap();
    }
    assert_eq!(
        waited,
        co::WAIT::OBJECT_0,
        "{name}: expected child completion within 15 seconds"
    );
    assert_eq!(
        process.GetExitCodeProcess().unwrap(),
        CHILD_SUCCESS,
        "{name}: expected the selected child check to finish"
    );
}

fn check_console_events() {
    assert!(Osd::signal_capabilities().console_interruptions);
    let mut processes = [0u32; 2];
    // SAFETY: this writable array has the advertised capacity. Refuse to generate
    // events unless Windows confirms the console contains this process alone.
    let count = unsafe { GetConsoleProcessList(processes.as_mut_ptr(), 2) };
    assert_eq!(count, 1);
    assert_eq!(processes[0], std::process::id());
    // SAFETY: a null callback with false clears only this child's inherited Ctrl-C ignore flag.
    assert_ne!(unsafe { SetConsoleCtrlHandler(None, 0) }, 0);
    Osd::set_signal(SignalMode::Set, false);
    for event in [CTRL_C_EVENT, CTRL_BREAK_EVENT] {
        // SAFETY: both event identifiers are valid; the private console was checked above.
        assert_ne!(unsafe { GenerateConsoleCtrlEvent(event, 0) }, 0);
        wait_for_console_break();
    }
    Osd::set_signal(SignalMode::Unset, false);
}

fn wait_for_console_break() {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Err(failure) = Osd::control_break() {
            assert_eq!(failure.exception_type(), "OSD_Exception_CTRL_BREAK");
            assert!(
                Osd::control_break().is_ok(),
                "expected interruption to be consumed once"
            );
            return;
        }
        assert!(
            Instant::now() < deadline,
            "expected console cancellation within 5 seconds"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn check_floating_controls() {
    assert!(Osd::signal_capabilities().floating_point_trap_control);
    Osd::set_thread_local_signal(SignalMode::Set, true);
    let enabled = Osd::to_catch_floating_signals();
    Osd::set_thread_local_signal(SignalMode::Unset, false);
    assert!(enabled);
    assert!(!Osd::to_catch_floating_signals());
    assert!(!Osd::signal_capabilities().recoverable_native_faults);
}

#[test]
fn console_control_events_reach_cancellation_once_in_private_child() {
    private_console_test(
        "console_control_events_reach_cancellation_once_in_private_child",
        check_console_events,
    );
}

#[test]
fn floating_trap_controls_round_trip_in_private_child() {
    private_console_test(
        "floating_trap_controls_round_trip_in_private_child",
        check_floating_controls,
    );
}
