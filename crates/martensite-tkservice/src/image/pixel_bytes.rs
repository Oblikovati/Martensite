//! The pixel buffer of `Image_PixMapData`: storage it allocates is 16-byte aligned, as the
//! `NCollection_AlignedAllocator(16)` of `Image_PixMap::DefaultAllocator()`, and a wrapped
//! caller buffer keeps the caller's alignment, as natively.
// The aligned storage is a Vec of 16-byte blocks viewed as bytes; Miri runs the tests.
#![allow(unsafe_code)]

/// The allocation unit of the owned storage: 16 bytes at a 16-byte aligned address.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub(super) struct AlignedBlock([u8; 16]);

/// The bytes of a pixel buffer; `PixelBytes::zeroed(64).as_slice().len() == 64`.
#[derive(Clone)]
pub(super) enum PixelBytes {
    /// Storage the buffer allocated: `len` bytes at the start of 16-byte aligned blocks.
    Aligned {
        blocks: Vec<AlignedBlock>,
        len: usize,
    },
    /// A caller's buffer adopted by `InitWrapper`, given back by `into_vec`.
    Adopted(Vec<u8>),
}

impl PixelBytes {
    /// `len` zero bytes at a 16-byte aligned address (the native allocation is uninitialised).
    pub(super) fn zeroed(len: usize) -> Self {
        let blocks = vec![AlignedBlock([0; 16]); len.div_ceil(16)];
        Self::Aligned { blocks, len }
    }
    /// The bytes; `bytes.as_slice()[0]`.
    pub(super) fn as_slice(&self) -> &[u8] {
        match self {
            // SAFETY: the blocks hold blocks.len() * 16 >= len initialized bytes, contiguous
            // because AlignedBlock is repr(C) of a byte array whose size is its alignment.
            Self::Aligned { blocks, len } => unsafe {
                std::slice::from_raw_parts(blocks.as_ptr().cast::<u8>(), *len)
            },
            Self::Adopted(bytes) => bytes,
        }
    }
    /// The bytes, mutable; `bytes.as_mut_slice()[0] = 1`.
    pub(super) fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            // SAFETY: as in as_slice; the exclusive borrow of the blocks makes the slice unique.
            Self::Aligned { blocks, len } => unsafe {
                std::slice::from_raw_parts_mut(blocks.as_mut_ptr().cast::<u8>(), *len)
            },
            Self::Adopted(bytes) => bytes,
        }
    }
    /// The bytes as a vector: the adopted one itself, or a copy of the owned storage.
    pub(super) fn into_vec(self) -> Vec<u8> {
        match self {
            Self::Aligned { .. } => self.as_slice().to_vec(),
            Self::Adopted(bytes) => bytes,
        }
    }
}

impl PartialEq for PixelBytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl std::fmt::Debug for PixelBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_storage_is_zeroed_and_sixteen_byte_aligned() {
        for len in [1, 15, 16, 17, 100] {
            let mut bytes = PixelBytes::zeroed(len);
            assert_eq!(bytes.as_slice().len(), len);
            assert_eq!(bytes.as_slice().as_ptr().align_offset(16), 0);
            assert!(bytes.as_slice().iter().all(|&byte| byte == 0));
            bytes.as_mut_slice()[len - 1] = 9;
            let copy = bytes.clone();
            assert_eq!(copy, bytes);
            assert_eq!(bytes.into_vec()[len - 1], 9);
        }
    }

    #[test]
    fn adopted_bytes_are_given_back_unchanged() {
        let mut bytes = PixelBytes::Adopted(vec![1, 2, 3]);
        bytes.as_mut_slice()[0] = 7;
        assert_eq!(bytes.as_slice(), &[7, 2, 3]);
        assert_eq!(bytes.into_vec(), vec![7, 2, 3]);
    }
}
