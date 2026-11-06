//! `StepDataField::FieldKind`: an encoded byte, because the values combine a type
//! with the arity bits (`List`, `List2`) and hold values no enumerator names.

/// `StepDataField::FieldKind`.
///
/// ```
/// use martensite_tkdestep::step_data::StepDataFieldKind;
/// let kind = StepDataFieldKind::REAL.combine(StepDataFieldKind::LIST);
/// assert_eq!(kind.type_of(), StepDataFieldKind::REAL);
/// assert_eq!(kind.arity_bits(), StepDataFieldKind::LIST.to_int());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StepDataFieldKind(u8);

impl StepDataFieldKind {
    /// `undefined`.
    pub const UNDEFINED: Self = Self(0);
    /// `integer`.
    pub const INTEGER: Self = Self(1);
    /// `boolean`.
    pub const BOOLEAN: Self = Self(2);
    /// `logical`.
    pub const LOGICAL: Self = Self(3);
    /// `Enum`.
    pub const ENUM: Self = Self(4);
    /// `real`.
    pub const REAL: Self = Self(5);
    /// `string`.
    pub const STRING: Self = Self(6);
    /// `entity`.
    pub const ENTITY: Self = Self(7);
    /// `any`.
    pub const ANY: Self = Self(8);
    /// `Derived`.
    pub const DERIVED: Self = Self(9);
    /// `select`.
    pub const SELECT: Self = Self(16);
    /// `List`.
    pub const LIST: Self = Self(64);
    /// `List2`.
    pub const LIST2: Self = Self(128);

    const TYPE_MASK: i32 = 0x0F;
    const ARITY_MASK: i32 = 0xC0;
    const ARITY_SHIFT: i32 = 6;

    /// `KindTools::to_int`.
    pub const fn to_int(self) -> i32 {
        self.0 as i32
    }
    /// `KindTools::from_int`: `undefined` out of a byte.
    pub const fn from_int(kind: i32) -> Self {
        if kind >= 0 && kind <= 0xFF {
            Self(kind as u8)
        } else {
            Self::UNDEFINED
        }
    }
    /// `KindTools::combine`.
    pub const fn combine(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    /// `KindTools::type_of`.
    pub const fn type_of(self) -> Self {
        Self::from_int(self.to_int() & Self::TYPE_MASK)
    }
    /// The arity bits (`KindTools::bits(kind, THE_ARITY_MASK)`).
    pub const fn arity_bits(self) -> i32 {
        self.to_int() & Self::ARITY_MASK
    }
    /// The arity: 0, 1 or 2 (3 when both bits are set).
    pub const fn arity(self) -> i32 {
        self.arity_bits() >> Self::ARITY_SHIFT
    }
}
