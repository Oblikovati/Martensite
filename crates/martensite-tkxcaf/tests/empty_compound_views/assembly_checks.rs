//! Empty geometry does not excuse dropping the actual assembly or identity relationships.
use super::support::document_viewer::ViewerLifecycle;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkv3d::ais::AisShapeParameters;
use martensite_tkvcaf::tprs_std::TPrsStdAISPresentation;
use martensite_tkxcaf::{xcaf_doc::XCAFDocShapeTool, xcaf_prs::XCAFPrsAISObject};

pub fn assert_case_compounds(empty: &Shape, nested: &Shape) {
    assert_eq!(empty.shape_type().unwrap(), ShapeEnum::Compound);
    assert_eq!(nested.shape_type().unwrap(), ShapeEnum::Compound);
    assert!(!empty.is_null() && !nested.is_null());
    assert!(!empty.is_partner(nested));
    assert_eq!(ShapeIterator::new(empty, true, true).count(), 0);
    let children: Vec<Shape> = ShapeIterator::new(nested, true, true).collect();
    assert_eq!(children.len(), 1);
    assert!(children[0].is_same(empty));
}

pub fn assert_document_assembly(assembly: &TdfLabel, empty: &Shape, nested: &Shape) {
    assert!(XCAFDocShapeTool::is_assembly(assembly).unwrap());
    let mut components = Vec::new();
    assert!(XCAFDocShapeTool::get_components(assembly, &mut components, false).unwrap());
    assert_eq!(components.len(), 1);
    let mut definition = TdfLabel::default();
    assert!(XCAFDocShapeTool::get_referred_shape(&components[0], &mut definition).unwrap());
    assert!(XCAFDocShapeTool::is_assembly(&definition).unwrap());
    assert_eq!(
        XCAFDocShapeTool::nb_components(&definition, false).unwrap(),
        0
    );
    let mut stored = Shape::default();
    assert!(XCAFDocShapeTool::get_shape(&definition, &mut stored).unwrap());
    assert!(stored.is_partner(empty));
    let children: Vec<Shape> = ShapeIterator::new(nested, true, true).collect();
    assert_eq!(children.len(), 1);
    assert!(children[0].is_partner(empty));
    assert_eq!(ShapeIterator::new(empty, true, true).count(), 0);
}

pub fn assert_displayed_assembly(
    lifecycle: &ViewerLifecycle,
    presentations: &[Handle<TPrsStdAISPresentation>],
    mode: i32,
) {
    assert_eq!(presentations.len(), 1);
    let object = presentations[0].get_ais();
    assert_presented_topology(&object.cast::<XCAFPrsAISObject>());
    let context = lifecycle.interactive_context();
    assert_eq!(context.display_mode(), mode);
    assert!(context.is_displayed_mode(&object, mode));
    assert_eq!(lifecycle.state().views.borrow().extent(), 1);
}

fn assert_presented_topology(object: &Handle<XCAFPrsAISObject>) {
    assert!(!object.is_null());
    let shown = object.shape();
    assert!(!shown.is_null());
    assert_eq!(shown.shape_type().unwrap(), ShapeEnum::Compound);
    let children: Vec<Shape> = ShapeIterator::new(&shown, true, true).collect();
    assert_eq!(children.len(), 1);
    assert_eq!(ShapeIterator::new(&children[0], true, true).count(), 0);
    let mut stored = Shape::default();
    assert!(XCAFDocShapeTool::get_shape(&object.get_label(), &mut stored).unwrap());
    assert!(shown.is_same(&stored));
}
