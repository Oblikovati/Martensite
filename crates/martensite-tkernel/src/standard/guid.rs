//! GUIDs and their UUID fields.
//!
//! The value is kept as its UUID fields; `Data4[0..2]` is hashed as
//! one `char16_t`, which matters only for the byte image hashed by
//! [`Guid::hash_code`]. The text forms live in `guid/text.rs`.

mod text;

use super::hashing::hash_bytes;

/// Number of ASCII characters in a GUID representation.
pub const GUID_SIZE: usize = 36;
/// Number of ASCII/UTF-16 code units including the NUL terminator.
pub const GUID_SIZE_ALLOC: usize = GUID_SIZE + 1;

/// Field layout of a UUID (also usable in const contexts).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Uuid {
    /// First 32 bits.
    pub data1: u32,
    /// Next 16 bits.
    pub data2: u16,
    /// Next 16 bits.
    pub data3: u16,
    /// Last eight bytes in displayed order.
    pub data4: [u8; 8],
}

/// A GUID with a strict 8-4-4-4-12 hexadecimal text format.
///
/// Equality is `PartialEq`, copies are
/// `Copy`, and the hash is [`Guid::hash_code`] (also fed to `Hash`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Guid(Uuid);

impl Guid {
    /// The all-zero GUID.
    ///
    /// ```
    /// assert_eq!(martensite_tkernel::standard::Guid::new().to_string(), "00000000-0000-0000-0000-000000000000");
    /// ```
    pub const fn new() -> Self {
        Self(Uuid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        })
    }

    /// The ten components;
    /// the first field keeps its bit pattern and `d` supplies `Data4[0..2]` high byte first.
    ///
    /// ```
    /// use martensite_tkernel::standard::Guid;
    /// let guid = Guid::from_components(-1, 2, 3, 0x0405, 6, 7, 8, 9, 10, 11);
    /// assert_eq!(guid.to_string(), "ffffffff-0002-0003-0405-060708090a0b");
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub const fn from_components(
        a: i32,
        b: u16,
        c: u16,
        d: u16,
        e: u8,
        f: u8,
        g: u8,
        h: u8,
        i: u8,
        j: u8,
    ) -> Self {
        Self(Uuid {
            data1: a as u32,
            data2: b,
            data3: c,
            data4: [(d >> 8) as u8, d as u8, e, f, g, h, i, j],
        })
    }

    /// A GUID from UUID fields.
    ///
    /// ```
    /// use martensite_tkernel::standard::{Guid, Uuid};
    /// assert_eq!(Guid::from_uuid(Uuid::default()), Guid::new());
    /// ```
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// The UUID fields.
    ///
    /// ```
    /// use martensite_tkernel::standard::Guid;
    /// assert_eq!(Guid::new().to_uuid().data1, 0);
    /// ```
    pub const fn to_uuid(self) -> Uuid {
        self.0
    }

    /// Equality, also usable in const contexts, where `==` is unavailable.
    ///
    /// ```
    /// use martensite_tkernel::standard::Guid;
    /// assert!(Guid::new().is_same(Guid::new()));
    /// ```
    pub const fn is_same(self, other: Self) -> bool {
        if self.0.data1 != other.0.data1
            || self.0.data2 != other.0.data2
            || self.0.data3 != other.0.data3
        {
            return false;
        }
        let mut i = 0;
        while i < 8 {
            if self.0.data4[i] != other.0.data4[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Inequality, also usable in const contexts.
    ///
    /// ```
    /// use martensite_tkernel::standard::Guid;
    /// assert!(!Guid::new().is_not_same(Guid::new()));
    /// ```
    pub const fn is_not_same(self, other: Self) -> bool {
        !self.is_same(other)
    }

    /// Copies another GUID.
    ///
    /// ```
    /// use martensite_tkernel::standard::Guid;
    /// let mut guid = Guid::new();
    /// guid.assign(Guid::from_components(1, 0, 0, 0, 0, 0, 0, 0, 0, 0));
    /// assert_eq!(guid.to_uuid().data1, 1);
    /// ```
    pub const fn assign(&mut self, other: Self) {
        *self = other;
    }

    /// Assigns a GUID and returns this value for chained assignment-style calls;
    /// `guid.assign_returning(other).assign_returning(Guid::new())`.
    pub const fn assign_returning(&mut self, other: Self) -> &mut Self {
        self.assign(other);
        self
    }

    /// Copies UUID fields.
    ///
    /// ```
    /// use martensite_tkernel::standard::{Guid, Uuid};
    /// let mut guid = Guid::from_components(1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    /// guid.assign_uuid(Uuid::default());
    /// assert_eq!(guid, Guid::new());
    /// ```
    pub const fn assign_uuid(&mut self, uuid: Uuid) {
        self.0 = uuid;
    }

    /// Assigns UUID fields and returns this GUID;
    /// `guid.assign_uuid_returning(uuid).assign_returning(other)`.
    pub const fn assign_uuid_returning(&mut self, uuid: Uuid) -> &mut Self {
        self.assign_uuid(uuid);
        self
    }

    /// The 16-byte image of the fields in declaration order, host byte order.
    fn field_bytes(self) -> [u8; 16] {
        let uuid: Uuid = self.0;
        let packed: u16 = u16::from_be_bytes([uuid.data4[0], uuid.data4[1]]);
        let mut bytes: [u8; 16] = [0; 16];
        bytes[0..4].copy_from_slice(&uuid.data1.to_ne_bytes());
        bytes[4..6].copy_from_slice(&uuid.data2.to_ne_bytes());
        bytes[6..8].copy_from_slice(&uuid.data3.to_ne_bytes());
        bytes[8..10].copy_from_slice(&packed.to_ne_bytes());
        bytes[10..16].copy_from_slice(&uuid.data4[2..8]);
        bytes
    }

    /// The byte hash over the 16-byte field image
    /// (`int`, three `char16_t`, six `uint8_t`; no padding).
    ///
    /// ```
    /// use martensite_tkernel::standard::Guid;
    /// assert_eq!(Guid::new().hash_code(), martensite_tkernel::standard::hash_bytes(&[0; 16]));
    /// ```
    pub fn hash_code(self) -> usize {
        hash_bytes(&self.field_bytes())
    }
}

impl std::hash::Hash for Guid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.hash_code());
    }
}

impl From<Uuid> for Guid {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl From<Guid> for Uuid {
    fn from(guid: Guid) -> Self {
        guid.to_uuid()
    }
}
