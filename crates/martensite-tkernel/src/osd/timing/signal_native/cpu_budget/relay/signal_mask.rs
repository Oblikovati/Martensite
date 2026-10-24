//! The diagnostic worker must not receive the action whose completion it reports.
#![allow(unsafe_code)]

pub(super) fn block_relay_signals() -> std::io::Result<()> {
    // SAFETY: the mask is initialized before pthread_sigmask reads it; valid
    // signal numbers change only this worker's mask, never the caller's (#278).
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        if libc::sigemptyset(&mut mask) != 0
            || libc::sigaddset(&mut mask, libc::SIGXCPU) != 0
            || libc::sigaddset(&mut mask, libc::SIGPIPE) != 0
        {
            return Err(std::io::Error::last_os_error());
        }
        let status = libc::pthread_sigmask(libc::SIG_BLOCK, &mask, std::ptr::null_mut());
        if status != 0 {
            return Err(std::io::Error::from_raw_os_error(status));
        }
    }
    Ok(())
}
