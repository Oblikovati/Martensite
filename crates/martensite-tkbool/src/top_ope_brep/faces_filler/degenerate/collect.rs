//! Vertex-to-edge incidence collection preserves the native carried vertex between sides.
use super::*;

impl TopOpeBRepFacesFiller<'_> {
    pub(super) fn collect_degenerate_data(
        &self,
        line: &TopOpeBRepLineInter,
        entries: &mut ShapeEdges,
    ) -> Result<(), Failure> {
        let (mut ranks, mut closing, mut degenerate) = (
            ShapeRanks::default(),
            ShapeEdges::default(),
            ShapeEdges::default(),
        );
        for index in 1..=line.nb_vpoint() {
            let point = line.vpoint(index)?;
            self.collect_point_edges(point, &mut ranks, &mut closing, &mut degenerate)?;
        }
        let peers = vertex_peers(&ranks, &degenerate)?;
        for (shape, _) in degenerate.items() {
            if shape.shape_type()? == ShapeEnum::Edge {
                self.collect_degenerate_entry(shape, &ranks, &closing, &peers, entries)?;
            }
        }
        self.fill_degenerate_peers(&ranks, &peers)
    }

    fn collect_point_edges(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        ranks: &mut ShapeRanks,
        closing: &mut ShapeEdges,
        degenerate: &mut ShapeEdges,
    ) -> Result<(), Failure> {
        let (a, b) = (point.is_vertex(1)?, point.is_vertex(2)?);
        if !a && !b {
            return Ok(());
        }
        let mut vertex = point.vertex(if a { 1 } else { 2 })?;
        for side in 1..=2 {
            if point.is_vertex(side)? {
                vertex = point.vertex(side)?;
                ranks.bind(vertex.clone(), side);
            }
            if point.shape_index() == side || point.shape_index() == 3 {
                let edge = topods::edge(&point.edge(side)?)?;
                ranks.bind(edge.clone().into_shape(), side);
                if BRepTool::degenerated(&edge)? {
                    add_vertex_edge(degenerate, &vertex, edge.as_ref())?;
                } else if TopOpeBRepToolShapeTool::closed(
                    edge.as_ref(),
                    self.faces[(side - 1) as usize].as_ref(),
                )? {
                    add_vertex_edge(closing, &vertex, edge.as_ref())?;
                }
            }
        }
        Ok(())
    }

    fn collect_degenerate_entry(
        &self,
        shape: &Shape,
        ranks: &ShapeRanks,
        closing: &ShapeEdges,
        peers: &VertexPeers,
        entries: &mut ShapeEdges,
    ) -> Result<(), Failure> {
        let rank = *ranks.find(shape)?;
        let edge = topods::edge(shape)?;
        let vertex = first_vertex(shape)?;
        let vertex_rank = *ranks.find(vertex.as_ref())?;
        let peer = if peers.is_bound(vertex.as_ref()) {
            Some(peers.find(vertex.as_ref())?)
        } else {
            None
        };
        let Some(seam) = self.find_closing_edge(&edge, &vertex, rank, ranks, closing, peer)? else {
            return Ok(());
        };
        let key = if vertex_rank == rank {
            vertex.into_shape()
        } else {
            peer.cloned().ok_or_else(|| {
                Failure::with_message(
                    "Standard_NoSuchObject",
                    "GetdgData: peer=None; expected opposite-rank degenerate vertex",
                )
            })?
        };
        let mut pair = List::default();
        pair.append(seam);
        pair.append(shape.clone());
        entries.bind(key, pair);
        Ok(())
    }

    fn find_closing_edge(
        &self,
        edge: &Edge,
        vertex: &Vertex,
        rank: i32,
        ranks: &ShapeRanks,
        closing: &ShapeEdges,
        peer: Option<&Shape>,
    ) -> Result<Option<Shape>, Failure> {
        let has_vertex = closing.is_bound(vertex.as_ref());
        let has_peer = peer.is_some_and(|shape| closing.is_bound(shape));
        if !has_vertex && !has_peer {
            let face = &self.faces[(rank - 1) as usize];
            let mut ancestors = IndexedDataMapOfShapeListOfShape::default();
            topexp::map_shapes_and_ancestors(
                face.as_ref(),
                ShapeEnum::Vertex,
                ShapeEnum::Edge,
                &mut ancestors,
            )?;
            for candidate in ancestors.find_from_key(vertex.as_ref())? {
                if !candidate.is_same(edge.as_ref())
                    && TopOpeBRepToolShapeTool::closed(candidate, face.as_ref())?
                {
                    return Ok(Some(candidate.clone()));
                }
            }
        }
        // The native loop is intentionally empty for a peer-only closing binding.
        if has_vertex {
            for candidate in closing.find(vertex.as_ref())? {
                if *ranks.find(candidate)? == rank {
                    return Ok(Some(candidate.clone()));
                }
            }
        }
        Ok(None)
    }

    fn fill_degenerate_peers(
        &self,
        ranks: &ShapeRanks,
        peers: &VertexPeers,
    ) -> Result<(), Failure> {
        for (_, vertex) in peers.items() {
            let opposite = peers.find(vertex)?;
            let rank = *ranks.find(vertex)?;
            let (a, b) = if rank == 1 {
                (vertex, opposite)
            } else {
                (opposite, vertex)
            };
            self.change_data_structure()?
                .fill_shapes_same_domain(a, b, true)?;
        }
        Ok(())
    }
}

fn add_vertex_edge(map: &mut ShapeEdges, vertex: &Shape, edge: &Shape) -> Result<(), Failure> {
    let vertex_bound = map.is_bound(vertex);
    let edge_bound = map.is_bound(edge);
    if vertex_bound && edge_bound && map.find(vertex)?.iter().any(|item| item.is_same(edge)) {
        return Ok(());
    }
    if !vertex_bound {
        map.bind(vertex.clone(), List::default());
    }
    if !edge_bound {
        map.bind(edge.clone(), List::default());
    }
    map.change_find(vertex)?.append(edge.clone());
    map.change_find(edge)?.append(vertex.clone());
    Ok(())
}

fn vertex_peers(ranks: &ShapeRanks, degenerate: &ShapeEdges) -> Result<VertexPeers, Failure> {
    let mut peers = VertexPeers::default();
    for (vertex, edges) in degenerate.items() {
        if vertex.shape_type()? != ShapeEnum::Vertex {
            continue;
        }
        for edge in edges {
            if ranks.find(vertex)? != ranks.find(edge)? {
                let peer = first_vertex(edge)?.into_shape();
                peers.bind(vertex.clone(), peer.clone());
                peers.bind(peer, vertex.clone());
            }
        }
    }
    Ok(peers)
}

pub(super) fn first_vertex(edge: &Shape) -> Result<Vertex, Failure> {
    let explorer = Explorer::new(edge, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    if !explorer.more() {
        return Err(Failure::with_message(
            "Standard_NoSuchObject",
            "GetdgData: edge has 0 vertices; expected at least 1",
        ));
    }
    topods::vertex(explorer.current())
}
