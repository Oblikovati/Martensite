//! Bounded original numeric corpus replay without processes, files or native libraries.
use super::complete_numeric_replay::{
    replay_operations::buffered_tokens::BufferedTokens, replay_request,
};
use martensite_tkernel::standard::Failure;

#[test]
fn case_curve_traversal_and_mutation_corpus() {
    replay_fixture(include_str!("fixtures/ds_curve_explorer.tsv"), 255);
}
#[test]
fn case_point_traversal_and_mutation_corpus() {
    replay_fixture(include_str!("fixtures/ds_point_explorer.tsv"), 109);
}
#[test]
fn case_interference_factory_and_alias_corpus() {
    replay_fixture(include_str!("fixtures/ds_interference_tool.tsv"), 288);
}
fn replay_fixture(fixture: &str, expected_count: usize) {
    assert_eq!(fixture.lines().count(), expected_count);
    for row in fixture.lines() {
        let (request, expected) = row.split_once('\t').expect("request and expected tokens");
        let actual = replay_request(request);
        assert_eq!(
            actual.split_whitespace().collect::<Vec<_>>(),
            expected.split_whitespace().collect::<Vec<_>>(),
            "{request}"
        );
    }
}
#[test]
fn captured_tokens_are_request_local_and_reset_after_finish() {
    let first = BufferedTokens::captured();
    let second = BufferedTokens::captured();
    first.int(-7);
    second.real(-0.);
    assert_eq!(first.finish(), "-7 ");
    assert_eq!(second.finish(), format!("{} ", (-0_f64).to_bits()));
    assert_eq!(first.finish(), "");
    assert_eq!(second.finish(), "");
}
#[test]
fn captured_encoder_preserves_text_point_flags_and_exception_tokens() {
    use martensite_tkmath::gp::Point2;
    let output = BufferedTokens::captured();
    output.text(b"A\0");
    output.pnt2d(Point2::new(0., -0.));
    output.flag(false);
    output.flag(true);
    output.attempt(|| Ok(()));
    output.attempt(sample_failure);
    output.end();
    assert_eq!(
        output.finish(),
        "t4100 0 9223372036854775808 0 1 Standard_Failure \n"
    );
}

#[test]
fn captured_output_can_cross_threads_without_shared_algorithm_state() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<BufferedTokens>();
    let output = std::thread::spawn(|| {
        let output = BufferedTokens::captured();
        output.flag(true);
        output
    })
    .join()
    .unwrap();
    assert_eq!(output.finish(), "1 ");
}

fn sample_failure() -> Result<(), Failure> {
    Err(Failure::with_message(
        "Standard_Failure",
        "unused diagnostic",
    ))
}

#[test]
fn extended_encoders_record_vectors_directions_and_planar_coordinates() {
    use martensite_tkmath::gp::{Direction3, Xy, Xyz};
    let output = BufferedTokens::captured();
    output.xyz(Xyz::new(1., 0., -0.));
    output.dir(Direction3::Z);
    output.xy(Xy::new(0., 1.));
    assert_eq!(
        output.finish(),
        "4607182418800017408 0 9223372036854775808 0 0 4607182418800017408 0 4607182418800017408 "
    );
}
#[test]
fn fallible_encoders_keep_value_callbacks_and_console_before_failure() {
    let output = BufferedTokens::captured();
    output.show(Ok(7), |value| output.int(value));
    output.show::<i32>(sample_failure().map(|_| 0), |_| {
        panic!("failed value callback")
    });
    output.captured_console(|text| {
        text.push_str("abc");
        sample_failure()
    });
    assert_eq!(
        output.finish(),
        "7 Standard_Failure t616263 Standard_Failure "
    );
}
