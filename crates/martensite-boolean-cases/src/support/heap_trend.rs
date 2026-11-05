//! `meminfo h` and `checktrend`: whether the heap grows along repeated work.
use super::script_error::script_error;
use martensite_tkernel::osd::{MemCounter, MemInfo};

/// The bytes in use on the heap of the whole process, writing nothing.
/// Example: `samples.push(heap_bytes())`.
pub fn heap_bytes() -> f64 {
    let mut info = MemInfo::new(false);
    info.set_active(false);
    info.set_counter_active(MemCounter::HeapUsage, true);
    info.update();
    info.value(MemCounter::HeapUsage) as f64
}

/// `meminfo h`: the bytes in use on the heap, written as the command writes them.
/// Example: `samples.push(meminfo_heap())`.
pub fn meminfo_heap() -> f64 {
    let heap = heap_bytes();
    println!("{heap} ");
    heap
}

/// The mean step and its deviation; `None` under three values, which the procedure accepts.
fn trend_of(values: &[f64]) -> Option<(f64, f64)> {
    if values.len() < 3 {
        return None;
    }
    let steps: Vec<f64> = values.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let mean = steps.iter().sum::<f64>() / steps.len() as f64;
    let squares: f64 = steps
        .iter()
        .map(|step| martensite_cmath::powi(step - mean, 2))
        .sum();
    Some((mean, (squares / (values.len() - 2) as f64).sqrt()))
}

/// `checktrend values delta tolerance message`: a step that is clearly off stops the script
/// with `message`. Example: `checktrend(&samples, 0., 100., "Memory leak detected")`.
pub fn checktrend(values: &[f64], delta: f64, tolerance: f64, message: &str) {
    let Some((mean, sigma)) = trend_of(values) else {
        return;
    };
    let count = values.len();
    println!("Checking trend: nb = {count}, mean delta = {mean:?}, sigma = {sigma:?}");
    if (mean - delta).abs() > tolerance + 2. * sigma {
        println!(
            "Checking trend failed: mean delta per step = {mean:?}, sigma = {sigma:?}, expected delta = {delta}"
        );
        script_error(message);
    }
}

#[test]
fn a_constant_step_has_its_mean_and_no_deviation() {
    assert_eq!(trend_of(&[1., 3., 5.]), Some((2., 0.)));
    assert_eq!(trend_of(&[1., 3.]), None);
}
