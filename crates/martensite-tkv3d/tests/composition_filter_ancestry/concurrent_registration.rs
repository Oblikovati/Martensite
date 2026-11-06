//! Released descriptor registration must stay canonical when new parent views are first queried together.
use super::native_classes;
use martensite_tkernel::standard::Handle;
use martensite_tkv3d::select_mgr::SelectMgrCompositionFilter;
use std::sync::{Arc, Barrier};

fn projected_descriptors() -> [usize; 3] {
    native_classes().map(|owner| {
        let parent: Handle<dyn SelectMgrCompositionFilter> = owner.cast();
        assert!(!parent.is_null());
        assert_eq!(owner.object_address(), parent.object_address());
        owner.dynamic_type().unwrap() as *const _ as usize
    })
}

fn registration_worker(barrier: Arc<Barrier>) -> Vec<[usize; 3]> {
    barrier.wait();
    (0..64).map(|_| projected_descriptors()).collect()
}

fn registration_workers() -> Vec<Vec<[usize; 3]>> {
    let barrier = Arc::new(Barrier::new(8));
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                let barrier = barrier.clone();
                scope.spawn(move || registration_worker(barrier))
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect()
    })
}

#[test]
fn concurrent_parent_queries_retain_one_descriptor_and_actual_owner() {
    let observations = registration_workers();
    let expected = observations[0][0];
    for worker in observations {
        assert!(worker.into_iter().all(|actual| actual == expected));
    }
}
