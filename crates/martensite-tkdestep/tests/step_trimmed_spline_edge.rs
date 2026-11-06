//! A vertex that lies inside the range of a B-spline edge curve: its tolerance covers the rounding
//! of the end point, the edge is built at once, and the curve keeps the end poles of the file. Moving
//! them would take the ends of the trimmed edge away from its vertices and coarsen the mesh beside it.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape, ShapeIterator};
use martensite_tkdestep::stepcaf_control::{STEPCAFControlController, STEPCAFControlReader};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::geom::GeomBSplineCurve;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdoc_std::{TDocStdApplication, TDocStdDocument};
use martensite_tkmath::gp::Point3;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkmesh::imesh_tools::MeshParameters;
use martensite_tkxcaf::xcaf_app::XCAFAppApplication;
use martensite_tkxcaf::xcaf_doc::XCAFDocDocumentTool;
use std::path::PathBuf;

/// The face whose outer wire starts with the trimmed spline edge, and its neighbour on that edge.
const SPLINE_FACE: usize = 41;
const NEIGHBOUR_FACE: usize = 39;

fn read_motor() -> Shape {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0/step/as1_motor.step");
    STEPCAFControlController::init().expect("controller");
    let mut document: Handle<TDocStdDocument> = Handle::default();
    XCAFAppApplication::get_application()
        .new_tdoc_std_document(&ExtendedString::from_utf8("BinXCAF"), &mut document)
        .expect("document");
    let mut reader = STEPCAFControlReader::new().expect("reader");
    let done = reader
        .perform(
            &path.to_string_lossy(),
            &document,
            &ProgressRange::default(),
        )
        .expect("perform");
    assert!(done, "{}", path.display());
    let main = document.get().expect("document").main().expect("main");
    XCAFDocDocumentTool::shape_tool(&main)
        .expect("tool")
        .get_one_shape()
        .expect("shape")
}

fn faces_of(shape: &Shape) -> Vec<Face> {
    let mut faces = Vec::new();
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    while explorer.more() {
        faces.push(topods::face(explorer.current()).expect("face"));
        explorer.next().expect("next");
    }
    faces
}

fn first_child(shape: &Shape) -> Shape {
    let iterator = ShapeIterator::new(shape, true, true);
    iterator.value().expect("child").clone()
}

fn assert_near(point: Point3, wanted: [f64; 3], tolerance: f64) {
    let distance = point.distance(&Point3::new(wanted[0], wanted[1], wanted[2]));
    assert!(
        distance <= tolerance,
        "{point:?} is {distance} away from {wanted:?}"
    );
}

#[test]
fn a_trimmed_spline_edge_keeps_the_end_poles_of_its_file() {
    let faces = faces_of(&read_motor());
    let wire = first_child(&faces[SPLINE_FACE - 1].clone().into());
    let edge = topods::edge(&first_child(&wire)).expect("edge");
    let curve = BRepTool::curve_with_location(&edge).expect("curve").curve;
    let spline = curve.cast::<GeomBSplineCurve>();
    let spline = spline.get().expect("a B-spline edge curve");
    assert_near(
        spline.pole(1).expect("first pole"),
        [-92000.00632378, 0.0, -57000.0],
        1e-7,
    );
    assert_near(
        spline.pole(spline.nb_poles()).expect("last pole"),
        [-107999.4267722, 0.0, -57000.0],
        1e-7,
    );
}

#[test]
fn the_vertices_of_a_trimmed_spline_edge_keep_a_small_tolerance() {
    let shape = read_motor();
    let tolerance = BRepTool::max_tolerance(&shape, ShapeEnum::Vertex).expect("tolerance");
    assert!(tolerance < 0.19, "largest vertex tolerance {tolerance}");
}

#[test]
fn the_faces_beside_a_trimmed_spline_edge_mesh_to_their_recorded_counts() {
    let shape = read_motor();
    let parameters = MeshParameters {
        deflection: 5.0,
        deflection_interior: 5.0,
        angle: 0.5,
        in_parallel: false,
        ..MeshParameters::default()
    };
    BRepMeshIncrementalMesh::with_parameters(&shape, &parameters, &ProgressRange::default())
        .expect("mesh");
    let faces = faces_of(&shape);
    let counts = |index: usize| {
        let (mesh, _) =
            BRepTool::triangulation(&faces[index - 1], mesh_purpose::NONE).expect("mesh");
        let mesh = mesh.get().expect("a meshed face");
        (mesh.nb_triangles(), mesh.nb_nodes(), mesh.deflection())
    };
    let (triangles, nodes, _) = counts(SPLINE_FACE);
    assert_eq!((triangles, nodes), (6569, 3367));
    let (triangles, nodes, _) = counts(NEIGHBOUR_FACE);
    assert_eq!((triangles, nodes), (363, 365));
    let totals = (1..=faces.len()).map(counts).fold(
        (0, 0, 0.0_f64),
        |(triangles, nodes, deflection), (t, n, d)| (triangles + t, nodes + n, deflection.max(d)),
    );
    assert_eq!((totals.0, totals.1), (142_040, 90_692));
    assert!(
        (totals.2 - 10.738_785_017_720_437).abs() < 1e-9,
        "largest deflection {}",
        totals.2
    );
}
