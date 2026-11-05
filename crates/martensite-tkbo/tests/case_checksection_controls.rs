//! Native-pinned complete section text and inherited warning policy controls.
#[path = "support/case_bop_fatal_rules.rs"]
mod case_bop_fatal_rules;
#[path = "support/case_checksection_report.rs"]
mod case_checksection_report;
use martensite_tkbo::bop_algo::{
    AlertBadPositioning, AlertIntersectionOfPairOfShapesFailed, Options,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::AlertHandle;
martensite_tkernel::define_simple_alert!(FatalWordWarning, "operation has failed");
use case_checksection_report::checksection_response;

#[test]
fn native_zero_and_negative_reference_responses_are_exact() {
    for reference in [None, Some(-1)] {
        assert_eq!(
            checksection_response(&[], reference),
            " nb alone Vertices : 0\n\n"
        );
    }
    assert_eq!(
        checksection_response(&[], Some(0)),
        " nb alone Vertices : 0\nSection is OK\n\n"
    );
    assert_eq!(
        checksection_response(&[], Some(2)),
        " nb alone Vertices : 0\nError: 2 vertices are expected but 0 are found.\n\n"
    );
}

#[test]
fn native_nonzero_response_keeps_all_publication_names() {
    let actual_vertices = [Shape::default(), Shape::default()];
    for reference in [None, Some(-1)] {
        assert_eq!(
            checksection_response(&actual_vertices, reference),
            " nb alone Vertices : 2\nalone_1 alone_2 \n"
        );
    }
    assert_eq!(
        checksection_response(&actual_vertices, Some(0)),
        " nb alone Vertices : 2\nError: 0 vertices are expected but 2 are found.\nalone_1 alone_2 \n"
    );
    assert_eq!(
        checksection_response(&actual_vertices, Some(2)),
        " nb alone Vertices : 2\nSection is OK\nalone_1 alone_2 \n"
    );
}

fn raw_warning_text(alert: AlertHandle) -> String {
    let options = Options::new();
    options.add_warning(&alert);
    assert!(options.has_warnings() && !options.has_errors());
    let mut actual = Vec::new();
    options.dump_warnings(&mut actual).unwrap();
    String::from_utf8(actual).unwrap()
}

#[test]
fn raw_warning_policy_distinguishes_keys_from_fatal_words() {
    fn thread_bounds<T: Send + Sync>() {}
    thread_bounds::<FatalWordWarning>();
    let okay = "OK : Correct result of boolean operation\nTEST COMPLETED\n";
    let bad_position: AlertHandle =
        std::sync::Arc::new(AlertBadPositioning::new(&Shape::default()));
    let pair_failure: AlertHandle =
        std::sync::Arc::new(AlertIntersectionOfPairOfShapesFailed::new(&Shape::default()));
    for alert in [bad_position, pair_failure] {
        assert!(case_bop_fatal_rules::accepted_bop_report(&format!(
            "{}{okay}",
            raw_warning_text(alert)
        )));
    }
    let fatal: AlertHandle = std::sync::Arc::new(FatalWordWarning);
    let response = format!("{}{okay}", raw_warning_text(fatal));
    assert!(!case_bop_fatal_rules::accepted_bop_report(&response));
}
