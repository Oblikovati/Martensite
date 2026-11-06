//! The script commands the cases use, with what each writes to the stream.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Shape, ShapeIterator};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkdeobj::rw_obj::{
    RwObjCafReader, RwObjCafWriter, RwObjReader as _, RwObjTriangulationReader,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::IndexedDataMap;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, ExtendedString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::poly::{Triangulation, mesh_purpose};
use martensite_tkrwmesh::rw_mesh::{RwMeshCafReader as _, RwMeshCoordinateSystem};
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use std::path::PathBuf;

/// A file of this case, in a folder of its process; `case_file("prism.obj")`.
pub fn case_file(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("tkdeobj-cases-{}", std::process::id()));
    std::fs::create_dir_all(&folder).unwrap();
    folder.join(name)
}

fn text_of(path: &std::path::Path) -> AsciiString {
    AsciiString::new(path.to_str().unwrap())
}

/// A new document; `Close`ing it is dropping it.
pub fn new_document() -> Handle<TDocStdDocument> {
    TDocStdDocument::new(&ExtendedString::new("BinXCAF")).unwrap()
}

fn caf_reader(document: &Handle<TDocStdDocument>) -> RwObjCafReader {
    let mut reader = RwObjCafReader::new();
    reader.set_system_length_unit(0.001);
    reader.set_system_coordinate_system_kind(RwMeshCoordinateSystem::Z_UP);
    reader.set_file_length_unit(-1.0);
    reader.set_file_coordinate_system_kind(RwMeshCoordinateSystem::Y_UP);
    reader.set_document(document).unwrap();
    reader
}

/// `ReadObj D file`.
pub fn read_obj(file: &std::path::Path) -> Handle<TDocStdDocument> {
    let document = new_document();
    let mut reader = caf_reader(&document);
    assert!(
        reader
            .perform(&text_of(file), &ProgressRange::default())
            .unwrap()
    );
    document
}

/// `readobj shape file -singleFace`: one face with all the triangles.
pub fn read_single_face(file: &std::path::Path) -> Shape {
    let reader_settings = caf_reader(&Handle::default());
    let mut reader = RwObjTriangulationReader::new();
    reader.set_create_shapes(false);
    reader.set_transformation(reader_settings.coordinate_system_converter());
    assert!(
        reader
            .read(&text_of(file), &ProgressRange::default())
            .unwrap()
    );
    reader.result_shape().unwrap()
}

/// `readobj shape file`.
pub fn read_shape(file: &std::path::Path) -> Shape {
    let mut reader = caf_reader(&Handle::default());
    assert!(
        reader
            .perform(&text_of(file), &ProgressRange::default())
            .unwrap()
    );
    reader.single_shape().unwrap()
}

/// `WriteObj D file`.
pub fn write_obj(document: &Handle<TDocStdDocument>, file: &std::path::Path) {
    let mut writer = RwObjCafWriter::new(&text_of(file));
    let converter = writer.change_coordinate_system_converter();
    converter.set_input_length_unit(0.001);
    converter.set_input_coordinate_system_kind(RwMeshCoordinateSystem::Z_UP);
    converter.set_output_length_unit(-1.0);
    converter.set_output_coordinate_system_kind(RwMeshCoordinateSystem::Y_UP);
    let information = IndexedDataMap::default();
    assert!(
        writer
            .perform(document, &information, &ProgressRange::default())
            .unwrap()
    );
}

/// `XGetOneShape name D`: the shape of one free label, or a compound of them.
pub fn one_shape(document: &Handle<TDocStdDocument>, name: &str) -> Shape {
    let main = document.get().unwrap().main().unwrap();
    let tool = XCAFDocDocumentTool::shape_tool(&main).unwrap();
    println!("{name}");
    tool.get().unwrap().get_one_shape().unwrap()
}

/// The children of a shape, as `explode` names them.
pub fn children(shape: &Shape) -> Vec<Shape> {
    let mut iterator = ShapeIterator::new(shape, true, true);
    let mut found = Vec::new();
    while iterator.more() {
        found.push(iterator.value().unwrap().clone());
        iterator.next();
    }
    found
}

/// The unique sub-shapes of a type in explorer order, as `explode name type` names them.
pub fn explode(shape: &Shape, kind: ShapeEnum) -> Vec<Shape> {
    let mut map = IndexedMapOfShape::default();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        map.add(explorer.current().clone());
        explorer.next().unwrap();
    }
    let found: Vec<Shape> = (1..=map.extent())
        .map(|i| map.find_key(i).unwrap().clone())
        .collect();
    found
}

/// Every sub-shape once, nested compounds included, as `nbshapes` counts them.
fn all_shapes(shape: &Shape) -> IndexedMapOfShape {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes(shape, &mut map, true, true);
    map
}

/// The lines of `nbshapes`, and the counts.
fn nbshapes(name: &str, shape: &Shape) -> [(&'static str, ShapeEnum, usize); 8] {
    let kinds = [
        ("VERTEX", ShapeEnum::Vertex),
        ("EDGE", ShapeEnum::Edge),
        ("WIRE", ShapeEnum::Wire),
        ("FACE", ShapeEnum::Face),
        ("SHELL", ShapeEnum::Shell),
        ("SOLID", ShapeEnum::Solid),
        ("COMPSOLID", ShapeEnum::CompSolid),
        ("COMPOUND", ShapeEnum::Compound),
    ];
    let map = all_shapes(shape);
    let of_kind = |kind: ShapeEnum| {
        (1..=map.extent())
            .filter(|index| map.find_key(*index).unwrap().shape_type().unwrap() == kind)
            .count()
    };
    let counts = kinds.map(|(label, kind)| (label, kind, of_kind(kind)));
    println!("Number of shapes in {name}");
    for (label, _, count) in &counts {
        println!(" {label:<10}: {count}");
    }
    let total: usize = counts.iter().map(|entry| entry.2).sum();
    println!(" SHAPE     : {total}");
    println!();
    counts
}

/// `checknbshapes name -face faces [-compound compounds]`.
pub fn checknbshapes(name: &str, shape: &Shape, faces: usize, compounds: Option<usize>) {
    let counts = nbshapes(name, shape);
    assert_eq!(counts[3].2, faces, "{name}: faces");
    println!(
        "OK :  is GOOD because number of FACE entities is equal to number of expected FACE entities"
    );
    if let Some(compounds) = compounds {
        assert_eq!(counts[7].2, compounds, "{name}: compounds");
        println!(
            "OK :  is GOOD because number of COMPOUND entities is equal to number of expected COMPOUND entities"
        );
    }
}

fn faces_of(shape: &Shape) -> Vec<topods::Face> {
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut faces = Vec::new();
    while explorer.more() {
        faces.push(topods::face(explorer.current()).unwrap());
        explorer.next().unwrap();
    }
    faces
}

/// The triangles, nodes and largest deflection over the faces.
pub fn mesh_totals(shape: &Shape) -> (i32, i32, f64) {
    let (mut triangles, mut nodes, mut deflection) = (0, 0, 0.0_f64);
    for face in faces_of(shape) {
        let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        if let Some(mesh) = mesh.get() {
            triangles += Triangulation::nb_triangles(mesh);
            nodes += Triangulation::nb_nodes(mesh);
            deflection = deflection.max(mesh.deflection());
        }
    }
    (triangles, nodes, deflection)
}

/// A real as the interpreter prints it: `%.17g`.
fn real17(value: f64) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    let exponent = value.abs().log10().floor() as i32;
    let fixed = format!("{value:.*}", (16 - exponent).max(0) as usize);
    if fixed.contains('.') {
        fixed.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        fixed
    }
}

fn polygons_on_triangulation(shape: &Shape) -> usize {
    let map = all_shapes(shape);
    (1..=map.extent())
        .map(|index| map.find_key(index).unwrap())
        .filter(|sub| sub.shape_type().unwrap() == ShapeEnum::Edge)
        .map(|edge| {
            let tedge = martensite_tkbrep::brep::access::edge_tshape(edge, "BRep_TEdge").unwrap();
            tedge
                .curves()
                .iter()
                .filter(|c| c.get().is_some_and(|c| c.is_polygon_on_triangulation()))
                .count()
        })
        .sum()
}

/// The largest meshing deflection, angle and minimal size over the faces.
fn meshing_parameters(shape: &Shape) -> [f64; 3] {
    let mut found = [-1.0_f64; 3];
    for face in faces_of(shape) {
        let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE).unwrap();
        let parameters = mesh.get().map(Triangulation::parameters);
        if let Some(parameters) = parameters.as_ref().and_then(|p| p.get()) {
            let values = [
                parameters.deflection(),
                parameters.angle(),
                parameters.min_size(),
            ];
            found = std::array::from_fn(|index| found[index].max(values[index]));
        }
    }
    found
}

/// The lines `trinfo` writes.
pub fn trinfo(shape: &Shape) {
    let (triangles, nodes, deflection) = mesh_totals(shape);
    println!();
    println!("This shape contains {} faces.", faces_of(shape).len());
    println!("                    {triangles} triangles.");
    println!("                    {nodes} nodes.");
    println!(
        "                    {} polygons on triangulation.",
        polygons_on_triangulation(shape)
    );
    println!("Maximal deflection {}", real17(deflection));
    let [meshing, angle, min_size] = meshing_parameters(shape);
    if meshing > 0.0 {
        println!("Meshing deflection {}", real17(meshing));
    }
    if angle > 0.0 {
        println!(
            "Meshing angular deflection {}",
            real17(angle * 180.0 / std::f64::consts::PI)
        );
    }
    if min_size > 0.0 {
        println!("Meshing min size {}", real17(min_size));
    }
    println!();
    println!();
}

/// `checktrinfo name -tri triangles -nod nodes`.
pub fn checktrinfo(shape: &Shape, triangles: i32, nodes: i32) {
    trinfo(shape);
    let found = mesh_totals(shape);
    assert_eq!(
        (found.0, found.1),
        (triangles, nodes),
        "triangles and nodes"
    );
    println!("Check of Number of triangles OK: value = {triangles}, expected = {triangles}");
    println!("Check of Number of nodes OK: value = {nodes}, expected = {nodes}");
}
