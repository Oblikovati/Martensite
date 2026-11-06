//! Fixed-size raw first-bounce records retain miss sentinels and actual float bits.
use crate::{RenderError, error::rejected};
pub(super) const HIT_BYTES: usize = 409 * 32 * 4;
pub(super) const HIT_STORAGE_BYTES: usize = HIT_BYTES + 16;
pub(super) fn initial_hit_storage(mode: u32) -> Result<Vec<u8>, RenderError> {
    if mode > 1 {
        return Err(rejected(mode, "immutable hit mode0/1"));
    }
    let mut bytes = vec![0; HIT_STORAGE_BYTES];
    bytes[..4].copy_from_slice(&mode.to_ne_bytes());
    Ok(bytes)
}
pub(super) fn require_hit_storage(bytes: &[u8], mode: u32) -> Result<&[u8], RenderError> {
    if bytes.len() != HIT_STORAGE_BYTES
        || mode > 1
        || bytes[..4] != mode.to_ne_bytes()
        || bytes[4..16] != [0; 12]
    {
        return Err(rejected(
            format!("storage bytes={}, mode={mode}", bytes.len()),
            "exact immutable mode prefix and409x32 words",
        ));
    }
    Ok(&bytes[16..])
}
pub(super) fn require_hit_records(bytes: &[u8], mode: u32) -> Result<(), RenderError> {
    if bytes.len() != HIT_BYTES || mode > 1 {
        return Err(rejected(
            format!("bytes={}, mode={mode}", bytes.len()),
            "409 records of 32 words, mode0/1",
        ));
    }
    for (row, chunk) in bytes.chunks_exact(128).enumerate() {
        let words: Vec<u32> = chunk
            .chunks_exact(4)
            .map(|word| u32::from_ne_bytes(word.try_into().unwrap()))
            .collect();
        require_hit_row(&words, row as u32, mode)?;
    }
    Ok(())
}
fn require_hit_row(words: &[u32], row: u32, mode: u32) -> Result<(), RenderError> {
    if words[3] != row + 1 || words[21] != mode || words[22] != row || words[23] != 1 {
        return Err(rejected(
            format!("row={row}, header={:?}", &words[20..24]),
            "one actual first-bounce record in physical/logical row",
        ));
    }
    require_hit_finite(words, row)
}
fn require_hit_finite(words: &[u32], row: u32) -> Result<(), RenderError> {
    let floats = [
        0, 1, 2, 4, 5, 6, 8, 9, 10, 11, 16, 17, 18, 19, 20, 24, 25, 26,
    ];
    if floats
        .iter()
        .any(|index| !f32::from_bits(words[*index]).is_finite())
        || words[27..] != [0; 5]
    {
        return Err(rejected(
            format!("row={row}"),
            "finite observed vectors/hit and zero reserved words",
        ));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn valid_hit_fixture() -> Vec<u8> {
        let mut bytes = vec![0; HIT_BYTES];
        for row in 0..409 {
            for (word, value) in [(3, row + 1), (22, row), (23, 1)] {
                let offset = row * 128 + word * 4;
                bytes[offset..][..4].copy_from_slice(&(value as u32).to_ne_bytes());
            }
        }
        bytes
    }
    #[test]
    fn records_refuse_size_mode_visit_row_nonfinite_and_reserved_changes() {
        let bytes = valid_hit_fixture();
        assert!(require_hit_records(&bytes, 0).is_ok());
        assert!(require_hit_records(&bytes[..HIT_BYTES - 1], 0).is_err());
        assert!(require_hit_records(&bytes, 2).is_err());
        for (word, value) in [(3, 0), (21, 1), (22, 1), (23, 2), (4, 0x7f800000), (31, 1)] {
            let mut changed = bytes.clone();
            changed[word * 4..][..4].copy_from_slice(&(value as u32).to_ne_bytes());
            assert!(require_hit_records(&changed, 0).is_err());
        }
    }
    #[test]
    fn storage_modes_change_only_one_word_and_refuse_mutated_headers() {
        let zero = initial_hit_storage(0).unwrap();
        let one = initial_hit_storage(1).unwrap();
        assert_eq!(&zero[4..], &one[4..]);
        assert_ne!(&zero[..4], &one[..4]);
        assert_eq!(require_hit_storage(&one, 1).unwrap().len(), HIT_BYTES);
        assert!(initial_hit_storage(2).is_err());
        assert!(require_hit_storage(&one, 0).is_err());
        assert!(require_hit_storage(&one[..HIT_STORAGE_BYTES - 1], 1).is_err());
        let mut changed = one;
        changed[4] = 1;
        assert!(require_hit_storage(&changed, 1).is_err());
    }
}
