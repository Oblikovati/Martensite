//! Owned streaming boundary over the pure Rust gzip backend.
use flate2::read::GzDecoder;
use std::io::{self, BufRead, BufReader};

pub struct FixtureReader<'a>(BufReader<GzDecoder<&'a [u8]>>);
impl<'a> FixtureReader<'a> {
    pub fn new(encoded: &'a [u8]) -> Self {
        Self(BufReader::new(GzDecoder::new(encoded)))
    }
    pub fn next_line(&mut self) -> io::Result<Option<String>> {
        let mut line = String::new();
        if self.0.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        Ok(Some(line))
    }
}

#[cfg(test)]
mod tests;
