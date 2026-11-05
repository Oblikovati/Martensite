// The vertex-edge ancestor-map step of BRepOffsetSimpleOffset::FillOffsetData.
use super::{
    OffsetEdgeGeometry, OffsetVertexGeometry, OffsetVertexIncidentEdge, build_simple_offset_vertex,
};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Edge, Shape, Vertex};
use martensite_tkbrep::toptools::IndexedDataMapOfShapeListOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// A source vertex and its computed simple-offset result.
#[derive(Clone)]
pub struct OffsetVertexData {
    /// The source vertex represented by this map entry.
    pub vertex: Vertex,
    /// The centroid and tolerance derived from incident offset curves.
    pub geometry: OffsetVertexGeometry,
}

/// Applies the reference vertex-to-edge ancestor map to already computed edge offsets.
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkoffset::brep_offset::build_simple_offset_vertices;
/// assert!(build_simple_offset_vertices(&Shape::default(), &[]).unwrap().is_empty());
/// ```
pub fn build_simple_offset_vertices(
    shape: &Shape,
    edge_offsets: &[(Edge, Option<OffsetEdgeGeometry>)],
) -> Result<Vec<OffsetVertexData>, Failure> {
    let map = vertex_edge_map(shape)?;
    let mut results: Vec<OffsetVertexData> = Vec::new();
    for index in 1..=map.extent() {
        append_vertex_result(&map, index, edge_offsets, &mut results)?;
    }
    Ok(results)
}

fn vertex_edge_map(shape: &Shape) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
    let mut map = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(shape, ShapeEnum::Vertex, ShapeEnum::Edge, &mut map)?;
    Ok(map)
}

fn append_vertex_result(
    map: &IndexedDataMapOfShapeListOfShape,
    index: usize,
    edge_offsets: &[(Edge, Option<OffsetEdgeGeometry>)],
    results: &mut Vec<OffsetVertexData>,
) -> Result<(), Failure> {
    let vertex_shape = map.find_key(index)?.clone();
    let vertex = Vertex::unchecked(vertex_shape);
    let incident = incident_edge_offsets(map.find_from_index(index)?, edge_offsets);
    let Some(geometry) = build_simple_offset_vertex(&vertex, &incident)? else {
        return Ok(());
    };
    results.push(OffsetVertexData { vertex, geometry });
    Ok(())
}

fn incident_edge_offsets(
    incident_shapes: &martensite_tkernel::ncollection::List<Shape>,
    edge_offsets: &[(Edge, Option<OffsetEdgeGeometry>)],
) -> Vec<OffsetVertexIncidentEdge> {
    incident_shapes
        .iter()
        .map(|shape| incident_edge_offset(Edge::unchecked(shape.clone()), edge_offsets))
        .collect()
}

fn incident_edge_offset(
    edge: Edge,
    edge_offsets: &[(Edge, Option<OffsetEdgeGeometry>)],
) -> OffsetVertexIncidentEdge {
    let geometry = edge_offsets
        .iter()
        .find(|(mapped, _)| mapped.is_equal(&edge))
        .and_then(|(_, geometry)| geometry.clone());
    OffsetVertexIncidentEdge { edge, geometry }
}
