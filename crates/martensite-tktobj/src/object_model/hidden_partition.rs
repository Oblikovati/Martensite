//! `martensite_object_model_HiddenPartition`: a partition whose type flags lack `Visible`.
use super::{ModelObject, ModelObjectBase, ModelPartition, ModelPartitionBase, ObjectTypeFlags};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
};
use martensite_tklcaf::tdf::TdfLabel;

/// `martensite_object_model_HiddenPartition`; `HiddenPartition::new(&label)`.
pub struct HiddenPartition {
    transient: StandardTransient,
    object: ModelObjectBase,
    partition: ModelPartitionBase,
}
impl_transient!(HiddenPartition, "martensite_object_model_HiddenPartition", dyn ModelPartition, transient,
    interfaces: [dyn ModelObject, dyn ModelPartition]);

impl HiddenPartition {
    /// `martensite_object_model_HiddenPartition(theLabel)`: a partition attached and named.
    pub fn new(label: &TdfLabel) -> Result<Handle<HiddenPartition>, Failure> {
        let partition = Handle::new(Self::unattached(label));
        ModelObjectBase::attach(&partition.cast(), true)?;
        Ok(partition)
    }
    /// The persistence constructor (`DECLARE_TOBJdocument data framework_PERSISTENCE`): bound, not attached.
    pub fn persistence_new(label: &TdfLabel) -> Result<Handle<dyn ModelObject>, Failure> {
        let partition: Handle<dyn ModelObject> = Handle::new(Self::unattached(label)).cast();
        let object = martensite_tkcdf::live(
            &partition,
            "martensite_object_model_HiddenPartition::Persistence_::New",
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

impl ModelObject for HiddenPartition {
    fn object_base(&self) -> &ModelObjectBase {
        &self.object
    }
    crate::object_model_partition_object_members!();
    /// `GetTypeFlags`: the partition's flags without `Visible`.
    fn get_type_flags(&self) -> i32 {
        ObjectTypeFlags::VISIBLE & !ObjectTypeFlags::VISIBLE
    }
}
impl ModelPartition for HiddenPartition {
    fn partition_base(&self) -> &ModelPartitionBase {
        &self.partition
    }
}
