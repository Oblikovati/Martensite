//! Expected bytes are assembled from the documented layout, not the codec under test.
/// Encode literal matrix values independently; `translation_record([1., 2., 3.])`.
pub(super) fn translation_record(offset: [f64; 3]) -> Vec<u8> {
    let rows: [[f64; 4]; 3] = [
        [1., 0., 0., offset[0]],
        [0., 1., 0., offset[1]],
        [0., 0., 1., offset[2]],
    ];
    matrix_record(rows)
}

/// Encode twelve explicit matrix scalars; `matrix_record([[0.; 4]; 3])`.
pub(super) fn matrix_record(rows: [[f64; 4]; 3]) -> Vec<u8> {
    let mut bytes = vec![1];
    for value in rows.into_iter().flatten() {
        bytes.extend(value.to_ne_bytes());
    }
    bytes
}

/// Encode explicit index/power pairs; `chain_record(&[(1, -2)])`.
pub(super) fn chain_record(terms: &[(i32, i32)]) -> Vec<u8> {
    let mut bytes = vec![2];
    for (index, power) in terms {
        bytes.extend(index.to_ne_bytes());
        bytes.extend(power.to_ne_bytes());
    }
    bytes.extend(0_i32.to_ne_bytes());
    bytes
}

/// Prefix raw records by their text count; `table(&[vec![3]])`.
pub(super) fn table(records: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = format!("Locations {}\n", records.len()).into_bytes();
    for record in records {
        bytes.extend(record);
    }
    bytes
}

/// Assemble the literal failure envelope; `wrapped("Standard_Failure", b"x")`.
pub(super) fn wrapped(kind: &str, message: &[u8]) -> Vec<u8> {
    let mut bytes = b"EXCEPTION in BinTools_LocationSet::Read(..)\n".to_vec();
    bytes.extend(kind.as_bytes());
    if !message.is_empty() {
        bytes.extend(b": ");
        bytes.extend(message);
    }
    bytes.push(b'\n');
    bytes
}

/// Preserve the inserted byte and C-string truncation; `unknown_kind(255)`.
pub(super) fn unknown_kind(kind: u8) -> Vec<u8> {
    let mut cause = b"Unexpected location's type = ".to_vec();
    if kind != 0 {
        cause.push(kind);
        cause.push(b'\n');
    }
    wrapped("Standard_Failure", &cause)
}
