use martensite_tkernel::fsd::FileHeader;

#[cfg(target_endian = "little")]
pub(super) const HEADER: [u8; 52] = [
    1, 2, 3, 4, 11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0, 55, 0, 0, 0, 66, 0, 0, 0, 77,
    0, 0, 0, 88, 0, 0, 0, 99, 0, 0, 0, 110, 0, 0, 0, 121, 0, 0, 0, 132, 0, 0, 0,
];
#[cfg(target_endian = "big")]
pub(super) const HEADER: [u8; 52] = [
    1, 2, 3, 4, 0, 0, 0, 11, 0, 0, 0, 22, 0, 0, 0, 33, 0, 0, 0, 44, 0, 0, 0, 55, 0, 0, 0, 66, 0, 0,
    0, 77, 0, 0, 0, 88, 0, 0, 0, 99, 0, 0, 0, 110, 0, 0, 0, 121, 0, 0, 0, 132,
];
pub(super) fn expected_header() -> FileHeader {
    let probe = if cfg!(target_endian = "little") {
        0x0403_0201
    } else {
        0x0102_0304
    };
    FileHeader::from_fields([probe, 11, 22, 33, 44, 55, 66, 77, 88, 99, 110, 121, 132])
}
pub(super) fn partial_first_field() -> i32 {
    if cfg!(target_endian = "little") {
        0x5555_0201
    } else {
        0x0102_5555
    }
}
pub(super) fn short_initial_header() -> FileHeader {
    let mut fields = [0x5555_5555; 13];
    fields[0] = partial_first_field();
    FileHeader::from_fields(fields)
}
