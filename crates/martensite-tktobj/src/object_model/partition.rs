//! `martensite_object_model_ModelPartition`: an object that holds a set of homogeneous objects of a model, with
//! a name prefix to name new objects.
use super::object_names::same_name;
use super::{DocumentModelState, ModelObject, ModelObjectBase, ModelObjectData, ModelObjectState};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::{ExtendedString, HExtendedString},
};
use martensite_tklcaf::{
    tdata_std::TDataStdName,
    tdf::{TdfLabel, TdfTagSource},
};

/// `martensite_object_model_ModelPartition::DataTag`: the ranks of the partition's data labels.
pub struct PartitionDataTag;
impl PartitionDataTag {
    /// `DataTag_First`: `martensite_object_model_ModelObject::DataTag_Last`.
    pub const FIRST: i32 = super::ObjectDataTag::LAST;
    /// `DataTag_LastIndex`: the last index used to name objects.
    pub const LAST_INDEX: i32 = Self::FIRST + 1;
    /// `DataTag_Last`: reserved up to `DataTag_First + 100`.
    pub const LAST: i32 = Self::FIRST + 100;
}

/// The private field `myPrefix` of `martensite_object_model_ModelPartition`.
#[derive(Default)]
pub struct ModelPartitionBase {
    prefix: SharedState<Handle<HExtendedString>>,
}

/// `martensite_object_model_ModelPartition`'s members over the embedded base; the `martensite_object_model_ModelObject` overrides of a
/// partition are filled by [`crate::object_model_partition_object_members`].
pub trait ModelPartition: ModelObject {
    /// The embedded base with the runtime private field.
    fn partition_base(&self) -> &ModelPartitionBase;
    /// `Update` (virtual): does nothing in a partition.
    fn update(&self) -> bool {
        true
    }
    /// `NewLabel`: a new child of the child label (`TDF_TagSource::NewChild`).
    fn new_label(&self) -> Result<TdfLabel, Failure> {
        TdfTagSource::new_child_of(&self.get_child_label()?)
    }
    /// `SetNamePrefix(thePrefix)`.
    fn set_name_prefix(&self, prefix: &Handle<HExtendedString>) {
        *self.partition_base().prefix.write() = prefix.clone();
    }
    /// `GetNamePrefix`.
    fn get_name_prefix(&self) -> Handle<HExtendedString> {
        self.partition_base().prefix.read().clone()
    }
    /// `GetNewName(theIsToChangeCount)`: the prefix followed by the first free index after
    /// the last one; the last index moves only past names actually skipped. Null without
    /// prefix.
    fn get_new_name(&self, is_to_change_count: bool) -> Result<Handle<HExtendedString>, Failure> {
        let prefix = self.get_name_prefix();
        let Some(prefix) = prefix.get() else {
            return Ok(Handle::default());
        };
        let saved_rank = self.get_last_index()? + 1;
        let mut rank = saved_rank;
        let model = self.get_model()?;
        let model =
            martensite_tkcdf::live(&model, "martensite_object_model_ModelPartition::GetNewName")?;
        let dictionary = self.get_dictionary()?;
        let name = loop {
            let text = prefix.string().cat(ExtendedString::from_integer(rank));
            rank += 1;
            let name = Handle::new(HExtendedString::from(text));
            if !model.is_registered_name(&name, &dictionary)? {
                break name;
            }
        };
        rank -= 1;
        if is_to_change_count && rank > saved_rank {
            self.set_last_index(rank)?;
        }
        Ok(name)
    }
    /// `GetLastIndex`.
    fn get_last_index(&self) -> Result<i32, Failure> {
        self.get_integer(PartitionDataTag::LAST_INDEX, 0)
    }
    /// `SetLastIndex(theIndex)`.
    fn set_last_index(&self, index: i32) -> Result<(), Failure> {
        self.set_integer(index, PartitionDataTag::LAST_INDEX, 0)
            .map(|_| ())
    }
}
impl_transient_interface!(
    dyn ModelPartition,
    "martensite_object_model_ModelPartition",
    dyn ModelObject
);

impl StandardPartition {
    /// `Create(theLabel, theSetName)` (static): a new partition on the label, with the
    /// last index 0.
    pub fn create(label: &TdfLabel, set_name: bool) -> Result<Handle<dyn ModelPartition>, Failure> {
        let partition = StandardPartition::new(label, set_name)?;
        partition.set_last_index(0)?;
        Ok(partition.cast())
    }
    /// `GetPartition(theObject)` (static): the nearest partition above the object, null
    /// when none.
    pub fn get_partition(
        object: &Handle<dyn ModelObject>,
    ) -> Result<Handle<dyn ModelPartition>, Failure> {
        let Some(object) = object.get() else {
            return Ok(Handle::default());
        };
        let mut label = object.get_label().father();
        while !label.is_null() {
            let mut found = Handle::default();
            if ModelObjectBase::get_obj(&label, &mut found, true)? {
                let partition: Handle<dyn ModelPartition> = found.cast();
                if !partition.is_null() {
                    return Ok(partition);
                }
            }
            label = label.father();
        }
        Ok(Handle::default())
    }
}

/// The body of `martensite_object_model_ModelPartition::SetName`: the name is set without registration (a
/// partition does not check the uniqueness of its name).
pub fn partition_set_name<T: ModelPartition + ?Sized>(
    partition: &T,
    name: &Handle<HExtendedString>,
) -> Result<bool, Failure> {
    if same_name(&partition.get_name()?, name)? {
        return Ok(true);
    }
    let text = martensite_tkcdf::live(name, "martensite_object_model_ModelPartition::SetName")?;
    TDataStdName::set(&partition.get_label(), &text.string())?;
    Ok(true)
}

/// The body of `martensite_object_model_ModelPartition::copyData`: the object data, then the prefix, when the
/// target is a partition.
pub fn partition_copy_data<T: ModelPartition + ?Sized>(
    partition: &T,
    target: &Handle<dyn ModelObject>,
) -> Result<bool, Failure> {
    let target_partition: Handle<dyn ModelPartition> = target.cast();
    let Some(target_partition) = target_partition.get() else {
        return Ok(false);
    };
    if !super::object_clone::object_copy_data(partition, target)? {
        return Ok(false);
    }
    target_partition.set_name_prefix(&partition.get_name_prefix());
    Ok(true)
}

/// Fills the `martensite_object_model_ModelObject` overrides of `martensite_object_model_ModelPartition` (`SetName`, `AfterRetrieval`,
/// `copyData`) in the `ModelObject` impl of a partition.
#[macro_export]
macro_rules! object_model_partition_object_members {
    () => {
        fn set_name(
            &self,
            name: &martensite_tkernel::standard::Handle<
                martensite_tkernel::tcollection::HExtendedString,
            >,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            $crate::object_model::partition_set_name(self, name)
        }
        fn after_retrieval(&self) -> Result<(), martensite_tkernel::standard::Failure> {
            Ok(())
        }
        fn copy_data(
            &self,
            target: &martensite_tkernel::standard::Handle<dyn $crate::object_model::ModelObject>,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            $crate::object_model::partition_copy_data(self, target)
        }
    };
}

/// `martensite_object_model_ModelPartition` itself; `StandardPartition::create(&label, true)`.
pub struct StandardPartition {
    transient: StandardTransient,
    object: ModelObjectBase,
    partition: ModelPartitionBase,
}
super::canonical_runtime_type::canonical_object_model_type!(StandardPartition, dyn ModelPartition, transient, [dyn ModelObject, dyn ModelPartition]);

impl StandardPartition {
    /// `martensite_object_model_ModelPartition(theLabel, theSetName)` (protected by the model): a partition attached
    /// to the label.
    pub fn new(label: &TdfLabel, set_name: bool) -> Result<Handle<StandardPartition>, Failure> {
        let partition = Handle::new(Self::unattached(label));
        ModelObjectBase::attach(&partition.cast(), set_name)?;
        Ok(partition)
    }
    /// The persistence constructor (`DECLARE_TOBJdocument data framework_PERSISTENCE`): bound, not attached.
    pub fn persistence_new(label: &TdfLabel) -> Result<Handle<dyn ModelObject>, Failure> {
        let partition: Handle<dyn ModelObject> = Handle::new(Self::unattached(label)).cast();
        let object = martensite_tkcdf::live(
            &partition,
            "martensite_object_model_ModelPartition::Persistence_::New",
        )?;
        object.object_base().bind_self(&partition);
        object.init_fields();
        Ok(partition)
    }
    fn unattached(label: &TdfLabel) -> Self {
        Self {
            transient: StandardTransient::default(),
            object: ModelObjectBase::new(label),
            partition: ModelPartitionBase::default(),
        }
    }
}

impl ModelObject for StandardPartition {
    fn object_base(&self) -> &ModelObjectBase {
        &self.object
    }
    crate::object_model_partition_object_members!();
}
impl ModelPartition for StandardPartition {
    fn partition_base(&self) -> &ModelPartitionBase {
        &self.partition
    }
}
