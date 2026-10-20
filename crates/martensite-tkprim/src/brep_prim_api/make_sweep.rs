//! `BRepPrimAPI_MakeSweep`: the root of the sweep makers.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// `BRepPrimAPI_MakeSweep`: a maker of a sweep, which
/// gives the bottom and the top of the sweep (the two pure virtual members).
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkmath::gp::{Point3, Vector3};
/// use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakePrism, BRepPrimAPIMakeSweep};
/// use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
/// let vertex: Shape = BRepBuilderAPIMakeVertex::new(Point3::new(0.0, 0.0, 0.0))?.vertex()?.into();
/// let mut maker = BRepPrimAPIMakePrism::new(&vertex, Vector3::new(0.0, 0.0, 1.0), false, true)?;
/// assert!(!maker.last_shape()?.is_null());
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub trait BRepPrimAPIMakeSweep: BRepBuilderAPIMakeShape {
    /// `FirstShape()`: the bottom of the sweep; `maker.first_shape()?`.
    fn first_shape(&mut self) -> Result<Shape, Failure>;

    /// `LastShape()`: the top of the sweep; `maker.last_shape()?`.
    fn last_shape(&mut self) -> Result<Shape, Failure>;
}
