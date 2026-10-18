use super::baseline;
fn with_fields(fields: [i32; 12], payload: &[u8]) -> Vec<u8> {
    let mut bytes = baseline::empty_container();
    bytes.truncate(59);
    for (offset, value) in (11..59).step_by(4).zip(fields) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
    }
    bytes.extend_from_slice(payload);
    bytes.extend_from_slice(b"body");
    bytes
}
fn integer(output: &mut Vec<u8>, value: i32) {
    output.extend_from_slice(&value.to_ne_bytes());
}
fn ascii(output: &mut Vec<u8>, bytes: &[u8]) {
    integer(output, bytes.len() as i32);
    output.extend_from_slice(bytes);
}
fn extended(output: &mut Vec<u8>, units: &[u16]) {
    integer(output, units.len() as i32);
    for unit in units {
        output.extend_from_slice(&unit.to_ne_bytes());
    }
}
pub(super) fn metadata() -> Vec<u8> {
    let mut payload = Vec::new();
    integer(&mut payload, 0);
    for bytes in [b"v\0z".as_slice(), b"2026", b"schema", b"v2"] {
        ascii(&mut payload, bytes);
    }
    extended(&mut payload, &[65, 937]);
    ascii(&mut payload, b"v3");
    extended(&mut payload, &[0xd83d, 0xde00]);
    integer(&mut payload, 2);
    ascii(&mut payload, b"x");
    ascii(&mut payload, b"second");
    integer(&mut payload, 1);
    extended(&mut payload, &[67]);
    payload.extend_from_slice(&[0; 12]);
    with_fields(
        [7, 135, 135, 145, 145, 149, 149, 153, 153, 157, 157, 157],
        &payload,
    )
}
pub(super) fn unattached_root() -> Vec<u8> {
    let mut payload = vec![0; 44];
    integer(&mut payload, 1);
    integer(&mut payload, 1);
    ascii(&mut payload, b"doc");
    integer(&mut payload, 0);
    integer(&mut payload, 0);
    with_fields(
        [7, 95, 95, 99, 99, 103, 103, 122, 122, 126, 126, 126],
        &payload,
    )
}
