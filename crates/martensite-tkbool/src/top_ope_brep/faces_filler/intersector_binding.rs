//! Owned operations and borrowed callers use the same intersector mutation path.
use super::*;
use std::ops::{Deref, DerefMut};

pub(super) enum IntersectorBinding<'a> {
    Borrowed(&'a mut TopOpeBRepFacesIntersector),
    Owned(Box<TopOpeBRepFacesIntersector>),
}
impl Deref for IntersectorBinding<'_> {
    type Target = TopOpeBRepFacesIntersector;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(pair) => pair,
            Self::Owned(pair) => pair,
        }
    }
}
impl DerefMut for IntersectorBinding<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Borrowed(pair) => pair,
            Self::Owned(pair) => pair,
        }
    }
}
impl TopOpeBRepFacesFiller<'static> {
    /// Takes ownership of an intersection without cloning its shared line array; `filler.bind_owned_context(&f1, &f2, pair, &hds)?`.
    pub fn bind_owned_context(
        &mut self,
        first: &Shape,
        second: &Shape,
        pair: TopOpeBRepFacesIntersector,
        hds: &Handle<TopOpeBRepDSHDataStructure>,
    ) -> Result<(), Failure> {
        self.bind_intersector(
            first,
            second,
            IntersectorBinding::Owned(Box::new(pair)),
            hds,
        )
    }
}
