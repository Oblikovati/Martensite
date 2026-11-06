//! Triangulated shapes, documents and shape counters of the provider and writer tests.
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{standard::Handle, tcollection::ExtendedString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdoc_std::{
    TDocStdApplication, TDocStdDocument, TDocStdStandardApplication,
};
use martensite_tkmath::{
    gp::{Direction3, Plane3, Point3},
    poly::{Triangle, Triangulation, mesh_purpose},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

/// `an explorer` counted over one shape type.
pub fn count_shapes(shape: &Shape, kind: ShapeEnum) -> i32 {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    let mut count = 0;
    while explorer.more() {
        count += 1;
        explorer.next().unwrap();
    }
    count
}

pub fn count_faces(shape: &Shape) -> i32 {
    count_shapes(shape, ShapeEnum::Face)
}

/// The `CountTriangles`: triangles over every face that has a mesh.
pub fn count_triangles(shape: &Shape) -> i32 {
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut count = 0;
    while explorer.more() {
        let face = topods::face(explorer.current()).unwrap();
        let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        count += mesh.get().map_or(0, Triangulation::nb_triangles);
        explorer.next().unwrap();
    }
    count
}

/// A planar face from (0, 0) to (`size`, `size`) carrying `mesh`.
pub fn planar_face_with_mesh(size: f64, mesh: Triangulation) -> Shape {
    let plane = Plane3::new(
        Point3::new(0., 0., 0.),
        Direction3::new(0., 0., 1.).unwrap(),
    )
    .unwrap();
    let maker = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (0., size, 0., size)).unwrap();
    let face = maker.face().unwrap();
    BRepBuilder::default()
        .update_face_triangulation(&face, &Handle::new(mesh), true)
        .unwrap();
    face.into()
}

/// A mesh on 1-based nodes and triangles.
pub fn mesh_of(nodes: &[[f64; 3]], triangles: &[[i32; 3]]) -> Triangulation {
    let mesh = Triangulation::with_sizes(nodes.len() as i32, triangles.len() as i32, false, false)
        .unwrap();
    for (index, [x, y, z]) in nodes.iter().enumerate() {
        mesh.set_node(index as i32 + 1, Point3::new(*x, *y, *z))
            .unwrap();
    }
    for (index, [a, b, c]) in triangles.iter().enumerate() {
        mesh.set_triangle(index as i32 + 1, Triangle::new(*a, *b, *c))
            .unwrap();
    }
    mesh
}

/// The `CreateTriangulatedFace`: a 10 by 10 quad of two triangles.
pub fn triangulated_face() -> Shape {
    let nodes = [[0., 0., 0.], [10., 0., 0.], [10., 10., 0.], [0., 10., 0.]];
    planar_face_with_mesh(10., mesh_of(&nodes, &[[1, 2, 3], [1, 3, 4]]))
}

/// A `BinXCAF` document with its application, which must outlive it.
pub fn new_xcaf_document() -> (Handle<TDocStdStandardApplication>, Handle<TDocStdDocument>) {
    let application = TDocStdStandardApplication::new();
    let mut document = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .unwrap();
    (application, document)
}

/// One face without a surface on a mesh, as `readstl` and the provider build it.
pub fn face_of_mesh(mesh: &Handle<Triangulation>) -> Shape {
    let builder = BRepBuilder::default();
    let mut face = topods::Face::default();
    builder.make_face(&mut face);
    builder
        .update_face_triangulation(&face, mesh, true)
        .unwrap();
    face.into()
}
