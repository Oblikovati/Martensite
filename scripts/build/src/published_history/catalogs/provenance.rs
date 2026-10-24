//! Reused schema-2 rows remain authoritative even when no payload ships again.
use super::{
    super::{archives::PublishedArchive, invalid, inventory::HistoryReceipt},
    Row,
};
use crate::{error::BuildResult, publication_record::PublicationReason};
use semver::Version;
use std::collections::BTreeMap;

pub(super) fn validate(
    row: &Row,
    schema: u32,
    receipt: Option<&HistoryReceipt>,
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    if schema == 2 {
        required(row)?;
    }
    validate_archive(row, archives)?;
    if let Some(reason) = row.publication_reason {
        validate_reason(row, reason, receipt)?;
    }
    Ok(())
}

fn validate_archive(
    row: &Row,
    archives: &BTreeMap<(String, Version), PublishedArchive>,
) -> BuildResult<()> {
    let archive = archives.get(&(row.name.clone(), row.version.clone()));
    validate_digest(row, archive)?;
    for origin in row.origins.as_deref().unwrap_or_default() {
        if !archive.is_some_and(|archive| archive.origins.contains(origin)) {
            return Err(invalid(
                &row.name,
                "row provenance matching verified publication origins",
            ));
        }
    }
    Ok(())
}

fn validate_digest(row: &Row, archive: Option<&PublishedArchive>) -> BuildResult<()> {
    let Some(expected) = &row.archive_sha256 else {
        return Ok(());
    };
    let archive = archive
        .ok_or_else(|| invalid(&row.name, "verified published archive for a claimed digest"))?;
    if archive.origins[0].sha256 != *expected {
        return Err(invalid(
            &row.name,
            "row digest matching verified published bytes",
        ));
    }
    Ok(())
}

fn required(row: &Row) -> BuildResult<()> {
    if row.publication_reason.is_none() || row.archive_sha256.is_none() || row.origins.is_none() {
        return Err(invalid(
            &row.name,
            "schema-2 reason, archive digest and origins",
        ));
    }
    Ok(())
}

fn validate_reason(
    row: &Row,
    reason: PublicationReason,
    receipt: Option<&HistoryReceipt>,
) -> BuildResult<()> {
    let origins = row
        .origins
        .as_deref()
        .ok_or_else(|| invalid(&row.name, "origins for a publication reason"))?;
    if reason == PublicationReason::HistoricalReuse {
        return reused(row, origins, receipt);
    }
    if !row.shipped || !origins.is_empty() {
        return Err(invalid(
            &row.name,
            "a shipped new identity with no historical origins",
        ));
    }
    require_new_identity(row, receipt)
}

fn require_new_identity(row: &Row, receipt: Option<&HistoryReceipt>) -> BuildResult<()> {
    let filename = format!("{}-{}.crate", row.name, row.version);
    if receipt.is_some_and(|receipt| {
        receipt
            .archives
            .iter()
            .any(|origin| origin.archive == filename)
    }) {
        return Err(invalid(
            &row.name,
            "an unpublished identity for changed-source or first-publication shipment",
        ));
    }
    Ok(())
}

fn reused(
    row: &Row,
    origins: &[super::super::ArchiveOrigin],
    receipt: Option<&HistoryReceipt>,
) -> BuildResult<()> {
    if row.shipped || origins.is_empty() {
        return Err(invalid(
            &row.name,
            "an unshipped reuse with a recorded historical origin",
        ));
    }
    require_receipt_origins(row, origins, receipt)
}

fn require_receipt_origins(
    row: &Row,
    origins: &[super::super::ArchiveOrigin],
    receipt: Option<&HistoryReceipt>,
) -> BuildResult<()> {
    let receipt = receipt.ok_or_else(|| {
        invalid(
            &row.name,
            "a publication history receipt for reuse provenance",
        )
    })?;
    if origins
        .iter()
        .any(|origin| !receipt.archives.contains(origin))
    {
        return Err(invalid(
            &row.name,
            "reuse provenance present in its retained history receipt",
        ));
    }
    Ok(())
}
