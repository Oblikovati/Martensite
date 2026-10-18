//! retained Rust retention of the complete sealed geometry numeric corpus.
mod comparison;
mod fixture_decoder;
mod operation_counts;
mod operations;
mod request_context;
use fixture_decoder::FixtureReader;
use std::collections::BTreeMap;

fn execute_request(request: &str) -> String {
    let mut fields = request.split_whitespace();
    let operation = fields
        .next()
        .expect("empty request; expected an operation name");
    let values: Vec<f64> = fields
        .map(|field| {
            field
                .parse()
                .unwrap_or_else(|_| panic!("field={field:?}; expected binary64 numeric text"))
        })
        .collect();
    operations::dispatch::execute(operation, &values)
        .unwrap_or_else(|| panic!("operation={operation:?}; expected a registered geometry case"))
}

fn replay_lines(
    requests: &mut FixtureReader<'_>,
    expected: &mut FixtureReader<'_>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    while let Some(request) = requests.next_line().unwrap() {
        let reference = expected
            .next_line()
            .unwrap()
            .expect("missing receipt; expected one per request");
        comparison::assert_receipt(&request, &reference, &execute_request(&request));
        let operation = request.split_whitespace().next().unwrap().to_owned();
        *counts.entry(operation).or_default() += 1;
    }
    assert!(
        expected.next_line().unwrap().is_none(),
        "extra receipt; expected exact request count"
    );
    counts
}

#[test]
fn version_one_numeric_corpus_replays_all_requests_in_process() {
    let mut requests = FixtureReader::new(include_bytes!("fixtures/requests.txt.gz"));
    let mut expected = FixtureReader::new(include_bytes!("fixtures/expected.txt.gz"));
    let counts = replay_lines(&mut requests, &mut expected);
    assert_eq!(counts.values().sum::<usize>(), 4695);
    assert_eq!(counts.len(), 71);
    for (operation, count) in operation_counts::REQUIRED_COUNTS {
        assert_eq!(
            counts.get(*operation),
            Some(count),
            "family={operation}; expected original request count"
        );
    }
}

#[test]
fn unknown_operations_are_rejected_and_request_state_is_local() {
    assert!(operations::dispatch::execute("missing", &[]).is_none());
    let first = operations::output::Output::captured();
    let second = operations::output::Output::captured();
    first.set_parameters(vec![1., 2.]);
    first.emit(format_args!("first"));
    assert_eq!(first.parameters(), [1., 2.]);
    assert!(second.parameters().is_empty());
    assert_eq!(second.finish(), "");
    assert_eq!(first.finish(), "first");
    assert!(first.finish().is_empty());
}
