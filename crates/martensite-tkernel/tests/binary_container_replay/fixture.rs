//! Fixed host-order bytes independent of the container writer.
pub(super) fn empty_container() -> Vec<u8> {
    let fields = [7, 95, 95, 99, 99, 103, 103, 107, 107, 111, 111, 111];
    let mut bytes = b"BINFILE".to_vec();
    bytes.extend_from_slice(&[1, 2, 3, 4]);
    for field in fields {
        bytes.extend_from_slice(&i32::to_ne_bytes(field));
    }
    bytes.extend_from_slice(&[0; 52]);
    bytes.extend_from_slice(b"body");
    bytes
}

pub(super) fn shifted_container() -> Vec<u8> {
    let mut bytes = empty_container();
    for start in (11..59).step_by(4) {
        let field = i32::from_ne_bytes(bytes[start..start + 4].try_into().unwrap());
        bytes[start..start + 4].copy_from_slice(&(field + 5).to_ne_bytes());
    }
    [b"prior".as_slice(), &bytes].concat()
}
