//! Preserve the single-parent history checked by `bugs_modalg_7::bug31031`.
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPIMakeVertex};

#[test]
fn vertex_prism_preserves_one_generated_history_relation() {
    let mut vertex = BRepBuilderAPIMakeVertex::new(Point3::new(0.0, 0.0, 0.0)).unwrap();
    let source = vertex.shape().unwrap().clone();
    let mut prism =
        BRepPrimAPIMakePrism::new(&source, Vector3::new(0.0, 0.0, 2.0), false, true).unwrap();
    // A vertex has no proper subshapes: its sole generated edge is the complete
    // history, independently of the ordering used to enumerate parent entities.
    assert!(!prism.is_deleted(&source).unwrap());
    assert_eq!(prism.generated(&source).unwrap().len(), 1);
}
