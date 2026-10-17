//! Preserve the GNU hostname buffer contract through the safe wrapper (#89).
#![forbid(unsafe_code)]

use nix::errno::Errno;
use std::os::unix::ffi::OsStringExt;

pub(in super::super) fn host_name() -> Result<Vec<u8>, i32> {
    let previous_errno = Errno::last_raw();
    let queried = nix::unistd::gethostname().map_err(|_| Errno::last_raw());
    let bytes = queried?.into_vec();
    hostname_bytes(bytes, previous_errno)
}

fn hostname_bytes(bytes: Vec<u8>, previous_errno: i32) -> Result<Vec<u8>, i32> {
    // GNU libc requires space for the terminator in the original 64-byte call.
    if bytes.len() >= 64 {
        Errno::set_raw(libc::ENAMETOOLONG);
        return Err(libc::ENAMETOOLONG);
    }
    Errno::set_raw(previous_errno);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hostname_adapter_preserves_raw_bytes_and_native_length_boundary() {
        let original = Errno::last_raw();
        for length in [0, 1, 63, 64, 255] {
            let bytes = vec![0xff; length];
            let expected = if length < 64 {
                (Ok(bytes.clone()), 12345)
            } else {
                (Err(libc::ENAMETOOLONG), libc::ENAMETOOLONG)
            };
            let result = hostname_bytes(bytes, 12345);
            let observed = Errno::last_raw();
            Errno::set_raw(original);
            assert_eq!((result, observed), expected);
        }
    }
}
