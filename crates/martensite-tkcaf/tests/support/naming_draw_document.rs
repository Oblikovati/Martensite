//! Actual caf begin document, case Close/NewDocument, and isolated application cleanup.
use super::naming_draw_presentation::NamingPresentation;
use martensite_tkernel::{standard::Handle, tcollection::ExtendedString};
use martensite_tklcaf::{
    tdata_std::TDataStdName,
    tdoc_std::{TDocStdApplication, TDocStdDocument, TDocStdStandardApplication},
};
/// Own the actual document through the case; `NamingDocumentCase::new()`.
/// Optional Rc presentation makes this fixture deliberately thread-confined.
pub(crate) struct NamingDocumentCase {
    application: Handle<TDocStdStandardApplication>,
    pub document: Handle<TDocStdDocument>,
    pub(super) presentation: NamingPresentation,
}
fn named_document(application: &Handle<TDocStdStandardApplication>) -> Handle<TDocStdDocument> {
    let mut document = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::new("BinOcaf"), &mut document)
        .unwrap();
    TDataStdName::set(
        &document.main().unwrap().root().unwrap(),
        &ExtendedString::new("D"),
    )
    .unwrap();
    document
}
impl NamingDocumentCase {
    /// The begin UndoLimit100 applies to the document closed by the case; `NamingDocumentCase::new()`.
    pub fn new() -> Self {
        let application = TDocStdStandardApplication::new();
        let begin = named_document(&application);
        begin.set_undo_limit(100).unwrap();
        begin.new_command().unwrap();
        assert!(begin.has_open_command());
        application.close_tdoc_std_document(&begin).unwrap();
        assert_eq!(application.nb_documents().unwrap(), 0);
        drop(begin);
        let document = named_document(&application);
        assert_eq!(document.get_undo_limit(), 0);
        assert_eq!(application.nb_documents().unwrap(), 1);
        Self {
            application,
            document,
            presentation: NamingPresentation::default(),
        }
    }
    fn single_document(undo_limit: i32) -> Self {
        let lifecycle = Self::prepare_driver();
        lifecycle.initialize_document(undo_limit);
        lifecycle
    }
    /// Create the caf/begin document before its process limit; `NamingDocumentCase::prepare_driver()`.
    pub fn prepare_driver() -> Self {
        let application = TDocStdStandardApplication::new();
        let document = named_document(&application);
        Self {
            application,
            document,
            presentation: NamingPresentation::default(),
        }
    }
    /// Enable the complete original's live source presentation; `case.with_presentation()`.
    pub(crate) fn with_presentation(mut self) -> Self {
        self.presentation = NamingPresentation::connected().unwrap();
        self
    }
    fn initialize_document(&self, undo_limit: i32) {
        self.document.set_undo_limit(undo_limit).unwrap();
        if undo_limit > 0 {
            self.document.new_command().unwrap();
        }
        assert_eq!(self.document.has_open_command(), undo_limit > 0);
        assert_eq!(self.application.nb_documents().unwrap(), 1);
    }
    /// Apply caf/begin state after its process limit; `prepared.begin_driver()`.
    pub fn begin_driver(self) -> Self {
        self.initialize_document(100);
        self
    }
    /// Retains the caf/begin document and its live undo command; `NamingDocumentCase::driver()`.
    pub fn driver() -> Self {
        Self::prepare_driver().begin_driver()
    }
    /// Creates the document directly, without a caf/begin replacement; `NamingDocumentCase::standalone()`.
    pub fn standalone() -> Self {
        Self::single_document(0)
    }
    /// Drop isolated-process document state after all original case assertions; `case.close()`.
    pub fn close(&self) {
        self.presentation.end();
        // Driver scripts retain their final empty command; cleanup follows all source checks.
        if self.document.get_undo_limit() == 100 {
            assert!(self.document.has_open_command());
            self.document.abort_command().unwrap();
        }
        assert!(!self.document.has_open_command());
        self.application
            .close_tdoc_std_document(&self.document)
            .unwrap();
        assert_eq!(self.application.nb_documents().unwrap(), 0);
    }
}
