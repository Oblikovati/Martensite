//! The CPU-budget exit action cannot allocate, lock or run destructors (#278).
#![allow(unsafe_code)]

mod clock;
mod relay;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

static REQUEST_FD: AtomicI32 = AtomicI32::new(-1);
static REQUEST_QUEUED: AtomicBool = AtomicBool::new(false);
static OUTPUT_COMPLETE: AtomicBool = AtomicBool::new(false);

pub(super) fn prepare_diagnostic_relay() -> std::io::Result<()> {
    relay::prepare()
}

pub(super) extern "C" fn cpu_budget_exit(_signal: i32) {
    if !REQUEST_QUEUED.swap(true, Ordering::AcqRel) {
        enqueue_diagnostic();
    }
    clock::wait_for_diagnostic();
    // SAFETY: _exit is async-signal-safe and terminates without Rust destructors.
    unsafe { libc::_exit(2) }
}

fn enqueue_diagnostic() {
    let descriptor = REQUEST_FD.load(Ordering::Acquire);
    let request = crate::osd::reported_process_cpu_limit().to_ne_bytes();
    // SAFETY: initialization retains this nonblocking descriptor for process life;
    // the fixed stack request is live, and write is async-signal-safe (#278).
    let written = unsafe { libc::write(descriptor, request.as_ptr().cast(), request.len()) };
    if written != request.len() as isize {
        // An unavailable channel must never turn expiry into a wait for output.
        OUTPUT_COMPLETE.store(true, Ordering::Release);
    }
}

fn cpu_budget_message(limit: i64) -> ([u8; 64], usize) {
    let mut message = [0; 64];
    let mut length = 0;
    append_fragment(&mut message, &mut length, b"Process killed by CPU limit  (");
    if limit < 0 {
        append_fragment(&mut message, &mut length, b"-");
    }
    let mut digits = [0; 20];
    let text = unsigned_digits(limit.unsigned_abs(), &mut digits);
    append_fragment(&mut message, &mut length, text);
    append_fragment(&mut message, &mut length, b" sec)\n");
    (message, length)
}

fn unsigned_digits(mut value: u64, digits: &mut [u8; 20]) -> &[u8] {
    let mut start = digits.len();
    loop {
        start -= 1;
        digits[start] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            return &digits[start..];
        }
    }
}

fn append_fragment(message: &mut [u8; 64], length: &mut usize, fragment: &[u8]) {
    // Prefix, sign, at most 20 digits and suffix fit in 64 bytes for every i64.
    let end = *length + fragment.len();
    message[*length..end].copy_from_slice(fragment);
    *length = end;
}

#[cfg(test)]
mod tests {
    use super::cpu_budget_message;

    #[test]
    fn signed_budget_messages_fit_the_signal_stack_buffer() {
        for (limit, expected) in [
            (0, "Process killed by CPU limit  (0 sec)\n"),
            (-1, "Process killed by CPU limit  (-1 sec)\n"),
            (120, "Process killed by CPU limit  (120 sec)\n"),
            (
                i64::MIN,
                "Process killed by CPU limit  (-9223372036854775808 sec)\n",
            ),
            (
                i64::MAX,
                "Process killed by CPU limit  (9223372036854775807 sec)\n",
            ),
        ] {
            let (actual, length) = cpu_budget_message(limit);
            assert_eq!(&actual[..length], expected.as_bytes());
        }
    }
}
