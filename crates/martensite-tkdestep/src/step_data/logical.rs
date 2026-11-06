//! `StepDataLogical`: the STEP logical value.

/// `StepDataLogical`; the discriminants are the integer values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum StepDataLogical {
    /// L false.
    #[default]
    LFalse = 0,
    /// L true.
    LTrue = 1,
    /// L unknown.
    LUnknown = 2,
}

impl StepDataLogical {
    /// The reading of an integer: 0 false, 1 true, anything else unknown;
    /// `StepDataLogical::from_int(7) == StepDataLogical::LUnknown`.
    pub fn from_int(value: i32) -> Self {
        match value {
            0 => Self::LFalse,
            1 => Self::LTrue,
            _ => Self::LUnknown,
        }
    }
}
