//! Copy-on-write geometry for draw snapshots and resident GPU buffer keys.
use std::{
    fmt,
    num::NonZeroUsize,
    ops::Deref,
    sync::{Arc, Weak},
};

#[derive(Clone)]
pub(crate) struct DrawStream<T>(Option<Arc<Vec<T>>>);

impl<T> From<Vec<T>> for DrawStream<T> {
    fn from(elements: Vec<T>) -> Self {
        // Moving the Vec keeps its existing allocation. Most draws leave several streams empty.
        Self((!elements.is_empty()).then(|| Arc::new(elements)))
    }
}

impl<T> Default for DrawStream<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Deref for DrawStream<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_deref().map_or(&[], Vec::as_slice)
    }
}

impl<T: fmt::Debug> fmt::Debug for DrawStream<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(formatter)
    }
}

impl<T: PartialEq> PartialEq for DrawStream<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> DrawStream<T> {
    /// The allocation address, valid as a cache key while its weak witness is kept.
    pub(crate) fn identity(&self) -> Option<NonZeroUsize> {
        self.0
            .as_ref()
            .and_then(|elements| NonZeroUsize::new(Arc::as_ptr(elements).addr()))
    }

    /// A witness pins the address without retaining the vertex or index allocation after drop.
    pub(crate) fn witness(&self) -> Option<Weak<Vec<T>>> {
        self.0.as_ref().map(Arc::downgrade)
    }

    /// Release this draw's reference without copying an otherwise shared stream.
    pub(crate) fn clear(&mut self) {
        self.0 = None;
    }
}

impl<T: Clone> DrawStream<T> {
    /// A mutable stream detaches from other draws and any weak GPU cache witness.
    pub(crate) fn make_mut(&mut self) -> &mut Vec<T> {
        Arc::make_mut(self.0.get_or_insert_with(Arc::default))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PrimitiveDraw, RenderLighting, RenderMaterial, RenderShading, RenderVertex};

    #[test]
    fn cloned_stream_detaches_and_preserves_original_contents() {
        let source = DrawStream::from(vec![1, 2, 3]);
        let mut clone = source.clone();
        assert_eq!(source.identity(), clone.identity());
        clone.make_mut()[1] = 5;
        assert_ne!(source.identity(), clone.identity());
        assert_eq!(&*source, [1, 2, 3]);
        assert_eq!(&*clone, [1, 5, 3]);
    }

    #[test]
    fn mutation_detaches_even_with_only_a_weak_witness() {
        let mut stream = DrawStream::from(vec![1, 2, 3]);
        let identity = stream.identity();
        let witness = stream.witness().unwrap();
        stream.make_mut()[1] = 5;
        assert_ne!(stream.identity(), identity);
        assert!(witness.upgrade().is_none());
        stream.clear();
        assert_eq!(stream.identity(), None);
        assert_eq!(stream, DrawStream::default());
    }

    #[test]
    fn draw_and_lighting_clones_share_geometry_until_edited() {
        let source = draw_with_all_streams();
        let mut clone = source.clone();
        assert_eq!(geometry_identities(&source), geometry_identities(&clone));
        let normals = &source.lighting.as_ref().unwrap().normals;
        let cloned_normals = &mut clone.lighting.as_mut().unwrap().normals;
        assert_eq!(normals.identity(), cloned_normals.identity());
        cloned_normals.make_mut()[0] = [1., 0., 0.];
        assert_eq!(normals[0], [0., 0., 1.]);
        assert_ne!(normals.identity(), cloned_normals.identity());
    }

    fn draw_with_all_streams() -> PrimitiveDraw {
        let matrix = crate::identity_clip();
        let vertex = RenderVertex::homogeneous([0., 0., 0., 1.], [1.; 4]);
        let mut source = PrimitiveDraw::new(vec![vertex; 3], vec![0, 1, 2], matrix).unwrap();
        source.quad_corners = vec![[0, 1, 2, 0]].into();
        source.native_flat_sources = vec![0].into();
        let lighting = RenderLighting::new(
            vec![[0., 0., 1.]; 3],
            RenderMaterial::default(),
            matrix,
            matrix,
            false,
            RenderShading::Phong,
        )
        .unwrap();
        source.with_lighting(lighting).unwrap()
    }

    fn geometry_identities(draw: &PrimitiveDraw) -> [Option<NonZeroUsize>; 4] {
        [
            draw.vertices.identity(),
            draw.indices.identity(),
            draw.quad_corners.identity(),
            draw.native_flat_sources.identity(),
        ]
    }
}
