//! Preserve the native DWORD mapping without executing long or infinite waits (#89).
#![forbid(unsafe_code)]

pub(super) fn from_seconds(seconds: i32) -> u32 {
    seconds.wrapping_mul(1_000) as u32
}

pub(super) fn from_milliseconds(milliseconds: i32) -> u32 {
    milliseconds as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn seconds_keep_signed_overflow_and_dword_wrap() {
        let cases = [
            (0, 0),
            (1, 1_000),
            (-1, 4_294_966_296),
            (i32::MIN, 0),
            (i32::MIN + 1, 1_000),
            (i32::MAX, 4_294_966_296),
            (2_147_483, 2_147_483_000),
            (2_147_484, 2_147_484_000),
            (4_294_967, 4_294_967_000),
            (4_294_968, 704),
        ];
        for (argument, expected) in cases {
            assert_eq!(
                super::from_seconds(argument),
                expected,
                "seconds {argument}"
            );
        }
    }

    #[test]
    fn milliseconds_keep_infinite_sentinel_and_signed_cast() {
        let cases = [
            (0, 0),
            (1, 1),
            (-1, u32::MAX),
            (i32::MIN, 2_147_483_648),
            (i32::MAX, 2_147_483_647),
        ];
        for (argument, expected) in cases {
            assert_eq!(
                super::from_milliseconds(argument),
                expected,
                "milliseconds {argument}"
            );
        }
    }
}
