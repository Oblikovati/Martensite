//! Frozen state/count/publication expectations for behavior version 1.
use super::{checked_input::*, fakes::FailingBufferedInput};
use std::fmt::Write;

struct MatrixBaseline<'a> { rows: std::str::Lines<'a>, count: usize }
impl MatrixBaseline<'_> {
    fn check(&mut self, actual: String) {
        assert_eq!(Some(actual.as_str()), self.rows.next(), "input matrix row {}", self.count + 1);
        self.count += 1;
    }
}
fn error_kind(answer: Result<(), InputFailure>) -> &'static str {
    match answer {
        Ok(()) => "none",
        Err(fault) => match fault.cause { InputCause::Device(_) => "device", InputCause::State => "state" }
    }
}
fn extraction_row(length: usize, mask: u8, failure: i32, operation: &str, capacity: usize) -> String {
    let mut source = FailingBufferedInput::new(&b"AB>CD"[..length]);
    source.failure_at = usize::try_from(failure).ok();
    let mut buffered = PositionedInput(&mut source);
    let mut context = InputContext::new(&mut buffered);
    context.set_exception_mask(mask).unwrap();
    let mut output = vec![0xee; if operation == "get" { 1 } else { capacity }];
    let error = extraction_error(&mut context, &mut output, operation);
    let (flags, count) = (context.state(), context.gcount());
    drop(context);
    let mut row = format!("{length}:{mask}:{failure}:{operation}:{capacity} {flags} {count} {} {error} ", source.cursor);
    for byte in output { write!(row, "{byte:x},").unwrap(); }
    row
}
fn extraction_error(input: &mut dyn UnformattedInput, output: &mut [u8], operation: &str) -> &'static str {
    error_kind(match operation {
        "get" => input.get(&mut output[0]),
        "line" => input.getline(output, b'>'),
        _ => input.read(output),
    })
}
fn capacity_rows(length: usize, mask: u8, failure: i32, baseline: &mut MatrixBaseline<'_>) {
    for capacity in [0, 1, 2, 3, 4, 6] {
        baseline.check(extraction_row(length, mask, failure, "line", capacity));
        baseline.check(extraction_row(length, mask, failure, "read", capacity));
    }
}
fn failure_rows(length: usize, mask: u8, baseline: &mut MatrixBaseline<'_>) {
    for failure in -1..=5 {
        baseline.check(extraction_row(length, mask, failure, "get", 1));
        capacity_rows(length, mask, failure, baseline);
    }
}
fn position_result(source: &mut dyn InputBuffer, mask: u8) -> (u8, usize, i64, &'static str) {
    let mut input = InputContext::new(source);
    input.set_exception_mask(mask).unwrap();
    input.get(&mut 0).unwrap();
    let (position, error) = match input.position() {
        Ok(value) => (value.map(|n| n as i64).unwrap_or(-1), "none"),
        Err(fault) => (-1, error_kind(Err(fault))),
    };
    (input.state(), input.gcount(), position, error)
}
fn position_row(origin: usize, mode: usize, mask: u8) -> String {
    let mut reader = FailingBufferedInput::new(b"AB>CD");
    reader.cursor = origin;
    reader.fail_position = mode == 2;
    let (flags, count, position, error) = if mode == 1 {
        position_result(&mut BufferedInput(&mut reader), mask)
    } else { position_result(&mut PositionedInput(&mut reader), mask) };
    format!("position:{origin}:{mode}:{mask} {flags} {count} {} {position} {error}", reader.cursor)
}
fn position_rows(origin: usize, baseline: &mut MatrixBaseline<'_>) {
    for mode in [0, 1, 2] {
        for mask in [0, 1, 2, 4, 5, 6, 7] { baseline.check(position_row(origin, mode, mask)); }
    }
}
#[test]
fn all_reviewed_state_count_position_and_partial_output_rows_are_retained() {
    let text = include_str!("state_matrix.txt");
    assert!(text.ends_with('\n'));
    let mut baseline = MatrixBaseline { rows: text.lines(), count: 0 };
    for length in 0..=5 {
        for mask in [0, 1, 2, 4, 5, 6, 7] { failure_rows(length, mask, &mut baseline); }
    }
    for origin in [0, 2] { position_rows(origin, &mut baseline); }
    assert_eq!(baseline.count, 3864);
    assert!(baseline.rows.next().is_none());
}
