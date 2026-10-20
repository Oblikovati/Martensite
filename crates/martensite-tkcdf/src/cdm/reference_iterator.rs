//! `CDM_ReferenceIterator`: the references from one document, in creation order.
use super::{CdmDocument, CdmReference};
use crate::live;
use martensite_tkernel::standard::{Failure, Handle};

/// `CDM_ReferenceIterator`: iterates over a copy of the reference list taken at creation
/// (the native iterator walks the list nodes; both agree while the list does not change).
///
/// ```
/// use martensite_tkcdf::cdm::{CdmDocument, CdmReferenceIterator};
/// # use martensite_tkcdf::cdm::CdmDocumentBase;
/// # use martensite_tkernel::{impl_transient, standard::{Handle, StandardTransient}};
/// # use martensite_tkernel::tcollection::ExtendedString;
/// # #[derive(Default)] struct Doc { transient: StandardTransient, base: CdmDocumentBase }
/// # impl_transient!(Doc, "Doc", dyn CdmDocument, transient, interfaces: [dyn CdmDocument]);
/// # impl CdmDocument for Doc {
/// #     fn document_base(&self) -> &CdmDocumentBase { &self.base }
/// #     fn storage_format(&self) -> ExtendedString { ExtendedString::new("Fmt") }
/// # }
/// let doc: Handle<dyn CdmDocument> = Handle::new(Doc::default()).cast();
/// let iterator = CdmReferenceIterator::new(&doc).unwrap();
/// assert!(!iterator.more());
/// ```
pub struct CdmReferenceIterator {
    references: Vec<Handle<CdmReference>>,
    index: usize,
}
impl CdmReferenceIterator {
    /// `CDM_ReferenceIterator(aDocument)`: `Standard_NullObject` for a null document.
    pub fn new(document: &Handle<dyn CdmDocument>) -> Result<Self, Failure> {
        let document = live(document, "CDM_ReferenceIterator")?;
        Ok(Self {
            references: document.document_base().references_from_this(),
            index: 0,
        })
    }
    /// `More`; `iterator.more()`.
    pub fn more(&self) -> bool {
        self.index < self.references.len()
    }
    /// `Next`; `iterator.next()`.
    pub fn next(&mut self) {
        self.index += 1;
    }
    fn current(&self) -> Result<&CdmReference, Failure> {
        let reference = self.references.get(self.index).ok_or_else(|| {
            martensite_tkernel::standard::exceptions::no_such_object(&format!(
                "CDM_ReferenceIterator: no current reference at index {}; expected More()",
                self.index
            ))
        })?;
        live(reference, "CDM_ReferenceIterator")
    }
    /// `Document`: the referenced document, retrieved if needed.
    pub fn document(&self) -> Result<Handle<dyn CdmDocument>, Failure> {
        self.current()?.to_document()
    }
    /// `ReferenceIdentifier`; `iterator.reference_identifier()?`.
    pub fn reference_identifier(&self) -> Result<i32, Failure> {
        Ok(self.current()?.reference_identifier())
    }
    /// `DocumentVersion`: the referenced version stored in the reference.
    pub fn document_version(&self) -> Result<i32, Failure> {
        Ok(self.current()?.document_version())
    }
}
