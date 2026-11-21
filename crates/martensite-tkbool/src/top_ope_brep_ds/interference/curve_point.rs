//! `TopOpeBRepDSCurvePointInterference`: an interference of a point (or vertex) on a
//! curve (or edge) at a parameter.
use super::{InterferenceRecord, InterferenceState};
use crate::top_ope_brep_ds::{TopOpeBRepDSKind, TopOpeBRepDSTransition};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::SharedState;
use std::ops::Deref;

/// A curve/point interference (`TopOpeBRepDSCurvePointInterference`); the
/// interference members are reached through `Deref`;
/// `TopOpeBRepDSCurvePointInterference::new(&t, TopOpeBRepDSKind::Curve, 1, TopOpeBRepDSKind::Point, 2, 0.5)`.
pub struct TopOpeBRepDSCurvePointInterference {
    record: InterferenceState,
    my_param: SharedState<f64>,
}
impl_transient!(
    TopOpeBRepDSCurvePointInterference,
    "TopOpeBRepDSCurvePointInterference",
    dyn InterferenceRecord,
    interfaces: [dyn InterferenceRecord]
);

impl InterferenceRecord for TopOpeBRepDSCurvePointInterference {
    fn interference_state(&self) -> &InterferenceState {
        &self.record
    }
}

impl Deref for TopOpeBRepDSCurvePointInterference {
    type Target = InterferenceState;

    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

impl TopOpeBRepDSCurvePointInterference {
    /// `TopOpeBRepDSCurvePointInterference(T, ST, S, GT, G, P)`.
    pub fn new(
        t: &TopOpeBRepDSTransition,
        st: TopOpeBRepDSKind,
        s: i32,
        gt: TopOpeBRepDSKind,
        g: i32,
        p: f64,
    ) -> Self {
        Self {
            record: InterferenceState::new(t, st, s, gt, g),
            my_param: SharedState::new(p),
        }
    }

    /// `Parameter()`; `i.parameter()`.
    pub fn parameter(&self) -> f64 {
        *self.my_param.read()
    }

    /// `Parameter(P)`; `i.set_parameter(0.25)`.
    pub fn set_parameter(&self, p: f64) {
        *self.my_param.write() = p;
    }
}
