// SPDX-License-Identifier: GPL-2.0-only
//! A refusal retains observed work without treating an unstarted census as completed.
use super::tests::Profile;
use super::*;
use martensite_tkernel::standard::Failure;

struct StopAtBoundary {
    after_census: bool,
}
impl OwnedPrismProgress for StopAtBoundary {
    fn checkpoint(&mut self, point: OwnedPrismCheckpoint) -> Result<bool, Failure> {
        Ok(if self.after_census {
            matches!(point.phase, OwnedPrismPhase::Partition(_))
        } else {
            point.phase == OwnedPrismPhase::Construction
        })
    }
}
fn cancelled_preparation(after_census: bool) -> Box<ProfilePreparationFailure> {
    let mut profile = Profile::default();
    profile.polygon(1, &[[0., 0.], [2., 0.], [2., 2.], [0., 2.]]);
    profile.select(1, [1., 1.]);
    prepare_owned_profile(profile.request(), &mut StopAtBoundary { after_census })
        .err()
        .unwrap()
}
#[test]
fn cancellation_before_census_has_no_observed_pair_receipt() {
    let failure = cancelled_preparation(false);
    assert_eq!(failure.class, OwnedPreparationClass::Cancelled);
    assert_eq!(failure.phase, OwnedPrismPhase::Construction);
    assert!(failure.pair_work.is_none());
}
#[test]
fn cancellation_after_census_retains_its_completed_work() {
    let failure = cancelled_preparation(true);
    assert_eq!(failure.class, OwnedPreparationClass::Cancelled);
    assert!(matches!(failure.phase, OwnedPrismPhase::Partition(_)));
    let work = failure.pair_work.unwrap();
    assert_eq!(work.requested_pair_bisections, 4096);
    assert_eq!(work.effective_pair_bisections, 4096);
    assert_eq!(work.consumed_pair_bisections, 0);
}
