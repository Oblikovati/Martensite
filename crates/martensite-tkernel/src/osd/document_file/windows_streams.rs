//! Refuse named-stream loss before staged-document access metadata is changed (#248).
#![allow(unsafe_code)]
use super::wide_path;
use std::{io, path::Path};
use windows_sys::Win32::Foundation::{ERROR_HANDLE_EOF, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    FindClose, FindFirstStreamW, FindNextStreamW, FindStreamInfoStandard, WIN32_FIND_STREAM_DATA,
};

struct StreamSearch {
    handle: HANDLE,
    entry: WIN32_FIND_STREAM_DATA,
}

impl Drop for StreamSearch {
    fn drop(&mut self) {
        // SAFETY: A successful FindFirstStreamW transfers this unique search handle;
        // it remains valid until this single close and is never used afterwards.
        unsafe { FindClose(self.handle) };
    }
}

impl StreamSearch {
    fn open(path: &Path) -> io::Result<Option<Self>> {
        let wide = wide_path(path)?;
        let mut entry = WIN32_FIND_STREAM_DATA::default();
        let handle = first_stream(&wide, &mut entry);
        if handle == INVALID_HANDLE_VALUE {
            return enumeration_end(io::Error::last_os_error()).map(|_| None);
        }
        Ok(Some(Self { handle, entry }))
    }

    fn advance(&mut self) -> io::Result<bool> {
        // SAFETY: This live search handle came from FindFirstStreamW and the
        // writable entry has the exact required WIN32_FIND_STREAM_DATA layout.
        if unsafe { FindNextStreamW(self.handle, std::ptr::from_mut(&mut self.entry).cast()) } != 0
        {
            return Ok(true);
        }
        enumeration_end(io::Error::last_os_error())
    }
}

fn first_stream(wide: &[u16], entry: &mut WIN32_FIND_STREAM_DATA) -> HANDLE {
    // SAFETY: The caller supplies a terminated path and a suitably sized, aligned
    // output that remain live. A valid result owns an initialized first entry.
    unsafe {
        FindFirstStreamW(
            wide.as_ptr(),
            FindStreamInfoStandard,
            std::ptr::from_mut(entry).cast(),
            0,
        )
    }
}

fn enumeration_end(error: io::Error) -> io::Result<bool> {
    if error.raw_os_error() == Some(ERROR_HANDLE_EOF as i32) {
        return Ok(false);
    }
    // Unsupported filesystems and access failures cannot prove stream absence.
    Err(error)
}

fn stream_name<'a>(path: &Path, entry: &'a WIN32_FIND_STREAM_DATA) -> io::Result<&'a [u16]> {
    let length = entry.cStreamName.iter().position(|&unit| unit == 0).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, format!(
            "stream name for {} has no terminator; expected a NUL-terminated Windows stream name", path.display(),
        ))
    })?;
    Ok(&entry.cStreamName[..length])
}

fn require_unnamed(path: &Path, entry: &WIN32_FIND_STREAM_DATA) -> io::Result<()> {
    let stream = stream_name(path, entry)?;
    if stream == [58, 58, 36, 68, 65, 84, 65] {
        // The unnamed byte stream is ::$DATA.
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!(
            "document destination {} has named stream {}; expected only the unnamed byte stream before replacement; publication refused",
            path.display(),
            String::from_utf16_lossy(stream),
        ),
    ))
}

/// Refuse destination metadata loss before publication; `reject_named_streams(path)?`.
pub(super) fn reject_named_streams(path: &Path) -> io::Result<()> {
    let Some(mut search) = StreamSearch::open(path)? else {
        return Ok(());
    };
    loop {
        require_unnamed(path, &search.entry)?;
        if !search.advance()? {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::Foundation::{ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER};

    #[test]
    fn only_enumeration_eof_establishes_completion() {
        assert!(!enumeration_end(io::Error::from_raw_os_error(ERROR_HANDLE_EOF as i32)).unwrap());
        for code in [ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER] {
            let error = enumeration_end(io::Error::from_raw_os_error(code as i32)).unwrap_err();
            assert_eq!(error.raw_os_error(), Some(code as i32));
        }
    }

    #[test]
    fn malformed_stream_names_refuse_publication() {
        let mut entry = WIN32_FIND_STREAM_DATA::default();
        entry.cStreamName.fill(65);
        let error = require_unnamed(Path::new("malformed.bin"), &entry).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("malformed.bin"));
    }
}
