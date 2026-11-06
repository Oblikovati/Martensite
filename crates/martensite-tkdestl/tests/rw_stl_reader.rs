//! `RWStl` and `RwStlReader` beyond the tests: callbacks, merging, domains and failures.
mod support;

use martensite_tkdestl::rw_stl::{
    DEFAULT_MERGE_ANGLE, OStreamWriter, RwStl, RwStlReader, RwStlReaderBase,
};
use martensite_tkernel::{
    impl_transient,
    message::{ProgressIndicator, ProgressRange},
    ncollection::Sequence,
    osd::{CadPath, IStream, StreamAccess},
    standard::{Handle, ReadLineBuffer, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkmath::{gp::Xyz, poly::Triangulation};
use std::f64::consts::FRAC_PI_4;
use support::memory_stream::{memory_istream, memory_ostream};
use support::shapes::mesh_of;

/// A reader recording its callbacks.
#[derive(Default)]
struct RecordingReader {
    transient: StandardTransient,
    base: RwStlReaderBase,
    nodes: SharedState<Vec<[f64; 3]>>,
    triangles: SharedState<Vec<[i32; 3]>>,
    solids: SharedState<i32>,
}
impl_transient!(RecordingReader, "RecordingReader", dyn RwStlReader, transient, interfaces: [dyn RwStlReader]);
impl RwStlReader for RecordingReader {
    fn reader_base(&self) -> &RwStlReaderBase {
        &self.base
    }
    fn add_node(&self, point: &Xyz) -> i32 {
        let mut nodes = self.nodes.write();
        nodes.push([point.x(), point.y(), point.z()]);
        // Identifiers need not be sequential; these prove the remap uses what this returns.
        100 + nodes.len() as i32
    }
    fn add_triangle(&self, node1: i32, node2: i32, node3: i32) {
        self.triangles.write().push([node1, node2, node3]);
    }
    fn add_solid(&self) -> Result<(), martensite_tkernel::standard::Failure> {
        *self.solids.write() += 1;
        Ok(())
    }
}

fn range() -> ProgressRange<'static> {
    ProgressRange::default()
}

fn temporary_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("tkdestl_{}_{name}", std::process::id()))
}

fn ascii_path(path: &std::path::Path) -> AsciiString {
    AsciiString::new(path.to_str().unwrap())
}

/// Two facets sharing an edge at a right angle.
const FOLDED_ASCII: &str = "solid fold\n\
facet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\n\
facet normal 0 1 0\nouter loop\nvertex 0 0 0\nvertex 0 0 1\nvertex 1 0 0\nendloop\nendfacet\n\
endsolid fold\n";

fn binary_stl(facets: &[[[f32; 3]; 3]], stated_count: i32) -> Vec<u8> {
    let mut bytes = vec![0; 80];
    bytes.extend_from_slice(&stated_count.to_le_bytes());
    for facet in facets {
        bytes.extend_from_slice(&[0; 12]);
        for value in facet.iter().flatten() {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.extend_from_slice(&[0, 0]);
    }
    bytes
}

const TRIANGLE: [[f32; 3]; 3] = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]];

#[test]
fn callbacks_receive_merged_nodes_and_their_own_identifiers() {
    let reader = RecordingReader::default();
    let mut stream = memory_istream(FOLDED_ASCII.as_bytes());
    let mut lines = ReadLineBuffer::new(1024);
    assert_eq!(
        reader.read_ascii(&mut stream, &mut lines, FOLDED_ASCII.len() as i64, &range()),
        Ok(true)
    );
    assert_eq!(reader.nodes.read().len(), 4);
    assert_eq!(*reader.triangles.read(), [[101, 102, 103], [101, 104, 102]]);
}

#[test]
fn a_merge_angle_keeps_nodes_split_at_a_sharp_edge() {
    let mut stream = memory_istream(FOLDED_ASCII.as_bytes());
    let merged = RwStl::read_ascii_stream(&mut stream, DEFAULT_MERGE_ANGLE, &range()).unwrap();
    assert_eq!(merged.get().unwrap().nb_nodes(), 4);
    let mut stream = memory_istream(FOLDED_ASCII.as_bytes());
    let split = RwStl::read_ascii_stream(&mut stream, FRAC_PI_4, &range()).unwrap();
    assert_eq!(split.get().unwrap().nb_nodes(), 6);
    assert_eq!(split.get().unwrap().nb_triangles(), 2);
}

#[test]
fn reader_defaults_and_setters() {
    let reader = RecordingReader::default();
    assert_eq!(reader.merge_angle(), std::f64::consts::FRAC_PI_2);
    assert_eq!(reader.merge_tolerance(), 0.0);
    reader.set_merge_angle(0.5);
    reader.set_merge_tolerance(0.25);
    assert_eq!(reader.merge_angle(), 0.5);
    assert_eq!(reader.merge_tolerance(), 0.25);
}

#[test]
fn a_merge_tolerance_merges_close_points() {
    let close = [[1.0005f32, 0., 0.], [2., 0., 0.], [1.0005, 1., 0.]];
    let facets = binary_stl(&[TRIANGLE, close], 2);
    let exact = RecordingReader::default();
    assert_eq!(
        exact.read_binary(&mut memory_istream(&facets), &range()),
        Ok(true)
    );
    assert_eq!(exact.nodes.read().len(), 6);
    let tolerant = RecordingReader::default();
    tolerant.set_merge_tolerance(0.01);
    assert_eq!(
        tolerant.read_binary(&mut memory_istream(&facets), &range()),
        Ok(true)
    );
    assert_eq!(tolerant.nodes.read().len(), 5);
}

#[test]
fn degenerate_facets_create_no_triangle() {
    let degenerate = [[0., 0., 0.], [0., 0., 0.], [0., 1., 0.]];
    let reader = RecordingReader::default();
    let bytes = binary_stl(&[degenerate, TRIANGLE], 2);
    assert_eq!(
        reader.read_binary(&mut memory_istream(&bytes), &range()),
        Ok(true)
    );
    assert_eq!(reader.triangles.read().len(), 1);
}

#[test]
fn is_ascii_probes_and_restores_the_stream() {
    let reader = RecordingReader::default();
    let mut text = vec![b'a'; 200];
    let mut stream = memory_istream(&text);
    assert!(reader.is_ascii(&mut stream, true));
    assert_eq!(stream.tellg(), 0);
    text[133] = 200;
    let mut stream = memory_istream(&text);
    assert!(!reader.is_ascii(&mut stream, true));
    assert_eq!(stream.tellg(), 0);
    let mut stream = memory_istream(&text);
    assert!(!reader.is_ascii(&mut stream, false));
    assert_eq!(stream.tellg(), 0);
    // A byte past the head does not count, and a short stream reports text and fails.
    text[133] = b'a';
    text[134] = 200;
    assert!(reader.is_ascii(&mut memory_istream(&text), true));
    let mut short = memory_istream(b"solid");
    assert!(reader.is_ascii(&mut short, true));
    assert!(short.fail());
}

#[test]
fn a_binary_count_beyond_the_data_fails_and_a_negative_one_reads_nothing() {
    let reader = RecordingReader::default();
    let beyond = binary_stl(&[TRIANGLE], 2);
    assert_eq!(
        reader.read_binary(&mut memory_istream(&beyond), &range()),
        Ok(false)
    );
    assert!(reader.triangles.read().is_empty());
    let negative = binary_stl(&[TRIANGLE], -1);
    assert_eq!(
        reader.read_binary(&mut memory_istream(&negative), &range()),
        Ok(true)
    );
    assert!(reader.triangles.read().is_empty());
    assert_eq!(
        reader.read_binary(&mut memory_istream(&[0; 83]), &range()),
        Ok(false)
    );
}

#[test]
fn text_failures_stop_reading_with_the_facets_read() {
    for (name, text, expected) in [
        ("not a facet", "solid\nnot a facet\n", false),
        ("no outer", "solid\nfacet\nnot outer\n", false),
        ("short vertex", "solid\nfacet\nouter\nvertex 1 2\n", false),
        (
            "no endsolid",
            "solid\nfacet\nouter\nvertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\n",
            false,
        ),
        (
            "end at a vertex line",
            "solid\nfacet\nouter\nvertex 0 0 0\n",
            true,
        ),
        (
            "empty lines before the header",
            "\n\nsolid\nendsolid\n",
            true,
        ),
        ("nothing", "", false),
        (
            "mixed case keywords",
            "solid\n FACET\n Outer\nVERTEX 0 0 0\nVertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\n ENDSOLID\n",
            true,
        ),
    ] {
        let reader = RecordingReader::default();
        let mut lines = ReadLineBuffer::new(1024);
        let read = reader.read_ascii(
            &mut memory_istream(text.as_bytes()),
            &mut lines,
            0,
            &range(),
        );
        assert_eq!(read, Ok(expected), "{name}");
    }
}

#[test]
fn a_vertex_line_takes_scientific_signed_and_split_numbers() {
    let text = "solid\nfacet\nouter\nvertex -1.5e+1 .5 +2\nvertex\t1\t0\t0\n  vertex 0 1 0 extra\nendloop\nendfacet\nendsolid\n";
    let reader = RecordingReader::default();
    let mut lines = ReadLineBuffer::new(8);
    assert_eq!(
        reader.read_ascii(
            &mut memory_istream(text.as_bytes()),
            &mut lines,
            0,
            &range()
        ),
        Ok(true)
    );
    assert_eq!(reader.nodes.read()[0], [-15.0, 0.5, 2.0]);
    assert_eq!(reader.nodes.read()[2], [0.0, 1.0, 0.0]);
}

#[test]
fn a_user_break_stops_both_readers() {
    use martensite_tkernel::message::{ProgressIndicatorHooks, ProgressScope};
    struct Cancelled;
    impl ProgressIndicatorHooks for Cancelled {
        fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
        fn user_break(&self) -> bool {
            true
        }
    }
    let indicator = ProgressIndicator::new(Box::new(Cancelled));
    let reader = RecordingReader::default();
    let bytes = binary_stl(&[TRIANGLE], 1);
    assert_eq!(
        reader.read_binary(&mut memory_istream(&bytes), &indicator.start()),
        Ok(false)
    );
    assert!(reader.triangles.read().is_empty());
    let mut lines = ReadLineBuffer::new(1024);
    let done = reader.read_ascii(
        &mut memory_istream(FOLDED_ASCII.as_bytes()),
        &mut lines,
        0,
        &indicator.start(),
    );
    assert_eq!(done, Ok(false));
}

/// `Read` returns `!fail()`: text files end in a short block, which fails the stream.
#[test]
fn read_reports_the_stream_state() {
    let path = temporary_path("read_state.stl");
    let reader = RecordingReader::default();
    std::fs::write(&path, FOLDED_ASCII).unwrap();
    assert_eq!(reader.read(&ascii_path(&path), &range()), Ok(false));
    assert_eq!(reader.triangles.read().len(), 2);
    assert_eq!(*reader.solids.read(), 1);

    let reader = RecordingReader::default();
    std::fs::write(&path, binary_stl(&[TRIANGLE], 1)).unwrap();
    // Probed by its bytes: a binary file with a high byte in the first 134.
    let mut bytes = binary_stl(&[TRIANGLE], 1);
    bytes[0] = 200;
    std::fs::write(&path, &bytes).unwrap();
    assert_eq!(reader.read(&ascii_path(&path), &range()), Ok(true));
    assert_eq!(reader.triangles.read().len(), 1);
    assert_eq!(*reader.solids.read(), 1);
    std::fs::remove_file(&path).unwrap();
    assert_eq!(reader.read(&ascii_path(&path), &range()), Ok(false));
}

#[test]
fn concatenated_binary_domains_are_read_separately() {
    let path = temporary_path("domains.stl");
    let mut bytes = binary_stl(&[TRIANGLE], 1);
    bytes[0] = 200;
    let mut file = bytes.clone();
    file.extend_from_slice(b"\n \t");
    file.extend_from_slice(&bytes);
    std::fs::write(&path, &file).unwrap();
    let mut list: Sequence<Handle<Triangulation>> = Sequence::default();
    RwStl::read_file_domains(&ascii_path(&path), DEFAULT_MERGE_ANGLE, &mut list, &range()).unwrap();
    assert_eq!(list.length(), 2);
    let joined = RwStl::read_file(&ascii_path(&path), &range()).unwrap();
    assert_eq!(joined.get().unwrap().nb_triangles(), 2);
    // Each domain has its own merge tool, so nodes do not merge across domains.
    assert_eq!(joined.get().unwrap().nb_nodes(), 6);
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn path_members_round_trip_both_formats() {
    let mesh = Handle::new(mesh_of(
        &[[0., 0., 0.], [2., 0., 0.], [0., 3., 0.]],
        &[[1, 2, 3]],
    ));
    let binary = temporary_path("path_binary.stl");
    let ascii = temporary_path("path_ascii.stl");
    let binary_path = CadPath::new(binary.to_str().unwrap());
    let ascii_path_value = CadPath::new(ascii.to_str().unwrap());
    assert_eq!(
        RwStl::write_binary_path(&mesh, &binary_path, &range()),
        Ok(true)
    );
    assert_eq!(
        RwStl::write_ascii_path(&mesh, &ascii_path_value, &range()),
        Ok(true)
    );
    for read in [
        RwStl::read_binary(&binary_path, &range()),
        RwStl::read_ascii(&ascii_path_value, &range()),
        RwStl::read_path(&binary_path, &range()),
        RwStl::read_path(&ascii_path_value, &range()),
    ] {
        let read = read.unwrap();
        let read = read.get().unwrap();
        assert_eq!((read.nb_nodes(), read.nb_triangles()), (3, 1));
        assert_eq!(read.node(3).unwrap().y(), 3.0);
    }
    std::fs::remove_file(&binary).unwrap();
    std::fs::remove_file(&ascii).unwrap();
    // A missing file gives a null handle from every path member.
    assert!(RwStl::read_path(&binary_path, &range()).unwrap().is_null());
    assert!(
        RwStl::read_binary(&binary_path, &range())
            .unwrap()
            .is_null()
    );
    assert!(RwStl::read_ascii(&binary_path, &range()).unwrap().is_null());
    assert!(
        RwStl::read_file(&ascii_path(&binary), &range())
            .unwrap()
            .is_null()
    );
}

#[test]
fn an_empty_mesh_or_unopenable_path_writes_nothing() {
    let path = CadPath::new("/no/such/directory/mesh.stl");
    let mesh = Handle::new(mesh_of(
        &[[0., 0., 0.], [2., 0., 0.], [0., 3., 0.]],
        &[[1, 2, 3]],
    ));
    assert_eq!(RwStl::write_binary_path(&mesh, &path, &range()), Ok(false));
    assert_eq!(RwStl::write_ascii_path(&mesh, &path, &range()), Ok(false));
    let null: Handle<Triangulation> = Handle::default();
    assert_eq!(RwStl::write_binary_path(&null, &path, &range()), Ok(false));
    assert_eq!(RwStl::write_ascii_path(&null, &path, &range()), Ok(false));
}

#[test]
fn an_output_stream_receives_what_a_vector_receives() {
    let mesh = Handle::new(mesh_of(
        &[[0., 0., 0.], [2., 0., 0.], [0., 3., 0.]],
        &[[1, 2, 3]],
    ));
    let (mut stream, written) = memory_ostream();
    assert_eq!(
        RwStl::write_ascii(&mesh, &mut OStreamWriter(&mut stream), &range()),
        Ok(true)
    );
    let mut bytes = Vec::new();
    assert_eq!(RwStl::write_ascii(&mesh, &mut bytes, &range()), Ok(true));
    assert_eq!(written.contents(), bytes);
}
