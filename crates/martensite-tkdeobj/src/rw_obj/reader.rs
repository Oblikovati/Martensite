//! `RwObjReader`: the procedure reading an OBJ file into any data model.
use super::{RwObjMaterial, RwObjSubMesh, RwObjSubMeshReason};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::{DataMap, IndexedMap, Vec2, Vec3};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Point3;
use martensite_tkrwmesh::rw_mesh::{RwMeshCoordinateSystemConverter, RwMeshInputStream};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Cursor};

/// Vertex positions with single or double floating point precision;
/// `RwObjVectorOfVertices::default()`.
#[derive(Clone, Debug, Default)]
pub struct RwObjVectorOfVertices {
    points: Vec<Point3>,
    floats: Vec<Vec3<f32>>,
    is_single_precision: bool,
}

impl RwObjVectorOfVertices {
    /// True with single precision, false by default; `vertices.is_single_precision()`.
    pub fn is_single_precision(&self) -> bool {
        self.is_single_precision
    }
    /// Sets the precision, emptying the vector; `vertices.set_single_precision(true)`.
    pub fn set_single_precision(&mut self, is_single_precision: bool) {
        self.is_single_precision = is_single_precision;
        self.reset();
    }
    /// Empties the vector; `vertices.reset()`.
    pub fn reset(&mut self) {
        self.points.clear();
        self.floats.clear();
    }
    /// The lower index; `vertices.lower()`.
    pub fn lower(&self) -> i32 {
        0
    }
    /// The upper index, -1 when empty; `vertices.upper()`.
    pub fn upper(&self) -> i32 {
        (if self.is_single_precision {
            self.floats.len()
        } else {
            self.points.len()
        }) as i32
            - 1
    }
    /// The point at an index in `lower()..=upper()`; `vertices.value(0)`.
    pub fn value(&self, index: i32) -> Point3 {
        if self.is_single_precision {
            let point = self.floats[index as usize];
            return Point3::new(f64::from(point.x), f64::from(point.y), f64::from(point.z));
        }
        self.points[index as usize]
    }
    /// Appends a point; `vertices.append(point)`.
    pub fn append(&mut self, point: Point3) {
        if self.is_single_precision {
            self.floats.push(Vec3::new(
                point.x() as f32,
                point.y() as f32,
                point.z() as f32,
            ));
        } else {
            self.points.push(point);
        }
    }
}

/// The state of a reader; `RwObjReaderBase::default()`.
pub struct RwObjReaderBase {
    /// The external file references.
    pub external_files: IndexedMap<AsciiString>,
    /// The header comments of the file.
    pub file_comments: AsciiString,
    /// The folder of the file.
    pub folder: AsciiString,
    /// The conversion applied to what is read.
    pub cs_trsf: RwMeshCoordinateSystemConverter,
    /// The memory limit in bytes; the largest value for none.
    pub mem_limit_bytes: usize,
    /// The memory estimated in use, in bytes.
    pub mem_estim: usize,
    /// The number of lines parsed.
    pub nb_lines: i32,
    /// The number of nodes probed.
    pub nb_probe_nodes: i32,
    /// The number of polygons probed.
    pub nb_probe_elems: i32,
    /// The number of polygons of five nodes or more.
    pub nb_elems_big: i32,
    /// Whether a syntax error or the memory limit stopped the mesh.
    pub to_abort: bool,
    /// The vertices so far.
    pub obj_verts: RwObjVectorOfVertices,
    /// The texture coordinates so far.
    pub obj_verts_uv: Vec<Vec2<f32>>,
    /// The normals so far.
    pub obj_norms: Vec<Vec3<f32>>,
    /// The node of each position, texture coordinate and normal index set.
    pub packed_indices: HashMap<[i32; 3], i32>,
    /// The materials known.
    pub materials: DataMap<AsciiString, RwObjMaterial>,
    /// The active sub-mesh definition.
    pub active_sub_mesh: RwObjSubMesh,
    /// The nodes of the current element.
    pub curr_elem: Vec<i32>,
}

impl Default for RwObjReaderBase {
    fn default() -> Self {
        Self {
            external_files: IndexedMap::default(),
            file_comments: AsciiString::default(),
            folder: AsciiString::default(),
            cs_trsf: RwMeshCoordinateSystemConverter::new(),
            mem_limit_bytes: usize::MAX,
            mem_estim: 0,
            nb_lines: 0,
            nb_probe_nodes: 0,
            nb_probe_elems: 0,
            nb_elems_big: 0,
            to_abort: false,
            obj_verts: RwObjVectorOfVertices::default(),
            obj_verts_uv: Vec::new(),
            obj_norms: Vec::new(),
            packed_indices: HashMap::new(),
            materials: DataMap::default(),
            active_sub_mesh: RwObjSubMesh::default(),
            curr_elem: Vec::new(),
        }
    }
}

/// A missing file reads as an empty stream, which [`RwObjReader::read_stream`] reports as empty.
fn open_stream(file: &AsciiString) -> Box<dyn RwMeshInputStream> {
    match File::open(String::from_utf8_lossy(file.as_bytes()).into_owned()) {
        Ok(opened) => Box::new(BufReader::new(opened)),
        Err(_) => Box::new(Cursor::new(Vec::new())),
    }
}

/// An OBJ reader, not bound to a data structure: implement the callbacks and call [`Self::read`].
///
/// ```no_run
/// use martensite_tkdeobj::rw_obj::{RwObjReader as _, RwObjTriangulationReader};
/// use martensite_tkernel::message::ProgressRange;
/// use martensite_tkernel::tcollection::AsciiString;
/// let mut reader = RwObjTriangulationReader::new();
/// reader.set_create_shapes(false);
/// let progress = ProgressRange::default();
/// assert!(reader.read(&AsciiString::new("cube.obj"), &progress).unwrap());
/// assert!(reader.get_triangulation().unwrap().is_some());
/// ```
pub trait RwObjReader {
    /// The reader state; `reader.obj_base()`.
    fn obj_base(&self) -> &RwObjReaderBase;
    /// The mutable reader state; `reader.obj_base_mut()`.
    fn obj_base_mut(&mut self) -> &mut RwObjReaderBase;
    /// Called with each new group, material or object; true when a new sub-mesh starts here;
    /// `reader.add_mesh(&mesh, RwObjSubMeshReason::NewGroup)?`.
    fn add_mesh(
        &mut self,
        mesh: &RwObjSubMesh,
        reason: RwObjSubMeshReason,
    ) -> Result<bool, Failure>;
    /// The position of a sub-mesh node from `add_node`; `reader.get_node(1)`.
    fn get_node(&self, index: i32) -> Point3;
    /// Creates a node and returns its identifier; `reader.add_node(point)`.
    fn add_node(&mut self, point: Point3) -> i32;
    /// Sets the normal of a node; `reader.set_node_normal(1, normal)`.
    fn set_node_normal(&mut self, index: i32, normal: Vec3<f32>);
    /// Sets the texture coordinates of a node; `reader.set_node_uv(1, uv)`.
    fn set_node_uv(&mut self, index: i32, uv: Vec2<f32>);
    /// Creates a triangle, or a quad when the last node is not -1; `reader.add_element(1, 2, 3, -1)`.
    fn add_element(&mut self, n1: i32, n2: i32, n3: i32, n4: i32);

    /// Reads a named file; false on an error or a user break; `reader.read(&file, &progress)?`.
    fn read(&mut self, file: &AsciiString, progress: &ProgressRange<'_>) -> Result<bool, Failure> {
        self.read_stream(&mut open_stream(file), file, progress)
    }
    /// Reads a stream, with the file's name for its companions;
    /// `reader.read_stream(&mut stream, &file, &progress)?`.
    fn read_stream(
        &mut self,
        stream: &mut dyn RwMeshInputStream,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        super::reader_lines::read_all(self, stream, file, progress, false)
    }
    /// Collects comments, references and counts of a named file; `reader.probe(&file, &progress)?`.
    fn probe(&mut self, file: &AsciiString, progress: &ProgressRange<'_>) -> Result<bool, Failure> {
        self.probe_stream(&mut open_stream(file), file, progress)
    }
    /// Collects comments, references and counts of a stream;
    /// `reader.probe_stream(&mut stream, &file, &progress)?`.
    fn probe_stream(
        &mut self,
        stream: &mut dyn RwMeshInputStream,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        super::reader_lines::read_all(self, stream, file, progress, true)
    }
    /// The comments at the head of the file; `reader.file_comments()`.
    fn file_comments(&self) -> &AsciiString {
        &self.obj_base().file_comments
    }
    /// The external file references; `reader.external_files()`.
    fn external_files(&self) -> &IndexedMap<AsciiString> {
        &self.obj_base().external_files
    }
    /// The number of nodes probed; `reader.nb_probe_nodes()`.
    fn nb_probe_nodes(&self) -> i32 {
        self.obj_base().nb_probe_nodes
    }
    /// The number of polygons probed; `reader.nb_probe_elems()`.
    fn nb_probe_elems(&self) -> i32 {
        self.obj_base().nb_probe_elems
    }
    /// The memory limit in bytes, the largest value for none; `reader.memory_limit()`.
    fn memory_limit(&self) -> usize {
        self.obj_base().mem_limit_bytes
    }
    /// Sets the memory limit in bytes, which aborts the import when reached;
    /// `reader.set_memory_limit(1 << 30)`.
    fn set_memory_limit(&mut self, limit: usize) {
        self.obj_base_mut().mem_limit_bytes = limit;
    }
    /// The conversion applied, none by default; `reader.transformation()`.
    fn transformation(&self) -> &RwMeshCoordinateSystemConverter {
        &self.obj_base().cs_trsf
    }
    /// Sets the conversion applied; `reader.set_transformation(&converter)`.
    fn set_transformation(&mut self, converter: &RwMeshCoordinateSystemConverter) {
        self.obj_base_mut().cs_trsf = converter.clone();
    }
    /// True with single precision vertices, false by default; `reader.is_single_precision()`.
    fn is_single_precision(&self) -> bool {
        self.obj_base().obj_verts.is_single_precision()
    }
    /// Sets the precision of vertices; `reader.set_single_precision(true)`.
    fn set_single_precision(&mut self, is_single_precision: bool) {
        self.obj_base_mut()
            .obj_verts
            .set_single_precision(is_single_precision);
    }
}
