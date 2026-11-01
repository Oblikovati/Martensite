//! GNU long names are bounded metadata for the next regular inner member (#360).
use super::super::invalid;
use crate::error::{BuildResult, io_error};
use std::{io::Read, path::Path};

pub(super) struct InnerSequence<'a> {
    expected_manifest: &'a str,
    pending_name: Option<String>,
}

impl<'a> InnerSequence<'a> {
    pub(super) fn new(expected_manifest: &'a str) -> Self {
        Self {
            expected_manifest,
            pending_name: None,
        }
    }

    pub(super) fn inspect<R: Read>(
        &mut self,
        entry: &mut tar::Entry<'_, R>,
        path: &Path,
        count: usize,
    ) -> BuildResult<()> {
        if count >= 65536 {
            return Err(invalid(&count.to_string(), "at most 65536 inner headers"));
        }
        match entry.header().entry_type() {
            tar::EntryType::Regular => self.regular(entry, path),
            tar::EntryType::GNULongName => self.long_name(entry, path),
            kind => Err(invalid(
                &format!("{kind:?}"),
                "regular file or bounded GNU long name",
            )),
        }
    }

    pub(super) fn finish(self) -> BuildResult<()> {
        if let Some(name) = self.pending_name {
            return Err(invalid(
                &name,
                "GNU long name followed by one regular member",
            ));
        }
        Ok(())
    }

    fn regular<R: Read>(&mut self, entry: &tar::Entry<'_, R>, path: &Path) -> BuildResult<()> {
        let raw_name = entry.path().map_err(|cause| io_error(path, cause))?;
        let resolved = self.pending_name.take();
        let manifest = raw_name == Path::new(self.expected_manifest)
            || resolved.as_deref() == Some(self.expected_manifest);
        if manifest && entry.size() > 1024 * 1024 {
            return Err(invalid(
                &raw_name.display().to_string(),
                "normalized manifest at most 1 MiB",
            ));
        }
        Ok(())
    }

    fn long_name<R: Read>(
        &mut self,
        entry: &mut tar::Entry<'_, R>,
        path: &Path,
    ) -> BuildResult<()> {
        let recognized = entry.header().as_gnu().is_some() || entry.header().as_ustar().is_some();
        if self.pending_name.is_some() || !recognized || entry.size() == 0 || entry.size() > 4096 {
            return Err(invalid(
                &entry.size().to_string(),
                "one recognized GNU long name of 1..4096 bytes",
            ));
        }
        let mut bytes = Vec::new();
        entry
            .read_to_end(&mut bytes)
            .map_err(|cause| io_error(path, cause))?;
        self.pending_name = Some(canonical_name(&bytes, self.expected_manifest)?);
        Ok(())
    }
}

fn canonical_name(bytes: &[u8], expected_manifest: &str) -> BuildResult<String> {
    let name = decoded_name(bytes)?;
    let root = expected_manifest
        .strip_suffix("/Cargo.toml")
        .ok_or_else(|| invalid(expected_manifest, "package-version/Cargo.toml"))?;
    let components = name
        .split('/')
        .all(|part| !part.is_empty() && !matches!(part, "." | ".."));
    if name.contains(['\\', '\0', ':'])
        || !components
        || !name.starts_with(&format!("{root}/"))
        || name == expected_manifest
    {
        return Err(invalid(
            name,
            "canonical package-rooted non-manifest GNU long name",
        ));
    }
    Ok(name.into())
}

fn decoded_name(bytes: &[u8]) -> BuildResult<&str> {
    let terminated = bytes
        .strip_suffix(&[0])
        .ok_or_else(|| invalid(&bytes.len().to_string(), "NUL-terminated GNU name"))?;
    std::str::from_utf8(terminated).map_err(|cause| invalid(&cause.to_string(), "UTF-8 GNU name"))
}
