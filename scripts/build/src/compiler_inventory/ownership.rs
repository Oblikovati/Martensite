//! Cargo membership, rather than package naming, defines first-party inventory.
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::parser::UnsafeDiagnostic;
use crate::error::BuildResult;
use crate::process::{CommandRunner, CommandSpec};

#[derive(Deserialize)]
struct WorkspaceMembers {
    workspace_members: BTreeSet<String>,
    packages: Vec<PackageIdentity>,
}

#[derive(Deserialize)]
struct PackageIdentity {
    id: String,
    name: String,
    version: String,
}

#[derive(Serialize)]
pub(super) struct ClassifiedDiagnostics {
    pub(super) workspace_summary: Vec<super::parser::SummaryEntry>,
    workspace: Vec<UnsafeDiagnostic>,
    dependencies: Vec<UnsafeDiagnostic>,
}

pub(super) fn classify(
    runner: &dyn CommandRunner,
    root: &Path,
    diagnostics: Vec<UnsafeDiagnostic>,
) -> BuildResult<ClassifiedDiagnostics> {
    let spec = CommandSpec::new("cargo", root).args([
        "metadata",
        "--format-version",
        "1",
        "--no-deps",
        "--locked",
    ]);
    let members = parse_members(&runner.capture(&spec)?)?;
    Ok(partition_diagnostics(root, diagnostics, &members))
}

fn partition_diagnostics(
    root: &Path,
    diagnostics: Vec<UnsafeDiagnostic>,
    members: &BTreeMap<String, String>,
) -> ClassifiedDiagnostics {
    let (mut workspace, mut dependencies): (Vec<_>, Vec<_>) = diagnostics
        .into_iter()
        .partition(|diagnostic| members.contains_key(&diagnostic.package_id));
    for diagnostic in &mut workspace {
        if let Some(identity) = members.get(&diagnostic.package_id) {
            diagnostic.normalize_workspace(root, identity);
        }
    }
    workspace.sort();
    dependencies.sort();
    ClassifiedDiagnostics {
        workspace_summary: super::parser::summarize(&workspace),
        workspace,
        dependencies,
    }
}

fn parse_members(json: &str) -> BuildResult<BTreeMap<String, String>> {
    let document: WorkspaceMembers = serde_json::from_str(json)
        .map_err(|error| super::output_error(format!("Cargo membership: {error}")))?;
    if document.workspace_members.is_empty() || document.workspace_members.contains("") {
        return Err(super::output_error(format!(
            "Cargo workspace membership {:?}: expected a nonempty set of nonempty package IDs",
            document.workspace_members
        )));
    }
    let identities = member_identities(&document);
    require_member_identities(&document.workspace_members, &identities)?;
    Ok(identities)
}

fn member_identities(document: &WorkspaceMembers) -> BTreeMap<String, String> {
    document
        .packages
        .iter()
        .filter(|package| document.workspace_members.contains(&package.id))
        .map(|package| {
            (
                package.id.clone(),
                format!("workspace:{}@{}", package.name, package.version),
            )
        })
        .collect()
}

fn require_member_identities(
    members: &BTreeSet<String>,
    identities: &BTreeMap<String, String>,
) -> BuildResult<()> {
    if identities.len() != members.len() {
        let missing: Vec<_> = members
            .iter()
            .filter(|id| !identities.contains_key(*id))
            .collect();
        return Err(super::output_error(format!(
            "Cargo workspace package IDs {missing:?}: expected a package identity for every member"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;
    use serde_json::json;

    fn compiler_event(package: &str) -> String {
        json!({"reason":"compiler-message", "package_id":package,
            "target":{"name":"same_name", "kind":["lib"], "src_path":"src/lib.rs"},
            "message":{"code":{"code":"unsafe_code"}, "message":"unsafe block", "spans":[{
                "file_name":"src/lib.rs", "byte_start":0,"byte_end":1,"line_start":1,
                "line_end":1,"column_start":1,"column_end":2,"is_primary":true,"expansion":null}]}})
        .to_string()
    }

    #[test]
    fn identical_target_names_are_classified_by_exact_package_id() {
        let stdout = format!(
            "{}\n{}\n{{\"reason\":\"build-finished\",\"success\":true}}",
            compiler_event("path+file:///repo#same"),
            compiler_event("registry+url#same")
        );
        let diagnostics = super::super::parser::collect_stream(&stdout, 0).unwrap();
        let runner = RecordingRunner::default().with_stdout(
            "cargo metadata --format-version 1 --no-deps --locked",
            r#"{"workspace_members":["path+file:///repo#same"],"packages":[{"id":"path+file:///repo#same","name":"same","version":"1.0.0"}]}"#,
        );
        let classified = classify(&runner, Path::new("/repo"), diagnostics).unwrap();
        assert_eq!(classified.workspace.len(), 1);
        assert_eq!(classified.dependencies.len(), 1);
        assert_eq!(classified.workspace[0].package_id, "workspace:same@1.0.0");
        assert_eq!(classified.dependencies[0].package_id, "registry+url#same");
    }

    #[test]
    fn missing_membership_cannot_silently_classify_everything_as_dependencies() {
        for json in [
            "{}",
            "{\"workspace_members\":[]}",
            "{\"workspace_members\":[\"\"]}",
        ] {
            assert!(parse_members(json).is_err());
        }
    }

    fn relocated_report(root: &str, reverse: bool) -> ClassifiedDiagnostics {
        let package = format!("opaque-cargo-id:{root}");
        let stdout = relocated_stream(root, &package, reverse);
        let metadata = json!({"workspace_members":[package], "packages":[{
            "id":package,"name":"same","version":"1.0.0"}]})
        .to_string();
        let runner = RecordingRunner::default().with_stdout(
            "cargo metadata --format-version 1 --no-deps --locked",
            &metadata,
        );
        let diagnostics = super::super::parser::collect_stream(&stdout, 0).unwrap();
        classify(&runner, Path::new(root), diagnostics).unwrap()
    }

    fn relocated_stream(root: &str, package: &str, reverse: bool) -> String {
        let mut event: serde_json::Value = serde_json::from_str(&compiler_event(package)).unwrap();
        event["target"]["src_path"] = json!(format!("{root}/src/lib.rs"));
        event["message"]["spans"][0]["file_name"] = json!(format!("{root}/src/lib.rs"));
        let mut other = event.clone();
        other["message"]["spans"][0]["byte_start"] = json!(5);
        other["message"]["spans"][0]["byte_end"] = json!(6);
        let mut events = [event.clone(), other, event];
        if reverse {
            events.reverse();
            events.rotate_left(1);
        }
        let mut stdout = events
            .iter()
            .map(serde_json::Value::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        stdout.push_str("\n{\"reason\":\"build-finished\",\"success\":true}");
        stdout
    }

    #[test]
    fn checkout_relocation_and_compiler_order_do_not_change_workspace_report() {
        let first = relocated_report("/checkout one", false);
        let second = relocated_report("/different/checkout two", true);
        assert_eq!(
            serde_json::to_vec(&first).unwrap(),
            serde_json::to_vec(&second).unwrap()
        );
        assert_eq!(first.workspace.len(), 3);
        assert_eq!(first.workspace[0], first.workspace[1]);
        assert_ne!(first.workspace[1], first.workspace[2]);
    }

    #[test]
    fn declared_member_without_package_identity_is_rejected() {
        let error =
            parse_members(r#"{"workspace_members":["missing"],"packages":[]}"#).unwrap_err();
        assert!(error.to_string().contains("missing"));
        assert!(error.to_string().contains("expected a package identity"));
    }

    #[test]
    fn empty_member_reports_invalid_membership_and_expected_shape() {
        let error = parse_members(r#"{"workspace_members":[""],"packages":[]}"#).unwrap_err();
        assert!(error.to_string().contains("{\"\"}"));
        assert!(
            error
                .to_string()
                .contains("expected a nonempty set of nonempty package IDs")
        );
    }
}
