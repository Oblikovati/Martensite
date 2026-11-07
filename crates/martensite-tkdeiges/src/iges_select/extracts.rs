//! The selections that keep the entities of their input by a criterion of the directory entry.
use crate::iges_data::{IgesDataIgesEntity, IgesDataLevelListEntity};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::{AsciiString, HAsciiString},
};
use martensite_tkxsbase::{
    if_select::{
        IFSelectIntParam, IFSelectSelectDeduct, IFSelectSelectDeductBase, IFSelectSelectExtract,
        IFSelectSelectExtractBase, IFSelectSelection, IFSelectSelectionIterator,
        deduct_fill_iterator, extract_label, extract_root_result,
    },
    interface::{InterfaceEntityIterator, InterfaceGraph, InterfaceInterfaceModel},
};

/// Declares a selection of extraction: its fields, its runtime type and its general members.
macro_rules! iges_extract {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, { $($field:ident: $ty:ty),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $class {
            transient: StandardTransient,
            base: IFSelectSelectExtractBase,
            $($field: SharedState<$ty>,)*
        }
        impl_transient!(
            $class,
            $recorded,
            dyn IFSelectSelectExtract,
            transient,
            interfaces: [dyn IFSelectSelection, dyn IFSelectSelectDeduct, dyn IFSelectSelectExtract]
        );
        impl IFSelectSelection for $class {
            fn root_result(&self, g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
                extract_root_result(self, g)
            }
            fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
                deduct_fill_iterator(self, iter)
            }
            fn label(&self) -> AsciiString {
                extract_label(self)
            }
        }
        impl IFSelectSelectDeduct for $class {
            fn deduct_base(&self) -> &IFSelectSelectDeductBase {
                &self.base.deduct
            }
        }
    };
}

iges_extract! {
    /// Keeps the entities whose blank status is visible.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectVisibleStatus;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExtract;
    /// let select = IgesSelectSelectVisibleStatus::default();
    /// assert_eq!(select.extract_label().as_bytes(), b"IGES Entity, Status Visible");
    /// ```
    IgesSelectSelectVisibleStatus, "IgesSelectSelectVisibleStatus", {}
}

impl IFSelectSelectExtract for IgesSelectSelectVisibleStatus {
    fn extract_base(&self) -> &IFSelectSelectExtractBase {
        &self.base
    }
    fn sort(
        &self,
        _rank: i32,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        Ok(entity.get().is_some_and(|live| live.blank_status() == 0))
    }
    fn extract_label(&self) -> AsciiString {
        AsciiString::new("IGES Entity, Status Visible")
    }
}

iges_extract! {
    /// Keeps the entities of a subordinate status: 0 to 3, 4 physically (1 or 3), 5 logically
    /// (2 or 3), 6 any dependence.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectSubordinate;
    /// assert_eq!(IgesSelectSelectSubordinate::new(4).status(), 4);
    /// ```
    IgesSelectSelectSubordinate, "IgesSelectSelectSubordinate", { thestatus: i32 }
}

impl IgesSelectSelectSubordinate {
    /// The selection of a status.
    pub fn new(status: i32) -> Self {
        let select = Self::default();
        *select.thestatus.write() = status;
        select
    }
    /// The status.
    pub fn status(&self) -> i32 {
        *self.thestatus.read()
    }
}

impl IFSelectSelectExtract for IgesSelectSelectSubordinate {
    fn extract_base(&self) -> &IFSelectSelectExtractBase {
        &self.base
    }
    fn sort(
        &self,
        _rank: i32,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return Ok(false);
        };
        let sub = live.subordinate_status();
        let status = self.status();
        Ok(sub == status
            || (status == 4 && (sub == 1 || sub == 3))
            || (status == 5 && (sub == 2 || sub == 3))
            || (status == 6 && sub != 0))
    }
    fn extract_label(&self) -> AsciiString {
        let kind = match self.status() {
            0 => "Independant (0)",
            1 => "Physically only Dependant (1)",
            2 => "Logically only Dependant (2) ",
            3 => "Both Phys. and Log. Dependant (3)",
            4 => "Physically Dependant (1 or 3)",
            5 => "Logically Dependant (2 or 3)",
            6 => "Dependant in any way (1 or 2 or 3)",
            _ => "",
        };
        AsciiString::new(&format!("IGESEntity, Subordinate {kind}"))
    }
}

iges_extract! {
    /// Keeps the entities of a name; blanks at the end do not count.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectName;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExtract;
    /// let select = IgesSelectSelectName::default();
    /// assert_eq!(select.extract_label().as_bytes(), b"IGES Entity, Name : (undefined)");
    /// ```
    IgesSelectSelectName, "IgesSelectSelectName", { thename: Handle<HAsciiString> }
}

impl IgesSelectSelectName {
    /// Sets the name.
    pub fn set_name(&self, name: &Handle<HAsciiString>) {
        *self.thename.write() = name.clone();
    }
    /// The name.
    pub fn name(&self) -> Handle<HAsciiString> {
        self.thename.read().clone()
    }
}

/// Whether two names are the same, up to blanks at the end of the longer.
fn same_name(first: &[u8], second: &[u8]) -> bool {
    let common = first.len().min(second.len());
    if first[..common] != second[..common] {
        return false;
    }
    let longer = if first.len() > second.len() {
        first
    } else {
        second
    };
    longer[common..].iter().all(|byte| *byte == b' ')
}

impl IFSelectSelectExtract for IgesSelectSelectName {
    fn extract_base(&self) -> &IFSelectSelectExtractBase {
        &self.base
    }
    fn sort(
        &self,
        _rank: i32,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let wanted = self.name();
        let (Some(live), Some(text)) = (entity.get(), wanted.get()) else {
            return Ok(false);
        };
        if !live.has_name() {
            return Ok(false);
        }
        let name = live.name_value()?;
        let Some(own) = name.get() else {
            return Ok(false);
        };
        Ok(same_name(own.string().as_bytes(), text.string().as_bytes()))
    }
    fn extract_label(&self) -> AsciiString {
        let name = self.name();
        let Some(text) = name.get() else {
            return AsciiString::new("IGES Entity, Name : (undefined)");
        };
        let mut label = b"IGES Entity, Name : ".to_vec();
        label.extend_from_slice(text.string().as_bytes());
        AsciiString::from_bytes(label)
    }
}

iges_extract! {
    /// Keeps the entities of a level, or that have it in their list; for no level, those attached
    /// to none.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_select::IgesSelectSelectLevelNumber;
    /// use martensite_tkxsbase::if_select::IFSelectSelectExtract;
    /// let select = IgesSelectSelectLevelNumber::default();
    /// assert_eq!(select.extract_label().as_bytes(), b"IGES Entity attached to no Level");
    /// ```
    IgesSelectSelectLevelNumber, "IgesSelectSelectLevelNumber", { thelevnum: Handle<IFSelectIntParam> }
}

impl IgesSelectSelectLevelNumber {
    /// Sets the level.
    pub fn set_level_number(&self, levnum: &Handle<IFSelectIntParam>) {
        *self.thelevnum.write() = levnum.clone();
    }
    /// The level.
    pub fn level_number(&self) -> Handle<IFSelectIntParam> {
        self.thelevnum.read().clone()
    }
    fn level_value(&self) -> i32 {
        self.thelevnum
            .read()
            .get()
            .map_or(0, IFSelectIntParam::value)
    }
}

/// Whether a list has a level.
fn list_has(list: &dyn IgesDataLevelListEntity, numlev: i32) -> Result<bool, Failure> {
    list.has_level_number(numlev)
}

impl IFSelectSelectExtract for IgesSelectSelectLevelNumber {
    fn extract_base(&self) -> &IFSelectSelectExtractBase {
        &self.base
    }
    fn sort(
        &self,
        _rank: i32,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return Ok(false);
        };
        let numlev = self.level_value();
        let levels = live.level_list();
        let Some(list) = levels.get() else {
            return Ok(live.level() == numlev);
        };
        if numlev == 0 {
            return Ok(false);
        }
        list_has(list, numlev)
    }
    fn extract_label(&self) -> AsciiString {
        match self.level_value() {
            0 => AsciiString::new("IGES Entity attached to no Level"),
            level => AsciiString::new(&format!("IGES Entity, Level Number admitting {level}")),
        }
    }
}
