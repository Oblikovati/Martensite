//! `TopOpeBRepDSEdgeVertexInterference`: an interference of a vertex on an edge (or
//! another support) at a parameter of the edge.
use super::{
    InterferenceRecord, InterferenceState, ShapeInterferenceRecord, ShapeInterferenceState,
};
use crate::top_ope_brep_ds::{TopOpeBRepDSConfig, TopOpeBRepDSKind, TopOpeBRepDSTransition};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::SharedState;
use std::ops::Deref;

/// An edge/vertex interference (`TopOpeBRepDSEdgeVertexInterference`); the
/// shape/shape members are reached through `Deref`;
/// `TopOpeBRepDSEdgeVertexInterference::on_edge(&t, 1, 2, true, TopOpeBRepDSConfig::UnshGeometry, 0.5)`.
pub struct TopOpeBRepDSEdgeVertexInterference {
    record: ShapeInterferenceState,
    my_param: SharedState<f64>,
}
impl_transient!(
    TopOpeBRepDSEdgeVertexInterference,
    "TopOpeBRepDSEdgeVertexInterference",
    dyn ShapeInterferenceRecord,
    interfaces: [dyn InterferenceRecord, dyn ShapeInterferenceRecord]
);

impl InterferenceRecord for TopOpeBRepDSEdgeVertexInterference {
    fn interference_state(&self) -> &InterferenceState {
        &self.record
    }
}

impl ShapeInterferenceRecord for TopOpeBRepDSEdgeVertexInterference {
    fn shape_interference_state(&self) -> &ShapeInterferenceState {
        &self.record
    }
}

impl Deref for TopOpeBRepDSEdgeVertexInterference {
    type Target = ShapeInterferenceState;

    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

impl TopOpeBRepDSEdgeVertexInterference {
    /// `TopOpeBRepDSEdgeVertexInterference(T, ST, S, G, GIsBound, C, P)`: the geometry
    /// is the VERTEX `g`.
    pub fn new(
        t: &TopOpeBRepDSTransition,
        st: TopOpeBRepDSKind,
        s: i32,
        g: i32,
        g_is_bound: bool,
        c: TopOpeBRepDSConfig,
        p: f64,
    ) -> Self {
        let record =
            ShapeInterferenceState::new(t, st, s, TopOpeBRepDSKind::Vertex, g, g_is_bound, c);
        Self {
            record,
            my_param: SharedState::new(p),
        }
    }

    /// `TopOpeBRepDSEdgeVertexInterference(T, S, G, GIsBound, C, P)`: the support is
    /// the EDGE `s`.
    pub fn on_edge(
        t: &TopOpeBRepDSTransition,
        s: i32,
        g: i32,
        g_is_bound: bool,
        c: TopOpeBRepDSConfig,
        p: f64,
    ) -> Self {
        Self::new(t, TopOpeBRepDSKind::Edge, s, g, g_is_bound, c, p)
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
