//! Spline curves (type 112) and rational B-spline curves (type 126).
use super::IgesToBRepBasicCurve;
use super::knot_gathering::{
    GatheredKnots, KnotCompare, correct_multiplicities, gather_knots, without_ranks,
};
use crate::iges_conv_geom;
use crate::iges_geom::{IgesGeomBSplineCurve, IgesGeomSplineCurve};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::real::epsilon;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomTrimmedCurve};
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkshhealing::shape_construct::ShapeConstructCurve;

const MESSAGE_WHERE: &str = "IgesToBRepBasicCurve";

/// The fix of the knots: a warning for weights too far apart, a fail for knots out of order.
fn checked_knots(
    tool: &IgesToBRepBasicCurve,
    start: &Handle<IgesGeomBSplineCurve>,
    polynomial: bool,
    knots: &mut Vec<f64>,
    weights: &[f64],
) -> Result<bool, Failure> {
    if !polynomial {
        let min = weights.iter().copied().fold(f64::INFINITY, f64::min);
        let max = weights.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        if max - min > 1000.0 {
            tool.warn_key(start, "IGES_1374");
        }
    }
    let descending = knots.windows(2).filter(|pair| pair[1] < pair[0]).count();
    for _ in 0..descending {
        tool.fail_key(start, "IGES_1373");
    }
    if descending > 0 {
        return Ok(false);
    }
    let mut fixed = owned_array(knots)?;
    ShapeConstructCurve::fix_knots(&mut fixed)?;
    *knots = fixed.as_slice().to_vec();
    Ok(true)
}

impl IgesToBRepBasicCurve {
    fn entity_poles(&self, live: &IgesGeomBSplineCurve) -> Result<Vec<Point3>, Failure> {
        let positioned = self.applies_position(live);
        (0..=live.upper_index())
            .map(|rank| {
                if positioned {
                    live.transformed_pole(rank)
                } else {
                    live.pole(rank)
                }
            })
            .collect()
    }
    /// The weights of the poles, `None` after a fail for a weight that is not positive.
    fn entity_weights(
        &self,
        start: &Handle<IgesGeomBSplineCurve>,
        live: &IgesGeomBSplineCurve,
    ) -> Result<Option<Vec<f64>>, Failure> {
        let reference = live.weight(0)?;
        let mut constant = true;
        let mut weights = Vec::new();
        for rank in 0..=live.upper_index() {
            let weight = live.weight(rank)?;
            constant = constant && (weight - reference).abs() <= epsilon(reference);
            // A weight that is not positive is refused.
            if weight < precision::p_confusion() {
                self.fail_key(start, "IGES_1215");
                return Ok(None);
            }
            weights.push(weight);
        }
        if constant {
            self.warn_with_args(start, "IGES_1220", &[b"curve"])?;
        }
        Ok(Some(weights))
    }
    /// The B-spline of the entity, cut to the parameters of the entity when they lie in it, or
    /// trimmed if it refuses the cut; `tool.transfer_bspline_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the entity.
    pub fn transfer_bspline_curve(
        &self,
        start: &Handle<IgesGeomBSplineCurve>,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        let degree = live.degree();
        if degree <= 0 || degree > GeomBSplineCurve::max_degree() {
            self.fail_key(start, "IGES_1190");
            return Ok(Handle::default());
        }
        let nb_poles = live.nb_poles();
        if nb_poles < 2 {
            self.fail_key(start, "IGES_1195");
            return Ok(Handle::default());
        }
        let read_poles = self.entity_poles(live)?;
        let read = gather_knots(
            live.nb_knots(),
            degree,
            |rank| live.knot(rank),
            KnotCompare::AtKnot,
        )?;
        let mut gathered = correct_multiplicities(read, degree, || {
            self.warn_with_args(start, "IGES_1200", &[b"", b"", b""])
        })?;
        let poles = without_ranks(&read_poles, &gathered.removed_poles);
        if gathered.sum_of_multiplicities != poles.len() as i32 + degree + 1 {
            self.warn_with_args(start, "IGES_1210", &[b"", b""])?;
        }
        let polynomial = live.is_polynomial(false)?;
        let weights = if polynomial {
            vec![0.0; poles.len()]
        } else {
            match self.entity_weights(start, live)? {
                Some(read_weights) => without_ranks(&read_weights, &gathered.removed_poles),
                None => return Ok(Handle::default()),
            }
        };
        if poles.len() < 2
            || !checked_knots(self, start, polynomial, &mut gathered.knots, &weights)?
        {
            return Ok(Handle::default());
        }
        // A curve that cannot be built is no result, and the reason is on the check.
        let weights_used = (!polynomial).then_some(&weights[..]);
        let curve = match built_curve(&poles, weights_used, &gathered, degree) {
            Ok(curve) => curve,
            Err(failure) => return Ok(self.no_result(start, &failure)),
        };
        Ok(Self::cut_to_entity(curve, live.u_min(), live.u_max()))
    }
    /// A cut that fails leaves a trimmed curve.
    fn cut_to_entity(curve: GeomBSplineCurve, first: f64, last: f64) -> Handle<dyn GeomCurve> {
        let tolerance = precision::p_confusion();
        let inside = first - curve.first_parameter() > -tolerance
            && curve.last_parameter() - last > -tolerance
            && first <= last;
        if !inside {
            return Handle::new(curve).cast();
        }
        let whole = GeomBSplineCurve::copy_of(&curve);
        if (last - first).abs() <= tolerance || curve.segment(first, last, tolerance).is_ok() {
            return Handle::new(curve).cast();
        }
        let basis: Handle<dyn GeomCurve> = match whole {
            Ok(copy) => Handle::new(copy).cast(),
            Err(_) => Handle::new(curve).cast(),
        };
        match GeomTrimmedCurve::new(&basis, first, last, true, true) {
            Ok(trimmed) => Handle::new(trimmed).cast(),
            Err(_) => Handle::default(),
        }
    }
    /// The curve in the plane of the entity, from its transfer in space and with its trim;
    /// `tool.transfer_2d_bspline_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_2d_bspline_curve(
        &self,
        start: &Handle<IgesGeomBSplineCurve>,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Handle::default());
        }
        let spatial = self.transfer_bspline_curve(start)?;
        if spatial.is_null() {
            return Ok(Handle::default());
        }
        let trimmed = spatial.cast::<GeomTrimmedCurve>();
        if let Some(live) = trimmed.get() {
            let bspline = live.basis_curve().cast::<GeomBSplineCurve>();
            if bspline.is_null() {
                return Ok(Handle::default());
            }
            let flat = flattened(&bspline)?;
            let (first, last) = (live.first_parameter(), live.last_parameter());
            return Ok(
                Handle::new(Geom2dTrimmedCurve::new(&flat, first, last, true, true)?).cast(),
            );
        }
        flattened(&spatial.cast())
    }
    /// The B-spline of a spline entity with its continuity raised; a fail without a curve;
    /// `tool.transfer_spline_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_spline_curve(
        &self,
        start: &Handle<IgesGeomSplineCurve>,
    ) -> Result<Handle<GeomBSplineCurve>, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Handle::default());
        }
        let eps_geom = self.get_eps_geom();
        let (status, curve) =
            iges_conv_geom::spline_curve_from_iges(start, self.get_eps_coeff(), eps_geom)?;
        let key = match status {
            5 => Some("XSTEP_246"),
            4 => Some("IGES_1170"),
            3 => Some("IGES_1175"),
            2 => Some("IGES_1180"),
            _ => None,
        };
        if let Some(key) = key {
            self.fail_key(start, key);
            return Ok(curve);
        }
        if let Some(live) = curve.get() {
            let tolerance = precision::confusion().min(eps_geom);
            iges_conv_geom::increase_curve_continuity(live, tolerance, self.get_continuity())?;
        }
        Ok(curve)
    }
    /// The same in the plane: the transfer in space at the parametric tolerance, flattened;
    /// `tool.transfer_2d_spline_curve(&curve)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_2d_spline_curve(
        &mut self,
        start: &Handle<IgesGeomSplineCurve>,
    ) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Handle::default());
        }
        let eps_geom = self.get_eps_geom();
        self.set_eps_geom(precision::p_confusion());
        let spatial = self.transfer_spline_curve(start);
        self.set_eps_geom(eps_geom);
        let spatial = spatial?;
        let Some(live) = spatial.get() else {
            return Ok(Handle::default());
        };
        let poles = planar_poles(live)?;
        Ok(Handle::new(Geom2dBSplineCurve::new(
            &Array1::from_shared(&poles, 1),
            &live.knots(),
            &live.multiplicities(),
            live.degree(),
            false,
        )?))
    }
}

fn built_curve(
    poles: &[Point3],
    weights: Option<&[f64]>,
    gathered: &GatheredKnots,
    degree: i32,
) -> Result<GeomBSplineCurve, Failure> {
    let poles = Array1::from_shared(poles, 1);
    let knots = Array1::from_shared(&gathered.knots, 1);
    let multiplicities = Array1::from_shared(&gathered.multiplicities, 1);
    match weights {
        None => GeomBSplineCurve::new(&poles, &knots, &multiplicities, degree, false),
        Some(values) => {
            let weights = Array1::from_shared(values, 1);
            GeomBSplineCurve::with_weights(&poles, &weights, &knots, &multiplicities, degree, false)
        }
    }
}

pub(super) fn owned_array(values: &[f64]) -> Result<Array1<'static, f64>, Failure> {
    let mut array = Array1::with_bounds(1, values.len() as i32)
        .map_err(|error| error.into_failure(MESSAGE_WHERE))?;
    for (rank, value) in (1..).zip(values) {
        array
            .set_value(rank, *value)
            .map_err(|error| error.into_failure(MESSAGE_WHERE))?;
    }
    Ok(array)
}

fn curve_weights(curve: &GeomBSplineCurve) -> Result<Array1<'static, f64>, Failure> {
    let mut weights = Array1::with_bounds(1, curve.nb_poles())
        .map_err(|error| error.into_failure(MESSAGE_WHERE))?;
    for rank in 1..=curve.nb_poles() {
        weights
            .set_value(rank, curve.weight(rank)?)
            .map_err(|error| error.into_failure(MESSAGE_WHERE))?;
    }
    Ok(weights)
}

fn planar_poles(curve: &GeomBSplineCurve) -> Result<Vec<Point2>, Failure> {
    (1..=curve.nb_poles())
        .map(|rank| {
            let pole = curve.pole(rank)?;
            Ok(Point2::new(pole.x(), pole.y()))
        })
        .collect()
}

/// A curve in the plane with the X and Y of the poles, and the same knots and weights.
fn flattened(curve: &Handle<GeomBSplineCurve>) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let live = crate::iges_data::iges_live(curve, MESSAGE_WHERE)?;
    let poles = planar_poles(live)?;
    let poles = Array1::from_shared(&poles, 1);
    let (knots, multiplicities) = (live.knots(), live.multiplicities());
    let degree = live.degree();
    let flat = if live.is_rational() {
        Geom2dBSplineCurve::with_weights(
            &poles,
            &curve_weights(live)?,
            &knots,
            &multiplicities,
            degree,
            false,
        )?
    } else {
        Geom2dBSplineCurve::new(&poles, &knots, &multiplicities, degree, false)?
    };
    Ok(Handle::new(flat).cast())
}
