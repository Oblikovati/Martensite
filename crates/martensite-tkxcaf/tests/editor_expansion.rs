//! Compound expansion changes process-wide naming temporarily and runs alone.
mod support;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocEditor, XCAFDocShapeTool};
use support::assembly_fixture::two_boxes;

#[test]
fn editor_expands_a_compound() {
    let (_application, document) = support::new_document("BinXCAF");
    let (compound, _) = two_boxes();
    let shapes = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
    let part = shapes
        .get()
        .unwrap()
        .add_shape(&compound, false, true)
        .unwrap();
    assert!(!XCAFDocShapeTool::is_assembly(&part).unwrap());
    let main = document.main().unwrap();
    assert!(XCAFDocEditor::expand(&main, true).unwrap());
    assert!(XCAFDocShapeTool::is_assembly(&part).unwrap());
    assert_eq!(XCAFDocShapeTool::nb_components(&part, false).unwrap(), 2);
}
