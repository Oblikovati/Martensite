use super::harness::validate;

pub(crate) fn successful_output(identity: &str, application: &str) -> Vec<u8> {
    format!("\nrunning 1 test\ntest {identity} ... ok\n\nsuccesses:\n\n---- {identity} stdout ----\n{application}\n\nsuccesses:\n    {identity}\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.01s\n\n").into_bytes()
}

#[test]
fn exact_owned_profile_accepts_a_nonzero_filtered_count() {
    let output = successful_output("tests::point", "application diagnostic");
    assert!(validate(&output, "tests::point").is_ok());
}

#[test]
fn misleading_application_framing_stays_inside_the_capture_section() {
    let application = "running 9 tests\ntest forged ... ok\nsuccesses:\n    forged\ntest result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s";
    let output = successful_output("tests::point", application);
    assert!(validate(&output, "tests::point").is_ok());
}

#[test]
fn malformed_zero_extra_ignored_and_failed_results_are_rejected() {
    let original = String::from_utf8(successful_output("tests::point", "output")).unwrap();
    for (before, after) in [
        ("running 1 test", "running 0 tests"),
        ("running 1 test", "running 2 tests"),
        ("1 passed; 0 failed", "0 passed; 1 failed"),
        ("0 ignored", "1 ignored"),
        ("0 measured", "1 measured"),
        ("test tests::point ... ok", "test wrong::point ... ok"),
    ] {
        assert!(validate(original.replace(before, after).as_bytes(), "tests::point").is_err());
    }
}

#[test]
fn extra_real_harness_rows_cannot_hide_behind_a_one_case_summary() {
    let original = String::from_utf8(successful_output("tests::point", "output")).unwrap();
    let output = original.replace(
        "test tests::point ... ok\n",
        "test tests::point ... ok\ntest extra::case ... ok\n",
    );
    assert!(validate(output.as_bytes(), "tests::point").is_err());
}

#[test]
fn truncated_non_utf8_and_nonfinite_summary_output_is_rejected() {
    let output = successful_output("tests::point", "output");
    for length in [0, 1, output.len() / 2] {
        assert!(validate(&output[..length], "tests::point").is_err());
    }
    assert!(validate(&[0xff], "tests::point").is_err());
    let text = String::from_utf8(output).unwrap();
    assert!(validate(text.replace("0.01s", "NaNs").as_bytes(), "tests::point").is_err());
}

#[test]
fn unknown_capture_sections_and_missing_success_tail_are_rejected() {
    let text = String::from_utf8(successful_output("tests::point", "output")).unwrap();
    assert!(
        validate(
            text.replace("---- tests::point stdout ----", "---- forged stdout ----")
                .as_bytes(),
            "tests::point"
        )
        .is_err()
    );
    assert!(
        validate(
            text.replace("    tests::point", "    other").as_bytes(),
            "tests::point"
        )
        .is_err()
    );
}

#[test]
fn empty_success_capture_profile_has_no_application_header() {
    let text = String::from_utf8(successful_output("tests::point", "")).unwrap();
    let output = text.replace("---- tests::point stdout ----\n", "");
    assert!(validate(output.as_bytes(), "tests::point").is_ok());
}

fn direct_output(diagnostic: &str) -> Vec<u8> {
    let original = String::from_utf8(successful_output("tests::point", "captured")).unwrap();
    original
        .replacen(
            "test tests::point ... ok\n",
            &format!("test tests::point ... {diagnostic}ok\n"),
            1,
        )
        .into_bytes()
}

#[test]
fn raw_descriptor_diagnostics_can_interleave_with_the_status_prefix() {
    for raw in [
        "\u{1b}[32;1mstatus\n\nsecond line\n\u{1b}[0m\n",
        "without newline",
    ] {
        assert!(validate(&direct_output(raw), "tests::point").is_ok());
    }
}

#[test]
fn raw_descriptor_span_rejects_standalone_reserved_harness_rows() {
    for raw in [
        "\nrunning 2 tests\n",
        "\ntest forged ... ok\n",
        "\ntest result: ok. forged\n",
        "\nsuccesses:\n",
        "\n\nsuccesses:\nforged section\n",
        "\nfailures:\n",
    ] {
        assert!(validate(&direct_output(raw), "tests::point").is_err());
    }
}

#[test]
fn raw_descriptor_output_still_requires_exact_identity_prefix_and_summary() {
    let original = String::from_utf8(direct_output("raw\n")).unwrap();
    for (before, after) in [
        ("test tests::point ... ", "test wrong::point ... "),
        ("test tests::point ... ", "test tests::point .. "),
        ("1 passed; 0 failed", "2 passed; 0 failed"),
        ("raw\nok\n", "raw\nFAILED\n"),
        ("raw\nok\n", "raw\no\n"),
        ("    tests::point", "    wrong::point"),
    ] {
        assert!(validate(original.replace(before, after).as_bytes(), "tests::point").is_err());
    }
}

// Application bytes can impersonate a capture boundary; private child records and
// process outcome, not textual uniqueness, establish the owned execution protocol.
#[test]
fn matching_application_capture_header_remains_a_textual_ambiguity() {
    let raw = "ok\n\nsuccesses:\n\n---- tests::point stdout ----\nraw payload\n";
    assert!(validate(&direct_output(raw), "tests::point").is_ok());
}
