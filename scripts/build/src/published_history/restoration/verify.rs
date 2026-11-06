//! Bind restoration bytes to a current catalog and a separately recorded asset (#358).
use super::super::{
    PublishedArchive, PublishedRelease, archives, invalid,
    inventory::{self, Asset, HistoryReceipt},
};
use super::{RestorationAnchor, RestorationManifest, bundle};
use crate::{
    archive,
    context::BuildContext,
    error::{BuildResult, io_error},
};
use semver::Version;
use serde::Deserialize;
use std::{collections::BTreeMap, path::Path};

#[derive(Deserialize)]
struct RetainedCatalog {
    publication_history: HistoryReceipt,
}

pub(super) fn collect_bundle(
    context: &BuildContext<'_>,
    release: &PublishedRelease,
    assets: &[Asset],
    asset: &Asset,
    directory: &Path,
    archives: &mut BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    require_size(asset)?;
    let path = inventory::download_pair(context, release, assets, asset, directory)?;
    let mut members = bundle::members(&path)?;
    let manifest = read_manifest(&mut members)?;
    let receipt = retained_catalog(context, release, assets, directory, &manifest)?;
    require_members(&manifest, &receipt, &members)?;
    let anchor = anchor(release, asset, &path, &manifest)?;
    admit_members(directory, manifest, members, anchor, archives)
}

fn admit_members(
    directory: &Path,
    manifest: RestorationManifest,
    members: BTreeMap<String, Vec<u8>>,
    anchor: RestorationAnchor,
    archives: &mut BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    let folder = directory.join(format!(
        "restored-{}-{}",
        anchor.release_id, anchor.asset_id
    ));
    std::fs::create_dir_all(&folder).map_err(|cause| io_error(&folder, cause))?;
    for entry in manifest.entries {
        let recovered = recovered_archive(&folder, entry.historical_origin, &members, &anchor)?;
        inventory::admit(archives, recovered)?;
    }
    Ok(())
}

fn require_size(asset: &Asset) -> BuildResult<()> {
    if asset.size > bundle::MAX_BYTES {
        return Err(invalid(
            &asset.name,
            "restoration container at most 128 MiB",
        ));
    }
    Ok(())
}

fn read_manifest(members: &mut BTreeMap<String, Vec<u8>>) -> BuildResult<RestorationManifest> {
    let bytes = members
        .remove("restoration.json")
        .ok_or_else(|| invalid("restoration.json", "one regular restoration manifest"))?;
    if bytes.len() > 1024 * 1024 {
        return Err(invalid(
            &bytes.len().to_string(),
            "restoration manifest at most 1 MiB",
        ));
    }
    let manifest: RestorationManifest = serde_json::from_slice(&bytes)
        .map_err(|cause| invalid(&cause.to_string(), "restoration schema 1 manifest"))?;
    if manifest.schema != 1 || manifest.entries.is_empty() {
        return Err(invalid(
            &manifest.schema.to_string(),
            "schema 1 with nonempty entries",
        ));
    }
    Ok(manifest)
}

fn retained_catalog(
    context: &BuildContext<'_>,
    release: &PublishedRelease,
    assets: &[Asset],
    directory: &Path,
    manifest: &RestorationManifest,
) -> BuildResult<HistoryReceipt> {
    let asset = catalog_asset(release, assets)?;
    let path = inventory::download_pair(context, release, assets, asset, directory)?;
    read_receipt(&path, &manifest.catalog_sha256)
}

fn catalog_asset<'a>(release: &PublishedRelease, assets: &'a [Asset]) -> BuildResult<&'a Asset> {
    let name = format!(
        "martensite-{}-packages.json",
        release.tag_name.trim_start_matches('v')
    );
    let asset = assets
        .iter()
        .find(|asset| asset.name == name)
        .ok_or_else(|| invalid(&name, "same-release published package catalog"))?;
    if asset.size > 1024 * 1024 {
        return Err(invalid(&name, "catalog at most 1 MiB"));
    }
    Ok(asset)
}

fn read_receipt(path: &Path, expected: &str) -> BuildResult<HistoryReceipt> {
    if archive::sha256_hex(path)? != expected {
        return Err(invalid(
            &path.display().to_string(),
            "catalog bytes pinned by restoration manifest",
        ));
    }
    let bytes = std::fs::read(path).map_err(|cause| io_error(path, cause))?;
    let catalog: RetainedCatalog = serde_json::from_slice(&bytes)
        .map_err(|cause| invalid(&cause.to_string(), "retained publication receipt"))?;
    Ok(catalog.publication_history)
}

fn require_members(
    manifest: &RestorationManifest,
    receipt: &HistoryReceipt,
    members: &BTreeMap<String, Vec<u8>>,
) -> BuildResult<()> {
    if manifest.entries.len() != receipt.archives.len() || members.len() != manifest.entries.len() {
        return Err(invalid(
            &members.len().to_string(),
            "exact closed retained-origin membership",
        ));
    }
    let mut names = std::collections::BTreeSet::new();
    for entry in &manifest.entries {
        require_origin(&entry.historical_origin, receipt, members, &mut names)?;
    }
    Ok(())
}

fn require_origin<'a>(
    origin: &'a archives::ArchiveOrigin,
    receipt: &HistoryReceipt,
    members: &BTreeMap<String, Vec<u8>>,
    names: &mut std::collections::BTreeSet<&'a String>,
) -> BuildResult<()> {
    let valid = origin.restoration.is_none()
        && receipt
            .archives
            .iter()
            .any(|retained| retained.same_publication(origin))
        && names.insert(&origin.archive)
        && members.contains_key(&format!("archives/{}", origin.archive));
    if !valid {
        return Err(invalid(
            &origin.archive,
            "unique exact historical receipt origin and payload",
        ));
    }
    Ok(())
}

fn anchor(
    release: &PublishedRelease,
    asset: &Asset,
    path: &Path,
    manifest: &RestorationManifest,
) -> BuildResult<RestorationAnchor> {
    Ok(RestorationAnchor {
        release_id: release.id,
        release_tag: release.tag_name.clone(),
        asset_id: asset.id,
        asset_name: asset.name.clone(),
        sha256: archive::sha256_hex(path)?,
        catalog_sha256: manifest.catalog_sha256.clone(),
    })
}

fn recovered_archive(
    folder: &Path,
    mut origin: archives::ArchiveOrigin,
    members: &BTreeMap<String, Vec<u8>>,
    anchor: &RestorationAnchor,
) -> BuildResult<PublishedArchive> {
    let (name, version) = archives::identity(&origin.archive)?;
    let bytes = &members[&format!("archives/{}", origin.archive)];
    let path = folder.join(&origin.archive);
    std::fs::write(&path, bytes).map_err(|cause| io_error(&path, cause))?;
    require_payload(&path, &origin, &name, &version)?;
    archive::write_checksum(&path)?;
    origin.restoration = Some(anchor.clone());
    Ok(PublishedArchive {
        name,
        version,
        path,
        origins: vec![origin],
    })
}

fn require_payload(
    path: &Path,
    origin: &archives::ArchiveOrigin,
    name: &str,
    version: &Version,
) -> BuildResult<()> {
    if archive::sha256_hex(path)? != origin.sha256 {
        return Err(invalid(&origin.archive, "exact retained archive digest"));
    }
    let expected = format!("{name}-{version}/Cargo.toml");
    super::bounded::inner_archive(path, &expected)?;
    let manifest = archives::manifest(path, name, version)?;
    if archives::manifest_hash(&manifest) != origin.manifest_sha256 {
        return Err(invalid(
            &origin.archive,
            "exact retained normalized manifest digest",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::fixtures::archive_bytes;
    use super::*;
    use sha2::{Digest, Sha256};

    fn origin(bytes: &[u8]) -> archives::ArchiveOrigin {
        archives::ArchiveOrigin {
            release_id: 1,
            release_tag: "v0.3.6".into(),
            asset_id: 2,
            archive: "martensite-math-0.3.5.crate".into(),
            sha256: format!("{:x}", Sha256::digest(bytes)),
            manifest_sha256: "0".repeat(64),
            restoration: None,
        }
    }

    #[test]
    fn normalized_manifest_and_identity_are_required_after_payload_digest_matches() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("fake.crate");
        let bytes = archive_bytes("martensite-math", "0.3.5", "");
        std::fs::write(&path, &bytes).unwrap();
        let mut claimed = origin(&bytes);
        let version = Version::parse("0.3.5").unwrap();
        assert!(require_payload(&path, &claimed, "martensite-math", &version).is_err());
        let manifest = archives::manifest(&path, "martensite-math", &version).unwrap();
        claimed.manifest_sha256 = archives::manifest_hash(&manifest);
        assert!(require_payload(&path, &claimed, "martensite-math", &version).is_ok());
        assert!(require_payload(&path, &claimed, "martensite-other", &version).is_err());
    }
}
