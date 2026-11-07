//! Writer behaviour: the header names this library, and a document round trips.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkde::de::{
    DeConfigurationNode, DeProvider as _, DeWrapper, new_configuration_node,
};
use martensite_tkdeobj::de_obj::{DeObjConfigurationNode, DeObjProvider};
use martensite_tkdeobj::rw_obj::{
    RwObjCafReader, RwObjCafWriter, RwObjObjMaterialMap, RwObjObjWriterContext, RwObjReader as _,
    RwObjTriangulationReader,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::{IndexedDataMap, Vec2, Vec3, Vec4};
use martensite_tkernel::standard::{Handle, Transient as _};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::{Triangulation, mesh_purpose};
use std::io::Cursor;
use std::path::PathBuf;

fn file_in_temp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("tkdeobj-{}-{name}", std::process::id()))
}

fn text(path: &std::path::Path) -> AsciiString {
    AsciiString::new(path.to_str().unwrap())
}

#[test]
fn the_context_writes_statements_with_shifted_indices_and_this_library_in_the_header() {
    let path = file_in_temp("context.obj");
    let mut writer = RwObjObjWriterContext::new(&text(&path));
    let mut information = IndexedDataMap::default();
    information.add(AsciiString::new("Author"), AsciiString::new("A"));
    information.add(AsciiString::new("Comments"), AsciiString::new("one\ntwo"));
    assert!(writer.write_header(3, 1, &AsciiString::new("c.mtl"), &information));
    assert!(writer.write_active_material(&AsciiString::new("m1")));
    assert!(writer.write_group(&AsciiString::new("line\nbreak")));
    assert!(writer.write_vertex(&Vec3::new(1.0, -0.5, 0.0)));
    writer.set_normals(true);
    assert!(writer.write_normal(&Vec3::new(0.0, 0.0, 1.0)));
    writer.set_tex_coords(true);
    assert!(writer.write_tex_coord(&martensite_tkernel::ncollection::Vec2::new(0.5, 0.25)));
    assert!(writer.write_triangle(&Vec3::new(0, 1, 2)));
    writer.flush_face(3);
    writer.set_tex_coords(false);
    assert!(writer.write_quad(&Vec4::new(0, 1, 2, 3)));
    assert!(writer.close());
    let written = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(
        written,
        "# Exported by Martensite\n#  Vertices: 3\n#     Faces: 1\n# Author: A\n# Comments: one\n# two\nmtllib c.mtl\nusemtl m1\ng line break\nv 1 -0.5 0\nvn 0 0 1\nvt 0.5 0.25\nf 1/1/1 2/2/2 3/3/3\nf 4//4 5//5 6//6 7//7\n"
    );
}

/// Writes one triangle with these values through the context and reads it with the crate's reader.
fn through_the_file(
    name: &str,
    nodes: [Vec3<f32>; 3],
    normal: Vec3<f32>,
    uv: Vec2<f32>,
) -> Handle<Triangulation> {
    let path = file_in_temp(name);
    let mut writer = RwObjObjWriterContext::new(&text(&path));
    for node in &nodes {
        assert!(writer.write_vertex(node));
    }
    writer.set_normals(true);
    assert!(writer.write_normal(&normal));
    writer.set_tex_coords(true);
    assert!(writer.write_tex_coord(&uv));
    let corners = "f 1/1/1 2/1/1 3/1/1\n";
    assert!(writer.close());
    let mut written = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    written.push_str(corners);
    let mut reader = RwObjTriangulationReader::new();
    reader.set_create_shapes(false);
    let mut stream = Cursor::new(written.into_bytes());
    let progress = ProgressRange::default();
    assert!(
        reader
            .read_stream(&mut stream, &AsciiString::new("values.obj"), &progress)
            .unwrap()
    );
    reader
        .get_triangulation()
        .unwrap()
        .expect("a triangulation")
}

#[test]
fn a_small_value_is_not_written_as_zero() {
    let path = file_in_temp("small.obj");
    let mut writer = RwObjObjWriterContext::new(&text(&path));
    assert!(writer.write_vertex(&Vec3::new(1.0e-7, 0.0, 0.0)));
    writer.set_normals(true);
    assert!(writer.write_normal(&Vec3::new(1.0e-7, 0.0, 1.0)));
    writer.set_tex_coords(true);
    assert!(writer.write_tex_coord(&Vec2::new(1.0e-7, 0.0)));
    assert!(writer.close());
    let written = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(written.contains("v 1.00000001e-07 0 0\n"), "{written}");
    assert!(written.contains("vn 1.00000001e-07 0 1\n"), "{written}");
    assert!(written.contains("vt 1.00000001e-07 0\n"), "{written}");
    assert!(!written.contains("0.000000"), "{written}");
}

#[test]
fn positions_normals_and_uvs_read_back_bit_identical() {
    let nodes = [
        Vec3::new(1.0e-7_f32, 1234.5678, -8_388_607.5),
        Vec3::new(16_777_215.0, 1.0 + f32::EPSILON, 0.1),
        Vec3::new(-0.333_333_34, 3.0e-5, 1.0e10),
    ];
    let normal = Vec3::new(1.0e-7_f32, 0.6, 0.8);
    let uv = Vec2::new(1.0e-7_f32, 0.333_333_34);
    let mesh = through_the_file("values.obj", nodes, normal, uv);
    assert_eq!(mesh.nb_nodes(), 3);
    for (rank, node) in nodes.iter().enumerate() {
        let index = rank as i32 + 1;
        let read = mesh.node(index).unwrap();
        for (found, written) in [(read.x(), node.x), (read.y(), node.y), (read.z(), node.z)] {
            assert_eq!((found as f32).to_bits(), written.to_bits(), "node {index}");
        }
        let read = mesh.normal_vec3(index).unwrap();
        for (found, written) in [(read.x, normal.x), (read.y, normal.y), (read.z, normal.z)] {
            assert_eq!(found.to_bits(), written.to_bits(), "normal {index}");
        }
        let read = mesh.uv_node(index).unwrap();
        for (found, written) in [(read.x(), uv.x), (read.y(), uv.y)] {
            assert_eq!((found as f32).to_bits(), written.to_bits(), "uv {index}");
        }
    }
}

#[test]
fn a_writer_without_data_refuses() {
    let document = martensite_tklcaf::tdoc_std::TDocStdDocument::new(
        &martensite_tkernel::tcollection::ExtendedString::new("BinXCAF"),
    )
    .unwrap();
    let path = file_in_temp("nothing.obj");
    let mut writer = RwObjCafWriter::new(&text(&path));
    let information = IndexedDataMap::default();
    assert!(
        !writer
            .perform(&document, &information, &ProgressRange::default())
            .unwrap()
    );
}

fn quad_file(name: &str) -> PathBuf {
    let path = file_in_temp(name);
    std::fs::write(&path, "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\n").unwrap();
    path
}

fn provider() -> DeObjProvider {
    let node = new_configuration_node(DeObjConfigurationNode::new());
    DeObjProvider::with_node(&node.cast::<dyn DeConfigurationNode>())
}

fn triangles_of(shape: &Shape) -> i32 {
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut total = 0;
    while faces.more() {
        let face = topods::face(faces.current()).unwrap();
        let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        total += mesh.get().map_or(0, Triangulation::nb_triangles);
        faces.next().unwrap();
    }
    total
}

#[test]
fn a_memory_limit_in_mib_is_a_limit_in_mib() {
    let path = quad_file("limit.obj");
    let node = new_configuration_node(DeObjConfigurationNode::new());
    node.get()
        .unwrap()
        .internal_parameters
        .write()
        .read_memory_limit_mib = 1;
    let provider = DeObjProvider::with_node(&node.cast::<dyn DeConfigurationNode>());
    let mut shape = Shape::default();
    assert!(
        provider
            .read_shape(&text(&path), &mut shape, &ProgressRange::default())
            .unwrap()
    );
    assert_eq!(triangles_of(&shape), 2);
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn a_one_letter_comment_is_kept_and_a_polygon_past_the_element_buffer_reads() {
    let path = file_in_temp("oneletter.obj");
    let mut writer = RwObjObjWriterContext::new(&text(&path));
    let mut information = IndexedDataMap::default();
    information.add(AsciiString::new("Author"), AsciiString::new("A"));
    assert!(writer.write_header(0, 0, &AsciiString::default(), &information));
    assert!(writer.close());
    assert!(
        std::fs::read_to_string(&path)
            .unwrap()
            .contains("# Author: A\n")
    );
    std::fs::remove_file(&path).unwrap();
    let mut big = String::new();
    let count = 1500;
    for index in 0..count {
        let angle = f64::from(index) * std::f64::consts::TAU / f64::from(count);
        big += &format!(
            "v {} {} 0\n",
            martensite_cmath::cos(angle),
            martensite_cmath::sin(angle)
        );
    }
    big += "f";
    for index in 1..=count {
        big += &format!(" {index}");
    }
    big += "\n";
    let path = file_in_temp("big.obj");
    std::fs::write(&path, big).unwrap();
    let mut shape = Shape::default();
    assert!(
        provider()
            .read_shape(&text(&path), &mut shape, &ProgressRange::default())
            .unwrap()
    );
    assert_eq!(triangles_of(&shape), count - 2);
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn a_repeated_line_of_a_comment_is_kept() {
    let path = file_in_temp("repeated.obj");
    let mut writer = RwObjObjWriterContext::new(&text(&path));
    let mut information = IndexedDataMap::default();
    information.add(
        AsciiString::new("Comments"),
        AsciiString::new("repeat\nother\nrepeat\nb"),
    );
    assert!(writer.write_header(0, 0, &AsciiString::default(), &information));
    assert!(writer.close());
    let written = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(
        written.contains("# Comments: repeat\n# other\n# repeat\n# b\n"),
        "{written}"
    );
}

#[test]
fn the_provider_reads_a_shape_as_one_face_and_as_shapes() {
    let path = quad_file("provider.obj");
    let node = new_configuration_node(DeObjConfigurationNode::new());
    let provider = DeObjProvider::with_node(&node.clone().cast::<dyn DeConfigurationNode>());
    let mut shape = Shape::default();
    let progress = ProgressRange::default();
    assert!(
        provider
            .read_shape(&text(&path), &mut shape, &progress)
            .unwrap()
    );
    assert!(!shape.is_null());
    node.get()
        .unwrap()
        .internal_parameters
        .write()
        .read_create_shapes = true;
    let mut shapes = Shape::default();
    assert!(
        provider
            .read_shape(&text(&path), &mut shapes, &progress)
            .unwrap()
    );
    assert_eq!(
        triangles_of(&shapes),
        2,
        "read.create.shapes keeps the mesh"
    );
    assert_eq!(triangles_of(&shape), 2);
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn the_provider_round_trips_a_document_and_writes_its_author() {
    let path = quad_file("provider_in.obj");
    let out = file_in_temp("provider_out.obj");
    let document = martensite_tklcaf::tdoc_std::TDocStdDocument::new(
        &martensite_tkernel::tcollection::ExtendedString::new("BinXCAF"),
    )
    .unwrap();
    let node = new_configuration_node(DeObjConfigurationNode::new());
    node.get().unwrap().internal_parameters.write().write_author = AsciiString::new("Someone");
    let provider = DeObjProvider::with_node(&node.cast::<dyn DeConfigurationNode>());
    let progress = ProgressRange::default();
    assert!(
        provider
            .read_document(&text(&path), &document, &progress)
            .unwrap()
    );
    assert!(
        provider
            .write_document(&text(&out), &document, &progress)
            .unwrap()
    );
    let written = std::fs::read_to_string(&out).unwrap();
    assert!(
        written.starts_with("# Exported by Martensite\n"),
        "{written}"
    );
    assert!(written.contains("# Author: Someone\n"), "{written}");
    assert_eq!(written.matches("\nf ").count(), 2, "{written}");
    for file in [path, out] {
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn a_wrapper_finds_the_format_by_the_file_extension_and_round_trips() {
    let path = quad_file("wrapper_in.obj");
    let out = file_in_temp("wrapper_out.obj");
    let node = new_configuration_node(DeObjConfigurationNode::new());
    let wrapper = DeWrapper::new();
    assert!(wrapper.bind(&node.cast::<dyn DeConfigurationNode>()));
    let progress = ProgressRange::default();
    let mut shape = Shape::default();
    assert!(
        wrapper
            .read_shape(&text(&path), &mut shape, &progress)
            .unwrap()
    );
    assert_eq!(triangles_of(&shape), 2);
    assert!(wrapper.write_shape(&text(&out), &shape, &progress).unwrap());
    let mut again = Shape::default();
    assert!(
        wrapper
            .read_shape(&text(&out), &mut again, &progress)
            .unwrap()
    );
    assert_eq!(triangles_of(&again), 2);
    for file in [path, out] {
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn the_vendor_is_this_library() {
    let provider = provider();
    assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
    assert_eq!(provider.get_format().as_bytes(), b"OBJ");
}

#[test]
fn the_exchange_types_say_their_type_and_their_parent() {
    let reader = RwObjCafReader::new();
    assert_eq!(reader.dynamic_type().name(), "RwObjCafReader");
    assert_eq!(
        reader.dynamic_type().parent().unwrap().name(),
        "RwMeshCafReader"
    );
    let file = AsciiString::new("model");
    let writer = RwObjCafWriter::new(&file);
    assert_eq!(writer.dynamic_type().name(), "RwObjCafWriter");
    let map = RwObjObjMaterialMap::new(&file);
    assert_eq!(map.dynamic_type().name(), "RwObjObjMaterialMap");
    assert_eq!(
        map.dynamic_type().parent().unwrap().name(),
        "RwMeshMaterialMap"
    );
}
