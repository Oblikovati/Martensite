//! `martensite_object_model_NameIndex`: the document data framework attribute with the unique names of a model's objects.
use martensite_tkernel::{
    impl_transient,
    ncollection::DataMap,
    standard::{Failure, Guid, Handle, SharedState, StandardTransient},
    tcollection::{HExtendedString, HExtendedStringHasher},
};
use martensite_tklcaf::{
    guid_from_text,
    tdata_std::support::{cast_attribute, find_or_add},
    tdf::{TdfAttribute, TdfAttributeBase, TdfAttributeState, TdfLabel, TdfRelocationTable},
};

/// `martensite_object_model_NameIndex::GetID()`.
const NAME_CONTAINER_ID: Guid = guid_from_text("3bbefb47-e618-11d4-ba38-0060b0ee18ea");

/// The map of names: `NCollection_DataMap<handle<TCollection_HExtendedString>, TDF_Label>`,
/// hashed and compared by the string (the runtime handle specialization).
pub type NameIndexMap = DataMap<Handle<HExtendedString>, TdfLabel, HExtendedStringHasher>;

/// `martensite_object_model_NameIndex`; `NameIndex::set(&label)`.
pub struct NameIndex {
    transient: StandardTransient,
    base: TdfAttributeBase,
    map: SharedState<NameIndexMap>,
}
impl_transient!(NameIndex, "martensite_object_model_NameIndex", dyn TdfAttribute, transient,
    interfaces: [dyn TdfAttribute]);

impl Default for NameIndex {
    fn default() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: TdfAttributeBase::default(),
            map: SharedState::new(NameIndexMap::with_hasher(HExtendedStringHasher, 1)),
        }
    }
}

impl NameIndex {
    /// `GetID`.
    pub fn get_id() -> Guid {
        NAME_CONTAINER_ID
    }
    /// `martensite_object_model_NameIndex()`: no name.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Set(theLabel)` (static): the container of the label, added when absent.
    pub fn set(label: &TdfLabel) -> Result<Handle<NameIndex>, Failure> {
        find_or_add(label, &NAME_CONTAINER_ID, false, Self::new)
    }
    /// `RecordName(theName, theLabel)`: records a copy of the name unless registered.
    pub fn record_name(
        &self,
        name: &Handle<HExtendedString>,
        label: &TdfLabel,
    ) -> Result<(), Failure> {
        if self.is_registered(name) {
            return Ok(());
        }
        let text = martensite_tkcdf::live(name, "martensite_object_model_NameIndex::RecordName")?;
        let copy = Handle::new(HExtendedString::from(text.string().clone()));
        self.backup()?;
        self.map.write().bind(copy, label.clone());
        Ok(())
    }
    /// `RemoveName(theName)`: backed up when the name is registered.
    pub fn remove_name(&self, name: &Handle<HExtendedString>) -> Result<(), Failure> {
        if !self.is_registered(name) {
            return Ok(());
        }
        self.backup()?;
        self.map.write().unbind(name);
        Ok(())
    }
    /// `IsRegistered(theName)`.
    pub fn is_registered(&self, name: &Handle<HExtendedString>) -> bool {
        self.map.read().is_bound(name)
    }
    /// `Clear`: backed up, then empty.
    pub fn clear(&self) -> Result<(), Failure> {
        self.backup()?;
        self.map.write().clear(true);
        Ok(())
    }
    /// `Set(theElem)`: backed up, then a copy of the map.
    pub fn set_map(&self, map: &NameIndexMap) -> Result<(), Failure> {
        self.backup()?;
        self.map.write().assign(map);
        Ok(())
    }
    /// `Get`: a copy of the map (by the model a reference to it).
    pub fn get_map(&self) -> NameIndexMap {
        let mut copy = NameIndexMap::with_hasher(HExtendedStringHasher, 1);
        copy.assign(&self.map.read());
        copy
    }
    /// `Get().Find(theName)`: the label of a registered name; `Standard_NoSuchObject`
    /// when the name is not registered.
    pub fn find_label(&self, name: &Handle<HExtendedString>) -> Result<TdfLabel, Failure> {
        self.map.read().find(name).cloned()
    }
}

impl TdfAttribute for NameIndex {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        NAME_CONTAINER_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other =
            cast_attribute::<NameIndex>(with, "martensite_object_model_NameIndex::Restore")?;
        let map = other.get_map();
        self.map.write().assign(&map);
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        _relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = cast_attribute::<NameIndex>(into, "martensite_object_model_NameIndex::Paste")?;
        target.set_map(&self.get_map())
    }
}
