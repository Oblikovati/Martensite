//! Independent MPFR references for exact test inputs; see #201 evidence.
pub(super) const CIRCLE_REFERENCE_JET: [[u64; 3]; 4] = [
    [0x3ff6a09e667f3bcd, 0x3ff6a09e667f3bcc, 0x0000000000000000],
    [0xbff6a09e667f3bcc, 0x3ff6a09e667f3bcd, 0x0000000000000000],
    [0xbff6a09e667f3bcd, 0xbff6a09e667f3bcc, 0x0000000000000000],
    [0x3ff6a09e667f3bcc, 0xbff6a09e667f3bcd, 0x0000000000000000],
];
pub(super) const ELLIPSE_REFERENCE_JET: [[u64; 3]; 4] = [
    [0x4000f876ccdf6cda, 0x3ff6a09e667f3bcc, 0x0000000000000000],
    [0xc000f876ccdf6cd9, 0x3ff6a09e667f3bcd, 0x0000000000000000],
    [0xc000f876ccdf6cda, 0xbff6a09e667f3bcc, 0x0000000000000000],
    [0x4000f876ccdf6cd9, 0xbff6a09e667f3bcd, 0x0000000000000000],
];
pub(super) const HYPERBOLA_REFERENCE_JET: [[u64; 3]; 4] = [
    [0x400b102249405d39, 0x3ff0acd00fe63b97, 0x0000000000000000],
    [0x3ff9033817d95962, 0x40020ac1862ae8d0, 0x0000000000000000],
    [0x400b102249405d39, 0x3ff0acd00fe63b97, 0x0000000000000000],
    [0x3ff9033817d95962, 0x40020ac1862ae8d0, 0x0000000000000000],
];
