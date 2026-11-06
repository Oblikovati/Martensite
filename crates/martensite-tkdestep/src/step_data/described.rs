//! The entities of the late-binding mechanism: a simple one has its fields, a complex one
//! ("plex") its simple members. `StepDataFieldList` is the trait of a list of fields.
use super::{StepDataECDescr, StepDataEDescr, StepDataESDescr, StepDataField, StepDataFieldKind};
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    standard::{Failure, Handle, SharedState, StandardTransient, Transient, exceptions},
    tcollection::AsciiString,
};
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceEntityIterator, interface_mismatch};

/// The bounds of `field` as `fill_shared` and `Shared` iterate them.
fn field_bounds(field: &StepDataField) -> [(i32, i32); 2] {
    let mut bounds = [(1, 1), (1, 1)];
    if field.arity() >= 1 {
        bounds[0] = (field.lower(1), field.length(1));
    }
    if field.arity() == 2 {
        bounds[1] = (field.lower(2), field.length(2));
    }
    bounds
}

fn entities_of(
    field: &StepDataField,
    mut each: impl FnMut(Handle<StandardTransient>),
) -> Result<(), Failure> {
    let [(low1, len1), (low2, len2)] = field_bounds(field);
    for offset1 in 0..len1 {
        for offset2 in 0..len2 {
            each(field.entity(low1 + offset1, low2 + offset2)?);
        }
    }
    Ok(())
}

/// `StepDataFieldList`: its base class has no field; [`StepDataFieldListN`] holds them.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataFieldList, StepDataFieldListN};
/// assert_eq!(StepDataFieldListN::new(2).unwrap().nb_fields(), 2);
/// ```
pub trait StepDataFieldList {
    /// `nb_fields`.
    fn nb_fields(&self) -> i32 {
        0
    }
    /// `Field(num)`.
    fn field(&self, num: i32) -> Result<&StepDataField, Failure> {
        Err(exceptions::out_of_range(&format!(
            "StepDataFieldList : Field {num}; this list holds no field"
        )))
    }
    /// `CField(num)`.
    fn c_field(&mut self, num: i32) -> Result<&mut StepDataField, Failure> {
        Err(exceptions::out_of_range(&format!(
            "StepDataFieldList : CField {num}; this list holds no field"
        )))
    }
    /// `fill_shared(iter)`: the entity values, null ones included.
    fn fill_shared(&self, iter: &mut InterfaceEntityIterator) -> Result<(), Failure> {
        for rank in 1..=self.nb_fields() {
            let field = self.field(rank)?;
            if field.kind(true) != StepDataFieldKind::ENTITY {
                continue;
            }
            entities_of(field, |entity| iter.add_item(&entity))?;
        }
        Ok(())
    }
}

/// Field list N.
#[derive(Clone, Default)]
pub struct StepDataFieldListN {
    thefields: Vec<StepDataField>,
}

impl StepDataFieldListN {
    /// `StepDataFieldListN(nb)`: a range failure for a negative count.
    pub fn new(nb: i32) -> Result<Self, Failure> {
        let count = usize::try_from(nb).map_err(|_| {
            exceptions::range_error(&format!(
                "StepDataFieldListN: negative field count {nb}; expected at least 0"
            ))
        })?;
        Ok(Self {
            thefields: vec![StepDataField::default(); count],
        })
    }
}

impl StepDataFieldList for StepDataFieldListN {
    fn nb_fields(&self) -> i32 {
        self.thefields.len() as i32
    }
    fn field(&self, num: i32) -> Result<&StepDataField, Failure> {
        let count = self.thefields.len();
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| self.thefields.get(rank))
            .ok_or_else(|| {
                exceptions::out_of_range(&format!(
                    "StepDataFieldListN: field {num}; expected [1, {count}]"
                ))
            })
    }
    fn c_field(&mut self, num: i32) -> Result<&mut StepDataField, Failure> {
        let count = self.thefields.len();
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| self.thefields.get_mut(rank))
            .ok_or_else(|| {
                exceptions::out_of_range(&format!(
                    "StepDataFieldListN: field {num}; expected [1, {count}]"
                ))
            })
    }
}

/// `StepDataDescribed`, abstract. `Field` gives a shallow copy and `CField` edits in place,
/// because a reference cannot outlive the object's lock.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataDescribed, StepDataESDescr, StepDataSimple};
/// let simple = StepDataSimple::new(&StepDataESDescr::new("POINT"));
/// assert!(!simple.is_complex() && simple.matches("POINT"));
/// ```
pub trait StepDataDescribed: Transient {
    /// `description`.
    fn description(&self) -> Handle<dyn StepDataEDescr>;
    /// `is_complex`.
    fn is_complex(&self) -> bool;
    /// `matches(steptype)`.
    fn matches(&self, steptype: &str) -> bool;
    /// `As(steptype)`.
    fn as_simple(&self, steptype: &str) -> Handle<StepDataSimple>;
    /// `has_field(name)`.
    fn has_field(&self, name: &str) -> bool;
    /// `Field(name)`.
    fn field(&self, name: &str) -> Result<StepDataField, Failure>;
    /// `CField(name)`.
    fn change_field(
        &self,
        name: &str,
        edit: &mut dyn FnMut(&mut StepDataField),
    ) -> Result<(), Failure>;
    /// `Check(ach)`.
    fn check(&self, ach: &Handle<InterfaceCheck>);
    /// `Shared(list)`.
    fn shared(&self, list: &mut InterfaceEntityIterator) -> Result<(), Failure>;
}
impl_transient_interface!(
    dyn StepDataDescribed,
    "StepDataDescribed",
    StandardTransient
);

/// Simple.
pub struct StepDataSimple {
    transient: StandardTransient,
    thedescr: Handle<StepDataESDescr>,
    thefields: SharedState<StepDataFieldListN>,
    this: std::sync::OnceLock<martensite_tkernel::standard::WeakHandle<StepDataSimple>>,
}
impl_transient!(
    StepDataSimple,
    "StepDataSimple",
    dyn StepDataDescribed,
    transient,
    interfaces: [dyn StepDataDescribed]
);

impl StepDataSimple {
    /// `StepDataSimple(descr)`.
    pub fn new(descr: &Handle<StepDataESDescr>) -> Self {
        let nb = descr.get().map_or(0, StepDataESDescr::nb_fields);
        Self {
            transient: StandardTransient::default(),
            thedescr: descr.clone(),
            thefields: SharedState::new(StepDataFieldListN::new(nb).unwrap_or_default()),
            this: std::sync::OnceLock::new(),
        }
    }
    /// Records its handle, for `As` (ocaf-design.md section 2).
    pub fn bind_self(&self, this: &Handle<StepDataSimple>) {
        let _ = self.this.set(this.downgrade());
    }
    /// `ESDescr`.
    pub fn es_descr(&self) -> Handle<StepDataESDescr> {
        self.thedescr.clone()
    }
    /// `step_type`.
    pub fn step_type(&self) -> String {
        self.thedescr
            .get()
            .map_or_else(String::new, |d| d.type_name().to_owned())
    }
    fn rank_of(&self, name: &str) -> Result<i32, Failure> {
        let rank = self.thedescr.get().map_or(0, |d| d.rank(name));
        if rank == 0 {
            return Err(interface_mismatch(&format!(
                "StepDataSimple : Field {name}; no field has this name"
            )));
        }
        Ok(rank)
    }
    /// `nb_fields`.
    pub fn nb_fields(&self) -> i32 {
        self.thefields.read().nb_fields()
    }
    /// `field_num(num)`.
    pub fn field_num(&self, num: i32) -> Result<StepDataField, Failure> {
        Ok(self.thefields.read().field(num)?.clone())
    }
    /// `CFieldNum(num)`: edits the field.
    pub fn change_field_num<R>(
        &self,
        num: i32,
        edit: impl FnOnce(&mut StepDataField) -> R,
    ) -> Result<R, Failure> {
        Ok(edit(self.thefields.write().c_field(num)?))
    }
    /// `fields`: a shallow copy.
    pub fn fields(&self) -> StepDataFieldListN {
        self.thefields.read().clone()
    }
    /// `CFields`: edits the list.
    pub fn change_fields<R>(&self, edit: impl FnOnce(&mut StepDataFieldListN) -> R) -> R {
        edit(&mut self.thefields.write())
    }
}

impl StepDataDescribed for StepDataSimple {
    fn description(&self) -> Handle<dyn StepDataEDescr> {
        self.thedescr.cast()
    }
    fn is_complex(&self) -> bool {
        false
    }
    fn matches(&self, steptype: &str) -> bool {
        self.thedescr.get().is_some_and(|d| d.matches(steptype))
    }
    fn as_simple(&self, steptype: &str) -> Handle<StepDataSimple> {
        if !self.matches(steptype) {
            return Handle::default();
        }
        self.this
            .get()
            .map(martensite_tkernel::standard::WeakHandle::upgrade)
            .unwrap_or_default()
    }
    fn has_field(&self, name: &str) -> bool {
        self.thedescr.get().is_some_and(|d| d.rank(name) > 0)
    }
    fn field(&self, name: &str) -> Result<StepDataField, Failure> {
        self.field_num(self.rank_of(name)?)
    }
    fn change_field(
        &self,
        name: &str,
        edit: &mut dyn FnMut(&mut StepDataField),
    ) -> Result<(), Failure> {
        let rank = self.rank_of(name)?;
        self.change_field_num(rank, |field| edit(field))
    }
    fn check(&self, _ach: &Handle<InterfaceCheck>) {}
    fn shared(&self, list: &mut InterfaceEntityIterator) -> Result<(), Failure> {
        let fields = self.thefields.read();
        for rank in 1..=fields.nb_fields() {
            entities_of(fields.field(rank)?, |entity| {
                if entity.is_some() {
                    list.add_item(&entity);
                }
            })?;
        }
        Ok(())
    }
}

/// Plex.
pub struct StepDataPlex {
    transient: StandardTransient,
    thedescr: Handle<StepDataECDescr>,
    themembers: SharedState<Vec<Handle<StepDataSimple>>>,
}
impl_transient!(
    StepDataPlex,
    "StepDataPlex",
    dyn StepDataDescribed,
    transient,
    interfaces: [dyn StepDataDescribed]
);

impl StepDataPlex {
    /// `StepDataPlex(descr)`.
    pub fn new(descr: &Handle<StepDataECDescr>) -> Self {
        Self {
            transient: StandardTransient::default(),
            thedescr: descr.clone(),
            themembers: SharedState::new(Vec::new()),
        }
    }
    /// `add(member)`.
    pub fn add(&self, member: &Handle<StepDataSimple>) {
        self.themembers.write().push(member.clone());
    }
    /// `ECDescr`.
    pub fn ec_descr(&self) -> Handle<StepDataECDescr> {
        self.thedescr.clone()
    }
    /// `nb_members`.
    pub fn nb_members(&self) -> i32 {
        self.themembers.read().len() as i32
    }
    /// `member(num)`: null out of range.
    pub fn member(&self, num: i32) -> Handle<StepDataSimple> {
        let members = self.themembers.read();
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| members.get(rank).cloned())
            .unwrap_or_default()
    }
    /// `type_list`.
    pub fn type_list(&self) -> Vec<AsciiString> {
        let members = self.themembers.read();
        members
            .iter()
            .filter_map(|m| m.get().map(|s| AsciiString::new(&s.step_type())))
            .collect()
    }
    fn holder_of(&self, name: &str) -> Result<Handle<StepDataSimple>, Failure> {
        let members = self.themembers.read();
        members
            .iter()
            .find(|m| m.get().is_some_and(|s| s.has_field(name)))
            .cloned()
            .ok_or_else(|| {
                interface_mismatch(&format!(
                    "StepDataPlex : Field {name}; no member has this field"
                ))
            })
    }
}

impl StepDataDescribed for StepDataPlex {
    fn description(&self) -> Handle<dyn StepDataEDescr> {
        self.thedescr.cast()
    }
    fn is_complex(&self) -> bool {
        true
    }
    fn matches(&self, steptype: &str) -> bool {
        let members = self.themembers.read();
        members
            .iter()
            .any(|m| m.get().is_some_and(|s| s.matches(steptype)))
    }
    fn as_simple(&self, steptype: &str) -> Handle<StepDataSimple> {
        let members = self.themembers.read();
        members
            .iter()
            .find(|m| m.get().is_some_and(|s| s.matches(steptype)))
            .cloned()
            .unwrap_or_default()
    }
    fn has_field(&self, name: &str) -> bool {
        self.holder_of(name).is_ok()
    }
    fn field(&self, name: &str) -> Result<StepDataField, Failure> {
        let holder = self.holder_of(name)?;
        holder
            .get()
            .map_or_else(|| Ok(StepDataField::default()), |s| s.field(name))
    }
    fn change_field(
        &self,
        name: &str,
        edit: &mut dyn FnMut(&mut StepDataField),
    ) -> Result<(), Failure> {
        let holder = self.holder_of(name)?;
        holder.get().map_or(Ok(()), |s| s.change_field(name, edit))
    }
    fn check(&self, ach: &Handle<InterfaceCheck>) {
        for member in self.themembers.read().iter() {
            if let Some(simple) = member.get() {
                simple.check(ach);
            }
        }
    }
    fn shared(&self, list: &mut InterfaceEntityIterator) -> Result<(), Failure> {
        for member in self.themembers.read().iter() {
            if let Some(simple) = member.get() {
                simple.shared(list)?;
            }
        }
        Ok(())
    }
}
