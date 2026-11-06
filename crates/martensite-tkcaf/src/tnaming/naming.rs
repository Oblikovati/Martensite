//! `TNamingNaming`: the attribute that stores a `TNamingName` on the label of a
//! selection. Building a name (`Name`) and solving it (`Solve`, `Regenerate`) are in
//! `naming_build.rs` and `naming_solve.rs`.
use super::TNamingName;
use martensite_tkcdf::json_dump::{class_begin, field_dumped};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Guid, Handle, SharedState, StandardTransient, dump::DumpStream},
};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::json::{AttributeIndexMap, dump_attribute_json_base};
use martensite_tklcaf::live;
use martensite_tklcaf::tdata_std::support::cast_attribute;
use martensite_tklcaf::tdf::{
    TdfAttribute, TdfAttributeBase, TdfChildIterator, TdfDataSet, TdfIdFilter, TdfLabel,
    TdfLabelMap, TdfRelocationTable, TdfTagSource,
};
use std::io;
use std::sync::RwLockWriteGuard;

/// `TNamingNaming::GetID()`.
const NAMING_ID: Guid = guid_from_text("c0a19201-5b78-11d1-8940-080009dc3333");

/// `TNamingNaming`.
///
/// ```
/// use martensite_tkcaf::tnaming::{TNamingNameType, TNamingNaming};
/// let naming = TNamingNaming::new();
/// assert!(!naming.is_defined());
/// naming.change_name().set_type(TNamingNameType::Identity);
/// assert!(naming.is_defined());
/// ```
pub struct TNamingNaming {
    transient: StandardTransient,
    base: TdfAttributeBase,
    name: SharedState<TNamingName>,
}
impl_transient!(TNamingNaming, "TNamingNaming", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl Default for TNamingNaming {
    fn default() -> Self {
        Self::new()
    }
}

impl TNamingNaming {
    /// `GetID`.
    pub fn get_id() -> Guid {
        NAMING_ID
    }
    /// `Insert(under)`: a new naming on a new child of `under` (`TDFTagSource::NewChild`).
    pub fn insert(under: &TdfLabel) -> Result<Handle<TNamingNaming>, Failure> {
        let child = TdfTagSource::new_child_of(under)?;
        let naming = Handle::new(TNamingNaming::new());
        child.add_attribute(&naming.cast(), true)?;
        Ok(naming)
    }
    /// `TNamingNaming()`.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: TdfAttributeBase::default(),
            name: SharedState::new(TNamingName::new()),
        }
    }
    /// `IsDefined`: the name's type is not `UNKNOWN`.
    pub fn is_defined(&self) -> bool {
        self.name.read().name_type() != super::TNamingNameType::Unknown
    }
    /// `GetName`: a copy of the name.
    pub fn get_name(&self) -> TNamingName {
        self.name.read().clone()
    }
    /// `ChangeName`: the name itself; drop the guard before calling back into the
    /// framework.
    pub fn change_name(&self) -> RwLockWriteGuard<'_, TNamingName> {
        self.name.write()
    }
    /// `Regenerate(scope)`: solves this naming's name only (`TNamingName::Solve` on a copy
    /// of the name, which an algorithm may change meanwhile, as natively through
    /// `ChangeName`).
    pub fn regenerate(&self, valid: &TdfLabelMap) -> bool {
        self.get_name().solve(&self.base.label(), valid)
    }
    /// `Solve(scope)`: solves the namings of the child labels first (stopping at the first
    /// failure), then this one; a solved naming adds its label to a non-empty scope.
    pub fn solve(&self, valid: &mut TdfLabelMap) -> Result<bool, Failure> {
        let label = self.base.label();
        let mut children = TdfChildIterator::new(&label, false)?;
        while children.more() {
            let mut child: Handle<TNamingNaming> = Handle::default();
            if children.value().find_attribute_as(&NAMING_ID, &mut child)?
                && !live(&child, "TNamingNaming::Solve")?.solve(valid)?
            {
                // Not necessary to continue.
                return Ok(false);
            }
            children.next();
        }
        if !self.regenerate(valid) {
            return Ok(false);
        }
        if !valid.is_empty() {
            valid.add(label);
        }
        Ok(true)
    }
}

impl TdfAttribute for TNamingNaming {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        NAMING_ID
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(TNamingNaming::new()).cast()
    }
    /// `Restore`: the other naming's name.
    fn restore(&self, other: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = cast_attribute::<TNamingNaming>(other, "TNamingNaming::Restore")?;
        let name = live(&other, "TNamingNaming::Restore")?.get_name();
        *self.name.write() = name;
        Ok(())
    }
    /// `Paste`: `TNamingName::Paste` into the target's name.
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = cast_attribute::<TNamingNaming>(into, "TNamingNaming::Paste")?;
        let table = live(relocation_table, "TNamingNaming::Paste")?;
        let name = self.get_name();
        name.paste(
            &mut live(&target, "TNamingNaming::Paste")?.change_name(),
            table,
        );
        Ok(())
    }
    /// `References`: the arguments and the stop named shape of the name.
    fn references(&self, data_set: &Handle<TdfDataSet>) {
        let Some(data_set) = data_set.get() else {
            return;
        };
        let name = self.get_name();
        for argument in name.arguments() {
            data_set.add_attribute(&argument.cast());
        }
        let stop = name.stop_named_shape();
        if stop.is_some() {
            data_set.add_attribute(&stop.cast());
        }
    }
    /// `Dump`: `TNamingNaming`.
    fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
        write!(stream, "TNamingNaming")
    }
    /// `ExtendedDump`: `TNamingNaming ExtendedDump `.
    fn extended_dump(
        &self,
        stream: &mut dyn io::Write,
        _filter: &TdfIdFilter,
        _map: &mut AttributeIndexMap,
    ) -> io::Result<()> {
        write!(stream, "TNamingNaming ExtendedDump  ")
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "TNamingNaming")?;
        dump_attribute_json_base(self, stream, depth)?;
        let name = self.get_name();
        let child = |s: &mut DumpStream, d: i32| name.dump_json(s, d);
        field_dumped(stream, depth, "&myName", Some(&child))
    }
}
