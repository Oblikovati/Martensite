//! Sanitizer ownership follows the package test scope, including dependency impact (#410).
use crate::git::ChangedFile;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Control {
    Cache2d,
    Cache3d,
    Capture,
    Groups,
    FaceInput,
    FaceCertificate,
    OpenExtrusion,
    VertexNeighborhood,
    SurfaceSketch,
    FontTextCapture,
}

pub(crate) const CONTROLS: [Control; 10] = [
    Control::Cache2d,
    Control::Cache3d,
    Control::Capture,
    Control::Groups,
    Control::FaceInput,
    Control::FaceCertificate,
    Control::OpenExtrusion,
    Control::VertexNeighborhood,
    Control::SurfaceSketch,
    Control::FontTextCapture,
];

impl Control {
    pub(crate) fn id(self) -> &'static str {
        match self {
            Self::Cache2d => "cache-2d",
            Self::Cache3d => "cache-3d",
            Self::Capture => "capture",
            Self::Groups => "groups",
            Self::FaceInput => "face-input",
            Self::FaceCertificate => "face-certificate",
            Self::OpenExtrusion => "open-extrusion",
            Self::VertexNeighborhood => "vertex-neighborhood",
            Self::SurfaceSketch => "surface-sketch",
            Self::FontTextCapture => "font-text-capture",
        }
    }

    pub(crate) fn owner(self) -> &'static str {
        match self {
            Self::Cache2d => "martensite-tkg2d",
            Self::Cache3d => "martensite-tkg3d",
            Self::Capture | Self::Groups => "martensite-tkservice",
            Self::FaceInput => "martensite-tkbrep",
            Self::FaceCertificate => "martensite-tktopalgo",
            Self::OpenExtrusion | Self::VertexNeighborhood => "martensite-tkmath",
            Self::SurfaceSketch => "martensite-tksketch",
            Self::FontTextCapture => "martensite-tkv3d",
        }
    }
}

pub(crate) fn select(packages: &BTreeSet<String>, changed: &[ChangedFile]) -> Vec<Control> {
    let driver_dependencies = packages.contains(crate::context::BUILD_TOOL_PACKAGE)
        && changed
            .iter()
            .any(|file| matches!(file.path.as_str(), "Cargo.lock" | "Cargo.toml"));
    let execution_changed =
        driver_dependencies || changed.iter().any(|file| affects_execution(&file.path));
    CONTROLS
        .into_iter()
        .filter(|control| execution_changed || packages.contains(control.owner()))
        .collect()
}

fn affects_execution(path: &str) -> bool {
    if path == ".github/workflows/martensite.yml"
        || path == "scripts/build/Cargo.toml"
        || path.starts_with("scripts/build/tests/fixtures/thread_sanitizer_startup/")
    {
        return true;
    }
    let Some(module) = path.strip_prefix("scripts/build/src/") else {
        return false;
    };
    EXECUTION_MODULES.iter().any(|name| {
        module == *name
            || name.strip_suffix(".rs").is_some_and(|stem| {
                module
                    .strip_prefix(stem)
                    .is_some_and(|suffix| suffix.starts_with('/'))
            })
    })
}

const EXECUTION_MODULES: &[&str] = &[
    "thread_sanitizer.rs",
    "thread_sanitizer_selection.rs",
    "cli.rs",
    "context.rs",
    "report.rs",
    "error.rs",
    "process.rs",
    "main.rs",
    "git.rs",
    "plan.rs",
    "ci_tests.rs",
    "affected.rs",
    "release_impact.rs",
    "release_scope.rs",
    "published_history.rs",
    "workspace.rs",
    "package_manifest.rs",
    "version.rs",
];

/// Keeps explicit local invocations complete while refusing malformed CI triggers.
/// Example: `let plan = execution_plan(|key| std::env::var(key).ok())?;`.
pub(crate) fn execution_plan(
    lookup: impl Fn(&str) -> Option<String>,
) -> crate::error::BuildResult<Option<crate::plan::CiPlan>> {
    let trigger = crate::plan::CiTrigger::from_lookup(&lookup);
    let hosted = ["CI", "GITHUB_ACTIONS"]
        .iter()
        .any(|key| lookup(key).is_some_and(|value| value != "false" && !value.is_empty()));
    if trigger.event.is_empty() && !hosted {
        return Ok(None);
    }
    crate::plan::plan_for(&trigger).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standalone_command_is_full_but_ci_missing_or_invalid_event_fails() {
        assert_eq!(execution_plan(|_| None).unwrap(), None);
        for key in ["CI", "GITHUB_ACTIONS", "GITHUB_EVENT_NAME"] {
            assert!(execution_plan(|name| (name == key).then(|| "true".to_owned())).is_err());
        }
        let plan = execution_plan(|name| match name {
            "GITHUB_EVENT_NAME" => Some("pull_request".into()),
            "MARTENSITE_CI_BASE_SHA" => Some("a".repeat(40)),
            _ => None,
        })
        .unwrap();
        assert_eq!(
            plan,
            Some(crate::plan::CiPlan::PullRequest {
                base: "a".repeat(40)
            })
        );
    }

    #[test]
    fn changed_driver_dependencies_select_controls_without_unrelated_lockfile_fanout() {
        let changed = [ChangedFile {
            path: "Cargo.lock".into(),
            deleted: false,
            untracked: false,
        }];
        assert!(select(&BTreeSet::new(), &changed).is_empty());
        assert_eq!(
            select(
                &BTreeSet::from([crate::context::BUILD_TOOL_PACKAGE.into()]),
                &changed
            ),
            CONTROLS
        );
    }

    #[test]
    fn receipt_control_ids_are_stable_and_unique() {
        let ids: Vec<_> = CONTROLS.into_iter().map(Control::id).collect();
        assert_eq!(
            ids,
            [
                "cache-2d",
                "cache-3d",
                "capture",
                "groups",
                "face-input",
                "face-certificate",
                "open-extrusion",
                "vertex-neighborhood",
                "surface-sketch",
                "font-text-capture"
            ]
        );
        assert_eq!(ids.iter().collect::<BTreeSet<_>>().len(), CONTROLS.len());
    }

    #[test]
    fn surface_sketch_owner_selects_control_without_unrelated_leaf_fanout() {
        assert_eq!(
            select(&BTreeSet::from(["martensite-tksketch".into()]), &[]),
            [Control::SurfaceSketch]
        );
        assert!(select(&BTreeSet::from(["martensite-tkdeobj".into()]), &[]).is_empty());
        assert_eq!(
            select(&BTreeSet::from(["martensite-tkmath".into()]), &[]),
            [Control::OpenExtrusion, Control::VertexNeighborhood]
        );
    }

    #[test]
    fn every_owner_selects_exactly_its_controls() {
        for control in CONTROLS {
            let packages = BTreeSet::from([control.owner().to_owned()]);
            let expected: Vec<_> = CONTROLS
                .into_iter()
                .filter(|item| item.owner() == control.owner())
                .collect();
            assert_eq!(select(&packages, &[]), expected);
        }
    }

    #[test]
    fn execution_changes_select_all_but_docs_and_unrelated_driver_changes_do_not() {
        for (path, expected) in [
            (".github/workflows/martensite.yml", true),
            ("scripts/build/src/thread_sanitizer.rs", true),
            (
                "scripts/build/tests/fixtures/thread_sanitizer_startup/src/main.rs",
                true,
            ),
            ("scripts/build/src/lint.rs", false),
            ("scripts/build/src/published_history/receipt.rs", true),
            ("scripts/build/src/process/bounded.rs", true),
            ("scripts/build/src/process_extra.rs", false),
            ("docs/features/ci-build-driver/README.md", false),
        ] {
            let changed = [ChangedFile {
                path: path.into(),
                deleted: false,
                untracked: false,
            }];
            assert_eq!(
                !select(&BTreeSet::new(), &changed).is_empty(),
                expected,
                "{path}"
            );
        }
    }
}

#[cfg(test)]
#[test]
fn font_capture_selects_only_its_affected_owner() {
    assert_eq!(
        select(&BTreeSet::from(["martensite-tkv3d".into()]), &[]),
        [Control::FontTextCapture]
    );
    assert!(
        !select(&BTreeSet::from(["martensite-tksketch".into()]), &[])
            .contains(&Control::FontTextCapture)
    );
}
