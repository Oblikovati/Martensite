use super::failure::CaseFailure;

pub(crate) fn validate(stdout: &[u8], identity: &str) -> Result<(), CaseFailure> {
    let text = std::str::from_utf8(stdout)
        .map_err(|error| mismatch(error.to_string(), "UTF-8 Rust 1.99 libtest framing"))?;
    let lines: Vec<&str> = text.lines().collect();
    let first = lines
        .iter()
        .position(|line| !line.is_empty())
        .ok_or_else(|| mismatch("empty output", "running 1 test"))?;
    require_line(lines.get(first).copied(), "running 1 test")?;
    let rest = &lines[first + 1..];
    let boundary = rest
        .windows(2)
        .position(|pair| pair == ["", "successes:"])
        .ok_or_else(|| mismatch("no result boundary", "blank line before successes section"))?;
    validate_progress(&rest[..boundary], identity)?;
    validate_sections(&rest[boundary + 1..], identity)
}

// Descriptor writes bypass libtest capture and may interrupt its unfinished result row.
// Consume only the first section boundary, retaining every original diagnostic byte.
fn validate_progress(lines: &[&str], identity: &str) -> Result<(), CaseFailure> {
    let prefix = format!("test {identity} ... ");
    let progress = lines.join("\n");
    let diagnostic = progress
        .strip_prefix(&prefix)
        .and_then(|text| text.strip_suffix("ok"))
        .ok_or_else(|| mismatch(&progress, format!("{prefix}<direct output>ok")))?;
    if diagnostic.lines().any(reserved_progress_line) {
        return Err(mismatch(
            diagnostic,
            "direct output without additional harness framing",
        ));
    }
    Ok(())
}

fn reserved_progress_line(line: &str) -> bool {
    line.starts_with("test ")
        || line.starts_with("running ")
        || matches!(line, "successes:" | "failures:")
}

fn validate_sections(lines: &[&str], identity: &str) -> Result<(), CaseFailure> {
    let last = lines
        .iter()
        .rposition(|line| !line.is_empty())
        .ok_or_else(|| mismatch("no result", "final successful summary"))?;
    validate_summary(lines[last])?;
    let before_summary = &lines[..last];
    let tail = ["successes:", &format!("    {identity}"), ""];
    if before_summary.ends_with(&tail) {
        return validate_capture_prefix(
            &before_summary[..before_summary.len() - tail.len()],
            identity,
        );
    }
    Err(mismatch(
        before_summary.join("\n"),
        "exact successes tail for the selected identity",
    ))
}

fn validate_capture_prefix(lines: &[&str], identity: &str) -> Result<(), CaseFailure> {
    let prefix = ["successes:", ""];
    if !lines.starts_with(&prefix) {
        return Err(mismatch(
            lines.join("\n"),
            "one show-output successes section",
        ));
    }
    let rest = &lines[prefix.len()..];
    if rest.iter().all(|line| line.is_empty()) {
        return Ok(());
    }
    require_line(
        rest.first().copied(),
        &format!("---- {identity} stdout ----"),
    )
}

fn validate_summary(line: &str) -> Result<(), CaseFailure> {
    let prefix = "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; ";
    let suffix = line
        .strip_prefix(prefix)
        .ok_or_else(|| mismatch(line, prefix))?;
    let (filtered, duration) = suffix
        .split_once(" filtered out; finished in ")
        .ok_or_else(|| mismatch(suffix, "filtered count; finished in elapsed seconds"))?;
    if filtered.parse::<usize>().is_err() || !valid_duration(duration) {
        return Err(mismatch(
            suffix,
            "unsigned filtered count and finite nonnegative duration ending in s",
        ));
    }
    Ok(())
}

fn valid_duration(text: &str) -> bool {
    let Some(number) = text.strip_suffix('s') else {
        return false;
    };
    number
        .parse::<f64>()
        .is_ok_and(|seconds| seconds.is_finite() && seconds >= 0.0)
}

fn require_line(actual: Option<&str>, expected: &str) -> Result<(), CaseFailure> {
    if actual == Some(expected) {
        return Ok(());
    }
    Err(mismatch(format!("{actual:?}"), expected))
}

pub(crate) fn mismatch(observed: impl Into<String>, expected: impl Into<String>) -> CaseFailure {
    CaseFailure::Protocol {
        observed: observed.into(),
        expected: expected.into(),
    }
}
