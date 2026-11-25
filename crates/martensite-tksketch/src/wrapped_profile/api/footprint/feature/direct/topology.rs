//! Exact, source-bound manifold topology of the freshly decoded attached solid.
use super::*;
use std::collections::BTreeSet;
use stock::OriginalBoundary;
mod incidence;
#[cfg(test)]
pub(super) mod tests;
mod wire;

type Checked<T> = Result<T, Refusal>;
type Ref = CylinderBodyRef;

#[derive(Debug)]
pub(super) enum Refusal {
    Native(Failure),
    Index(Box<CylinderFeatureFailure>),
    Root {
        kind: ShapeEnum,
        sense: Orientation,
        children: usize,
    },
    Child {
        expected: ShapeEnum,
        actual: ShapeEnum,
    },
    Orientation {
        kind: ShapeEnum,
        actual: Orientation,
    },
    Shell {
        sense: Orientation,
        closed: bool,
    },
    Inventory(Ref),
    DuplicateFace(Ref),
    EmptyFace(Ref),
    MultipleWiresAtVertex {
        face: Ref,
        vertex: Ref,
    },
    EdgeVertices {
        edge: Ref,
        count: usize,
    },
    EdgeEndpointOrientation {
        edge: Ref,
        actual: [Orientation; 2],
    },
    Degenerate(Ref),
    Wire {
        face: Ref,
        defect: wire::Defect,
    },
    StockBoundary(Ref),
    EdgeUses {
        edge: Ref,
        actual: usize,
    },
    EdgeSenses {
        edge: Ref,
        actual: [Orientation; 2],
    },
    EdgeEnds(Ref),
    SameFace {
        edge: Ref,
        face: Ref,
    },
    VertexGerms(Ref),
    VertexDegree {
        vertex: Ref,
        edge: Ref,
        actual: usize,
    },
    VertexFan {
        vertex: Ref,
        connected: usize,
        actual: usize,
    },
    FaceComponents {
        connected: usize,
        actual: usize,
    },
    Euler(i64),
}

impl Refusal {
    pub(super) fn feature(self, attempt: Attempt) -> CylinderFeatureFailure {
        let query = "exact decoded attached topology";
        let cause = match self {
            Self::Native(error) => error,
            Self::Index(error) => return *error,
            other => Failure::new(&other.to_string()),
        };
        attempt.fail(
            CylinderFeatureStage::Validation,
            CylinderFeatureCause::Native {
                query,
                cause: Box::new(cause),
            },
        )
    }
}

impl From<Failure> for Refusal {
    fn from(error: Failure) -> Self {
        Self::Native(error)
    }
}
impl From<CylinderFeatureFailure> for Refusal {
    fn from(error: CylinderFeatureFailure) -> Self {
        Self::Index(Box::new(error))
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(err) => write!(f, "native query failed: {err}"),
            Self::Index(err) => write!(f, "could not index decoded owner: {err}"),
            Self::Root {
                kind,
                sense,
                children,
            } => write!(
                f,
                "expected one forward solid shell, got {kind:?} {sense:?} with {children} children"
            ),
            Self::Child { expected, actual } => {
                write!(f, "expected child {expected:?}, got {actual:?}")
            }
            Self::Orientation { kind, actual } => {
                write!(f, "expected binary {kind:?} orientation, got {actual:?}")
            }
            Self::Shell { sense, closed } => write!(
                f,
                "expected closed forward shell, got {sense:?}, closed={closed}"
            ),
            Self::Inventory(id) => write!(
                f,
                "native archive reference {id:?} does not resolve to its exact indexed occurrence"
            ),
            Self::DuplicateFace(face) => write!(f, "duplicate shell face {face:?}"),
            Self::EmptyFace(face) => write!(f, "face {face:?} has no boundary wires"),
            Self::MultipleWiresAtVertex { face, vertex } => {
                write!(f, "face {face:?} has distinct wires meeting at {vertex:?}")
            }
            Self::EdgeVertices { edge, count } => {
                write!(f, "edge {edge:?} has {count} vertex uses, expected two")
            }
            Self::EdgeEndpointOrientation { edge, actual } => write!(
                f,
                "edge {edge:?} endpoint orientations {actual:?}, expected one forward and one reversed"
            ),
            Self::Degenerate(edge) => write!(f, "edge {edge:?} is degenerate"),
            Self::Wire { face, defect } => write!(f, "face {face:?} wire: {defect}"),
            Self::StockBoundary(band) => write!(
                f,
                "expected the original rebound periodic wire exactly once on {band:?}"
            ),
            Self::EdgeUses { edge, actual } => {
                write!(f, "edge {edge:?} used {actual} times, expected two")
            }
            Self::EdgeSenses { edge, actual } => write!(
                f,
                "edge {edge:?} uses {actual:?}, expected opposite orientations"
            ),
            Self::EdgeEnds(edge) => write!(
                f,
                "edge {edge:?} canonical endpoint identities differ across uses"
            ),
            Self::SameFace { edge, face } => write!(
                f,
                "edge {edge:?} repeats on {face:?} outside the original periodic wire"
            ),
            Self::VertexGerms(vertex) => write!(
                f,
                "vertex {vertex:?} link does not contain exactly its physical edge ends"
            ),
            Self::VertexDegree {
                vertex,
                edge,
                actual,
            } => write!(
                f,
                "vertex {vertex:?} end of {edge:?} has {actual} corners, expected two"
            ),
            Self::VertexFan {
                vertex,
                connected,
                actual,
            } => write!(
                f,
                "vertex {vertex:?} link has a disconnected fan ({connected} of {actual} edge ends)"
            ),
            Self::FaceComponents { connected, actual } => write!(
                f,
                "shell has disconnected face components ({connected} of {actual} faces)"
            ),
            Self::Euler(value) => write!(
                f,
                "invalid Euler characteristic {value} for a connected orientable closed shell"
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Topology {
    faces: Vec<FaceUse>,
    edges: BTreeMap<Ref, Vec<EdgeUse>>,
    vertices: BTreeSet<Ref>,
    genus: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FaceUse {
    face: Ref,
    wires: Vec<Vec<DirectedUse>>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DirectedUse {
    pub(super) edge: Ref,
    pub(super) sense: Orientation,
    pub(super) ends: [Ref; 2],
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct EdgeUse {
    face: Ref,
    sense: Orientation,
    ends: [Ref; 2],
}

impl Topology {
    pub(super) fn capture(
        attempt: Attempt,
        body: &DetachedCylinderFeature,
        stock: &OriginalBoundary,
    ) -> Checked<Self> {
        let index = body::Index::new(attempt, &body.body, CylinderFeatureStage::Encoding)?;
        let stock = wire::StockWire::resolve(attempt, &index, body, stock)?;
        Self::capture_indexed(attempt, body, &index, Some(&stock))
    }

    fn capture_indexed(
        attempt: Attempt,
        body: &DetachedCylinderFeature,
        index: &body::Index,
        stock: Option<&wire::StockWire<'_>>,
    ) -> Checked<Self> {
        let shell = shell(&body.body)?;
        let faces = wire::faces(attempt, index, &shell, stock)?;
        let edges = collect_edges(&faces);
        let vertices = incidence::vertices(&edges);
        let genus = incidence::certify(&faces, &edges, &vertices, stock)?;
        complete_inventory(index, body)?;
        Ok(Self {
            faces,
            edges,
            vertices,
            genus,
        })
    }

    pub(super) fn counts(&self) -> [usize; 4] {
        let wires = self.faces.iter().map(|face| face.wires.len()).sum();
        [
            self.faces.len(),
            self.edges.len(),
            self.vertices.len(),
            wires,
        ]
    }
    pub(super) fn wires(&self, face: Ref) -> Option<&[Vec<DirectedUse>]> {
        self.faces
            .iter()
            .find(|row| row.face == face)
            .map(|row| row.wires.as_slice())
    }
    pub(super) fn face_uses(&self) -> impl Iterator<Item = (Ref, &DirectedUse)> {
        self.faces
            .iter()
            .flat_map(|f| f.wires.iter().flatten().map(|edge| (f.face, edge)))
    }
    pub(super) fn genus(&self) -> i64 {
        self.genus
    }
}

fn shell(root: &Shape) -> Checked<Shape> {
    let kind = root.shape_type()?;
    let sense = root.orientation();
    let all = children(root)?;
    let children = all.len();
    if kind != ShapeEnum::Solid || sense != Orientation::Forward {
        return Err(Refusal::Root {
            kind,
            sense,
            children,
        });
    }
    let [shell]: [_; 1] = all.try_into().map_err(|_| Refusal::Root {
        kind,
        sense,
        children,
    })?;
    require_kind(&shell, ShapeEnum::Shell)?;
    let (sense, closed) = (shell.orientation(), shell.closed()?);
    if sense != Orientation::Forward || !closed {
        return Err(Refusal::Shell { sense, closed });
    }
    Ok(shell)
}

fn collect_edges(faces: &[FaceUse]) -> BTreeMap<Ref, Vec<EdgeUse>> {
    let mut edges = BTreeMap::<Ref, Vec<_>>::new();
    for face in faces {
        for edge in face.wires.iter().flatten() {
            edges.entry(edge.edge).or_default().push(EdgeUse {
                face: face.face,
                sense: edge.sense,
                ends: edge.ends,
            });
        }
    }
    edges
}

fn complete_inventory(index: &body::Index, owner: &DetachedCylinderFeature) -> Checked<()> {
    let mut named = owner.entities.iter();
    for item in index.members() {
        let (id, indexed) = item?;
        let Some((&key, decoded)) = named.next() else {
            return Err(Refusal::Inventory(id));
        };
        if id != key || !decoded.is_equal(indexed) {
            return Err(Refusal::Inventory(id.min(key)));
        }
    }
    if let Some((&key, _)) = named.next() {
        return Err(Refusal::Inventory(key));
    }
    Ok(())
}

fn child_kind(root: &Shape, expected: ShapeEnum) -> Checked<Vec<Shape>> {
    let children = children(root)?;
    for child in &children {
        require_kind(child, expected)?;
    }
    Ok(children)
}

fn children(root: &Shape) -> Checked<Vec<Shape>> {
    let mut cursor = ShapeIterator::new(root, true, true);
    let mut children = Vec::new();
    while cursor.more() {
        children.push(cursor.value()?.to_shape());
        cursor.next();
    }
    Ok(children)
}

fn require_kind(shape: &Shape, expected: ShapeEnum) -> Checked<()> {
    let actual = shape.shape_type()?;
    if actual == expected {
        return Ok(());
    }
    Err(Refusal::Child { expected, actual })
}

fn reference(attempt: Attempt, index: &body::Index, shape: &Shape) -> Checked<Ref> {
    let kind = shape.shape_type()?;
    index
        .reference(shape, attempt)?
        .ok_or(Refusal::Inventory(Ref { kind, ordinal: 0 }))
}

fn oriented(shape: &Shape) -> Checked<Orientation> {
    let actual = shape.orientation();
    match actual {
        Orientation::Forward | Orientation::Reversed => Ok(actual),
        _ => Err(Refusal::Orientation {
            kind: shape.shape_type()?,
            actual,
        }),
    }
}
