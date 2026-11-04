//! Direction aliases bind the actual axis or rectangular-ray state, not a getter snapshot.
use super::rectangular_frustum::RectangularRay;
use martensite_tkernel::standard::SharedState;
use martensite_tkmath::gp::{Axis1, Direction3};

static BASE_DIRECTION: Direction3 = Direction3::X;

#[derive(Clone, Copy)]
enum DirectionField<'a> {
    Static(&'static Direction3),
    Axis(&'a SharedState<Axis1>),
    Rectangular(&'a SharedState<RectangularRay>),
}

/// Borrow one live direction field; `let alias=volume.get_view_ray_direction_ref()?; alias.value()`.
/// Reading briefly locks the existing state; the view itself holds no lock or owner.
#[derive(Clone, Copy)]
pub struct SelectMgrDirectionRef<'a> {
    field: DirectionField<'a>,
}

impl<'a> SelectMgrDirectionRef<'a> {
    pub(super) fn base_direction() -> Self {
        Self {
            field: DirectionField::Static(&BASE_DIRECTION),
        }
    }
    pub(super) fn axis(axis: &'a SharedState<Axis1>) -> Self {
        Self {
            field: DirectionField::Axis(axis),
        }
    }
    pub(super) fn rectangular(ray: &'a SharedState<RectangularRay>) -> Self {
        Self {
            field: DirectionField::Rectangular(ray),
        }
    }
    /// Read the same field after later initialization or build; `alias.value().z()`.
    pub fn value(&self) -> Direction3 {
        match self.field {
            DirectionField::Static(direction) => *direction,
            DirectionField::Axis(axis) => axis.read().direction(),
            DirectionField::Rectangular(ray) => ray.read().direction,
        }
    }
    /// Compare the captured source and direction leaf; `left.is_same_reference(&right)`.
    pub fn is_same_reference(&self, other: &SelectMgrDirectionRef<'_>) -> bool {
        match (self.field, other.field) {
            (DirectionField::Static(left), DirectionField::Static(right)) => {
                std::ptr::eq(left, right)
            }
            (DirectionField::Axis(left), DirectionField::Axis(right)) => std::ptr::eq(left, right),
            (DirectionField::Rectangular(left), DirectionField::Rectangular(right)) => {
                std::ptr::eq(left, right)
            }
            _ => false,
        }
    }
}
