//! Authored identities determine actual proof scheduling, including the work receipt (#453).
use super::super::selection::qualify;
use super::*;

fn requests() -> Vec<Request> {
    let mut requests = super::super::family::tests::requests();
    requests[0].loops = vec![curved_cap()];
    requests[1].loops = vec![triangle()];
    requests
}
fn curved_cap() -> Vec<AuthoredPiece> {
    super::super::tests::contour(
        10,
        &[
            &[[0., 0.], [1., 0.]],
            &[[1., 0.], [2., 0.]],
            &[[2., 0.], [1., 1.], [0., 0.]],
        ],
    )
}
fn triangle() -> Vec<AuthoredPiece> {
    super::super::tests::contour(
        1000,
        &[
            &[[4., 0.25], [5., 0.25]],
            &[[5., 0.25], [4., 1.25]],
            &[[4., 1.25], [4., 0.25]],
        ],
    )
}
#[test]
fn two_disjoint_profiles_replay_the_actual_planar_work_receipt() {
    let mut requests = requests();
    let forward = qualify(&requests).unwrap();
    requests.reverse();
    let reverse = qualify(&requests).unwrap();
    assert_eq!(forward.proof_work, reverse.proof_work);
    assert!(forward.proof_work.consumed_pair_bisections > 0);
    assert!(forward.members[&SurfaceRegionId(5)].area.encloses(2. / 3.));
    assert!(reverse.members[&SurfaceRegionId(100)].area.encloses(0.5));
}
#[test]
fn completed_proof_input_keeps_all_authored_data_under_reordering() {
    let mut requests = requests();
    let forward = combined_profile(&requests).unwrap();
    requests.reverse();
    for request in &mut requests {
        request.loops[0].rotate_left(1);
    }
    let reverse = combined_profile(&requests).unwrap();
    assert_eq!(forward, reverse);
}
#[test]
fn concurrent_planar_calls_keep_identical_actual_receipts() {
    let mut requests = requests();
    let forward = qualify(&requests).unwrap().proof_work;
    requests.reverse();
    std::thread::scope(|scope| {
        let worker = scope.spawn(|| qualify(&requests).unwrap().proof_work);
        assert_eq!(forward, qualify(&requests).unwrap().proof_work);
        assert_eq!(forward, worker.join().unwrap());
    });
}
#[test]
fn zero_cap_refusal_keeps_actual_work_and_the_same_authored_pair() {
    let mut requests = requests();
    let forward = exhausted(&requests);
    requests.reverse();
    let reverse = exhausted(&requests);
    assert_eq!(forward.proof_work(), reverse.proof_work());
    assert_eq!(forward.proof_work().unwrap().consumed_pair_bisections, 0);
    assert_eq!(forward.pair_root_failure(), reverse.pair_root_failure());
    assert!(forward.pair_root_failure().is_some());
}
fn exhausted(requests: &[Request]) -> Box<PlanarRegionWorkFailure> {
    let result = capture_with_work(requests, 1e-5, PlanarProofWorkLimit::PairBisections(0));
    let Err(PlanarFailure::WorkCapture(failure)) = result else {
        panic!("expected retained zero-cap root refusal, with no partial geometry");
    };
    failure
}
#[test]
fn invalid_source_precedence_remains_in_original_caller_order() {
    let mut requests = requests();
    requests.reverse();
    for request in &mut requests {
        request.loops[0][0].span.level = 1;
    }
    assert!(matches!(
        combined_profile(&requests),
        Err(PlanarFailure::SplitSource {
            curve: SurfaceCurveId(1000),
            level: 1,
            ..
        })
    ));
}
