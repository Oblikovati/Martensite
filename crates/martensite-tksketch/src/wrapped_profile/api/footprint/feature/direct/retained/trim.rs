use super::native_snapshot::{
    Checked, EdgeGeometry, FaceGeometry, FaceTrace, LocationBits, edge_geometry, face_geometry,
    face_trace, location_bits,
};
use super::{BRepTool, Failure, Orientation, Shape, ShapeEnum, ShapeIterator, topods};
use martensite_tkbrep::toptools::IndexedMapOfShape;
#[derive(Debug, PartialEq, Eq)]
struct EdgeUse<E, V> {
    edge: E,
    orientation: Orientation,
    space: Option<EdgeGeometry>,
    trace: FaceTrace,
    vertices: Vec<(V, Orientation, u64)>,
}

#[derive(Debug, PartialEq, Eq)]
struct Wire<E, V> {
    orientation: Orientation,
    location: LocationBits,
    edges: Vec<EdgeUse<E, V>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Face<E, V> {
    pub(super) surface: FaceGeometry,
    wires: Vec<Wire<E, V>>,
}

pub(super) struct ShapeKeys<K> {
    shapes: IndexedMapOfShape,
    keys: Vec<K>,
}

impl<K: Copy> ShapeKeys<K> {
    pub(super) fn new() -> Self {
        Self {
            shapes: IndexedMapOfShape::new(17),
            keys: Vec::new(),
        }
    }
    pub(super) fn add(&mut self, shape: &Shape, key: K) -> bool {
        let index = self.shapes.add(shape.clone());
        if index != self.keys.len() + 1 {
            return false;
        }
        self.keys.push(key);
        true
    }
    fn get(&self, shape: &Shape) -> Option<K> {
        let index = self.shapes.find_index(shape).checked_sub(1)?;
        self.keys.get(index).copied()
    }
}

impl<E: Copy, V: Copy> Face<E, V> {
    pub(super) fn capture(
        f: &topods::Face,
        wires: impl IntoIterator<Item = Shape>,
        ids: (&ShapeKeys<E>, &ShapeKeys<V>),
        full_edges: bool,
    ) -> Checked<Self> {
        let forward = topods::face(&f.oriented(Orientation::Forward))?;
        let wires = wires
            .into_iter()
            .map(|wire| {
                let edges = direct_children(&wire, ShapeEnum::Edge)?
                    .iter()
                    .map(|use_| edge_use(use_, &forward, ids, full_edges))
                    .collect::<Checked<Vec<_>>>()?;
                Ok(Wire {
                    orientation: wire.orientation(),
                    location: location_bits(wire.location())?,
                    edges,
                })
            })
            .collect::<Checked<Vec<_>>>()?;
        Ok(Face {
            surface: face_geometry(f)?,
            wires,
        })
    }
}

fn edge_use<E: Copy, V: Copy>(
    shape: &Shape,
    face: &topods::Face,
    ids: (&ShapeKeys<E>, &ShapeKeys<V>),
    full: bool,
) -> Checked<EdgeUse<E, V>> {
    let edge = topods::edge(shape)?;
    let edge_id = ids
        .0
        .get(&edge)
        .ok_or_else(|| Failure::new("edge outside native owner map"))?;
    let vertices = direct_children(&edge, ShapeEnum::Vertex)?
        .iter()
        .map(|v| {
            let id = ids
                .1
                .get(v)
                .ok_or_else(|| Failure::new("vertex outside native owner map"))?;
            let vertex = topods::vertex(v)?;
            let parameter = BRepTool::parameter(&vertex, &edge)?.to_bits();
            Ok((id, v.orientation(), parameter))
        })
        .collect::<Checked<Vec<_>>>()?;
    Ok(EdgeUse {
        edge: edge_id,
        orientation: edge.orientation(),
        space: full.then(|| edge_geometry(&edge, None)).transpose()?,
        trace: face_trace(face, &edge)?,
        vertices,
    })
}

fn direct_children(shape: &Shape, expected: ShapeEnum) -> Checked<Vec<Shape>> {
    let mut children = ShapeIterator::new(shape, true, true);
    let mut shapes = Vec::new();
    while children.more() {
        let child = children.value()?;
        if child.shape_type()? != expected {
            return Err(Failure::new("unexpected child type in owned boundary"));
        }
        shapes.push(child.to_shape());
        children.next();
    }
    Ok(shapes)
}

pub(super) fn normalized_wires(face: &topods::Face) -> Checked<Vec<Shape>> {
    direct_children(&face.oriented(Orientation::Forward), ShapeEnum::Wire)
}
