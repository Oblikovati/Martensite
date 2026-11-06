//! One directory entry as it stands in a file.
use super::IgesDataIgesType;

/// The fixed text fields of a directory entry, eight characters then a terminator.
pub type IgesDataDirText = [u8; 9];

/// Every field of one directory entry; the same shape in and out of [`IgesDataDirPart`].
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataDirValues;
/// assert_eq!(IgesDataDirValues::default().values[16], 0);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IgesDataDirValues {
    /// The seventeen integer fields, in the order of the file.
    pub values: [i32; 17],
    /// The first reserved text.
    pub res1: IgesDataDirText,
    /// The second reserved text.
    pub res2: IgesDataDirText,
    /// The entity label.
    pub label: IgesDataDirText,
    /// The entity subscript.
    pub subscript: IgesDataDirText,
}

/// One directory entry read from, or to be written to, a file.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataDirPart, IgesDataDirValues};
/// let mut values = IgesDataDirValues::default();
/// values.values[0] = 110;
/// let mut part = IgesDataDirPart::default();
/// part.init(&values);
/// assert_eq!(part.dir_type().type_number(), 110);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct IgesDataDirPart {
    fields: IgesDataDirValues,
}

impl IgesDataDirPart {
    /// Records all the fields; each text keeps eight characters and its terminator.
    pub fn init(&mut self, values: &IgesDataDirValues) {
        self.fields = *values;
        for text in [
            &mut self.fields.res1,
            &mut self.fields.res2,
            &mut self.fields.label,
            &mut self.fields.subscript,
        ] {
            text[8] = 0;
        }
    }
    /// All the fields.
    pub fn values(&self) -> IgesDataDirValues {
        self.fields
    }
    /// The type and form numbers.
    pub fn dir_type(&self) -> IgesDataIgesType {
        IgesDataIgesType::new(self.fields.values[0], self.fields.values[16])
    }
}
