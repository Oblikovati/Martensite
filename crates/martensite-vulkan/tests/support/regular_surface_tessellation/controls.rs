use super::*;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Edge, Shape},
};
use martensite_tkg3d::{
    geom::{GeomCylindricalSurface, GeomPlane, GeomRectangularTrimmedSurface, GeomToroidalSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::toploc::Location;
use martensite_tkmath::{
    gp::Axis3,
    poly::{Triangulation, mesh_purpose},
};

fn plane_surface() -> Handle<dyn GeomSurface> {
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    Handle::new(
        GeomRectangularTrimmedSurface::new(&plane, (0., 4.), (0., 3.), (true, true)).unwrap(),
    )
    .cast()
}

fn mesh(face: &Face) -> Handle<Triangulation> {
    let (mesh, location) = BRepTool::triangulation(face, mesh_purpose::NONE).unwrap();
    assert!(location.is_identity());
    assert!(!mesh.has_uv_nodes() && !mesh.has_normals());
    mesh
}

#[test]
fn plane_grid_keeps_exact_node_and_triangle_order_and_four_boundary_polygons() {
    let face = from_surface(&plane_surface(), 2, 3).unwrap();
    let mesh = mesh(&face);
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (12, 12));
    plane_nodes(&mesh);
    plane_triangles(&mesh);
    let mut polygons = edge_polygons(&face, &mesh);
    polygons.sort();
    assert_eq!(
        polygons,
        [
            vec![1, 2, 3, 4],
            vec![1, 5, 9],
            vec![4, 8, 12],
            vec![9, 10, 11, 12]
        ]
    );
}

fn polygon_nodes(edge: &Edge, mesh: &Handle<Triangulation>) -> Vec<i32> {
    let polygon = BRepTool::polygon_on_triangulation(edge, mesh, &Location::default()).unwrap();
    assert!(!polygon.is_null());
    assert!(!polygon.has_parameters());
    let nodes = polygon.nodes();
    (nodes.lower()..=nodes.upper())
        .map(|i| *nodes.value(i).unwrap())
        .collect()
}

fn edge_polygons(face: &Face, mesh: &Handle<Triangulation>) -> Vec<Vec<i32>> {
    face_edges(face)
        .map(|s| polygon_nodes(&topods::edge(&s).unwrap(), mesh))
        .collect()
}

fn bounded_cylinder() -> Face {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 2.).unwrap()).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(
        &surface,
        (0., std::f64::consts::TAU, 1., 4.),
        precision::confusion(),
    )
    .unwrap()
    .face()
    .unwrap()
}

fn seam_pair(face: &Face, mesh: &Handle<Triangulation>, edge: &Edge) -> [Vec<i32>; 2] {
    assert!(BRepTool::is_closed_on_face(edge, face).unwrap());
    [Orientation::Forward, Orientation::Reversed].map(|orientation| {
        let mut oriented = edge.clone();
        oriented.set_orientation(orientation);
        polygon_nodes(&oriented, mesh)
    })
}

#[test]
fn cylinder_seam_retains_both_u_boundaries_and_both_v_boundary_polygons() {
    let source = bounded_cylinder();
    let face = from_face(&source, 4, 2).unwrap();
    let mesh = mesh(&face);
    assert_eq!((mesh.nb_nodes(), mesh.nb_triangles()), (15, 16));
    let seams = cylinder_seams(&face, &mesh);
    assert_eq!(seams, 2);
    assert!(
        BRepTool::triangulation(&source, mesh_purpose::NONE)
            .unwrap()
            .0
            .is_null()
    );
    cylinder_polygons(&face, &mesh);
}

fn cylinder_seams(face: &Face, mesh: &Handle<Triangulation>) -> usize {
    let mut seams = 0;
    for s in face_edges(face) {
        let edge = topods::edge(&s).unwrap();
        if !BRepTool::is_closed_on_face(&edge, face).unwrap() {
            continue;
        }
        assert_eq!(
            seam_pair(face, mesh, &edge),
            [vec![1, 2, 3], vec![13, 14, 15]]
        );
        seams += 1;
    }
    seams
}

fn cylinder_polygons(face: &Face, mesh: &Handle<Triangulation>) {
    let mut polygons = edge_polygons(face, mesh);
    polygons.sort();
    assert_eq!(
        polygons,
        [
            vec![1, 2, 3],
            vec![1, 4, 7, 10, 13],
            vec![3, 6, 9, 12, 15],
            vec![13, 14, 15]
        ]
    );
}

#[test]
fn torus_records_both_polygon_variants_for_closed_u_and_closed_v_edges() {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomToroidalSurface::new(Axis3::default(), 5., 1.).unwrap()).cast();
    let face = from_surface(&surface, 3, 2).unwrap();
    let mesh = mesh(&face);
    let pairs = periodic_pairs(&face, &mesh);
    assert_eq!(
        pairs,
        [
            [vec![1, 2, 3], vec![10, 11, 12]],
            [vec![1, 4, 7, 10], vec![3, 6, 9, 12]]
        ]
    );
}

fn periodic_pairs(face: &Face, mesh: &Handle<Triangulation>) -> Vec<[Vec<i32>; 2]> {
    let mut pairs = Vec::new();
    for s in face_edges(face) {
        let edge = topods::edge(&s).unwrap();
        let pair = seam_pair(face, mesh, &edge);
        if !pairs.contains(&pair) {
            pairs.push(pair);
        }
    }
    pairs.sort();
    pairs
}

#[test]
fn invalid_subdivisions_and_infinite_surface_ranges_are_typed_refusals() {
    for (u, v) in [(0, 1), (1, 0), (-1, 2), (i32::MAX, 2), (32768, 32768)] {
        assert!(
            from_surface(&plane_surface(), u, v)
                .unwrap_err()
                .to_string()
                .contains("expected positive subdivisions")
        );
    }
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    assert!(
        from_surface(&plane, 1, 1)
            .unwrap_err()
            .to_string()
            .contains("finite parametric range")
    );
}

const PLANE_NODES: [(f64, f64); 12] = [
    (0., 0.),
    (0., 1.),
    (0., 2.),
    (0., 3.),
    (2., 0.),
    (2., 1.),
    (2., 2.),
    (2., 3.),
    (4., 0.),
    (4., 1.),
    (4., 2.),
    (4., 3.),
];
const PLANE_TRIANGLES: [(i32, i32, i32); 12] = [
    (1, 6, 2),
    (1, 5, 6),
    (2, 7, 3),
    (2, 6, 7),
    (3, 8, 4),
    (3, 7, 8),
    (5, 10, 6),
    (5, 9, 10),
    (6, 11, 7),
    (6, 10, 11),
    (7, 12, 8),
    (7, 11, 12),
];
fn plane_nodes(mesh: &Handle<Triangulation>) {
    for (offset, expected) in PLANE_NODES.into_iter().enumerate() {
        let p = mesh.node(offset as i32 + 1).unwrap();
        assert_eq!((p.x(), p.y(), p.z()), (expected.0, expected.1, 0.));
    }
}
fn plane_triangles(mesh: &Handle<Triangulation>) {
    let expected = PLANE_TRIANGLES;
    for (offset, triangle) in expected.into_iter().enumerate() {
        assert_eq!(mesh.triangle(offset as i32 + 1).unwrap().get(), triangle);
    }
}

fn face_edges(face: &Face) -> Explorer {
    Explorer::new(
        &Shape::from(face.clone()),
        ShapeEnum::Edge,
        ShapeEnum::Shape,
    )
    .unwrap()
}

#[test]
fn face_source_stays_unmeshed_after_refusal_and_success() {
    let source = bounded_cylinder();
    assert!(from_face(&source, 0, 2).is_err());
    assert!(
        BRepTool::triangulation(&source, mesh_purpose::NONE)
            .unwrap()
            .0
            .is_null()
    );
    let result = from_face(&source, 4, 2).unwrap();
    let mesh = mesh(&result);
    for (index, expected) in [(1, (2., 0., 1.)), (3, (2., 0., 4.)), (7, (-2., 0., 1.))] {
        let point = mesh.node(index).unwrap();
        assert!((point.x() - expected.0).abs() < 1e-14);
        assert!((point.y() - expected.1).abs() < 1e-14);
        assert_eq!(point.z(), expected.2);
    }
    assert!(
        BRepTool::triangulation(&source, mesh_purpose::NONE)
            .unwrap()
            .0
            .is_null()
    );
}
