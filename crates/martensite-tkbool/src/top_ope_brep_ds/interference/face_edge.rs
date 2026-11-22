//! `TopOpeBRepDSFaceEdgeInterference`: an interference of an edge on a face.
use super::{
    InterferenceRecord, InterferenceState, ShapeInterferenceRecord, ShapeInterferenceState,
};
use crate::top_ope_brep_ds::{TopOpeBRepDSConfig, TopOpeBRepDSKind, TopOpeBRepDSTransition};
use martensite_tkernel::impl_transient;
use std::ops::Deref;

/// A face/edge interference (`TopOpeBRepDSFaceEdgeInterference`): the support is the
/// FACE `s`, the geometry the EDGE `g`; the shape/shape members are reached through
/// `Deref`; `TopOpeBRepDSFaceEdgeInterference::new(&t, 1, 2, false, TopOpeBRepDSConfig::SameOriented)`.
pub struct TopOpeBRepDSFaceEdgeInterference {
    record: ShapeInterferenceState,
}
impl_transient!(
    TopOpeBRepDSFaceEdgeInterference,
    "TopOpeBRepDSFaceEdgeInterference",
    dyn ShapeInterferenceRecord,
    interfaces: [dyn InterferenceRecord, dyn ShapeInterferenceRecord]
);

impl InterferenceRecord for TopOpeBRepDSFaceEdgeInterference {
    fn interference_state(&self) -> &InterferenceState {
        &self.record
    }
}

impl ShapeInterferenceRecord for TopOpeBRepDSFaceEdgeInterference {
    fn shape_interference_state(&self) -> &ShapeInterferenceState {
        &self.record
    }
}

impl Deref for TopOpeBRepDSFaceEdgeInterference {
    type Target = ShapeInterferenceState;

    fn deref(&self) -> &Self::Target {
        &self.record
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
            record: ShapeInterferenceState::new(
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
