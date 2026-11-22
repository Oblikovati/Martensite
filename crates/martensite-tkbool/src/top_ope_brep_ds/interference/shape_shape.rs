//! `TopOpeBRepDSShapeShapeInterference`: an interference of a shape on a shape, with
//! the bound flag of its geometry and the orientation configuration of the shapes.
use super::{InterferenceRecord, InterferenceState};
use crate::top_ope_brep_ds::{TopOpeBRepDSConfig, TopOpeBRepDSKind, TopOpeBRepDSTransition};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::SharedState;
use std::ops::Deref;

/// A shape/shape interference (`TopOpeBRepDSShapeShapeInterference`); the
/// interference members are reached through `Deref`;
/// `TopOpeBRepDSShapeShapeInterference::new(&t, TopOpeBRepDSKind::Face, 1, TopOpeBRepDSKind::Edge, 2, false, TopOpeBRepDSConfig::SameOriented)`.
pub struct TopOpeBRepDSShapeShapeInterference {
    record: ShapeInterferenceState,
}
impl_transient!(
    TopOpeBRepDSShapeShapeInterference,
    "TopOpeBRepDSShapeShapeInterference",
    dyn InterferenceRecord,
    interfaces: [dyn InterferenceRecord, dyn ShapeInterferenceRecord]
);
parent_interface!(
    dyn ShapeInterferenceRecord,
    TopOpeBRepDSShapeShapeInterference,
    [dyn InterferenceRecord]
);

/// Common attachment, bound flag and orientation fields of shape interferences.
pub struct ShapeInterferenceState {
    record: InterferenceState,
    my_gbound: SharedState<bool>,
    my_c: TopOpeBRepDSConfig,
}

/// The shape/shape attachment role of a standalone record or a specialized edge/vertex or face/edge record.
pub trait ShapeInterferenceRecord: InterferenceRecord {
    /// Borrow the shared shape fields; `record.shape_interference_state().config()`.
    fn shape_interference_state(&self) -> &ShapeInterferenceState;
}

impl ShapeInterferenceRecord for TopOpeBRepDSShapeShapeInterference {
    fn shape_interference_state(&self) -> &ShapeInterferenceState {
        &self.record
    }
}

impl InterferenceRecord for TopOpeBRepDSShapeShapeInterference {
    fn interference_state(&self) -> &InterferenceState {
        &self.record
    }
}

impl Deref for TopOpeBRepDSShapeShapeInterference {
    type Target = ShapeInterferenceState;

    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

impl Deref for dyn ShapeInterferenceRecord {
    type Target = ShapeInterferenceState;

    fn deref(&self) -> &Self::Target {
        self.shape_interference_state()
    }
}

impl Deref for ShapeInterferenceState {
    type Target = InterferenceState;

    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

impl TopOpeBRepDSShapeShapeInterference {
    /// `TopOpeBRepDSShapeShapeInterference(T, ST, S, GT, G, GBound, C)`.
    pub fn new(
        t: &TopOpeBRepDSTransition,
        st: TopOpeBRepDSKind,
        s: i32,
        gt: TopOpeBRepDSKind,
        g: i32,
        gbound: bool,
        c: TopOpeBRepDSConfig,
    ) -> Self {
        Self {
            record: ShapeInterferenceState::new(t, st, s, gt, g, gbound, c),
        }
    }
}

impl ShapeInterferenceState {
    /// `TopOpeBRepDSShapeShapeInterference(T, ST, S, GT, G, GBound, C)`.
    pub(super) fn new(
        t: &TopOpeBRepDSTransition,
        st: TopOpeBRepDSKind,
        s: i32,
        gt: TopOpeBRepDSKind,
        g: i32,
        gbound: bool,
        c: TopOpeBRepDSConfig,
    ) -> Self {
        Self {
            record: InterferenceState::new(t, st, s, gt, g),
            my_gbound: SharedState::new(gbound),
            my_c: c,
        }
    }

    /// `Config()`; `i.config()`.
    pub fn config(&self) -> TopOpeBRepDSConfig {
        self.my_c
    }

    /// `GBound()`: whether the geometry is a bound of the support; `i.gbound()`.
    pub fn gbound(&self) -> bool {
        *self.my_gbound.read()
    }

    /// `SetGBound(b)`; `i.set_gbound(true)`.
    pub fn set_gbound(&self, b: bool) {
        *self.my_gbound.write() = b;
    }
}
