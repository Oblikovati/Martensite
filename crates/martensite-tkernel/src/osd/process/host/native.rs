//! The narrow FFI boundary of the Unix host queries: `uname`, `gethostname`,
//! `gethostbyname`. Other hosts answer empty names.
#![allow(unsafe_code)]
use super::{HostSource, SystemNames};

/// The real machine; `NativeHost.system_names()`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativeHost;

#[cfg(unix)]
mod unix {
    use super::SystemNames;
    use std::ffi::CString;

    #[cfg(test)]
    mod hostname_contracts;
    #[cfg(test)]
    mod system_name_contracts;

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    mod hostname;
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    pub(super) use hostname::host_name;

    // gethostbyname is POSIX (glibc, macOS libSystem) but not bound by the libc crate.
    unsafe extern "C" {
        fn gethostbyname(name: *const libc::c_char) -> *mut libc::hostent;
    }

    fn field(bytes: &[libc::c_char]) -> Vec<u8> {
        bytes
            .iter()
            .copied()
            .take_while(|&byte| byte != 0)
            .map(|byte| byte.to_ne_bytes()[0])
            .collect()
    }

    pub(super) fn system_names() -> SystemNames {
        let mut info: libc::utsname = EMPTY_SYSTEM_NAMES;
        // SAFETY: `info` is live and writable; the status is ignored on purpose.
        unsafe { libc::uname(&mut info) };
        SystemNames {
            sysname: field(&info.sysname),
            release: field(&info.release),
        }
    }

    const EMPTY_SYSTEM_NAMES: libc::utsname = libc::utsname {
        sysname: empty_field(),
        nodename: empty_field(),
        release: empty_field(),
        version: empty_field(),
        machine: empty_field(),
        #[cfg(any(
            target_env = "newlib",
            target_os = "linux",
            target_os = "l4re",
            target_os = "android",
            target_os = "emscripten",
            target_os = "redox",
            target_os = "cygwin",
            target_os = "hurd"
        ))]
        domainname: empty_field(),
    };

    const fn empty_field<const N: usize>() -> [libc::c_char; N] {
        [0; N]
    }

    #[cfg(not(all(target_os = "linux", target_env = "gnu")))]
    pub(super) fn host_name() -> Result<Vec<u8>, i32> {
        let mut value: [libc::c_char; 65] = [0; 65];
        // SAFETY: at most 64 bytes are written into the 65-byte buffer, whose last byte
        // stays 0, so the result is always NUL-terminated.
        if unsafe { libc::gethostname(value.as_mut_ptr(), 64) } == -1 {
            return Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(0));
        }
        Ok(field(&value))
    }

    pub(super) fn resolve_ipv4(name: &[u8]) -> Option<[u8; 4]> {
        let name: CString = CString::new(name).ok()?;
        // SAFETY: `name` is a valid C string; the static result is read before any other
        // resolver call on this thread.
        let entry: *mut libc::hostent = unsafe { gethostbyname(name.as_ptr()) };
        if entry.is_null() {
            return None;
        }
        // SAFETY: a non-null hostent from gethostbyname has a null-terminated address list
        // of AF_INET addresses, each h_length (4) bytes long.
        unsafe {
            let first: *mut libc::c_char = *(*entry).h_addr_list;
            if first.is_null() {
                return None;
            }
            let bytes: &[u8] = std::slice::from_raw_parts(first.cast::<u8>(), 4);
            Some([bytes[0], bytes[1], bytes[2], bytes[3]])
        }
    }

    #[cfg(test)]
    mod field_tests {
        use super::field;

        #[test]
        fn system_name_output_fields_start_empty() {
            let info = super::EMPTY_SYSTEM_NAMES;
            for bytes in [
                &info.sysname[..],
                &info.nodename[..],
                &info.release[..],
                &info.version[..],
                &info.machine[..],
            ] {
                assert!(bytes.iter().all(|&byte| byte == 0));
            }
            #[cfg(target_os = "linux")]
            assert!(info.domainname.iter().all(|&byte| byte == 0));
        }

        #[test]
        fn terminated_fields_keep_raw_bytes_and_stop_at_first_nul() {
            let bytes: Vec<libc::c_char> = [b'h', 0xff, 0, b'x', 0]
                .into_iter()
                .map(|byte| byte as libc::c_char)
                .collect();
            assert_eq!(field(&bytes), [b'h', 0xff]);
            assert!(field(&[0, 1, 0]).is_empty());
            assert_eq!(field(&[65, 66, 0]), b"AB");
        }

        #[test]
        fn field_conversion_preserves_every_nonzero_byte() {
            let expected: Vec<u8> = (1..=255).collect();
            let mut bytes: Vec<libc::c_char> =
                expected.iter().map(|&byte| byte as libc::c_char).collect();
            bytes.push(0);
            assert_eq!(field(&bytes), expected);
        }
    }
}

#[cfg(unix)]
impl HostSource for NativeHost {
    fn system_names(&self) -> SystemNames {
        unix::system_names()
    }
    fn host_name(&self) -> Result<Vec<u8>, i32> {
        unix::host_name()
    }
    fn resolve_ipv4(&self, name: &[u8]) -> Option<[u8; 4]> {
        unix::resolve_ipv4(name)
    }
}

// TODO(platform): the Windows host queries (GetVersionExW, GetComputerNameA,
// GlobalMemoryStatus, Winsock) are not implemented; Host records ERROR_CALL_NOT_IMPLEMENTED
// before reaching this source there.
#[cfg(not(unix))]
impl HostSource for NativeHost {
    fn system_names(&self) -> SystemNames {
        SystemNames::default()
    }
    fn host_name(&self) -> Result<Vec<u8>, i32> {
        Ok(Vec::new())
    }
    fn resolve_ipv4(&self, _name: &[u8]) -> Option<[u8; 4]> {
        None
    }
}
