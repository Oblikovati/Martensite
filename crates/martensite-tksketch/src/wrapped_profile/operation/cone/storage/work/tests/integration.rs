//! Ruled-cell counts stay separate from arithmetic and are shared across members.
use super::{CylindricalFluxOptions, SurfaceRegionId, Work};
#[test]
fn independent_members_share_the_original_total_cell_limit() {
    let mut work = Work::new(Default::default());
    let requested = CylindricalFluxOptions {
        max_total_subintervals: 10,
        ..Default::default()
    };
    work.charge_integration(SurfaceRegionId(41), 3, requested)
        .unwrap();
    assert_eq!(work.integration(requested).max_total_subintervals, 7);
    work.charge_integration(SurfaceRegionId(42), 5, requested)
        .unwrap();
    assert_eq!(work.integration(requested).max_total_subintervals, 2);
    assert_eq!(work.integration_cells, 8);
    assert_eq!(work.consumed, 0);
}
#[test]
fn excess_and_overflow_counts_leave_completed_work_unchanged() {
    let region = SurfaceRegionId(42);
    let mut work = Work::new(Default::default());
    let requested = CylindricalFluxOptions {
        max_total_subintervals: 10,
        ..Default::default()
    };
    work.charge_integration(SurfaceRegionId(41), 8, requested)
        .unwrap();
    let error = work.charge_integration(region, 3, requested).unwrap_err();
    assert_eq!(error.region, Some(region));
    assert!(
        work.charge_integration(region, u64::MAX, requested)
            .is_err()
    );
    assert_eq!(work.integration_cells, 8);
}
#[test]
fn zero_budget_allows_completed_work_without_numerical_cells() {
    let mut work = Work::new(Default::default());
    let requested = CylindricalFluxOptions {
        max_total_subintervals: 0,
        ..Default::default()
    };
    work.charge_integration(SurfaceRegionId(41), 0, requested)
        .unwrap();
    assert_eq!(work.integration(requested).max_total_subintervals, 0);
    assert!(
        work.charge_integration(SurfaceRegionId(41), 1, requested)
            .is_err()
    );
}
