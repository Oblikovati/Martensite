//! Darwin directs a pipe's SIGPIPE at the process, so a masked thread is insufficient (#289).
#![allow(unsafe_code)]

mod child;
mod inventory;
mod runtime;

use super::{REQUEST_QUEUED, clock, enqueue_snapshot};
pub(super) use runtime::PreparedWriter;
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::atomic::Ordering;

/// Owns fresh private sockets; `darwin::private_pair()` never modifies an application handle.
pub(super) fn private_pair() -> io::Result<(UnixStream, UnixStream)> {
    let (sender, receiver) = UnixStream::pair()?;
    suppress_private_sigpipe(&sender)?;
    suppress_private_sigpipe(&receiver)?;
    Ok((sender, receiver))
}

fn suppress_private_sigpipe(socket: &UnixStream) -> io::Result<()> {
    let enabled: libc::c_int = 1;
    // SAFETY: this is a newly owned socket; the initialized integer matches SO_NOSIGPIPE.
    let status = unsafe {
        libc::setsockopt(
            socket.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_NOSIGPIPE,
            (&enabled as *const libc::c_int).cast(),
            size_of_val(&enabled) as libc::socklen_t,
        )
    };
    if status != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

/// Shares one absolute grace deadline with the private writer; `darwin::wait_for_expiry_output()`.
pub(super) fn wait_for_expiry_output() {
    let Some(deadline) = clock::diagnostic_deadline() else {
        return;
    };
    if !REQUEST_QUEUED.swap(true, Ordering::AcqRel) {
        let mut request = [0; 16];
        request[..8].copy_from_slice(&crate::osd::reported_process_cpu_limit().to_ne_bytes());
        request[8..].copy_from_slice(&deadline.to_ne_bytes());
        enqueue_snapshot(&request);
    }
    clock::wait_until_deadline(deadline);
}
