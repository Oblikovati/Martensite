//! One canonical request parser, comparison policy and fixture byte lock for every retained replay.
pub fn checked_request(request: &str, expected: &str, family: &str) -> Option<String> {
    let mut fields = request.split_whitespace();
    if fields.next() != Some(family) {
        return None;
    }
    let values: Vec<f64> = fields.map(|field| field.parse().unwrap()).collect();
    let actual = super::run_values(family, &values).expect("owned family");
    assert!(
        super::policy::equivalent(expected, &actual),
        "{request}: {}",
        super::policy::difference(expected, &actual)
    );
    Some(actual)
}
pub fn replay_request(request: &str, expected: &str, family: &str) -> bool {
    checked_request(request, expected, family).is_some()
}
pub fn replay_family(chunks: &[(&str, &str)], family: &str, expected_count: usize) {
    let mut count = 0;
    for (requests, answers) in chunks {
        assert_eq!(requests.lines().count(), answers.lines().count());
        for (request, expected) in requests.lines().zip(answers.split_inclusive('\n')) {
            count += usize::from(replay_request(request, expected, family));
        }
    }
    assert_eq!(
        count, expected_count,
        "Every original {family} request must run"
    );
}
pub fn fixture_hash<'text>(parts: impl Iterator<Item = &'text str>) -> u64 {
    parts
        .flat_map(str::bytes)
        .fold(0xcbf29ce484222325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn matching_rows_execute_and_other_families_leave_the_count_unchanged() {
        let chunks = [("gce_status\n", "0 1 2 3 4 5 6 7 8 9 10 11 12 13 \n")];
        replay_family(&chunks, "gce_status", 1);
        replay_family(&chunks, "different-family", 0);
        assert!(checked_request("gce_status", "unused", "different-family").is_none());
    }
    #[test]
    fn byte_lock_handles_empty_and_single_byte_parts() {
        assert_eq!(fixture_hash(std::iter::empty()), 0xcbf29ce484222325);
        assert_eq!(fixture_hash(["a"].into_iter()), 0xaf63dc4c8601ec8c);
    }
}
