//! A hasher for maps keyed by the process's own allocation addresses (#594). SipHash protects
//! maps whose keys an attacker can choose; these keys are pointers and flags, and SipHash took
//! 5% of a frame of 1,000 draws. The mixing step is the one rustc's FxHasher uses.
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

/// `AddressMap::<GeometryKey, GeometryLease>::default()`.
pub(crate) type AddressMap<K, V> = HashMap<K, V, BuildHasherDefault<AddressHasher>>;

/// Multiply-rotate hashing of whole words; `AddressHasher::default()`.
#[derive(Clone, Copy, Default)]
pub(crate) struct AddressHasher(u64);

impl AddressHasher {
    const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;
    fn add(&mut self, word: u64) {
        self.0 = (self.0.rotate_left(5) ^ word).wrapping_mul(Self::SEED);
    }
}

impl Hasher for AddressHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut word = [0u8; 8];
            word[..chunk.len()].copy_from_slice(chunk);
            self.add(u64::from_le_bytes(word));
        }
    }
    fn write_u8(&mut self, value: u8) {
        self.add(u64::from(value));
    }
    fn write_u32(&mut self, value: u32) {
        self.add(u64::from(value));
    }
    fn write_u64(&mut self, value: u64) {
        self.add(value);
    }
    fn write_usize(&mut self, value: usize) {
        self.add(value as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::{AddressHasher, AddressMap};
    use std::hash::{BuildHasher, BuildHasherDefault};

    #[test]
    fn equal_keys_hash_equal_and_nearby_addresses_differ() {
        let build = BuildHasherDefault::<AddressHasher>::default();
        let key = [Some(0x7f00_1000_usize), None, Some(0x7f00_2000)];
        assert_eq!(build.hash_one(key), build.hash_one(key));
        assert_ne!(
            build.hash_one(0x7f00_1000_usize),
            build.hash_one(0x7f00_1040_usize)
        );
    }

    #[test]
    fn a_map_finds_every_inserted_address() {
        let mut map = AddressMap::default();
        for address in (0..1000_usize).map(|i| 0x5000_0000 + i * 64) {
            map.insert(address, address / 64);
        }
        assert!((0..1000_usize).all(|i| map[&(0x5000_0000 + i * 64)] == 0x5000_0000 / 64 + i));
    }
}
