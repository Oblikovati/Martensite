//! Apple extended ACLs include ordered deny entries and inheritance flags (#245).
#![allow(unsafe_code)]
use std::{ffi::CString, io, os::unix::ffi::OsStrExt, path::Path, ptr::NonNull};

// Apple sys/acl.h: libc 0.2.190 does not expose these declarations.
// https://github.com/apple-oss-distributions/Libc/blob/main/include/sys/acl.h
unsafe extern "C" {
    fn acl_init(count: libc::c_int) -> *mut libc::c_void;
    fn acl_get_file(path: *const libc::c_char, kind: libc::c_int) -> *mut libc::c_void;
    fn acl_set_file(
        path: *const libc::c_char,
        kind: libc::c_int,
        acl: *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_free(acl: *mut libc::c_void) -> libc::c_int;
    fn acl_size(acl: *mut libc::c_void) -> libc::ssize_t;
    fn acl_copy_ext(
        buffer: *mut libc::c_void,
        acl: *mut libc::c_void,
        size: libc::ssize_t,
    ) -> libc::ssize_t;
}

pub(super) struct ExtendedAcl(NonNull<libc::c_void>);

impl ExtendedAcl {
    pub(super) fn empty() -> io::Result<Self> {
        // SAFETY: zero is a supported initial entry count; ownership is transferred.
        Self::from_allocation(unsafe { acl_init(0) })
    }

    pub(super) fn read(path: &Path) -> io::Result<Self> {
        let name = path_name(path)?;
        // SAFETY: name is NUL-terminated and 0x100 is Apple's ACL_TYPE_EXTENDED.
        let result = Self::from_allocation(unsafe { acl_get_file(name.as_ptr(), 0x100) });
        match result {
            // Apple's filesec_get_property reports ENOENT for an absent ACL too.
            // Distinguish that from an absent file before normalizing to empty.
            Err(error) if error.raw_os_error() == Some(libc::ENOENT) => {
                std::fs::metadata(path)?;
                Self::empty()
            }
            other => other,
        }
    }

    fn from_allocation(pointer: *mut libc::c_void) -> io::Result<Self> {
        NonNull::new(pointer)
            .map(Self)
            .ok_or_else(io::Error::last_os_error)
    }

    pub(super) fn apply(&self, path: &Path) -> io::Result<()> {
        let name = path_name(path)?;
        // SAFETY: the path and live ACL remain valid throughout this synchronous call.
        if unsafe { acl_set_file(name.as_ptr(), 0x100, self.0.as_ptr()) } == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub(super) fn encoded(&self) -> io::Result<Vec<u8>> {
        // SAFETY: self owns a live ACL allocation.
        let length = unsafe { acl_size(self.0.as_ptr()) };
        if length < 0 {
            return Err(io::Error::last_os_error());
        }
        self.encode_into(length as usize)
    }

    fn encode_into(&self, length: usize) -> io::Result<Vec<u8>> {
        // Apple's acl_copy_ext zeroes the header and exports every ACE, preserving
        // UUIDs, ordered allow/deny entries and flags without name-service lookups.
        // https://github.com/apple-oss-distributions/Libc/blob/main/posix1e/acl_translate.c
        let mut buffer = vec![0_u64; length.div_ceil(8)];
        // SAFETY: storage is aligned for Apple's kauth_filesec, has at least length
        // writable bytes, and self stays live. Apple initializes every exported byte.
        let written =
            unsafe { acl_copy_ext(buffer.as_mut_ptr().cast(), self.0.as_ptr(), length as isize) };
        if written < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: all words were initialized and the successful export fits length.
        Ok(
            unsafe { std::slice::from_raw_parts(buffer.as_ptr().cast(), written as usize) }
                .to_vec(),
        )
    }
}

impl Drop for ExtendedAcl {
    fn drop(&mut self) {
        // SAFETY: this allocation came from the ACL API and is freed exactly once.
        unsafe { acl_free(self.0.as_ptr()) };
    }
}

fn path_name(path: &Path) -> io::Result<CString> {
    CString::new(path.as_os_str().as_bytes())
        .map_err(|cause| io::Error::new(io::ErrorKind::InvalidInput, cause))
}
