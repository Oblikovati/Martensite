//! Fetch immutable historical assets, never substitute a newly packaged local sibling.
use crate::{
    archive,
    context::BuildContext,
    error::{BuildError, BuildResult},
    process::CommandSpec,
    version::parse_release_tag,
};
use semver::Version;
use std::path::{Path, PathBuf};

pub(super) fn published(
    context: &BuildContext<'_>,
    release: &Version,
    archive: &str,
    directory: &Path,
) -> BuildResult<(PathBuf, String)> {
    let tags = prior_tags(context, release)?;
    for (_, tag) in tags {
        if !has_asset(context, &tag, archive)? {
            continue;
        }
        download(context, &tag, archive, directory)?;
        let path = directory.join(archive);
        archive::verify_checksum(&path)?;
        return Ok((path, tag));
    }
    Err(BuildError::ReleaseRule(format!(
        "historical archive {archive:?}; expected a checksum-verified asset in a prior final release"
    )))
}

fn has_asset(context: &BuildContext<'_>, tag: &str, name: &str) -> BuildResult<bool> {
    let text = context.runner.capture(
        &CommandSpec::new("gh", context.root()).args(["release", "view", tag, "--json", "assets"]),
    )?;
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|cause| {
        BuildError::ReleaseRule(format!(
            "release {tag} assets {text:?}: {cause}; expected asset JSON"
        ))
    })?;
    let assets = value["assets"]
        .as_array()
        .ok_or_else(|| BuildError::ReleaseRule(format!("release {tag} assets; expected array")))?;
    Ok(assets
        .iter()
        .any(|asset| asset["name"].as_str() == Some(name)))
}

fn prior_tags(
    context: &BuildContext<'_>,
    release: &Version,
) -> BuildResult<Vec<(Version, String)>> {
    let mut tags: Vec<_> = context
        .git
        .release_tags()?
        .into_iter()
        .filter_map(|tag| parse_release_tag(&tag).ok().map(|version| (version, tag)))
        .filter(|(version, _)| version.pre.is_empty() && version < release)
        .collect();
    tags.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(tags)
}

fn download(
    context: &BuildContext<'_>,
    tag: &str,
    archive: &str,
    directory: &Path,
) -> BuildResult<()> {
    context
        .runner
        .run(&CommandSpec::new("gh", context.root()).args([
            "release",
            "download",
            tag,
            "--pattern",
            archive,
            "--pattern",
            &format!("{archive}.sha256"),
            "--dir",
            &directory.to_string_lossy(),
        ]))
}
