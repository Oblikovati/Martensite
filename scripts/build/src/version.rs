//! Semantic-versioning gate for a release: the tag, the crate versions and
//! the changelog must agree, and the bump must be at least what the changes
//! demand.

use semver::Version;

use crate::changelog::ReleaseNotes;
use crate::error::{BuildError, BuildResult};
use crate::workspace::WorkspacePackage;

/// How far a version moved, or must move, from the previous release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BumpLevel {
    /// `x.y.Z`: fixes only.
    Patch,
    /// `x.Y.0`: additive or deprecating changes.
    Minor,
    /// `X.0.0`: breaking changes.
    Major,
}

/// Parses a release tag `vX.Y.Z[-pre]`; build metadata is refused because it
/// does not order releases.
///
/// ```ignore
/// let version = parse_release_tag("v1.4.0")?;
/// ```
pub fn parse_release_tag(tag: &str) -> BuildResult<Version> {
    let shape = "expected `vMAJOR.MINOR.PATCH[-PRERELEASE]`";
    let text = tag.strip_prefix('v').ok_or_else(|| {
        BuildError::ReleaseRule(format!("tag {tag:?} lacks the `v` prefix; {shape}"))
    })?;
    let version = Version::parse(text).map_err(|error| {
        BuildError::ReleaseRule(format!("tag {tag:?} is not semver ({error}); {shape}"))
    })?;
    if !version.build.is_empty() {
        return Err(BuildError::ReleaseRule(format!(
            "tag {tag:?} carries build metadata; {shape}"
        )));
    }
    Ok(version)
}

/// Parses a version typed by a person, with or without the `v` prefix.
///
/// ```ignore
/// assert_eq!(parse_version_argument("1.4.0")?, parse_version_argument("v1.4.0")?);
/// ```
pub fn parse_version_argument(text: &str) -> BuildResult<Version> {
    parse_release_tag(&format!("v{}", text.strip_prefix('v').unwrap_or(text)))
}

/// The highest final (non-prerelease) release below `version`, ignoring tags
/// that are not release tags. Release candidates never count as a baseline.
pub fn previous_release(tags: &[String], version: &Version) -> Option<Version> {
    tags.iter()
        .filter_map(|tag| parse_release_tag(tag).ok())
        .filter(|tagged| tagged.pre.is_empty() && tagged < version)
        .max()
}

/// Every releasable package must carry exactly the release version.
pub fn check_package_versions(
    packages: &[&WorkspacePackage],
    version: &Version,
) -> BuildResult<()> {
    if packages.is_empty() {
        return Err(BuildError::ReleaseRule(
            "the workspace has no releasable package".into(),
        ));
    }
    let mismatched = mismatched_packages(packages, version);
    if mismatched.is_empty() {
        return Ok(());
    }
    Err(BuildError::ReleaseRule(format!(
        "package versions {mismatched:?} differ from the release; expected every package at {version}"
    )))
}

fn mismatched_packages(packages: &[&WorkspacePackage], version: &Version) -> Vec<String> {
    let mismatched = packages
        .iter()
        .filter(|package| &package.version != version);
    mismatched
        .map(|package| format!("{} {}", package.name, package.version))
        .collect()
}

/// Refuses a bump smaller than the changelog's headings require.
pub fn check_bump(
    previous: Option<&Version>,
    version: &Version,
    notes: &ReleaseNotes,
) -> BuildResult<()> {
    let required = required_bump(notes)?;
    let Some(previous) = previous else {
        return Ok(());
    };
    let required = relaxed_for_initial_development(required, previous);
    let actual = actual_bump(previous, version)?;
    if actual >= required {
        return Ok(());
    }
    Err(BuildError::ReleaseRule(format!(
        "{previous} -> {version} is a {actual:?} bump but the changelog requires {required:?}"
    )))
}

/// The smallest bump the changelog section allows.
pub fn required_bump(notes: &ReleaseNotes) -> BuildResult<BumpLevel> {
    if notes.headings.is_empty() {
        return Err(BuildError::ReleaseRule(
            "the changelog section has no `### <kind>` heading; expected Added/Changed/Deprecated/Removed/Fixed/Security"
                .into(),
        ));
    }
    let levels = notes.headings.iter().map(|heading| heading_bump(heading));
    let highest = levels
        .collect::<BuildResult<Vec<_>>>()?
        .into_iter()
        .max()
        .unwrap_or(BumpLevel::Patch);
    Ok(if notes.breaking {
        BumpLevel::Major
    } else {
        highest
    })
}

fn heading_bump(heading: &str) -> BuildResult<BumpLevel> {
    match heading {
        "Fixed" | "Security" => Ok(BumpLevel::Patch),
        "Added" | "Changed" | "Deprecated" => Ok(BumpLevel::Minor),
        "Removed" => Ok(BumpLevel::Major),
        other => Err(BuildError::ReleaseRule(format!(
            "unknown changelog heading {other:?}; expected Added/Changed/Deprecated/Removed/Fixed/Security"
        ))),
    }
}

/// Before 1.0.0 cargo treats `0.x.y` and `0.x.z` as compatible, so each level moves
/// down one: a breaking change (`Removed` or `BREAKING`) needs a minor bump and an
/// additive change (`Added`, `Deprecated`, non-breaking `Changed`) a patch bump.
/// From 1.0.0 the headings' own levels apply.
fn relaxed_for_initial_development(required: BumpLevel, previous: &Version) -> BumpLevel {
    if previous.major != 0 {
        return required;
    }
    match required {
        BumpLevel::Major => BumpLevel::Minor,
        BumpLevel::Minor | BumpLevel::Patch => BumpLevel::Patch,
    }
}

fn actual_bump(previous: &Version, version: &Version) -> BuildResult<BumpLevel> {
    let level = if version.major != previous.major {
        BumpLevel::Major
    } else if version.minor != previous.minor {
        BumpLevel::Minor
    } else if version.patch != previous.patch {
        BumpLevel::Patch
    } else {
        // previous_release only returns strictly lower final versions, so
        // equal cores mean a caller passed an inconsistent baseline.
        return Err(BuildError::ReleaseRule(format!(
            "{version} does not advance past {previous}"
        )));
    };
    Ok(level)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn notes(headings: &[&str], breaking: bool) -> ReleaseNotes {
        ReleaseNotes {
            headings: headings.iter().map(|h| (*h).to_owned()).collect(),
            breaking,
            body: String::new(),
        }
    }

    fn v(text: &str) -> Version {
        Version::parse(text).unwrap()
    }

    fn package(name: &str, version: &str) -> WorkspacePackage {
        WorkspacePackage {
            name: name.into(),
            version: v(version),
            dir: name.into(),
            path_dependencies: vec![],
        }
    }

    #[test]
    fn release_tag_shape_is_enforced() {
        assert_eq!(parse_release_tag("v1.2.3-rc.1").unwrap(), v("1.2.3-rc.1"));
        assert!(parse_release_tag("1.2.3").is_err());
        assert!(parse_release_tag("v1.2").is_err());
        assert!(parse_release_tag("v1.2.3+build.5").is_err());
    }

    #[test]
    fn version_argument_accepts_an_optional_prefix() {
        assert_eq!(parse_version_argument("1.4.0").unwrap(), v("1.4.0"));
        assert_eq!(parse_version_argument("v1.4.0").unwrap(), v("1.4.0"));
        assert!(parse_version_argument("").is_err());
    }

    #[test]
    fn previous_release_skips_candidates_and_foreign_tags() {
        let tags = ["v1.2.0", "v1.3.0-rc.1", "vnext", "v1.3.0", "v2.0.0"].map(String::from);
        assert_eq!(previous_release(&tags, &v("1.3.1")), Some(v("1.3.0")));
        assert_eq!(previous_release(&tags, &v("1.3.0")), Some(v("1.2.0")));
        assert_eq!(previous_release(&tags, &v("0.1.0")), None);
    }

    #[test]
    fn added_requires_minor() {
        let added = notes(&["Fixed", "Added"], false);
        assert!(check_bump(Some(&v("1.2.0")), &v("1.2.1"), &added).is_err());
        assert!(check_bump(Some(&v("1.2.0")), &v("1.3.0"), &added).is_ok());
    }

    #[test]
    fn additive_change_needs_only_a_patch_before_one_zero() {
        for heading in ["Added", "Changed", "Deprecated"] {
            let additive = notes(&["Fixed", heading], false);
            assert!(check_bump(Some(&v("0.3.0")), &v("0.3.1"), &additive).is_ok());
            assert!(check_bump(Some(&v("0.3.0")), &v("0.4.0"), &additive).is_ok());
        }
    }

    #[test]
    fn removal_needs_a_minor_before_one_zero() {
        let removed = notes(&["Removed"], false);
        assert!(check_bump(Some(&v("0.3.0")), &v("0.3.1"), &removed).is_err());
        assert!(check_bump(Some(&v("0.3.0")), &v("0.4.0"), &removed).is_ok());
    }

    #[test]
    fn breaking_requires_major_after_one_zero_and_minor_before() {
        let breaking = notes(&["Changed"], true);
        assert!(check_bump(Some(&v("1.2.0")), &v("1.3.0"), &breaking).is_err());
        assert!(check_bump(Some(&v("1.2.0")), &v("2.0.0"), &breaking).is_ok());
        assert!(check_bump(Some(&v("0.2.7")), &v("0.3.0"), &breaking).is_ok());
        assert!(check_bump(Some(&v("0.2.7")), &v("0.2.8"), &breaking).is_err());
    }

    #[test]
    fn first_release_only_needs_valid_notes() {
        assert!(check_bump(None, &v("0.1.0"), &notes(&["Added"], false)).is_ok());
        assert!(check_bump(None, &v("0.1.0"), &notes(&[], false)).is_err());
    }

    #[test]
    fn unknown_heading_is_named() {
        let error = required_bump(&notes(&["Misc"], false)).unwrap_err();
        assert!(error.to_string().contains("\"Misc\""));
    }

    #[test]
    fn package_versions_must_all_match() {
        let (a, b) = (package("a", "1.0.0"), package("b", "0.9.0"));
        assert!(check_package_versions(&[&a], &v("1.0.0")).is_ok());
        let error = check_package_versions(&[&a, &b], &v("1.0.0")).unwrap_err();
        assert!(error.to_string().contains("b 0.9.0"));
        assert!(check_package_versions(&[], &v("1.0.0")).is_err());
    }
}
