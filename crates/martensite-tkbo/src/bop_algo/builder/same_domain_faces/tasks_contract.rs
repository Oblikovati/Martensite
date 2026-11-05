//! These owned invariants do not relabel the Native ordinary-false candidate as an exception.
use super::super::fixtures::{SameDomainFixtureKind as Kind, same_domain_fixture};
use super::*;
use crate::bop_algo::progress_test_support::pulse_indicator;

#[test]
fn pair_options_copy_only_fuzzy_and_keep_native_default_parallel_flag() {
    static_assertions::assert_impl_all!(SameDomainPairWork<'static>: Send);
    static_assertions::assert_not_impl_any!(SameDomainPairWork<'static>: Sync);
    static_assertions::assert_impl_all!(CheckedSameDomainPair: Send, Sync);
    let fixture = same_domain_fixture(Kind::Cylinder);
    let range = ProgressRange::default();
    let pair = (
        fixture.sources[0].clone().into_shape(),
        fixture.sources[1].clone().into_shape(),
    );
    let task = SameDomainPairWork::new(pair, 1.0e-5, range);
    assert!(!task.options.run_parallel());
    assert_eq!(task.options.fuzzy_value().to_bits(), 1.0e-5_f64.to_bits());
    assert!(!task.same_domain);
}

#[test]
fn consumed_pair_range_is_an_owned_misuse_error_not_a_geometry_failure_claim() {
    let fixture = same_domain_fixture(Kind::Cylinder);
    let pair = (
        fixture.sources[0].clone().into_shape(),
        fixture.sources[1].clone().into_shape(),
    );
    let mut task = SameDomainPairWork::new(pair, 1.0e-7, ProgressRange::default());
    drop(task.take_range().unwrap());
    let error = task
        .perform(&fixture.builder.context, &ProgressCancellation::default())
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert!(error.message().contains("progress=None"));
}

#[test]
fn cancelled_pair_uses_own_report_and_retains_ordinary_false_flag() {
    let fixture = same_domain_fixture(Kind::Cylinder);
    let pair = (
        fixture.sources[0].clone().into_shape(),
        fixture.sources[1].clone().into_shape(),
    );
    let (progress, _) = pulse_indicator(1);
    let observed = ProgressCancellation::default();
    let mut task = SameDomainPairWork::new(pair, 1.0e-7, progress.start());
    task.perform(&fixture.builder.context, &observed).unwrap();
    assert!(observed.is_cancelled());
    assert!(task.options.has_errors());
    assert!(!task.same_domain);
    assert!(!fixture.builder.options().has_errors());
}
