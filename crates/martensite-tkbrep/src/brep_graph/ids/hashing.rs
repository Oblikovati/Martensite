//! The `hash` and `hashBytes` combinations shared by the id hashes.
//!
//! Every id hash of BRepGraph fills a `size_t aCombination[n]` with
//! `hash(field)` and returns `hashBytes(aCombination,
//! sizeof(aCombination))`; [`combine`] is that last step over the same bytes.
use martensite_tkernel::standard::hash_bytes;
use martensite_tkernel::standard::hashing::hash;

/// `hash(int)`; `hash_i32(3)`.
pub(crate) fn hash_i32(value: i32) -> usize {
    hash::<usize, i32>(&value)
}

/// `hash(uint32_t)`; `hash_u32(3)`.
pub(crate) fn hash_u32(value: u32) -> usize {
    hash::<usize, u32>(&value)
}

/// `hash(size_t)`; `hash_usize(3)`.
pub(crate) fn hash_usize(value: usize) -> usize {
    hash::<usize, usize>(&value)
}

/// `hashBytes(aCombination, sizeof(aCombination))` over a `size_t` array.
pub(crate) fn combine(words: &[usize]) -> usize {
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_ne_bytes()).collect();
    hash_bytes(&bytes)
}

/// `hash(T)` of a two-field `{int Kind; uint32_t Index;}` id struct: the
/// Murmur hash of its eight object bytes (no padding), as `BRepGraph_UsagePath` hashes
/// a `BRepGraph_NodeId` or a `BRepGraph_RefId`.
pub(crate) fn hash_kind_index(kind: i32, index: u32) -> usize {
    let words: [u32; 2] = [kind as u32, index];
    hash::<usize, [u32; 2]>(&words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_hashes_the_native_bytes() {
        let words = [hash_i32(1), hash_u32(2)];
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&words[0].to_ne_bytes());
        bytes.extend_from_slice(&words[1].to_ne_bytes());
        assert_eq!(combine(&words), hash_bytes(&bytes));
    }

    #[test]
    fn kind_index_hash_matches_the_struct_bytes() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2_i32.to_ne_bytes());
        bytes.extend_from_slice(&7_u32.to_ne_bytes());
        assert_eq!(hash_kind_index(2, 7), hash_bytes(&bytes));
        assert_eq!(hash_usize(5), hash::<usize, usize>(&5));
    }
}
