//! Source diagnostics retain order and the cases' independent weak predicates.
use super::*;
use super::{objects::named_text, predicates::*, selected::has_selected_text, topology::*};

/// Preserve original query/pick/compare diagnostic order; `roundtrip_checks(session, operands, false)?`.
pub(super) fn roundtrip_checks(
    session: &mut ViewerSession,
    operands: &mut TopologyOperands,
    local: bool,
) -> Result<(), Failure> {
    report_has_selected(session);
    operands.pick(session, "aSelected")?;
    report_missing_alias(session, local)?;
    let result = compare_text(operands.shape("aBox")?, operands.shape("aSelected")?);
    report_compare(result, local);
    Ok(())
}

fn report_has_selected(session: &ViewerSession) {
    if has_token(&has_selected_text(session.context()), "0") {
        println!("ERROR: Incorrect result of HasSelectedShape of the context!");
    }
}

fn report_missing_alias(session: &ViewerSession, local: bool) -> Result<(), Failure> {
    if missing_selected_literal(&named_text(session, "aSelected")?) {
        println!(
            "ERROR: No shape selected in {}!",
            if local {
                "local selection"
            } else {
                "neutral point"
            }
        );
    }
    Ok(())
}

fn report_compare(text: &str, local: bool) {
    if has_token(text, "not") {
        println!(
            "ERROR: Selected box and selected {} are not equal!",
            if local {
                "solid"
            } else {
                "shape from the context"
            }
        );
    }
}

/// Print captured source items before errors; `report(&items, &errors)`.
pub(super) fn report(lines: &[String], errors: &[&str]) {
    let mut output = StdoutQueryLines;
    report_to(&mut output, lines, errors);
}

/// Own the source CLI boundary; a named output fake implements `line(text)`.
pub(super) trait QueryOutputLines {
    fn line(&mut self, text: &str);
}

struct StdoutQueryLines;
impl QueryOutputLines for StdoutQueryLines {
    fn line(&mut self, text: &str) {
        // Original CLI output is plain text; only control observations are structured JSON.
        println!("{text}");
    }
}

/// Exercise the same output ordering with a named sink; `report_to(sink, items, errors)`.
pub(super) fn report_to(output: &mut impl QueryOutputLines, lines: &[String], errors: &[&str]) {
    for line in lines {
        output.line(line);
    }
    for error in errors {
        output.line(error);
    }
}
