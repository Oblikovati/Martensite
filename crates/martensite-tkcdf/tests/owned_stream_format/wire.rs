//! Independent standard binary-container layout: host-order fields and empty sections.
pub fn binary(lines: &[&[u8]], origin: i32) -> Vec<u8> {
    let info = information(lines);
    let end = origin + 7 + 13 * 4 + info.len() as i32;
    let offsets = section_offsets(end, origin);
    let mut bytes = b"BINFILE".to_vec();
    bytes.extend_from_slice(&[1, 2, 3, 4]);
    for offset in offsets {
        integer(&mut bytes, offset);
    }
    bytes.extend_from_slice(&info);
    for _ in 0..4 {
        integer(&mut bytes, 0);
    }
    bytes
}

fn section_offsets(end: i32, origin: i32) -> [i32; 12] {
    let comment = end + 4;
    let types = comment + 4;
    let roots = types + 4;
    let refs = roots + 4;
    [
        origin + 7,
        end,
        end,
        comment,
        comment,
        types,
        types,
        roots,
        roots,
        refs,
        refs,
        refs,
    ]
}

fn information(lines: &[&[u8]]) -> Vec<u8> {
    let mut bytes = Vec::new();
    // Object count, four ASCII fields, application name, application version, type.
    for _ in 0..8 {
        integer(&mut bytes, 0);
    }
    integer(&mut bytes, lines.len() as i32);
    for line in lines {
        integer(&mut bytes, line.len() as i32);
        bytes.extend_from_slice(line);
    }
    bytes
}

fn integer(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_ne_bytes());
}

pub fn ordinary_binary(origin: i32) -> Vec<u8> {
    binary(&[b"FILE_FORMAT: first", b"FILE_FORMAT: last"], origin)
}

pub fn short_string(extended: bool) -> Vec<u8> {
    let mut bytes = ordinary_binary(0);
    let field = if extended { 79 } else { 63 };
    bytes.truncate(field);
    integer(&mut bytes, 5);
    bytes.extend_from_slice(b"abc");
    bytes
}
