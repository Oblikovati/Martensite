//! The list setters of `StepDataField`. Setting a value of another type than the list's
//! changes the list to transients holding select members.
use super::field::{StringList, TransientList, array_failure};
use super::{
    StepDataField, StepDataFieldKind as Kind, StepDataLogical, StepDataSelectInt,
    StepDataSelectMember, StepDataSelectNamed, StepDataSelectReal,
};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle, StandardTransient},
    tcollection::HAsciiString,
};

fn set_item<T: Send + Sync + 'static>(
    list: &HArray1<T>,
    num: i32,
    value: T,
) -> Result<(), Failure> {
    list.change_array1()
        .set_value(num, value)
        .map_err(array_failure)
}

fn item_of<T: Clone + Send + Sync + 'static>(list: &HArray1<T>, num: i32) -> Result<T, Failure> {
    Ok(list.array1().value(num).map_err(array_failure)?.clone())
}

impl StepDataField {
    /// `clear_item(num)`: ignored unless a list of strings or transients.
    pub fn clear_item(&mut self, num: i32) -> Result<(), Failure> {
        if let Some(list) = self.theany.cast::<TransientList>().get() {
            set_item(list, num, Handle::default())?;
        }
        if let Some(list) = self.theany.cast::<StringList>().get() {
            set_item(list, num, Handle::default())?;
        }
        Ok(())
    }
    /// `set_int(num, val, kind)`.
    pub fn set_int_at(&mut self, num: i32, val: i32, kind: Kind) -> Result<(), Failure> {
        if let Some(list) = self.theany.cast::<HArray1<i32>>().get() {
            return set_item(list, num, val);
        }
        let list = self.theany.cast::<TransientList>();
        let Some(live) = list.get() else {
            return Ok(());
        };
        self.thekind = Kind::ANY.combine(Kind::LIST);
        let mut member = item_of(live, num)?.cast::<dyn StepDataSelectMember>();
        if member.is_null() {
            member = Handle::new(StepDataSelectInt::default()).cast();
            set_item(live, num, member.cast())?;
        }
        if let Some(member) = member.get() {
            member.set_kind(kind.to_int());
            member.set_int(val);
        }
        Ok(())
    }
    /// `set_integer(num, val)`.
    pub fn set_integer_at(&mut self, num: i32, val: i32) -> Result<(), Failure> {
        self.set_int_at(num, val, Kind::INTEGER)
    }
    /// `set_boolean(num, val)`.
    pub fn set_boolean_at(&mut self, num: i32, val: bool) -> Result<(), Failure> {
        self.set_int_at(num, i32::from(val), Kind::BOOLEAN)
    }
    /// `set_logical(num, val)`.
    pub fn set_logical_at(&mut self, num: i32, val: StepDataLogical) -> Result<(), Failure> {
        self.set_int_at(num, val as i32, Kind::LOGICAL)
    }
    /// `set_enum(num, val, text = "")`.
    pub fn set_enum_at(&mut self, num: i32, val: i32, text: &[u8]) -> Result<(), Failure> {
        let list = self.theany.cast::<TransientList>();
        let Some(live) = list.get() else {
            return self.set_integer_at(num, val);
        };
        self.thekind = Kind::ANY.combine(Kind::LIST);
        let mut member = item_of(live, num)?.cast::<dyn StepDataSelectMember>();
        if member.is_null() {
            member = Handle::new(StepDataSelectNamed::default()).cast();
            set_item(live, num, member.cast())?;
        }
        if let Some(member) = member.get() {
            member.set_enum(val, text);
        }
        Ok(())
    }
    /// `set_real(num, val)`.
    pub fn set_real_at(&mut self, num: i32, val: f64) -> Result<(), Failure> {
        if let Some(list) = self.theany.cast::<HArray1<f64>>().get() {
            return set_item(list, num, val);
        }
        let list = self.theany.cast::<TransientList>();
        let Some(live) = list.get() else {
            return Ok(());
        };
        self.thekind = Kind::ANY.combine(Kind::LIST);
        let mut member = item_of(live, num)?.cast::<dyn StepDataSelectMember>();
        if member.is_null() {
            member = Handle::new(StepDataSelectReal::default()).cast();
            set_item(live, num, member.cast())?;
        }
        if let Some(member) = member.get() {
            member.set_real(val);
        }
        Ok(())
    }
    /// `set_string(num, val)`.
    pub fn set_string_at(&mut self, num: i32, val: &[u8]) -> Result<(), Failure> {
        let text = Handle::new(HAsciiString::new(val));
        if let Some(list) = self.theany.cast::<StringList>().get() {
            return set_item(list, num, text);
        }
        let list = self.theany.cast::<TransientList>();
        let Some(live) = list.get() else {
            return Ok(());
        };
        self.thekind = Kind::ANY.combine(Kind::LIST);
        set_item(live, num, text.cast())
    }
    /// `set_entity(num, val)`: a list of immediates becomes a list of transients.
    /// That new list is not stored in the field, only the kind changes.
    pub fn set_entity_at(
        &mut self,
        num: i32,
        val: &Handle<StandardTransient>,
    ) -> Result<(), Failure> {
        if let Some(list) = self.theany.cast::<TransientList>().get() {
            return set_item(list, num, val.clone());
        }
        let is_immediate = self.theany.cast::<HArray1<i32>>().is_some()
            || self.theany.cast::<HArray1<f64>>().is_some()
            || self.theany.cast::<StringList>().is_some();
        if is_immediate {
            self.thekind = Kind::ANY.combine(Kind::LIST);
        }
        Ok(())
    }
}
