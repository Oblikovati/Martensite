//! `TopOpeBRepDSFaceEdgeInterference`: an interference of an edge on a face.
use super::TopOpeBRepDSShapeShapeInterference;
use crate::top_ope_brep_ds::{TopOpeBRepDSConfig, TopOpeBRepDSKind, TopOpeBRepDSTransition};
use martensite_tkernel::impl_transient;
use std::ops::Deref;

/// A face/edge interference (`TopOpeBRepDSFaceEdgeInterference`): the support is the
/// FACE `s`, the geometry the EDGE `g`; the shape/shape members are reached through
/// `Deref`; `TopOpeBRepDSFaceEdgeInterference::new(&t, 1, 2, false, TopOpeBRepDSConfig::SameOriented)`.
pub struct TopOpeBRepDSFaceEdgeInterference {
    parent: TopOpeBRepDSShapeShapeInterference,
}
impl_transient!(
    TopOpeBRepDSFaceEdgeInterference,
    "TopOpeBRepDSFaceEdgeInterference",
    TopOpeBRepDSShapeShapeInterference,
    parent
);

impl Deref for TopOpeBRepDSFaceEdgeInterference {
    type Target = TopOpeBRepDSShapeShapeInterference;

    fn deref(&self) -> &TopOpeBRepDSShapeShapeInterference {
        &self.parent
    }
}

impl TopOpeBRepDSFaceEdgeInterference {
    /// `TopOpeBRepDSFaceEdgeInterference(T, S, G, GIsBound, C)`.
    pub fn new(
        t: &TopOpeBRepDSTransition,
        s: i32,
        g: i32,
        g_is_bound: bool,
        c: TopOpeBRepDSConfig,
    ) -> Self {
        Self {
            parent: TopOpeBRepDSShapeShapeInterference::new(
                t,
                TopOpeBRepDSKind::Face,
                s,
                TopOpeBRepDSKind::Edge,
                g,
                g_is_bound,
                c,
            ),
        }
    }
}
