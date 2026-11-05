//! Actual workers consume one range each and publish terminals only after joining.
use super::super::{connections::FeatureIncidences, fixture::parent_fixture};
use super::*;
#[test]
fn disjoint_gap_tasks_keep_feature_order_and_owned_histories_on_both_executors() {
    for parallel in [false, true] {
        let remover = parent_fixture(4, parallel);
        let incidences = FeatureIncidences::new(&remover.input_shape).unwrap();
        let range = ProgressRange::default();
        let scope = ProgressScope::unnamed(&range, 2., false);
        let (gaps, completion) = fill_feature_gaps(
            remover.new_feature_gaps(),
            &incidences.connections(),
            &scope,
            parallel,
        )
        .unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        assert_eq!(gaps.len(), 2);
        for (gap, source) in gaps.iter().zip(&remover.features) {
            assert!(gap.feature.is_same(source));
            assert_eq!(gap.feature_faces.len(), 6);
            assert_eq!(gap.solids.extent(), 1);
            assert!(!gap.has_adjacent_faces);
            assert!(gap.history.get().is_some());
        }
    }
}
#[test]
fn gap_task_requires_one_range_and_one_terminal_result_with_its_original_ordinal() {
    let remover = parent_fixture(2, false);
    let incidences = FeatureIncidences::new(&remover.input_shape).unwrap();
    let mut task = FeatureGapTask {
        ordinal: 9,
        gap: remover.new_feature_gaps().remove(0),
        range: None,
        completion: None,
    };
    let cause = task.perform(&incidences.connections()).unwrap_err();
    assert_eq!(cause.exception_type(), "Standard_ProgramError");
    assert!(
        cause
            .message()
            .contains("task=9; expected one unused range")
    );
    let cause = collect_feature_gaps(vec![task]).err().unwrap();
    assert!(
        cause
            .message()
            .contains("task=9; expected one terminal worker outcome")
    );
}
#[test]
fn a_gap_task_is_send_and_its_range_is_not_shared_for_mutation() {
    static_assertions::assert_impl_all!(FeatureGapTask<'static>: Send);
    static_assertions::assert_not_impl_any!(FeatureGapTask<'static>: Sync);
}
