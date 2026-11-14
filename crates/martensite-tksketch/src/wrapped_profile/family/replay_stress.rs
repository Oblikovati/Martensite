//! Same authored keys across different operations must not share geometry or proof state.
use super::*;
const WORKERS: usize = 4;
const CASES: usize = 3;

#[test]
#[ignore = "qualification stress campaign; run explicitly before publication"]
fn different_geometry_with_shared_authored_ids_replays_under_contention() {
    let expected: Vec<_> = (0..CASES).map(|case| case_signature(case, false)).collect();
    assert_distinct(&expected);
    let barrier = std::sync::Barrier::new(WORKERS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..WORKERS)
            .map(|worker| {
                let (expected, barrier) = (&expected, &barrier);
                scope.spawn(move || exercise(worker, expected, barrier))
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert_sequential(&expected);
}

fn exercise(worker: usize, expected: &[String], barrier: &std::sync::Barrier) {
    barrier.wait();
    for iteration in 0..CASES {
        let case = (worker + iteration) % CASES;
        let actual = case_signature(case, (worker + iteration) % 2 == 1);
        assert!(
            actual == expected[case],
            "stress replay differs: worker={worker}, iteration={iteration}, case={case}"
        );
    }
}

fn case_signature(case: usize, reverse: bool) -> String {
    let mut inputs = case_requests(case);
    if reverse {
        inputs.reverse();
    }
    let result = construct_budgeted(&inputs, CylindricalMeasureLimit::Report)
        .unwrap_or_else(|cause| panic!("stress case={case}: {cause}"));
    check_authored_volume(&inputs, &result);
    signature_of(&result)
}

fn case_requests(case: usize) -> Vec<Request> {
    let (radius, depths, translation) = [
        (10., [-1., 2.], 0.),
        (12., [-0.5, 1.], 4.),
        (8., [-2., 0.5], -4.),
    ][case];
    let mut inputs = requests();
    for request in &mut inputs {
        request.radius = radius;
        request.depths = depths;
        translate_height(request, translation);
    }
    inputs
}

fn translate_height(request: &mut Request, translation: f64) {
    for pole in request
        .loops
        .iter_mut()
        .flatten()
        .flat_map(|p| &mut p.poles)
    {
        pole[1] += translation;
    }
}

fn check_authored_volume(inputs: &[Request], result: &BudgetedFamily) {
    for request in inputs {
        let area = match request.region {
            SurfaceRegionId(5) => 3.525,
            SurfaceRegionId(100) => 1.,
            other => panic!("stress region={other:?}; expected 5 or 100"),
        };
        let [first, last] = request.depths;
        let expected = area * (last - first) * (1. + (last + first) / (2. * request.radius));
        let member = &result.family.members[&request.region];
        let [lo, hi] = member
            .measures
            .comparison
            .normal_certificate()
            .authored_volume_enclosure();
        assert!(lo <= hi && (lo - expected).abs().max((hi - expected).abs()) < 1e-12);
    }
}

fn assert_distinct(expected: &[String]) {
    for (index, signature) in expected.iter().enumerate() {
        assert!(expected[..index].iter().all(|other| other != signature));
    }
}

fn assert_sequential(expected: &[String]) {
    for (case, signature) in expected.iter().enumerate().rev() {
        assert!(
            case_signature(case, true) == *signature,
            "sequential stress replay changed after contention: case={case}"
        );
    }
}
