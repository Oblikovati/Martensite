use super::references as visit_references;
use crate::pcdm::PcdmReference;
use martensite_tkernel::{message::Gravity, tcollection::ExtendedString};
#[path = "../../tests/header_counters/printer.rs"]
mod printer;

fn references(
    file: &ExtendedString,
    output: &mut Vec<PcdmReference>,
    messenger: Option<&std::sync::Arc<martensite_tkernel::message::Messenger>>,
) -> Result<(), martensite_tkernel::standard::Failure> {
    visit_references(file, messenger, |reference| output.push(reference))
}

fn fixture(kind: &str, scenario: usize) -> ExtendedString {
    let path = format!(
        "{}/tests/header_references/fixtures/{kind}-{scenario}.fsd",
        env!("CARGO_MANIFEST_DIR")
    );
    ExtendedString::from_c_string(path.as_bytes(), true)
}
fn expected(scenario: usize) -> Vec<(i32, i32, String)> {
    // The Windows reference syntax spells every separator as '/', including the
    // document folder's own.
    let manifest = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
    let base = format!("{manifest}/tests/header_references");
    let folder = format!("{base}/fixtures");
    match scenario {
        0 => vec![
            (1, 5, format!("{folder}/child.std")),
            (2, 7, format!("{base}/sibling.std")),
        ],
        1 => vec![
            (3, 9, "/already//absolute.std".into()),
            (4, 11, format!("{folder}/sub/part.std")),
            (5, 13, format!("{folder}/./dot.std")),
            (6, 15, format!("{folder}/..")),
        ],
        2 => vec![(7, 21, format!("{folder}/selected.std"))],
        3 | 4 => vec![],
        5 => vec![
            (12, 4, format!("{folder}/café.std")),
            (1, 3, format!("{folder}/prefix.std")),
        ],
        other => panic!("reference scenario {other}; expected 0..=5"),
    }
}
fn tuple(reference: &PcdmReference) -> (i32, i32, String) {
    (
        reference.reference_identifier(),
        reference.document_version(),
        reference.file_name().to_utf8(),
    )
}
fn warning_lines(scenario: usize, found: &[(i32, i32, String)]) -> Vec<String> {
    let mut messages = Vec::new();
    if scenario == 1 {
        messages.push("Warning: invalid document reference: invalid".into());
    }
    for (id, version, path) in found {
        messages.push(format!(
            "Warning: reference found; ReferenceIdentifier:  {id}; File:{path}, version:{version}"
        ));
    }
    if scenario == 5 {
        messages.push("Warning: invalid document reference: 13 4 ".into());
    }
    messages
}
fn check_fixture(kind: &str, scenario: usize) {
    let expected = expected(scenario);
    let (messenger, printer) = printer::recorder();
    let mut found = Vec::new();
    references(&fixture(kind, scenario), &mut found, Some(&messenger)).unwrap();
    assert_eq!(
        found.iter().map(tuple).collect::<Vec<_>>(),
        expected,
        "{kind}-{scenario}"
    );
    let actual = printer.messages.lock().unwrap();
    let expected_messages: Vec<(Vec<u8>, Gravity)> = warning_lines(scenario, &expected)
        .into_iter()
        .map(|line| (line.into_bytes(), Gravity::Warning))
        .collect();
    assert_eq!(*actual, expected_messages, "{kind}-{scenario}");
}

#[test]
fn all_frozen_reference_records_preserve_delimiters_paths_versions_and_warning_bytes() {
    for kind in ["cmp", "text", "binary"] {
        for scenario in 0..6 {
            check_fixture(kind, scenario);
        }
    }
}

#[test]
fn null_message_sink_preserves_reference_contents_and_ignored_invalid_records() {
    for kind in ["cmp", "text", "binary"] {
        for scenario in 0..6 {
            let mut found = Vec::new();
            references(&fixture(kind, scenario), &mut found, None).unwrap();
            assert_eq!(
                found.iter().map(tuple).collect::<Vec<_>>(),
                expected(scenario)
            );
        }
    }
}

#[test]
fn a_header_read_exception_propagates_without_mutating_existing_output_references() {
    let file = ExtendedString::from_c_string(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/header_references/fixtures/truncated-info.fsd"
        )
        .as_bytes(),
        true,
    );
    let mut found = vec![PcdmReference::new(41, &ExtendedString::new("sentinel"), 43)];
    let failure = references(&file, &mut found, None).unwrap_err();
    assert_eq!(failure.exception_type(), "Storage_StreamWriteError");
    assert_eq!(found.len(), 1);
    assert_eq!(tuple(&found[0]), (41, 43, "sentinel".into()));
}

#[test]
fn reopen_failure_propagates_the_native_open_message_without_leaking_a_driver() {
    use crate::pcdm::{
        file_header::select_driver,
        file_header_test_support::{FROZEN_FILES, FakeHeaderFiles},
    };
    let files = FakeHeaderFiles::new(FROZEN_FILES[0].1[0], Some(2));
    let driver = select_driver(
        &martensite_tkernel::tcollection::AsciiString::new("owned.fsd"),
        files.clone(),
    )
    .unwrap()
    .unwrap();
    let failure =
        super::records_from_driver(driver, &ExtendedString::new("owned.fsd")).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_Failure");
    assert_eq!(
        failure.message(),
        "could not open the file: owned.fsd; file was not found or permission denied"
    );
    assert_eq!(files.trace.opens.borrow().len(), 2);
    assert_eq!(files.trace.closed.get(), 1);
}

#[test]
fn header_failure_drops_the_open_driver_at_the_uncaught_error_boundary() {
    use crate::pcdm::{file_header::select_driver, file_header_test_support::FakeHeaderFiles};
    let bytes = include_bytes!("../../tests/header_references/fixtures/truncated-info.fsd");
    let files = FakeHeaderFiles::new(bytes, None);
    let driver = select_driver(
        &martensite_tkernel::tcollection::AsciiString::new("owned.fsd"),
        files.clone(),
    )
    .unwrap()
    .unwrap();
    let failure =
        super::records_from_driver(driver, &ExtendedString::new("owned.fsd")).unwrap_err();
    assert_eq!(failure.exception_type(), "Storage_StreamWriteError");
    assert_eq!(files.trace.closed.get(), files.trace.opens.borrow().len());
    assert_eq!(files.trace.closed.get(), 4);
}

#[test]
fn delimiter_selection_requires_exact_lines_and_uses_the_last_end_independently() {
    use martensite_tkernel::tcollection::AsciiString;
    let cases = [
        (vec!["START_REF-ish", "first", "END_REF"], vec![]),
        (vec!["START_REF", "first", "END_REF-ish"], vec![]),
        (
            vec!["START_REF", "first", "END_REF", "second", "END_REF"],
            vec!["first", "END_REF", "second"],
        ),
        (vec!["END_REF", "START_REF", "after"], vec![]),
    ];
    for (lines, expected) in cases {
        let lines: Vec<AsciiString> = lines.into_iter().map(AsciiString::new).collect();
        let actual: Vec<String> = super::delimited_records(lines.iter())
            .iter()
            .map(ExtendedString::to_utf8)
            .collect();
        assert_eq!(actual, expected);
    }
}

#[test]
fn already_open_references_driver_reports_native_error_then_drops_its_open_device() {
    use crate::pcdm::{
        file_header::select_driver,
        file_header_test_support::{FROZEN_FILES, FakeHeaderFiles},
    };
    use martensite_tkernel::{storage::OpenMode, tcollection::AsciiString};
    let files = FakeHeaderFiles::new(FROZEN_FILES[0].1[0], None);
    let mut driver = select_driver(&AsciiString::new("owned.fsd"), files.clone())
        .unwrap()
        .unwrap();
    driver
        .open(&AsciiString::new("owned.fsd"), OpenMode::Read)
        .unwrap();
    let failure =
        super::records_from_driver(driver, &ExtendedString::new("owned.fsd")).unwrap_err();
    assert_eq!(
        failure.message(),
        "could not open the file: owned.fsd; file was already opened"
    );
    assert_eq!(files.trace.closed.get(), 2);
}
