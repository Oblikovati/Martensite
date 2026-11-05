//! The shared shape-addition document-unit fixture.
use martensite_tkernel::{standard::Handle, units_methods::LengthUnit};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::xs_algo::{XSAlgoShapeProcessor, xs_algo_units};

/// Retain an existing unit or initialize the document unit; `ensure_shape_document_unit(&document);`.
pub fn ensure_shape_document_unit(document: &Handle<TDocStdDocument>) {
    let mut unit = 1.;
    if !XCAFDocDocumentTool::get_length_unit(document, &mut unit).unwrap() {
        // Adding a shape to a document that has no unit:
        // use the configured transfer unit before recording the document scale.
        XSAlgoShapeProcessor::prepare_for_transfer();
        unit = xs_algo_units().get_cascade_length_unit(LengthUnit::Meter);
        XCAFDocDocumentTool::set_length_unit(document, unit).unwrap();
    }
}
