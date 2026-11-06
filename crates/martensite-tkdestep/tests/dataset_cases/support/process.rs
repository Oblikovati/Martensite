//! Runs each case in its own child process, under the group's budget, and grades everything
//! that child wrote to its stdout and stderr.
use super::{
    raster_session::KEEP_IMAGES_ENV,
    rules::{Rule, Verdict, grade},
};
use martensite_process_policy::case_runner::{
    CaseReceipt, CaseRunError, CaseRunner, CaseSpec, FactorProfile,
};
use std::{cell::Cell, path::PathBuf};

/// The verdict of a case's stream in the parent, `None` in the child that ran the body.
/// Example: `verdict_of("controls::fatal", 300, rules, || println!("Error"))`.
pub fn verdict_of(
    identity: &str,
    cpu_seconds: i64,
    rules: &[Rule],
    body: impl FnOnce(),
) -> Result<Option<Verdict>, CaseRunError> {
    let stream = case_stream(identity, cpu_seconds, body)?;
    Ok(stream.map(|text| grade(&text, rules)))
}

/// The child's whole stream in the parent, `None` in the child that ran the body: the policy
/// holds before the body starts, and its receipt folder goes once the stream is read.
/// Example: `case_stream("controls::line", 300, || println!("line"))`.
pub fn case_stream(
    identity: &str,
    cpu_seconds: i64,
    body: impl FnOnce(),
) -> Result<Option<String>, CaseRunError> {
    let spec = CaseSpec::new(identity, cpu_seconds, FactorProfile::RequireUnset);
    let executed = Cell::new(false);
    let root = receipts_root();
    let receipt = CaseRunner::new(&root, None).run(spec, || {
        executed.set(true);
        body();
        Ok(())
    })?;
    if executed.get() {
        return Ok(None);
    }
    let stream = finalized_stream(&receipt, identity);
    keep_stream_for_inspection(identity, &stream);
    std::fs::remove_dir_all(&receipt.directory).unwrap();
    Ok(Some(stream))
}

fn receipts_root() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("dataset-case-receipts")
}

fn finalized_stream(receipt: &CaseReceipt, identity: &str) -> String {
    let read = |file: &str| {
        let path = receipt.directory.join(file);
        std::fs::read_to_string(&path).unwrap_or_else(|cause| panic!("{path:?}: {cause}"))
    };
    let stdout = read("stdout.log");
    let body = captured_body(&stdout, identity).unwrap_or_else(|| {
        panic!("case {identity}: expected one complete libtest frame, got {stdout:?}")
    });
    format!("{body}\n{}", read("stderr.log"))
}

/// Writes the stream beside the kept images, so a run can be compared with a reference log.
fn keep_stream_for_inspection(identity: &str, stream: &str) {
    let Some(folder) = std::env::var_os(KEEP_IMAGES_ENV) else {
        return;
    };
    std::fs::create_dir_all(&folder).unwrap();
    let file = identity.replace("::", "_") + ".stream.txt";
    std::fs::write(PathBuf::from(folder).join(file), stream).unwrap();
}

/// What the child's test wrote: everything libtest's frame holds, plus any bytes written past
/// libtest's capture before the frame. `None` unless it holds exactly this one passing test;
/// a test that wrote nothing has no frame, and its stream is empty.
fn captured_body(stdout: &str, identity: &str) -> Option<String> {
    let header = format!("\n---- {identity} stdout ----\n");
    let trailer = format!("\n\nsuccesses:\n    {identity}\n\ntest result:");
    if stdout.matches(&header).count() > 1 || stdout.matches(&trailer).count() != 1 {
        return None;
    }
    let (head, summary) = stdout.split_once(&trailer)?;
    if summary.trim().lines().count() != 1 {
        return None;
    }
    let (prefix, body) = match head.split_once(&header) {
        Some((prefix, body)) => (prefix.to_owned(), body),
        None => (format!("{head}\n"), ""),
    };
    let progress = format!("running 1 test\ntest {identity} ... ");
    let direct = prefix.trim_start_matches('\n').strip_prefix(&progress)?;
    let direct = direct.strip_suffix("ok\n\nsuccesses:\n")?;
    Some(format!("{direct}{body}"))
}

#[test]
fn only_one_complete_passing_frame_is_read() {
    let frame = "\nrunning 1 test\ntest g::c ... ok\n\nsuccesses:\n\n---- g::c stdout ----\nline\n\nsuccesses:\n    g::c\n\ntest result: ok. 1 passed\n";
    assert_eq!(captured_body(frame, "g::c").as_deref(), Some("line"));
    assert_eq!(captured_body(frame, "g::other"), None);
    assert_eq!(captured_body("no frame", "g::c"), None);
    let silent = "\nrunning 1 test\ntest g::c ... ok\n\nsuccesses:\n\nsuccesses:\n    g::c\n\ntest result: ok. 1 passed\n";
    assert_eq!(captured_body(silent, "g::c").as_deref(), Some(""));
    let direct = "\nrunning 1 test\ntest g::c ... raw\nok\n\nsuccesses:\n\nsuccesses:\n    g::c\n\ntest result: ok. 1 passed\n";
    assert_eq!(captured_body(direct, "g::c").as_deref(), Some("raw\n"));
}
