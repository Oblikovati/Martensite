//! Complete native boss/pocket controls retain independent measures and source isolation.
use martensite_tkbo::{bop_algo::Operation, brep_algo_api::BRepAlgoAPIBooleanOperation};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
use martensite_tksketch::surface_sketch::{attachment::*, *};
mod checks;
mod fixture;
mod membership;
mod replay;
fn boolean(source: &Shape, tool: &Shape, kind: Operation) -> BRepAlgoAPIBooleanOperation {
    let mut operation = BRepAlgoAPIBooleanOperation::new();
    operation.set_operation(kind);
    operation.set_non_destructive(true);
    operation.set_arguments(&List::from_iter([source.clone()]));
    operation.set_tools(&List::from_iter([tool.clone()]));
    operation.build().unwrap();
    assert!(!operation.has_errors(), "native wrapped {kind:?} refused");
    operation
}
fn apply(kind: Operation, delta: f64, shift: f64, oblique: bool) {
    let attachment = fixture::attachment(oblique);
    let source = attachment.export_native(SupportRevision(3)).unwrap();
    let tool = normal_extrude_profile(&fixture::request(shift, oblique)).unwrap();
    let native = tool.export_native().unwrap();
    let before = checks::tool_signature(&native);
    let result = boolean(&source.body, &native.bodies, kind);
    assert_eq!(before, checks::tool_signature(&native));
    checks::body(result.shape(), delta);
    checks::history(&result, &native);
    membership::check(result.shape(), kind, shift, oblique);
    checks::rebound(&attachment, &source, &result);
    checks::isolated(&attachment, &source, &tool, &native);
}
#[test]
fn wrapped_boss_fuses_with_the_existing_cylindrical_body() {
    apply(Operation::Fuse, 13.2, 0.5, false);
}
#[test]
fn wrapped_pocket_removes_material_from_the_existing_cylindrical_body() {
    apply(Operation::Cut, -5.7, 0.5, false);
}

#[test]
fn seam_crossing_wrapped_boss_retains_support_and_history() {
    apply(Operation::Fuse, 13.2, -1., false);
}
#[test]
fn seam_crossing_wrapped_pocket_retains_support_and_history() {
    apply(Operation::Cut, -5.7, -1., false);
}
#[test]
fn oblique_wrapped_boss_retains_support_and_history() {
    apply(Operation::Fuse, 13.2, 0.5, true);
}
#[test]
fn oblique_wrapped_pocket_retains_support_and_history() {
    apply(Operation::Cut, -5.7, 0.5, true);
}
