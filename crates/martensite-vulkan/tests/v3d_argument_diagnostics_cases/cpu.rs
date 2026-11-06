use super::*;

#[test]
fn both_source_phrase_predicates_preserve_all_four_authored_messages() {
    let faulty = format!("{FAULTY_PHRASE}.\n");
    let valid = crate::case_argument_response::VALID_ARGUMENT_RESPONSE;
    assert_eq!(diagnostic_message(true, &faulty), "OK : message is correct");
    assert_eq!(
        diagnostic_message(true, valid),
        "Error : message is not correct"
    );
    assert_eq!(diagnostic_message(false, &faulty), "Error : Wrong result");
    assert_eq!(diagnostic_message(false, valid), "OK : Good result");
}

#[test]
fn actual_default_analyzers_classify_both_case_fixture_arguments() {
    for expected_faulty in [true, false] {
        let (_, fixture, _) = case_case(expected_faulty);
        let shape = read_dataset_shape(fixture).unwrap();
        let response = default_argument_response(&shape);
        assert_eq!(
            response.contains(FAULTY_PHRASE),
            expected_faulty,
            "fixture {fixture}; actual response {response:?}"
        );
    }
}
