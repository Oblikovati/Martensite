//! Owns the libc boundary used by the locale conversions on Unix.
#![allow(unsafe_code)]

use crate::ncollection::WideChar;
use std::sync::Mutex;

// mblen has hidden conversion state; serialize calls made by this adapter.
static MULTIBYTE_STATE: Mutex<()> = Mutex::new(());

unsafe extern "C" {
    fn mblen(text: *const libc::c_char, limit: libc::size_t) -> libc::c_int;
    fn mbstowcs(
        output: *mut libc::wchar_t,
        text: *const libc::c_char,
        limit: libc::size_t,
    ) -> libc::size_t;
}

pub(super) fn decode(text: &[u8]) -> Vec<WideChar> {
    let end = text
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(text.len());
    let mut terminated = text[..end].to_vec();
    terminated.push(0);
    let _guard = MULTIBYTE_STATE
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // SAFETY: `terminated` is a live NUL-terminated buffer of `terminated.len()` bytes, and MULTIBYTE_STATE serializes the shared mblen state. The terminator bounds every read; a valid character cannot exceed MB_CUR_MAX.
    let count = unsafe { mblen(terminated.as_ptr().cast(), terminated.len()) };
    if count <= 0 {
        return Vec::new();
    }
    decode_wide(&terminated, count as usize)
}

fn decode_wide(terminated: &[u8], count: usize) -> Vec<WideChar> {
    // Storage after an invalid later character would be left uninitialized. Zeroing
    // keeps Rust safe; nothing defines those bytes, so no behavior depends on them.
    let mut wide = vec![0 as libc::wchar_t; count + 1];
    // SAFETY: `wide` holds `count + 1` units, so mbstowcs writes at most `count` units plus
    // none past it, and `terminated` is NUL-terminated.
    unsafe { mbstowcs(wide.as_mut_ptr(), terminated.as_ptr().cast(), count) };
    wide.into_iter().map(|unit| unit as WideChar).collect()
}

pub(super) fn encode(wide: &[WideChar], buffer: &mut [u8]) -> usize {
    let mut terminated: Vec<libc::wchar_t> =
        wide.iter().map(|unit| *unit as libc::wchar_t).collect();
    terminated.push(0);
    // SAFETY: Both buffers are owned, terminated, and the output bound is its allocation size.
    unsafe {
        libc::wcstombs(
            buffer.as_mut_ptr().cast(),
            terminated.as_ptr(),
            buffer.len(),
        )
    }
}
