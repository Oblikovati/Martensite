//! Bound decompressed reads, including tar extension headers and trailing bytes (#358).
use crate::error::{BuildResult, io_error};
use std::{
    io::{self, Read},
    path::Path,
};

pub(super) struct ExpandedLimit<R> {
    input: R,
    remaining: usize,
}

impl<R: Read> ExpandedLimit<R> {
    pub(super) fn new(input: R, remaining: usize) -> Self {
        Self { input, remaining }
    }
}

impl<R: Read> Read for ExpandedLimit<R> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        if self.remaining == 0 {
            return match self.input.read(&mut [0])? {
                0 => Ok(0),
                _ => Err(io::Error::other(
                    "restoration expanded bytes exceed declared resource limit",
                )),
            };
        }
        let length = output.len().min(self.remaining);
        let count = self.input.read(&mut output[..length])?;
        self.remaining -= count;
        Ok(count)
    }
}

pub(super) fn finish<R: Read>(mut input: R, path: &Path) -> BuildResult<()> {
    io::copy(&mut input, &mut io::sink()).map_err(|cause| io_error(path, cause))?;
    Ok(())
}

pub(super) fn inner_archive(path: &Path, expected_manifest: &str) -> BuildResult<()> {
    let file = std::fs::File::open(path).map_err(|cause| io_error(path, cause))?;
    let limit = ExpandedLimit::new(flate2::read::GzDecoder::new(file), 1024 * 1024 * 1024);
    let mut tar = tar::Archive::new(limit);
    for (count, entry) in tar
        .entries()
        .map_err(|cause| io_error(path, cause))?
        .raw(true)
        .enumerate()
    {
        let entry = entry.map_err(|cause| io_error(path, cause))?;
        require_inner(&entry, path, expected_manifest, count)?;
    }
    finish(tar.into_inner(), path)
}

fn require_inner<R: Read>(
    entry: &tar::Entry<'_, R>,
    path: &Path,
    expected: &str,
    count: usize,
) -> BuildResult<()> {
    let name = entry.path().map_err(|cause| io_error(path, cause))?;
    let large_manifest = name == Path::new(expected) && entry.size() > 1024 * 1024;
    if count >= 65536 || !entry.header().entry_type().is_file() || large_manifest {
        return Err(super::super::invalid(
            &name.display().to_string(),
            "at most 65536 regular inner members and normalized manifest at most 1 MiB",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn expanded_limit_reads_exact_end_and_refuses_trailing_bytes() {
        let mut exact = ExpandedLimit::new(&b"abc"[..], 3);
        assert_eq!(io::copy(&mut exact, &mut io::sink()).unwrap(), 3);
        let mut excess = ExpandedLimit::new(&b"abcd"[..], 3);
        assert!(io::copy(&mut excess, &mut io::sink()).is_err());
        assert_eq!(excess.read(&mut []).unwrap(), 0);
    }
}
