//! GitHub publication evidence is independent of local Git tags (#283).
mod api;
mod archives;
mod catalogs;
mod inventory;

use crate::{
    context::BuildContext,
    error::{BuildError, BuildResult},
    version::parse_release_tag,
};
use serde::{Deserialize, Serialize};

pub(crate) use archives::manifest as archive_manifest;
pub(crate) use archives::{ArchiveOrigin, PublishedArchive};
pub(crate) use inventory::{AssetHistory, HistoryReceipt};

pub(super) const RELEASES: &str = "repos/{owner}/{repo}/releases?per_page=100";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct PublishedRelease {
    pub id: u64,
    pub tag_name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub published_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct ReleaseIndex {
    pub releases: Vec<PublishedRelease>,
}

impl ReleaseIndex {
    /// Reads all published releases; e.g. `let index = ReleaseIndex::load(context)?;`.
    pub fn load(context: &BuildContext<'_>) -> BuildResult<Self> {
        let releases: Vec<PublishedRelease> = api::pages(context, RELEASES)?;
        Ok(Self {
            releases: published_releases(releases)?,
        })
    }

    /// Supplies only actual published tags to source/version policy; e.g. `index.final_tags()`.
    pub fn final_tags(&self) -> Vec<String> {
        self.releases
            .iter()
            .filter(|release| {
                !release.prerelease
                    && parse_release_tag(&release.tag_name)
                        .is_ok_and(|version| version.pre.is_empty())
            })
            .map(|release| release.tag_name.clone())
            .collect()
    }

    /// Binds release jobs to the planner's inventory; e.g. `index.require_pin(Some(hash))?;`.
    pub fn require_pin(&self, expected: Option<&str>) -> BuildResult<()> {
        let Some(expected) = expected.filter(|value| !value.is_empty()) else {
            return Ok(());
        };
        let actual = self.fingerprint()?;
        if actual != expected {
            return Err(invalid(
                &actual,
                &format!("pinned published-release history {expected}"),
            ));
        }
        Ok(())
    }

    /// Hashes a deterministic published-release index; e.g. `index.fingerprint()?`.
    pub fn fingerprint(&self) -> BuildResult<String> {
        fingerprint(self)
    }
}

fn published_releases(releases: Vec<PublishedRelease>) -> BuildResult<Vec<PublishedRelease>> {
    let mut ids = std::collections::BTreeSet::new();
    let mut tags = std::collections::BTreeSet::new();
    let mut published = Vec::new();
    for release in releases {
        if release.id == 0 || !ids.insert(release.id) {
            return Err(invalid(&release.tag_name, "unique positive release IDs"));
        }
        if release.draft {
            continue;
        }
        validate_published_release(&release, &mut tags)?;
        published.push(release);
    }
    published.sort_by_key(|release| release.id);
    Ok(published)
}

fn validate_published_release(
    release: &PublishedRelease,
    tags: &mut std::collections::BTreeSet<String>,
) -> BuildResult<()> {
    if release.published_at.as_deref().is_none_or(str::is_empty) {
        return Err(invalid(&release.tag_name, "a published timestamp"));
    }
    if release.tag_name.is_empty() || !tags.insert(release.tag_name.clone()) {
        return Err(invalid(
            &release.tag_name,
            "unique nonempty published release tags",
        ));
    }
    Ok(())
}

pub(super) fn fingerprint(value: &impl Serialize) -> BuildResult<String> {
    use sha2::{Digest, Sha256};
    let bytes = serde_json::to_vec(value)
        .map_err(|cause| invalid(&cause.to_string(), "serializable publication evidence"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub(super) fn invalid(value: &str, expected: &str) -> BuildError {
    BuildError::ReleaseRule(format!(
        "publication evidence {value:?}; expected {expected}"
    ))
}

#[cfg(test)]
pub(crate) mod fixtures;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod provenance_tests;
