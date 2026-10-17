//! Darwin exposes a scalar signal mask; other Unix layouts need separate proof (#89).
#![forbid(unsafe_code)]

pub(super) fn action(handler: libc::sighandler_t, flags: i32) -> libc::sigaction {
    libc::sigaction {
        sa_sigaction: handler,
        sa_mask: 0,
        sa_flags: flags,
    }
}
