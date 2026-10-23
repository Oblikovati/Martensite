//! The CPU-timing test of `bugs_modalg_6`, alone in its target.
use super::*;

/// The boxed 2D face classifier must beat the unboxed one by 4, 3, 15 and 15 percent.
///
/// Classification reuses finite planar bounds within one operation without storing curves on input edges.
#[test]
fn bug27884() {
    let identity = "bugs_modalg_6::bug27884";
    run_case(identity, BUGS_SECONDS, || {
        let mut session = NoView;
        crate::cases::bugs_modalg_6::bug27884(&mut session);
        crate::support::log::finish(crate::support::process::group_of("bugs_modalg_6"));
    });
}
