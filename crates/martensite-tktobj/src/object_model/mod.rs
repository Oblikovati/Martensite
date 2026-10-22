//! Typed document objects, their partitions, references, and attributes.
mod application;
mod application_errors;
mod assistant;
mod base_types;
mod canonical_runtime_type;
mod check_model;
mod hidden_partition;
mod label_iterator;
mod messages;
mod model;
mod model_init;
mod model_io;
mod model_iterator;
mod model_state;
mod object;
mod object_arrays;
mod object_children;
mod object_clone;
mod object_data;
mod object_detach;
mod object_iterator;
mod object_names;
mod object_references;
mod object_state;
mod object_tree_iterator;
mod partition;
mod persistence;
mod reference_iterator;
mod sequence_iterator;
mod t_int_sparse_array;
mod t_model;
mod t_name_container;
mod t_object;
mod t_reference;
mod t_xyz;
pub use application::{DocumentApplication, DocumentApplicationApi, DocumentApplicationBase};
pub use assistant::ModelAssistant;
pub use base_types::{
    CoordinateAttributeParent, DocumentApplicationParent, DocumentModelParent,
    HiddenPartitionParent, IntegerSparseArrayParent, LabelIteratorParent, ModelAttributeParent,
    ModelIntegrityCheckParent, ModelIteratorParent, ModelObjectParent, ModelPartitionParent,
    NameIndexParent, ObjectAttributeParent, ObjectIteratorParent, ObjectTreeIteratorParent,
    ReferenceAttributeParent, ReferenceIteratorParent, TypedSequenceIteratorParent,
};
pub use check_model::ModelIntegrityCheck;
pub use hidden_partition::HiddenPartition;
pub use label_iterator::{
    LabelIterator, LabelIteratorBase, label_iterator_more, label_iterator_next,
};
pub use model::{DocumentModel, DocumentModelBase, ModelDataTag};
pub use model_iterator::ModelIterator;
pub use model_state::DocumentModelState;
pub use object::{
    BackReferences, ModelObject, ModelObjectBase, ObjectChildTag, ObjectDataTag,
    ObjectReferenceTag, ObjectStateFlags, ObjectTypeFlags,
};
pub use object_arrays::{ArrayAttribute, ModelObjectArrays};
pub use object_clone::copy_label_references;
pub use object_data::ModelObjectData;
pub use object_iterator::{ObjectIterator, StandardObjectIterator};
pub use object_state::ModelObjectState;
pub use object_tree_iterator::ObjectTreeIterator;
pub use partition::{
    ModelPartition, ModelPartitionBase, PartitionDataTag, StandardPartition, partition_copy_data,
    partition_set_name,
};
pub use persistence::{TypeRegistry, TypeRegistryFactory, TypeRegistryMap};
pub use reference_iterator::ReferenceIterator;
pub use sequence_iterator::TypedSequenceIterator;
pub use t_int_sparse_array::IntegerSparseArray;
pub use t_model::ModelAttribute;
pub use t_name_container::{NameIndex, NameIndexMap};
pub use t_object::ObjectAttribute;
pub use t_reference::ReferenceAttribute;
pub use t_xyz::CoordinateAttribute;

/// Controls how [`ModelObject::detach`] treats objects that depend on a removed object.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DeletionMode {
    /// Remove only objects with no dependents.
    #[default]
    FreeOnly = 0,
    /// Remove the object when its dependents can discard the reference.
    KeepDepending,
    /// Remove this object and all of its dependents.
    Forced,
}
