//! `TopOpeBRepDSSolidSurfaceInterference`: an interference of a surface on a solid.
use super::TopOpeBRepDSInterference;
use crate::top_ope_brep_ds::{TopOpeBRepDSKind, TopOpeBRepDSTransition};
use martensite_tkernel::impl_transient;
use std::ops::Deref;

/// A solid/surface interference (`TopOpeBRepDSSolidSurfaceInterference`); the
/// interference members are reached through `Deref`;
/// `TopOpeBRepDSSolidSurfaceInterference::new(&t, TopOpeBRepDSKind::Solid, 1, TopOpeBRepDSKind::Surface, 1)`.
pub struct TopOpeBRepDSSolidSurfaceInterference {
    parent: TopOpeBRepDSInterference,
}
impl_transient!(
    TopOpeBRepDSSolidSurfaceInterference,
    "TopOpeBRepDSSolidSurfaceInterference",
    TopOpeBRepDSInterference,
    parent
);

impl Deref for TopOpeBRepDSSolidSurfaceInterference {
    type Target = TopOpeBRepDSInterference;

    fn deref(&self) -> &TopOpeBRepDSInterference {
        &self.parent
    }
}

impl TopOpeBRepDSSolidSurfaceInterference {
    /// `TopOpeBRepDSSolidSurfaceInterference(Transition, SupportType, Support,
    /// GeometryType, Geometry)`.
    pub fn new(
        t: &TopOpeBRepDSTransition,
        st: TopOpeBRepDSKind,
        s: i32,
        gt: TopOpeBRepDSKind,
        g: i32,
    ) -> Self {
        Self {
            parent: TopOpeBRepDSInterference::new(t, st, s, gt, g),
        }
    }
}
