//! Keep message bytes intact while offering the existing Rust text presentation.
use std::sync::Arc;

#[derive(Debug, Eq, PartialEq)]
pub(super) struct NonUtf8Text {
    original: Box<[u8]>,
    display: Box<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum FailureText {
    /// `None` represents an empty string with zero heap allocation.
    Utf8(Option<Arc<str>>),
    Bytes(Arc<NonUtf8Text>),
}

impl FailureText {
    #[inline]
    pub(super) fn from_str(text: &str) -> Self {
        if text.is_empty() {
            return Self::Utf8(None);
        }
        let bytes = text.as_bytes();
        let prefix = match bytes.iter().position(|&byte| byte == 0) {
            Some(0) => return Self::Utf8(None),
            Some(end) => &text[..end],
            None => text,
        };
        Self::Utf8(Some(Arc::from(prefix)))
    }

    pub(super) fn from_bytes(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Self::Utf8(None);
        }
        let end = bytes
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(bytes.len());
        if end == 0 {
            return Self::Utf8(None);
        }
        let prefix = &bytes[..end];
        match std::str::from_utf8(prefix) {
            Ok(text) => Self::Utf8(Some(Arc::from(text))),
            Err(_) => Self::Bytes(Arc::new(NonUtf8Text {
                original: Box::from(prefix),
                display: Box::from(String::from_utf8_lossy(prefix).as_ref()),
            })),
        }
    }

    #[inline]
    pub(super) fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Utf8(None) => b"",
            Self::Utf8(Some(text)) => text.as_bytes(),
            Self::Bytes(non_utf8) => &non_utf8.original,
        }
    }

    #[inline]
    pub(super) fn as_str(&self) -> &str {
        match self {
            Self::Utf8(None) => "",
            Self::Utf8(Some(text)) => text,
            Self::Bytes(non_utf8) => &non_utf8.display,
        }
    }
}
