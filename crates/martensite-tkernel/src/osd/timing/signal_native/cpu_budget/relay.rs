//! Retained channel ownership prevents fd reuse while the CPU action can run.
use super::{OUTPUT_COMPLETE, REQUEST_FD, cpu_budget_message};
use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::OnceLock;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{SyncSender, sync_channel};

mod signal_mask;

static DIAGNOSTIC_CHANNEL: OnceLock<Result<UnixStream, io::Error>> = OnceLock::new();

pub(super) fn prepare() -> io::Result<()> {
    match DIAGNOSTIC_CHANNEL.get_or_init(start_relay) {
        Ok(channel) => {
            REQUEST_FD.store(channel.as_raw_fd(), Ordering::Release);
            Ok(())
        }
        Err(cause) => Err(copy_setup_error(cause)),
    }
}

fn start_relay() -> io::Result<UnixStream> {
    let (request, receiver) = UnixStream::pair()?;
    request.set_nonblocking(true)?;
    let (send, ready) = sync_channel(1);
    std::thread::Builder::new()
        .name("process-cpu-diagnostic".into())
        .spawn(move || run_relay(receiver, send))?;
    ready.recv().map_err(io::Error::other)??;
    Ok(request)
}

fn run_relay(mut receiver: UnixStream, ready: SyncSender<io::Result<()>>) {
    let stdout = std::io::stdout();
    let mask_result = signal_mask::block_relay_signals();
    let mask_ready = mask_result.is_ok();
    if ready.send(mask_result).is_err() || !mask_ready {
        return;
    }
    let mut request = [0; 8];
    if receiver.read_exact(&mut request).is_err() {
        return;
    }
    let (message, length) = cpu_budget_message(i64::from_ne_bytes(request));
    if write_diagnostic(&stdout, &message[..length]).is_ok() {
        OUTPUT_COMPLETE.store(true, Ordering::Release);
    }
}

fn write_diagnostic(stdout: &std::io::Stdout, mut remaining: &[u8]) -> io::Result<()> {
    while !remaining.is_empty() {
        match rustix::io::write(stdout, remaining) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::WriteZero)),
            Ok(written) => remaining = &remaining[written..],
            Err(rustix::io::Errno::INTR) => continue,
            Err(cause) => return Err(cause.into()),
        }
    }
    Ok(())
}

fn copy_setup_error(cause: &io::Error) -> io::Error {
    match cause.raw_os_error() {
        Some(errno) => io::Error::from_raw_os_error(errno),
        None => io::Error::new(cause.kind(), cause.to_string()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn setup_cause_retains_os_error_identity() {
        let cause = std::io::Error::from_raw_os_error(libc::EACCES);
        assert_eq!(
            super::copy_setup_error(&cause).raw_os_error(),
            Some(libc::EACCES)
        );
        let cause = std::io::Error::other("relay setup refused");
        assert_eq!(
            super::copy_setup_error(&cause).to_string(),
            "relay setup refused"
        );
    }
}
