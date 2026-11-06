//! Capture the mouse getter's selected field once, before later selection-mode changes.
use super::SelectMgrSelectionRectangle;
use martensite_tkernel::standard::SharedState;
use martensite_tkmath::gp::Point2;

static BASE_MOUSE: Point2 = Point2::new(f64::MAX, f64::MAX);

#[derive(Clone, Copy)]
enum MouseField<'a> {
    Static(&'static Point2),
    Rectangle(&'a SharedState<SelectMgrSelectionRectangle>),
}

/// Borrow the chosen mouse slot; `let alias=volume.get_mouse_position_ref(); alias.value()`.
/// A mode change never redirects an already captured slot to another field.
#[derive(Clone, Copy)]
pub struct SelectMgrMouseRef<'a> {
    field: MouseField<'a>,
}

impl<'a> SelectMgrMouseRef<'a> {
    pub(super) fn base_mouse() -> Self {
        Self {
            field: MouseField::Static(&BASE_MOUSE),
        }
    }
    pub(super) fn rectangle(rectangle: &'a SharedState<SelectMgrSelectionRectangle>) -> Self {
        Self {
            field: MouseField::Rectangle(rectangle),
        }
    }
    /// Read the current value from the original slot; `alias.value().x()`.
    pub fn value(&self) -> Point2 {
        match self.field {
            MouseField::Static(point) => *point,
            MouseField::Rectangle(rectangle) => rectangle.read().mouse_pos(),
        }
    }
    /// Compare the fixed static or rectangle field; `left.is_same_reference(&right)`.
    pub fn is_same_reference(&self, other: &SelectMgrMouseRef<'_>) -> bool {
        match (self.field, other.field) {
            (MouseField::Static(left), MouseField::Static(right)) => std::ptr::eq(left, right),
            (MouseField::Rectangle(left), MouseField::Rectangle(right)) => {
                std::ptr::eq(left, right)
            }
            _ => false,
        }
    }
}
