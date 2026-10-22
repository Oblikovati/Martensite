//! Usable immediate parent interfaces for the declared ObjectModel class hierarchy.
use super::{LabelIterator, ModelObject, ModelPartition, ObjectIterator};
use martensite_tklcaf::{tdf::TdfAttribute, tdoc_std::TDocStdApplication};

/// The immediate parent interface of `DocumentApplication`; use `Handle<DocumentApplicationParent>` for parent dispatch.
pub type DocumentApplicationParent = dyn TDocStdApplication;
/// The immediate parent interface of `ModelPartition`; use `Handle<ModelPartitionParent>` for parent dispatch.
pub type ModelPartitionParent = dyn ModelObject;
/// The immediate parent interface of `HiddenPartition`; use `Handle<HiddenPartitionParent>` for parent dispatch.
pub type HiddenPartitionParent = dyn ModelPartition;
/// The immediate parent interface of `LabelIterator`; use `Handle<LabelIteratorParent>` for parent dispatch.
pub type LabelIteratorParent = dyn ObjectIterator;
/// The immediate parent interface of `ModelIterator`; use `Handle<ModelIteratorParent>` for parent dispatch.
pub type ModelIteratorParent = dyn ObjectIterator;
/// The immediate parent interface of `TypedSequenceIterator`; use `Handle<TypedSequenceIteratorParent>` for parent dispatch.
pub type TypedSequenceIteratorParent = dyn ObjectIterator;
/// The immediate parent interface of `ObjectTreeIterator`; use `Handle<ObjectTreeIteratorParent>` for parent dispatch.
pub type ObjectTreeIteratorParent = dyn LabelIterator;
/// The immediate parent interface of `ReferenceIterator`; use `Handle<ReferenceIteratorParent>` for parent dispatch.
pub type ReferenceIteratorParent = dyn LabelIterator;
/// The immediate parent interface of `ModelAttribute`; use `Handle<ModelAttributeParent>` for parent dispatch.
pub type ModelAttributeParent = dyn TdfAttribute;
/// The immediate parent interface of `ObjectAttribute`; use `Handle<ObjectAttributeParent>` for parent dispatch.
pub type ObjectAttributeParent = dyn TdfAttribute;
/// The immediate parent interface of `ReferenceAttribute`; use `Handle<ReferenceAttributeParent>` for parent dispatch.
pub type ReferenceAttributeParent = dyn TdfAttribute;
/// The immediate parent interface of `CoordinateAttribute`; use `Handle<CoordinateAttributeParent>` for parent dispatch.
pub type CoordinateAttributeParent = dyn TdfAttribute;
/// The immediate parent interface of `IntegerSparseArray`; use `Handle<IntegerSparseArrayParent>` for parent dispatch.
pub type IntegerSparseArrayParent = dyn TdfAttribute;
/// The immediate parent interface of `NameIndex`; use `Handle<NameIndexParent>` for parent dispatch.
pub type NameIndexParent = dyn TdfAttribute;

/// The immediate algorithm base of `ModelIntegrityCheck`; use `Handle<ModelIntegrityCheckParent>` for shared status access.
pub type ModelIntegrityCheckParent = martensite_tkernel::message::Algorithm;

/// The model's reference-count base; borrow it with `model.transient_base()`.
pub type DocumentModelParent = martensite_tkernel::standard::StandardTransient;
/// The object's reference-count base; borrow it with `object.transient_base()`.
pub type ModelObjectParent = martensite_tkernel::standard::StandardTransient;
/// The iterator's reference-count base; borrow it with `iterator.transient_base()`.
pub type ObjectIteratorParent = martensite_tkernel::standard::StandardTransient;
