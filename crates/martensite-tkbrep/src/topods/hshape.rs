//! `TopoDS_HShape`: a shape shared by handle.
use super::Shape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{SharedState, StandardTransient};

/// A transient holding one shape, so that several owners share and modify it. The
/// shape is behind the object lock: `Shape()` returns a
/// copy and `ChangeShape()` lends the stored shape to a closure.
///
/// ```
/// use martensite_tkbrep::topods::{HShape, Shape};
/// use martensite_tkg3d::top_abs::Orientation;
/// let holder = HShape::new(&Shape::default());
/// holder.change_shape(|shape| shape.set_orientation(Orientation::Forward));
/// assert_eq!(holder.shape().orientation(), Orientation::Forward);
/// ```
#[derive(Default)]
pub struct HShape {
    transient: StandardTransient,
    shape: SharedState<Shape>,
}
impl_transient!(HShape, "TopoDS_HShape", StandardTransient, transient);

impl HShape {
    /// `TopoDS_HShape(aShape)`; `HShape::new(&shape)`.
    pub fn new(shape: &Shape) -> Self {
        Self {
            transient: StandardTransient::default(),
            shape: SharedState::new(shape.clone()),
        }
    }

    /// `Shape(aShape)`: replaces the held shape; `holder.set_shape(&shape)`.
    pub fn set_shape(&self, shape: &Shape) {
        *self.shape.write() = shape.clone();
    }

    /// `Shape()`: a copy of the held shape; `holder.shape()`.
    pub fn shape(&self) -> Shape {
        self.shape.read().clone()
    }

    /// `ChangeShape()`: lends the held shape for modification;
    /// `holder.change_shape(|shape| shape.reverse())`.
    pub fn change_shape<R>(&self, change: impl FnOnce(&mut Shape) -> R) -> R {
        change(&mut self.shape.write())
    }
}
