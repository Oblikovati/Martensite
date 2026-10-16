//! MurmurHash64A and MurmurHash2A with a fixed byte loading.
//!
//! Whole words are read with `memcpy` (host byte order); the 1..7 trailing bytes are
//! assembled little-endian from unsigned bytes on every platform.
//! The length is the slice length; lengths beyond `INT_MAX` are
//! rejected.

pub mod murmur_hash_utils;

use super::{HashWord, object_bytes};
use bytemuck::NoUninit;
use murmur_hash_utils::{little_endian_word, shift_mix};

const MUL_64: u64 = 0xc6a4_a793_5bd1_e995;
const MUL_32: u32 = 0x5bd1_e995;

/// The 64-bit Murmur2 variant.
///
/// ```
/// use martensite_tkernel::standard::hashing::murmur_hash::murmur_hash_64a;
/// assert_eq!(murmur_hash_64a(&[], 0), 0);
/// ```
pub fn murmur_hash_64a(key: &[u8], seed: u64) -> u64 {
    let mut hash: u64 = seed ^ (key.len() as u64).wrapping_mul(MUL_64);
    let (words, tail) = key.as_chunks::<8>();
    for word in words {
        let mixed: u64 = shift_mix(u64::from_ne_bytes(*word).wrapping_mul(MUL_64));
        hash = (hash ^ mixed.wrapping_mul(MUL_64)).wrapping_mul(MUL_64);
    }
    if !tail.is_empty() {
        hash = (hash ^ little_endian_word(tail)).wrapping_mul(MUL_64);
    }
    shift_mix(shift_mix(hash).wrapping_mul(MUL_64))
}

/// The 32-bit Murmur2 variant.
///
/// ```
/// use martensite_tkernel::standard::hashing::murmur_hash::murmur_hash_2a;
/// assert_eq!(murmur_hash_2a(&[], 0), 0);
/// ```
pub fn murmur_hash_2a(key: &[u8], seed: u32) -> u32 {
    let mut hash: u32 = seed ^ key.len() as u32;
    let (words, tail) = key.as_chunks::<4>();
    for word in words {
        let mut mixed: u32 = u32::from_ne_bytes(*word).wrapping_mul(MUL_32);
        mixed ^= mixed >> 24;
        hash = hash.wrapping_mul(MUL_32) ^ mixed.wrapping_mul(MUL_32);
    }
    if !tail.is_empty() {
        // The fall-through switch XORs byte i shifted by 8*i, i.e. a little-endian load.
        let loaded: u32 = tail
            .iter()
            .rev()
            .fold(0, |word, &byte| (word << 8) | u32::from(byte));
        hash = (hash ^ loaded).wrapping_mul(MUL_32);
    }
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(MUL_32);
    hash ^ (hash >> 15)
}

/// Murmur hash combining; an 8-byte `W`
/// runs [`murmur_hash_64a`], any other width [`murmur_hash_2a`]. The defaults are
/// `length = size_of::<T1>()` and `seed = optimal_seed::<T>()`.
///
/// # Panics
/// When `length` exceeds the byte size of `values`.
///
/// ```
/// use martensite_tkernel::standard::hashing::murmur_hash::{hash_combine, optimal_seed};
/// let code: u32 = hash_combine(&[1_u8, 2, 3], 3, optimal_seed());
/// assert_ne!(code, 0);
/// ```
pub fn hash_combine<W: HashWord, T: NoUninit>(values: &[T], length: usize, seed: W) -> W {
    W::murmur(object_bytes(values, length), seed)
}

/// `MurmurHash::optimalSeed<T>()`: `0xA329F1D3A586` for 8-byte words, else `0xc70f6907`.
///
/// ```
/// use martensite_tkernel::standard::hashing::murmur_hash::optimal_seed;
/// assert_eq!(optimal_seed::<u32>(), 0xc70f_6907);
/// ```
pub fn optimal_seed<W: HashWord>() -> W {
    W::MURMUR_SEED
}
