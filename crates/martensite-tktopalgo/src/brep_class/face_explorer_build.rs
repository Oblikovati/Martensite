//! The constructor body of `BRepClass_FaceExplorer`: the wires and edges of the face,
//! the next edge of each edge and the edges probed for classification segments.
use super::edge::BRepClassEdge;
use super::face_explorer::WireData;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The anonymous `VertexEdges` of the native file: the first two edges met at a
/// vertex of a wire and the number of edge occurrences at it.
#[derive(Default)]
struct VertexEdges {
    first: Edge,
    second: Edge,
    count: u32,
}

impl VertexEdges {
    fn add(&mut self, edge: &Edge) {
        if self.count == 0 {
            self.first = edge.clone();
        } else if self.count == 1 {
            self.second = edge.clone();
        }
        self.count += 1;
    }
}

/// The anonymous `EdgeOccurrences`: the first and last index of an edge (IsSame) in
/// the edge list; the other occurrences chain through `next_occurrence`.
#[derive(Clone, Copy, Debug, Default)]
struct EdgeOccurrences {
    first: usize,
    last: usize,
}

/// What the native constructor fills: the wires, the edges and the probe edges.
pub(super) struct ExplorerTopology {
    pub(super) wires: Vec<WireData>,
    pub(super) edges: Vec<BRepClassEdge>,
    pub(super) probe_edges: Vec<usize>,
    occurrences: DataMap<Shape, EdgeOccurrences, ShapeMapHasher>,
    // `aNextOccurrence`: the next index of the same edge, None for `THE_NO_EDGE`.
    next_occurrence: Vec<Option<usize>>,
}

impl ExplorerTopology {
    /// The constructor body over `face` (already FORWARD): each wire, then the probe
    /// edges of the face.
    pub(super) fn of_face(face: &Face) -> Result<Self, Failure> {
        let mut topology = Self::empty();
        topology.capture_wires(face)?;
        topology.capture_probe_edges(face)?;
        Ok(topology)
    }

    fn empty() -> Self {
        Self {
            wires: Vec::new(),
            edges: Vec::new(),
            probe_edges: Vec::new(),
            occurrences: DataMap::default(),
            next_occurrence: Vec::new(),
        }
    }

    fn capture_wires(&mut self, face: &Face) -> Result<(), Failure> {
        let mut wires = Explorer::new(face, ShapeEnum::Wire, ShapeEnum::Shape)?;
        while wires.more() {
            self.add_wire(wires.current(), face)?;
            wires.next()?;
        }
        Ok(())
    }

    fn capture_probe_edges(&mut self, face: &Face) -> Result<(), Failure> {
        let mut edges = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)?;
        while edges.more() {
            let index = Edge::with_shape_reference(edges.current(), |edge| {
                self.probe_edge_index(edge, face)
            })?;
            self.probe_edges.push(index);
            edges.next()?;
        }
        Ok(())
    }

    // Current() and TopoDS::Edge return references: only the cached record
    // retains an edge. Borrowing avoids owned convenience copies per occurrence.
    fn add_wire(&mut self, wire: &Shape, face: &Face) -> Result<(), Failure> {
        let mut record = WireData {
            first_edge: self.edges.len(),
            nb_edges: 0,
        };
        let mut vertex_edges: DataMap<Shape, VertexEdges, ShapeMapHasher> = DataMap::default();
        self.capture_wire_edges(wire, face, &mut vertex_edges, &mut record)?;
        for index in record.first_edge..record.first_edge + record.nb_edges {
            link_next_edge(&mut self.edges[index], &vertex_edges);
        }
        self.wires.push(record);
        Ok(())
    }

    fn capture_wire_edges(
        &mut self,
        wire: &Shape,
        face: &Face,
        vertex_edges: &mut DataMap<Shape, VertexEdges, ShapeMapHasher>,
        record: &mut WireData,
    ) -> Result<(), Failure> {
        let mut edges = Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape)?;
        while edges.more() {
            Edge::with_shape_reference(edges.current(), |edge| {
                self.cache_wire_edge(edge, face, vertex_edges)
            })?;
            record.nb_edges += 1;
            edges.next()?;
        }
        Ok(())
    }

    fn cache_wire_edge(
        &mut self,
        edge: &Edge,
        face: &Face,
        vertex_edges: &mut DataMap<Shape, VertexEdges, ShapeMapHasher>,
    ) -> Result<(), Failure> {
        self.edges.push(cached_edge(edge, face)?);
        self.record_occurrence(edge, self.edges.len() - 1);
        let mut vertices = Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape)?;
        while vertices.more() {
            vertex_edges
                .try_emplaced(vertices.current().clone(), VertexEdges::default)
                .add(edge);
            vertices.next()?;
        }
        Ok(())
    }

    // Chains the new index `index` of `edge` after its last occurrence.
    fn record_occurrence(&mut self, edge: &Edge, index: usize) {
        self.next_occurrence.push(None);
        let Some(occurrence) = self.occurrences.change_seek(edge) else {
            let first = EdgeOccurrences {
                first: index,
                last: index,
            };
            self.occurrences.bind(Shape::clone(edge), first);
            return;
        };
        self.next_occurrence[occurrence.last] = Some(index);
        occurrence.last = index;
    }

    // The index of the wire edge equal (IsEqual) to `edge`, or of a new edge
    // appended for it when no wire holds it.
    fn probe_edge_index(&mut self, edge: &Edge, face: &Face) -> Result<usize, Failure> {
        let mut candidate: Option<usize> = self.occurrences.seek(edge).map(|found| found.first);
        while let Some(index) = candidate {
            if self.edges[index].edge().is_equal(edge) {
                return Ok(index);
            }
            candidate = self.next_occurrence.get(index).copied().flatten();
        }
        self.edges.push(cached_edge(edge, face)?);
        Ok(self.edges.len() - 1)
    }
}

// `BRepClass_Edge(E, F)` with `cacheGeometry`: the pcurve of the edge on the face and
// its range, when the edge has one.
fn cached_edge(edge: &Edge, face: &Face) -> Result<BRepClassEdge, Failure> {
    let mut data = BRepClassEdge::with_edge_face(edge.clone(), face.clone());
    let pcurve = BRepTool::curve_on_face(data.edge(), data.face())?;
    if !pcurve.pcurve.is_null() {
        data.set_geometry(pcurve.pcurve, pcurve.first, pcurve.last);
    }
    Ok(data)
}

// The next edge of `data`: the other edge at its last vertex (FORWARD-oriented
// vertices of the wire), when exactly two edge occurrences meet there and the edge
// is not closed on one vertex.
fn link_next_edge(
    data: &mut BRepClassEdge,
    vertex_edges: &DataMap<Shape, VertexEdges, ShapeMapHasher>,
) {
    let (first_vertex, last_vertex) = topexp::vertices(data.edge(), true);
    data.set_cached_vertices((first_vertex.clone(), last_vertex.clone()));
    if last_vertex.is_null() || last_vertex.is_same(&first_vertex) {
        return;
    }
    let Some(edges) = vertex_edges.seek(&last_vertex) else {
        return;
    };
    if edges.count != 2 {
        return;
    }
    let next: &Edge = if edges.first.is_same(data.edge()) {
        &edges.second
    } else {
        &edges.first
    };
    if !next.is_null() && !next.is_same(data.edge()) {
        data.set_next_edge(next.clone());
    }
}
