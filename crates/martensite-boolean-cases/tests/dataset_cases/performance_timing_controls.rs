//! Independent timing-boundary controls run outside the isolated performance binary (#480).
#[path = "../parallel_speedup/measurement.rs"]
mod measurement;
use martensite_boolean_cases::support::bugs_commands::options_bop::ScriptBop;
use martensite_tkernel::osd::Timer;

fn stopped_seconds(seconds: f64) -> f64 {
    let mut timer = Timer::new(false);
    timer.reset_to(seconds);
    measurement::recorded_seconds(&timer)
}

#[test]
fn displayed_centiseconds_cannot_falsely_reject_the_original_bound() {
    let serial = stopped_seconds(0.264);
    let parallel = stopped_seconds(0.207);
    assert_eq!(serial.to_bits(), 0.264_f64.to_bits());
    assert_eq!(parallel.to_bits(), 0.207_f64.to_bits());
    assert!(measurement::meets_requirement(serial, parallel));
    assert!(0.21 > 0.8 * 0.26);
}

#[test]
fn displayed_centiseconds_cannot_falsely_accept_the_original_bound() {
    let serial = stopped_seconds(0.254);
    let parallel = stopped_seconds(0.204);
    assert!(!measurement::meets_requirement(serial, parallel));
    assert!(0.20 <= 0.8 * 0.25);
}

#[test]
fn exact_twenty_percent_boundary_passes_and_next_float_fails() {
    assert!(measurement::meets_requirement(5., 4.));
    let above = f64::from_bits(4_f64.to_bits() + 1);
    assert!(!measurement::meets_requirement(5., above));
}

#[test]
fn filler_measurement_returns_finite_unrounded_elapsed_seconds() {
    let elapsed = measurement::filler_seconds(&mut ScriptBop::default());
    assert!(elapsed.is_finite());
    assert!(elapsed >= 0.);
}
