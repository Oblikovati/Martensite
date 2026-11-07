//! `GeomToIgesGeomCurve`: curves into IGES curves. Each kind of curve has its own member, this
//! file holds the choice among them and the offset and line curves.
use super::{GeomToIgesGeomEntity, bounded_range};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_geom::{IgesGeomLine, IgesGeomOffsetCurve};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomLine,
    GeomOffsetCurve, GeomParabola, GeomTrimmedCurve,
};
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::InterfaceStatic;
use std::ops::{Deref, DerefMut};

/// Translates curves.
///
/// ```
/// use martensite_tkdeiges::geom_to_iges::GeomToIgesGeomCurve;
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkg3d::geom::GeomCurve;
/// let tool = GeomToIgesGeomCurve::new();
/// let none: Handle<dyn GeomCurve> = Handle::default();
/// assert!(tool.transfer_curve(&none, 0.0, 1.0).unwrap().is_null());
/// ```
#[derive(Clone, Default)]
pub struct GeomToIgesGeomCurve {
    base: GeomToIgesGeomEntity,
}

impl Deref for GeomToIgesGeomCurve {
    type Target = GeomToIgesGeomEntity;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for GeomToIgesGeomCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// What a transfer of a curve gives: an entity, null when the curve has no translation.
pub(super) type EntityResult = Result<Handle<IgesDataIgesEntity>, Failure>;

impl GeomToIgesGeomCurve {
    /// A tool at the initial state; `GeomToIgesGeomCurve::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A tool with the model and the unit of `base`; `GeomToIgesGeomCurve::with_entity(&base)`.
    pub fn with_entity(base: &GeomToIgesGeomEntity) -> Self {
        Self { base: base.clone() }
    }
    /// The entity of any curve between two parameters; null for a null curve or an unknown kind;
    /// `tool.transfer_curve(&curve, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the evaluation or the conversion of the curve.
    pub fn transfer_curve(
        &self,
        start: &Handle<dyn GeomCurve>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        if let Some(entity) = self.transfer_bounded(start, first, last)? {
            return Ok(entity);
        }
        if let Some(entity) = self.transfer_conic(start, first, last)? {
            return Ok(entity);
        }
        let offset = start.cast::<GeomOffsetCurve>();
        if offset.is_some() {
            return self.transfer_offset_curve(&offset, first, last);
        }
        self.transfer_line(&start.cast::<GeomLine>(), first, last)
    }
    fn transfer_bounded(
        &self,
        start: &Handle<dyn GeomCurve>,
        first: f64,
        last: f64,
    ) -> Result<Option<Handle<IgesDataIgesEntity>>, Failure> {
        let bspline = start.cast::<GeomBSplineCurve>();
        if bspline.is_some() {
            return self.transfer_bspline_curve(&bspline, first, last).map(Some);
        }
        let bezier = start.cast::<GeomBezierCurve>();
        if bezier.is_some() {
            return self.transfer_bezier_curve(&bezier, first, last).map(Some);
        }
        let trimmed = start.cast::<GeomTrimmedCurve>();
        if trimmed.is_some() {
            return self.transfer_trimmed_curve(&trimmed, first, last).map(Some);
        }
        Ok(None)
    }
    fn transfer_conic(
        &self,
        start: &Handle<dyn GeomCurve>,
        first: f64,
        last: f64,
    ) -> Result<Option<Handle<IgesDataIgesEntity>>, Failure> {
        let circle = start.cast::<GeomCircle>();
        if circle.is_some() {
            return self.transfer_circle(&circle, first, last).map(Some);
        }
        let ellipse = start.cast::<GeomEllipse>();
        if ellipse.is_some() {
            return self.transfer_ellipse(&ellipse, first, last).map(Some);
        }
        let hyperbola = start.cast::<GeomHyperbola>();
        if hyperbola.is_some() {
            return self.transfer_hyperbola(&hyperbola, first, last).map(Some);
        }
        let parabola = start.cast::<GeomParabola>();
        if parabola.is_some() {
            return self.transfer_parabola(&parabola, first, last).map(Some);
        }
        Ok(None)
    }
    /// The entity of the basis of a trimmed curve, between the given parameters and not its trims;
    /// `tool.transfer_trimmed_curve(&curve, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the transfer of the basis.
    pub fn transfer_trimmed_curve(
        &self,
        start: &Handle<GeomTrimmedCurve>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        match start.get() {
            None => Ok(Handle::default()),
            Some(live) => self.transfer_curve(&live.basis_curve(), first, last),
        }
    }
    /// The line segment (type 110) between two parameters; `tool.transfer_line(&line, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the line.
    pub fn transfer_line(&self, start: &Handle<GeomLine>, first: f64, last: f64) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let (lower, upper) = bounded_range(first, last);
        let line = IgesGeomLine::new();
        line.init(
            self.in_unit(live.value(lower)?.xyz()),
            self.in_unit(live.value(upper)?.xyz()),
        );
        Ok(Handle::new(line).cast())
    }
    /// The offset curve (type 130) when the offset mode asks for it and the basis is planar,
    /// else its B-spline; `tool.transfer_offset_curve(&curve, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the transfer of the basis, or of the conversion in the offset mode 0.
    pub fn transfer_offset_curve(
        &self,
        start: &Handle<GeomOffsetCurve>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let (lower, upper) = bounded_range(first, last);
        if InterfaceStatic::i_val("write.iges.offset.mode") == 0 {
            let converted = geom_convert::curve_to_bspline_curve(
                &start.cast(),
                ParameterisationType::TgtThetaOver2,
            )?;
            return self.transfer_bspline_curve(&converted, lower, upper);
        }
        let basis = live.basis_curve();
        if !super::curve_bspline::planar_normal(&basis)?.0
            && let Some(entity) = self.non_planar_offset(start, lower, upper)
        {
            return Ok(entity);
        }
        self.offset_entity(live, &basis, (lower, upper))
    }
    /// A non-planar offset has to be its B-spline; a failed conversion falls back on the entity.
    fn non_planar_offset(
        &self,
        start: &Handle<GeomOffsetCurve>,
        lower: f64,
        upper: f64,
    ) -> Option<Handle<IgesDataIgesEntity>> {
        let converted = geom_convert::curve_to_bspline_curve(
            &start.cast(),
            ParameterisationType::TgtThetaOver2,
        )
        .ok()?;
        self.transfer_bspline_curve(&converted, lower, upper).ok()
    }
    fn offset_entity(
        &self,
        live: &GeomOffsetCurve,
        basis: &Handle<dyn GeomCurve>,
        (lower, upper): (f64, f64),
    ) -> EntityResult {
        let base_curve = self.transfer_curve(basis, basis_first(basis)?, basis_last(basis)?)?;
        let distance = live.offset() / self.get_unit();
        let normal: Xyz = live.direction().xyz().reversed();
        let entity = IgesGeomOffsetCurve::new();
        entity.init(
            &base_curve,
            1,
            &Handle::default(),
            0,
            0,
            distance,
            0.0,
            distance,
            0.0,
            normal,
            lower,
            upper,
        );
        Ok(Handle::new(entity).cast())
    }
}

fn basis_first(basis: &Handle<dyn GeomCurve>) -> Result<f64, Failure> {
    crate::iges_data::iges_live(basis, "GeomToIgesGeomCurve")?.try_first_parameter()
}

fn basis_last(basis: &Handle<dyn GeomCurve>) -> Result<f64, Failure> {
    crate::iges_data::iges_live(basis, "GeomToIgesGeomCurve")?.try_last_parameter()
}
