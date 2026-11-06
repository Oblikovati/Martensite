//! Documents written into PLY files: counts, ids, colors and refusals.
use martensite_tkbrep::topods::Shape;
use martensite_tkdeply::rw_ply::{RwPlyCafWriter, RwPlyPlyWriterContext, face_color_bytes};
use martensite_tkernel::message::{
    LazyProgressScope, ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope,
};
use martensite_tkernel::ncollection::{IndexedDataMap, Map, Vec4};
use martensite_tkernel::quantity::{Color, ColorRgba, ColorSpace};
use martensite_tkernel::standard::{Handle, RuntimeTyped, Transient as _};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::toploc::Location;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tkrwmesh::rw_mesh::{RwMeshCoordinateSystemConverter, RwMeshFaceIterator};
use martensite_tkxcaf::xcaf_doc::{XCAFDocColorType, XCAFDocDocumentTool, XCAFDocShapeTool};
use martensite_tkxcaf::xcaf_prs::{
    XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_ONLY_LEAF_NODES, XCAFPrsDocumentExplorer, XCAFPrsStyle,
};
use std::path::{Path, PathBuf};

fn file_in_temp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("tkdeply-{}-{name}", std::process::id()))
}

fn name_of(path: &Path) -> AsciiString {
    AsciiString::new(path.to_str().unwrap())
}

fn meshed_box(origin: f64, size: f64) -> Shape {
    let (low, high) = (
        Point3::new(origin, 0.0, 0.0),
        Point3::new(origin + size, size, size),
    );
    let solid: Shape = BRepPrimAPIMakeBox::from_points(low, high)
        .unwrap()
        .solid()
        .unwrap()
        .into_shape();
    BRepMeshIncrementalMesh::with_shape(&solid, 0.1, false, 0.5, false).unwrap();
    solid
}

fn new_document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::new("BinXCAF")).unwrap()
}

fn document_of(shapes: &[&Shape]) -> Handle<TDocStdDocument> {
    let document = new_document();
    let tool = XCAFDocDocumentTool::shape_tool(&document.get().unwrap().main().unwrap()).unwrap();
    for shape in shapes {
        tool.get().unwrap().add_shape(shape, true, true).unwrap();
    }
    document
}

fn free_shapes(document: &Handle<TDocStdDocument>) -> Vec<TdfLabel> {
    let tool = XCAFDocDocumentTool::shape_tool(&document.get().unwrap().main().unwrap()).unwrap();
    tool.get()
        .map_or(Ok(Vec::new()), XCAFDocShapeTool::get_free_shapes)
        .unwrap()
}

/// Writes with a configured writer and gives back the text.
fn written(
    name: &str,
    document: &Handle<TDocStdDocument>,
    configure: fn(&mut RwPlyCafWriter),
) -> String {
    let path = file_in_temp(name);
    let mut writer = RwPlyCafWriter::new(&name_of(&path));
    configure(&mut writer);
    assert!(
        writer
            .perform(
                document,
                &IndexedDataMap::default(),
                &ProgressRange::default()
            )
            .unwrap()
    );
    let text = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    text
}

fn after_header(text: &str) -> Vec<&str> {
    let (_, records) = text.split_once("end_header\n").unwrap();
    records.lines().collect()
}

/// The last word of each element line: the surface id.
fn surface_ids(text: &str, nb_vertices: usize) -> Vec<i32> {
    after_header(text)[nb_vertices..]
        .iter()
        .map(|line| line.rsplit(' ').next().unwrap().parse().unwrap())
        .collect()
}

#[test]
fn a_meshed_box_writes_its_counts_and_every_vertex_with_a_unit_normal_and_white() {
    let document = document_of(&[&meshed_box(0.0, 1.0)]);
    let text = written("box.ply", &document, |_| {});
    assert!(text.contains("element vertex 24\n"), "{text}");
    assert!(
        text.contains(
            "element face 12\nproperty list uchar uint vertex_indices\nproperty uint SurfaceID\n"
        ),
        "{text}"
    );
    let records = after_header(&text);
    assert_eq!(records.len(), 36);
    for line in &records[..24] {
        let words: Vec<f64> = line.split(' ').map(|word| word.parse().unwrap()).collect();
        assert_eq!(words.len(), 9, "{line}");
        let length = words[3] * words[3] + words[4] * words[4] + words[5] * words[5];
        assert!((length - 1.0).abs() < 1.0e-5, "{line}");
        assert_eq!(words[6..], [255.0, 255.0, 255.0], "{line}");
        assert!(
            words[..3]
                .iter()
                .all(|value| *value == 0.0 || *value == 1.0),
            "{line}"
        );
    }
    for line in &records[24..] {
        let words: Vec<i32> = line.split(' ').map(|word| word.parse().unwrap()).collect();
        assert_eq!((words.len(), words[0], words[4]), (5, 3, 1), "{line}");
        assert!(
            words[1..4].iter().all(|index| (0..24).contains(index)),
            "{line}"
        );
    }
}

#[test]
fn each_part_has_its_id_and_the_indices_of_the_second_follow_the_first() {
    let (first, second) = (meshed_box(0.0, 1.0), meshed_box(5.0, 1.0));
    let document = document_of(&[&first, &second]);
    let text = written("parts.ply", &document, |_| {});
    assert!(
        text.contains("element vertex 48\n") && text.contains("element face 24\n"),
        "{text}"
    );
    let ids = surface_ids(&text, 48);
    assert_eq!(ids[..12], [1; 12]);
    assert_eq!(ids[12..], [2; 12]);
    let records = after_header(&text);
    let index_of = |line: &str| -> i32 { line.split(' ').nth(1).unwrap().parse().unwrap() };
    assert!(records[48..60].iter().all(|line| index_of(line) < 24));
    assert!(records[60..].iter().all(|line| index_of(line) >= 24));
}

#[test]
fn face_ids_count_the_faces_across_parts() {
    let document = document_of(&[&meshed_box(0.0, 1.0), &meshed_box(5.0, 1.0)]);
    let text = written("faces.ply", &document, |writer| writer.set_face_id(true));
    let ids = surface_ids(&text, 48);
    let expected: Vec<i32> = (1..=12).flat_map(|face| [face, face]).collect();
    assert_eq!(ids, expected);
}

#[test]
fn optional_attributes_leave_the_file_when_turned_off() {
    let document = document_of(&[&meshed_box(0.0, 1.0)]);
    let text = written("plain.ply", &document, |writer| {
        writer.set_normals(false);
        writer.set_colors(false);
        writer.set_part_id(false);
    });
    assert!(
        !text.contains("nx") && !text.contains("red") && !text.contains("SurfaceID"),
        "{text}"
    );
    let records = after_header(&text);
    assert_eq!(records[0].split(' ').count(), 3);
    assert_eq!(records[24].split(' ').count(), 4);
}

#[test]
fn texture_coordinates_and_double_positions_are_declared_and_written() {
    let document = document_of(&[&meshed_box(0.0, 1.0)]);
    let text = written("uv.ply", &document, |writer| {
        writer.set_tex_coords(true);
        writer.set_double_precision(true);
    });
    assert!(
        text.contains("property double x\n")
            && text.contains("property float s\nproperty float t\n"),
        "{text}"
    );
    assert_eq!(after_header(&text)[0].split(' ').count(), 11);
}

#[test]
fn a_color_of_the_document_is_written_truncated_to_bytes() {
    let solid = meshed_box(0.0, 1.0);
    let document = document_of(&[&solid]);
    let colors = XCAFDocDocumentTool::color_tool(&document.get().unwrap().main().unwrap()).unwrap();
    let color = ColorRgba::from_rgb(Color::new(0.5, 0.25, 1.0, ColorSpace::Rgb).unwrap(), 1.0);
    assert!(
        colors
            .get()
            .unwrap()
            .set_shape_color(&solid, &color, XCAFDocColorType::ColorSurf)
            .unwrap()
    );
    let text = written("colored.ply", &document, |_| {});
    assert!(after_header(&text)[0].ends_with(" 127 63 255"), "{text}");
}

#[test]
fn a_face_without_a_color_has_white_bytes() {
    let solid = meshed_box(0.0, 1.0);
    let faces = RwMeshFaceIterator::with_shape(&solid, &XCAFPrsStyle::default()).unwrap();
    assert_eq!(face_color_bytes(&faces), Vec4::new(255, 255, 255, 255));
}

#[test]
fn nothing_to_write_is_refused_and_writes_no_file() {
    let path = file_in_temp("nothing.ply");
    let information = IndexedDataMap::default();
    let progress = ProgressRange::default();
    let mut writer = RwPlyCafWriter::new(&name_of(&path));
    assert!(
        !writer
            .perform(&new_document(), &information, &progress)
            .unwrap()
    );
    let unmeshed: Shape =
        BRepPrimAPIMakeBox::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0))
            .unwrap()
            .solid()
            .unwrap()
            .into_shape();
    assert!(
        !writer
            .perform(&document_of(&[&unmeshed]), &information, &progress)
            .unwrap()
    );
    assert!(!path.exists());
    assert!(
        writer
            .perform(&Handle::default(), &information, &progress)
            .is_err()
    );
}

#[test]
fn a_file_that_cannot_be_created_is_refused() {
    let document = document_of(&[&meshed_box(0.0, 1.0)]);
    let mut writer =
        RwPlyCafWriter::new(&AsciiString::new("/nonexistent-folder-of-tkdeply/file.ply"));
    assert!(
        !writer
            .perform(
                &document,
                &IndexedDataMap::default(),
                &ProgressRange::default()
            )
            .unwrap()
    );
}

#[test]
fn a_label_filter_chooses_the_parts_and_an_empty_one_refuses() {
    let document = document_of(&[&meshed_box(0.0, 1.0), &meshed_box(5.0, 1.0)]);
    let roots = free_shapes(&document);
    let path = file_in_temp("filtered.ply");
    let mut writer = RwPlyCafWriter::new(&name_of(&path));
    let (information, progress) = (IndexedDataMap::default(), ProgressRange::default());
    let empty: Map<AsciiString> = Map::default();
    assert!(
        !writer
            .perform_roots(&document, &roots, Some(&empty), &information, &progress)
            .unwrap()
    );
    assert!(
        !writer
            .perform_roots(&document, &[], None, &information, &progress)
            .unwrap()
    );
    let explorer = XCAFPrsDocumentExplorer::with_roots(
        &document,
        &roots,
        XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_ONLY_LEAF_NODES,
        &XCAFPrsStyle::default(),
    )
    .unwrap();
    let mut filter: Map<AsciiString> = Map::default();
    filter.add(explorer.current().id.clone());
    assert!(
        writer
            .perform_roots(&document, &roots, Some(&filter), &information, &progress)
            .unwrap()
    );
    let text = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(
        text.contains("element vertex 24\n") && text.contains("element face 12\n"),
        "{text}"
    );
}

/// A user who stops the export at once.
struct ImpatientUser;

impl ProgressIndicatorHooks for ImpatientUser {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
    fn user_break(&self) -> bool {
        true
    }
}

#[test]
fn a_stopped_export_answers_false() {
    let document = document_of(&[&meshed_box(0.0, 1.0)]);
    let path = file_in_temp("stopped.ply");
    let indicator = ProgressIndicator::new(Box::new(ImpatientUser));
    let mut writer = RwPlyCafWriter::new(&name_of(&path));
    assert!(
        !writer
            .perform(&document, &IndexedDataMap::default(), &indicator.start())
            .unwrap()
    );
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn part_and_face_ids_rule_each_other_out_and_settings_read_back() {
    let mut writer = RwPlyCafWriter::new(&AsciiString::new("unused.ply"));
    assert!(writer.has_part_id() && !writer.has_face_id());
    writer.set_face_id(true);
    assert!(!writer.has_part_id() && writer.has_face_id());
    writer.set_part_id(true);
    assert!(writer.has_part_id() && !writer.has_face_id());
    assert!(writer.has_normals() && writer.has_colors() && !writer.has_tex_coords());
    assert!(!writer.is_double_precision());
    writer.set_colors(false);
    writer.set_normals(false);
    assert!(!writer.has_colors() && !writer.has_normals());
    let mut converter = RwMeshCoordinateSystemConverter::new();
    converter.set_input_length_unit(0.001);
    writer.set_coordinate_system_converter(&converter);
    assert_eq!(
        writer.coordinate_system_converter().input_length_unit(),
        0.001
    );
    writer
        .change_coordinate_system_converter()
        .set_input_length_unit(1.0);
    assert_eq!(
        writer.coordinate_system_converter().input_length_unit(),
        1.0
    );
    let style = XCAFPrsStyle::default();
    writer.set_default_style(&style);
    assert!(!writer.default_style().is_set_color_curv());
    assert_eq!(
        writer.dynamic_type().name(),
        RwPlyCafWriter::static_type().name()
    );
}

#[test]
fn the_length_unit_of_a_document_reaches_the_converter() {
    let document = document_of(&[&meshed_box(0.0, 1.0)]);
    XCAFDocDocumentTool::set_length_unit(&document, 0.01).unwrap();
    let path = file_in_temp("unit.ply");
    let mut writer = RwPlyCafWriter::new(&name_of(&path));
    assert!(
        writer
            .perform(
                &document,
                &IndexedDataMap::default(),
                &ProgressRange::default()
            )
            .unwrap()
    );
    std::fs::remove_file(&path).unwrap();
    assert_eq!(
        writer.coordinate_system_converter().input_length_unit(),
        0.01
    );
}

#[test]
fn a_face_is_counted_and_only_the_two_steps_write() {
    let solid = meshed_box(0.0, 1.0);
    let document = document_of(&[&solid]);
    let writer = RwPlyCafWriter::new(&AsciiString::new("unused.ply"));
    let faces = RwMeshFaceIterator::with_shape(&solid, &XCAFPrsStyle::default()).unwrap();
    assert!(!writer.to_skip_face_mesh(&faces));
    let (mut nb_nodes, mut nb_elems) = (1, 2);
    writer.add_face_info(&faces, &mut nb_nodes, &mut nb_elems);
    assert_eq!((nb_nodes, nb_elems), (5, 4));
    let roots = free_shapes(&document);
    let style = XCAFPrsStyle::default();
    let progress = ProgressRange::default();
    let mut scope = LazyProgressScope::new(&progress, "test", 100.0, 1.0, false);
    let mut unopened = RwPlyPlyWriterContext::new();
    let mut step = |number: i32| {
        writer
            .write_shape(
                &mut unopened,
                &mut scope,
                number,
                &roots[0],
                &Location::default(),
                &style,
            )
            .unwrap()
    };
    assert!(!step(0));
    assert!(!step(1));
    assert!(step(2));
}

#[test]
fn the_writer_moves_between_threads() {
    fn assert_send<T: Send>() {}
    assert_send::<RwPlyCafWriter>();
}
