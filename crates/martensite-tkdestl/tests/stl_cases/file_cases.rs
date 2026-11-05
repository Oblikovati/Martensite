//! Cases that need no real view: read, write, count, measure and validate.
use super::checks::{
    CaseFile, assert_mesh_counts, assert_shape_counts, input, mesh_shape, read_faces, write_stl,
};
use super::raster_view::assert_shape_draws;
use martensite_tkbrep::{brep::BRepBuilder, topods::Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomSphericalSurface, GeomSurface};
use martensite_tkmath::{
    gp::{Axis3, Point3},
    poly::{Triangle, Triangulation},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

const BOX_STL: &str = "dataset-7.9.0/others/OMF6391_box.stl";
/// One compound of the twelve triangular faces of a box, sharing nodes and edges.
const BOX_OF_FACES: [usize; 9] = [8, 18, 12, 12, 0, 0, 0, 1, 51];

#[test]
fn a_facet_file_reads_into_faces() {
    // The script only reads; a failed read leaves no shape, and the box has twelve facets.
    assert_shape_counts(&read_faces(&input(BOX_STL)), BOX_OF_FACES);
}

#[test]
fn a_file_of_two_facets_reads_into_a_compound_of_two_faces() {
    let shape = read_faces(&input("dataset-7.9.0/others/bug27622_Design1.stl"));
    assert_shape_counts(&shape, [4, 5, 2, 2, 0, 0, 0, 1, 14]);
    assert_shape_draws(&shape, "two_facets");
}

fn box_round_trip_through_text(name: &str) -> Shape {
    let read = read_faces(&input(BOX_STL));
    mesh_shape(&read, 0.1);
    let file = CaseFile::new(name);
    write_stl(&read, &file.name(), 0);
    read_faces(&file.name())
}

#[test]
fn a_meshed_box_of_faces_round_trips_through_text() {
    let result = box_round_trip_through_text("box_text.stl");
    assert_shape_counts(&result, BOX_OF_FACES);
    assert_shape_draws(&result, "box_text");
}

/// A sphere of radius 10 with 2 million triangles: `sphere s 10`, `tessellate result s 1000 1000`.
fn tessellated_sphere() -> Shape {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomSphericalSurface::new(Axis3::default(), 10.).unwrap()).cast();
    let surface_ref = surface.get().unwrap();
    let bounds = surface_ref.bounds();
    let maker = BRepBuilderAPIMakeFace::from_surface_bounds(&surface, bounds, 1.0e-7).unwrap();
    let face = maker.face().unwrap();
    let mesh = sphere_grid(surface_ref, bounds);
    BRepBuilder::default()
        .update_face_triangulation(&face, &Handle::new(mesh), true)
        .unwrap();
    face.into()
}

const GRID: i32 = 1000;

fn sphere_grid(surface: &dyn GeomSurface, bounds: (f64, f64, f64, f64)) -> Triangulation {
    let (u_min, u_max, v_min, v_max) = bounds;
    let mesh =
        Triangulation::with_sizes((GRID + 1) * (GRID + 1), 2 * GRID * GRID, false, false).unwrap();
    let (du, dv) = (
        (u_max - u_min) / f64::from(GRID),
        (v_max - v_min) / f64::from(GRID),
    );
    for u in 0..=GRID {
        for v in 0..=GRID {
            let point: Point3 = surface
                .value(u_min + f64::from(u) * du, v_min + f64::from(v) * dv)
                .unwrap();
            mesh.set_node(1 + u * (GRID + 1) + v, point).unwrap();
        }
    }
    let mut triangle = 0;
    for u in 0..GRID {
        for v in 0..GRID {
            let base = 1 + u * (GRID + 1) + v;
            mesh.set_triangle(triangle + 1, Triangle::new(base, base + GRID + 2, base + 1))
                .unwrap();
            mesh.set_triangle(
                triangle + 2,
                Triangle::new(base, base + GRID + 1, base + GRID + 2),
            )
            .unwrap();
            triangle += 2;
        }
    }
    mesh
}

#[test]
fn two_million_triangles_write_to_a_binary_file_of_the_expected_size() {
    let sphere = tessellated_sphere();
    assert_mesh_counts(&sphere, 2_000_000, 1_002_001);
    let file = CaseFile::new("sphere-binary.stl");
    write_stl(&sphere, &file.name(), 1);
    assert!(
        file.size() >= 100_000_000,
        "unexpected file size {}",
        file.size()
    );
}

#[test]
fn two_million_triangles_write_to_a_text_file_of_the_expected_size() {
    let sphere = tessellated_sphere();
    let file = CaseFile::new("sphere-ascii.stl");
    write_stl(&sphere, &file.name(), 0);
    assert!(
        file.size() >= 490_000_000,
        "unexpected file size {}",
        file.size()
    );
}
