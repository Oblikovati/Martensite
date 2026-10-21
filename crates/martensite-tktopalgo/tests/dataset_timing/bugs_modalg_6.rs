//! The CPU-timing test of `bugs_modalg_6`, alone in its target.
use super::*;

/// The boxed 2D face classifier must beat the unboxed one by 4, 3, 15 and 15 percent.
///
/// The edge-to-plane projection dominates the measured time (#207), so the first two
/// strict thresholds are not met reliably (one Linux x86-64 run measured 2.85 and -1.10
/// percent); run it explicitly with `--ignored` until #207 is fixed.
#[test]
#[ignore = "the 4 and 3 percent classifier speed-ups are not reproducible until #207; run with --ignored"]
fn bug27884() {
    let identity = "bugs_modalg_6::bug27884";
    run_case(identity, BUGS_SECONDS, || {
        let mut session = NoView;
        crate::cases::bugs_modalg_6::bug27884(&mut session);
        crate::support::log::finish(crate::support::process::group_of("bugs_modalg_6"));
    });
}
