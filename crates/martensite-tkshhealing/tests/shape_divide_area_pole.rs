//! Dividing a face that closes at a pole keeps one vertex at the pole: its boundary there has no
//! length, so each cut across it must end on the vertex that is already there.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::precision;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeShapeDivideArea;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

const RADIUS: f64 = 10.;

fn parts_of(shape: &Shape, kind: ShapeEnum) -> IndexedMapOfShape {
    let mut found = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut found).unwrap();
    found
}

fn hemisphere_face() -> Shape {
    let quarter_turn = 90_f64.to_radians();
    let mut maker = BRepPrimAPIMakeSphere::with_latitudes(RADIUS, 0., quarter_turn).unwrap();
    let solid = maker.shape().unwrap().clone();
    parts_of(&solid, ShapeEnum::Face)
        .find_key(1)
        .unwrap()
        .clone()
}

fn divided(numbers: (i32, Option<i32>)) -> Shape {
    let divider = ShapeUpgradeShapeDivideArea::new(hemisphere_face());
    divider.set_splitting_by_number(true);
    match numbers {
        (parts, None) => divider.set_nb_parts(parts),
        (u, Some(v)) => divider.set_numbers_uv_splits(u, v).unwrap(),
    }
    divider.perform(true).unwrap();
    divider.result()
}

fn vertices_at_pole(shape: &Shape) -> usize {
    let pole = Point3::new(0., 0., RADIUS);
    let vertices = parts_of(shape, ShapeEnum::Vertex);
    (1..=vertices.extent())
        .map(|index| topods::vertex(vertices.find_key(index).unwrap()).unwrap())
        .filter(|vertex| BRepTool::pnt(vertex).unwrap().distance(&pole) <= precision::confusion())
        .count()
}

fn assert_counts(numbers: (i32, Option<i32>), expected: [usize; 3]) {
    let result = divided(numbers);
    let counts = [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face]
        .map(|kind| parts_of(&result, kind).extent());
    assert_eq!(counts, expected, "vertices, edges, faces of {numbers:?}");
    assert_eq!(
        vertices_at_pole(&result),
        1,
        "vertices at the pole of {numbers:?}"
    );
}

#[test]
fn a_few_parts_share_the_vertex_at_the_pole() {
    assert_counts((10, None), [11, 25, 10]);
}

#[test]
fn many_parts_share_the_vertex_at_the_pole() {
    assert_counts((100, None), [103, 221, 102]);
}

#[test]
fn a_grid_of_parts_shares_the_vertex_at_the_pole() {
    assert_counts((3, Some(5)), [16, 33, 15]);
}
