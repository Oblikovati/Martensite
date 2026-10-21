//! `meminfo h` (Draw_BasicCommands.cxx dmeminfo) and the `checktrend` procedure
//! (resources/DrawResources/TestCommands.tcl) the memory-leak cases use.
use super::draw::{Draw, Reply, Tcl, TclError};
use martensite_tkernel::osd::{MemCounter, MemInfo};

/// `meminfo h`: `OSD_MemInfo(false)` with only MemHeapUsage active, updated, printed
/// `%.17g` followed by a blank; returns the counter value; `meminfo_heap(&mut draw)?`.
pub fn meminfo_heap(draw: &mut Draw) -> Tcl<f64> {
    let mut value = 0.0;
    draw.command(&["meminfo", "h"], |_| {
        let mut info = MemInfo::new(false);
        info.set_active(false);
        info.set_counter_active(MemCounter::HeapUsage, true);
        info.update();
        // `double(aMemInfo.Value(...))`: an unknown counter is size_t(-1) as a double.
        value = info.value(MemCounter::HeapUsage) as f64;
        Ok(Reply::ok(format!("{value} ")))
    })?;
    Ok(value)
}

/// `checktrend listval delta tolerance message`: with three values or more, raise
/// `message` when the mean step deviates from `delta` by more than `tolerance` plus two
/// standard deviations of the steps; returns whether the trend is clearly within range
/// (always false below three values). `delta` is the script's word, as Tcl prints it;
/// `checktrend(&mut draw, &values, "0", 1., "leak")?`.
pub fn checktrend(
    draw: &mut Draw,
    values: &[f64],
    delta: &str,
    tolerance: f64,
    message: &str,
) -> Tcl<bool> {
    let count = values.len();
    if count < 3 {
        return Ok(false);
    }
    let expected: f64 = delta.parse().expect("checktrend delta is a number");
    let steps: Vec<f64> = values.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let mean = steps.iter().fold(0.0, |sum, step| sum + step) / (count - 1) as f64;
    let squares = steps
        .iter()
        .fold(0.0, |sum, step| sum + (step - mean) * (step - mean));
    let sigma = (squares / (count - 2) as f64).sqrt();
    let (mean_text, sigma_text) = (tcl_double(mean), tcl_double(sigma));
    draw.puts(&format!(
        "Checking trend: nb = {count}, mean delta = {mean_text}, sigma = {sigma_text}"
    ));
    if (mean - expected).abs() > tolerance + 2. * sigma {
        draw.puts(&format!(
            "Checking trend failed: mean delta per step = {mean_text}, sigma = {sigma_text}, expected delta = {delta}"
        ));
        return Err(TclError(message.to_string()));
    }
    Ok((mean - expected).abs() <= sigma && sigma <= tolerance)
}

/// Tcl's string form of a double: the shortest round-trip digits, with `.0` on an
/// integral value (Tcl 8.6, `tcl_precision` 0); `tcl_double(0.) == "0.0"`.
fn tcl_double(value: f64) -> String {
    let text = format!("{value}");
    if value.is_finite() && !text.contains(['.', 'e']) {
        return format!("{text}.0");
    }
    text
}

#[test]
fn checktrend_follows_the_procedure() {
    let mut draw = Draw::default();
    assert!(!checktrend(&mut draw, &[1., 2.], "0", 1., "m").unwrap());
    // Constant memory: mean 0, sigma 0, clearly within range.
    assert!(checktrend(&mut draw, &[5., 5., 5., 5.], "0", 1., "m").unwrap());
    assert!(
        draw.log
            .text()
            .contains("Checking trend: nb = 4, mean delta = 0.0, sigma = 0.0")
    );
    // A steady growth of 100 per step is a leak.
    let error = checktrend(&mut draw, &[0., 100., 200., 300.], "0", 1., "leak").unwrap_err();
    assert_eq!(error.0, "leak");
    assert!(draw.log.text().contains(
        "Checking trend failed: mean delta per step = 100.0, sigma = 0.0, expected delta = 0"
    ));
}
