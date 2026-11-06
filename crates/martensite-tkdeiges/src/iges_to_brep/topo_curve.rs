//! `IgesToBRepTopoCurve`: curve entities as edges and wires. This file holds the state, the choice
//! among the kinds and points; the kinds with more to them have their own files.
use super::{IgesToBRep, IgesToBRepCurveAndSurface};
use crate::iges_data::{IgesDataIgesEntity, IgesDataToolLocation};
use crate::iges_geom::{
    IgesGeomBoundary, IgesGeomCompositeCurve, IgesGeomCurveOnSurface, IgesGeomOffsetCurve,
    IgesGeomPoint,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Face, Shape, Vertex};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkmath::gp::{Point3, Transform2, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use std::ops::{Deref, DerefMut};

/// Translates curve entities.
///
/// ```
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepTopoCurve;
/// let tool = IgesToBRepTopoCurve::new();
/// assert_eq!(tool.nb_curves(), 0);
/// assert!(!tool.bad_case());
/// ```
#[derive(Clone, Default)]
pub struct IgesToBRepTopoCurve {
    base: IgesToBRepCurveAndSurface,
    pub(super) curves: Vec<Handle<dyn GeomCurve>>,
    pub(super) curves_2d: Vec<Handle<dyn Geom2dCurve>>,
    pub(super) bad_case: bool,
}

impl Deref for IgesToBRepTopoCurve {
    type Target = IgesToBRepCurveAndSurface;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for IgesToBRepTopoCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IgesToBRepTopoCurve {
    /// A tool at the default settings; `IgesToBRepTopoCurve::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the settings of `base`; `IgesToBRepTopoCurve::with_base(&base)`.
    pub fn with_base(base: &IgesToBRepCurveAndSurface) -> Self {
        Self {
            base: base.clone(),
            ..Self::default()
        }
    }
    /// A tool with the given settings;
    /// `IgesToBRepTopoCurve::with_settings(1e-4, 1e-6, 1e-4, true, false, false)`.
    pub fn with_settings(
        eps: f64,
        eps_coeff: f64,
        eps_geom: f64,
        mode: bool,
        mode_approx: bool,
        optimized: bool,
    ) -> Self {
        Self::with_base(&IgesToBRepCurveAndSurface::with_settings(
            eps,
            eps_coeff,
            eps_geom,
            mode,
            mode_approx,
            optimized,
        ))
    }
    /// The fail of a null entity.
    pub(super) fn null_entity<T: ?Sized + RuntimeTyped>(&self, start: &Handle<T>) {
        self.fail_key(start, "IGES_1005");
    }
    /// The compound location of an entity as a move of a shape, a warning when it is not a similarity.
    /// With `to_unit` the translation goes into the file unit.
    pub(super) fn moved_to_location(
        &mut self,
        start: &IgesDataIgesEntity,
        handle: &Handle<IgesDataIgesEntity>,
        shape: &mut Shape,
        to_unit: bool,
    ) -> Result<(), Failure> {
        let mut similarity = Transform3::default();
        self.set_epsilon(1.0e-4);
        let unit = if to_unit { self.get_unit_factor() } else { 1.0 };
        let location = start.compound_location()?;
        if IgesDataToolLocation::convert_location(
            self.get_epsilon(),
            &location,
            &mut similarity,
            unit,
        )? {
            shape.move_by(&Location::new(similarity), false)?;
        } else {
            self.warn_key(handle, "IGES_1035");
        }
        Ok(())
    }
    /// The vertex of a point in the file unit, moved by its location in topology mode;
    /// `tool.transfer_point(&point)?`.
    ///
    /// # Errors
    /// A failure of the entity.
    pub fn transfer_point(&mut self, start: &Handle<IgesGeomPoint>) -> Result<Vertex, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Vertex::default());
        };
        let point = if !self.get_mode_transfer() && live.has_transf() {
            live.transformed_value()?
        } else {
            live.value()
        };
        let scaled = point.scaled(Point3::new(0.0, 0.0, 0.0), self.get_unit_factor());
        let mut vertex = Vertex::default();
        BRepBuilder::default().make_vertex_at(&mut vertex, scaled, precision::confusion())?;
        // The vertex follows the position of its point.
        if self.get_mode_transfer() && live.has_transf() {
            let mut similarity = Transform3::default();
            self.set_epsilon(1.0e-4);
            let location = live.compound_location()?;
            let unit = self.get_unit_factor();
            if IgesDataToolLocation::convert_location(
                self.get_epsilon(),
                &location,
                &mut similarity,
                unit,
            )? {
                vertex.move_by(&Location::new(similarity), false)?;
            }
        }
        Ok(vertex)
    }
    /// The vertex of a point with Z at 0, not scaled; `tool.transfer_2d_point(&point)?`.
    ///
    /// # Errors
    /// A failure of the entity.
    pub fn transfer_2d_point(&self, start: &Handle<IgesGeomPoint>) -> Result<Vertex, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Vertex::default());
        };
        let value = if !self.get_mode_transfer() && live.has_transf() {
            live.transformed_value()?
        } else {
            live.value()
        };
        let mut vertex = Vertex::default();
        let flat = Point3::new(value.x(), value.y(), 0.0);
        BRepBuilder::default().make_vertex_at(&mut vertex, flat, precision::confusion())?;
        Ok(vertex)
    }
    /// The shape of any curve entity; `tool.transfer_topo_curve(&entity)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_topo_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Shape, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Shape::default());
        }
        if IgesToBRep::is_basic_curve(start) {
            return self.transfer_topo_basic_curve(start);
        }
        let composite = start.cast::<IgesGeomCompositeCurve>();
        if composite.is_some() {
            return self.transfer_composite_curve(&composite);
        }
        let on_surface = start.cast::<IgesGeomCurveOnSurface>();
        if on_surface.is_some() {
            return self.transfer_curve_on_surface(&on_surface);
        }
        let boundary = start.cast::<IgesGeomBoundary>();
        if boundary.is_some() {
            return self.transfer_boundary(&boundary);
        }
        let point = start.cast::<IgesGeomPoint>();
        if point.is_some() {
            return Ok(self.transfer_point(&point)?.into_shape());
        }
        let offset = start.cast::<IgesGeomOffsetCurve>();
        if offset.is_some() {
            return self.transfer_offset_curve(&offset);
        }
        Ok(Shape::default())
    }
    /// The shape of a curve entity in the parameters of a face;
    /// `tool.transfer_2d_topo_curve(&entity, &face, trans, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the shape.
    pub fn transfer_2d_topo_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
    ) -> Result<Shape, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Shape::default());
        }
        if IgesToBRep::is_basic_curve(start) {
            return self.transfer_2d_topo_basic_curve(start, face, trans, u_fact);
        }
        let composite = start.cast::<IgesGeomCompositeCurve>();
        if composite.is_some() {
            return self.transfer_2d_composite_curve(&composite, face, trans, u_fact);
        }
        let point = start.cast::<IgesGeomPoint>();
        if point.is_some() {
            return Ok(self.transfer_2d_point(&point)?.into_shape());
        }
        let offset = start.cast::<IgesGeomOffsetCurve>();
        if offset.is_some() {
            return self.transfer_2d_offset_curve(&offset, face, trans, u_fact);
        }
        Ok(Shape::default())
    }
    /// The pieces of a degree 1 B-spline curve, in the list of curves;
    /// `tool.approx_bspline_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the approximation.
    pub fn approx_bspline_curve(
        &mut self,
        start: &Handle<GeomBSplineCurve>,
    ) -> Result<(), Failure> {
        let pieces =
            crate::iges_data::iges_live(&ShapeAlgo::algo_container(), "IgesToBRepTopoCurve")?
                .approx_bspline_curve(start)?;
        self.curves.extend(pieces.iter().cloned());
        Ok(())
    }
    /// The count of curves; `tool.nb_curves()`.
    pub fn nb_curves(&self) -> i32 {
        self.curves.len() as i32
    }
    /// A curve by its rank from 1, null out of range; `tool.curve(1)`.
    pub fn curve(&self, num: i32) -> Handle<dyn GeomCurve> {
        pick_rank(&self.curves, num)
    }
    /// The pieces of a degree 1 B-spline curve in the plane, in the list of curves in the plane;
    /// `tool.approx_2d_bspline_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the approximation.
    pub fn approx_2d_bspline_curve(
        &mut self,
        start: &Handle<Geom2dBSplineCurve>,
    ) -> Result<(), Failure> {
        let pieces =
            crate::iges_data::iges_live(&ShapeAlgo::algo_container(), "IgesToBRepTopoCurve")?
                .approx_bspline_curve_2d(start)?;
        self.curves_2d.extend(pieces.iter().cloned());
        Ok(())
    }
    /// The count of curves in the plane; `tool.nb_curves_2d()`.
    pub fn nb_curves_2d(&self) -> i32 {
        self.curves_2d.len() as i32
    }
    /// A curve in the plane by its rank from 1, null out of range; `tool.curve_2d(1)`.
    pub fn curve_2d(&self, num: i32) -> Handle<dyn Geom2dCurve> {
        pick_rank(&self.curves_2d, num)
    }
    /// Sets the mark of an arc whose ends coincide; `tool.set_bad_case(false)`.
    pub fn set_bad_case(&mut self, value: bool) {
        self.bad_case = value;
    }
    /// The mark of an arc whose ends coincide: its 2D form says more; `tool.bad_case()`.
    pub fn bad_case(&self) -> bool {
        self.bad_case
    }
}

fn pick_rank<T: ?Sized + RuntimeTyped>(list: &[Handle<T>], num: i32) -> Handle<T> {
    usize::try_from(num - 1)
        .ok()
        .and_then(|index| list.get(index))
        .cloned()
        .unwrap_or_default()
}
