//! The type and form numbers that identify an entity's kind in a file.

/// A couple of an IGES type number and a form number.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataIgesType;
/// let line = IgesDataIgesType::new(110, 0);
/// assert_eq!((line.type_number(), line.form()), (110, 0));
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct IgesDataIgesType {
    thetype: i32,
    theform: i32,
}

impl IgesDataIgesType {
    /// A couple of a type number and a form number.
    pub fn new(atype: i32, aform: i32) -> Self {
        Self {
            thetype: atype,
            theform: aform,
        }
    }
    /// The type number.
    pub fn type_number(&self) -> i32 {
        self.thetype
    }
    /// The form number.
    pub fn form(&self) -> i32 {
        self.theform
    }
    /// Whether both numbers are the same as in `another`.
    pub fn is_equal(&self, another: &Self) -> bool {
        self == another
    }
    /// Resets both numbers to zero.
    pub fn nullify(&mut self) {
        *self = Self::default();
    }
}
