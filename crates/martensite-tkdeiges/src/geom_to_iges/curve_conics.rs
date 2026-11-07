//! Conics into the circular arc (type 100) and the conic arc (type 104), each with its position.
use super::GeomToIgesGeomCurve;
use super::bounded_range;
use super::curve::EntityResult;
use crate::iges_conv_geom::IgesConvGeomGeomBuilder;
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::{IgesGeomCircularArc, IgesGeomConicArc};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomCircle, GeomConic, GeomCurve, GeomEllipse, GeomHyperbola, GeomParabola,
};
use martensite_tkgeombase::geom_convert::{self, ApproxCurve};
use martensite_tkmath::bsplclib;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{self, Axis22d, Ellipse2, Hyperbola2, Parabola2, Xy, Xyz};
use std::f64::consts::PI;

const WHERE: &str = "GeomToIgesGeomCurve";

/// The arc end points of a conic in its position, in the unit of the model.
struct ConicArcEnds {
    start: Xy,
    end: Xy,
}

/// The builder standing at the position of a conic.
fn conic_builder(conic: &dyn GeomConic) -> IgesConvGeomGeomBuilder {
    let mut builder = IgesConvGeomGeomBuilder::new();
    builder.set_position_axis2(conic.position());
    builder
}

/// A parameter that is zero up to the resolution of the geometry is zero.
fn snapped_parameter(parameter: f64) -> f64 {
    if parameter.abs() <= gp::resolution() {
        0.0
    } else {
        parameter
    }
}

/// The plane system the implicit equations are written in.
fn standard_plane_frame() -> Axis22d {
    Axis22d::default()
}

impl GeomToIgesGeomCurve {
    fn local_xy(&self, builder: &IgesConvGeomGeomBuilder, point: Xyz) -> Result<Xy, Failure> {
        let local = self.in_unit(builder.eval_xyz(point)?);
        Ok(Xy::new(local.x(), local.y()))
    }
    fn arc_ends(
        &self,
        builder: &IgesConvGeomGeomBuilder,
        curve: &dyn GeomCurve,
        lower: f64,
        upper: f64,
    ) -> Result<ConicArcEnds, Failure> {
        Ok(ConicArcEnds {
            start: self.local_xy(builder, curve.value(lower)?.xyz())?,
            end: self.local_xy(builder, curve.value(upper)?.xyz())?,
        })
    }
    /// Gives an arc the transformation of its position, unless that is the identity.
    fn positioned(
        &self,
        entity: Handle<IgesDataIgesEntity>,
        builder: &IgesConvGeomGeomBuilder,
    ) -> EntityResult {
        if !builder.is_identity() {
            let matrix = builder.make_transformation(self.get_unit())?;
            iges_live(&entity, WHERE)?.init_transf(&matrix.cast());
        }
        Ok(entity)
    }
    /// The circular arc between two parameters; `tool.transfer_circle(&circle, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the circle.
    pub fn transfer_circle(
        &self,
        start: &Handle<GeomCircle>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let builder = conic_builder(live);
        let begin = live.value(snapped_parameter(first))?;
        // A whole circle is written with equal ends, or it reads back as a B-spline.
        let is_whole = (last - first - 2.0 * PI).abs() <= precision::p_confusion();
        let end = if is_whole { begin } else { live.value(last)? };
        let center = self.in_unit(builder.eval_xyz(live.location().xyz())?);
        let entity = IgesGeomCircularArc::new();
        entity.init(
            center.z(),
            Xy::new(center.x(), center.y()),
            self.local_xy(&builder, begin.xyz())?,
            self.local_xy(&builder, end.xyz())?,
        );
        self.positioned(Handle::new(entity).cast(), &builder)
    }
    /// The conic arc between two parameters, a B-spline for a whole ellipse, which the entity
    /// would orient wrongly; `tool.transfer_ellipse(&ellipse, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the evaluation or of the conversion of the ellipse.
    pub fn transfer_ellipse(
        &self,
        start: &Handle<GeomEllipse>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        if (last - first - 2.0 * PI).abs() <= precision::p_confusion() {
            return self.transfer_whole_ellipse(live, first, last);
        }
        let unit = self.get_unit();
        let plane = Ellipse2::new(
            standard_plane_frame(),
            live.major_radius() / unit,
            live.minor_radius() / unit,
        )?;
        // The plane conic answers half of the mixed and linear terms.
        let [a, c, b, d, e, f] = plane.coefficients();
        let equation = [a, 2.0 * b, c, 2.0 * d, 2.0 * e, f];
        self.conic_arc(live, live, (snapped_parameter(first), last), equation)
    }
    /// An ellipse turned to start at `first` and converted, which keeps the weights small.
    fn transfer_whole_ellipse(&self, live: &GeomEllipse, first: f64, last: f64) -> EntityResult {
        let frame = live.position();
        let turned = GeomEllipse::new(
            frame.rotated(frame.axis(), first)?,
            live.major_radius(),
            live.minor_radius(),
        )?;
        let curve: Handle<dyn GeomCurve> = Handle::new(turned).cast();
        let bspline = Self::approximated_or_converted(&curve)?;
        let body = iges_live(&bspline, WHERE)?;
        let mut knots = body.knots();
        bsplclib::reparametrize(first, first + 2.0 * PI, &mut knots)?;
        body.set_knots(&knots)?;
        self.transfer_bspline_curve(&bspline, first, last)
    }
    fn approximated_or_converted(
        curve: &Handle<dyn GeomCurve>,
    ) -> Result<Handle<GeomBSplineCurve>, Failure> {
        let approximation =
            ApproxCurve::from_curve(curve, precision::approximation(), Shape::C1, 100, 6)?;
        if approximation.has_result() {
            let result = approximation.curve();
            if result.is_some() {
                return Ok(result);
            }
        }
        geom_convert::curve_to_bspline_curve(curve, ParameterisationType::QuasiAngular)
    }
    fn conic_arc(
        &self,
        curve: &dyn GeomCurve,
        conic: &dyn GeomConic,
        (lower, upper): (f64, f64),
        [a, b, c, d, e, f]: [f64; 6],
    ) -> EntityResult {
        let builder = conic_builder(conic);
        let ends = self.arc_ends(&builder, curve, lower, upper)?;
        let entity = IgesGeomConicArc::new();
        entity.init(a, b, c, d, e, f, 0.0, ends.start, ends.end);
        self.positioned(Handle::new(entity).cast(), &builder)
    }
    /// The conic arc of a hyperbola; `tool.transfer_hyperbola(&hyperbola, -1.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the hyperbola.
    pub fn transfer_hyperbola(
        &self,
        start: &Handle<GeomHyperbola>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let unit = self.get_unit();
        let plane = Hyperbola2::new(
            standard_plane_frame(),
            live.major_radius() / unit,
            live.minor_radius() / unit,
        )?;
        let [a, c, b, d, e, f] = plane.coefficients();
        self.conic_arc(live, live, bounded_range(first, last), [a, b, c, d, e, f])
    }
    /// The conic arc of a parabola; `tool.transfer_parabola(&parabola, -1.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the evaluation of the parabola.
    pub fn transfer_parabola(
        &self,
        start: &Handle<GeomParabola>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        let Some(live) = start.get() else {
            return Ok(Handle::default());
        };
        let plane = Parabola2::new(standard_plane_frame(), live.focal() * 2.0)?;
        let [a, c, b, d, e, f] = plane.coefficients();
        self.conic_arc(live, live, bounded_range(first, last), [a, b, c, d, e, f])
    }
}
