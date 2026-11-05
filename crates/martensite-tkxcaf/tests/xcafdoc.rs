//! The shape tool of a document: its labels, assemblies and naming.
#![allow(non_snake_case)]

mod support;

use martensite_tkernel::standard::Guid;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tkxcaf::xcaf_doc::{XCAFDoc, XCAFDocDocumentTool, XCAFDocShapeTool};

/// `StandardGUID("00000000-0000-0000-0000-000000000000")`.
fn null_guid() -> Guid {
    Guid::from_ascii(b"00000000-0000-0000-0000-000000000000").unwrap()
}

#[test]
fn XCAFDocTest__OCC738_ShapeRefGUID() {
    let guid = XCAFDoc::shape_ref_guid();
    assert_ne!(guid, null_guid(), "ShapeRefGUID should not be null");
}

#[test]
fn XCAFDocTest__OCC738_AssemblyGUID() {
    let guid = XCAFDoc::assembly_guid();
    assert_ne!(guid, null_guid(), "AssemblyGUID should not be null");
}

/// RAII guard to save and restore `XCAFDocShapeTool` AutoNaming state.
struct AutoNamingGuard {
    saved: bool,
}
impl AutoNamingGuard {
    fn new() -> Self {
        Self {
            saved: XCAFDocShapeTool::auto_naming(),
        }
    }
}
impl Drop for AutoNamingGuard {
    fn drop(&mut self) {
        XCAFDocShapeTool::set_auto_naming(self.saved);
    }
}

#[test]
fn XCAFDocTest__OCC23595_AutoNaming() {
    let _guard = AutoNamingGuard::new();
    let (_application, document) = support::new_document("XmlXCAF");
    assert!(!document.is_null());
    let shape_tool = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
    let tool = shape_tool.get().unwrap();
    assert!(
        XCAFDocShapeTool::auto_naming(),
        "AutoNaming should be true by default"
    );
    XCAFDocShapeTool::set_auto_naming(true);
    let label1 = tool
        .add_shape(&support::make_box(100.0, 200.0, 300.0), true, true)
        .unwrap();
    assert!(
        label1.is_attribute(&TDataStdName::get_id()).unwrap(),
        "Shape should have a name attribute when AutoNaming is true"
    );
    XCAFDocShapeTool::set_auto_naming(false);
    let label2 = tool
        .add_shape(&support::make_box(300.0, 200.0, 100.0), true, true)
        .unwrap();
    assert!(
        !label2.is_attribute(&TDataStdName::get_id()).unwrap(),
        "Shape should not have a name attribute when AutoNaming is false"
    );
}
