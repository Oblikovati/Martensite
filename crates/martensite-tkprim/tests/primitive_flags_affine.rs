//! Native cases for bugs/modalg_6/bug21727 and bug26313 at the pinned source revision.
use martensite_tkbrep::{
    topexp,
    topods::{Builder, Compound, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{GeneralTransform3, Matrix3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIGTransform, BRepBuilderAPIMakeShape,
};

fn analytic_box(side: f64) -> Shape {
    BRepPrimAPIMakeBox::from_sizes(side, side, side)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

#[test]
fn compound_freezes_source_and_geometry_copy_is_free() {
    crate::isolated_primitive_case::run_primitive_case(
        "primitive_flags_affine::compound_freezes_source_and_geometry_copy_is_free",
        compound_freezes_source_and_geometry_copy_is_free_body,
    );
}

fn compound_freezes_source_and_geometry_copy_is_free_body() {
    let source = analytic_box(10.0);
    assert!(source.free().unwrap());
    let mut compound = Compound::default();
    let builder = Builder;
    builder.make_compound(&mut compound);
    builder.add(&mut compound, &source).unwrap();
    assert!(!source.free().unwrap());
    let mut copy = BRepBuilderAPICopy::new();
    copy.perform(&source, true, false).unwrap();
    assert!(copy.shape().unwrap().free().unwrap());
}

fn unique_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut entries = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut entries).unwrap();
    entries.extent()
}

#[test]
fn affine_modified_shape_preserves_all_original_box_topology_counts() {
    crate::isolated_primitive_case::run_primitive_case(
        "primitive_flags_affine::affine_modified_shape_preserves_all_original_box_topology_counts",
        affine_modified_shape_preserves_all_original_box_topology_counts_body,
    );
}

fn affine_modified_shape_preserves_all_original_box_topology_counts_body() {
    let source = analytic_box(100.0);
    let mut transform = GeneralTransform3::default();
    transform.set_vectorial_part(Matrix3::new([[1., 0., 0.], [0., 2., 0.], [0., 0., 3.]]));
    let mut operation = BRepBuilderAPIGTransform::new(transform);
    operation.perform(&source, false).unwrap();
    assert!(operation.is_done());
    let result = operation.modified_shape(&source).unwrap();
    assert_original_box_counts(&result);
}

const BOX_COUNTS: [(ShapeEnum, usize); 8] = [
    (ShapeEnum::Vertex, 8),
    (ShapeEnum::Edge, 12),
    (ShapeEnum::Wire, 6),
    (ShapeEnum::Face, 6),
    (ShapeEnum::Shell, 1),
    (ShapeEnum::Solid, 1),
    (ShapeEnum::CompSolid, 0),
    (ShapeEnum::Compound, 0),
];

fn assert_original_box_counts(result: &Shape) {
    for (kind, expected) in BOX_COUNTS {
        assert_eq!(unique_count(result, kind), expected, "kind {kind:?}");
    }
    let mut entries = IndexedMapOfShape::default();
    topexp::map_shapes(result, &mut entries, true, true);
    assert_eq!(entries.extent(), 34);
}
