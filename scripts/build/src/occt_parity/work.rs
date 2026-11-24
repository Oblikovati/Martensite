//! The optional work column of a case line (#583): counters a cases program reports per
//! operation beside its time, such as the device work of the Vulkan renderer. Only one side
//! reports them, so they are shown and stored, never compared.

/// One reported counter of a case, per operation; `WorkValue { name: "buffers".into(), value: 2.0 }`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct WorkValue {
    /// The counter name, with its unit when it has one; `"upload_kib"`.
    pub name: String,
    /// The value per operation.
    pub value: f64,
}

/// Parses space-separated `name=value` pairs; None when a pair is malformed.
/// Example: `parse_work("buffers=2 upload_kib=0.5")`.
pub fn parse_work(field: &str) -> Option<Vec<WorkValue>> {
    field.split_whitespace().map(parse_pair).collect()
}

fn parse_pair(pair: &str) -> Option<WorkValue> {
    let (name, value) = pair.split_once('=')?;
    let value = value.parse().ok()?;
    (!name.is_empty()).then(|| WorkValue {
        name: name.to_owned(),
        value,
    })
}

/// A Markdown table of the cases that reported work, with the counter names of the first one
/// as columns; None when no case reported any. Example: `render_work_table(&rows)`.
pub fn render_work_table(rows: &[(&str, &[WorkValue])]) -> Option<String> {
    let reported: Vec<&(&str, &[WorkValue])> =
        rows.iter().filter(|(_, work)| !work.is_empty()).collect();
    let (_, first) = reported.first()?;
    let names: Vec<&str> = first.iter().map(|value| value.name.as_str()).collect();
    let rule = format!("| --- |{}", " ---: |".repeat(names.len()));
    let mut table = format!("| Case | {} |\n{rule}\n", names.join(" | "));
    for (case, work) in reported {
        table.push_str(&work_row(case, &names, work));
    }
    Some(table)
}

/// One row; a counter the case did not report is shown as `-`.
fn work_row(case: &str, names: &[&str], work: &[WorkValue]) -> String {
    let cells: Vec<String> = names
        .iter()
        .map(|name| {
            work.iter()
                .find(|value| value.name == *name)
                .map_or_else(|| "-".to_owned(), |value| format!("{:.2}", value.value))
        })
        .collect();
    format!("| {case} | {} |\n", cells.join(" | "))
}

#[cfg(test)]
#[path = "work_tests.rs"]
mod tests;
