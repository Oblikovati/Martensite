//! The child observes its own descriptors; neither a parent snapshot nor a lowered limit is sufficient (#289).
#![allow(unsafe_code)]

use std::mem::MaybeUninit;
pub(super) const STACK_ENTRIES: usize = 4096;
type Entry = MaybeUninit<libc::proc_fdinfo>;

/// Binds the native inventory wrapper before expiry; `inventory::preflight()` creates no handles.
pub(super) fn preflight() -> Result<(), i32> {
    let bytes = query(std::ptr::null_mut(), 0);
    entry_count(bytes, usize::MAX).map(|_| ())
}

/// Refuses incomplete closure before output; `inventory::close_inherited(ack, &mut entries)`.
pub(super) fn close_inherited(
    acknowledgement: i32,
    entries: &mut [Entry; STACK_ENTRIES],
) -> Result<(), i32> {
    let required = entry_count(query(std::ptr::null_mut(), 0), entries.len())?;
    let bytes = size_of_val(entries);
    let used = query(entries.as_mut_ptr().cast(), bytes as i32);
    let count = entry_count(used, entries.len())?;
    if count > required {
        return Err(libc::EOVERFLOW);
    }
    for entry in entries.iter().take(count) {
        // SAFETY: the validated byte count covers exactly these initialized native records.
        close_record(unsafe { entry.assume_init_ref() }, acknowledgement)?;
    }
    Ok(())
}

fn close_record(entry: &libc::proc_fdinfo, acknowledgement: i32) -> Result<(), i32> {
    let descriptor = entry.proc_fd;
    if descriptor < 0 {
        return Err(libc::EIO);
    }
    if descriptor == libc::STDOUT_FILENO || descriptor == acknowledgement {
        return Ok(());
    }
    close_one(descriptor)
}

fn query(entries: *mut libc::c_void, bytes: i32) -> i32 {
    let process = rustix::process::getpid().as_raw_pid();
    // SAFETY: callers supply either a null sizing request or a live bounded native array.
    // proc_pidinfo reads this process only; its direct wrapper was bound before expiry.
    unsafe { libc::proc_pidinfo(process, libc::PROC_PIDLISTFDS, 0, entries, bytes) }
}

fn entry_count(bytes: i32, capacity: usize) -> Result<usize, i32> {
    let stride = size_of::<libc::proc_fdinfo>() as i32;
    if bytes <= 0 || bytes % stride != 0 {
        return Err(libc::EIO);
    }
    let count = (bytes / stride) as usize;
    if count > capacity {
        return Err(libc::EOVERFLOW);
    }
    Ok(count)
}

fn close_one(descriptor: i32) -> Result<(), i32> {
    loop {
        // This is the single-threaded private child. No descriptor can be
        // reopened between retries; the parent's descriptor table is independent.
        if nix::unistd::close(descriptor).is_ok() {
            return Ok(());
        }
        let failure = errno();
        if failure == libc::EBADF {
            return Ok(());
        }
        if failure != libc::EINTR {
            return Err(failure);
        }
    }
}

/// Reads this native thread's errno without allocation; `inventory::errno()`.
pub(super) fn errno() -> i32 {
    // Preserve raw values that a typed errno conversion could normalize.
    nix::errno::Errno::last_raw()
}

#[cfg(test)]
mod tests {
    #[test]
    fn malformed_and_truncated_inventory_never_exposes_uninitialized_entries() {
        assert_eq!(size_of::<libc::proc_fdinfo>(), 8);
        for bytes in [-1, 0, 1, 7, 9] {
            assert_eq!(super::entry_count(bytes, 2), Err(libc::EIO));
        }
        assert_eq!(super::entry_count(8, 1), Ok(1));
        assert_eq!(super::entry_count(24, 2), Err(libc::EOVERFLOW));
        let malformed = libc::proc_fdinfo {
            proc_fd: -1,
            proc_fdtype: 0,
        };
        assert_eq!(super::close_record(&malformed, 2), Err(libc::EIO));
    }
}

#[cfg(test)]
mod descriptor_tests;
