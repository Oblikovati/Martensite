//! Screen state belongs to each thread-confined drawable, never a global context.
use std::cell::Cell;
#[derive(Debug, Default)]
pub(crate) struct DrawableState {
    pub bounds: Cell<[f64; 4]>,
    pub visible: Cell<bool>,
}
