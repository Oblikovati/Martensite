//! `TopOpeBRepDSSolidSurfaceInterference`: an interference of a surface on a solid.
use super::{InterferenceRecord, InterferenceState};
use crate::top_ope_brep_ds::{TopOpeBRepDSKind, TopOpeBRepDSTransition};
use martensite_tkernel::impl_transient;
use std::ops::Deref;

/// A solid/surface interference (`TopOpeBRepDSSolidSurfaceInterference`); the
/// interference members are reached through `Deref`;
/// `TopOpeBRepDSSolidSurfaceInterference::new(&t, TopOpeBRepDSKind::Solid, 1, TopOpeBRepDSKind::Surface, 1)`.
pub struct TopOpeBRepDSSolidSurfaceInterference {
    record: InterferenceState,
}
impl_transient!(
    TopOpeBRepDSSolidSurfaceInterference,
    "TopOpeBRepDSSolidSurfaceInterference",
    dyn InterferenceRecord,
    interfaces: [dyn InterferenceRecord]
);

impl InterferenceRecord for TopOpeBRepDSSolidSurfaceInterference {
    fn interference_state(&self) -> &InterferenceState {
        &self.record
    }
}

impl Deref for TopOpeBRepDSSolidSurfaceInterference {
    type Target = InterferenceState;

    fn deref(&self) -> &Self::Target {
        &self.record
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
            record: InterferenceState::new(t, st, s, gt, g),
        }
    }
}
