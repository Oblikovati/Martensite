//! A rendering operation records picks through its explicitly owned sink.
use martensite_tkbrep::topods::Shape;
use std::cell::RefCell;
/// A selected entity and parameters in a thread-confined rendering context; `let pick = ShapePick::default();`.
#[derive(Debug, Clone, Default)]
pub struct ShapePick {
    /// The selected topological entity.
    pub shape: Shape,
    /// The first geometry parameter.
    pub u: f64,
    /// The second geometry parameter.
    pub v: f64,
}
/// The destination for geometry picks; source compatibility can supply an adapter.
pub trait ShapePickSink {
    /// Records the exact selected shape and parameters; `sink.record_pick(&shape, u, v);`.
    fn record_pick(&self, shape: &Shape, u: f64, v: f64);
}
/// A caller-owned, thread-confined pick result; `let picks = LocalShapePicks::default();`.
#[derive(Debug, Default)]
pub struct LocalShapePicks(RefCell<ShapePick>);
impl LocalShapePicks {
    /// Reads the latest pick; `let selected = picks.last_pick();`.
    pub fn last_pick(&self) -> ShapePick {
        self.0.borrow().clone()
    }
}
impl ShapePickSink for LocalShapePicks {
    fn record_pick(&self, shape: &Shape, u: f64, v: f64) {
        *self.0.borrow_mut() = ShapePick {
            shape: shape.clone(),
            u,
            v,
        };
    }
}
