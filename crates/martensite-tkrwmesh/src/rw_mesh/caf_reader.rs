//! `RwMeshCafReader`: reading a mesh file into shapes and an exchange document.
use super::{RwMeshCoordinateSystem, RwMeshCoordinateSystemConverter, RwMeshNodeAttributes};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Compound, Shape};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::message::{Message, ProgressRange};
use martensite_tkernel::ncollection::{DataMap, IndexedDataMap, IndexedMap};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, Transient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Axis3;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocDocumentTool, XCAFDocShapeTool, XCAFDocVisMaterialTool,
};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek};
use std::time::Instant;

/// No extended status; `RWMESH_CAF_READER_STATUS_EX_NONE`.
pub const RWMESH_CAF_READER_STATUS_EX_NONE: i32 = 0;
/// A partial read, from an unexpected end of file, a syntax error or the memory limit.
pub const RWMESH_CAF_READER_STATUS_EX_PARTIAL: i32 = 0x01;

/// The mesh file input: a stream that can report its length.
pub trait RwMeshInputStream: Read + Seek {}
impl<T: Read + Seek> RwMeshInputStream for T {}

/// The tools that fill a document, with the labels already made for a shape.
#[derive(Default)]
pub struct RwMeshCafDocumentTools {
    /// The shape tool of the document.
    pub shape_tool: Handle<XCAFDocShapeTool>,
    /// The color tool of the document.
    pub color_tool: Handle<XCAFDocColorTool>,
    /// The material tool of the document.
    pub vis_material_tool: Handle<XCAFDocVisMaterialTool>,
    /// The component label of each unlocated shape.
    pub component_map: DataMap<Shape, TdfLabel, ShapeMapHasher>,
    /// The assembly label of each original shape.
    pub original_shape_map: DataMap<Shape, TdfLabel, ShapeMapHasher>,
}

/// The state of a reader; `RwMeshCafReaderBase::default()`.
pub struct RwMeshCafReaderBase {
    /// The target document.
    pub xde_doc: Handle<TDocStdDocument>,
    /// The metadata of the file.
    pub metadata: IndexedDataMap<AsciiString, AsciiString>,
    /// The complementary files: textures, data and the like.
    pub external_files: IndexedMap<AsciiString>,
    /// The prefix for generated root names.
    pub root_prefix: AsciiString,
    /// The result root shapes.
    pub root_shapes: Vec<Shape>,
    /// The attributes of each shape.
    pub attrib_map: DataMap<Shape, RwMeshNodeAttributes, ShapeMapHasher>,
    /// The axis and unit conversion.
    pub coord_sys_converter: RwMeshCoordinateSystemConverter,
    /// Whether the document is filled from the shapes.
    pub to_fill_doc: bool,
    /// Whether a partial read still fills the document.
    pub to_fill_incomplete: bool,
    /// The memory limit in MiB, -1 for none.
    pub memory_limit_mib: i32,
    /// The extended status bits.
    pub extra_status: i32,
}

impl Default for RwMeshCafReaderBase {
    fn default() -> Self {
        Self {
            xde_doc: Handle::default(),
            metadata: IndexedDataMap::default(),
            external_files: IndexedMap::default(),
            root_prefix: AsciiString::default(),
            root_shapes: Vec::new(),
            attrib_map: DataMap::default(),
            coord_sys_converter: RwMeshCoordinateSystemConverter::new(),
            to_fill_doc: true,
            to_fill_incomplete: true,
            memory_limit_mib: -1,
            extra_status: RWMESH_CAF_READER_STATUS_EX_NONE,
        }
    }
}

fn path_text(file: &AsciiString) -> String {
    String::from_utf8_lossy(file.as_bytes()).into_owned()
}

/// A failed open reads as an empty stream, which the format reader reports as a missing file.
fn open_stream(file: &AsciiString) -> Box<dyn RwMeshInputStream> {
    match File::open(path_text(file)) {
        Ok(opened) => Box::new(BufReader::new(opened)),
        Err(_) => Box::new(Cursor::new(Vec::new())),
    }
}

fn mesh_counts(roots: &[Shape]) -> Result<(i64, i64), Failure> {
    let (mut nodes, mut elements) = (0, 0);
    for root in roots {
        let mut faces = Explorer::new(root, ShapeEnum::Face, ShapeEnum::Shape)?;
        while faces.more() {
            let (mesh, _) =
                BRepTool::triangulation(&topods::face(faces.current())?, mesh_purpose::NONE)?;
            if let Some(mesh) = mesh.get() {
                nodes += i64::from(mesh.nb_nodes());
                elements += i64::from(mesh.nb_triangles());
            }
            faces.next()?;
        }
    }
    Ok((nodes, elements))
}

impl_transient_interface!(dyn RwMeshCafReader, "RwMeshCafReader", StandardTransient);

/// A mesh reader into shapes and a document; a format implements [`Self::perform_mesh`].
pub trait RwMeshCafReader: Transient {
    /// The reader state; `reader.caf_base()`.
    fn caf_base(&self) -> &RwMeshCafReaderBase;
    /// The mutable reader state; `reader.caf_base_mut()`.
    fn caf_base_mut(&mut self) -> &mut RwMeshCafReaderBase;
    /// Reads the mesh from a stream, the part a format supplies;
    /// `reader.perform_mesh(&mut stream, &file, &progress, false)?`.
    fn perform_mesh(
        &mut self,
        stream: &mut dyn RwMeshInputStream,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
        to_probe: bool,
    ) -> Result<bool, Failure>;

    /// Reads the mesh from a named file; `reader.perform_mesh_file(&file, &progress, false)?`.
    fn perform_mesh_file(
        &mut self,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
        to_probe: bool,
    ) -> Result<bool, Failure> {
        self.perform_mesh(&mut open_stream(file), file, progress, to_probe)
    }
    /// The target document; `reader.document()`.
    fn document(&self) -> &Handle<TDocStdDocument> {
        &self.caf_base().xde_doc
    }
    /// Sets the target document, taking its length unit as the system unit;
    /// `reader.set_document(&document)?`.
    fn set_document(&mut self, document: &Handle<TDocStdDocument>) -> Result<(), Failure> {
        self.caf_base_mut().xde_doc = document.clone();
        let mut scale = 1.0;
        if XCAFDocDocumentTool::get_length_unit(document, &mut scale)? {
            self.set_system_length_unit(scale);
        }
        Ok(())
    }
    /// The prefix for root names; `reader.root_prefix()`.
    fn root_prefix(&self) -> &AsciiString {
        &self.caf_base().root_prefix
    }
    /// Sets the prefix for root names; `reader.set_root_prefix(&prefix)`.
    fn set_root_prefix(&mut self, prefix: &AsciiString) {
        self.caf_base_mut().root_prefix = prefix.clone();
    }
    /// Whether a partial read fills the document, true by default;
    /// `reader.to_fill_incomplete_document()`.
    fn to_fill_incomplete_document(&self) -> bool {
        self.caf_base().to_fill_incomplete
    }
    /// Sets whether a partial read fills the document; `reader.set_fill_incomplete_document(false)`.
    fn set_fill_incomplete_document(&mut self, to_fill: bool) {
        self.caf_base_mut().to_fill_incomplete = to_fill;
    }
    /// The memory limit in MiB, -1 for none; `reader.memory_limit_mib()`.
    fn memory_limit_mib(&self) -> i32 {
        self.caf_base().memory_limit_mib
    }
    /// Sets the memory limit in MiB; `reader.set_memory_limit_mib(256)`.
    fn set_memory_limit_mib(&mut self, limit: i32) {
        self.caf_base_mut().memory_limit_mib = limit;
    }
    /// The axis and unit conversion; `reader.coordinate_system_converter()`.
    fn coordinate_system_converter(&self) -> &RwMeshCoordinateSystemConverter {
        &self.caf_base().coord_sys_converter
    }
    /// Sets the axis and unit conversion; `reader.set_coordinate_system_converter(&converter)`.
    fn set_coordinate_system_converter(&mut self, converter: &RwMeshCoordinateSystemConverter) {
        self.caf_base_mut().coord_sys_converter = converter.clone();
    }
    /// The unit to convert into, as a scale to meters; `reader.system_length_unit()`.
    fn system_length_unit(&self) -> f64 {
        self.caf_base().coord_sys_converter.output_length_unit()
    }
    /// Sets the unit to convert into; `reader.set_system_length_unit(0.001)`.
    fn set_system_length_unit(&mut self, units: f64) {
        self.caf_base_mut()
            .coord_sys_converter
            .set_output_length_unit(units);
    }
    /// True once a system frame is defined; `reader.has_system_coordinate_system()`.
    fn has_system_coordinate_system(&self) -> bool {
        self.caf_base()
            .coord_sys_converter
            .has_output_coordinate_system()
    }
    /// The system frame; `reader.system_coordinate_system()`.
    fn system_coordinate_system(&self) -> &Axis3 {
        self.caf_base()
            .coord_sys_converter
            .output_coordinate_system()
    }
    /// Sets the system frame; `reader.set_system_coordinate_system(frame)`.
    fn set_system_coordinate_system(&mut self, frame: Axis3) {
        self.caf_base_mut()
            .coord_sys_converter
            .set_output_coordinate_system(frame);
    }
    /// Sets the system convention; `reader.set_system_coordinate_system_kind(System::Z_UP)`.
    fn set_system_coordinate_system_kind(&mut self, system: RwMeshCoordinateSystem) {
        self.caf_base_mut()
            .coord_sys_converter
            .set_output_coordinate_system_kind(system);
    }
    /// The file unit as a scale to meters, -1 when undefined; `reader.file_length_unit()`.
    fn file_length_unit(&self) -> f64 {
        self.caf_base().coord_sys_converter.input_length_unit()
    }
    /// Sets the file unit; `reader.set_file_length_unit(0.001)`.
    fn set_file_length_unit(&mut self, units: f64) {
        self.caf_base_mut()
            .coord_sys_converter
            .set_input_length_unit(units);
    }
    /// True once a file frame is defined; `reader.has_file_coordinate_system()`.
    fn has_file_coordinate_system(&self) -> bool {
        self.caf_base()
            .coord_sys_converter
            .has_input_coordinate_system()
    }
    /// The file frame; `reader.file_coordinate_system()`.
    fn file_coordinate_system(&self) -> &Axis3 {
        self.caf_base()
            .coord_sys_converter
            .input_coordinate_system()
    }
    /// Sets the file frame; `reader.set_file_coordinate_system(frame)`.
    fn set_file_coordinate_system(&mut self, frame: Axis3) {
        self.caf_base_mut()
            .coord_sys_converter
            .set_input_coordinate_system(frame);
    }
    /// Sets the file convention; `reader.set_file_coordinate_system_kind(System::Y_UP)`.
    fn set_file_coordinate_system_kind(&mut self, system: RwMeshCoordinateSystem) {
        self.caf_base_mut()
            .coord_sys_converter
            .set_input_coordinate_system_kind(system);
    }
    /// Reads a named file; `reader.perform(&file, &progress)?`.
    fn perform(
        &mut self,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_file(file, progress, false)
    }
    /// Reads a stream; `reader.perform_stream(&mut stream, &progress, &file)?`.
    fn perform_stream(
        &mut self,
        stream: &mut dyn RwMeshInputStream,
        progress: &ProgressRange<'_>,
        file: &AsciiString,
    ) -> Result<bool, Failure> {
        self.perform_stream_with(stream, file, progress, false)
    }
    /// The extended status bits; `reader.extra_status()`.
    fn extra_status(&self) -> i32 {
        self.caf_base().extra_status
    }
    /// The result as one shape, a compound for several roots and null for none;
    /// `reader.single_shape()?`.
    fn single_shape(&self) -> Result<Shape, Failure> {
        let roots = &self.caf_base().root_shapes;
        if roots.len() <= 1 {
            return Ok(roots.first().cloned().unwrap_or_default());
        }
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        for root in roots {
            builder.add(&mut compound, root)?;
        }
        Ok(compound.into())
    }
    /// The complementary files; `reader.external_files()`.
    fn external_files(&self) -> &IndexedMap<AsciiString> {
        &self.caf_base().external_files
    }
    /// The metadata of the file; `reader.metadata()`.
    fn metadata(&self) -> &IndexedDataMap<AsciiString, AsciiString> {
        &self.caf_base().metadata
    }
    /// Collects the metadata and references of a named file; `reader.probe_header(&file, &progress)?`.
    fn probe_header(
        &mut self,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_file(file, progress, true)
    }
    /// Collects the metadata and references of a stream;
    /// `reader.probe_header_stream(&mut stream, &file, &progress)?`.
    fn probe_header_stream(
        &mut self,
        stream: &mut dyn RwMeshInputStream,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.perform_stream_with(stream, file, progress, true)
    }
    /// Opens a file and reads it; `reader.perform_file(&file, &progress, false)?`.
    fn perform_file(
        &mut self,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
        to_probe: bool,
    ) -> Result<bool, Failure> {
        self.perform_stream_with(&mut open_stream(file), file, progress, to_probe)
    }
    /// Reads and fills the document; a partial read of under 100 elements fails;
    /// `reader.perform_stream_with(&mut stream, &file, &progress, false)?`.
    fn perform_stream_with(
        &mut self,
        stream: &mut dyn RwMeshInputStream,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
        to_probe: bool,
    ) -> Result<bool, Failure> {
        let new_roots_lower = self.free_shape_count()? + 1;
        let started = Instant::now();
        let is_done = self.perform_mesh(stream, file, progress, to_probe)?;
        if to_probe || progress.user_break() {
            return Ok(is_done);
        }
        if !is_done {
            if !self.caf_base().to_fill_incomplete {
                return Ok(false);
            }
            self.caf_base_mut().extra_status |= RWMESH_CAF_READER_STATUS_EX_PARTIAL;
        }
        let (nodes, elements) = mesh_counts(&self.caf_base().root_shapes)?;
        if !is_done && elements < 100 {
            return Ok(false);
        }
        self.fill_document()?;
        self.generate_names(file, new_roots_lower, false)?;
        send_read_info(file, nodes, elements, is_done, started);
        Ok(true)
    }
    /// The number of free shapes of the document, zero without one; `reader.free_shape_count()?`.
    fn free_shape_count(&self) -> Result<i32, Failure> {
        let Some(document) = self.caf_base().xde_doc.get() else {
            return Ok(0);
        };
        let tool = XCAFDocDocumentTool::shape_tool(&document.main()?)?;
        Ok(tool
            .get()
            .map_or(Ok(Vec::new()), XCAFDocShapeTool::get_free_shapes)?
            .len() as i32)
    }
    /// Fills the document with the root shapes; `reader.fill_document()?`.
    fn fill_document(&mut self) -> Result<(), Failure> {
        self.caf_base_mut().fill_document_from_roots()
    }
    /// Names roots from the root index on; `reader.generate_names(&file, 1, false)?`.
    fn generate_names(
        &self,
        file: &AsciiString,
        root_lower: i32,
        with_sub_labels: bool,
    ) -> Result<(), Failure> {
        self.caf_base()
            .generate_root_names(file, root_lower, with_sub_labels)
    }
}

fn send_read_info(file: &AsciiString, nodes: i64, elements: i64, is_done: bool, started: Instant) {
    let partly = if is_done { "" } else { "PARTIALLY " };
    let text = format!(
        "Mesh {}\n[{nodes} nodes] [{elements} 2d elements]\n[{partly}read in {} s]",
        path_text(file),
        started.elapsed().as_secs_f64()
    );
    Message::send_info_text(&AsciiString::new(&text));
}

/// The file name of a path; `file_name_of(&file)`.
pub(super) fn file_name_of(file: &AsciiString) -> AsciiString {
    CadPath::folder_and_file_from_path(file.as_bytes()).1
}
