//! Collect every matrix mismatch instead of hiding later cases after a first failure.
use martensite_tkernel::standard::Failure;

pub(super) fn record_failure(
    failures: &mut Vec<String>,
    result: Result<(), Failure>,
    expected: (&str, &str),
) -> Option<Failure> {
    let Some(failure) = result.err() else {
        failures.push(format!(
            "{}: expected {}, operation succeeded",
            expected.1, expected.0
        ));
        return None;
    };
    record_mismatch(failures, &failure, expected);
    Some(failure)
}

fn record_mismatch(failures: &mut Vec<String>, failure: &Failure, expected: (&str, &str)) {
    if failure.exception_type() == expected.0 && failure.message() == expected.1 {
        return;
    }
    failures.push(format!(
        "{}: got {} {:?}",
        expected.1,
        failure.exception_type(),
        failure.message()
    ));
}
