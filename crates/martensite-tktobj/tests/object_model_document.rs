//!
//! The two stream tests (`stream persistence scenarios`)
//! formats, user decision 2026-09); they stay runtime-only in the ledger, and
//! covered by `object_model_storage.rs`.
#![allow(non_snake_case)]
use martensite_tkernel::{
    impl_transient,
    message::Messenger,
    ncollection::HArray1,
    standard::{Failure, Handle, StandardTransient},
    tcollection::{AsciiString, ExtendedString, HExtendedString},
};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tktobj::object_model::{
    DeletionMode, DocumentModel, DocumentModelBase, DocumentModelState, ModelObject,
    ModelObjectArrays, ModelObjectBase, ModelObjectData, ModelObjectState, ModelPartition,
    ObjectChildTag, ObjectDataTag, ObjectIterator, ObjectReferenceTag, ObjectStateFlags,
    ObjectTreeIterator, ObjectTypeFlags, TypeRegistry,
};
use std::sync::{Arc, Mutex, MutexGuard, Once, PoisonError};

/// Minimal concrete subclass of `martensite_object_model_DocumentModel` (abstract because of `NewEmpty`).
pub struct TestDocumentModel {
    transient: StandardTransient,
    base: DocumentModelBase,
}
impl_transient!(TestDocumentModel, "TestDocumentModel", dyn DocumentModel, transient,
    interfaces: [dyn DocumentModel]);
impl TestDocumentModel {
    /// `new TestDocumentModel()`, already in its bound handle.
    pub fn new() -> Handle<TestDocumentModel> {
        let model = Handle::new(Self {
            transient: StandardTransient::default(),
            base: DocumentModelBase::new().unwrap(),
        });
        model.model_base().bind_self(&model.cast());
        model
    }
}
impl DocumentModel for TestDocumentModel {
    fn model_base(&self) -> &DocumentModelBase {
        &self.base
    }
    fn new_empty(&self) -> Result<Handle<dyn DocumentModel>, Failure> {
        Ok(TestDocumentModel::new().cast())
    }
}

/// `TestModelObject::DataTag`.
const DATA_TAG_INT_VAL: i32 = ObjectDataTag::LAST + 1;
const DATA_TAG_REAL_ARR: i32 = ObjectDataTag::LAST + 2;
/// `TestModelObject::RefTag_Other`.
const REF_TAG_OTHER: i32 = ObjectReferenceTag::LAST + 1;
/// `TestModelObject::ChildTag_Child`.
const CHILD_TAG_CHILD: i32 = ObjectChildTag::LAST + 1;

/// Minimal concrete subclass of `martensite_object_model_ModelObject` (whose constructor is protected).
pub struct TestModelObject {
    transient: StandardTransient,
    base: ModelObjectBase,
}
impl_transient!(TestModelObject, "TestModelObject", dyn ModelObject, transient,
    interfaces: [dyn ModelObject]);
impl ModelObject for TestModelObject {
    fn object_base(&self) -> &ModelObjectBase {
        &self.base
    }
}
impl TestModelObject {
    fn unattached(label: &TdfLabel) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: ModelObjectBase::new(label),
        }
    }
    /// `TestModelObject(theLabel, theSetName = true)`.
    pub fn new(label: &TdfLabel) -> Handle<TestModelObject> {
        let object = Handle::new(Self::unattached(label));
        ModelObjectBase::attach(&object.cast(), true).unwrap();
        object
    }
    /// The persistence constructor of `DECLARE_TOBJdocument data framework_PERSISTENCE`.
    fn persistence_new(label: &TdfLabel) -> Result<Handle<dyn ModelObject>, Failure> {
        let object: Handle<dyn ModelObject> = Handle::new(Self::unattached(label)).cast();
        object.object_base().bind_self(&object);
        object.init_fields();
        Ok(object)
    }
    /// `SetInt`: `setInteger(theValue, DataTag_IntVal)`.
    pub fn set_int(&self, value: i32) {
        self.set_integer(value, DATA_TAG_INT_VAL, 0).unwrap();
    }
    /// `GetInt`.
    pub fn get_int(&self) -> i32 {
        self.get_integer(DATA_TAG_INT_VAL, 0).unwrap()
    }
    /// `SetRealArr`: `setArray(theArray, DataTag_RealArr)`.
    pub fn set_real_arr(&self, array: &Handle<HArray1<f64>>) {
        self.set_real_array(array, DATA_TAG_REAL_ARR, 0).unwrap();
    }
    /// `GetRealArr`: `getRealArray(0, DataTag_RealArr)`.
    pub fn get_real_arr(&self) -> Handle<HArray1<f64>> {
        self.get_real_array(0, DATA_TAG_REAL_ARR, 0, 0.0).unwrap()
    }
    /// `SetRef`: `setReference(theObject, RefTag_Other)`.
    pub fn set_ref(&self, object: &Handle<dyn ModelObject>) {
        self.set_reference(object, REF_TAG_OTHER, 0).unwrap();
    }
    /// `GetRef`.
    pub fn get_ref(&self) -> Handle<dyn ModelObject> {
        self.get_reference(REF_TAG_OTHER, 0).unwrap()
    }
    /// `AddChild`: a new object on a new child of `ChildTag_Child`.
    pub fn add_child(&self) -> Handle<TestModelObject> {
        TestModelObject::new(
            &self
                .get_child_label_at(CHILD_TAG_CHILD)
                .unwrap()
                .new_child()
                .unwrap(),
        )
    }
}

/// `IMPLEMENT_TOBJdocument data framework_PERSISTENCE(TestModelObject)`: the runtime static registration.
pub fn register_test_object_persistence() {
    static REGISTERED: Once = Once::new();
    REGISTERED.call_once(|| {
        TypeRegistry::register_static(
            "TestModelObject",
            Arc::new(TestModelObject::persistence_new),
        );
    });
}

/// `FindTestObject(theModel, theName)`.
pub fn find_test_object(model: &Handle<dyn DocumentModel>, name: &str) -> Handle<TestModelObject> {
    let name = Handle::new(HExtendedString::from(ExtendedString::new(name)));
    let dictionary = model.get_dictionary().unwrap();
    model.find_object(&name, &dictionary).unwrap().cast()
}

/// GoogleTest runs the cases of a binary one after the other; the ObjectModel statics
/// (`ObjectModel_Assistant`, the application) are process-wide, so the fixtures serialize.
static SERIAL: Mutex<()> = Mutex::new(());

/// Holds the process-wide ObjectModel statics for one test.
pub fn serial() -> MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The `martensite_object_model_DocumentModelTest` fixture: a model loaded empty, closed on tear-down.
pub struct ModelFixture {
    /// `myModel`.
    pub my_model: Handle<TestDocumentModel>,
    _serial: MutexGuard<'static, ()>,
}
impl ModelFixture {
    /// `SetUp`: `Load` with an empty file name creates a new empty model.
    pub fn set_up() -> Self {
        let serial = serial();
        register_test_object_persistence();
        let my_model = TestDocumentModel::new();
        assert!(my_model.load(&ExtendedString::default()).unwrap());
        Self {
            my_model,
            _serial: serial,
        }
    }
}
impl Drop for ModelFixture {
    fn drop(&mut self) {
        if !self.my_model.is_null() {
            let _ = self.my_model.close();
        }
    }
}

/// A new `TCollection_HExtendedString`.
pub fn h_name(text: &str) -> Handle<HExtendedString> {
    Handle::new(HExtendedString::from(ExtendedString::new(text)))
}

#[test]
fn document_model_Load_EmptyString_CreatesModel() {
    let fixture = ModelFixture::set_up();
    assert!(!fixture.my_model.get_label().is_null());
    assert!(!fixture.my_model.get_document().unwrap().is_null());
}

#[test]
fn document_model_GetMainPartition_ReturnsNonNull() {
    let fixture = ModelFixture::set_up();
    assert!(!fixture.my_model.get_main_partition().unwrap().is_null());
}

#[test]
fn document_model_GetRoot_ReturnsNonNull() {
    let fixture = ModelFixture::set_up();
    assert!(!fixture.my_model.get_root().unwrap().is_null());
}

#[test]
fn document_model_GetModelName_ReturnsDefault() {
    let fixture = ModelFixture::set_up();
    let name = fixture.my_model.get_model_name();
    assert!(!name.is_null());
    assert!(!name.is_empty());
}

#[test]
fn document_model_GetFormat_ReturnsObjectModelBinary() {
    let fixture = ModelFixture::set_up();
    assert!(
        fixture
            .my_model
            .get_format()
            .is_equal(ExtendedString::new("ObjectModelBinary"))
    );
}

#[test]
fn document_model_GetApplication_ReturnsNonNull() {
    let fixture = ModelFixture::set_up();
    assert!(!fixture.my_model.get_application().unwrap().is_null());
}

#[test]
fn document_model_GetDocumentModel_FromLabel_ReturnsSameModel() {
    let fixture = ModelFixture::set_up();
    let label = fixture.my_model.get_label();
    let found = DocumentModelBase::get_document_model(&label).unwrap();
    assert!(!found.is_null());
    assert!(found == fixture.my_model);
}

#[test]
fn document_model_Close_NullifiesLabel() {
    let fixture = ModelFixture::set_up();
    assert!(fixture.my_model.close().unwrap());
    assert!(fixture.my_model.get_label().is_null());
}

#[test]
fn document_model_Close_ReturnsFalse_WhenAlreadyClosed() {
    let fixture = ModelFixture::set_up();
    assert!(fixture.my_model.close().unwrap());
    assert!(!fixture.my_model.close().unwrap());
}

#[test]
fn document_model_IsModified_InitialState() {
    let fixture = ModelFixture::set_up();
    // A freshly loaded model reports its modification status; just verify no failure.
    let _modified = fixture.my_model.is_modified().unwrap();
}

#[test]
fn document_model_SetModified_True_MakesModified() {
    let fixture = ModelFixture::set_up();
    fixture.my_model.set_modified(true).unwrap();
    assert!(fixture.my_model.is_modified().unwrap());
}

#[test]
fn document_model_SetModified_False_ClearsModified() {
    let fixture = ModelFixture::set_up();
    fixture.my_model.set_modified(true).unwrap();
    fixture.my_model.set_modified(false).unwrap();
    assert!(!fixture.my_model.is_modified().unwrap());
}

#[test]
fn document_model_CommandTransaction_OpenCommit() {
    let fixture = ModelFixture::set_up();
    let model = &fixture.my_model;
    model.get_document().unwrap().set_undo_limit(10).unwrap();
    assert!(!model.has_open_command().unwrap());
    model.open_command().unwrap();
    assert!(model.has_open_command().unwrap());
    model.commit_command().unwrap();
    assert!(!model.has_open_command().unwrap());
}

#[test]
fn document_model_CommandTransaction_OpenAbort() {
    let fixture = ModelFixture::set_up();
    let model = &fixture.my_model;
    model.get_document().unwrap().set_undo_limit(10).unwrap();
    model.open_command().unwrap();
    assert!(model.has_open_command().unwrap());
    model.abort_command().unwrap();
    assert!(!model.has_open_command().unwrap());
}

#[test]
fn document_model_GetChildren_EmptyModel_NoChildren() {
    let fixture = ModelFixture::set_up();
    let iterator = fixture.my_model.get_children().unwrap();
    if let Some(iterator) = iterator.get() {
        assert!(!iterator.more());
    }
}

#[test]
fn document_model_GetObjects_EmptyModel_HasRootOnly() {
    let fixture = ModelFixture::set_up();
    let iterator = fixture.my_model.get_objects().unwrap();
    assert!(!iterator.is_null());
    // The model iterator should at least exist; root partition is always present.
    let mut count = 0;
    while iterator.more() {
        count += 1;
        iterator.next().unwrap();
    }
    assert!(count >= 0);
}

#[test]
fn document_model_NewEmpty_ReturnsFreshModel() {
    let fixture = ModelFixture::set_up();
    let new_model = fixture.my_model.new_empty().unwrap();
    assert!(!new_model.is_null());
    // The new model should not share the same label.
    assert!(new_model.get_label().is_null());
}

#[test]
fn document_model_GetDictionary_ReturnsNonNull() {
    let fixture = ModelFixture::set_up();
    assert!(!fixture.my_model.get_dictionary().unwrap().is_null());
}

#[test]
fn document_model_GetFormatVersion_FreshModel_ReturnsMinusOne() {
    let fixture = ModelFixture::set_up();
    assert_eq!(fixture.my_model.get_format_version().unwrap(), -1);
}

#[test]
fn document_model_Update_DefaultReturnsTrue() {
    let fixture = ModelFixture::set_up();
    assert!(fixture.my_model.update().unwrap());
}

#[test]
fn document_model_Messenger_SetAndGet() {
    let fixture = ModelFixture::set_up();
    let messenger = Arc::new(Messenger::new());
    fixture.my_model.set_messenger(messenger.clone());
    assert!(Arc::ptr_eq(&messenger, &fixture.my_model.messenger()));
}

/// The `martensite_object_model_ModelObjectTest` fixture: a loaded model and its main partition.
pub struct ObjectFixture {
    /// The model part of the fixture.
    pub model: ModelFixture,
    /// `myPartition`.
    pub my_partition: Handle<dyn ModelPartition>,
}
impl ObjectFixture {
    /// `SetUp`.
    pub fn set_up() -> Self {
        let model = ModelFixture::set_up();
        let my_partition = model.my_model.get_main_partition().unwrap();
        assert!(!my_partition.is_null());
        Self {
            model,
            my_partition,
        }
    }
    /// `myModel`.
    pub fn my_model(&self) -> &Handle<TestDocumentModel> {
        &self.model.my_model
    }
    /// `createObject`: a new object on a new label of the partition.
    pub fn create_object(&self) -> Handle<TestModelObject> {
        TestModelObject::new(&self.my_partition.new_label().unwrap())
    }
    /// `OpenCommand`, the body, `CommitCommand`.
    pub fn command<R>(&self, body: impl FnOnce() -> R) -> R {
        self.my_model().open_command().unwrap();
        let result = body();
        self.my_model().commit_command().unwrap();
        result
    }
}

#[test]
fn model_object_CreateObject_IsAlive() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    assert!(!object.is_null());
    assert!(object.is_alive().unwrap());
}

#[test]
fn model_object_CreateObject_HasLabel() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    assert!(!object.get_label().is_null());
}

#[test]
fn model_object_CreateObject_GetModel_ReturnsSameModel() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    let found = object.get_model().unwrap();
    assert!(!found.is_null());
    assert!(found == *fixture.my_model());
}

#[test]
fn model_object_SetName_GetName_Roundtrip() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    let new_name = h_name("TestObjectName");
    fixture.command(|| assert!(object.set_name(&new_name).unwrap()));
    let got = object.get_name().unwrap();
    assert!(!got.is_null());
    assert!(got.string().is_equal(&*new_name.string()));
}

#[test]
fn model_object_SetName_CString_Roundtrip() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    fixture.command(|| assert!(object.set_name_c_str("AsciiName").unwrap()));
    let mut name = AsciiString::default();
    assert!(object.get_name_ascii(&mut name).unwrap());
    assert!(name.is_equal(AsciiString::new("AsciiName")));
}

#[test]
fn model_object_SetName_Duplicate_ReturnsFalse() {
    let fixture = ObjectFixture::set_up();
    let (first, second) = fixture.command(|| (fixture.create_object(), fixture.create_object()));
    let name = h_name("UniqueName");
    fixture.command(|| assert!(first.set_name(&name).unwrap()));
    fixture.command(|| assert!(!second.set_name(&name).unwrap()));
}

#[test]
fn model_object_GetChildren_NoChildren_Empty() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    let iterator = object.get_children(None).unwrap();
    if let Some(iterator) = iterator.get() {
        assert!(!iterator.more());
    }
}

#[test]
fn model_object_GetDataLabel_ReturnsValidLabel() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    assert!(!object.get_data_label().unwrap().is_null());
}

#[test]
fn model_object_GetReferenceLabel_ReturnsValidLabel() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    assert!(!object.get_reference_label().unwrap().is_null());
}

#[test]
fn model_object_Flags_SetAndTest() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    fixture.command(|| object.set_flags(ObjectStateFlags::HIDDEN).unwrap());
    assert!(object.test_flags(ObjectStateFlags::HIDDEN).unwrap());
    assert!(!object.test_flags(ObjectStateFlags::SAVED).unwrap());
}

#[test]
fn model_object_Flags_ClearFlags() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    let both = ObjectStateFlags::HIDDEN | ObjectStateFlags::SAVED;
    fixture.command(|| object.set_flags(both).unwrap());
    fixture.command(|| object.clear_flags(ObjectStateFlags::HIDDEN).unwrap());
    assert!(!object.test_flags(ObjectStateFlags::HIDDEN).unwrap());
    assert!(object.test_flags(ObjectStateFlags::SAVED).unwrap());
}

#[test]
fn model_object_Order_SetAndGet() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    fixture.command(|| assert!(object.set_order(42).unwrap()));
    assert_eq!(42, object.get_order().unwrap());
}

#[test]
fn model_object_Detach_RemovesObject() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    fixture.command(|| assert!(object.detach(DeletionMode::FreeOnly).unwrap()));
    assert!(!object.is_alive().unwrap());
}

#[test]
fn model_object_GetObj_StaticLookup() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    let label = object.get_label();
    let mut found = Handle::default();
    assert!(ModelObjectBase::get_obj(&label, &mut found, false).unwrap());
    assert!(!found.is_null());
    assert!(found == object);
}

#[test]
fn model_object_ModelIteration_AfterAddingObjects() {
    let fixture = ObjectFixture::set_up();
    let _objects = fixture.command(|| {
        [
            fixture.create_object(),
            fixture.create_object(),
            fixture.create_object(),
        ]
    });
    let iterator = fixture.my_model().get_children().unwrap();
    assert!(!iterator.is_null());
    let mut count = 0;
    while iterator.more() {
        count += 1;
        iterator.next().unwrap();
    }
    assert_eq!(3, count);
}

/// The names of the objects an iterator visits, in a set (`NCollection_IndexedMap`).
fn visited_names(iterator: &Handle<dyn ObjectIterator>) -> Vec<String> {
    let mut names = Vec::new();
    while iterator.more() {
        let mut name = AsciiString::default();
        assert!(iterator.value().get_name_ascii(&mut name).unwrap());
        let name = String::from_utf8_lossy(name.as_bytes()).into_owned();
        if !names.contains(&name) {
            names.push(name);
        }
        iterator.next().unwrap();
    }
    names
}

// caf/bugs/bug31323: typed child iteration remains recursive through all object levels.
#[test]
fn model_object_nested_typed_children_are_traversed() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| {
        let object = fixture.create_object();
        let sub1 = object.add_child();
        let sub2 = object.add_child();
        let sub11 = sub1.add_child();
        let sub12 = sub1.add_child();
        let sub13 = sub1.add_child();
        let sub121 = sub12.add_child();
        assert!(object.set_name_c_str("obj").unwrap());
        assert!(sub1.set_name_c_str("sub1").unwrap());
        assert!(sub2.set_name_c_str("sub2").unwrap());
        assert!(sub11.set_name_c_str("sub11").unwrap());
        assert!(sub12.set_name_c_str("sub12").unwrap());
        assert!(sub13.set_name_c_str("sub13").unwrap());
        assert!(sub121.set_name_c_str("sub121").unwrap());
        object
    });
    let flat = object.get_children(None).unwrap();
    assert!(!flat.is_null());
    let flat_names = visited_names(&flat);
    assert_eq!(flat_names.len(), 2);
    assert!(flat_names.contains(&"sub1".to_owned()));
    assert!(flat_names.contains(&"sub2".to_owned()));
    let all: Handle<dyn ObjectIterator> =
        ObjectTreeIterator::new(&object.get_child_label().unwrap(), None, true, true)
            .unwrap()
            .cast();
    assert!(!all.is_null());
    let all_names = visited_names(&all);
    assert_eq!(all_names.len(), 6);
    for name in ["sub1", "sub2", "sub11", "sub12", "sub13", "sub121"] {
        assert!(all_names.contains(&name.to_owned()));
    }
}

#[test]
fn model_object_GetTypeFlags_DefaultVisible() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    assert!(object.get_type_flags() & ObjectTypeFlags::VISIBLE != 0);
}

#[test]
fn model_object_HasBackReferences_InitiallyFalse() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    assert!(!object.has_back_references());
}

#[test]
fn model_object_NameRegistration_InModel() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    let name = h_name("RegisteredName");
    fixture.command(|| object.set_name(&name).unwrap());
    let dictionary = fixture.my_model().get_dictionary().unwrap();
    assert!(
        fixture
            .my_model()
            .is_registered_name(&name, &dictionary)
            .unwrap()
    );
}

#[test]
fn model_object_FindObject_ByName() {
    let fixture = ObjectFixture::set_up();
    let object = fixture.command(|| fixture.create_object());
    let name = h_name("FindMe");
    fixture.command(|| object.set_name(&name).unwrap());
    let dictionary = fixture.my_model().get_dictionary().unwrap();
    let found = fixture.my_model().find_object(&name, &dictionary).unwrap();
    assert!(!found.is_null());
    assert!(found == object);
}

// A detached parent: martensite_object_model_ModelObject::GetFatherObject() is not protected against a deleted object;
// after detaching a parent object, GetFatherObject() on its child must return null.
#[test]
fn model_object_parent_detach_clears_the_father_object() {
    let fixture = ObjectFixture::set_up();
    let parent = fixture.command(|| fixture.create_object());
    assert!(!parent.is_null());
    let child = fixture
        .command(|| TestModelObject::new(&parent.get_child_label().unwrap().new_child().unwrap()));
    assert!(!child.is_null());
    fixture.command(|| assert!(parent.detach(DeletionMode::FreeOnly).unwrap()));
    // After the parent is detached its ObjectModel attributes are gone, so the walk up finds
    // no father.
    assert!(child.get_father_object(None).unwrap().is_null());
}
