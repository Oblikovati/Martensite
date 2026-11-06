//! `incmesh` and `checktrinfo` of the exchange scripts, on real API.
use crate::support::{session::Session, tolerance_check::checkreal};
use martensite_tkbrep::brep::{BRepTEdge, BRepTool};
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::mesh_purpose::NONE;
use martensite_tkmesh::{brep_mesh::BRepMeshIncrementalMesh, imesh_tools::MeshParameters};

/// The digits a stream writes a double with.
const STREAM_DIGITS: usize = 17;

/// `incmesh name deflection [-a degrees]`. Example: `incmesh(&raster, &["f", "0.01", "-a", "90"])`.
pub fn incmesh(session: &dyn Session, words: &[&str]) {
    println!("incmesh {} ", words.join(" "));
    let shape = session
        .shape(words[0])
        .unwrap_or_else(|| panic!("incmesh {}: expected a shape", words[0]));
    let mut parameters = MeshParameters {
        deflection: words[1].parse().expect("a deflection"),
        ..MeshParameters::default()
    };
    if let ["-a", degrees] = words[2..] {
        parameters.angle = degrees.parse::<f64>().expect("an angle").to_radians();
    }
    BRepMeshIncrementalMesh::with_parameters(shape, &parameters, &ProgressRange::default())
        .unwrap_or_else(|failure| panic!("incmesh {}: {failure:?}", words[0]));
    println!("Incremental Mesh, multi-threading OFF");
    println!("Meshing statuses: NoError");
}

/// What `trinfo` prints, in the digits its text carries.
struct TriangulationTotals {
    faces: i64,
    empty_faces: i64,
    triangles: i64,
    nodes: i64,
    polygons: i64,
    deflection: f64,
    meshing_deflection: f64,
    meshing_angle: f64,
    meshing_min_size: f64,
}

impl TriangulationTotals {
    fn add_face(&mut self, face: &Face) {
        self.faces += 1;
        let (mesh, _) = BRepTool::triangulation(face, NONE).expect("a triangulation");
        let Some(mesh) = mesh.get() else {
            self.empty_faces += 1;
            return;
        };
        self.triangles += i64::from(mesh.nb_triangles());
        self.nodes += i64::from(mesh.nb_nodes());
        self.deflection = self.deflection.max(mesh.deflection());
        if let Some(parameters) = mesh.parameters().get() {
            self.meshing_deflection = self.meshing_deflection.max(parameters.deflection());
            self.meshing_angle = self.meshing_angle.max(parameters.angle());
            self.meshing_min_size = self.meshing_min_size.max(parameters.min_size());
        }
    }

    fn of(shape: &Shape) -> Self {
        let mut totals = Self {
            faces: 0,
            empty_faces: 0,
            triangles: 0,
            nodes: 0,
            polygons: polygons_on_triangulation(shape),
            deflection: 0.0,
            meshing_deflection: -1.0,
            meshing_angle: -1.0,
            meshing_min_size: -1.0,
        };
        let mut faces =
            Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).expect("an explorer");
        while faces.more() {
            totals.add_face(&topods::face(faces.current()).expect("a face"));
            faces.next().expect("an explorer step");
        }
        totals
    }

    /// The text of `trinfo`: a `Meshing` line only when a mesh recorded its parameters.
    fn print(&self) {
        println!("\nThis shape contains {} faces.", self.faces);
        if self.empty_faces > 0 {
            println!("                    {} empty faces.", self.empty_faces);
        }
        println!("                    {} triangles.", self.triangles);
        println!("                    {} nodes.", self.nodes);
        println!(
            "                    {} polygons on triangulation.",
            self.polygons
        );
        println!(
            "Maximal deflection {}",
            format_general(self.deflection, STREAM_DIGITS)
        );
        let degrees = self.meshing_angle.to_degrees();
        for (label, value) in [
            ("Meshing deflection", self.meshing_deflection),
            ("Meshing angular deflection", degrees),
            ("Meshing min size", self.meshing_min_size),
        ] {
            if value > 0.0 {
                println!("{label} {}", format_general(value, STREAM_DIGITS));
            }
        }
    }
}

/// How many polygons on a triangulation the distinct edges of `shape` hold.
fn polygons_on_triangulation(shape: &Shape) -> i64 {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("the edges");
    let mut count = 0;
    for index in 1..=edges.extent() {
        let edge: Handle<BRepTEdge> = edges.find_key(index).expect("an edge").tshape().cast();
        let curves = edge.get().expect("an edge's own data").curves();
        count += curves
            .iter()
            .filter(|curve| curve.is_polygon_on_triangulation())
            .count() as i64;
    }
    count
}

/// `string is boolean` for what these options hold: `-face 1` asks only that there are faces.
fn is_boolean_word(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "0" | "1" | "true" | "false" | "yes" | "no" | "on" | "off"
    )
}

/// One count against what the script expects, by the procedure's rule: a boolean word checks that the
/// count is not 0 when it is true, and anything else goes to `checkreal`.
fn check_count(what: &str, value: i64, expected: &str) {
    if !is_boolean_word(expected) {
        checkreal(what, &value.to_string(), expected, 0.0, 0.0);
    } else if value <= 0 && expected != "0" {
        println!("Error: {what} is equal to 0");
    }
}

/// `checktrinfo name [-face n] [-nod n] [-tri n] [-defl v [-tol_abs_defl v]]`: `trinfo`'s totals against
/// what the script expects, each by `checkreal`, as the procedure does.
pub fn checktrinfo(session: &dyn Session, words: &[&str]) {
    let shape = session
        .shape(words[0])
        .unwrap_or_else(|| panic!("checktrinfo {}: expected a shape", words[0]));
    let totals = TriangulationTotals::of(shape);
    totals.print();
    let options = &words[1..];
    let word_after = |flag: &str| {
        options
            .iter()
            .position(|word| *word == flag)
            .map(|at| options[at + 1])
    };
    let tol_abs =
        word_after("-tol_abs_defl").map_or(0.0, |word| word.parse().expect("a tolerance"));
    for (flag, what, value) in [
        ("-face", "Number of faces", totals.faces),
        ("-tri", "Number of triangles", totals.triangles),
        ("-nod", "Number of nodes", totals.nodes),
    ] {
        if let Some(expected) = word_after(flag) {
            check_count(what, value, expected);
        }
    }
    if let Some(expected) = word_after("-defl") {
        checkreal(
            "Maximal deflection",
            &format_general(totals.deflection, STREAM_DIGITS),
            expected,
            tol_abs,
            0.0,
        );
    }
}

#[test]
fn a_boolean_word_asks_only_that_the_count_is_not_zero() {
    assert!(is_boolean_word("1"));
    assert!(is_boolean_word("False"));
    assert!(!is_boolean_word("12"));
    check_count("Number of faces", 1, "1");
}
