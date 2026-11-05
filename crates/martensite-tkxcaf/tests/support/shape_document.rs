//! Owned shape-document fixture, shared without importing a test suite.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    standard::Handle,
    tcollection::{AsciiString, ExtendedString},
};
use martensite_tklcaf::{
    tdata_std::TDataStdName,
    tdf::{TdfLabel, TdfTool},
    tdoc_std::{TDocStdApplication, TDocStdDocument, TDocStdStandardApplication},
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};

pub(crate) struct XdeShapeRegressionContext {
    application: Handle<TDocStdStandardApplication>,
    pub(crate) document: Handle<TDocStdDocument>,
    shapes: Handle<XCAFDocShapeTool>,
}

impl XdeShapeRegressionContext {
    pub(crate) fn new() -> Self {
        assert_default_shape_naming();
        let (application, document) = super::new_document("BinXCAF");
        Self::from_document(application, document)
    }

    /// Reuse the caller's application while opening a fresh shape document.
    /// Example: `XdeShapeRegressionContext::with_application(&application)`.
    pub(crate) fn with_application(application: &Handle<TDocStdStandardApplication>) -> Self {
        assert_default_shape_naming();
        let document = super::new_document_in(application, "BinXCAF");
        Self::from_document(application.clone(), document)
    }

    fn from_document(
        application: Handle<TDocStdStandardApplication>,
        document: Handle<TDocStdDocument>,
    ) -> Self {
        TDataStdName::set(
            &document.main().unwrap().root().unwrap(),
            &ExtendedString::from_utf8("D"),
        )
        .unwrap();
        let shapes = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
        Self {
            application,
            document,
            shapes,
        }
    }

    pub(crate) fn shape_tool(&self) -> &XCAFDocShapeTool {
        self.shapes.get().unwrap()
    }

    pub(crate) fn add_shape(&self, shape: &Shape, make_assembly: bool) -> TdfLabel {
        ensure_document_unit(&self.document);
        self.shape_tool()
            .add_shape(shape, make_assembly, true)
            .unwrap()
    }

    pub(crate) fn close(self) {
        self.application
            .close_tdoc_std_document(&self.document)
            .unwrap();
    }
}

fn assert_default_shape_naming() {
    assert!(
        XCAFDocShapeTool::auto_naming(),
        "fresh document case requires default process naming"
    );
}

#[path = "shape_units.rs"]
mod shape_units;
pub(crate) use shape_units::ensure_shape_document_unit as ensure_document_unit;

pub(crate) fn label_at_entry(context: &XdeShapeRegressionContext, entry: &str) -> TdfLabel {
    let mut label = TdfLabel::default();
    TdfTool::label_from_entry(
        &context.document.get_data(),
        &AsciiString::from(entry),
        &mut label,
        false,
    )
    .unwrap();
    assert!(!label.is_null());
    label
}
