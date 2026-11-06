//! `tricheck shape`: the free links, cross face errors, async edges, free nodes and unconnected meshes of
//! the triangulations of a shape; returns what the command writes. No script here passes `-small`.
//! The command's `DrawTrSurf::Set(".")` markers draw for a person looking at the viewer, so they are left out.
use super::check_topology::MeshTestCheckTopology;
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{ncollection::Map, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tkmesh::brep_mesh::{BRepMeshEdge, DegreeOfFreedom};

fn free_links(check: &MeshTestCheckTopology, text: &mut String) -> Result<i32, Failure> {
    let mut count = 0;
    for k in 1..=check.nb_faces_with_fl() {
        let edges = check.nb_free_links(k)?;
        count += edges;
        *text += &format!("free links of face {}\n", check.get_face_num_with_fl(k)?);
        for i in 1..=edges {
            let (n1, n2) = check.get_free_link(k, i)?;
            *text += &format!("{{{n1} {n2}}} ");
        }
        text.push('\n');
    }
    Ok(count)
}

fn cross_face_errors(check: &MeshTestCheckTopology, text: &mut String) -> Result<i32, Failure> {
    let count = check.nb_cross_face_errors();
    if count > 0 {
        *text += "cross face errors: {face1, node1, face2, node2, distance}\n";
        for i in 1..=count {
            let (face1, node1, face2, node2, value) = check.get_cross_face_error(i)?;
            *text += &format!(
                "{{{face1} {node1} {face2} {node2} {}}} ",
                crate::reports::g17(value)
            );
        }
        text.push('\n');
    }
    Ok(count)
}

fn async_edges(check: &MeshTestCheckTopology, text: &mut String) -> Result<i32, Failure> {
    let count = check.nb_async_edges();
    if count > 0 {
        *text += "async edges:\n";
        for i in 1..=count {
            *text += &format!("{} ", check.get_async_edge_num(i)?);
        }
        text.push('\n');
    }
    Ok(count)
}

fn free_nodes(check: &MeshTestCheckTopology, text: &mut String) -> Result<i32, Failure> {
    let count = check.nb_free_nodes();
    if count > 0 {
        *text += "free nodes (in pairs: face / node): \n";
        for i in 1..=count {
            let (face, node) = check.get_free_node_num(i)?;
            *text += &format!("{{{face} {node}}} ");
        }
        text.push('\n');
    }
    Ok(count)
}

/// The boundary links of a face: its edges' polygons on its triangulation; `None` when there are none.
fn boundary_links(
    face_shape: &Shape,
    triangulation: &martensite_tkernel::standard::Handle<martensite_tkmath::poly::Triangulation>,
    location: &martensite_tkmath::toploc::Location,
) -> Result<Map<BRepMeshEdge>, Failure> {
    let mut links: Map<BRepMeshEdge> = Map::default();
    let mut edges = Explorer::new(face_shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        let edge = topods::edge(edges.current())?;
        let polygon = BRepTool::polygon_on_triangulation(&edge, triangulation, location)?;
        if let Some(polygon) = polygon.get() {
            let indices = polygon.nodes();
            let mut previous = -1;
            for j in indices.lower()..=indices.upper() {
                let node = *indices
                    .value(j)
                    .map_err(|error| error.into_failure("Array1"))?;
                if j != indices.lower() {
                    links.add(BRepMeshEdge::new(previous, node, DegreeOfFreedom::Frontier));
                }
                previous = node;
            }
        }
        edges.next()?;
    }
    Ok(links)
}

/// The last part of the command: a link used once inside a face, other than a boundary one.
fn unconnected_meshes(shape: &Shape, text: &mut String) -> Result<(), Failure> {
    let mut face_id = 1;
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        let face_shape = faces.current().clone();
        let face = topods::face(&face_shape)?;
        let (triangulation, location) = BRepTool::triangulation(&face, mesh_purpose::NONE)?;
        let boundary = boundary_links(&face_shape, &triangulation, &location)?;
        if boundary.extent() == 0 {
            break;
        }
        let mesh = triangulation
            .get()
            .ok_or_else(|| Failure::with_message("Standard_NullObject", "Triangulation"))?;
        let mut free: Map<BRepMeshEdge> = Map::default();
        for index in 1..=mesh.nb_triangles() {
            let triangle = mesh.triangle(index)?;
            let nodes = [triangle.value(1)?, triangle.value(2)?, triangle.value(3)?];
            for j in 1..=3 {
                let link = BRepMeshEdge::new(nodes[j - 1], nodes[j % 3], DegreeOfFreedom::Free);
                if !boundary.contains(&link) && !free.add(link) {
                    free.remove(&link);
                }
            }
        }
        if free.extent() != 0 {
            *text += &format!("Not connected mesh inside face {face_id}\n");
            for link in free.iter() {
                *text += &format!("{{{} {}}} ", link.first_node(), link.last_node());
            }
            text.push('\n');
        }
        faces.next()?;
        face_id += 1;
    }
    Ok(())
}

fn report(shape: &Shape) -> Result<String, Failure> {
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Face, &mut faces)?;
    let mut text = String::new();
    let mut check = MeshTestCheckTopology::new(shape);
    check.perform(&mut text)?;
    let links = free_links(&check, &mut text)?;
    let errors = cross_face_errors(&check, &mut text)?;
    let asynchronous = async_edges(&check, &mut text)?;
    let nodes = free_nodes(&check, &mut text)?;
    if links > 0 || errors > 0 || asynchronous > 0 || nodes > 0 {
        text += &format!(
            "Free_links {links} Cross_face_errors {errors} Async_edges {asynchronous} Free_nodes {nodes} Small triangles 0\n"
        );
    }
    unconnected_meshes(shape, &mut text)?;
    Ok(text)
}

/// The command's text, written to the stream and returned.
/// Example: `tricheck("result", &shape)`.
pub fn tricheck(name: &str, shape: &Shape) -> String {
    println!("tricheck {name} ");
    let text = report(shape).unwrap_or_else(|failure| panic!("tricheck {name}: {failure:?}"));
    print!("{text}");
    text
}
