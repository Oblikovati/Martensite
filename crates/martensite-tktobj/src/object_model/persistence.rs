//! `ObjectModel_Persistence`: the process-wide registry that creates correctly typed objects
//! from their type names (object-model.md section 4).
use super::{HiddenPartition, ModelObject, StandardPartition};
use martensite_tkernel::{
    ncollection::DataMap,
    standard::{Failure, Handle},
    tcollection::AsciiString,
};
use martensite_tklcaf::tdf::TdfLabel;
use std::{
    io,
    sync::{Arc, LazyLock, Mutex, MutexGuard, PoisonError},
};

/// `ObjectModel_Persistence::New` (pure virtual): creates an object of the registered type on
/// a label with the persistence constructor (not attached, `initFields` called).
pub trait TypeRegistryFactory: Send + Sync {
    /// `New(theLabel)`.
    fn new_object(&self, label: &TdfLabel) -> Result<Handle<dyn ModelObject>, Failure>;
}
impl<F> TypeRegistryFactory for F
where
    F: Fn(&TdfLabel) -> Result<Handle<dyn ModelObject>, Failure> + Send + Sync,
{
    fn new_object(&self, label: &TdfLabel) -> Result<Handle<dyn ModelObject>, Failure> {
        self(label)
    }
}

/// The registry map (`getMapOfTypes`), keyed by type name.
type TypeMap = DataMap<AsciiString, Arc<dyn TypeRegistryFactory>>;

/// `getMapOfTypes`: the runtime static map, with the types of this toolkit registered
/// when it is first used (their runtime static `myPersistence_` members).
static MAP_OF_TYPES: LazyLock<Mutex<TypeMap>> = LazyLock::new(|| {
    let mut map = TypeMap::default();
    let partition: Arc<dyn TypeRegistryFactory> = Arc::new(StandardPartition::persistence_new);
    let hidden: Arc<dyn TypeRegistryFactory> = Arc::new(HiddenPartition::persistence_new);
    map.bind(
        AsciiString::new("martensite_object_model_ModelPartition"),
        partition,
    );
    map.bind(
        AsciiString::new("martensite_object_model_HiddenPartition"),
        hidden,
    );
    Mutex::new(map)
});

fn map_of_types() -> MutexGuard<'static, TypeMap> {
    MAP_OF_TYPES.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The registry map type of `getMapOfTypes`: the tools by type name.
pub type TypeRegistryMap = TypeMap;

/// `ObjectModel_Persistence`: a registration of one type; the constructor registers the tool
/// and `Drop` (the runtime destructor) unregisters the type name.
pub struct TypeRegistry {
    type_name: AsciiString,
}

impl TypeRegistry {
    /// `ObjectModel_Persistence(theType)`: registers the tool under the type name.
    ///
    /// ```
    /// use martensite_tklcaf::tdf::TdfLabel;
    /// use martensite_tktobj::object_model::{ModelObject, TypeRegistry};
    /// use martensite_tkernel::standard::{Failure, Handle};
    /// fn none(_: &TdfLabel) -> Result<Handle<dyn ModelObject>, Failure> { Ok(Handle::default()) }
    /// let registration = TypeRegistry::new("Doc_None", std::sync::Arc::new(none));
    /// let mut names = Vec::new();
    /// TypeRegistry::dump_types(&mut names).unwrap();
    /// assert!(String::from_utf8(names).unwrap().contains("Doc_None\n"));
    /// drop(registration);
    /// ```
    pub fn new(type_name: &str, tool: Arc<dyn TypeRegistryFactory>) -> Self {
        let type_name = AsciiString::new(type_name);
        map_of_types().bind(type_name.clone(), tool);
        Self { type_name }
    }
    /// A registration that lives as long as the process, as the runtime static
    /// `myPersistence_` members of `IMPLEMENT_TOBJdocument data framework_PERSISTENCE`.
    pub fn register_static(type_name: &str, tool: Arc<dyn TypeRegistryFactory>) {
        map_of_types().bind(AsciiString::new(type_name), tool);
    }
    /// `getMapOfTypes` (protected): the locked registry map; the guard must be dropped
    /// before a tool is called.
    pub fn map_of_types() -> MutexGuard<'static, TypeRegistryMap> {
        map_of_types()
    }
    /// `CreateNewObject(theType, theLabel)`: an object of the registered type, null when
    /// the type is not registered.
    pub fn create_new_object(
        type_name: &str,
        label: &TdfLabel,
    ) -> Result<Handle<dyn ModelObject>, Failure> {
        let tool = map_of_types().seek(&AsciiString::new(type_name)).cloned();
        match tool {
            Some(tool) => tool.new_object(label),
            None => Ok(Handle::default()),
        }
    }
    /// `DumpTypes(theOs)`: each registered type name and a newline, in map order.
    pub fn dump_types(stream: &mut dyn io::Write) -> io::Result<()> {
        let names: Vec<AsciiString> = map_of_types()
            .items()
            .map(|(name, _)| name.clone())
            .collect();
        for name in names {
            stream.write_all(name.as_bytes())?;
            stream.write_all(b"\n")?;
        }
        Ok(())
    }
}

impl Drop for TypeRegistry {
    /// `~ObjectModel_Persistence`: unregisters the type name.
    fn drop(&mut self) {
        map_of_types().unbind(&self.type_name);
    }
}
