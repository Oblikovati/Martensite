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
    let mut sequence = super::long_name::InnerSequence::new(expected_manifest);
    for (count, entry) in tar
        .entries()
        .map_err(|cause| io_error(path, cause))?
        .raw(true)
        .enumerate()
    {
        let mut entry = entry.map_err(|cause| io_error(path, cause))?;
        sequence.inspect(&mut entry, path, count)?;
    }
    sequence.finish()?;
    finish(tar.into_inner(), path)
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
