use super::*;
use Orientation::Forward;

type Edges = BTreeMap<Ref, Vec<EdgeUse>>;
type Germ = (Ref, bool);
type Link = BTreeMap<Germ, Vec<Germ>>;

pub(super) fn vertices(edges: &Edges) -> BTreeSet<Ref> {
    edges
        .values()
        .flatten()
        .flat_map(|item| item.ends)
        .collect()
}

pub(super) fn certify(
    faces: &[FaceUse],
    edges: &Edges,
    vertices: &BTreeSet<Ref>,
    stock: Option<&wire::StockWire<'_>>,
) -> Checked<i64> {
    let ids = faces.iter().map(|f| f.face).collect::<BTreeSet<_>>();
    let (linked, germs) = pair_edges(edges, stock)?;
    certify_vertex_links(faces, &germs)?;
    certify_connected(&linked, &ids)?;
    euler(faces, edges.len(), vertices.len())
}

fn canonical(item: &EdgeUse) -> [Ref; 2] {
    if item.sense == Forward {
        item.ends
    } else {
        [item.ends[1], item.ends[0]]
    }
}

type Faces = BTreeMap<Ref, Vec<Ref>>;
type Germs = BTreeMap<Ref, BTreeSet<Germ>>;

fn pair_edges(edges: &Edges, stock: Option<&wire::StockWire<'_>>) -> Checked<(Faces, Germs)> {
    let mut linked = Faces::new();
    let mut germs = Germs::new();
    for (&edge, uses) in edges {
        let [a, b] = uses.as_slice() else {
            return Err(Refusal::EdgeUses {
                edge,
                actual: uses.len(),
            });
        };
        if a.sense == b.sense {
            return Err(Refusal::EdgeSenses {
                edge,
                actual: [a.sense, b.sense],
            });
        }
        let ends = canonical(a);
        if ends != canonical(b) {
            return Err(Refusal::EdgeEnds(edge));
        }
        if a.face == b.face && !stock.is_some_and(|s| s.band == a.face && s.generator == edge) {
            return Err(Refusal::SameFace { edge, face: a.face });
        }
        linked.entry(a.face).or_default().push(b.face);
        linked.entry(b.face).or_default().push(a.face);
        germs.entry(ends[0]).or_default().insert((edge, false));
        germs.entry(ends[1]).or_default().insert((edge, true));
    }
    Ok((linked, germs))
}

fn certify_vertex_links(faces: &[FaceUse], expected: &Germs) -> Checked<()> {
    let mut links = BTreeMap::<Ref, Link>::new();
    for wire in faces.iter().flat_map(|face| &face.wires) {
        for (a, b) in wire.iter().zip(wire.iter().cycle().skip(1)) {
            let vertex = a.ends[1];
            let end = (a.edge, a.sense == Forward);
            let start = (b.edge, b.sense != Forward);
            if vertex != b.ends[0] || end == start {
                return Err(Refusal::VertexGerms(vertex));
            }
            let link = links.entry(vertex).or_default();
            link.entry(end).or_default().push(start);
            link.entry(start).or_default().push(end);
        }
    }
    for (&vertex, germs) in expected {
        let actual = links.remove(&vertex).ok_or(Refusal::VertexGerms(vertex))?;
        check_link(vertex, germs, &actual)?;
    }
    if let Some((&vertex, _)) = links.first_key_value() {
        return Err(Refusal::VertexGerms(vertex));
    }
    Ok(())
}

fn check_link(vertex: Ref, germs: &BTreeSet<Germ>, actual: &Link) -> Checked<()> {
    if actual.keys().ne(germs) {
        return Err(Refusal::VertexGerms(vertex));
    }
    for (&(edge, _), arms) in actual {
        if arms.len() != 2 {
            return Err(Refusal::VertexDegree {
                vertex,
                edge,
                actual: arms.len(),
            });
        }
    }
    let connected = reachable(actual, germs).len();
    if connected != germs.len() {
        return Err(Refusal::VertexFan {
            vertex,
            connected,
            actual: germs.len(),
        });
    }
    Ok(())
}

fn certify_connected(linked: &Faces, faces: &BTreeSet<Ref>) -> Checked<()> {
    let reached = reachable(linked, faces);
    if reached.is_empty() || reached != *faces {
        return Err(Refusal::FaceComponents {
            connected: reached.len(),
            actual: faces.len(),
        });
    }
    Ok(())
}

fn reachable<K: Ord + Copy>(graph: &BTreeMap<K, Vec<K>>, expected: &BTreeSet<K>) -> BTreeSet<K> {
    let mut found = BTreeSet::new();
    let mut pending = expected.first().copied().into_iter().collect::<Vec<_>>();
    while let Some(item) = pending.pop() {
        if found.insert(item) {
            pending.extend(graph.get(&item).into_iter().flatten().copied());
        }
    }
    found
}

fn euler(faces: &[FaceUse], edges: usize, vertices: usize) -> Checked<i64> {
    // Each oriented face with b natural boundary cycles contributes 2-b.
    let wire_count = faces.iter().map(|f| f.wires.len() as i128).sum::<i128>();
    let value = vertices as i128 - edges as i128 + 2 * (faces.len() as i128) - wire_count;
    let chi = i64::try_from(value).map_err(|_| Refusal::Euler(i64::MIN))?;
    if chi > 2 || chi % 2 != 0 {
        return Err(Refusal::Euler(chi));
    }
    Ok(1 - chi / 2)
}
