//! Reads one released version's section out of the Keep-a-Changelog file.

use crate::error::{BuildError, BuildResult};

/// The `## [X.Y.Z]` section of `CHANGELOG.md` for one release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseNotes {
    /// The `### Added`/`### Fixed`/... headings in the section, in order.
    pub headings: Vec<String>,
    /// True when any line in the section carries the `BREAKING` marker.
    pub breaking: bool,
    /// The section body without its `## [X.Y.Z]` heading line.
    pub body: String,
}

/// Extracts the notes for `version` from the changelog text.
///
/// ```ignore
/// let notes = release_notes(&std::fs::read_to_string("CHANGELOG.md")?, "1.4.0")?;
/// ```
pub fn release_notes(changelog: &str, version: &str) -> BuildResult<ReleaseNotes> {
    let mut lines = changelog
        .lines()
        .skip_while(|line| !is_version_heading(line, version));
    if lines.next().is_none() {
        return Err(BuildError::ReleaseRule(format!(
            "CHANGELOG.md has no `## [{version}]` section; expected one describing this release"
        )));
    }
    let body: Vec<&str> = lines.take_while(|line| !line.starts_with("## ")).collect();
    Ok(ReleaseNotes {
        headings: body
            .iter()
            .filter_map(|line| line.strip_prefix("### "))
            .map(|h| h.trim().to_owned())
            .collect(),
        breaking: body.iter().any(|line| line.contains("BREAKING")),
        body: body.join("\n").trim().to_owned(),
    })
}

/// Matches `## [1.4.0]` and `## [1.4.0] - 2026-10-14`.
fn is_version_heading(line: &str, version: &str) -> bool {
    line.strip_prefix("## [")
        .and_then(|rest| rest.split_once(']'))
        .is_some_and(|(heading_version, _)| heading_version == version)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHANGELOG: &str = "# Changelog\n\n## [Unreleased]\n\n### Added\n- next\n\n\
        ## [1.4.0] - 2026-10-14\n\n### Fixed\n- a fix (#12)\n\n### Removed\n- BREAKING: old api (#13)\n\n\
        ## [1.3.0]\n\n### Added\n- older\n";

    #[test]
    fn section_stops_at_the_next_version() {
        let notes = release_notes(CHANGELOG, "1.4.0").unwrap();
        assert_eq!(notes.headings, ["Fixed", "Removed"]);
        assert!(notes.breaking);
        assert!(notes.body.starts_with("### Fixed"));
        assert!(!notes.body.contains("older"));
    }

    #[test]
    fn version_prefix_does_not_match_a_longer_version() {
        assert!(release_notes(CHANGELOG, "1.4").is_err());
    }

    #[test]
    fn missing_section_names_the_version() {
        let error = release_notes(CHANGELOG, "9.9.9").unwrap_err();
        assert!(error.to_string().contains("## [9.9.9]"));
    }
}
