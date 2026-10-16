//! Compiler-stream contract tests for the expanded inventory collector (#81).
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(tag = "reason")]
enum CargoEvent {
    #[serde(rename = "compiler-message")]
    Diagnostic {
        package_id: String,
        target: CompilerTarget,
        message: CompilerDiagnostic,
    },
    #[serde(rename = "build-finished")]
    Finished { success: bool },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct CompilerTarget {
    name: String,
    kind: Vec<String>,
    src_path: String,
}

#[derive(Debug, Deserialize)]
struct CompilerDiagnostic {
    code: Option<DiagnosticCode>,
    message: String,
    spans: Vec<CompilerSpan>,
}

#[derive(Debug, Deserialize)]
struct DiagnosticCode {
    code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct CompilerSpan {
    file_name: String,
    byte_start: usize,
    byte_end: usize,
    line_start: usize,
    line_end: usize,
    column_start: usize,
    column_end: usize,
    is_primary: bool,
    expansion: Option<Box<MacroExpansion>>,
}

impl CompilerSpan {
    fn normalize_path(&mut self, root: &Path) {
        self.file_name = relative_source(&self.file_name, root);
        if let Some(expansion) = &mut self.expansion {
            expansion.span.normalize_path(root);
            if let Some(definition) = &mut expansion.def_site_span {
                definition.normalize_path(root);
            }
        }
    }
}

fn relative_source(source: &str, root: &Path) -> String {
    Path::new(source)
        .strip_prefix(root)
        .unwrap_or_else(|_| Path::new(source))
        .to_string_lossy()
        .replace('\\', "/")
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct MacroExpansion {
    span: CompilerSpan,
    macro_decl_name: String,
    def_site_span: Option<CompilerSpan>,
}

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct UnsafeDiagnostic {
    pub(super) package_id: String,
    target: CompilerTarget,
    message: String,
    spans: Vec<CompilerSpan>,
}

impl UnsafeDiagnostic {
    pub(super) fn normalize_workspace(&mut self, root: &Path, identity: &str) {
        self.package_id = identity.into();
        self.target.src_path = relative_source(&self.target.src_path, root);
        for span in &mut self.spans {
            span.normalize_path(root);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct SummaryLocation {
    package: String,
    target: String,
    target_kinds: Vec<String>,
    source: String,
    diagnostic: String,
    expansion_depth: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(super) struct SummaryEntry {
    location: SummaryLocation,
    emitted_primary_spans: usize,
}

fn expansion_depth(span: &CompilerSpan) -> usize {
    let mut depth = 0;
    let mut expansion = &span.expansion;
    while let Some(frame) = expansion {
        depth += 1;
        expansion = &frame.span.expansion;
    }
    depth
}

fn summary_location(diagnostic: &UnsafeDiagnostic, span: &CompilerSpan) -> SummaryLocation {
    SummaryLocation {
        package: diagnostic.package_id.clone(),
        target: diagnostic.target.name.clone(),
        target_kinds: diagnostic.target.kind.clone(),
        source: span.file_name.clone(),
        diagnostic: diagnostic.message.clone(),
        expansion_depth: expansion_depth(span),
    }
}

pub(super) fn summarize(diagnostics: &[UnsafeDiagnostic]) -> Vec<SummaryEntry> {
    let mut counts = BTreeMap::new();
    for diagnostic in diagnostics {
        for span in diagnostic.spans.iter().filter(|span| span.is_primary) {
            *counts
                .entry(summary_location(diagnostic, span))
                .or_default() += 1;
        }
    }
    counts
        .into_iter()
        .map(|(location, emitted_primary_spans)| SummaryEntry {
            location,
            emitted_primary_spans,
        })
        .collect()
}

pub(super) fn collect_stream(
    stdout: &str,
    exit_code: i32,
) -> Result<Vec<UnsafeDiagnostic>, String> {
    if exit_code != 0 {
        return Err(format!(
            "compiler exit {exit_code}; expected successful exit 0"
        ));
    }
    let mut diagnostics = Vec::new();
    let mut finished = false;
    for (index, line) in stdout.lines().enumerate() {
        if finished {
            return Err(format!(
                "line {} follows terminal build-finished",
                index + 1
            ));
        }
        let event = serde_json::from_str(line)
            .map_err(|error| format!("line {}: expected Cargo JSON event: {error}", index + 1))?;
        accept_event(event, &mut diagnostics, &mut finished)?;
    }
    if !finished {
        return Err("missing build-finished; expected complete successful compiler stream".into());
    }
    Ok(diagnostics)
}

fn accept_event(
    event: CargoEvent,
    diagnostics: &mut Vec<UnsafeDiagnostic>,
    finished: &mut bool,
) -> Result<(), String> {
    match event {
        CargoEvent::Finished { success: false } => {
            return Err("build-finished success=false; expected successful compilation".into());
        }
        CargoEvent::Finished { success: true } => *finished = true,
        CargoEvent::Diagnostic {
            package_id,
            target,
            message,
        } => {
            collect_diagnostic(package_id, target, message, diagnostics)?;
        }
        CargoEvent::Other => {}
    }
    Ok(())
}

fn collect_diagnostic(
    package_id: String,
    target: CompilerTarget,
    message: CompilerDiagnostic,
    diagnostics: &mut Vec<UnsafeDiagnostic>,
) -> Result<(), String> {
    if !message.code.is_some_and(|code| code.code == "unsafe_code") {
        return Ok(());
    }
    if !message.spans.iter().any(|span| span.is_primary) {
        return Err(format!(
            "unsafe diagnostic {:?}: expected a primary span",
            message.message
        ));
    }
    diagnostics.push(UnsafeDiagnostic {
        package_id,
        target,
        message: message.message,
        spans: message.spans,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    fn span() -> Value {
        json!({"file_name":"src/lib.rs", "byte_start":10, "byte_end":20,
            "line_start":2, "line_end":2, "column_start":1, "column_end":11,
            "is_primary":true, "expansion":null})
    }

    fn diagnostic(package: &str) -> Value {
        json!({"reason":"compiler-message", "package_id":package,
            "target":{"name":"same_name", "kind":["lib"], "src_path":"src/lib.rs"},
            "message":{"code":{"code":"unsafe_code"}, "message":"usage of an unsafe block",
                "spans":[span()]}})
    }

    fn stream(events: &[Value]) -> String {
        events
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn finished() -> Value {
        json!({"reason":"build-finished", "success":true})
    }

    #[test]
    fn preserves_package_identity_and_compilation_multiplicity() {
        let events = [
            diagnostic("path+file:///repo#one"),
            diagnostic("registry+url#one"),
            diagnostic("path+file:///repo#one"),
            finished(),
        ];
        let parsed = collect_stream(&stream(&events), 0).unwrap();
        assert_eq!(parsed.len(), 3);
        assert_ne!(parsed[0].package_id, parsed[1].package_id);
        assert_eq!(parsed[0], parsed[2]);
    }

    #[test]
    fn preserves_nested_macro_call_sites_and_definition_sites() {
        let mut expanded = diagnostic("workspace");
        let expansion = json!({"span":span(), "macro_decl_name":"inner!", "def_site_span":span()});
        expanded["message"]["spans"][0]["expansion"] = expansion.clone();
        expanded["message"]["spans"][0]["expansion"]["span"]["expansion"] = expansion;
        let parsed = collect_stream(&stream(&[expanded.clone(), finished()]), 0).unwrap();
        assert_eq!(
            serde_json::to_value(&parsed[0].spans).unwrap(),
            expanded["message"]["spans"]
        );
    }

    #[test]
    fn rejects_failed_truncated_and_malformed_streams() {
        for input in [
            "".into(),
            diagnostic("workspace").to_string(),
            "{bad".into(),
            "{}".into(),
            json!({"reason":"build-finished", "success":false}).to_string(),
            stream(&[finished(), finished()]),
        ] {
            assert!(collect_stream(&input, 0).is_err(), "accepted {input}");
        }
        assert!(collect_stream(&finished().to_string(), 101).is_err());
    }

    #[test]
    fn ignores_other_lints_but_rejects_unsafe_without_primary_span() {
        let mut event = diagnostic("workspace");
        event["message"]["spans"] = json!([]);
        assert!(collect_stream(&stream(&[event.clone(), finished()]), 0).is_err());
        event["message"]["code"]["code"] = json!("unused_variables");
        assert!(
            collect_stream(&stream(&[event, finished()]), 0)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn workspace_normalization_preserves_external_paths_and_macro_locations() {
        let mut event = diagnostic("path+file:///checkout#package");
        event["target"]["src_path"] = json!("/checkout/src/lib.rs");
        event["message"]["spans"][0]["file_name"] = json!("/checkout/src/lib.rs");
        let mut call_site = span();
        call_site["file_name"] = json!("/checkout/tests/caller.rs");
        let mut definition = span();
        definition["file_name"] = json!("/checkout-other/macros.rs");
        event["message"]["spans"][0]["expansion"] = json!({"span":call_site,
            "macro_decl_name":"expand!", "def_site_span":definition});
        let mut parsed = collect_stream(&stream(&[event, finished()]), 0).unwrap();
        parsed[0].normalize_workspace(Path::new("/checkout"), "workspace:package@1.0.0");
        let value = serde_json::to_value(&parsed[0]).unwrap();
        assert_eq!(value["target"]["src_path"], "src/lib.rs");
        assert_eq!(
            value["spans"][0]["expansion"]["span"]["file_name"],
            "tests/caller.rs"
        );
        assert_eq!(
            value["spans"][0]["expansion"]["def_site_span"]["file_name"],
            "/checkout-other/macros.rs"
        );
        assert_eq!(value["spans"][0]["byte_start"], 10);
    }

    #[test]
    fn summaries_count_repeated_compilations_and_distinguish_expansion_depth() {
        let direct = diagnostic("workspace");
        let mut expanded = direct.clone();
        expanded["message"]["spans"][0]["expansion"] = json!({"span":span(),
            "macro_decl_name":"make!", "def_site_span":null});
        let events = [expanded, direct.clone(), direct, finished()];
        let diagnostics = collect_stream(&stream(&events), 0).unwrap();
        let summary = summarize(&diagnostics);
        assert_eq!(summary.len(), 2);
        assert_eq!(summary[0].location.expansion_depth, 0);
        assert_eq!(summary[0].emitted_primary_spans, 2);
        assert_eq!(summary[1].location.expansion_depth, 1);
        assert_eq!(summary[1].emitted_primary_spans, 1);
    }
}
