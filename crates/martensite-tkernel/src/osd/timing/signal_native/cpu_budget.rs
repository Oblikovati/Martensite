//! The CPU-budget exit action cannot allocate, lock or run destructors (#278).
#![allow(unsafe_code)]

pub(super) extern "C" fn cpu_budget_exit(_signal: i32) {
    let limit = crate::osd::reported_process_cpu_limit();
    let (message, length) = cpu_budget_message(limit);
    // SAFETY: the initialized stack slice is live for this bounded write. write and
    // _exit are async-signal-safe; neither uses Rust I/O, allocation or destructors.
    unsafe {
        libc::write(1, message.as_ptr().cast(), length);
        libc::_exit(2);
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
