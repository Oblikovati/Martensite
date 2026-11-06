//! The enumerations of IFSelect.

/// `IFSelectReturnStatus`: the status of a command or an action.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IFSelectReturnStatus {
    /// `IFSelectRetVoid`: nothing done.
    #[default]
    RetVoid = 0,
    /// `IFSelectRetDone`.
    RetDone = 1,
    /// `IFSelectRetError`: an error in the data.
    RetError = 2,
    /// `IFSelectRetFail`: an execution failure.
    RetFail = 3,
    /// `IFSelectRetStop`: stop the session.
    RetStop = 4,
}

/// `IFSelectRemainMode`: what to do with the entities not yet sent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IFSelectRemainMode {
    /// `IFSelectRemainForget`.
    #[default]
    RemainForget = 0,
    /// `IFSelectRemainCompute`.
    RemainCompute = 1,
    /// `IFSelectRemainDisplay`.
    RemainDisplay = 2,
    /// `IFSelectRemainUndo`.
    RemainUndo = 3,
}

/// `IFSelectPrintCount`: the modes of printing a count.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IFSelectPrintCount {
    /// `IFSelectItemsByEntity`.
    #[default]
    ItemsByEntity = 0,
    /// `IFSelectCountByItem`.
    CountByItem = 1,
    /// `IFSelectShortByItem`.
    ShortByItem = 2,
    /// `IFSelectListByItem`.
    ListByItem = 3,
    /// `IFSelectEntitiesByItem`.
    EntitiesByItem = 4,
    /// `IFSelectCountSummary`.
    CountSummary = 5,
    /// `IFSelectGeneralInfo`.
    GeneralInfo = 6,
    /// `IFSelectMapping`.
    Mapping = 7,
    /// `IFSelectResultCount`.
    ResultCount = 8,
}

/// `IFSelectPrintFail`: print the fails only, or the warnings too.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IFSelectPrintFail {
    /// `IFSelectFailOnly`.
    #[default]
    FailOnly = 0,
    /// `IFSelectFailAndWarn`.
    FailAndWarn = 1,
}

/// `IFSelectEditValue`: the edit mode of a value of an editor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum IFSelectEditValue {
    /// `IFSelectOptional`.
    #[default]
    Optional = 0,
    /// `IFSelectEditable`.
    Editable = 1,
    /// `IFSelectEditProtected`.
    EditProtected = 2,
    /// `IFSelectEditComputed`.
    EditComputed = 3,
    /// `IFSelectEditRead`.
    EditRead = 4,
    /// `IFSelectEditDynamic`.
    EditDynamic = 5,
}

impl IFSelectPrintCount {
    /// The mode of a native integer value (`None` out of range).
    pub fn from_i32(value: i32) -> Option<Self> {
        const ALL: [IFSelectPrintCount; 9] = [
            IFSelectPrintCount::ItemsByEntity,
            IFSelectPrintCount::CountByItem,
            IFSelectPrintCount::ShortByItem,
            IFSelectPrintCount::ListByItem,
            IFSelectPrintCount::EntitiesByItem,
            IFSelectPrintCount::CountSummary,
            IFSelectPrintCount::GeneralInfo,
            IFSelectPrintCount::Mapping,
            IFSelectPrintCount::ResultCount,
        ];
        usize::try_from(value)
            .ok()
            .and_then(|i| ALL.get(i))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants() {
        assert_eq!(IFSelectReturnStatus::RetStop as i32, 4);
        assert_eq!(IFSelectRemainMode::RemainUndo as i32, 3);
        assert_eq!(
            IFSelectPrintCount::from_i32(8),
            Some(IFSelectPrintCount::ResultCount)
        );
        assert_eq!(IFSelectPrintCount::from_i32(9), None);
        assert_eq!(IFSelectPrintFail::FailAndWarn as i32, 1);
        assert_eq!(IFSelectEditValue::EditDynamic as i32, 5);
    }

    /// Every enumerator keeps its native IFSelect_* order and value.
    #[test]
    fn every_enumerator_keeps_native_value() {
        use IFSelectEditValue as E;
        let edit = [
            E::Optional,
            E::Editable,
            E::EditProtected,
            E::EditComputed,
            E::EditRead,
            E::EditDynamic,
        ];
        assert!(edit.iter().enumerate().all(|(i, v)| *v as i32 == i as i32));
        use IFSelectReturnStatus as R;
        let ret = [R::RetVoid, R::RetDone, R::RetError, R::RetFail, R::RetStop];
        assert!(ret.iter().enumerate().all(|(i, v)| *v as i32 == i as i32));
        use IFSelectRemainMode as M;
        let remain = [
            M::RemainForget,
            M::RemainCompute,
            M::RemainDisplay,
            M::RemainUndo,
        ];
        assert!(
            remain
                .iter()
                .enumerate()
                .all(|(i, v)| *v as i32 == i as i32)
        );
        let fail = [IFSelectPrintFail::FailOnly, IFSelectPrintFail::FailAndWarn];
        assert!(fail.iter().enumerate().all(|(i, v)| *v as i32 == i as i32));
        for value in 0..9 {
            let mode = IFSelectPrintCount::from_i32(value).expect("native PrintCount value");
            assert_eq!(mode as i32, value);
        }
    }
}
