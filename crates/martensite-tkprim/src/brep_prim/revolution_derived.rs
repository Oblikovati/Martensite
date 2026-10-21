//! The classes derived from `BRepPrim_Revolution`, which only override
//! `MakeEmptyLateralFace`.
use super::{BRepPrimOneAxis, BRepPrimOneAxisBase, BRepPrimRevolution};
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::gp::Point2;

/// A class derived from `BRepPrim_Revolution`: the revolution it holds and its own
/// lateral face. Every implementor is a [`BRepPrimOneAxis`] whose other members are the
/// revolution's.
///
/// ```
/// use martensite_tkprim::brep_prim::{BRepPrimCylinder, BRepPrimOneAxis, BRepPrimRevolutionDerived};
/// let mut cylinder = BRepPrimCylinder::from_radius_height(1.0, 2.0);
/// assert_eq!(cylinder.revolution().vmax(), 2.0);
/// assert!(!cylinder.lateral_face()?.is_null());
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub trait BRepPrimRevolutionDerived {
    /// The `BRepPrim_Revolution` base.
    fn revolution(&self) -> &BRepPrimRevolution;

    /// The `BRepPrim_Revolution` base, to write.
    fn revolution_mut(&mut self) -> &mut BRepPrimRevolution;

    /// The override of `MakeEmptyLateralFace()`: a face on the elementary surface of
    /// the primitive, its normal outward; `cylinder.derived_lateral_face()?`.
    fn derived_lateral_face(&self) -> Result<Face, Failure>;
}

impl<T: BRepPrimRevolutionDerived> BRepPrimOneAxis for T {
    fn one_axis_base(&self) -> &BRepPrimOneAxisBase {
        self.revolution().one_axis_base()
    }

    fn one_axis_base_mut(&mut self) -> &mut BRepPrimOneAxisBase {
        self.revolution_mut().one_axis_base_mut()
    }

    fn make_empty_lateral_face(&self) -> Result<Face, Failure> {
        self.derived_lateral_face()
    }

    fn make_empty_meridian_edge(&self, angle: f64) -> Result<Edge, Failure> {
        self.revolution().make_empty_meridian_edge(angle)
    }

    fn set_meridian_pcurve(&self, edge: &Edge, face: &Face) -> Result<(), Failure> {
        self.revolution().set_meridian_pcurve(edge, face)
    }

    fn meridian_value(&self, v: f64) -> Result<Point2, Failure> {
        self.revolution().meridian_value(v)
    }
}

/// The empty lateral face of a primitive: `MakeFace(F, S, Precision::Confusion())`
/// with the surface `surface` built by the primitive.
pub(crate) fn lateral_face_on(
    revolution: &BRepPrimRevolution,
    surface: Handle<dyn GeomSurface>,
) -> Result<Face, Failure> {
    let mut face = Face::default();
    revolution
        .one_axis_base()
        .my_builder
        .builder()
        .make_face_on_surface(&mut face, &surface, precision::confusion())?;
    Ok(face)
}
