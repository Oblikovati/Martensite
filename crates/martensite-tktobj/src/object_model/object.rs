//! `martensite_object_model_ModelObject`: the base of the transient objects of an document data framework-based model. Its virtual
//! members are the trait [`ModelObject`]; its non-virtual members are the extension traits
//! [`super::ModelObjectState`] and [`super::ModelObjectData`] (object-model.md section 3).
use super::DocumentModelBase;
use super::object_children::object_children;
use super::object_detach::{
    object_can_detach, object_detach, object_remove_back_reference, object_remove_back_references,
};
use super::object_names::{object_get_dictionary, object_get_name, object_set_name};
use super::object_references::{
    object_get_bad_reference, object_has_reference, object_relocate_references,
    object_replace_reference,
};
use super::{
    DeletionMode, DocumentModel, ModelObjectState, NameIndex, ObjectIterator, ReferenceIterator,
    TypedSequenceIterator,
};
use martensite_tkernel::{
    impl_transient_interface,
    ncollection::HSequence,
    standard::{
        Failure, Handle, RuntimeType, SharedState, StandardTransient, Transient, WeakHandle,
        exceptions,
    },
    tcollection::HExtendedString,
};
use martensite_tklcaf::tdf::{TdfLabel, TdfRelocationTable};
use std::sync::OnceLock;

/// The shared sequence of back references (`NCollection_HSequence<handle<martensite_object_model_ModelObject>>`).
pub type BackReferences = HSequence<Handle<dyn ModelObject>>;

/// `martensite_object_model_ModelObject::TypeFlags`: flags that define type-specific behaviour of objects.
pub struct ObjectTypeFlags;
impl ObjectTypeFlags {
    /// `Visible`: the object is visible in a data viewer.
    pub const VISIBLE: i32 = 0x0001;
}

/// `martensite_object_model_ModelObject::ObjectState`: the object state bit flags of `SetFlags`/`GetFlags`.
pub struct ObjectStateFlags;
impl ObjectStateFlags {
    /// `ObjectState_Hidden`: hidden in a tree browser.
    pub const HIDDEN: i32 = 0x0001;
    /// `ObjectState_Saved`: has (or should have) a saved file on disk.
    pub const SAVED: i32 = 0x0002;
    /// `ObjectState_Imported`: the data are just imported from somewhere.
    pub const IMPORTED: i32 = 0x0004;
    /// `ObjectState_ImportedByFile`: a model imported by file.
    pub const IMPORTED_BY_FILE: i32 = 0x0008;
    /// `ObjectState_Ordered`: the partition keeps its objects ordered.
    pub const ORDERED: i32 = 0x0010;
}

/// `martensite_object_model_ModelObject::DataTag`: the ranks of the labels under the Data section.
pub struct ObjectDataTag;
impl ObjectDataTag {
    /// `DataTag_First`.
    pub const FIRST: i32 = 0;
    /// `DataTag_Flags`: the flags of the object.
    pub const FLAGS: i32 = 1;
    /// `DataTag_Order`: the order of the object.
    pub const ORDER: i32 = 2;
    /// `DataTag_Last`: reserved up to `DataTag_First + 100`.
    pub const LAST: i32 = Self::FIRST + 100;
}

/// `martensite_object_model_ModelObject::ChildTag`: the ranks of the labels under the Child section.
pub struct ObjectChildTag;
impl ObjectChildTag {
    /// `ChildTag_First`.
    pub const FIRST: i32 = 0;
    /// `ChildTag_Last`: no children.
    pub const LAST: i32 = Self::FIRST;
}

/// `martensite_object_model_ModelObject::RefTag`: the ranks of the labels under the Reference section.
pub struct ObjectReferenceTag;
impl ObjectReferenceTag {
    /// `RefTag_First`.
    pub const FIRST: i32 = 0;
    /// `RefTag_Last`: no children.
    pub const LAST: i32 = Self::FIRST;
}

/// The private fields of `martensite_object_model_ModelObject` (`myLabel`, `myHSeqBackRef`) and the self handle
/// (object-model.md section 2).
pub struct ModelObjectBase {
    this: OnceLock<WeakHandle<dyn ModelObject>>,
    label: SharedState<TdfLabel>,
    back_references: SharedState<Handle<BackReferences>>,
}

impl ModelObjectBase {
    /// The fields of `martensite_object_model_ModelObject(const ObjectModel_Persistence*, theLabel)`: the label only, not
    /// yet attached; `ModelObjectBase::new(&label)`.
    pub fn new(label: &TdfLabel) -> Self {
        Self {
            this: OnceLock::new(),
            label: SharedState::new(label.clone()),
            back_references: SharedState::new(Handle::default()),
        }
    }
    /// Records the owning handle once; the constructors that return a handle call it.
    pub fn bind_self(&self, object: &Handle<dyn ModelObject>) {
        let _ = self.this.set(object.downgrade());
    }
    /// The handle `this` converts to by the model; `Standard_NullObject` before `bind_self`.
    pub fn this(&self) -> Result<Handle<dyn ModelObject>, Failure> {
        let this = self.this.get().map(WeakHandle::upgrade).unwrap_or_default();
        if this.is_null() {
            return Err(exceptions::null_object(
                "martensite_object_model_ModelObject: the object has no owning handle; expected bind_self",
            ));
        }
        Ok(this)
    }
    /// The body of `martensite_object_model_ModelObject(theLabel, theSetName)` once the object is in `object`:
    /// binds it, attaches it to its label with a `martensite_object_model_ObjectAttribute` and names it.
    pub fn attach(object: &Handle<dyn ModelObject>, set_name: bool) -> Result<(), Failure> {
        let target = martensite_tkcdf::live(
            object,
            "martensite_object_model_ModelObject::martensite_object_model_ModelObject",
        )?;
        target.object_base().bind_self(object);
        super::ObjectAttribute::set(&target.get_label(), object)?;
        if set_name {
            DocumentModelBase::set_new_name(object)?;
        }
        Ok(())
    }
    /// `myLabel`.
    pub(crate) fn label(&self) -> TdfLabel {
        self.label.read().clone()
    }
    /// Sets `myLabel` (the friend `martensite_object_model_ObjectAttribute` kills and raises objects this way).
    pub(crate) fn set_label(&self, label: &TdfLabel) {
        *self.label.write() = label.clone();
    }
    /// `myHSeqBackRef` (null when there is no back reference).
    pub(crate) fn back_references(&self) -> Handle<BackReferences> {
        self.back_references.read().clone()
    }
    /// Replaces `myHSeqBackRef`.
    pub(crate) fn set_back_references(&self, sequence: Handle<BackReferences>) {
        *self.back_references.write() = sequence;
    }
}

/// `martensite_object_model_ModelObject`'s virtual members; a concrete object embeds a [`ModelObjectBase`] and
/// registers `dyn ModelObject` as an interface of its `impl_transient!`. The provided
/// methods carry the runtime `martensite_object_model_ModelObject` bodies.
pub trait ModelObject: Transient {
    /// The embedded base with the runtime private fields.
    fn object_base(&self) -> &ModelObjectBase;
    /// `GetModel`: the model of the document holding the object, null when none.
    fn get_model(&self) -> Result<Handle<dyn DocumentModel>, Failure> {
        super::object_names::model_of_label(&self.get_label())
    }
    /// `GetChildren(theType)`: the objects under the child label (ordered by `GetOrder`
    /// when the object has the `ObjectState_Ordered` flag).
    fn get_children(
        &self,
        kind: Option<&'static RuntimeType>,
    ) -> Result<Handle<dyn ObjectIterator>, Failure> {
        object_children(self, kind)
    }
    /// `GetDictionary`: the name container of the model, null without a model.
    fn get_dictionary(&self) -> Result<Handle<NameIndex>, Failure> {
        object_get_dictionary(self)
    }
    /// `GetName`: the `TDataStd_Name` of the label, an empty string when there is none.
    fn get_name(&self) -> Result<Handle<HExtendedString>, Failure> {
        object_get_name(self)
    }
    /// `SetName(theName)`: false when the name is registered already.
    fn set_name(&self, name: &Handle<HExtendedString>) -> Result<bool, Failure> {
        object_set_name(self, name)
    }
    /// `GetNameForClone`: the same name by default.
    fn get_name_for_clone(
        &self,
        _clone: &Handle<dyn ModelObject>,
    ) -> Result<Handle<HExtendedString>, Failure> {
        self.get_name()
    }
    /// `HasReference(theObject)`.
    fn has_reference(&self, object: &Handle<dyn ModelObject>) -> Result<bool, Failure> {
        object_has_reference(self, object)
    }
    /// `GetReferences(theType)`: a `martensite_object_model_ReferenceIterator` on the reference label.
    fn get_references(
        &self,
        kind: Option<&'static RuntimeType>,
    ) -> Result<Handle<dyn ObjectIterator>, Failure> {
        let label = self.get_reference_label()?;
        Ok(ReferenceIterator::new(&label, kind, true)?.cast())
    }
    /// `RemoveAllReferences`: forgets every attribute of the reference label.
    fn remove_all_references(&self) -> Result<(), Failure> {
        self.get_reference_label()?.forget_all_attributes(true)
    }
    /// `GetBackReferences(theType)`: a `martensite_object_model_TypedSequenceIterator` on the back references.
    fn get_back_references(
        &self,
        kind: Option<&'static RuntimeType>,
    ) -> Handle<dyn ObjectIterator> {
        let sequence = self.object_base().back_references();
        Handle::new(TypedSequenceIterator::new(&sequence, kind)).cast()
    }
    /// `AddBackReference(theObject)`.
    fn add_back_reference(&self, object: &Handle<dyn ModelObject>) {
        let base = self.object_base();
        let mut sequence = base.back_references();
        if sequence.is_null() {
            sequence = Handle::new(BackReferences::default());
            base.set_back_references(sequence.clone());
        }
        sequence.append(object.clone());
    }
    /// `RemoveBackReference(theObject, theSingleOnly)`.
    fn remove_back_reference(&self, object: &Handle<dyn ModelObject>, single_only: bool) {
        object_remove_back_reference(self, object, single_only);
    }
    /// `RemoveBackReferences(theMode)`: unlinks or deletes the referring objects.
    fn remove_back_references(&self, mode: DeletionMode) -> Result<bool, Failure> {
        object_remove_back_references(self, mode)
    }
    /// `ClearBackReferences`.
    fn clear_back_references(&self) {
        self.object_base().set_back_references(Handle::default());
    }
    /// `ReplaceReference(theOldObject, theNewObject)`.
    fn replace_reference(
        &self,
        old_object: &Handle<dyn ModelObject>,
        new_object: &Handle<dyn ModelObject>,
    ) -> Result<(), Failure> {
        object_replace_reference(self, old_object, new_object)
    }
    /// `GetBadReference(theRoot, theBadReference)`.
    fn get_bad_reference(
        &self,
        root: &TdfLabel,
        bad_reference: &mut TdfLabel,
    ) -> Result<bool, Failure> {
        object_get_bad_reference(self, root, bad_reference)
    }
    /// `RelocateReferences(theFromRoot, theToRoot, theUpdateBackRefs)`.
    fn relocate_references(
        &self,
        from_root: &TdfLabel,
        to_root: &TdfLabel,
        update_back_references: bool,
    ) -> Result<bool, Failure> {
        object_relocate_references(self, from_root, to_root, update_back_references)
    }
    /// `CanRemoveReference`: false by default.
    fn can_remove_reference(&self, _object: &Handle<dyn ModelObject>) -> Result<bool, Failure> {
        Ok(false)
    }
    /// `RemoveReference(theObject)`: replaces the reference with a null one.
    fn remove_reference(&self, object: &Handle<dyn ModelObject>) -> Result<(), Failure> {
        self.replace_reference(object, &Handle::default())
    }
    /// `BeforeForgetReference`: called by `martensite_object_model_ReferenceAttribute::BeforeForget`; empty by default.
    fn before_forget_reference(&self, _label: &TdfLabel) -> Result<(), Failure> {
        Ok(())
    }
    /// `CanDetach(theMode)`.
    fn can_detach(&self, mode: DeletionMode) -> Result<bool, Failure> {
        object_can_detach(self, mode)
    }
    /// `Detach(theMode)`: deletes the object from the model.
    fn detach(&self, mode: DeletionMode) -> Result<bool, Failure> {
        object_detach(self, mode)
    }
    /// `IsAlive`: the label holds this object.
    fn is_alive(&self) -> Result<bool, Failure> {
        let label = self.get_label();
        if label.is_null() {
            return Ok(false);
        }
        let mut found = Handle::default();
        ModelObjectBase::get_obj(&label, &mut found, false)
    }
    /// `Clone(theTargetLabel, theRelocTable)`: copies the object, its data and children.
    fn clone_object(
        &self,
        target_label: &TdfLabel,
        relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<Handle<dyn ModelObject>, Failure> {
        super::object_clone::object_clone(self, target_label, relocation_table)
    }
    /// `CopyReferences(theTargetObject, theRelocTable)`.
    fn copy_references(
        &self,
        target: &Handle<dyn ModelObject>,
        relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        super::object_clone::object_copy_references(self, target, relocation_table)
    }
    /// `CopyChildren(theTargetLabel, theRelocTable)`.
    fn copy_children(
        &self,
        target_label: &TdfLabel,
        relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        super::object_clone::object_copy_children(self, target_label, relocation_table)
    }
    /// `GetOrder`: the stored order, else the tag of the label.
    fn get_order(&self) -> Result<i32, Failure> {
        let order = self.get_integer(ObjectDataTag::ORDER, 0)?;
        Ok(if order == 0 {
            self.get_label().tag()
        } else {
            order
        })
    }
    /// `SetOrder(theIndx)`.
    fn set_order(&self, index: i32) -> Result<bool, Failure> {
        self.set_integer(index, ObjectDataTag::ORDER, 0)?;
        Ok(true)
    }
    /// `copyData(theTargetObject)` (protected): copies the data label.
    fn copy_data(&self, target: &Handle<dyn ModelObject>) -> Result<bool, Failure> {
        super::object_clone::object_copy_data(self, target)
    }
    /// `GetTypeFlags`: `Visible` by default.
    fn get_type_flags(&self) -> i32 {
        ObjectTypeFlags::VISIBLE
    }
    /// `AfterRetrieval`: registers the name in the model.
    fn after_retrieval(&self) -> Result<(), Failure> {
        super::object_names::object_after_retrieval(self)
    }
    /// `BeforeStoring`: does nothing by default.
    fn before_storing(&self) -> Result<(), Failure> {
        Ok(())
    }
    /// `initFields` (protected): the persistence constructor calls it; empty by default.
    fn init_fields(&self) {}
}
impl_transient_interface!(
    dyn ModelObject,
    "martensite_object_model_ModelObject",
    StandardTransient
);

use super::ModelObjectData;
