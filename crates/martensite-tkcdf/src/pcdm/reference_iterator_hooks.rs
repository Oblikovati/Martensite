//! The six virtual reference-provider callbacks, including the private virtual methods.
use super::PcdmReferenceIterator;
use crate::cdm::{CdmMetaData, MetaDataLookUpTable};
use martensite_tkernel::standard::{Failure, Handle};

/// Supplies reference records independently of the default native-container reader.
///
/// Each callback can delegate to [`PcdmDefaultReferenceIteratorHooks`]. The owning
/// iterator invokes initialization once, then More/MetaData/identifier/version/Next
/// for each reference. A failing callback stops the traversal before Next.
///
/// ```
/// use martensite_tkcdf::pcdm::{PcdmReferenceIterator, PcdmReferenceIteratorHooks};
/// struct EmptyReferences;
/// impl PcdmReferenceIteratorHooks for EmptyReferences {
///     fn more(&self, _: &PcdmReferenceIterator) -> bool { false }
/// }
/// ```
pub trait PcdmReferenceIteratorHooks: Send + Sync {
    /// Initializes a provider from document metadata. Example: `hooks.init(&cursor, &metadata)?`.
    fn init(
        &self,
        iterator: &PcdmReferenceIterator,
        meta_data: &Handle<CdmMetaData>,
    ) -> Result<(), Failure> {
        iterator.default_init(meta_data)
    }
    /// Tests whether a record remains. Example: `hooks.more(&cursor)`.
    fn more(&self, iterator: &PcdmReferenceIterator) -> bool {
        iterator.default_more()
    }
    /// Advances after successful reference creation. Example: `hooks.next(&cursor)`.
    fn next(&self, iterator: &PcdmReferenceIterator) {
        iterator.default_next();
    }
    /// Resolves the current target metadata through the application's lookup table.
    /// Example: `hooks.meta_data(&cursor, &mut table, true)?`.
    fn meta_data(
        &self,
        iterator: &PcdmReferenceIterator,
        table: &mut MetaDataLookUpTable,
        use_storage_configuration: bool,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        iterator.default_meta_data(table, use_storage_configuration)
    }
    /// Reads the stored reference identifier. Example: `hooks.reference_identifier(&cursor)?`.
    fn reference_identifier(&self, iterator: &PcdmReferenceIterator) -> Result<i32, Failure> {
        iterator.default_reference_identifier()
    }
    /// Reads the stored target version. Example: `hooks.document_version(&cursor)?`.
    fn document_version(&self, iterator: &PcdmReferenceIterator) -> Result<i32, Failure> {
        iterator.default_document_version()
    }
}

/// Explicit native-header base callbacks for providers that override only selected methods.
///
/// ```
/// use martensite_tkcdf::pcdm::PcdmDefaultReferenceIteratorHooks;
/// let defaults = PcdmDefaultReferenceIteratorHooks;
/// ```
#[derive(Default)]
pub struct PcdmDefaultReferenceIteratorHooks;
impl PcdmReferenceIteratorHooks for PcdmDefaultReferenceIteratorHooks {}
