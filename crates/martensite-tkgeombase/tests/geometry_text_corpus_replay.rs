//! All text/state requests run in their own process, including extension-hook controls.
#[path = "support/construction_corpus/mod.rs"]
mod construction_corpus;
#[path = "support/geometry_text_corpus/independent.rs"]
mod independent;
use construction_corpus::run_values;

const REQUESTS: &str = include_str!("fixtures/geometry_text_corpus/requests.txt");
const EXPECTED: &str = include_str!("fixtures/geometry_text_corpus/expected.txt");
const FAMILIES: &[(&str, usize)] = &[
    ("gt_curve", 31),
    ("gt_curve2d", 22),
    ("gt_surface", 27),
    ("gt_read", 148),
    ("gt_get_real", 9),
    ("gt_set", 4),
    ("gt_set_null", 1),
    ("gt_set_read", 16),
    ("gt_progress", 15),
    ("gt_debug_dump", 14),
    ("gt_handler", 9),
];

fn execute(request: &str) -> String {
    let mut fields = request.split_whitespace();
    let operation = fields.next().unwrap();
    let values: Vec<f64> = fields.map(|value| value.parse().unwrap()).collect();
    run_values(operation, &values).expect("owned text family")
}

fn worker_rows(rows: &[&str], worker: usize, workers: usize) -> Vec<(usize, String)> {
    rows.iter()
        .enumerate()
        .skip(worker)
        .step_by(workers)
        .map(|(index, request)| (index, execute(request)))
        .collect()
}

fn replay(workers: usize) -> String {
    let rows: Vec<&str> = REQUESTS.lines().collect();
    let mut results = std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..workers)
            .map(|worker| {
                let rows = &rows;
                scope.spawn(move || worker_rows(rows, worker, workers))
            })
            .collect();
        jobs.into_iter()
            .flat_map(|job| job.join().unwrap())
            .collect::<Vec<_>>()
    });
    results.sort_by_key(|(index, _)| *index);
    results.into_iter().map(|(_, output)| output).collect()
}

fn check_family_counts() {
    for &(family, count) in FAMILIES {
        let actual = REQUESTS
            .lines()
            .filter(|row| row.split_whitespace().next() == Some(family))
            .count();
        assert_eq!(actual, count);
    }
}

#[test]
fn all_296_requests_and_independent_text_state_controls() {
    assert_eq!(REQUESTS.lines().count(), 296);
    assert_eq!(EXPECTED.lines().count(), 296);
    check_family_counts();
    assert_eq!(replay(1), EXPECTED);
    assert_eq!(replay(4), EXPECTED);
    independent::line_roundtrip_and_precision();
    independent::circle_roundtrip_and_remainder();
    independent::failed_read_and_console_isolation();
    independent::handler_scope_restores_during_unwind();
}
