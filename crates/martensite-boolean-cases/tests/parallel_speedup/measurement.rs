//! Keep the performance decision independent of the script's two-decimal display (#480).
use martensite_boolean_cases::support::bugs_commands::options_bop::ScriptBop;
use martensite_tkernel::osd::Timer;

pub(super) fn filler_seconds(bop: &mut ScriptBop) -> f64 {
    let mut timer = Timer::new(false);
    timer.start();
    bop.bfillds();
    timer.stop();
    recorded_seconds(&timer)
}

pub(super) fn recorded_seconds(timer: &Timer) -> f64 {
    let seconds = timer.elapsed_time();
    println!("  Tps: {seconds:7.2}");
    seconds
}

pub(super) fn meets_requirement(sequential: f64, parallel: f64) -> bool {
    println!(
        "{{\"performance_case\":\"bug24639\",\"sequential_seconds\":{sequential:.17},\"parallel_seconds\":{parallel:.17},\"required_maximum_ratio\":0.8}}"
    );
    parallel <= 0.8 * sequential
}
