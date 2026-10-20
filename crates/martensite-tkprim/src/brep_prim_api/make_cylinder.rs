//! `BRepPrimAPI_MakeCylinder`: the maker of cylinders.
use super::make_one_axis::one_axis_maker;
use crate::brep_prim::{BRepPrimCylinder, BRepPrimOneAxis};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis2, xoy};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShapeBase;

/// `BRepPrimAPI_MakeCylinder`: a cylinder of radius R and height H on the Z axis of the
/// reference frame or of `Axes`, or the portion of it between the angles 0 and `Angle`.
///
/// ```
/// use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
/// use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
/// let mut maker = BRepPrimAPIMakeCylinder::with_angle(5.0, 10.0, std::f64::consts::PI)?;
/// assert!(!maker.shape()?.is_null());
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
#[derive(Clone)]
pub struct BRepPrimAPIMakeCylinder {
    base: BRepBuilderAPIMakeShapeBase,
    my_cylinder: BRepPrimCylinder,
}

impl BRepPrimAPIMakeCylinder {
    fn of(my_cylinder: BRepPrimCylinder) -> Self {
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            my_cylinder,
        }
    }

    /// `BRepPrimAPI_MakeCylinder(R, H)`; `BRepPrimAPIMakeCylinder::new(1.0, 2.0)`.
    pub fn new(r: f64, h: f64) -> Self {
        Self::of(BRepPrimCylinder::new(xoy(), r, h))
    }

    /// `BRepPrimAPI_MakeCylinder(R, H, Angle)`: the portion 0..`angle`;
    /// `BRepPrimAPIMakeCylinder::with_angle(1.0, 2.0, 3.0)?`.
    pub fn with_angle(r: f64, h: f64, angle: f64) -> Result<Self, Failure> {
        let mut cylinder = BRepPrimCylinder::from_radius_height(r, h);
        cylinder.set_angle(angle)?;
        Ok(Self::of(cylinder))
    }

    /// `BRepPrimAPI_MakeCylinder(Axes, R, H)`; `BRepPrimAPIMakeCylinder::with_axes(axes, 1.0, 2.0)`.
    pub fn with_axes(axes: Axis2, r: f64, h: f64) -> Self {
        Self::of(BRepPrimCylinder::new(axes, r, h))
    }

    /// `BRepPrimAPI_MakeCylinder(Axes, R, H, Angle)`;
    /// `BRepPrimAPIMakeCylinder::with_axes_angle(axes, 1.0, 2.0, 3.0)?`.
    pub fn with_axes_angle(axes: Axis2, r: f64, h: f64, angle: f64) -> Result<Self, Failure> {
        let mut cylinder = BRepPrimCylinder::new(axes, r, h);
        cylinder.set_angle(angle)?;
        Ok(Self::of(cylinder))
    }

    /// `Cylinder()`: the algorithm; `maker.cylinder()`.
    pub fn cylinder(&mut self) -> &mut BRepPrimCylinder {
        &mut self.my_cylinder
    }
}

one_axis_maker!(BRepPrimAPIMakeCylinder, my_cylinder);
