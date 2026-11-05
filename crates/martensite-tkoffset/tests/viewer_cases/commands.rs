//! The modelling commands the viewer scripts call, on the library's builders with the options the
//! commands pass.
use crate::support::checkshape::checkshape;
use crate::variables::Variables;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkernel::{message::ProgressRange, precision};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkmesh::imesh_tools::MeshParameters;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;

/// `restore [locate_data_file file] name`; `restore(&mut variables, "a.brep", "a")`.
pub fn restore(variables: &mut Variables, file: &str, name: &str) {
    crate::support::restore::restore(variables, file, name);
}

/// `checkshape name`; `check_shape(&variables, "result")`.
pub fn check_shape(variables: &Variables, name: &str) {
    checkshape(&variables.get(name));
}

/// `explode name kind`: the distinct sub-shapes in explorer order, as `name_1`, `name_2` and so on.
/// Example: `explode(&mut variables, "s", ShapeEnum::edge)`.
pub fn explode(variables: &mut Variables, name: &str, kind: ShapeEnum) -> usize {
    let shape = variables.get(name);
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    let mut explorer = Explorer::new(&shape, kind, ShapeEnum::Shape).unwrap();
    let mut count = 0;
    while explorer.more() {
        if seen.add(explorer.current().clone()) {
            count += 1;
            variables.set(&format!("{name}_{count}"), explorer.current().clone());
        }
        explorer.next().unwrap();
    }
    count
}

/// How many triangles and nodes the faces of a shape hold, as `trinfo` adds them up.
pub fn triangulation_counts(shape: &Shape) -> (usize, usize) {
    let (mut triangles, mut nodes) = (0, 0);
    let mut seen = MapOfShape::default();
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        if seen.add(faces.current().clone()) {
            let face = topods::face(faces.current()).unwrap();
            let (mesh, _) = BRepTool::triangulation(&face, 0).unwrap();
            if let Some(mesh) = mesh.get() {
                triangles += mesh.nb_triangles() as usize;
                nodes += mesh.nb_nodes() as usize;
            }
        }
        faces.next().unwrap();
    }
    (triangles, nodes)
}

/// `incmesh name deflection`; `incremental_mesh(&cutter, 0.01)`.
pub fn incremental_mesh(shape: &Shape, deflection: f64) {
    let parameters = MeshParameters {
        deflection,
        ..MeshParameters::default()
    };
    let range = ProgressRange::default();
    BRepMeshIncrementalMesh::with_parameters(shape, &parameters, &range).unwrap();
    println!("Incremental Mesh, multi-threading OFF");
}

/// `unifysamedom result shape`, with the command's defaults.
pub fn unify_same_domain(shape: &Shape) -> Shape {
    let unifier = ShapeUpgradeUnifySameDomain::new(shape, true, true, false);
    unifier.set_safe_input_mode(true);
    unifier.set_linear_tolerance(precision::confusion());
    unifier.set_angular_tolerance(precision::angular());
    unifier.allow_internal_edges(false);
    unifier.build().unwrap();
    unifier.shape().clone()
}
