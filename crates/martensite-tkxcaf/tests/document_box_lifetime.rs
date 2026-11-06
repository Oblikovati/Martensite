//! Document destruction releases the shape tool and all its named topology.
#[path = "support/release_graph.rs"]
mod release_graph;
mod support;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use martensite_tkxsbase::xs_algo::XSAlgo;

#[test]
fn standalone_box_document_releases_all_transient_owners() {
    let document = TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap();
    let framework = document.get_data();
    let shape = support::make_box(10., 10., 10.);
    support::shape_document::ensure_document_unit(&document);
    let shapes = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
    let weak_document = document.downgrade();
    let weak_framework = framework.downgrade();
    let weak_tool = shapes.downgrade();
    let weak_shape = shape.tshape().downgrade();
    shapes.get().unwrap().add_shape(&shape, true, true).unwrap();
    drop((shapes, shape, framework, document));
    assert!(weak_document.upgrade().is_null());
    assert!(weak_framework.upgrade().is_null());
    assert!(weak_tool.upgrade().is_null());
    assert!(weak_shape.upgrade().is_null());
}

#[test]
fn repeated_document_release_expires_geometry_labels_and_naming_graph() {
    XSAlgo::init().unwrap();
    for _ in 1..100 {
        let document = TDocStdDocument::new(&ExtendedString::from_utf8("dummy")).unwrap();
        let shape = support::make_box(10., 10., 10.);
        support::shape_document::ensure_document_unit(&document);
        let shapes = XCAFDocDocumentTool::shape_tool(&document.main().unwrap()).unwrap();
        shapes.get().unwrap().add_shape(&shape, true, true).unwrap();
        let mut graph = release_graph::ReleasedDocumentGraph::default();
        graph.observe("document", &document);
        graph.observe("framework", &document.get_data());
        graph.observe("shape tool", &shapes);
        graph.observe_topology(&shape);
        graph.observe_framework(&document.get_data().root());
        graph.assert_observed_scope();
        drop((shapes, shape, document));
        graph.assert_expired();
    }
}
