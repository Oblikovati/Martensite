//! Captured point fields keep their source when the selecting volume changes state.
use super::rectangular_frustum::RectangularRay;
use martensite_tkernel::standard::SharedState;
use martensite_tkmath::gp::{Axis1, Point3};

static BASE_NEAR: Point3 = Point3::new(0., 0., 0.);
static BASE_FAR: Point3 = Point3::new(f64::MAX, f64::MAX, f64::MAX);
static AXIS_FAR: Point3 = Point3::new(f64::MAX, f64::MAX, f64::MAX);

#[derive(Clone, Copy)]
enum PointField<'a> {
    Static(&'static Point3),
    AxisLocation(&'a SharedState<Axis1>),
    RectangularNear(&'a SharedState<RectangularRay>),
    RectangularFar(&'a SharedState<RectangularRay>),
}

/// Borrow a fixed live point field; `let alias=volume.get_near_pnt_ref()?; alias.value()`.
/// The borrow retains no owner and holds no lock between observations.
#[derive(Clone, Copy)]
pub struct SelectMgrPointRef<'a> {
    field: PointField<'a>,
}

impl<'a> SelectMgrPointRef<'a> {
    pub(super) fn base_near() -> Self {
        Self {
            field: PointField::Static(&BASE_NEAR),
        }
    }
    pub(super) fn base_far() -> Self {
        Self {
            field: PointField::Static(&BASE_FAR),
        }
    }
    pub(super) fn axis_far() -> Self {
        Self {
            field: PointField::Static(&AXIS_FAR),
        }
    }
    pub(super) fn axis_location(axis: &'a SharedState<Axis1>) -> Self {
        Self {
            field: PointField::AxisLocation(axis),
        }
    }
    pub(super) fn rectangular_near(ray: &'a SharedState<RectangularRay>) -> Self {
        Self {
            field: PointField::RectangularNear(ray),
        }
    }
    pub(super) fn rectangular_far(ray: &'a SharedState<RectangularRay>) -> Self {
        Self {
            field: PointField::RectangularFar(ray),
        }
    }
    /// Read the currently bound point without retaining a lock; `alias.value().x()`.
    pub fn value(&self) -> Point3 {
        match self.field {
            PointField::Static(point) => *point,
            PointField::AxisLocation(axis) => axis.read().location(),
            PointField::RectangularNear(ray) => ray.read().near,
            PointField::RectangularFar(ray) => ray.read().far,
        }
    }
    /// Compare the fixed storage source and leaf; `left.is_same_reference(&right)`.
    pub fn is_same_reference(&self, other: &SelectMgrPointRef<'_>) -> bool {
        match (self.field, other.field) {
            (PointField::Static(left), PointField::Static(right)) => std::ptr::eq(left, right),
            (PointField::AxisLocation(left), PointField::AxisLocation(right)) => {
                std::ptr::eq(left, right)
            }
            (PointField::RectangularNear(left), PointField::RectangularNear(right))
            | (PointField::RectangularFar(left), PointField::RectangularFar(right)) => {
                std::ptr::eq(left, right)
            }
            _ => false,
        }
    }
}
