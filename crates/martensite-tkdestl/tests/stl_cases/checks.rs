//! The checks the cases make: inputs, shape counts, mesh counts, area and validity.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkdestl::{
    rw_stl::{DEFAULT_MERGE_ANGLE, RwStl},
    stl_api::{StlApiReader, StlApiWriter},
};
use martensite_tkernel::{message::ProgressRange, tcollection::AsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::{Triangulation, mesh_purpose};
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};
use std::path::PathBuf;

/// A public input of `external/martensite-test-data`; `input("dataset-7.9.0/others/b.stl")`.
pub fn input(relative: &str) -> AsciiString {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data")
        .join(relative);
    AsciiString::new(path.to_str().unwrap())
}

/// A file of this case in the temporary folder, deleted with its guard.
pub struct CaseFile(pub PathBuf);
impl CaseFile {
    pub fn new(name: &str) -> Self {
        Self(std::env::temp_dir().join(format!("tkdestl-{}-{name}", std::process::id())))
    }
    pub fn name(&self) -> AsciiString {
        AsciiString::new(self.0.to_str().unwrap())
    }
    pub fn size(&self) -> u64 {
        std::fs::metadata(&self.0).unwrap().len()
    }
}
impl Drop for CaseFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// The deprecated read reaches this reader.
pub fn read_faces(file: &AsciiString) -> Shape {
    let mut shape = Shape::default();
    assert!(StlApiReader.read_file(&mut shape, file).unwrap());
    shape
}

/// One face on the mesh read.
pub fn read_mesh_face(file: &AsciiString, angle_degrees: Option<f64>) -> Shape {
    let angle = angle_degrees.map_or(DEFAULT_MERGE_ANGLE, |angle| {
        angle * std::f64::consts::PI / 180.0
    });
    let mesh = RwStl::read_file_with_angle(file, angle, &ProgressRange::default()).unwrap();
    crate::support::shapes::face_of_mesh(&mesh)
}

/// `mode` 0 writes text, any other value binary.
pub fn write_stl(shape: &Shape, file: &AsciiString, mode: i32) {
    let mut writer = StlApiWriter::new();
    *writer.ascii_mode() = mode == 0;
    assert!(
        writer
            .write_file(shape, file, &ProgressRange::default())
            .unwrap()
    );
}

/// `incmesh shape deflection`.
pub fn mesh_shape(shape: &Shape, deflection: f64) {
    BRepMeshIncrementalMesh::with_shape(shape, deflection, false, 0.5, false).unwrap();
}

/// `checknbshapes`: vertices, edges, wires, faces, shells, solids, compsolids, compounds, shapes.
pub fn assert_shape_counts(shape: &Shape, expected: [usize; 9]) {
    let kinds = [
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        ShapeEnum::Wire,
        ShapeEnum::Face,
        ShapeEnum::Shell,
        ShapeEnum::Solid,
        ShapeEnum::CompSolid,
        ShapeEnum::Compound,
    ];
    let mut counts = [0; 9];
    for (slot, kind) in kinds.into_iter().enumerate() {
        counts[slot] = unique_shapes(shape, kind);
    }
    counts[8] = counts[..8].iter().sum();
    assert_eq!(
        counts, expected,
        "vertices, edges, wires, faces, shells, solids, compsolids, compounds, shapes"
    );
}

fn unique_shapes(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut map).unwrap();
    map.extent()
}

/// The totals over the faces, each mesh of a face once per face.
pub fn assert_mesh_counts(shape: &Shape, triangles: i32, nodes: i32) {
    let (mut found_triangles, mut found_nodes) = (0, 0);
    let mut explorer = topexp::Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let face = topods::face(explorer.current()).unwrap();
        let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        found_triangles += mesh.get().map_or(0, Triangulation::nb_triangles);
        found_nodes += mesh.get().map_or(0, Triangulation::nb_nodes);
        explorer.next().unwrap();
    }
    assert_eq!(
        (found_triangles, found_nodes),
        (triangles, nodes),
        "triangles and nodes"
    );
}

/// The default relative deviation of 1e-2, at `sprops` epsilon 1e-4.
pub fn assert_area(shape: &Shape, expected: f64) {
    let mut properties = Default::default();
    BRepGProp::surface_properties_eps(shape, &mut properties, 1.0e-4, false).unwrap();
    let area: f64 = properties.mass();
    assert!(
        ((expected - area) / expected).abs() <= 1.0e-2,
        "area {area}, expected {expected}"
    );
}

/// Checks that the shape is valid.
pub fn assert_valid(shape: &Shape) {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false).unwrap();
    assert!(analyzer.is_valid().unwrap());
}
