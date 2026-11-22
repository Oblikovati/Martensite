//! Keep a bounded witness for a partial upload from one immutable vertex stream to its successor.
use super::{DrawStream, PrimitiveDraw, RenderVertex};
use std::fmt;
use std::num::NonZeroUsize;
use std::ops::Range;
use std::sync::{Arc, Weak};

pub(crate) struct VertexRevision {
    previous: StreamWitness,
    current: StreamWitness,
    dirty_vertices: Range<usize>,
}

struct StreamWitness {
    identity: NonZeroUsize,
    // Pins the allocation address so a later stream cannot accidentally match it.
    _owner: Weak<Vec<RenderVertex>>,
}

impl StreamWitness {
    fn of(stream: &DrawStream<RenderVertex>) -> Option<Self> {
        Some(Self {
            identity: stream.identity()?,
            _owner: stream.witness()?,
        })
    }
}

impl VertexRevision {
    pub(crate) fn previous_identity(&self) -> NonZeroUsize {
        self.previous.identity
    }

    pub(crate) fn dirty_vertices(&self) -> Range<usize> {
        self.dirty_vertices.clone()
    }
}

#[derive(Clone, Default)]
pub(crate) struct VertexRevisionHint(Option<Arc<VertexRevision>>);

// Upload provenance does not change the draw's value or introduce addresses in diagnostics.
impl PartialEq for VertexRevisionHint {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Debug for VertexRevisionHint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("VertexRevisionHint")
    }
}

impl PrimitiveDraw {
    pub(crate) fn vertex_revision(&self) -> Option<&VertexRevision> {
        let revision = self.vertex_revision.0.as_deref()?;
        (self.vertices.identity() == Some(revision.current.identity)).then_some(revision)
    }

    pub(crate) fn retain_vertex_edit(&mut self, previous: &Self) {
        if !same_topology(self, previous) {
            return;
        }
        self.indices = previous.indices.clone();
        self.quad_corners = previous.quad_corners.clone();
        self.native_flat_sources = previous.native_flat_sources.clone();
        if let Some(dirty) = changed_range(&previous.vertices, &self.vertices) {
            self.vertex_revision = edit_hint(&previous.vertices, &self.vertices, dirty);
        } else {
            self.vertices = previous.vertices.clone();
            self.vertex_revision = previous.vertex_revision.clone();
        }
    }
}

fn same_topology(current: &PrimitiveDraw, previous: &PrimitiveDraw) -> bool {
    current.vertices.len() == previous.vertices.len()
        && current.indices == previous.indices
        && current.quad_corners == previous.quad_corners
        && current.native_flat_sources == previous.native_flat_sources
}

fn changed_range(previous: &[RenderVertex], current: &[RenderVertex]) -> Option<Range<usize>> {
    let equal = |(old, new)| vertex_bits_equal(old, new);
    let first = previous.iter().zip(current).position(|pair| !equal(pair))?;
    let changed_tail = previous[first..].iter().zip(&current[first..]);
    let unchanged_tail = changed_tail
        .rev()
        .take_while(|&(old, new)| vertex_bits_equal(old, new))
        .count();
    Some(first..current.len() - unchanged_tail)
}

fn vertex_bits_equal(previous: &RenderVertex, current: &RenderVertex) -> bool {
    previous.position.map(f32::to_bits) == current.position.map(f32::to_bits)
        && previous.weight.to_bits() == current.weight.to_bits()
        && previous.color.map(f32::to_bits) == current.color.map(f32::to_bits)
}

fn edit_hint(
    previous: &DrawStream<RenderVertex>,
    current: &DrawStream<RenderVertex>,
    dirty_vertices: Range<usize>,
) -> VertexRevisionHint {
    let Some((previous, current)) = StreamWitness::of(previous).zip(StreamWitness::of(current))
    else {
        return VertexRevisionHint::default();
    };
    VertexRevisionHint(Some(Arc::new(VertexRevision {
        previous,
        current,
        dirty_vertices,
    })))
}
