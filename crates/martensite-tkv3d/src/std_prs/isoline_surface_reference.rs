//! Native const handle slots are read anew across valid application callbacks.
use super::StdPrsIsolineSurfaceTarget;
use martensite_tkernel::standard::{Handle, SharedState};
pub(super) enum IsolineSurfaceReference<'a, S: StdPrsIsolineSurfaceTarget + ?Sized> {
    Borrowed(&'a Handle<S>),
    Live(&'a SharedState<Handle<S>>),
}
impl<S: StdPrsIsolineSurfaceTarget + ?Sized> IsolineSurfaceReference<'_, S> {
    pub(super) fn snapshot(&self) -> Handle<S> {
        // A method callback may replace a live slot; never retain a lock during virtual dispatch.
        match self {
            Self::Borrowed(source) => (*source).clone(),
            Self::Live(source) => source.snapshot(),
        }
    }
}
