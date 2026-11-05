//! Empty worker batches must not publish progress before the real post-join poll.
use super::*;
use crate::bop_algo::pave_filler::{
    face_face_task_tests::prepared_plane_input, owned_progress_fixture::progress_scope_ancestry,
};
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks};
use std::sync::{Arc, Mutex, OnceLock, Weak};

type ScopeBits = Vec<(String, u64, u64)>;
type ScopeObservation = (u64, ScopeBits);

#[derive(Default)]
struct EmptyFaceJournal {
    events: Mutex<Vec<ScopeObservation>>,
    polls: Mutex<Vec<(u64, usize)>>,
}

struct EmptyFacePositionHooks {
    indicator: Arc<OnceLock<Weak<ProgressIndicator>>>,
    journal: Arc<EmptyFaceJournal>,
    cancel_at: f64,
}

impl ProgressIndicatorHooks for EmptyFacePositionHooks {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let chain = progress_scope_ancestry(scope)
            .into_iter()
            .map(|(name, maximum, value)| (name, maximum.to_bits(), value.to_bits()))
            .collect();
        self.journal
            .events
            .lock()
            .unwrap()
            .push((position.to_bits(), chain));
    }

    fn user_break(&self) -> bool {
        let indicator = self.indicator.get().unwrap().upgrade().unwrap();
        let position = indicator.get_position();
        let shows = self.journal.events.lock().unwrap().len();
        self.journal
            .polls
            .lock()
            .unwrap()
            .push((position.to_bits(), shows));
        // Read the actual SDK indicator, not a synthetic counter or cached Show position.
        position >= self.cancel_at
    }
}

#[test]
fn zero_face_workers_close_the_named_scope_after_the_parent_poll() {
    for glue in [Glue::Off, Glue::Full] {
        for parallel in [false, true] {
            assert_empty_face_lifetime(glue, parallel, f64::INFINITY);
        }
    }
}

#[test]
fn zero_face_workers_do_not_cancel_from_a_premature_position_increment() {
    for glue in [Glue::Off, Glue::Full] {
        for parallel in [false, true] {
            assert_empty_face_lifetime(glue, parallel, 1.0);
        }
    }
}

fn assert_empty_face_lifetime(glue: Glue, parallel: bool, cancel_at: f64) {
    let mut filler = empty_face_worker_receiver(glue, parallel);
    let (indicator, journal) = empty_face_indicator(cancel_at);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let completions = filler.perform_ff_with_progress(&range, &observed).unwrap();
    assert!(completions.is_empty());
    assert!(!observed.is_cancelled());
    assert!(!filler.options().has_errors());
    assert_eq!(
        filler.data_structure().unwrap().interf_ff_records().len(),
        1
    );
    assert_empty_face_observations(&journal);
    assert_eq!(indicator.get_position().to_bits(), 1.0_f64.to_bits());
}

fn empty_face_worker_receiver(glue: Glue, parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    for face in prepared_plane_input().faces {
        filler.add_argument(&face.into_shape());
    }
    filler.set_glue(glue);
    filler.options_mut().set_run_parallel(parallel);
    filler.initialize().unwrap();
    filler
        .initialize_progress_pairs(ShapeEnum::Face, ShapeEnum::Face)
        .unwrap();
    assert_eq!(filler.progress_pair_cursor().unwrap().expected_length(), 1);
    filler
}

fn empty_face_indicator(cancel_at: f64) -> (Arc<ProgressIndicator>, Arc<EmptyFaceJournal>) {
    let indicator = Arc::new(OnceLock::new());
    let journal = Arc::new(EmptyFaceJournal::default());
    let hooks = EmptyFacePositionHooks {
        indicator: indicator.clone(),
        journal: journal.clone(),
        cancel_at,
    };
    let owner = Arc::new(ProgressIndicator::new(Box::new(hooks)));
    indicator.set(Arc::downgrade(&owner)).unwrap();
    (owner, journal)
}

fn assert_empty_face_observations(journal: &EmptyFaceJournal) {
    let initial = vec![("<null>".to_owned(), 1.0_f64.to_bits(), 0.0_f64.to_bits())];
    let expected = vec![
        (0.0_f64.to_bits(), initial),
        (1.0_f64.to_bits(), closed_face_chain()),
    ];
    assert_eq!(*journal.events.lock().unwrap(), expected);
    // Native _6 polls while its zero-task named scope is still active at position zero.
    assert_eq!(
        *journal.polls.lock().unwrap(),
        vec![(0.0_f64.to_bits(), 1); 2]
    );
}

fn closed_face_chain() -> ScopeBits {
    vec![
        (
            "Performing Face-Face intersection".to_owned(),
            1.0e-6_f64.to_bits(),
            1.0e-6_f64.to_bits(),
        ),
        ("<null>".to_owned(), 1.0_f64.to_bits(), 1.0_f64.to_bits()),
        ("<null>".to_owned(), 1.0_f64.to_bits(), 1.0_f64.to_bits()),
    ]
}
