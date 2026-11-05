//! A filtered candidate must not observe a break caused by early scope retirement.
use super::contract::planar_vertex_face_fixture;
use crate::bop_algo::{AlertUserBreak, BOPAlgoPaveFiller, ProgressCancellation};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope},
    standard::RuntimeTyped,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[derive(Clone, Default)]
struct ScopeRetirementBreak {
    retired: Arc<AtomicBool>,
    polls: Arc<AtomicUsize>,
}

impl ProgressIndicatorHooks for ScopeRetirementBreak {
    fn user_break(&self) -> bool {
        self.polls.fetch_add(1, Ordering::SeqCst);
        self.retired.load(Ordering::SeqCst)
    }

    fn show(&self, _: &ProgressScope<'_>, position: f64, _: bool) {
        if position >= 0.9 {
            self.retired.store(true, Ordering::SeqCst);
        }
    }
}

#[test]
fn filtered_vertex_face_scope_retires_after_final_cancellation_poll() {
    for parallel in [false, true] {
        let mut filler = planar_vertex_face_fixture(1, parallel);
        filler.perform_vf().unwrap();
        assert_eq!(
            filler.data_structure().unwrap().interf_vf_records().len(),
            1
        );
        assert_filtered_scope_retirement(&mut filler);
    }
}

fn assert_filtered_scope_retirement(filler: &mut BOPAlgoPaveFiller) {
    let hook = ScopeRetirementBreak::default();
    let indicator = ProgressIndicator::new(Box::new(hook.clone()));
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let outcomes = filler.perform_vf_with_progress(&range, &observed).unwrap();
    assert!(outcomes.is_empty());
    assert_eq!(hook.polls.load(Ordering::SeqCst), 2);
    assert!(hook.retired.load(Ordering::SeqCst));
    assert!(!observed.is_cancelled());
    assert!(!filler.options().has_error(AlertUserBreak::static_type()));
    assert_eq!(
        filler.data_structure().unwrap().interf_vf_records().len(),
        1
    );
}
