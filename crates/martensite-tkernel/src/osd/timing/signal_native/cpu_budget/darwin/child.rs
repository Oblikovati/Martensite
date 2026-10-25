//! Only native calls and checked stack storage execute after full-fork returns (#289).
#![allow(unsafe_code)]

use super::super::super::action;
use super::inventory;
use std::mem::MaybeUninit;

/// Borrows stack storage across full-fork; `attempt.finish()` never returns into copied application state.
pub(super) struct ChildAttempt<'a> {
    pub(super) message: &'a [u8],
    pub(super) acknowledgement: i32,
    pub(super) parent: i32,
    pub(super) deadline: u64,
    pub(super) entries: &'a mut [MaybeUninit<libc::proc_fdinfo>; inventory::STACK_ENTRIES],
}

impl ChildAttempt<'_> {
    /// Completes the isolated attempt without destructors; `attempt.finish()` belongs only to the child.
    pub(super) fn finish(self) -> ! {
        let outcome = prepare_deadline(self.parent, self.deadline)
            .and_then(|()| inventory::close_inherited(self.acknowledgement, self.entries))
            .and_then(|()| write_all(libc::STDOUT_FILENO, self.message));
        let failure = outcome.err().unwrap_or(0).to_ne_bytes();
        let _ = write_all(self.acknowledgement, &failure);
        exit_child()
    }
}

fn prepare_deadline(parent: i32, deadline: u64) -> Result<(), i32> {
    install_private_alarm()?;
    let now = super::super::clock::monotonic_nanoseconds().ok_or(libc::EIO)?;
    if rustix::process::getppid().map(|pid| pid.as_raw_pid()) != Some(parent) || now >= deadline {
        return Err(libc::ETIMEDOUT);
    }
    arm_private_timer(remaining_grace(deadline, now)?)
}

fn remaining_grace(deadline: u64, now: u64) -> Result<u64, i32> {
    let remaining = deadline
        .checked_sub(now)
        .filter(|&remaining| remaining != 0);
    match remaining {
        None => Err(libc::ETIMEDOUT),
        Some(remaining) if remaining > super::super::clock::DIAGNOSTIC_GRACE_NS => {
            Err(libc::EINVAL)
        }
        Some(remaining) => Ok(remaining),
    }
}

fn install_private_alarm() -> Result<(), i32> {
    let wanted = action(expire_private_writer as *const () as usize, 0);
    // SAFETY: this is the isolated child, not the caller. Initialized masks block
    // all ordinary signals except its private alarm; no application state changes.
    unsafe {
        let mut blocked: libc::sigset_t = std::mem::zeroed();
        if libc::sigaction(libc::SIGALRM, &wanted, std::ptr::null_mut()) != 0
            || libc::sigfillset(&mut blocked) != 0
            || libc::sigdelset(&mut blocked, libc::SIGALRM) != 0
            || libc::sigprocmask(libc::SIG_SETMASK, &blocked, std::ptr::null_mut()) != 0
        {
            return Err(inventory::errno());
        }
    }
    Ok(())
}

fn arm_private_timer(remaining: u64) -> Result<(), i32> {
    let micros = remaining.div_ceil(1_000);
    let timer = libc::itimerval {
        it_interval: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: libc::timeval {
            tv_sec: (micros / 1_000_000) as libc::time_t,
            tv_usec: (micros % 1_000_000) as libc::suseconds_t,
        },
    };
    // SAFETY: this initialized timer belongs only to the single-threaded child.
    if unsafe { libc::setitimer(libc::ITIMER_REAL, &timer, std::ptr::null_mut()) } != 0 {
        return Err(inventory::errno());
    }
    Ok(())
}

extern "C" fn expire_private_writer(_signal: i32) {
    // This process is born only after expiry. Its one-shot remaining grace also
    // ends a blocked write if the caller disappears; no process-global child state is needed.
    exit_child();
}

fn write_all(descriptor: i32, mut remaining: &[u8]) -> Result<(), i32> {
    while !remaining.is_empty() {
        // SAFETY: the stack buffer stays live; only this private process performs
        // the potentially blocking output write. Its alarm and parent exit are independent.
        let written =
            unsafe { libc::write(descriptor, remaining.as_ptr().cast(), remaining.len()) };
        if written > 0 {
            remaining = &remaining[written as usize..];
        } else if written == 0 {
            return Err(libc::EIO);
        } else if inventory::errno() != libc::EINTR {
            return Err(inventory::errno());
        }
    }
    Ok(())
}

fn exit_child() -> ! {
    // SAFETY: _exit is async-signal-safe; no copied application destructor may run.
    unsafe { libc::_exit(0) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn malformed_deadline_cannot_leave_an_unbounded_writer() {
        assert_eq!(super::remaining_grace(100_000_001, 1), Ok(100_000_000));
        assert_eq!(super::remaining_grace(100_000_002, 1), Err(libc::EINVAL));
        assert_eq!(super::remaining_grace(1, 1), Err(libc::ETIMEDOUT));
        assert_eq!(super::remaining_grace(0, 1), Err(libc::ETIMEDOUT));
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<super::ChildAttempt<'static>>();
    }
}
