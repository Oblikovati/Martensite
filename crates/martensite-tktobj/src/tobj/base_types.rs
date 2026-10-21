//! Usable immediate parent interfaces for the declared TObj class hierarchy.
use super::{TObjLabelIterator, TObjObject, TObjObjectIterator, TObjPartition};
use martensite_tklcaf::{tdf::TdfAttribute, tdoc_std::TDocStdApplication};

/// The immediate parent interface of `TObjApplication`; use `Handle<TObjApplicationBaseType>` for parent dispatch.
pub type TObjApplicationBaseType = dyn TDocStdApplication;
/// The immediate parent interface of `TObjPartition`; use `Handle<TObjPartitionBaseType>` for parent dispatch.
pub type TObjPartitionBaseType = dyn TObjObject;
/// The immediate parent interface of `TObjHiddenPartition`; use `Handle<TObjHiddenPartitionBaseType>` for parent dispatch.
pub type TObjHiddenPartitionBaseType = dyn TObjPartition;
/// The immediate parent interface of `TObjLabelIterator`; use `Handle<TObjLabelIteratorBaseType>` for parent dispatch.
pub type TObjLabelIteratorBaseType = dyn TObjObjectIterator;
/// The immediate parent interface of `TObjModelIterator`; use `Handle<TObjModelIteratorBaseType>` for parent dispatch.
pub type TObjModelIteratorBaseType = dyn TObjObjectIterator;
/// The immediate parent interface of `TObjSequenceIterator`; use `Handle<TObjSequenceIteratorBaseType>` for parent dispatch.
pub type TObjSequenceIteratorBaseType = dyn TObjObjectIterator;
/// The immediate parent interface of `TObjOcafObjectIterator`; use `Handle<TObjOcafObjectIteratorBaseType>` for parent dispatch.
pub type TObjOcafObjectIteratorBaseType = dyn TObjLabelIterator;
/// The immediate parent interface of `TObjReferenceIterator`; use `Handle<TObjReferenceIteratorBaseType>` for parent dispatch.
pub type TObjReferenceIteratorBaseType = dyn TObjLabelIterator;
/// The immediate parent interface of `TObjTModel`; use `Handle<TObjTModelBaseType>` for parent dispatch.
pub type TObjTModelBaseType = dyn TdfAttribute;
/// The immediate parent interface of `TObjTObject`; use `Handle<TObjTObjectBaseType>` for parent dispatch.
pub type TObjTObjectBaseType = dyn TdfAttribute;
/// The immediate parent interface of `TObjTReference`; use `Handle<TObjTReferenceBaseType>` for parent dispatch.
pub type TObjTReferenceBaseType = dyn TdfAttribute;
/// The immediate parent interface of `TObjTXyz`; use `Handle<TObjTXyzBaseType>` for parent dispatch.
pub type TObjTXyzBaseType = dyn TdfAttribute;
/// The immediate parent interface of `TObjTIntSparseArray`; use `Handle<TObjTIntSparseArrayBaseType>` for parent dispatch.
pub type TObjTIntSparseArrayBaseType = dyn TdfAttribute;
/// The immediate parent interface of `TObjTNameContainer`; use `Handle<TObjTNameContainerBaseType>` for parent dispatch.
pub type TObjTNameContainerBaseType = dyn TdfAttribute;

/// The immediate algorithm base of `TObjCheckModel`; use `Handle<TObjCheckModelBaseType>` for shared status access.
pub type TObjCheckModelBaseType = martensite_tkernel::message::Algorithm;

/// The model's reference-count base; borrow it with `model.transient_base()`.
pub type TObjModelBaseType = martensite_tkernel::standard::StandardTransient;
/// The object's reference-count base; borrow it with `object.transient_base()`.
pub type TObjObjectBaseType = martensite_tkernel::standard::StandardTransient;
/// The iterator's reference-count base; borrow it with `iterator.transient_base()`.
pub type TObjObjectIteratorBaseType = martensite_tkernel::standard::StandardTransient;
