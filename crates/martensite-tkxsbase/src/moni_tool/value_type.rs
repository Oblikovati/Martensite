//! `MoniToolValueType`: the kinds of value a typed value or an attribute holds.

/// `MoniToolValueType`, with the native discriminants;
/// `MoniToolValueType::ValueInteger as i32 == 1`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MoniToolValueType {
    /// `MoniToolValueMisc`.
    ValueMisc = 0,
    /// `MoniToolValueInteger`.
    ValueInteger = 1,
    /// `MoniToolValueReal`.
    ValueReal = 2,
    /// `MoniToolValueIdent`: any object.
    ValueIdent = 3,
    /// `MoniToolValueVoid`: not recorded.
    ValueVoid = 4,
    /// `MoniToolValueText`: free text, the default of a typed value.
    #[default]
    ValueText = 5,
    /// `MoniToolValueEnum`.
    ValueEnum = 6,
    /// `MoniToolValueLogical`.
    ValueLogical = 7,
    /// `MoniToolValueSub`.
    ValueSub = 8,
    /// `MoniToolValueHexa`.
    ValueHexa = 9,
    /// `MoniToolValueBinary`.
    ValueBinary = 10,
}

#[cfg(test)]
mod tests {
    use super::MoniToolValueType;

    #[test]
    fn discriminants_follow_the_native_enum() {
        assert_eq!(MoniToolValueType::ValueMisc as i32, 0);
        assert_eq!(MoniToolValueType::ValueText as i32, 5);
        assert_eq!(MoniToolValueType::ValueBinary as i32, 10);
        assert_eq!(MoniToolValueType::default(), MoniToolValueType::ValueText);
    }
}
