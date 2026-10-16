//! A performance case for timer CPU counters, checked through its log.
//!
//! The test rebuilds the log the case prints and checks it with the performance rules.
#[path = "case_log_rules/mod.rs"]
mod case_log_rules;

use case_log_rules::{COMMON_RULES, PERF_RULES, check_log};
use martensite_tkernel::osd::Timer;
use std::hint::black_box;

/// Stops the timer and prints its CPU user time as a named counter.
fn stop_counter(timer: &mut Timer, name: &str) -> String {
    timer.stop();
    let (_, _, _, cpu_seconds) = timer.show_values();
    format!("COUNTER {name}: {cpu_seconds}")
}

/// Two timers restarted around a loop of 10000 increments, then stopped with their CPU
/// counters printed.
#[test]
fn restarted_timers_report_cpu_counters() {
    const ITERATION_COUNT: i32 = 10000;
    let mut log = vec![
        "========".to_owned(),
        "TIMER_COUNTERS".to_owned(),
        "========".to_owned(),
        String::new(),
    ];
    let (mut first, mut second) = (Timer::new(false), Timer::new(false));
    first.restart();
    second.restart();
    let mut counter = 1;
    while counter != ITERATION_COUNT {
        counter = black_box(counter + 1);
    }
    log.push(stop_counter(&mut first, "bug_info_1"));
    log.push(stop_counter(&mut second, "bug_info_2"));
    assert!(
        first.show_values().3 >= 0.0 && second.show_values().3 >= 0.0,
        "{log:?}"
    );
    assert_eq!(check_log(&[PERF_RULES, COMMON_RULES], &[], &log), Ok(()));
}
