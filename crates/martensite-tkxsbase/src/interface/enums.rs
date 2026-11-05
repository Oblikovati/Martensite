//! The enumerations of the Interface package.

/// `InterfaceCheckStatus`: the state of a check, or a criterion on checks;
/// `InterfaceCheckStatus::CheckFail as i32 == 2`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InterfaceCheckStatus {
    /// `InterfaceCheckOK`: no fail, no warning.
    #[default]
    CheckOK = 0,
    /// `InterfaceCheckWarning`: warnings, no fail.
    CheckWarning = 1,
    /// `InterfaceCheckFail`: fails.
    CheckFail = 2,
    /// `InterfaceCheckAny`: any status.
    CheckAny = 3,
    /// `InterfaceCheckMessage`: warnings or fails.
    CheckMessage = 4,
    /// `InterfaceCheckNoFail`: OK or warnings.
    CheckNoFail = 5,
}

/// `InterfaceDataState`: the state of an entity in a model;
/// `InterfaceDataState::StateUnknown as i32 == 6`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InterfaceDataState {
    /// `InterfaceStateOK`.
    #[default]
    StateOK = 0,
    /// `InterfaceLoadWarning`.
    LoadWarning = 1,
    /// `InterfaceLoadFail`.
    LoadFail = 2,
    /// `InterfaceDataWarning`.
    DataWarning = 3,
    /// `InterfaceDataFail`.
    DataFail = 4,
    /// `InterfaceStateUnloaded`.
    StateUnloaded = 5,
    /// `InterfaceStateUnknown`.
    StateUnknown = 6,
}

/// `InterfaceParamType`: the type of a parameter, with the values of
/// [`MoniToolValueType`](crate::moni_tool::MoniToolValueType);
/// `InterfaceParamType::ParamText as i32 == 5`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum InterfaceParamType {
    /// `InterfaceParamMisc`.
    #[default]
    ParamMisc = 0,
    /// `InterfaceParamInteger`.
    ParamInteger = 1,
    /// `InterfaceParamReal`.
    ParamReal = 2,
    /// `InterfaceParamIdent`.
    ParamIdent = 3,
    /// `InterfaceParamVoid`.
    ParamVoid = 4,
    /// `InterfaceParamText`.
    ParamText = 5,
    /// `InterfaceParamEnum`.
    ParamEnum = 6,
    /// `InterfaceParamLogical`.
    ParamLogical = 7,
    /// `InterfaceParamSub`.
    ParamSub = 8,
    /// `InterfaceParamHexa`.
    ParamHexa = 9,
    /// `InterfaceParamBinary`.
    ParamBinary = 10,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants_follow_the_native_enums() {
        assert_eq!(InterfaceCheckStatus::CheckNoFail as i32, 5);
        assert_eq!(InterfaceDataState::DataFail as i32, 4);
        assert_eq!(InterfaceParamType::ParamBinary as i32, 10);
    }
}
