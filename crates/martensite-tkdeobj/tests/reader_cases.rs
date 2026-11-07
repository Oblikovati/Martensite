//! Reader behaviour on small inline files.
use martensite_tkdeobj::rw_obj::{RwObjReader as _, RwObjTools, RwObjTriangulationReader};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Xyz;
use std::io::Cursor;

fn read_one_mesh(
    text: &str,
) -> martensite_tkernel::standard::Handle<martensite_tkmath::poly::Triangulation> {
    let mut reader = RwObjTriangulationReader::new();
    reader.set_create_shapes(false);
    let mut stream = Cursor::new(text.as_bytes().to_vec());
    let progress = ProgressRange::default();
    assert!(
        reader
            .read_stream(&mut stream, &AsciiString::new("inline.obj"), &progress)
            .unwrap()
    );
    reader
        .get_triangulation()
        .unwrap()
        .expect("a triangulation")
}

#[test]
fn a_quad_becomes_two_triangles_and_nodes_are_shared() {
    let mesh = read_one_mesh("v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1 2 3 4\n");
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (4, 2));
    assert_eq!(mesh.triangle(2).unwrap().get(), (1, 3, 4));
}

#[test]
fn negative_indices_count_back_from_the_last_vertex() {
    let mesh = read_one_mesh("v 0 0 0\nv 1 0 0\nv 1 1 0\nf -3 -2 -1\n");
    assert_eq!(mesh.triangle(1).unwrap().get(), (1, 2, 3));
}

#[test]
fn normals_and_uv_attach_to_the_nodes_of_their_corners() {
    let mesh =
        read_one_mesh("v 0 0 0\nv 1 0 0\nv 1 1 0\nvt 0.5 0.25\nvn 0 0 2\nf 1/1/1 2/1/1 3/1/1\n");
    assert!(mesh.has_uv_nodes() && mesh.has_normals());
    assert_eq!(mesh.uv_node(2).unwrap().y(), 0.25);
    assert_eq!(mesh.normal_vec3(3).unwrap().z, 2.0);
}

#[test]
fn a_vertex_index_out_of_range_drops_the_element_and_the_read_still_answers() {
    let mut reader = RwObjTriangulationReader::new();
    reader.set_create_shapes(false);
    let mut stream = Cursor::new(b"v 0 0 0\nf 1 2 3\n".to_vec());
    let progress = ProgressRange::default();
    assert!(
        reader
            .read_stream(&mut stream, &AsciiString::new("bad.obj"), &progress)
            .unwrap()
    );
    assert!(reader.get_triangulation().unwrap().is_none());
}

#[test]
fn an_empty_stream_fails() {
    let mut reader = RwObjTriangulationReader::new();
    let mut stream = Cursor::new(Vec::new());
    let progress = ProgressRange::default();
    assert!(
        !reader
            .read_stream(&mut stream, &AsciiString::new("empty.obj"), &progress)
            .unwrap()
    );
}

#[test]
fn a_memory_limit_stops_the_read() {
    let mut reader = RwObjTriangulationReader::new();
    reader.set_memory_limit(10);
    let mut stream = Cursor::new(b"v 0 0 0\nv 1 0 0\nv 1 1 0\nf 1 2 3\n".to_vec());
    let progress = ProgressRange::default();
    assert!(
        !reader
            .read_stream(&mut stream, &AsciiString::new("limit.obj"), &progress)
            .unwrap()
    );
}

#[test]
fn probing_counts_without_reading() {
    let mut reader = RwObjTriangulationReader::new();
    let mut stream =
        Cursor::new(b"# head\n#  second \nv 0 0 0\nv 1 0 0\nf 1 2 2\n# late\n".to_vec());
    let progress = ProgressRange::default();
    assert!(
        reader
            .probe_stream(&mut stream, &AsciiString::new("probe.obj"), &progress)
            .unwrap()
    );
    assert_eq!((reader.nb_probe_nodes(), reader.nb_probe_elems()), (2, 1));
    assert_eq!(reader.file_comments().as_bytes(), b"head\nsecond");
}

#[test]
fn parsers_of_the_format() {
    let mut name = AsciiString::default();
    assert!(RwObjTools::read_name(b"  a name \r\n", &mut name));
    assert_eq!(name.as_bytes(), b"a name");
    assert!(!RwObjTools::read_name(b"  \n", &mut name));
    assert!(name.is_empty());
    let mut point = Xyz::new(0.0, 0.0, 0.0);
    assert_eq!(RwObjTools::read_vec3(b"1 2.5 -3 x", &mut point), (8, true));
    assert_eq!(point.y(), 2.5);
    let mut vector = Vec3::new(0.0_f32, 0.0, 0.0);
    assert!(!RwObjTools::read_vec3_f32(b"1 2", &mut vector).1);
    assert!(RwObjTools::is_space_char(b'\t') && !RwObjTools::is_space_char(b'\n'));
}
