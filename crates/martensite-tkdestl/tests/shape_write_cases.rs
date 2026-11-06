//! Writing shapes to STL and reading the files back.
mod support;

use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{Compound, Shape},
};
use martensite_tkdestl::{
    rw_stl::{DEFAULT_MERGE_ANGLE, RwStl},
    stl_api::{StlApiReader, StlApiWriter},
};
use martensite_tkernel::message::{
    ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::Transform3,
    poly::{Triangle, Triangulation},
};
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis, BRepPrimAPIMakeTorus,
};
use martensite_tktopalgo::{
    brep_bnd_lib::BRepBndLib, brep_builder_api::BRepBuilderAPITransform,
    brep_extrema::BRepExtremaShapeProximity,
};
use std::sync::{Arc, Mutex};
use support::memory_stream::memory_istream;
use support::shapes::{count_faces, count_triangles, planar_face_with_mesh};

/// The `ProgressObserver`: one formatted line per `show`.
#[derive(Clone, Default)]
struct ProgressObserver(Arc<Mutex<Vec<String>>>);
impl ProgressIndicatorHooks for ProgressObserver {
    fn show(&self, scope: &ProgressScope<'_>, position: f64, _force: bool) {
        let mut message = format!("Progress: {}%", (100.0 * position + 0.5) as i32);
        if let Some(name) = scope.name() {
            message += &format!(
                " {}: {} / {}",
                String::from_utf8_lossy(name),
                (scope.value() + 0.5) as i32,
                (scope.max_value() + 0.5) as i32
            );
        }
        self.0.lock().unwrap().push(message);
    }
}

fn mesh_box(dx: f64, dy: f64, dz: f64) -> Shape {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(dx, dy, dz).unwrap();
    maker.solid().unwrap().into_shape()
}

fn mesh_shape(shape: &Shape, deflection: f64) {
    BRepMeshIncrementalMesh::with_shape(shape, deflection, false, 0.5, false).unwrap();
}

/// The `MakeRegularTriangulatedFace`: 100 by 100 cells, 20000 triangles.
fn regular_triangulated_face() -> Shape {
    const CELLS: i32 = 100;
    let mesh =
        Triangulation::with_sizes((CELLS + 1) * (CELLS + 1), 2 * CELLS * CELLS, false, false)
            .unwrap();
    for u in 0..=CELLS {
        for v in 0..=CELLS {
            let point = martensite_tkmath::gp::Point3::new(f64::from(u), f64::from(v), 0.);
            mesh.set_node(1 + u * (CELLS + 1) + v, point).unwrap();
        }
    }
    let mut triangle = 1;
    for u in 0..CELLS {
        for v in 0..CELLS {
            let base = 1 + u * (CELLS + 1) + v;
            mesh.set_triangle(triangle, Triangle::new(base, base + CELLS + 2, base + 1))
                .unwrap();
            mesh.set_triangle(
                triangle + 1,
                Triangle::new(base, base + CELLS + 1, base + CELLS + 2),
            )
            .unwrap();
            triangle += 2;
        }
    }
    planar_face_with_mesh(100.0, mesh)
}

fn make_binary_stl(has_facet: bool) -> Vec<u8> {
    let mut bytes = vec![0; if has_facet { 134 } else { 84 }];
    bytes[..4].copy_from_slice(b"stl ");
    if has_facet {
        bytes[80] = 1;
        let values: [f32; 12] = [0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 1., 0.];
        for (index, value) in values.iter().enumerate() {
            bytes[84 + index * 4..88 + index * 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    bytes
}

fn read_stl(content: &[u8]) -> (bool, Shape) {
    let mut stream = memory_istream(content);
    let mut shape = Shape::default();
    let done = StlApiReader.read_stream(&mut shape, &mut stream).unwrap();
    (done, shape)
}

fn write_stl(shape: &Shape, ascii_mode: bool) -> Option<Vec<u8>> {
    let mut writer = StlApiWriter::new();
    *writer.ascii_mode() = ascii_mode;
    let mut bytes = Vec::new();
    let done = writer
        .write_stream(shape, &mut bytes, &ProgressRange::default())
        .unwrap();
    done.then_some(bytes)
}

/// The `RoundTripStl`: a null shape when any step fails.
fn round_trip_stl(shape: &Shape) -> Shape {
    let Some(bytes) = write_stl(shape, false) else {
        return Shape::default();
    };
    let (done, restored) = read_stl(&bytes);
    if done { restored } else { Shape::default() }
}

/// The `RoundTripStlMesh`: one face on the mesh read back.
fn round_trip_stl_mesh(shape: &Shape) -> Shape {
    let Some(bytes) = write_stl(shape, false) else {
        return Shape::default();
    };
    let mut stream = memory_istream(&bytes);
    let mesh =
        RwStl::read_stream(&mut stream, DEFAULT_MERGE_ANGLE, &ProgressRange::default()).unwrap();
    if mesh.is_null() {
        return Shape::default();
    }
    let builder = BRepBuilder::default();
    let mut face = martensite_tkbrep::topods::Face::default();
    builder.make_face(&mut face);
    builder
        .update_face_triangulation(&face, &mesh, true)
        .unwrap();
    face.into()
}

const EXPECTED_PROGRESS: [&str; 21] = [
    "Progress: 0%",
    "Progress: 5% Triangles: 1000 / 20000",
    "Progress: 10% Triangles: 2000 / 20000",
    "Progress: 15% Triangles: 3000 / 20000",
    "Progress: 20% Triangles: 4000 / 20000",
    "Progress: 25% Triangles: 5000 / 20000",
    "Progress: 30% Triangles: 6000 / 20000",
    "Progress: 35% Triangles: 7000 / 20000",
    "Progress: 40% Triangles: 8000 / 20000",
    "Progress: 45% Triangles: 9000 / 20000",
    "Progress: 50% Triangles: 10000 / 20000",
    "Progress: 55% Triangles: 11000 / 20000",
    "Progress: 60% Triangles: 12000 / 20000",
    "Progress: 65% Triangles: 13000 / 20000",
    "Progress: 70% Triangles: 14000 / 20000",
    "Progress: 75% Triangles: 15000 / 20000",
    "Progress: 80% Triangles: 16000 / 20000",
    "Progress: 85% Triangles: 17000 / 20000",
    "Progress: 90% Triangles: 18000 / 20000",
    "Progress: 95% Triangles: 19000 / 20000",
    "Progress: 100% Triangles: 20000 / 20000",
];

#[test]
fn b1_write_progress_and_read_back() {
    let shape = regular_triangulated_face();
    assert!(!shape.is_null());
    let observer = ProgressObserver::default();
    let messages = observer.0.clone();
    let indicator = ProgressIndicator::new(Box::new(observer));
    let mut writer = StlApiWriter::new();
    *writer.ascii_mode() = false;
    let mut bytes = Vec::new();
    assert!(
        writer
            .write_stream(&shape, &mut bytes, &indicator.start())
            .unwrap()
    );
    assert_eq!(*messages.lock().unwrap(), EXPECTED_PROGRESS);
    assert_eq!(indicator.get_position(), 1.0);
    assert!(!bytes.is_empty());
    let (read, restored) = read_stl(&bytes);
    assert!(read);
    assert!(!restored.is_null());
    assert!(count_faces(&restored) > 0);
}

const MINIMAL_ASCII: &str = "solid\nfacet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\nendsolid";
const DOS_ASCII: &str = "solid\r\nfacet normal 0 0 1\r\nouter loop\r\nvertex 0 0 0\r\nvertex 1 0 0\r\nvertex 0 1 0\r\nendloop\r\nendfacet\r\nendsolid\r\n";

#[test]
fn d1_read_ascii_and_binary_boundary_cases() {
    let (read, shape) = read_stl(DOS_ASCII.as_bytes());
    assert!(read);
    assert_eq!(count_faces(&shape), 1);
    let (read, shape) = read_stl(format!("{MINIMAL_ASCII}\n").as_bytes());
    assert!(read);
    assert_eq!(count_faces(&shape), 1);
    let (read, shape) = read_stl(MINIMAL_ASCII.as_bytes());
    assert!(read);
    assert_eq!(count_faces(&shape), 1);
    let (read, shape) = read_stl(b"solid \nendsolid");
    assert!(!read);
    assert!(shape.is_null());
    let (read, shape) = read_stl(&make_binary_stl(true));
    assert!(read);
    assert_eq!(count_faces(&shape), 1);
    let (read, shape) = read_stl(&make_binary_stl(false));
    assert!(!read);
    assert!(shape.is_null());
    let (read, shape) = read_stl(b"");
    assert!(!read);
    assert!(shape.is_null());
}

#[test]
fn modalg_bug_30829_proximity_after_stl_round_trip() {
    let solid_box = mesh_box(10., 10., 10.);
    let mut maker = BRepPrimAPIMakeCylinder::new(5., 10.);
    let cylinder = maker.solid().unwrap().into_shape();
    mesh_shape(&solid_box, 0.1);
    mesh_shape(&cylinder, 0.1);
    let restored_box = round_trip_stl_mesh(&solid_box);
    let restored_cylinder = round_trip_stl_mesh(&cylinder);
    assert!(!restored_box.is_null());
    assert!(!restored_cylinder.is_null());
    let mut proximity = BRepExtremaShapeProximity::new(0.0);
    assert!(proximity.load_shape1(&restored_box).unwrap());
    assert!(proximity.load_shape2(&restored_cylinder).unwrap());
    proximity.perform().unwrap();
    assert!(proximity.is_done());
    assert!(proximity.element_set1().get().unwrap().size() > 0);
    assert!(proximity.element_set2().get().unwrap().size() > 0);
}

#[test]
fn stl_bug_26338_compound_locations() {
    let first_box = mesh_box(5., 5., 5.);
    let translation = Transform3::translation([10., 10., 10.]);
    let transform =
        BRepBuilderAPITransform::with_shape(&first_box, translation, false, false).unwrap();
    let second_box = transform.modified_shape(&first_box).unwrap();
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    builder.add(&mut compound, &first_box).unwrap();
    builder.add(&mut compound, &second_box).unwrap();
    let compound: Shape = compound.into();
    mesh_shape(&compound, 1.0);
    let restored = round_trip_stl(&compound);
    assert!(!restored.is_null());
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(&restored, &mut bounds, true).unwrap();
    let [x_min, y_min, z_min, x_max, y_max, z_max] = bounds.get().unwrap();
    for minimum in [x_min, y_min, z_min] {
        assert!((minimum - 0.0).abs() <= 1.0e-6, "{minimum}");
    }
    for maximum in [x_max, y_max, z_max] {
        assert!((maximum - 15.0).abs() <= 1.0e-6, "{maximum}");
    }
}

#[test]
fn stl_bug_30389_missing_face_triangulation() {
    let solid_box = mesh_box(1., 1., 1.);
    let builder = BRepBuilder::default();
    let mut meshed_faces = Compound::default();
    builder.make_compound(&mut meshed_faces);
    let mut explorer = Explorer::new(&solid_box, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut face_index = 0;
    while explorer.more() && face_index < 5 {
        builder.add(&mut meshed_faces, explorer.current()).unwrap();
        explorer.next().unwrap();
        face_index += 1;
    }
    assert_eq!(face_index, 5);
    mesh_shape(&meshed_faces.into(), 0.1);
    let restored = round_trip_stl(&solid_box);
    assert!(!restored.is_null());
    assert_eq!(count_faces(&restored), 10);
}

#[test]
fn xde_bug_25357_preserve_triangulation() {
    let mut maker = BRepPrimAPIMakeTorus::new(10., 8.).unwrap();
    let torus = maker.solid().unwrap().into_shape();
    mesh_shape(&torus, 0.5);
    let before = count_triangles(&torus);
    assert!(before > 0);
    let bytes = write_stl(&torus, false).unwrap();
    assert!(!bytes.is_empty());
    assert_eq!(count_triangles(&torus), before);
}
