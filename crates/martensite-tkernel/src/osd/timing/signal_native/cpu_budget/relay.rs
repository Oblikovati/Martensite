//! Retained channel ownership prevents fd reuse while the CPU action can run.
use super::{OUTPUT_COMPLETE, REQUEST_FD, cpu_budget_message};
use std::io::{self, Read};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::OnceLock;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{SyncSender, sync_channel};

mod signal_mask;

#[cfg(target_os = "macos")]
type WorkerOutput = super::darwin::PreparedWriter;
#[cfg(not(target_os = "macos"))]
type WorkerOutput = std::io::Stdout;
#[cfg(target_os = "macos")]
const REQUEST_BYTES: usize = 16;
#[cfg(not(target_os = "macos"))]
const REQUEST_BYTES: usize = 8;

static DIAGNOSTIC_CHANNEL: OnceLock<UnixStream> = OnceLock::new();

pub(super) fn prepare() -> io::Result<()> {
    let channel = match DIAGNOSTIC_CHANNEL.get() {
        Some(channel) => channel,
        None => {
            // The setter serializes startup; only successful ownership is cached.
            // A host resource shortage must remain retryable after it clears (#278).
            let candidate = start_relay()?;
            DIAGNOSTIC_CHANNEL.get_or_init(|| candidate)
        }
    };
    REQUEST_FD.store(channel.as_raw_fd(), Ordering::Release);
    Ok(())
}

fn start_relay() -> io::Result<UnixStream> {
    let (request, receiver) = request_pair()?;
    request.set_nonblocking(true)?;
    let (send, ready) = sync_channel(1);
    std::thread::Builder::new()
        .name("process-cpu-diagnostic".into())
        .spawn(move || run_relay(receiver, send))?;
    ready.recv().map_err(io::Error::other)??;
    Ok(request)
}

fn run_relay(mut receiver: UnixStream, ready: SyncSender<io::Result<()>>) {
    let Some(output) = worker_ready(ready) else {
        return;
    };
    let mut request = [0; REQUEST_BYTES];
    if receiver.read_exact(&mut request).is_err() {
        return;
    }
    let limit = i64::from_ne_bytes(std::array::from_fn(|index| request[index]));
    let (message, length) = cpu_budget_message(limit);
    finish_output(output, &message[..length], &request);
}

fn worker_ready(ready: SyncSender<io::Result<()>>) -> Option<WorkerOutput> {
    match prepare_output() {
        Ok(output) => ready.send(Ok(())).ok().map(|()| output),
        Err(cause) => {
            let _ = ready.send(Err(cause));
            None
        }
    }
}

fn prepare_output() -> io::Result<WorkerOutput> {
    signal_mask::block_relay_signals()?;
    #[cfg(target_os = "macos")]
    return WorkerOutput::new();
    #[cfg(not(target_os = "macos"))]
    Ok(std::io::stdout())
}

fn request_pair() -> io::Result<(UnixStream, UnixStream)> {
    #[cfg(target_os = "macos")]
    return super::darwin::private_pair();
    #[cfg(not(target_os = "macos"))]
    UnixStream::pair()
}

#[cfg(target_os = "macos")]
fn finish_output(output: WorkerOutput, message: &[u8], request: &[u8; REQUEST_BYTES]) {
    let deadline = u64::from_ne_bytes(std::array::from_fn(|index| request[index + 8]));
    let _ = output.write_before_exit(message, deadline);
    OUTPUT_COMPLETE.store(true, Ordering::Release);
}

#[cfg(not(target_os = "macos"))]
fn finish_output(stdout: WorkerOutput, message: &[u8], _request: &[u8; REQUEST_BYTES]) {
    if write_diagnostic(&stdout, message).is_ok() {
        OUTPUT_COMPLETE.store(true, Ordering::Release);
    }
}

#[cfg(not(target_os = "macos"))]
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
