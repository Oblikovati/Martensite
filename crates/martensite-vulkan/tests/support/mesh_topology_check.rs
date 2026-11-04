//! Topology check of a shape's face triangulations: free links inside faces, cross-face node
//! mismatches on shared edges, edges whose polygons differ in length between faces, nodes no
//! triangle uses, and faces without triangulation. A regression's mesh is valid when every
//! list is empty (the empty `tricheck` report of the regressions).
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Edge, Face, Shape};
use martensite_tkbrep::toptools::{IndexedDataMapOfShapeListOfShape, IndexedMapOfShape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::{Connect, Triangulation, mesh_purpose};
use martensite_tkmath::toploc::Location;
use std::collections::BTreeSet;

/// The findings of one check; `MeshTopologyReport::default().is_clean()` is true.
#[derive(Debug, Default, PartialEq)]
pub struct MeshTopologyReport {
    /// (face index, node pair) of each free link not on the face boundary.
    pub free_links: Vec<(usize, [i32; 2])>,
    /// (face 1, node index 1, face 2, node index 2, distance) beyond the edge tolerance.
    pub cross_face_errors: Vec<(usize, i32, usize, i32, f64)>,
    /// Edge indices whose polygons have different lengths on two faces.
    pub async_edges: Vec<usize>,
    /// (face index, node) of nodes no triangle uses.
    pub free_nodes: Vec<(usize, i32)>,
    /// Face indices without triangulation.
    pub untriangulated_faces: Vec<usize>,
}

impl MeshTopologyReport {
    /// Whether nothing was found.
    pub fn is_clean(&self) -> bool {
        *self == Self::default()
    }
}

/// Check the triangulations of `shape`; `check_mesh_topology(&shape)?.is_clean()`.
pub fn check_mesh_topology(shape: &Shape) -> Result<MeshTopologyReport, Failure> {
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Face, &mut faces)?;
    let mut report = MeshTopologyReport::default();
    check_shared_edges(shape, &faces, &mut report)?;
    for index in 1..=faces.extent() {
        check_face(index, &topods::face(faces.find_key(index)?)?, &mut report)?;
    }
    Ok(report)
}

/// The regression verdict: `Mesh is OK` for a clean report, else `Error : Invalid mesh`.
/// Example: `report_mesh_validity(&result)?`.
pub fn report_mesh_validity(shape: &Shape) -> Result<(), Failure> {
    if check_mesh_topology(shape)?.is_clean() {
        println!("Mesh is OK");
    } else {
        println!("Error : Invalid mesh");
    }
    Ok(())
}

type Triangulated = (Handle<Triangulation>, Location);

fn triangulation(face: &Face) -> Result<Triangulated, Failure> {
    BRepTool::triangulation(face, mesh_purpose::NONE)
}

fn polygon_nodes(
    edge: &Edge,
    (mesh, location): &Triangulated,
) -> Result<Option<Vec<i32>>, Failure> {
    let polygon = BRepTool::polygon_on_triangulation(edge, mesh, location)?;
    let Some(polygon) = polygon.get() else {
        return Ok(None);
    };
    let nodes = polygon.nodes();
    Ok(Some(
        (nodes.lower()..=nodes.upper())
            .map(|i| nodes.value(i).copied().unwrap_or(0))
            .collect(),
    ))
}

fn check_shared_edges(
    shape: &Shape,
    faces: &IndexedMapOfShape,
    report: &mut MeshTopologyReport,
) -> Result<(), Failure> {
    let mut edge_faces = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(shape, ShapeEnum::Edge, ShapeEnum::Face, &mut edge_faces)?;
    for index in 1..=edge_faces.extent() {
        let edge = topods::edge(edge_faces.find_key(index)?)?;
        let owners: Vec<Face> = edge_faces
            .find_from_index(index)?
            .iter()
            .map(topods::face)
            .collect::<Result<_, _>>()?;
        if owners.len() >= 2 {
            check_edge(index, &edge, &owners, faces, report)?;
        }
    }
    Ok(())
}

fn check_edge(
    index: usize,
    edge: &Edge,
    owners: &[Face],
    faces: &IndexedMapOfShape,
    report: &mut MeshTopologyReport,
) -> Result<(), Failure> {
    let first = triangulation(&owners[0])?;
    let Some(first_nodes) = polygon_nodes(edge, &first)? else {
        return Ok(());
    };
    for other in &owners[1..] {
        let second = triangulation(other)?;
        let Some(second_nodes) = polygon_nodes(edge, &second)? else {
            continue;
        };
        if second_nodes.len() != first_nodes.len() {
            report.async_edges.push(index);
            break;
        }
        let tolerance = BRepTool::edge_tolerance(edge)?;
        let pair = [
            (&owners[0], &first, &first_nodes),
            (other, &second, &second_nodes),
        ];
        compare_polygons(pair, tolerance, faces, report)?;
    }
    Ok(())
}

type PolygonSide<'a> = (&'a Face, &'a Triangulated, &'a Vec<i32>);

fn compare_polygons(
    [a, b]: [PolygonSide<'_>; 2],
    tolerance: f64,
    faces: &IndexedMapOfShape,
    report: &mut MeshTopologyReport,
) -> Result<(), Failure> {
    let (face_a, face_b) = (
        faces.find_index(a.0.as_ref()),
        faces.find_index(b.0.as_ref()),
    );
    for (offset, (node_a, node_b)) in a.2.iter().zip(b.2.iter()).enumerate() {
        let point_a = node_point(a.0, a.1, *node_a)?;
        let point_b = node_point(b.0, b.1, *node_b)?;
        let distance = point_a.distance(&point_b);
        if distance * distance > tolerance * tolerance {
            let position = 1 + offset as i32;
            report
                .cross_face_errors
                .push((face_a, position, face_b, position, distance));
        }
    }
    Ok(())
}

/// A node of the face triangulation moved by the face location (as the native check does).
fn node_point(
    face: &Face,
    (mesh, _): &Triangulated,
    node: i32,
) -> Result<martensite_tkmath::gp::Point3, Failure> {
    let mesh = mesh
        .get()
        .ok_or_else(|| Failure::new("triangulation vanished; expected the face mesh"))?;
    Ok(mesh
        .node(node)?
        .transformed(face.location().transformation()))
}

fn boundary_nodes(face: &Face, mesh: &Triangulated) -> Result<BTreeSet<i32>, Failure> {
    let mut nodes = BTreeSet::new();
    let mut explorer = Explorer::new(face.as_ref(), ShapeEnum::Edge, ShapeEnum::Shape)?;
    while explorer.more() {
        if let Some(polygon) = polygon_nodes(&topods::edge(explorer.current())?, mesh)? {
            nodes.extend(polygon);
        }
        explorer.next()?;
    }
    Ok(nodes)
}

fn check_face(index: usize, face: &Face, report: &mut MeshTopologyReport) -> Result<(), Failure> {
    let mesh = triangulation(face)?;
    let Some(triangles) = mesh.0.get() else {
        report.untriangulated_faces.push(index);
        return Ok(());
    };
    let boundary = boundary_nodes(face, &mesh)?;
    let connect = Connect::new(&mesh.0)?;
    let mut used = BTreeSet::new();
    for triangle in 1..=triangles.nb_triangles() {
        let (n0, n1, n2) = triangles.triangle(triangle)?.get();
        let nodes = [n0, n1, n2];
        used.extend(nodes);
        let (t0, t1, t2) = connect.triangles(triangle)?;
        for (j, neighbour) in [t0, t1, t2].into_iter().enumerate() {
            let link = [nodes[j], nodes[(j + 1) % 3]];
            if neighbour == 0 && !(boundary.contains(&link[0]) && boundary.contains(&link[1])) {
                report.free_links.push((index, link));
            }
        }
    }
    let unused = (1..=triangles.nb_nodes()).filter(|node| !used.contains(node));
    report.free_nodes.extend(unused.map(|node| (index, node)));
    Ok(())
}
