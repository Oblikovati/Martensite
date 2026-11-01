//! Retired publication origins retain exact bytes under a distinct live anchor (#358).
mod bounded;
mod bundle;
#[cfg(test)]
mod tests;
mod verify;

use super::{PublishedArchive, PublishedRelease, archives::ArchiveOrigin, inventory::Asset};
use crate::{context::BuildContext, error::BuildResult};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};

const BUNDLE_NAME: &str = "martensite-historical-package-restoration-1.tar.gz";

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct RestorationAnchor {
    pub release_id: u64,
    pub release_tag: String,
    pub asset_id: u64,
    pub asset_name: String,
    pub sha256: String,
    pub catalog_sha256: String,
}

#[derive(Deserialize)]
struct RestorationManifest {
    schema: u32,
    catalog_sha256: String,
    entries: Vec<RestorationEntry>,
}

#[derive(Deserialize)]
struct RestorationEntry {
    historical_origin: ArchiveOrigin,
}

pub(super) fn collect(
    context: &BuildContext<'_>,
    release: &PublishedRelease,
    assets: &[Asset],
    directory: &Path,
    archives: &mut BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    let Some(asset) = assets.iter().find(|asset| asset.name == BUNDLE_NAME) else {
        return Ok(());
    };
    verify::collect_bundle(context, release, assets, asset, directory, archives)
}
