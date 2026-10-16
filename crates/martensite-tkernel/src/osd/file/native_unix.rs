//! The narrow libc boundary of the OSD file classes on Unix hosts: the exact system calls
//! that POSIX semantics need, so errno values and side effects come from the OS.
#![allow(unsafe_code)]
use super::calls::{
    AccessMode, CallResult, Errno, NodeStatus, NodeType, OpenFlags, PosixFileCalls, RecordLock,
    SeekOrigin,
};
use super::directory_stream::DirectoryEntries;
use crate::osd::LocalTime;
use crate::osd::OpenMode;
use std::ffi::CString;
use std::os::unix::{ffi::OsStrExt, fs::DirBuilderExt};
mod directory_cursor;
mod lock_record;

/// The real POSIX calls; `NativePosixCalls.stat(b"/tmp")`.
#[derive(Clone, Copy, Debug, Default)]
pub struct NativePosixCalls;

fn last_errno() -> Errno {
    Errno(std::io::Error::last_os_error().raw_os_error().unwrap_or(0))
}

fn c_path(path: &[u8]) -> CString {
    // The OSD paths are C strings: everything from the first NUL on is not part of them.
    let end: usize = path
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(path.len());
    CString::new(&path[..end]).expect("no interior NUL after truncation")
}

fn status_zero(status: libc::c_int) -> CallResult<()> {
    if status == -1 {
        Err(last_errno())
    } else {
        Ok(())
    }
}

fn native_path(name: &std::ffi::CStr) -> &std::path::Path {
    std::path::Path::new(std::ffi::OsStr::from_bytes(name.to_bytes()))
}

fn path_result(result: std::io::Result<()>) -> CallResult<()> {
    result.map_err(|error| Errno(error.raw_os_error().unwrap_or(0)))
}

// mode_t is u16 on macOS and the stat field widths vary by target: the casts are needed there.
#[allow(clippy::unnecessary_cast)]
fn node_status(raw: &libc::stat) -> NodeStatus {
    node_status_fields(
        raw.st_mode as u32,
        raw.st_size as u64,
        raw.st_ctime as i64,
        raw.st_blksize as u64,
    )
}

#[allow(clippy::unnecessary_cast)]
fn node_type(mode: u32) -> NodeType {
    match mode & (libc::S_IFMT as u32) {
        kind if kind == libc::S_IFREG as u32 => NodeType::Regular,
        kind if kind == libc::S_IFDIR as u32 => NodeType::Directory,
        kind if kind == libc::S_IFLNK as u32 => NodeType::Link,
        kind if kind == libc::S_IFIFO as u32 => NodeType::Fifo,
        kind if kind == libc::S_IFSOCK as u32 => NodeType::Socket,
        _ => NodeType::Other,
    }
}

fn node_status_fields(mode: u32, size: u64, change_time: i64, block_size: u64) -> NodeStatus {
    NodeStatus {
        node_type: node_type(mode),
        permissions: mode & 0o7777,
        size,
        change_time,
        block_size,
    }
}

fn native_open_flags(flags: OpenFlags) -> libc::c_int {
    let mut bits = match flags.access {
        OpenMode::ReadOnly => libc::O_RDONLY,
        OpenMode::WriteOnly => libc::O_WRONLY,
        OpenMode::ReadWrite => libc::O_RDWR,
    };
    bits |= if flags.create { libc::O_CREAT } else { 0 };
    bits |= if flags.truncate { libc::O_TRUNC } else { 0 };
    bits | if flags.append { libc::O_APPEND } else { 0 }
}

#[cfg(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(
        target_os = "linux",
        target_env = "gnu",
        target_pointer_width = "64",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
))]
fn native_open_descriptor(name: &std::ffi::CStr, bits: i32, mode: u32) -> CallResult<i32> {
    use std::os::fd::IntoRawFd;
    let flags = rustix::fs::OFlags::from_bits_retain(bits as _);
    let mode = rustix::fs::Mode::from_bits_retain(mode as _);
    rustix::fs::open(name, flags, mode)
        .map(IntoRawFd::into_raw_fd)
        .map_err(|error| Errno(error.raw_os_error()))
}

#[cfg(not(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(
        target_os = "linux",
        target_env = "gnu",
        target_pointer_width = "64",
        any(target_arch = "x86_64", target_arch = "aarch64")
    )
)))]
fn native_open_descriptor(name: &std::ffi::CStr, bits: i32, mode: u32) -> CallResult<i32> {
    // SAFETY: name is NUL-terminated; the variadic mode is passed as c_uint.
    let fd: libc::c_int = unsafe { libc::open(name.as_ptr(), bits, mode as libc::c_uint) };
    if fd < 0 { Err(last_errno()) } else { Ok(fd) }
}

impl PosixFileCalls for NativePosixCalls {
    fn access(&self, path: &[u8], mode: AccessMode) -> CallResult<()> {
        let mut bits: libc::c_int = libc::F_OK;
        bits |= if mode.read { libc::R_OK } else { 0 };
        bits |= if mode.write { libc::W_OK } else { 0 };
        bits |= if mode.execute { libc::X_OK } else { 0 };
        let name: CString = c_path(path);
        rustix::fs::access(name.as_c_str(), rustix::fs::Access::from_bits_retain(bits))
            .map_err(|error| Errno(error.raw_os_error()))
    }
    #[cfg(target_os = "macos")]
    fn stat(&self, path: &[u8]) -> CallResult<NodeStatus> {
        use std::os::unix::fs::MetadataExt;
        let name: CString = c_path(path);
        let metadata = std::fs::metadata(native_path(&name))
            .map_err(|error| Errno(error.raw_os_error().unwrap_or(0)))?;
        Ok(node_status_fields(
            metadata.mode(),
            metadata.size(),
            metadata.ctime(),
            metadata.blksize(),
        ))
    }
    #[cfg(not(target_os = "macos"))]
    fn stat(&self, path: &[u8]) -> CallResult<NodeStatus> {
        let name: CString = c_path(path);
        // SAFETY: an all-zero stat is a valid value of this plain C struct.
        let mut raw: libc::stat = unsafe { std::mem::zeroed() };
        // SAFETY: name is NUL-terminated; raw is a writable stat alive for the call.
        status_zero(unsafe { libc::stat(name.as_ptr(), &mut raw) })?;
        Ok(node_status(&raw))
    }
    fn fstat(&self, fd: i32) -> CallResult<NodeStatus> {
        #[cfg(target_os = "macos")]
        let mut raw = libc::stat::default();
        // SAFETY: an all-zero stat is a valid value of this plain C struct.
        #[cfg(not(target_os = "macos"))]
        let mut raw: libc::stat = unsafe { std::mem::zeroed() };
        // SAFETY: raw is a writable stat alive for the call; any fd value is accepted.
        status_zero(unsafe { libc::fstat(fd, &mut raw) })?;
        Ok(node_status(&raw))
    }
    fn open(&self, path: &[u8], flags: OpenFlags, mode: u32) -> CallResult<i32> {
        let bits = native_open_flags(flags);
        let name: CString = c_path(path);
        native_open_descriptor(&name, bits, mode)
    }
    fn close(&self, fd: i32) -> CallResult<()> {
        // SAFETY: closing a descriptor number has no memory-safety requirement.
        status_zero(unsafe { libc::close(fd) })
    }
    fn read(&self, fd: i32, buffer: &mut [u8]) -> CallResult<usize> {
        // SAFETY: buffer is writable for its full length during the call.
        let count: isize = unsafe { libc::read(fd, buffer.as_mut_ptr().cast(), buffer.len()) };
        usize::try_from(count).map_err(|_| last_errno())
    }
    fn write(&self, fd: i32, buffer: &[u8]) -> CallResult<usize> {
        // SAFETY: buffer is readable for its full length during the call.
        let count: isize = unsafe { libc::write(fd, buffer.as_ptr().cast(), buffer.len()) };
        usize::try_from(count).map_err(|_| last_errno())
    }
    fn lseek(&self, fd: i32, offset: i64, origin: SeekOrigin) -> CallResult<i64> {
        let whence: libc::c_int = match origin {
            SeekOrigin::Start => libc::SEEK_SET,
            SeekOrigin::Current => libc::SEEK_CUR,
            SeekOrigin::End => libc::SEEK_END,
        };
        // SAFETY: lseek only takes plain integers.
        let position: libc::off_t = unsafe { libc::lseek(fd, offset as libc::off_t, whence) };
        if position == -1 {
            Err(last_errno())
        } else {
            Ok(position as i64)
        }
    }
    fn record_lock(&self, fd: i32, lock: RecordLock) -> CallResult<()> {
        let mut key: libc::flock = lock_record::empty_lock_record();
        key.l_whence = libc::SEEK_SET as _;
        let (command, kind) = match lock {
            RecordLock::Read => (libc::F_SETLKW, libc::F_RDLCK),
            RecordLock::Write => (libc::F_SETLKW, libc::F_WRLCK),
            RecordLock::Unlock => (libc::F_SETLK, libc::F_UNLCK),
        };
        key.l_type = kind as _;
        // SAFETY: key is a valid flock alive for the call.
        status_zero(unsafe { libc::fcntl(fd, command, &key) })
    }
    fn rename(&self, from: &[u8], to: &[u8]) -> CallResult<()> {
        let (from, to) = (c_path(from), c_path(to));
        path_result(std::fs::rename(native_path(&from), native_path(&to)))
    }
    fn unlink(&self, path: &[u8]) -> CallResult<()> {
        let name: CString = c_path(path);
        path_result(std::fs::remove_file(native_path(&name)))
    }
    fn rmdir(&self, path: &[u8]) -> CallResult<()> {
        let name: CString = c_path(path);
        path_result(std::fs::remove_dir(native_path(&name)))
    }
    fn mkdir(&self, path: &[u8], mode: u32) -> CallResult<()> {
        let name: CString = c_path(path);
        path_result(
            std::fs::DirBuilder::new()
                .mode(mode)
                .create(native_path(&name)),
        )
    }
    fn chmod(&self, path: &[u8], mode: u32) -> CallResult<()> {
        let name: CString = c_path(path);
        // Retain all bits: from_raw_mode would strip file-type bits before the OS sees them.
        let mode = rustix::fs::Mode::from_bits_retain(mode as _);
        rustix::fs::chmod(name.as_c_str(), mode).map_err(|error| Errno(error.raw_os_error()))
    }
    fn umask(&self, mask: u32) -> u32 {
        let mask = rustix::fs::Mode::from_bits_retain(mask as _);
        rustix::process::umask(mask).bits() as u32
    }
    fn mkstemp(&self, template: &[u8]) -> CallResult<(i32, Vec<u8>)> {
        let mut name: Vec<u8> = c_path(template).into_bytes_with_nul();
        // SAFETY: name is a writable NUL-terminated buffer that mkstemp edits in place.
        let fd: libc::c_int = unsafe { libc::mkstemp(name.as_mut_ptr().cast()) };
        name.pop();
        if fd < 0 {
            Err(last_errno())
        } else {
            Ok((fd, name))
        }
    }
    fn mkdtemp(&self, template: &[u8]) -> CallResult<Vec<u8>> {
        let mut name: Vec<u8> = c_path(template).into_bytes_with_nul();
        // SAFETY: name is a writable NUL-terminated buffer that mkdtemp edits in place.
        let made: *mut libc::c_char = unsafe { libc::mkdtemp(name.as_mut_ptr().cast()) };
        name.pop();
        if made.is_null() {
            Err(last_errno())
        } else {
            Ok(name)
        }
    }
    fn read_directory(&self, path: &[u8]) -> CallResult<Vec<Vec<u8>>> {
        let mut cursor = self.open_directory(path)?;
        Ok(std::iter::from_fn(|| cursor.next_name()).collect())
    }
    fn open_directory(&self, path: &[u8]) -> CallResult<Box<dyn DirectoryEntries>> {
        directory_cursor::open_cursor(path)
    }
    fn local_time(&self, seconds: i64) -> Option<LocalTime> {
        let time: libc::time_t = seconds as libc::time_t;
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let mut fields = crate::native_calendar::empty_calendar();
        // SAFETY: an all-zero tm is a valid value of this plain C struct.
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        let mut fields: libc::tm = unsafe { std::mem::zeroed() };
        // SAFETY: time and fields are valid for the call; localtime_r is reentrant.
        let result: *mut libc::tm = unsafe { libc::localtime_r(&time, &mut fields) };
        (!result.is_null()).then_some(LocalTime {
            month: fields.tm_mon + 1,
            day: fields.tm_mday,
            year: fields.tm_year + 1900,
            hour: fields.tm_hour,
            minute: fields.tm_min,
            second: fields.tm_sec,
            millisecond: 0,
        })
    }
}
