//! Workers share one context (#496), so lookups must agree and must not wait on other builds.
use super::ShapeCache;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{ncollection::BaseAllocator, standard::Failure};
use std::sync::{
    Arc, Barrier,
    atomic::{AtomicUsize, Ordering},
    mpsc,
};
use std::time::Duration;

fn probe_shape(height: f64) -> Shape {
    martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge::from_points(
        martensite_tkmath::gp::Point3::default(),
        martensite_tkmath::gp::Point3::new(0., 0., height),
    )
    .unwrap()
    .edge()
    .unwrap()
    .into()
}

#[test]
fn concurrent_lookups_of_one_shape_share_the_first_inserted_service() {
    let cache = ShapeCache::<usize>::new(&BaseAllocator::common());
    let shape = probe_shape(1.);
    let builds = AtomicUsize::new(0);
    let start = Barrier::new(4);
    let services: Vec<_> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4)
            .map(|_| {
                scope.spawn(|| {
                    start.wait();
                    cache.get(&shape, || Ok(builds.fetch_add(1, Ordering::SeqCst)))
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap().unwrap())
            .collect()
    });
    assert!(builds.load(Ordering::SeqCst) >= 1);
    let first = &services[0];
    assert!(services.iter().all(|service| Arc::ptr_eq(service, first)));
    let again = cache
        .get(&shape, || panic!("a cached shape must not be built again"))
        .unwrap();
    assert!(Arc::ptr_eq(&again, first));
}

#[test]
fn a_slow_build_does_not_block_lookups_of_other_shapes() {
    let cache = ShapeCache::<usize>::new(&BaseAllocator::common());
    let (slow, other) = (probe_shape(1.), probe_shape(2.));
    let (finished, waiting) = mpsc::channel::<()>();
    let (cache, slow) = (&cache, &slow);
    std::thread::scope(|scope| {
        let builder = scope.spawn(move || {
            cache.get(slow, || {
                // Under the old map lock the other lookup could not finish before this one.
                waiting
                    .recv_timeout(Duration::from_secs(30))
                    .map_err(|_| Failure::new("other lookup did not finish during this build"))?;
                Ok(1)
            })
        });
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(*cache.get(&other, || Ok(2)).unwrap().read(), 2);
        finished.send(()).unwrap();
        assert_eq!(*builder.join().unwrap().unwrap().read(), 1);
    });
}

#[test]
fn a_failed_build_caches_nothing_and_returns_its_failure() {
    let cache = ShapeCache::<usize>::new(&BaseAllocator::common());
    let shape = probe_shape(1.);
    let failure = cache
        .get(&shape, || Err(Failure::new("build refused")))
        .err()
        .unwrap();
    assert!(failure.message().contains("build refused"));
    assert_eq!(*cache.get(&shape, || Ok(5)).unwrap().read(), 5);
}
