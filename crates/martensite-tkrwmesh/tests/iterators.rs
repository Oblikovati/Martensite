//! The face, edge and vertex iterators on shapes and document labels.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Edge, Face, Shape, Vertex};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::{Color, ColorRgba, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::{Point2, Point3, Transform3, Vector3};
use martensite_tkmath::poly::{Polygon3D, Triangle, Triangulation};
use martensite_tkmath::toploc::Location;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkrwmesh::rw_mesh::{
    RwMeshEdgeIterator, RwMeshFaceIterator, RwMeshShapeIterator as _, RwMeshVertexIterator,
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorType, XCAFDocDocumentTool};
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

fn triangle_face(with_normals: bool) -> Face {
    let mesh = Triangulation::with_sizes(3, 1, true, with_normals).unwrap();
    mesh.set_node(1, Point3::new(0.0, 0.0, 0.0)).unwrap();
    mesh.set_node(2, Point3::new(1.0, 0.0, 0.0)).unwrap();
    mesh.set_node(3, Point3::new(0.0, 1.0, 0.0)).unwrap();
    mesh.set_uv_node(2, Point2::new(1.0, 0.0)).unwrap();
    mesh.set_triangle(1, Triangle::new(1, 2, 3)).unwrap();
    if with_normals {
        mesh.set_normal_vec3(1, Vec3::new(0.0, 1.0, 0.0)).unwrap();
        mesh.set_normal_vec3(2, Vec3::new(0.0, 0.0, 0.0)).unwrap();
    }
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_with_triangulation(&mut face, &Handle::new(mesh))
        .unwrap();
    face
}

#[test]
fn a_face_gives_its_nodes_triangles_and_uv() {
    let face = triangle_face(true);
    let mut faces =
        RwMeshFaceIterator::with_shape(&face.clone().into(), &XCAFPrsStyle::default()).unwrap();
    assert!(faces.more());
    assert!(!faces.is_empty_mesh());
    assert_eq!((faces.nb_nodes(), faces.nb_triangles()), (3, 1));
    assert_eq!(
        (
            faces.node_lower(),
            faces.node_upper(),
            faces.elem_lower(),
            faces.elem_upper()
        ),
        (1, 3, 1, 1)
    );
    assert!(faces.has_normals() && faces.has_tex_coords());
    assert_eq!(faces.node_tex_coord(2).unwrap().x(), 1.0);
    assert_eq!(faces.normal(1).unwrap().coordinates()[1], 1.0);
    assert_eq!(
        faces.normal(2).unwrap().coordinates()[2],
        1.0,
        "a zero normal falls back to +Z"
    );
    assert_eq!(faces.triangle(1).unwrap().get(), (1, 2, 3));
    assert_eq!(faces.triangle_oriented(1).unwrap().get(), (1, 2, 3));
    assert!(faces.triangulation().is_some());
    assert!(!faces.face().is_null() && !faces.shape().is_null());
    assert_eq!(faces.node_transformed(2).unwrap().x(), 1.0);
    faces.next().unwrap();
    assert!(!faces.more());
    assert!(faces.is_empty_mesh());
    assert_eq!(faces.nb_nodes(), 0);
}

#[test]
fn a_reversed_face_flips_its_triangles_and_normals() {
    let face = triangle_face(true);
    let reversed = face.oriented(Orientation::Reversed);
    let faces = RwMeshFaceIterator::with_shape(&reversed, &XCAFPrsStyle::default()).unwrap();
    assert_eq!(faces.triangle_oriented(1).unwrap().get(), (1, 3, 2));
    assert_eq!(faces.normal_transformed(1).unwrap().coordinates()[1], -1.0);
}

#[test]
fn a_location_moves_nodes_and_a_mirror_flips_triangles() {
    let face = triangle_face(false);
    let moved = Location::new(Transform3::translation_vector(Vector3::new(5.0, 0.0, 0.0)));
    let located = face.located(&moved, true).unwrap();
    let faces = RwMeshFaceIterator::with_shape(&located, &XCAFPrsStyle::default()).unwrap();
    assert_eq!(faces.node_transformed(1).unwrap().x(), 5.0);
    assert!(
        faces.has_tex_coords() && !faces.has_normals(),
        "a face without surface has no normals"
    );
    let mirror = Location::new(Transform3::mirror_point(Point3::new(0.0, 0.0, 0.0)));
    let mirrored = face.located(&mirror, false).unwrap();
    let faces = RwMeshFaceIterator::with_shape(&mirrored, &XCAFPrsStyle::default()).unwrap();
    assert_eq!(faces.triangle_oriented(1).unwrap().get(), (1, 3, 2));
}

#[test]
fn a_meshed_box_takes_normals_from_its_surfaces_and_skips_nothing() {
    let mut maker =
        BRepPrimAPIMakeBox::from_points(Point3::new(0., 0., 0.), Point3::new(1., 1., 1.)).unwrap();
    let solid: Shape = maker.solid().unwrap().into_shape();
    BRepMeshIncrementalMesh::with_shape(&solid, 0.1, false, 0.5, false).unwrap();
    let mut faces = RwMeshFaceIterator::with_shape(&solid, &XCAFPrsStyle::default()).unwrap();
    let mut count = 0;
    while faces.more() {
        assert!(faces.has_normals());
        let normal = faces.normal_transformed(1).unwrap();
        let [x, y, z] = *normal.coordinates();
        assert!(((x * x + y * y + z * z) - 1.0).abs() < 1e-9);
        count += 1;
        faces.next().unwrap();
    }
    assert_eq!(count, 6);
}

#[test]
fn a_style_gives_the_color_of_a_shape_without_a_label() {
    let mut style = XCAFPrsStyle::default();
    style.set_color_curv(Color::named(NamedColor::RED));
    let faces = RwMeshFaceIterator::with_shape(&triangle_face(false).into(), &style).unwrap();
    assert!(faces.has_face_color() && faces.has_color());
    assert!(
        faces
            .face_color()
            .rgb()
            .is_equal(Color::named(NamedColor::RED))
    );
    assert!(faces.face_style().is_set_color_curv());
    assert!(faces.style().is_set_color_curv());
    assert_eq!(faces.color().alpha(), 1.0);
    assert!(faces.explored_shape().shape_type().unwrap() == ShapeEnum::Face);
}

#[test]
fn a_document_label_dispatches_the_colors_of_its_sub_shapes_to_its_faces() {
    let document = TDocStdDocument::new(&ExtendedString::new("BinXCAF")).unwrap();
    let main = document.get().unwrap().main().unwrap();
    let tool = XCAFDocDocumentTool::shape_tool(&main).unwrap();
    let colors = XCAFDocDocumentTool::color_tool(&main).unwrap();
    let (colored, plain): (Shape, Shape) =
        (triangle_face(false).into(), triangle_face(false).into());
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    builder.add(&mut compound, &colored).unwrap();
    builder.add(&mut compound, &plain).unwrap();
    let label = tool
        .get()
        .unwrap()
        .add_shape(&compound.into(), false, false)
        .unwrap();
    let blue = ColorRgba::from_rgb(Color::named(NamedColor::BLUE), 1.0);
    assert!(
        colors
            .get()
            .unwrap()
            .set_shape_color(&colored, &blue, XCAFDocColorType::ColorSurf)
            .unwrap()
    );
    let mut mapped = RwMeshFaceIterator::with_label(
        &label,
        &Location::default(),
        true,
        &XCAFPrsStyle::default(),
    )
    .unwrap();
    assert!(mapped.has_face_color());
    assert!(
        mapped
            .face_color()
            .rgb()
            .is_equal(Color::named(NamedColor::BLUE))
    );
    mapped.next().unwrap();
    assert!(mapped.more() && !mapped.has_face_color());
    let plain = RwMeshFaceIterator::with_label(
        &label,
        &Location::default(),
        false,
        &XCAFPrsStyle::default(),
    )
    .unwrap();
    assert!(plain.more() && !plain.has_face_color());
}

fn edge_with_polygon() -> Edge {
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder.make_edge(&mut edge).unwrap();
    let polygon = Polygon3D::new(2, false).unwrap();
    polygon.change_nodes(|nodes| {
        nodes.set_value(1, Point3::new(0.0, 0.0, 0.0)).unwrap();
        nodes.set_value(2, Point3::new(1.0, 0.0, 0.0)).unwrap();
    });
    builder
        .update_edge_polygon3d(&edge, &Handle::new(polygon), &Location::default())
        .unwrap();
    edge
}

#[test]
fn edges_and_vertices_outside_faces_are_visited() {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    builder
        .add(&mut compound, &edge_with_polygon().into())
        .unwrap();
    let mut vertex = Vertex::default();
    builder
        .make_vertex_at(&mut vertex, Point3::new(1.0, 2.0, 3.0), 1e-7)
        .unwrap();
    builder.add(&mut compound, &vertex.into()).unwrap();
    let shape: Shape = compound.into();
    let mut edges = RwMeshEdgeIterator::with_shape(&shape, &XCAFPrsStyle::default()).unwrap();
    assert!(edges.more() && !edges.is_empty());
    assert_eq!(
        (edges.nb_nodes(), edges.node_lower(), edges.node_upper()),
        (2, 1, 2)
    );
    assert_eq!((edges.elem_lower(), edges.elem_upper()), (1, 2));
    assert_eq!(edges.node_transformed(2).unwrap().x(), 1.0);
    assert!(!edges.edge().is_null() && !edges.shape().is_null() && edges.polygon3d().is_some());
    edges.next().unwrap();
    assert!(!edges.more() && edges.is_empty());
    let mut vertices = RwMeshVertexIterator::with_shape(&shape, &XCAFPrsStyle::default()).unwrap();
    assert!(vertices.more() && !vertices.is_empty());
    assert_eq!(
        (
            vertices.nb_nodes(),
            vertices.node_lower(),
            vertices.node_upper()
        ),
        (1, 1, 1)
    );
    assert_eq!((vertices.elem_lower(), vertices.elem_upper()), (1, 1));
    assert_eq!(vertices.point().y(), 2.0);
    assert_eq!(vertices.node_transformed(1).unwrap().z(), 3.0);
    assert!(!vertices.vertex().is_null() && !vertices.shape().is_null());
    vertices.next().unwrap();
    assert!(!vertices.more() && vertices.is_empty());
}
