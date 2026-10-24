//! Published receipts distinguish intentional unshipped identities from incomplete uploads.
mod provenance;
use super::{
    PublishedRelease, ReleaseIndex,
    archives::PublishedArchive,
    invalid,
    inventory::{self, Asset, HistoryReceipt},
};
use crate::{
    context::BuildContext,
    error::{BuildResult, io_error},
};
use semver::Version;
use serde::Deserialize;
use std::{collections::BTreeMap, fs};

#[derive(Deserialize)]
struct Catalog {
    #[serde(default = "legacy_schema")]
    schema: u32,
    release: String,
    packages: Vec<Row>,
    publication_history: Option<HistoryReceipt>,
}

#[derive(Deserialize)]
pub(super) struct Row {
    pub(super) name: String,
    pub(super) version: Version,
    pub(super) shipped: bool,
    pub(super) archive_sha256: Option<String>,
    pub(super) publication_reason: Option<crate::publication_record::PublicationReason>,
    pub(super) origins: Option<Vec<super::ArchiveOrigin>>,
}

pub(super) fn validate(
    context: &BuildContext<'_>,
    index: &ReleaseIndex,
    listings: &BTreeMap<u64, Vec<Asset>>,
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    let directory = tempfile::tempdir()
        .map_err(|cause| io_error(std::path::Path::new("publication catalogs"), cause))?;
    for release in &index.releases {
        let assets = &listings[&release.id];
        let expected = format!(
            "martensite-{}-packages.json",
            release.tag_name.trim_start_matches('v')
        );
        let asset = catalog_asset(assets, &expected)?;
        let Some(asset) = asset else {
            continue;
        };
        let path = inventory::download_pair(context, release, assets, asset, directory.path())?;
        let bytes = fs::read(&path).map_err(|cause| io_error(&path, cause))?;
        let catalog: Catalog = serde_json::from_slice(&bytes)
            .map_err(|cause| invalid(&cause.to_string(), "a complete published package catalog"))?;
        validate_catalog(release, &catalog, assets, archives)?;
    }
    Ok(())
}

fn catalog_asset<'a>(assets: &'a [Asset], expected: &str) -> BuildResult<Option<&'a Asset>> {
    let catalog = assets.iter().find(|asset| asset.name == expected);
    let unexpected = assets
        .iter()
        .any(|asset| asset.name.ends_with("-packages.json") && asset.name != expected);
    let orphan = catalog.is_none()
        && assets
            .iter()
            .any(|asset| asset.name == format!("{expected}.sha256"));
    if unexpected || orphan {
        return Err(invalid(
            expected,
            "a matching complete release catalog payload",
        ));
    }
    Ok(catalog)
}

fn validate_catalog(
    release: &PublishedRelease,
    catalog: &Catalog,
    assets: &[Asset],
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    validate_header(release, catalog)?;
    let mut names = std::collections::BTreeSet::new();
    for row in &catalog.packages {
        if !names.insert(&row.name) {
            return Err(invalid(&row.name, "one package row per catalog"));
        }
        validate_row(release, row, assets, archives)?;
        provenance::validate(
            row,
            catalog.schema,
            catalog.publication_history.as_ref(),
            archives,
        )?;
    }
    validate_asset_rows(catalog, assets)?;
    if let Some(receipt) = &catalog.publication_history {
        validate_receipt(receipt, archives)?;
    }
    Ok(())
}

fn validate_header(release: &PublishedRelease, catalog: &Catalog) -> BuildResult<()> {
    if catalog.release != release.tag_name.trim_start_matches('v') {
        return Err(invalid(
            &catalog.release,
            "catalog identity matching its release",
        ));
    }
    if !matches!(catalog.schema, 1 | 2) {
        return Err(invalid(
            &catalog.schema.to_string(),
            "catalog schema 1 or 2",
        ));
    }
    if catalog.schema == 2 && catalog.publication_history.is_none() {
        return Err(invalid(
            &catalog.release,
            "schema-2 publication history receipt",
        ));
    }
    Ok(())
}

fn validate_asset_rows(catalog: &Catalog, assets: &[Asset]) -> BuildResult<()> {
    for asset in assets.iter().filter(|asset| asset.name.ends_with(".crate")) {
        if !catalog
            .packages
            .iter()
            .any(|row| format!("{}-{}.crate", row.name, row.version) == asset.name)
        {
            return Err(invalid(
                &asset.name,
                "a package row in the published catalog",
            ));
        }
    }
    Ok(())
}

fn validate_row(
    release: &PublishedRelease,
    row: &Row,
    assets: &[Asset],
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    let filename = format!("{}-{}.crate", row.name, row.version);
    let uploaded = assets.iter().any(|asset| asset.name == filename);
    if uploaded != row.shipped {
        return Err(invalid(
            &filename,
            "catalog shipment flag matching actual release assets",
        ));
    }
    if !row.shipped {
        return Ok(());
    }
    validate_shipped_digest(release, row, archives)
}

fn validate_shipped_digest(
    release: &PublishedRelease,
    row: &Row,
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    let filename = format!("{}-{}.crate", row.name, row.version);
    let archive = archives
        .get(&(row.name.clone(), row.version.clone()))
        .ok_or_else(|| invalid(&filename, "verified published archive"))?;
    let origin = archive
        .origins
        .iter()
        .find(|origin| origin.release_id == release.id)
        .ok_or_else(|| invalid(&filename, "archive origin matching its catalog release"))?;
    if row
        .archive_sha256
        .as_ref()
        .is_some_and(|expected| expected != &origin.sha256)
    {
        return Err(invalid(&filename, "archive digest matching its catalog"));
    }
    Ok(())
}

fn validate_receipt(
    receipt: &HistoryReceipt,
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    for known in &receipt.archives {
        let present = archives
            .values()
            .flat_map(|archive| &archive.origins)
            .any(|origin| origin == known);
        if !present {
            return Err(invalid(
                &known.archive,
                "every previously recorded publication still present with identical bytes",
            ));
        }
    }
    Ok(())
}

fn legacy_schema() -> u32 {
    1
}
