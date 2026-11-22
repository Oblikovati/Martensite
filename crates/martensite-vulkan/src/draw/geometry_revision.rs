//! Weak witnesses keep partial transfers tied to exact immutable geometry streams.
use super::{DrawStream, PrimitiveDraw, RenderVertex};
use std::fmt;
use std::num::NonZeroUsize;
use std::ops::Range;
use std::sync::{Arc, Weak};

#[derive(Clone, Copy)]
pub(crate) struct MutableGeometry {
    pub(crate) vertices: bool,
    pub(crate) indices: bool,
}

impl MutableGeometry {
    pub(crate) fn any(self) -> bool {
        self.vertices || self.indices
    }
}

#[derive(Clone)]
pub(crate) struct StreamRevision<T> {
    previous: StreamWitness<T>,
    current: StreamWitness<T>,
    dirty: Range<usize>,
}

#[derive(Clone)]
struct StreamWitness<T> {
    identity: NonZeroUsize,
    // Pins the allocation address until it can no longer match a pending upload.
    _owner: Weak<Vec<T>>,
}

impl<T> StreamWitness<T> {
    fn of(stream: &DrawStream<T>) -> Option<Self> {
        Some(Self {
            identity: stream.identity()?,
            _owner: stream.witness()?,
        })
    }
}

impl<T> StreamRevision<T> {
    fn new(previous: &DrawStream<T>, current: &DrawStream<T>, dirty: Range<usize>) -> Option<Self> {
        Some(Self {
            previous: StreamWitness::of(previous)?,
            current: StreamWitness::of(current)?,
            dirty,
        })
    }

    pub(crate) fn previous_identity(&self) -> NonZeroUsize {
        self.previous.identity
    }

    pub(crate) fn dirty_range(&self) -> Range<usize> {
        self.dirty.clone()
    }

    fn is_current(&self, stream: &DrawStream<T>) -> bool {
        stream.identity() == Some(self.current.identity)
    }
}

enum GeometryRevision {
    Vertices(StreamRevision<RenderVertex>),
    Indices(StreamRevision<u32>),
}

#[derive(Clone, Default)]
pub(crate) struct GeometryRevisionHint(Option<Arc<GeometryRevision>>);

// Upload provenance does not change the draw's value or introduce addresses in diagnostics.
impl PartialEq for GeometryRevisionHint {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Debug for GeometryRevisionHint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("GeometryRevisionHint")
    }
}

impl PrimitiveDraw {
    pub(crate) fn vertex_revision(&self) -> Option<&StreamRevision<RenderVertex>> {
        let GeometryRevision::Vertices(revision) = self.geometry_revision.0.as_deref()? else {
            return None;
        };
        revision.is_current(&self.vertices).then_some(revision)
    }

    pub(crate) fn index_revision(&self) -> Option<&StreamRevision<u32>> {
        let GeometryRevision::Indices(revision) = self.geometry_revision.0.as_deref()? else {
            return None;
        };
        revision.is_current(&self.indices).then_some(revision)
    }

    pub(crate) fn retain_geometry_edit(&mut self, previous: &Self, mutable: MutableGeometry) {
        if self.vertices.len() != previous.vertices.len()
            || self.indices.len() != previous.indices.len()
        {
            return;
        }
        let vertices_dirty = changed_range(&previous.vertices, &self.vertices, vertex_bits_equal);
        let indices_dirty = changed_range(&previous.indices, &self.indices, u32::eq);
        let same_corners = self.quad_corners == previous.quad_corners;
        let same_sources = self.native_flat_sources == previous.native_flat_sources;
        self.share_unchanged_streams(previous, same_corners, same_sources);
        self.retain_edits(
            previous,
            vertices_dirty,
            indices_dirty,
            MutableGeometry {
                vertices: mutable.vertices && same_corners && same_sources,
                indices: mutable.indices && same_corners,
            },
        );
    }

    fn share_unchanged_streams(&mut self, previous: &Self, same_corners: bool, same_sources: bool) {
        if same_corners {
            self.quad_corners = previous.quad_corners.clone();
        }
        if same_sources {
            self.native_flat_sources = previous.native_flat_sources.clone();
        }
    }

    fn retain_edits(
        &mut self,
        previous: &Self,
        vertices_dirty: Option<Range<usize>>,
        indices_dirty: Option<Range<usize>>,
        mutable: MutableGeometry,
    ) {
        self.share_unchanged_bindings(previous, vertices_dirty.is_none(), indices_dirty.is_none());
        let revision = match (vertices_dirty, indices_dirty) {
            (None, None) => {
                self.geometry_revision = previous.geometry_revision.clone();
                return;
            }
            (Some(dirty), None) if mutable.vertices => {
                StreamRevision::new(&previous.vertices, &self.vertices, dirty)
                    .map(GeometryRevision::Vertices)
            }
            (None, Some(dirty)) if mutable.indices => {
                StreamRevision::new(&previous.indices, &self.indices, dirty)
                    .map(GeometryRevision::Indices)
            }
            _ => None,
        };
        self.geometry_revision = GeometryRevisionHint(revision.map(Arc::new));
    }

    fn share_unchanged_bindings(&mut self, previous: &Self, vertices: bool, indices: bool) {
        if vertices {
            self.vertices = previous.vertices.clone();
        }
        if indices {
            self.indices = previous.indices.clone();
        }
        if vertices && indices {
            self.guard_band_bounds = previous.guard_band_bounds.clone();
        }
    }
}

fn changed_range<T>(
    previous: &[T],
    current: &[T],
    equal: impl Fn(&T, &T) -> bool,
) -> Option<Range<usize>> {
    let first = previous
        .iter()
        .zip(current)
        .position(|(old, new)| !equal(old, new))?;
    let unchanged_tail = previous[first..]
        .iter()
        .zip(&current[first..])
        .rev()
        .take_while(|&(old, new)| equal(old, new))
        .count();
    Some(first..current.len() - unchanged_tail)
}

fn vertex_bits_equal(previous: &RenderVertex, current: &RenderVertex) -> bool {
    previous.position.map(f32::to_bits) == current.position.map(f32::to_bits)
        && previous.weight.to_bits() == current.weight.to_bits()
        && previous.color.map(f32::to_bits) == current.color.map(f32::to_bits)
}
