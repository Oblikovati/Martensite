use super::*;
use Orientation::{Forward, Reversed};

#[derive(Debug)]
pub(in super::super) enum Defect {
    NotClosed {
        ordinal: usize,
    },
    Duplicate {
        ordinal: usize,
    },
    Empty {
        ordinal: usize,
    },
    RepeatedEdge {
        ordinal: usize,
        edge: Ref,
    },
    Branch {
        ordinal: usize,
        vertex: Ref,
    },
    Disconnected {
        ordinal: usize,
        reached: usize,
        actual: usize,
    },
    Periodic {
        ordinal: usize,
        generator: Ref,
        uses: usize,
    },
}

impl std::fmt::Display for Defect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotClosed { ordinal } => write!(f, "wire {ordinal} is not closed"),
            Self::Duplicate { ordinal } => {
                write!(f, "wire {ordinal} repeats an actual wire identity")
            }
            Self::Empty { ordinal } => write!(f, "wire {ordinal} has no edge uses"),
            Self::RepeatedEdge { ordinal, edge } => {
                write!(f, "wire {ordinal} repeats unauthorized edge {edge:?}")
            }
            Self::Branch { ordinal, vertex } => write!(f, "wire {ordinal} branches at {vertex:?}"),
            Self::Disconnected {
                ordinal,
                reached,
                actual,
            } => write!(
                f,
                "wire {ordinal} reaches {reached} of {actual} edge uses in its cycle"
            ),
            Self::Periodic {
                ordinal,
                generator,
                uses,
            } => write!(
                f,
                "wire {ordinal} has invalid original periodic boundary with generator {generator:?} and {uses} uses"
            ),
        }
    }
}

pub(super) struct StockWire<'a> {
    pub(super) band: Ref,
    original: &'a Shape,
    rims: [Ref; 2],
    pub(super) generator: Ref,
    anchors: [Ref; 2],
}

impl<'a> StockWire<'a> {
    pub(super) fn resolve(
        attempt: Attempt,
        index: &body::Index,
        owner: &DetachedCylinderFeature,
        stock: &'a OriginalBoundary,
    ) -> Checked<Self> {
        let band = reference(attempt, index, &stock.band)?;
        if !owner
            .entities
            .get(&band)
            .is_some_and(|shape| shape.is_equal(&stock.band))
        {
            return Err(Refusal::Inventory(band));
        }
        let rims = [
            reference(attempt, index, &stock.rims[0])?,
            reference(attempt, index, &stock.rims[1])?,
        ];
        let generator = reference(attempt, index, &stock.generator)?;
        let anchors = [
            reference(attempt, index, &stock.anchors[0])?,
            reference(attempt, index, &stock.anchors[1])?,
        ];
        if rims[0] == rims[1] || rims.contains(&generator) || anchors[0] == anchors[1] {
            return Err(Refusal::StockBoundary(band));
        }
        Ok(Self {
            band,
            original: &stock.wire,
            rims,
            generator,
            anchors,
        })
    }

    fn on_wire(&self, face: Ref, wire: &Shape) -> bool {
        face == self.band && self.original.is_equal(wire)
    }
}

pub(super) fn faces(
    attempt: Attempt,
    index: &body::Index,
    shell: &Shape,
    stock: Option<&StockWire<'_>>,
) -> Checked<Vec<FaceUse>> {
    let mut seen = BTreeSet::new();
    child_kind(shell, ShapeEnum::Face)?
        .iter()
        .map(|face| {
            oriented(face)?;
            let id = reference(attempt, index, face)?;
            if !seen.insert(id) {
                return Err(Refusal::DuplicateFace(id));
            }
            face_use(attempt, index, face, id, stock)
        })
        .collect()
}

fn face_use(
    attempt: Attempt,
    index: &body::Index,
    face: &Shape,
    id: Ref,
    stock: Option<&StockWire<'_>>,
) -> Checked<FaceUse> {
    let children = child_kind(face, ShapeEnum::Wire)?;
    if children.is_empty() {
        return Err(Refusal::EmptyFace(id));
    }
    let mut indexed = martensite_tkbrep::toptools::IndexedMapOfShape::default();
    let mut vertices = BTreeSet::new();
    let mut original_count = 0;
    let mut wires = Vec::new();
    for (i, wire) in children.iter().enumerate() {
        let ordinal = i + 1;
        let before = indexed.extent();
        if indexed.add(wire.clone()) <= before {
            return Err(Refusal::Wire {
                face: id,
                defect: Defect::Duplicate { ordinal },
            });
        }
        let periodic = stock.filter(|s| s.on_wire(id, wire));
        original_count += usize::from(periodic.is_some());
        let uses = wire_use(attempt, index, wire, id, ordinal, periodic)?;
        let local: BTreeSet<_> = uses.iter().flat_map(|edge| edge.ends).collect();
        if let Some(&vertex) = local.iter().find(|v| vertices.contains(*v)) {
            return Err(Refusal::MultipleWiresAtVertex { face: id, vertex });
        }
        vertices.extend(local);
        wires.push(uses);
    }
    if stock.is_some_and(|s| s.band == id) && original_count != 1 {
        return Err(Refusal::StockBoundary(id));
    }
    Ok(FaceUse { face: id, wires })
}

fn wire_use(
    attempt: Attempt,
    index: &body::Index,
    wire: &Shape,
    face: Ref,
    ordinal: usize,
    periodic: Option<&StockWire<'_>>,
) -> Checked<Vec<DirectedUse>> {
    oriented(wire)?;
    let reject = |defect| Refusal::Wire { face, defect };
    if !wire.closed()? {
        return Err(reject(Defect::NotClosed { ordinal }));
    }
    let uses = child_kind(wire, ShapeEnum::Edge)?
        .iter()
        .map(|edge| edge_use(attempt, index, edge))
        .collect::<Checked<Vec<_>>>()?;
    let Some(first) = uses.first() else {
        return Err(reject(Defect::Empty { ordinal }));
    };
    let head = first.clone();
    match periodic {
        Some(stock) => periodic_cycle(uses, &head, stock, ordinal).map_err(reject),
        None => regular_cycle(uses, head.ends[0], ordinal).map_err(reject),
    }
}

fn edge_use(attempt: Attempt, index: &body::Index, edge: &Shape) -> Checked<DirectedUse> {
    let id = reference(attempt, index, edge)?;
    let sense = oriented(edge)?;
    if BRepTool::degenerated(&topods::edge(edge)?)? {
        return Err(Refusal::Degenerate(id));
    }
    let intrinsic = edge.oriented(Forward);
    let vertices = child_kind(&intrinsic, ShapeEnum::Vertex)?;
    let count = vertices.len();
    let [a, b]: [_; 2] = vertices
        .try_into()
        .map_err(|_| Refusal::EdgeVertices { edge: id, count })?;
    let actual = [a.orientation(), b.orientation()];
    let canonical = match actual {
        [Forward, Reversed] => [
            reference(attempt, index, &a)?,
            reference(attempt, index, &b)?,
        ],
        [Reversed, Forward] => [
            reference(attempt, index, &b)?,
            reference(attempt, index, &a)?,
        ],
        _ => return Err(Refusal::EdgeEndpointOrientation { edge: id, actual }),
    };
    let ends = if sense == Forward {
        canonical
    } else {
        [canonical[1], canonical[0]]
    };
    Ok(DirectedUse {
        edge: id,
        sense,
        ends,
    })
}

fn regular_cycle(
    uses: Vec<DirectedUse>,
    start: Ref,
    ordinal: usize,
) -> Result<Vec<DirectedUse>, Defect> {
    let actual = uses.len();
    let mut outgoing = BTreeMap::new();
    let mut incoming = BTreeSet::new();
    let mut identities = BTreeSet::new();
    for step in uses {
        if !identities.insert(step.edge) {
            return Err(Defect::RepeatedEdge {
                ordinal,
                edge: step.edge,
            });
        }
        let [a, b] = step.ends;
        if outgoing.insert(a, step).is_some() {
            return Err(Defect::Branch { ordinal, vertex: a });
        }
        if !incoming.insert(b) {
            return Err(Defect::Branch { ordinal, vertex: b });
        }
    }
    let mut ordered = Vec::with_capacity(actual);
    let mut current = start;
    while let Some(step) = outgoing.remove(&current) {
        current = step.ends[1];
        ordered.push(step);
    }
    if current != start || ordered.len() != actual {
        return Err(Defect::Disconnected {
            ordinal,
            reached: ordered.len(),
            actual,
        });
    }
    Ok(ordered)
}

fn periodic_cycle(
    uses: Vec<DirectedUse>,
    head: &DirectedUse,
    stock: &StockWire<'_>,
    ordinal: usize,
) -> Result<Vec<DirectedUse>, Defect> {
    let reject = Defect::Periodic {
        ordinal,
        generator: stock.generator,
        uses: uses.len(),
    };
    if uses.len() != 4 {
        return Err(reject);
    }
    let [low, high] = stock.anchors;
    let mut slots = [None, None, None, None];
    for step in uses {
        let i = match (step.edge, step.sense, step.ends) {
            (e, Forward, [a, b]) if e == stock.generator && [a, b] == [low, high] => 0,
            (e, _, [a, b]) if e == stock.rims[1] && [a, b] == [high; 2] => 1,
            (e, Reversed, [a, b]) if e == stock.generator && [a, b] == [high, low] => 2,
            (e, _, [a, b]) if e == stock.rims[0] && [a, b] == [low; 2] => 3,
            _ => return Err(reject),
        };
        if slots[i].replace(step).is_some() {
            return Err(reject);
        }
    }
    let [Some(up), Some(lid), Some(down), Some(base)] = slots else {
        return Err(reject);
    };
    let mut ordered = vec![up, lid, down, base];
    let Some(offset) = ordered.iter().position(|step| step == head) else {
        return Err(reject);
    };
    ordered.rotate_left(offset);
    Ok(ordered)
}
