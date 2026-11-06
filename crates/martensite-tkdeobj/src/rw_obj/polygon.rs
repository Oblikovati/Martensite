//! Splitting a polygon of five nodes or more into triangles, on the plane fitted to it.
use super::RwObjReader;
use martensite_tkernel::message::Message;
use martensite_tkernel::ncollection::IncrementalAllocatorObject;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::{self, Xy, Xyz};
use martensite_tkmesh::brep_mesh::{
    BRepMeshDataStructureOfDelaun, BRepMeshDelaun, BRepMeshEdge, BRepMeshVertex, DegreeOfFreedom,
};
use martensite_tkmesh::imesh_data::MEMORY_BLOCK_SIZE_HUGE;

fn node_xyz<R: RwObjReader + ?Sized>(reader: &R, node: i32) -> Xyz {
    Xyz::from(reader.get_node(node))
}

/// A triangle fan from the first node; returns the triangles added.
fn triangulate_fan<R: RwObjReader + ?Sized>(reader: &mut R, nodes: &[i32]) -> i32 {
    for pair in nodes[1..].windows(2) {
        reader.add_element(nodes[0], pair[0], pair[1], -1);
    }
    nodes.len() as i32 - 2
}

fn polygon_center<R: RwObjReader + ?Sized>(reader: &R, nodes: &[i32]) -> Xyz {
    if nodes.len() < 3 {
        return Xyz::new(0.0, 0.0, 0.0);
    }
    if nodes.len() == 4 {
        return (node_xyz(reader, nodes[0]) + node_xyz(reader, nodes[2])) / 2.0;
    }
    let sum = nodes.iter().fold(Xyz::new(0.0, 0.0, 0.0), |sum, node| {
        sum + node_xyz(reader, *node)
    });
    sum / nodes.len() as f64
}

/// The normal of a nearly planar polygon, summing the crossings around its center.
fn polygon_normal<R: RwObjReader + ?Sized>(reader: &R, nodes: &[i32]) -> Xyz {
    let center = polygon_center(reader, nodes);
    let direction = |node: i32| node_xyz(reader, node) - center;
    let mut normal = direction(nodes[nodes.len() - 1]).crossed(direction(nodes[0]));
    for pair in nodes.windows(2) {
        let mut delta = direction(pair[0]).crossed(direction(pair[1]));
        if normal.dot(delta) < 0.0 {
            delta *= -1.0;
        }
        normal += delta;
    }
    let modulus = normal.modulus();
    if modulus > gp::resolution() {
        normal /= modulus;
    }
    normal
}

/// A direction in the plane of a normal, built from its two largest coordinates.
fn plane_x_direction(normal: Xyz) -> Xyz {
    let coords = [normal.x(), normal.y(), normal.z()];
    let magnitudes = coords.map(f64::abs);
    let mut min = if magnitudes[0] < magnitudes[1] { 0 } else { 1 };
    min = if magnitudes[min] < magnitudes[2] {
        min
    } else {
        2
    };
    let (first, second) = ((min + 1) % 3, (min + 2) % 3);
    let mut result = [0.0; 3];
    result[first] = coords[second];
    result[second] = -coords[first];
    Xyz::new(result[0], result[1], result[2])
}

fn is_clockwise(vertices: &[Xy]) -> bool {
    let count = vertices.len();
    let sum: f64 = (0..count)
        .map(|index| {
            let (first, second) = (vertices[index], vertices[(index + 1) % count]);
            (second.x() - first.x()) * (second.y() + first.y())
        })
        .sum();
    sum < 0.0
}

/// Triangulates a polygon: a Delaunay mesh of its plane, a fan when that fails or is empty.
pub(super) fn triangulate_polygon<R: RwObjReader + ?Sized>(
    reader: &mut R,
    nodes: &[i32],
) -> Result<i32, Failure> {
    if nodes.len() < 3 {
        return Ok(0);
    }
    let normal = polygon_normal(reader, nodes);
    let x_direction = plane_x_direction(normal);
    let y_direction = normal.crossed(x_direction);
    let planar: Vec<Xy> = nodes
        .iter()
        .map(|node| {
            let point = node_xyz(reader, *node);
            Xy::new(x_direction.dot(point), y_direction.dot(point))
        })
        .collect();
    match delaunay_triangles(nodes, &planar) {
        Ok(triangles) if !triangles.is_empty() => {
            for [n1, n2, n3] in &triangles {
                reader.add_element(*n1, *n2, *n3, -1);
            }
            Ok(triangles.len() as i32)
        }
        Ok(_) => Ok(triangulate_fan(reader, nodes)),
        Err(failure) => {
            let text = format!("Error: exception raised during polygon split\n[{failure}]");
            Message::send_warning_text(&AsciiString::new(&text));
            Ok(triangulate_fan(reader, nodes))
        }
    }
}

/// The mesh of a polygon in its plane, as triangles of the original nodes.
fn delaunay_triangles(nodes: &[i32], planar: &[Xy]) -> Result<Vec<[i32; 3]>, Failure> {
    let allocator = Handle::new(IncrementalAllocatorObject::new(MEMORY_BLOCK_SIZE_HUGE));
    let mesh = Handle::new(BRepMeshDataStructureOfDelaun::new(
        &allocator,
        nodes.len() as i32,
    ));
    let structure = mesh.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            "polygon mesh: expected a new structure",
            "",
        )
    })?;
    let mut indices = Vec::with_capacity(nodes.len());
    for (node, point) in nodes.iter().zip(planar) {
        let vertex = BRepMeshVertex::new(*point, *node, DegreeOfFreedom::Frontier);
        indices.push(structure.add_node(&vertex, false)?);
    }
    let clockwise = is_clockwise(planar);
    add_boundary_links(structure, &indices, clockwise)?;
    BRepMeshDelaun::with_indices(&mesh, &mut indices, (-1, -1))?;
    let mut triangles = Vec::new();
    for id in structure.elements_of_domain().iter() {
        let element = structure.get_element(id)?;
        if element.movability() == DegreeOfFreedom::Deleted {
            continue;
        }
        let mut corners = structure.element_nodes(&element)?;
        if !clockwise {
            corners.swap(1, 2);
        }
        let location = |corner: i32| Ok::<i32, Failure>(structure.get_node(corner)?.location_3d());
        triangles.push([
            location(corners[0])?,
            location(corners[1])?,
            location(corners[2])?,
        ]);
    }
    Ok(triangles)
}

fn add_boundary_links(
    structure: &BRepMeshDataStructureOfDelaun,
    indices: &[i32],
    clockwise: bool,
) -> Result<(), Failure> {
    let count = indices.len();
    for index in 0..count {
        let next = (index + 1) % count;
        let (from, to) = if clockwise {
            (index, next)
        } else {
            (next, index)
        };
        let edge = BRepMeshEdge::new(indices[from], indices[to], DegreeOfFreedom::Frontier);
        structure.add_link(&edge)?;
    }
    Ok(())
}
