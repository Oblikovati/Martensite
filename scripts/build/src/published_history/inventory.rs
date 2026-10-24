//! A completed inventory owns downloaded bytes until publication selection finishes.
use super::{
    PublishedRelease, ReleaseIndex, api,
    archives::{self, ArchiveOrigin, PublishedArchive},
    catalogs, fingerprint, invalid,
};
use crate::{
    archive,
    context::BuildContext,
    error::{BuildResult, io_error},
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct Asset {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub state: String,
    pub digest: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct HistoryReceipt {
    pub index_sha256: String,
    pub assets_sha256: String,
    pub archives: Vec<ArchiveOrigin>,
}

pub(crate) struct AssetHistory {
    pub receipt: HistoryReceipt,
    pub archives: BTreeMap<(String, Version), PublishedArchive>,
    _directory: tempfile::TempDir,
}

impl AssetHistory {
    /// Verifies complete publication history; e.g. `AssetHistory::load(context, &index)?`.
    pub fn load(context: &BuildContext<'_>, index: &ReleaseIndex) -> BuildResult<Self> {
        let directory = tempfile::tempdir()
            .map_err(|cause| io_error(Path::new("publication history"), cause))?;
        let assets_by_release = listings(context, index)?;
        let archives = verified_archives(context, index, &assets_by_release, directory.path())?;
        catalogs::validate(context, index, &assets_by_release, &archives)?;
        require_stable(context, index, &assets_by_release)?;
        let receipt = receipt(index, &assets_by_release, &archives)?;
        Ok(Self {
            receipt,
            archives,
            _directory: directory,
        })
    }
}

fn require_stable(
    context: &BuildContext<'_>,
    index: &ReleaseIndex,
    assets: &BTreeMap<u64, Vec<Asset>>,
) -> BuildResult<()> {
    if ReleaseIndex::load(context)? != *index || listings(context, index)? != *assets {
        return Err(invalid(
            "changing release/asset inventory",
            "a stable complete publication snapshot",
        ));
    }
    Ok(())
}

fn verified_archives(
    context: &BuildContext<'_>,
    index: &ReleaseIndex,
    listings: &BTreeMap<u64, Vec<Asset>>,
    directory: &Path,
) -> BuildResult<BTreeMap<(String, Version), PublishedArchive>> {
    let mut archives = BTreeMap::new();
    for release in &index.releases {
        collect_archives(
            context,
            release,
            &listings[&release.id],
            directory,
            &mut archives,
        )?;
    }
    Ok(archives)
}

fn receipt(
    index: &ReleaseIndex,
    assets: &BTreeMap<u64, Vec<Asset>>,
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<HistoryReceipt> {
    Ok(HistoryReceipt {
        index_sha256: index.fingerprint()?,
        assets_sha256: fingerprint(assets)?,
        archives: archives
            .values()
            .flat_map(|archive| archive.origins.clone())
            .collect(),
    })
}

fn listings(
    context: &BuildContext<'_>,
    index: &ReleaseIndex,
) -> BuildResult<BTreeMap<u64, Vec<Asset>>> {
    let mut result = BTreeMap::new();
    let mut ids = std::collections::BTreeSet::new();
    for release in &index.releases {
        let endpoint = format!(
            "repos/{{owner}}/{{repo}}/releases/{}/assets?per_page=100",
            release.id
        );
        let mut assets: Vec<Asset> = api::pages(context, &endpoint)?;
        validate_assets(&assets, &mut ids)?;
        assets.sort_by_key(|asset| asset.id);
        result.insert(release.id, assets);
    }
    Ok(result)
}

fn validate_assets(assets: &[Asset], ids: &mut std::collections::BTreeSet<u64>) -> BuildResult<()> {
    let mut names = std::collections::BTreeSet::new();
    for asset in assets {
        if asset.name.is_empty()
            || asset.name.contains(['/', '\\', ':'])
            || asset.state != "uploaded"
            || asset.size == 0
        {
            return Err(invalid(
                &asset.name,
                "an uploaded, nonempty, portable asset",
            ));
        }
        if !names.insert(&asset.name) || !ids.insert(asset.id) {
            return Err(invalid(&asset.name, "unique asset names and IDs"));
        }
    }
    Ok(())
}

fn collect_archives(
    context: &BuildContext<'_>,
    release: &PublishedRelease,
    assets: &[Asset],
    directory: &Path,
    archives: &mut BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    for asset in assets.iter().filter(|asset| asset.name.ends_with(".crate")) {
        admit(
            archives,
            read_archive(context, release, assets, asset, directory)?,
        )?;
    }
    for companion in assets
        .iter()
        .filter(|asset| asset.name.ends_with(".crate.sha256"))
    {
        if !assets
            .iter()
            .any(|asset| asset.name == companion.name.trim_end_matches(".sha256"))
        {
            return Err(invalid(&companion.name, "a matching crate payload"));
        }
    }
    Ok(())
}

fn read_archive(
    context: &BuildContext<'_>,
    release: &PublishedRelease,
    assets: &[Asset],
    asset: &Asset,
    directory: &Path,
) -> BuildResult<PublishedArchive> {
    let (name, version) = archives::identity(&asset.name)?;
    let path = download_pair(context, release, assets, asset, directory)?;
    let manifest = archives::manifest(&path, &name, &version)?;
    let origin = ArchiveOrigin {
        release_id: release.id,
        release_tag: release.tag_name.clone(),
        asset_id: asset.id,
        archive: asset.name.clone(),
        sha256: archive::sha256_hex(&path)?,
        manifest_sha256: archives::manifest_hash(&manifest),
    };
    Ok(PublishedArchive {
        name,
        version,
        path,
        origins: vec![origin],
    })
}

fn admit(
    archives: &mut BTreeMap<(String, Version), PublishedArchive>,
    archive: PublishedArchive,
) -> BuildResult<()> {
    let key = (archive.name.clone(), archive.version.clone());
    if let Some(previous) = archives.get_mut(&key) {
        if previous.origins[0].sha256 != archive.origins[0].sha256 {
            return Err(invalid(
                &archive.origins[0].archive,
                "identical bytes for every published package identity",
            ));
        }
        previous.origins.extend(archive.origins);
        return Ok(());
    }
    archives.insert(key, archive);
    Ok(())
}

pub(super) fn download_pair(
    context: &BuildContext<'_>,
    release: &PublishedRelease,
    assets: &[Asset],
    asset: &Asset,
    directory: &Path,
) -> BuildResult<std::path::PathBuf> {
    let companion_name = format!("{}.sha256", asset.name);
    let companion = assets
        .iter()
        .find(|item| item.name == companion_name)
        .ok_or_else(|| invalid(&asset.name, "a checksum companion"))?;
    let folder = directory.join(release.id.to_string());
    fs::create_dir_all(&folder).map_err(|cause| io_error(&folder, cause))?;
    let path = folder.join(&asset.name);
    write_asset(context, asset, &path)?;
    write_asset(context, companion, &folder.join(&companion.name))?;
    archives::verify_companion(&path)?;
    Ok(path)
}

fn write_asset(context: &BuildContext<'_>, asset: &Asset, path: &Path) -> BuildResult<()> {
    use sha2::{Digest, Sha256};
    let bytes = api::asset(context, asset.id)?;
    let digest = format!("sha256:{:x}", Sha256::digest(&bytes));
    if bytes.len() as u64 != asset.size
        || asset
            .digest
            .as_ref()
            .is_some_and(|expected| expected != &digest)
    {
        return Err(invalid(
            &asset.name,
            "matching published asset size and SHA-256",
        ));
    }
    fs::write(path, bytes).map_err(|cause| io_error(path, cause))
}
