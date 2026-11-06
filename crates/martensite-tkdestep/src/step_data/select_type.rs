//! `StepDataSelectType`: the basis of the EXPRESS SELECT types: a value class holding an
//! entity or a select member, recognised by case number.
use super::{
    StepDataLogical, StepDataPDescr, StepDataSelectInt, StepDataSelectMember, StepDataSelectNamed,
    StepDataSelectReal, StepDataUndefinedEntity,
};
use martensite_tkernel::standard::{
    Failure, Handle, RuntimeType, RuntimeTyped, StandardTransient, exceptions,
};

/// The stored value of a select: `thevalue`, the private field of the class.
pub type StepDataSelectValue = Handle<StandardTransient>;

/// Implements the two storage accessors of [`StepDataSelectType`] over a field of type
/// [`StepDataSelectValue`], inside the `impl StepDataSelectType for ...` block, so each SELECT
/// class does not repeat them; `impl_step_select_storage!(value);`.
#[macro_export]
macro_rules! impl_step_select_storage {
    ($field:ident) => {
        fn select_value(&self) -> &$crate::step_data::StepDataSelectValue {
            &self.$field
        }
        fn select_value_mut(&mut self) -> &mut $crate::step_data::StepDataSelectValue {
            &mut self.$field
        }
    };
}

/// The `case_num` of a SELECT of entity types: the rank of the first type `ent` is a
/// kind of, 0 for null or none; `step_select_case!(ent; StepGeomLine, StepGeomCircle)`.
#[macro_export]
macro_rules! step_select_case {
    ($ent:expr; $($class:ty),+ $(,)?) => {{
        let ent: &martensite_tkernel::standard::Handle<martensite_tkernel::standard::StandardTransient> = $ent;
        let kinds: &[fn() -> &'static martensite_tkernel::standard::RuntimeType] =
            &[$(<$class as martensite_tkernel::standard::RuntimeTyped>::static_type),+];
        $crate::step_data::step_select_case_number(ent, kinds)
    }};
}

/// The rank (from 1) of the first of `kinds` that `ent` is a kind of; 0 for null or none.
/// `step_select_case_number(&Handle::default(), &[]) == 0`.
pub fn step_select_case_number(
    ent: &Handle<StandardTransient>,
    kinds: &[fn() -> &'static RuntimeType],
) -> i32 {
    if ent.is_null() {
        return 0;
    }
    kinds
        .iter()
        .position(|kind| ent.is_kind(kind()))
        .map_or(0, |rank| rank as i32 + 1)
}

fn mismatch(what: &str) -> Failure {
    exceptions::type_mismatch(&format!(
        "StepData : SelectType, {what}: the value does not comply with the select definition"
    ))
}

/// `select_val`: the member that receives an immediate value. `mode` 0 asks for a
/// `SelectInt`, 1 for a `SelectReal`.
fn select_val(
    value: &StepDataSelectValue,
    name: &str,
    mode: i32,
) -> Result<Handle<dyn StepDataSelectMember>, Failure> {
    let member = value.cast::<dyn StepDataSelectMember>();
    if let Some(live) = member.get() {
        if !name.is_empty() && !live.set_name(name) {
            return Err(mismatch("SetInteger"));
        }
        return Ok(member);
    }
    if !name.is_empty() {
        let named = StepDataSelectNamed::default();
        named.set_name(name);
        return Ok(Handle::new(named).cast());
    }
    if mode == 0 {
        return Ok(Handle::new(StepDataSelectInt::default()).cast());
    }
    Ok(Handle::new(StepDataSelectReal::default()).cast())
}

/// `StepDataSelectType`, abstract: `case_num` and the two storage accessors are required
/// (see [`impl_step_select_storage`]); a SELECT with immediate values overrides `case_mem`
/// and `new_member`.
///
/// ```
/// use martensite_tkdestep::{impl_step_select_storage, step_data::{StepDataSelectType, StepDataSelectValue}};
/// use martensite_tkernel::standard::{Handle, StandardTransient};
/// #[derive(Clone, Default)]
/// struct Open { value: StepDataSelectValue }
/// impl StepDataSelectType for Open {
///     impl_step_select_storage!(value);
///     fn case_num(&self, ent: &Handle<StandardTransient>) -> i32 { i32::from(ent.is_some()) }
/// }
/// assert!(Open::default().is_null());
/// ```
pub trait StepDataSelectType {
    /// The stored value.
    fn select_value(&self) -> &StepDataSelectValue;
    /// The stored value, to replace it.
    fn select_value_mut(&mut self) -> &mut StepDataSelectValue;
    /// `case_num(ent)`: the rank of its type in the definition list, 0 when absent.
    fn case_num(&self, ent: &Handle<StandardTransient>) -> i32;
    /// `description`: null by default.
    fn description(&self) -> Handle<StepDataPDescr> {
        Handle::default()
    }
    /// `new_member`: the preferred member, null by default.
    fn new_member(&self) -> Handle<dyn StepDataSelectMember> {
        Handle::default()
    }
    /// `case_mem(ent)`: the rank in the immediate cases, 0 by default.
    fn case_mem(&self, _ent: &Handle<dyn StepDataSelectMember>) -> i32 {
        0
    }
    /// `matches(ent)`: `case_num` or, for a member, `case_mem` is positive.
    fn matches(&self, ent: &Handle<StandardTransient>) -> bool {
        if self.case_num(ent) > 0 {
            return true;
        }
        let member = ent.cast::<dyn StepDataSelectMember>();
        member.is_some() && self.case_mem(&member) > 0
    }
    /// `set_value(ent)`: a type mismatch failure when it does not match; an undefined
    /// entity is always accepted.
    fn set_value(&mut self, ent: &Handle<StandardTransient>) -> Result<(), Failure> {
        if ent.is_some()
            && !ent.is_kind(StepDataUndefinedEntity::static_type())
            && !self.matches(ent)
        {
            return Err(mismatch("SetValue"));
        }
        *self.select_value_mut() = ent.clone();
        Ok(())
    }
    /// `nullify`.
    fn nullify(&mut self) {
        self.select_value_mut().nullify();
    }
    /// `Value`.
    fn value(&self) -> Handle<StandardTransient> {
        self.select_value().clone()
    }
    /// `is_null`.
    fn is_null(&self) -> bool {
        self.select_value().is_null()
    }
    /// `Type`: the dynamic type, `StandardTransient` for null.
    fn dynamic_value_type(&self) -> &'static RuntimeType {
        self.select_value()
            .dynamic_type()
            .unwrap_or_else(StandardTransient::static_type)
    }
    /// `case_number`.
    fn case_number(&self) -> i32 {
        let value = self.select_value();
        if value.is_null() {
            return 0;
        }
        self.case_num(value)
    }
    /// `case_member`.
    fn case_member(&self) -> i32 {
        let member = self.member();
        if member.is_null() {
            return 0;
        }
        self.case_mem(&member)
    }
    /// `member`: the value as a member, null when it is not one.
    fn member(&self) -> Handle<dyn StepDataSelectMember> {
        self.select_value().cast()
    }
    /// `select_name`.
    fn select_name(&self) -> String {
        self.member().get().map_or_else(String::new, |m| m.name())
    }
    /// `int`.
    fn int(&self) -> i32 {
        self.member().get().map_or(0, |m| m.int())
    }
    /// `set_int(val)`: a member must already be there.
    fn set_int(&self, val: i32) -> Result<(), Failure> {
        let member = self.member();
        let live = member.get().ok_or_else(|| mismatch("SetInt"))?;
        live.set_int(val);
        Ok(())
    }
    /// `integer`.
    fn integer(&self) -> i32 {
        self.member().get().map_or(0, |m| m.integer())
    }
    /// `set_integer(val, name = "")`.
    fn set_integer(&mut self, val: i32, name: &str) -> Result<(), Failure> {
        let member = select_val(self.select_value(), name, 0)?;
        if let Some(live) = member.get() {
            live.set_integer(val);
        }
        self.store_member(member, "SetInteger")
    }
    /// `boolean`.
    fn boolean(&self) -> bool {
        self.member().get().is_some_and(|m| m.boolean())
    }
    /// `set_boolean(val, name = "")`.
    fn set_boolean(&mut self, val: bool, name: &str) -> Result<(), Failure> {
        let member = select_val(self.select_value(), name, 0)?;
        if let Some(live) = member.get() {
            live.set_boolean(val);
        }
        self.store_member(member, "SetBoolean")
    }
    /// `logical`: unknown without a member.
    fn logical(&self) -> StepDataLogical {
        self.member()
            .get()
            .map_or(StepDataLogical::LUnknown, |m| m.logical())
    }
    /// `set_logical(val, name = "")`.
    fn set_logical(&mut self, val: StepDataLogical, name: &str) -> Result<(), Failure> {
        let member = select_val(self.select_value(), name, 0)?;
        if let Some(live) = member.get() {
            live.set_logical(val);
        }
        self.store_member(member, "SetLogical")
    }
    /// `real`.
    fn real(&self) -> f64 {
        self.member().get().map_or(0.0, |m| m.real())
    }
    /// `set_real(val, name = "")`.
    fn set_real(&mut self, val: f64, name: &str) -> Result<(), Failure> {
        let member = select_val(self.select_value(), name, 1)?;
        if let Some(live) = member.get() {
            live.set_real(val);
        }
        self.store_member(member, "SetReal")
    }
    /// The end of the immediate setters: refuse what `case_mem` does not know, else
    /// store the member.
    fn store_member(
        &mut self,
        member: Handle<dyn StepDataSelectMember>,
        what: &str,
    ) -> Result<(), Failure> {
        if self.case_mem(&member) == 0 {
            return Err(mismatch(what));
        }
        *self.select_value_mut() = member.cast();
        Ok(())
    }
}
