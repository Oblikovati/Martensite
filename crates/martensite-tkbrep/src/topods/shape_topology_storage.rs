//! Private scoped topology projection keeps native const-reference reads uncounted; every value copy acquires ownership.
use super::TShape;
use martensite_tkernel::standard::Handle;
use std::ops::Deref;
pub(super) enum ShapeTopologyStorage<'a> {
    Owned(Handle<dyn TShape>),
    Observed(&'a Handle<dyn TShape>),
}
impl Default for ShapeTopologyStorage<'_> {
    fn default() -> Self {
        let topology = Handle::default();
        Self::Owned(topology)
    }
}
impl Deref for ShapeTopologyStorage<'_> {
    type Target = Handle<dyn TShape>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(topology) => topology,
            Self::Observed(topology) => topology,
        }
    }
}
impl ShapeTopologyStorage<'_> {
    pub(super) fn nullify(&mut self) {
        // Rust assignment destroys the prior variant before replacing its pointer field.
        *self = Self::default();
    }
}
