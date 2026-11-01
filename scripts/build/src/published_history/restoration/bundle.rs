//! Parse bounded regular members before writing any recovered package (#358).
use super::super::invalid;
use crate::error::{BuildResult, io_error};
use std::{collections::BTreeMap, io::Read, path::Path};

pub(super) const MAX_BYTES: u64 = 128 * 1024 * 1024;
const MAX_MEMBERS: usize = 257;

pub(super) fn members(path: &Path) -> BuildResult<BTreeMap<String, Vec<u8>>> {
    let file = std::fs::File::open(path).map_err(|cause| io_error(path, cause))?;
    let limit = super::bounded::ExpandedLimit::new(
        flate2::read::GzDecoder::new(file),
        (MAX_BYTES + 1024 * 1024) as usize,
    );
    let mut tar = tar::Archive::new(limit);
    let retained = collect_entries(&mut tar, path)?;
    super::bounded::finish(tar.into_inner(), path)?;
    Ok(retained)
}

fn collect_entries<R: Read>(
    tar: &mut tar::Archive<R>,
    path: &Path,
) -> BuildResult<BTreeMap<String, Vec<u8>>> {
    let mut retained = BTreeMap::new();
    let mut total = 0;
    for entry in tar
        .entries()
        .map_err(|cause| io_error(path, cause))?
        .raw(true)
    {
        retain_entry(
            entry.map_err(|cause| io_error(path, cause))?,
            path,
            &mut total,
            &mut retained,
        )?;
    }
    Ok(retained)
}

fn retain_entry<R: Read>(
    mut entry: tar::Entry<'_, R>,
    path: &Path,
    total: &mut u64,
    retained: &mut BTreeMap<String, Vec<u8>>,
) -> BuildResult<()> {
    let name = member_name(&entry, path)?;
    *total = charge(*total, entry.size(), retained.len(), &name)?;
    let mut bytes = Vec::new();
    entry
        .read_to_end(&mut bytes)
        .map_err(|cause| io_error(path, cause))?;
    if retained.insert(name.clone(), bytes).is_some() {
        return Err(invalid(&name, "unique restoration member"));
    }
    Ok(())
}

fn charge(total: u64, size: u64, count: usize, name: &str) -> BuildResult<u64> {
    let sum = total.checked_add(size);
    if count >= MAX_MEMBERS || size > MAX_BYTES || sum.is_none_or(|value| value > MAX_BYTES) {
        return Err(invalid(
            name,
            "at most 257 regular members and 128 MiB expanded bytes",
        ));
    }
    Ok(sum.unwrap_or_default())
}

fn member_name<R: Read>(entry: &tar::Entry<'_, R>, path: &Path) -> BuildResult<String> {
    let name = entry.path().map_err(|cause| io_error(path, cause))?;
    let name = name.to_str().unwrap_or_default();
    let archive = name.strip_prefix("archives/");
    let valid = name == "restoration.json"
        || archive.is_some_and(|name| super::super::archives::identity(name).is_ok());
    if !entry.header().entry_type().is_file() || !valid {
        return Err(invalid(
            name,
            "regular restoration.json or archives/name-version.crate",
        ));
    }
    Ok(name.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RestorationTarFake;

    impl RestorationTarFake {
        fn write(path: &Path, kind: tar::EntryType, name: &str) {
            let file = std::fs::File::create(path).unwrap();
            let gzip = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut tar = tar::Builder::new(gzip);
            let mut header = tar::Header::new_gnu();
            header.as_mut_bytes()[..name.len()].copy_from_slice(name.as_bytes());
            header.set_entry_type(kind);
            header.set_size(0);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append(&header, std::io::empty()).unwrap();
            tar.into_inner().unwrap().finish().unwrap();
        }
    }

    impl RestorationTarFake {
        fn extension(path: &Path, kind: tar::EntryType) {
            use std::io::Write;
            let file = std::fs::File::create(path).unwrap();
            let mut gzip = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut header = tar::Header::new_gnu();
            header.set_path("restoration.json").unwrap();
            header.set_entry_type(kind);
            header.set_size(u64::MAX / 2);
            header.set_cksum();
            gzip.write_all(header.as_bytes()).unwrap();
            gzip.finish().unwrap();
        }
    }

    #[test]
    fn explicit_resource_bounds_accept_exact_limit_and_refuse_excess() {
        assert_eq!(charge(0, MAX_BYTES, 0, "fake").unwrap(), MAX_BYTES);
        assert!(charge(MAX_BYTES, 1, 0, "fake").is_err());
        assert!(charge(u64::MAX, 1, 0, "fake").is_err());
        assert!(charge(0, 0, MAX_MEMBERS, "fake").is_err());
    }

    #[test]
    fn foreign_paths_links_and_directories_cannot_be_recovered() {
        let folder = tempfile::tempdir().unwrap();
        for (kind, name) in [
            (tar::EntryType::Regular, "../escape"),
            (tar::EntryType::Symlink, "restoration.json"),
            (tar::EntryType::Directory, "restoration.json"),
        ] {
            let path = folder.path().join("fake.tar.gz");
            RestorationTarFake::write(&path, kind, name);
            assert!(members(&path).is_err(), "{name}");
        }
        assert!(!folder.path().parent().unwrap().join("escape").exists());
    }
    #[test]
    fn oversized_extension_header_refuses_before_reading_its_payload() {
        let folder = tempfile::tempdir().unwrap();
        for kind in [
            tar::EntryType::GNULongName,
            tar::EntryType::GNULongLink,
            tar::EntryType::XHeader,
            tar::EntryType::XGlobalHeader,
            tar::EntryType::GNUSparse,
        ] {
            let path = folder.path().join("fake-extension.tar.gz");
            RestorationTarFake::extension(&path, kind);
            assert!(members(&path).is_err());
            assert!(super::super::bounded::inner_archive(&path, "Cargo.toml").is_err());
        }
    }
}
