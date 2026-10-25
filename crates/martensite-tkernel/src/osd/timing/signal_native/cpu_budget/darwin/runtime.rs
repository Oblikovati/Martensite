//! Preparation stays outside expiry; the terminal path never owns a numeric child PID (#289).
#![allow(unsafe_code)]

use super::{child, inventory, private_pair};
use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::UnixStream;

type PublicFork = unsafe extern "C" fn() -> libc::pid_t;

/// Owns a single terminal attempt; `PreparedWriter::new()` binds before the resource cap.
pub(in super::super) struct PreparedWriter {
    primitive: PublicFork,
    response: UnixStream,
    acknowledgement: UnixStream,
}

impl PreparedWriter {
    /// Rejects unvalidated older semantics; `PreparedWriter::new()` creates no child.
    pub(in super::super) fn new() -> io::Result<Self> {
        let names = nix::sys::utsname::uname().map_err(io::Error::from)?;
        require_full_fork_release(names.release().as_bytes())?;
        prepare_native_calls()?;
        let primitive = resolve_public_fork()?;
        let (response, acknowledgement) = private_pair()?;
        Ok(Self {
            primitive,
            response,
            acknowledgement,
        })
    }

    /// Consumes the attempt after terminal expiry; `writer.write_before_exit(message, deadline)`.
    pub(in super::super) fn write_before_exit(
        mut self,
        message: &[u8],
        deadline: u64,
    ) -> io::Result<()> {
        let mut entries = [const { std::mem::MaybeUninit::uninit() }; inventory::STACK_ENTRIES];
        let attempt = child::ChildAttempt {
            message,
            acknowledgement: self.acknowledgement.as_raw_fd(),
            parent: parent_identity(),
            deadline,
            entries: &mut entries,
        };
        self.spawn_output(attempt)?;
        drop(self.acknowledgement);
        read_outcome(&mut self.response)
    }

    fn spawn_output(&self, attempt: child::ChildAttempt<'_>) -> io::Result<()> {
        // SAFETY: the validated Mac12+ public primitive skips application atfork callbacks.
        // Only the checked native child path runs after its return of zero (#289).
        let spawned = unsafe { (self.primitive)() };
        if spawned == 0 {
            attempt.finish();
        }
        if spawned < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

fn parent_identity() -> i32 {
    rustix::process::getpid().as_raw_pid()
}

fn read_outcome(response: &mut impl Read) -> io::Result<()> {
    let mut bytes = [0; size_of::<libc::c_int>()];
    response.read_exact(&mut bytes)?;
    let failure = libc::c_int::from_ne_bytes(bytes);
    if failure != 0 {
        return Err(io::Error::from_raw_os_error(failure));
    }
    Ok(())
}

fn require_full_fork_release(release: &[u8]) -> io::Result<()> {
    let major = release
        .split(|&byte| byte == b'.')
        .next()
        .unwrap_or_default();
    let parsed = std::str::from_utf8(major)
        .ok()
        .and_then(|text| text.parse::<u32>().ok());
    if parsed.is_some_and(|version| version >= 21) {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        format!(
            "CPU diagnostic received Darwin release {release:?}; expected validated full-fork semantics from Darwin 21"
        ),
    ))
}

fn resolve_public_fork() -> io::Result<PublicFork> {
    // SAFETY: this static NUL-terminated public symbol is resolved in regular initialization.
    let address = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"vfork".as_ptr()) };
    if address.is_null() {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "CPU diagnostic requires the public vfork symbol; received null",
        ));
    }
    // SAFETY: Darwin's documented vfork symbol has this exact C signature. Mac12+
    // gives it full-fork semantics; this deprecated capability may still refuse a call.
    Ok(unsafe { std::mem::transmute::<*mut libc::c_void, PublicFork>(address) })
}

fn prepare_native_calls() -> io::Result<()> {
    inventory::preflight().map_err(io::Error::from_raw_os_error)?;
    super::super::clock::monotonic_nanoseconds().ok_or_else(io::Error::last_os_error)?;
    bind_timer_without_change();
    query_alarm_action()?;
    let _ = rustix::process::getppid();
    Ok(())
}

fn query_alarm_action() -> io::Result<()> {
    let mut old = super::super::super::action(libc::SIG_DFL, 0);
    // SAFETY: querying this initialized record changes no application signal disposition.
    if unsafe { libc::sigaction(libc::SIGALRM, std::ptr::null(), &mut old) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn bind_timer_without_change() {
    let timer = libc::itimerval {
        it_interval: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: libc::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };
    // SAFETY: an invalid selector binds the native wrapper without changing any timer (#289).
    unsafe { libc::setitimer(-1, &timer, std::ptr::null_mut()) };
}

#[cfg(test)]
mod tests {
    use std::io::{self, Cursor, Read};

    struct DiagnosticReply {
        payload: Cursor<Vec<u8>>,
    }

    impl Read for DiagnosticReply {
        fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
            let offered = output.len().min(1);
            self.payload.read(&mut output[..offered])
        }
    }

    #[test]
    fn short_native_reads_preserve_failure_and_missing_reply() {
        for failure in [0_i32, libc::EPIPE, libc::EOVERFLOW] {
            let mut reply = DiagnosticReply {
                payload: Cursor::new(failure.to_ne_bytes().to_vec()),
            };
            match super::read_outcome(&mut reply) {
                Ok(()) => assert_eq!(failure, 0),
                Err(cause) => assert_eq!(cause.raw_os_error(), Some(failure)),
            }
        }
        let mut missing = DiagnosticReply {
            payload: Cursor::new(vec![0, 0]),
        };
        assert_eq!(
            super::read_outcome(&mut missing).unwrap_err().kind(),
            io::ErrorKind::UnexpectedEof
        );
    }

    #[test]
    fn older_full_fork_semantics_are_never_assumed() {
        assert!(super::require_full_fork_release(b"21.0.0").is_ok());
        assert!(super::require_full_fork_release(b"25.0.0").is_ok());
        for release in [b"20.6.0".as_slice(), b"", b"invalid"] {
            assert_eq!(
                super::require_full_fork_release(release)
                    .unwrap_err()
                    .kind(),
                std::io::ErrorKind::Unsupported
            );
        }
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<super::PreparedWriter>();
    }
}
