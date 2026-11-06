//! Preserve seed/vertex/edge iteration and the original stop-on-all-edges rule.
use crate::bop_algo::{Options, ProgressCancellation, builder_area::progress::observe_user_break};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp,
    topods::{Shape, ShapeIterator, Wire},
    toptools::IndexedDataMapOfShapeListOfShape,
};
use martensite_tkernel::{
    message::ProgressScope,
    ncollection::{List, Map},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) struct InternalWireWalk {
    vertex_edges: IndexedDataMapOfShapeListOfShape,
    added: Map<Shape>,
    total: usize,
    more: bool,
}

impl InternalWireWalk {
    /// Prepare the same source incidence map before seeds; `InternalWireWalk::new(edges)`.
    pub(super) fn new(edges: &[Shape]) -> Result<Self, Failure> {
        let vertex_edges = internal_vertex_edges(edges)?;
        Ok(Self {
            vertex_edges,
            added: Map::new(1),
            total: edges.len(),
            more: true,
        })
    }

    /// Append a real seed wire or stop at the source boundary; `walk.append_seed(edge, loops, scope, options, observed)`.
    pub(super) fn append_seed(
        &mut self,
        edge: &Shape,
        loops: &mut List<Shape>,
        scope: &ProgressScope<'_>,
        options: &Options,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        if !self.more {
            return Ok(false);
        }
        if self.added.add(edge.clone()) {
            if observe_user_break(options, scope, observed) {
                return Ok(false);
            }
            loops.append(self.grow(edge)?.into_shape());
        }
        Ok(true)
    }

    /// A wire of seed and connected edges; more becomes false once all total edges were added.
    fn grow(&mut self, seed: &Shape) -> Result<Wire, Failure> {
        let builder = BRepBuilder::default();
        let mut wire = Wire::default();
        builder.make_wire(&mut wire);
        let mut wire = wire.into_shape();
        builder.add(&mut wire, seed)?;
        let mut position = 0;
        while self.more {
            let Some(edge) = ShapeIterator::new(&wire, true, true).nth(position) else {
                break;
            };
            position += 1;
            self.grow_edge(&edge, &mut wire, &builder)?;
        }
        wire.set_closed(BRepTool::is_closed(&wire)?)?;
        Ok(Wire::unchecked(wire))
    }

    fn grow_edge(
        &mut self,
        edge: &Shape,
        wire: &mut Shape,
        builder: &BRepBuilder,
    ) -> Result<(), Failure> {
        for vertex in ShapeIterator::new(edge, true, true) {
            if !self.more {
                break;
            }
            self.grow_vertex(&vertex, wire, builder)?;
        }
        Ok(())
    }

    fn grow_vertex(
        &mut self,
        vertex: &Shape,
        wire: &mut Shape,
        builder: &BRepBuilder,
    ) -> Result<(), Failure> {
        for other in self.vertex_edges.find_from_key(vertex)?.iter() {
            if !self.more {
                break;
            }
            if !self.added.add(other.clone()) {
                continue;
            }
            builder.add(wire, other)?;
            if self.added.extent() == self.total {
                self.more = false;
            }
        }
        Ok(())
    }
}

fn internal_vertex_edges(edges: &[Shape]) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
    let mut vertex_edges = IndexedDataMapOfShapeListOfShape::default();
    for edge in edges {
        topexp::map_shapes_and_ancestors(
            edge,
            ShapeEnum::Vertex,
            ShapeEnum::Edge,
            &mut vertex_edges,
        )?;
    }
    Ok(vertex_edges)
}

#[cfg(test)]
mod contract {
    use super::InternalWireWalk;

    #[test]
    fn face_wire_seed_walk_thread_bounds_are_structural() {
        static_assertions::assert_impl_all!(InternalWireWalk: Send, Sync);
        static_assertions::assert_impl_all!(crate::bop_algo::BuilderFace: Send, Sync);
        static_assertions::assert_impl_all!(crate::bop_algo::WireSplitter: Send, Sync);
        static_assertions::assert_impl_all!(martensite_tkernel::message::ProgressScope<'static>: Send, Sync);
        static_assertions::assert_not_impl_any!(martensite_tkernel::message::ProgressRange<'static>: Sync);
        assert!(InternalWireWalk::new(&[]).unwrap().more);
    }
}
