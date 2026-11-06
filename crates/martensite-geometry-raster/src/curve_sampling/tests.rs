//! An unfinished uniform distribution must not evaluate a missing first point.
use super::{DrawDisplay, UniformDeflection, plot_deflection};
use std::cell::Cell;
#[test]
fn unfinished_distribution_is_a_successful_no_draw() {
    let calls = Cell::new(0);
    let mut display = DrawDisplay::batch();
    plot_deflection(
        &UniformDeflection::default(),
        &mut display,
        |_, _| calls.set(calls.get() + 1),
        |_, _| calls.set(calls.get() + 1),
    )
    .unwrap();
    assert_eq!(calls.get(), 0);
}
