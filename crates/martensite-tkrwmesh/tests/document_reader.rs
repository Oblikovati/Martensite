//! Filling a document from the shapes of a reader, with a reader that makes them by hand.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Face, Shape};
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::StandardTransient;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tklcaf::tdata_std::TDataStdNamedData;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::poly::{Triangle, Triangulation};
use martensite_tkrwmesh::rw_mesh::{
    RWMESH_CAF_READER_STATUS_EX_PARTIAL, RwMesh, RwMeshCafReader, RwMeshCafReaderBase,
    RwMeshCoordinateSystem as System, RwMeshCoordinateSystemConverter, RwMeshInputStream,
    RwMeshNodeAttributes,
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use std::io::Cursor;

fn triangle_face() -> Shape {
    let mesh = Triangulation::with_sizes(3, 1, false, false).unwrap();
    for (index, x) in [(1, 0.0), (2, 1.0), (3, 2.0)] {
        mesh.set_node(index, Point3::new(x, x * x, 0.0)).unwrap();
    }
    mesh.set_triangle(1, Triangle::new(1, 2, 3)).unwrap();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_with_triangulation(&mut face, &Handle::new(mesh))
        .unwrap();
    face.into()
}

fn compound_of(children: &[Shape]) -> Shape {
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for child in children {
        builder.add(&mut compound, child).unwrap();
    }
    compound.into()
}

/// A reader of the shapes it holds, which can report a partial read.
struct HandReader {
    transient: StandardTransient,
    base: RwMeshCafReaderBase,
    shapes: Vec<(Shape, &'static str)>,
    roots: Vec<Shape>,
    is_done: bool,
}

impl HandReader {
    fn new(roots: Vec<Shape>, named: Vec<(Shape, &'static str)>) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: RwMeshCafReaderBase::default(),
            shapes: named,
            roots,
            is_done: true,
        }
    }
}

impl_transient!(HandReader, "HandReader", dyn RwMeshCafReader, transient, interfaces: [dyn RwMeshCafReader]);

impl RwMeshCafReader for HandReader {
    fn caf_base(&self) -> &RwMeshCafReaderBase {
        &self.base
    }
    fn caf_base_mut(&mut self) -> &mut RwMeshCafReaderBase {
        &mut self.base
    }
    fn perform_mesh(
        &mut self,
        _stream: &mut dyn RwMeshInputStream,
        _file: &AsciiString,
        _progress: &ProgressRange<'_>,
        to_probe: bool,
    ) -> Result<bool, Failure> {
        if to_probe {
            self.base
                .external_files
                .add(AsciiString::new("texture.png"));
            self.base
                .metadata
                .add(AsciiString::new("Comments"), AsciiString::new("probe"));
            return Ok(self.is_done);
        }
        for (shape, name) in &self.shapes {
            let attributes = RwMeshNodeAttributes {
                name: AsciiString::new(name),
                ..Default::default()
            };
            self.base.attrib_map.bind(shape.clone(), attributes);
        }
        self.base.root_shapes = self.roots.clone();
        Ok(self.is_done)
    }
}

fn new_document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::new("BinXCAF")).unwrap()
}

fn free_shapes(document: &Handle<TDocStdDocument>) -> Vec<martensite_tklcaf::tdf::TdfLabel> {
    let main = document.get().unwrap().main().unwrap();
    XCAFDocDocumentTool::shape_tool(&main)
        .unwrap()
        .get()
        .unwrap()
        .get_free_shapes()
        .unwrap()
}

fn perform(reader: &mut HandReader) -> bool {
    let mut stream = Cursor::new(Vec::new());
    reader
        .perform_stream(
            &mut stream,
            &ProgressRange::default(),
            &AsciiString::new("dir/model.mesh"),
        )
        .unwrap()
}

#[test]
fn settings_reach_the_converter() {
    let mut reader = HandReader::new(Vec::new(), Vec::new());
    reader.set_system_length_unit(0.001);
    reader.set_system_coordinate_system_kind(System::Z_UP);
    reader.set_file_length_unit(1.0);
    reader.set_file_coordinate_system_kind(System::Y_UP);
    assert_eq!(
        (reader.system_length_unit(), reader.file_length_unit()),
        (0.001, 1.0)
    );
    assert!(reader.has_system_coordinate_system() && reader.has_file_coordinate_system());
    let frame = *reader.file_coordinate_system();
    reader.set_system_coordinate_system(frame);
    reader.set_file_coordinate_system(*reader.system_coordinate_system());
    let converter = RwMeshCoordinateSystemConverter::new();
    reader.set_coordinate_system_converter(&converter);
    assert!(reader.coordinate_system_converter().is_empty());
    reader.set_memory_limit_mib(16);
    assert_eq!(reader.memory_limit_mib(), 16);
    assert!(reader.to_fill_incomplete_document());
    reader.set_fill_incomplete_document(false);
    assert!(!reader.to_fill_incomplete_document());
    reader.set_root_prefix(&AsciiString::new("p/"));
    assert_eq!(reader.root_prefix().as_bytes(), b"p/");
    assert!(reader.document().is_null());
}

#[test]
fn a_document_takes_a_compound_of_named_groups_as_an_assembly_named_after_the_file() {
    let (first, second) = (triangle_face(), triangle_face());
    let root = compound_of(&[first.clone(), second.clone()]);
    let mut reader = HandReader::new(vec![root], vec![(first, "One"), (second, "")]);
    let document = new_document();
    reader.set_system_length_unit(0.001);
    reader.set_document(&document).unwrap();
    reader.set_root_prefix(&AsciiString::new("prefix/"));
    assert!(perform(&mut reader));
    let labels = free_shapes(&document);
    assert_eq!(labels.len(), 1);
    assert!(XCAFDocShapeTool::is_assembly(&labels[0]).unwrap());
    assert_eq!(
        RwMesh::read_name_attribute(&labels[0]).unwrap().as_bytes(),
        b"prefix/model.mesh"
    );
    let mut components = Vec::new();
    XCAFDocShapeTool::get_components(&labels[0], &mut components, false).unwrap();
    assert_eq!(components.len(), 2);
    assert_eq!(
        RwMesh::read_name_attribute(&components[0])
            .unwrap()
            .as_bytes(),
        b"One"
    );
    assert_eq!(
        RwMesh::read_name_attribute(&components[1])
            .unwrap()
            .as_bytes(),
        b"Face"
    );
    let mut unit = 0.0;
    assert!(XCAFDocDocumentTool::get_length_unit(&document, &mut unit).unwrap());
    assert_eq!(unit, 0.001);
    assert!(!reader.single_shape().unwrap().is_null());
}

#[test]
fn a_compound_of_unnamed_faces_stays_a_shape_with_sub_shape_labels() {
    let first = triangle_face();
    let root = compound_of(&[first.clone(), triangle_face()]);
    let mut reader = HandReader::new(vec![root.clone()], vec![(first, "")]);
    let document = new_document();
    reader.set_document(&document).unwrap();
    assert!(perform(&mut reader));
    let labels = free_shapes(&document);
    assert!(!XCAFDocShapeTool::is_assembly(&labels[0]).unwrap());
    let mut subs = Vec::new();
    XCAFDocShapeTool::get_sub_shapes(&labels[0], &mut subs).unwrap();
    assert_eq!(subs.len(), 1, "only the face with attributes has a label");
}

#[test]
fn several_roots_read_as_one_compound_and_none_as_null() {
    let mut reader = HandReader::new(vec![triangle_face(), triangle_face()], Vec::new());
    assert!(perform(&mut reader), "no document is fine");
    assert_eq!(reader.single_shape().unwrap().nb_children(), 2);
    let empty = HandReader::new(Vec::new(), Vec::new());
    assert!(empty.single_shape().unwrap().is_null());
}

#[test]
fn a_partial_read_fills_only_when_large_and_allowed() {
    let mut small = HandReader::new(vec![triangle_face()], Vec::new());
    small.is_done = false;
    assert!(!perform(&mut small), "under 100 elements fails");
    assert_eq!(small.extra_status(), RWMESH_CAF_READER_STATUS_EX_PARTIAL);
    let mut refused = HandReader::new(vec![triangle_face()], Vec::new());
    refused.is_done = false;
    refused.set_fill_incomplete_document(false);
    assert!(!perform(&mut refused));
    assert_eq!(refused.extra_status(), 0);
}

#[test]
fn a_probe_only_collects_and_a_missing_file_is_an_empty_stream() {
    let mut reader = HandReader::new(vec![triangle_face()], Vec::new());
    let progress = ProgressRange::default();
    assert!(
        reader
            .probe_header(&AsciiString::new("/nowhere/at/all.mesh"), &progress)
            .unwrap()
    );
    let mut stream = Cursor::new(Vec::new());
    assert!(
        reader
            .probe_header_stream(&mut stream, &AsciiString::new("a"), &progress)
            .unwrap()
    );
    assert_eq!(reader.external_files().len(), 1);
    assert_eq!(reader.metadata().len(), 1);
    assert!(reader.caf_base().root_shapes.is_empty());
    assert!(
        reader
            .perform(&AsciiString::new("/nowhere/at/all.mesh"), &progress)
            .unwrap()
    );
    assert!(
        reader
            .perform_mesh_file(&AsciiString::new("/nowhere/at/all.mesh"), &progress, false)
            .unwrap()
    );
}

#[test]
fn styles_and_named_data_go_to_the_labels() {
    let face = triangle_face();
    let mut attributes = RwMeshNodeAttributes {
        name: AsciiString::new("Colored"),
        ..Default::default()
    };
    attributes
        .style
        .set_color_surf(Color::named(NamedColor::RED));
    attributes
        .style
        .set_color_curv(Color::named(NamedColor::BLUE));
    attributes.named_data = Handle::new(TDataStdNamedData::new());
    let mut reader = HandReader::new(vec![face.clone()], Vec::new());
    reader.base.attrib_map.bind(face, attributes);
    let document = new_document();
    reader.set_document(&document).unwrap();
    let mut stream = Cursor::new(Vec::new());
    let progress = ProgressRange::default();
    assert!(
        reader
            .perform_stream(&mut stream, &progress, &AsciiString::new("colored.mesh"))
            .unwrap()
    );
    let labels = free_shapes(&document);
    assert_eq!(
        RwMesh::read_name_attribute(&labels[0]).unwrap().as_bytes(),
        b"Colored"
    );
    let colors = XCAFDocDocumentTool::color_tool(&document.get().unwrap().main().unwrap()).unwrap();
    assert!(
        colors
            .get()
            .unwrap()
            .is_set(
                &labels[0],
                martensite_tkxcaf::xcaf_doc::XCAFDocColorType::ColorSurf
            )
            .unwrap()
    );
    assert!(
        labels[0]
            .is_attribute(&TDataStdNamedData::get_id())
            .unwrap()
    );
}
