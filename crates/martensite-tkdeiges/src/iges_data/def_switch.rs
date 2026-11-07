//! The value of a directory field that is either void, a rank or a reference.
use super::IgesDataDefType;

/// A field given by its sign: 0 is void, a negative value a reference, a positive one a rank.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataDefSwitch, IgesDataDefType};
/// let mut field = IgesDataDefSwitch::default();
/// field.set_rank(3);
/// assert_eq!((field.def_type(), field.value()), (IgesDataDefType::DefValue, 3));
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IgesDataDefSwitch {
    theval: i32,
}

impl IgesDataDefSwitch {
    /// Makes the field void.
    pub fn set_void(&mut self) {
        self.theval = 0;
    }
    /// Makes the field a reference.
    pub fn set_reference(&mut self) {
        self.theval = -1;
    }
    /// Makes the field hold `val`, a rank when positive, a reference when negative.
    pub fn set_rank(&mut self, val: i32) {
        self.theval = val;
    }
    /// The kind of definition of the field.
    pub fn def_type(&self) -> IgesDataDefType {
        match self.theval {
            value if value < 0 => IgesDataDefType::DefReference,
            value if value > 0 => IgesDataDefType::DefValue,
            _ => IgesDataDefType::DefVoid,
        }
    }
    /// The stored value.
    pub fn value(&self) -> i32 {
        self.theval
    }
}
