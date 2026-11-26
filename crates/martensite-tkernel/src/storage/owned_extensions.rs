// SPDX-License-Identifier: GPL-2.0-only
//! Wave 4 owned storage, stream seeking, dynamic root view, exception ancestry,
//! and dynamic-array gap cleanup contracts (#96, #115, #116, #145, #373).
use super::IoStream;
use crate::standard::Failure;
use std::io::SeekFrom;

/// Origin for checked stream seeking (#145).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SeekOrigin {
    /// Offset from byte 0.
    Start,
    /// Offset relative to current stream position.
    Current,
    /// Offset relative to stream end.
    End,
}

/// Seeks `stream` from `origin` by `offset` and returns the new byte position (#145).
pub fn seek_from_origin(
    stream: &mut IoStream,
    origin: SeekOrigin,
    offset: i64,
) -> Result<u64, Failure> {
    let seek = match origin {
        SeekOrigin::Start => {
            let pos = u64::try_from(offset)
                .map_err(|_| Failure::with_message("Storage_SeekError", "negative start offset"))?;
            SeekFrom::Start(pos)
        }
        SeekOrigin::Current => SeekFrom::Current(offset),
        SeekOrigin::End => SeekFrom::End(offset),
    };
    stream
        .seek_origin_device(seek)
        .ok_or_else(|| Failure::with_message("Storage_SeekError", "stream seek refused"))
}

/// Checked borrowed input reader preserving exact byte position on error (#116).
#[derive(Clone, Debug)]
pub struct CheckedBorrowedInput<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> CheckedBorrowedInput<'a> {
    /// Wraps `bytes` starting at byte offset 0.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    /// Current exact byte position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Remaining unread bytes.
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    /// Reads `len` bytes or returns an error without advancing `position` (#116).
    pub fn read_exact_checked(&mut self, len: usize) -> Result<&'a [u8], Failure> {
        let end = self
            .position
            .checked_add(len)
            .ok_or_else(|| Failure::with_message("Storage_ReadError", "position overflow"))?;
        if end > self.bytes.len() {
            return Err(Failure::with_message(
                "Storage_ReadError",
                &format!(
                    "requested {len} bytes at position {} with only {} remaining",
                    self.position,
                    self.remaining()
                ),
            ));
        }
        let slice = &self.bytes[self.position..end];
        self.position = end;
        Ok(slice)
    }

    /// Reads a little-endian `u32` without advancing `position` on EOF (#116).
    pub fn read_u32_le(&mut self) -> Result<u32, Failure> {
        let raw = self.read_exact_checked(4)?;
        Ok(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
    }
}

/// Owned, immutable snapshot view of a dynamic root storage object (#115).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedDynamicRootView {
    type_name: String,
    root_id: u64,
    revision: u64,
    payload: Vec<u8>,
}

impl OwnedDynamicRootView {
    /// Creates an owned dynamic root view.
    pub fn new(
        type_name: impl Into<String>,
        root_id: u64,
        revision: u64,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            type_name: type_name.into(),
            root_id,
            revision,
            payload,
        }
    }

    /// Dynamic type name of the root object.
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// Stable root identifier.
    pub fn root_id(&self) -> u64 {
        self.root_id
    }

    /// Revision number of the captured snapshot.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Owned payload bytes.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

/// Owned multi-level ancestry chain for extension exception types (#96).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExceptionAncestry {
    chain: Vec<String>,
}

impl ExceptionAncestry {
    /// Builds an ancestry chain ordered from most-derived type to base root (#96).
    pub fn new(chain: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, Failure> {
        let chain: Vec<String> = chain.into_iter().map(Into::into).collect();
        if chain.is_empty() || chain.iter().any(|name| name.is_empty()) {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "exception ancestry chain must be non-empty with non-empty type names",
            ));
        }
        Ok(Self { chain })
    }

    /// Most-derived exception type name.
    pub fn leaf_type(&self) -> &str {
        &self.chain[0]
    }

    /// Full ordered ancestry chain from leaf to root.
    pub fn chain(&self) -> &[String] {
        &self.chain
    }

    /// True when `ancestor` appears anywhere in this exception's ancestry chain (#96).
    pub fn is_subclass_of(&self, ancestor: &str) -> bool {
        self.chain.iter().any(|entry| entry == ancestor)
    }
}

/// Versioned policy governing cleanup of abandoned dynamic-array gap resources (#373).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GapCleanupPolicy {
    /// Policy schema version.
    pub version: u32,
    /// Maximum abandoned gaps retained before eager reclamation.
    pub max_abandoned_gaps: usize,
    /// Whether reset/clear reclaims abandoned gap metadata immediately.
    pub reclaim_on_reset: bool,
}

impl Default for GapCleanupPolicy {
    fn default() -> Self {
        Self {
            version: 1,
            max_abandoned_gaps: 64,
            reclaim_on_reset: true,
        }
    }
}

impl GapCleanupPolicy {
    /// True when `abandoned_gaps` reaches or exceeds the reclamation threshold (#373).
    pub fn should_reclaim(&self, abandoned_gaps: usize) -> bool {
        abandoned_gaps >= self.max_abandoned_gaps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seek_from_origin_supports_start_current_and_end() {
        let mut stream = IoStream::from_bytes(b"abcdefghij".to_vec());
        assert_eq!(
            seek_from_origin(&mut stream, SeekOrigin::Start, 4).unwrap(),
            4
        );
        assert_eq!(
            seek_from_origin(&mut stream, SeekOrigin::Current, 2).unwrap(),
            6
        );
        assert_eq!(
            seek_from_origin(&mut stream, SeekOrigin::End, -3).unwrap(),
            7
        );
        assert!(seek_from_origin(&mut stream, SeekOrigin::Start, -1).is_err());
    }

    #[test]
    fn checked_borrowed_input_preserves_position_on_eof() {
        let mut input = CheckedBorrowedInput::new(&[1, 0, 0, 0, 9, 8]);
        assert_eq!(input.read_u32_le().unwrap(), 1);
        assert_eq!(input.position(), 4);
        assert!(input.read_u32_le().is_err());
        assert_eq!(input.position(), 4);
        assert_eq!(input.remaining(), 2);
    }

    #[test]
    fn exception_ancestry_checks_multi_level_inheritance() {
        let chain = ExceptionAncestry::new([
            "Custom_DomainFailure",
            "Standard_DomainError",
            "Standard_Failure",
        ])
        .unwrap();
        assert_eq!(chain.leaf_type(), "Custom_DomainFailure");
        assert!(chain.is_subclass_of("Standard_DomainError"));
        assert!(chain.is_subclass_of("Standard_Failure"));
        assert!(!chain.is_subclass_of("Standard_OutOfRange"));
        assert!(ExceptionAncestry::new(Vec::<String>::new()).is_err());
    }

    #[test]
    fn owned_dynamic_root_view_and_gap_policy_work() {
        let view = OwnedDynamicRootView::new("RootPart", 7, 3, vec![1, 2, 3]);
        assert_eq!(view.type_name(), "RootPart");
        assert_eq!(view.root_id(), 7);
        assert_eq!(view.revision(), 3);
        assert_eq!(view.payload(), &[1, 2, 3]);

        let policy = GapCleanupPolicy::default();
        assert!(!policy.should_reclaim(63));
        assert!(policy.should_reclaim(64));
    }
}
