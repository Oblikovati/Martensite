//! The literal label and standard driver reproduce the original document commands.
use super::support;
use super::support::document_viewer::{ViewerLifecycle, register_vulkan_factory};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNamedShape};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::{
    tdata_std::TDataStdName,
    tdoc_std::{TDocStdApplication, TDocStdDocument, TDocStdStandardApplication},
};
use martensite_tkvcaf::tprs_std::{TPrsStdAISPresentation, TPrsStdAISViewer};
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;

pub struct DocumentSettingsFixture {
    application: Handle<TDocStdStandardApplication>,
    pub document: Handle<TDocStdDocument>,
}
impl DocumentSettingsFixture {
    fn new() -> Self {
        let (application, document) = support::new_document("BinXCAF");
        TDataStdName::set(&document.get_data().root(), &ExtendedString::from_utf8("D")).unwrap();
        Self {
            application,
            document,
        }
    }
    fn close(self) {
        self.application
            .close_tdoc_std_document(&self.document)
            .unwrap();
    }
}

const DOCUMENT_VIEW: &str = "Driver1/Document_D/View1";
pub struct PresentationSettingsScene {
    pub fixture: DocumentSettingsFixture,
    pub label: TdfLabel,
    pub presentation: Handle<TPrsStdAISPresentation>,
    pub lifecycle: ViewerLifecycle,
}
impl PresentationSettingsScene {
    pub fn new() -> Self {
        let fixture = DocumentSettingsFixture::new();
        let label = case_named_box(&fixture);
        let lifecycle = initialized_viewer(&label);
        let presentation = displayed_standard_presentation(&lifecycle, &label);
        Self {
            fixture,
            label,
            presentation,
            lifecycle,
        }
    }
    pub fn check_transparency_and_repaint(&self) {
        assert_eq!(self.presentation.transparency().unwrap(), 0.);
        // The existing NS presentation is retained; XSetTransparency creates no XCAF driver here.
        let tool = XCAFDocDocumentTool::shape_tool(&self.fixture.document.main().unwrap()).unwrap();
        assert!(!tool.is_null());
        assert!(martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool::is_shape(&self.label).unwrap());
        let retained = self.presentation.get_ais();
        self.presentation.set_transparency(0.9).unwrap();
        assert_eq!(self.presentation.get_ais(), retained);
        TPrsStdAISViewer::update_label(&self.label.root().unwrap()).unwrap();
        assert_eq!(self.presentation.transparency().unwrap(), 0.9);
        TPrsStdAISViewer::update_label(&self.label.root().unwrap()).unwrap();
    }
    pub fn close(self) {
        self.lifecycle.remove(DOCUMENT_VIEW, true).unwrap();
        self.lifecycle
            .remove("Driver1/Document_D/View2", true)
            .unwrap();
        self.fixture.close();
        assert!(self.lifecycle.state().views.borrow().is_empty());
        assert!(self.lifecycle.state().contexts.borrow().is_empty());
    }
}

fn case_named_box(fixture: &DocumentSettingsFixture) -> TdfLabel {
    fixture.document.set_undo_limit(100).unwrap();
    fixture.document.new_command().unwrap();
    let label = fixture
        .document
        .get_data()
        .root()
        .find_child(2, true)
        .unwrap();
    TNamingBuilder::new(&label)
        .unwrap()
        .generated(&support::make_box(100., 200., 300.))
        .unwrap();
    label
}
fn initialized_viewer(label: &TdfLabel) -> ViewerLifecycle {
    register_vulkan_factory();
    let lifecycle = ViewerLifecycle::new(true);
    lifecycle.initialize(DOCUMENT_VIEW).unwrap();
    TPrsStdAISViewer::new_with_context(&label.root().unwrap(), &lifecycle.interactive_context())
        .unwrap();
    lifecycle
}

fn displayed_standard_presentation(
    lifecycle: &ViewerLifecycle,
    label: &TdfLabel,
) -> Handle<TPrsStdAISPresentation> {
    let presentation = TPrsStdAISPresentation::set(label, &TNamingNamedShape::get_id()).unwrap();
    lifecycle
        .interactive_context()
        .set_default_display_mode(1, true)
        .unwrap();
    presentation.display(true).unwrap();
    TPrsStdAISViewer::update_label(label).unwrap();
    presentation
}
