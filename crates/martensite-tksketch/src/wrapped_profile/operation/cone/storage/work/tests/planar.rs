//! Numeric reference controls for the shared original/decoded census allowance.
use super::{SurfaceRegionId, Work};
use martensite_tkbo::profile_extrusion::{PlanarProofWorkLimit, PlanarProofWorkReceipt};
fn receipt(limit: u16, consumed: u64) -> PlanarProofWorkReceipt {
    PlanarProofWorkReceipt {
        requested_pair_bisections: limit,
        effective_pair_bisections: limit,
        consumed_pair_bisections: consumed,
    }
}
#[test]
fn original_and_decoded_censuses_consume_one_numeric_allowance() {
    let mut work = Work::new(Default::default());
    let requested = PlanarProofWorkLimit::PairBisections(10);
    assert_eq!(work.planar_remaining(requested), requested);
    work.charge_planar(SurfaceRegionId(41), receipt(10, 3))
        .unwrap();
    assert_eq!(
        work.planar_remaining(requested),
        PlanarProofWorkLimit::PairBisections(7)
    );
    work.charge_planar(SurfaceRegionId(42), receipt(7, 4))
        .unwrap();
    assert_eq!(
        work.planar_remaining(requested),
        PlanarProofWorkLimit::PairBisections(3)
    );
    assert_eq!(work.planar_receipt(), Some(receipt(10, 7)));
}
#[test]
fn altered_remaining_limit_is_refused_without_losing_original_work() {
    let region = SurfaceRegionId(41);
    let mut work = Work::new(Default::default());
    work.charge_planar(region, receipt(10, 3)).unwrap();
    let error = work.charge_planar(region, receipt(8, 1)).unwrap_err();
    assert_eq!(error.region, Some(region));
    assert_eq!(work.planar_receipt(), Some(receipt(10, 3)));
}
#[test]
fn exhausted_and_overflowing_census_counts_keep_the_actual_owner() {
    let region = SurfaceRegionId(42);
    let mut work = Work::new(Default::default());
    work.charge_planar(SurfaceRegionId(41), receipt(10, 3))
        .unwrap();
    let error = work.charge_planar(region, receipt(7, 8)).unwrap_err();
    assert_eq!(error.region, Some(region));
    let overflow = work
        .charge_planar(region, receipt(7, u64::MAX))
        .unwrap_err();
    assert_eq!(overflow.region, Some(region));
    assert_eq!(work.planar_receipt(), Some(receipt(10, 3)));
}
