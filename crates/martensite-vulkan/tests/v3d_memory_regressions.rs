//! Memory regressions of the V3d group: removed presentations release their heap memory.
#![cfg(target_os = "linux")]
include!("case_host/session_modules.rs");
use crate::case_view::viewer_session::DisplayRequest;
use crate::primitive_shapes::{box_at, box_sized};
use crate::session_run::{SessionRun, banner};
use std::error::Error;

type Outcome = Result<(), Box<dyn Error>>;

/// The glibc allocator statistics, as `mallinfo2` returns them.
#[repr(C)]
struct HeapStatistics {
    arena: usize,
    ordblks: usize,
    smblks: usize,
    hblks: usize,
    hblkhd: usize,
    usmblks: usize,
    fsmblks: usize,
    uordblks: usize,
    fordblks: usize,
    keepcost: usize,
}

// The heap figure of the memory report is the allocator statistic, available only by FFI.
#[allow(unsafe_code)]
unsafe extern "C" {
    fn mallinfo2() -> HeapStatistics;
}

/// Bytes allocated on the heap by `malloc`, the heap figure of the memory report.
#[allow(unsafe_code)]
fn heap_usage() -> usize {
    // SAFETY: mallinfo2 takes no arguments and returns a plain struct by value; it only
    // reads allocator statistics of this process.
    unsafe { mallinfo2() }.uordblks
}

/// `checktrend values 0 1 message`: the mean step must stay within 1 + 2 sigma of zero.
fn check_trend(values: &[usize], message: &str) -> Outcome {
    let steps: Vec<f64> = values
        .windows(2)
        .map(|w| w[1] as f64 - w[0] as f64)
        .collect();
    let mean = steps.iter().sum::<f64>() / steps.len() as f64;
    let spread = steps.iter().map(|d| (d - mean) * (d - mean)).sum::<f64>();
    let sigma = (spread / (values.len() - 2) as f64).sqrt();
    println!(
        "Checking trend: nb = {}, mean delta = {mean}, sigma = {sigma}",
        values.len()
    );
    if mean.abs() > 1. + 2. * sigma {
        println!(
            "Checking trend failed: mean delta per step = {mean}, sigma = {sigma}, expected delta = 0"
        );
        return Err(message.into());
    }
    Ok(())
}

/// `bug5682`: displaying and clearing ten boxes fifty times leaves the heap where it was.
fn removed_objects_memory() -> Outcome {
    banner(&[
        "============",
        "OCC5682: AisInteractiveContext::Remove does not remove the object from memory",
        "============",
        "",
    ]);
    let boxes: Vec<_> = (1..=10)
        .map(|i| box_at([f64::from(i), 0., 0.], [1., 2., 3.]))
        .collect::<Result<_, _>>()?;
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    // The selection mode command fails without objects; the original catches it.
    let _ = run.session().set_selection_mode(4, true);
    run.session()
        .display("b", &box_sized([1., 1., 1.])?, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().clear()?;
    let base = heap_usage();
    println!("Memory base: {} KiB", base / 1024);
    let mut cleared = Vec::new();
    for _ in 0..50 {
        for (index, shape) in boxes.iter().enumerate() {
            run.session().display(
                &format!("b{}", index + 1),
                shape,
                &DisplayRequest::default(),
            )?;
        }
        run.session().fit()?;
        println!(
            "Memory usage: {} KiB",
            (heap_usage() as i64 - base as i64) / 1024
        );
        run.session().clear()?;
        cleared.push(heap_usage());
    }
    println!("{cleared:?}");
    check_trend(&cleared, "Memory leak detected")?;
    run.finish_without_image()
}

#[test]
fn bug5682() {
    case_v3d_process_case::run_case("bug5682", removed_objects_memory);
}
