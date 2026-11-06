//! B-spline and Bezier curves into the rational B-spline entity (type 126).
use super::GeomToIgesGeomCurve;
use super::curve::EntityResult;
use crate::iges_data::iges_live;
use crate::iges_geom::IgesGeomBSplineCurve;
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomConic, GeomCurve, GeomLine, GeomOffsetCurve,
    GeomTrimmedCurve,
};
use martensite_tkgeombase::geom_convert;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::gp::{Point3, Xyz};

const WHERE: &str = "GeomToIgesGeomCurve";

/// Some normal of a direction, the Z axis when the direction is along it.
fn any_normal(direction: Xyz) -> Xyz {
    let z_axis = Xyz::new(0.0, 0.0, 1.0);
    if direction.z().abs() < precision::confusion() {
        return z_axis;
    }
    let normal = Xyz::new(direction.z(), 0.0, -direction.x());
    let modulus = normal.modulus();
    if modulus < precision::confusion() {
        return z_axis;
    }
    normal / modulus
}

/// The area vector of the closed polygon of the poles.
fn polygon_normal(poles: &[Point3]) -> Xyz {
    let closing = poles.len() - 1;
    let start = poles[closing].xyz().crossed(poles[0].xyz());
    poles.windows(2).fold(start, |normal, pair| {
        normal + pair[0].xyz().crossed(pair[1].xyz())
    })
}

/// Whether the poles lie in a plane, and its normal; with no normal the Z axis is answered.
fn poles_plane(poles: &[Point3]) -> (bool, Xyz) {
    if poles.len() < 3 {
        return (true, any_normal(poles[0].xyz() - poles[1].xyz()));
    }
    let tolerance = precision::confusion();
    let area = polygon_normal(poles);
    let modulus = area.modulus();
    if modulus < tolerance {
        return (false, Xyz::new(0.0, 0.0, 1.0));
    }
    let normal = area / modulus;
    let reference = poles[0].xyz().dot(normal);
    let planar = poles[1..]
        .iter()
        .all(|pole| (pole.xyz().dot(normal) - reference).abs() <= tolerance);
    (planar, normal)
}

/// Whether a curve is planar, and its normal; a curve of unknown kind has a null normal.
pub(super) fn planar_normal(curve: &Handle<dyn GeomCurve>) -> Result<(bool, Xyz), Failure> {
    let line = curve.cast::<GeomLine>();
    if let Some(live) = line.get() {
        return Ok((true, any_normal(live.position().direction().xyz())));
    }
    let conic = curve.cast::<dyn GeomConic>();
    if let Some(live) = conic.get() {
        return Ok((true, live.axis().direction().xyz()));
    }
    let trimmed = curve.cast::<GeomTrimmedCurve>();
    if let Some(live) = trimmed.get() {
        return planar_normal(&live.basis_curve());
    }
    let offset = curve.cast::<GeomOffsetCurve>();
    if let Some(live) = offset.get() {
        return planar_normal(&live.basis_curve());
    }
    planar_normal_of_poles(curve)
}

fn planar_normal_of_poles(curve: &Handle<dyn GeomCurve>) -> Result<(bool, Xyz), Failure> {
    let bspline = curve.cast::<GeomBSplineCurve>();
    if let Some(live) = bspline.get() {
        return Ok(live.poles_array(|poles| poles_plane(poles.as_slice())));
    }
    let bezier = curve.cast::<GeomBezierCurve>();
    if let Some(live) = bezier.get() {
        return Ok(poles_plane(live.poles().as_slice()));
    }
    Ok((false, Xyz::new(0.0, 0.0, 0.0)))
}

/// An array with the values of `values`, its first item at `lower`.
pub(super) fn shared_array<T>(values: &[T], lower: i32) -> Result<Handle<HArray1<T>>, Failure>
where
    T: Clone + Default + Send + Sync + 'static,
{
    let upper = lower + values.len() as i32 - 1;
    let array = crate::iges_data::iges_new_array1(lower, upper)?;
    for (rank, value) in (lower..).zip(values) {
        crate::iges_data::iges_array1_set(&array, rank, value.clone(), WHERE)?;
    }
    Ok(array)
}

impl GeomToIgesGeomCurve {
    /// The range to write and the curve it is a whole of: the given parameters, snapped to the
    /// bounds of the curve, cut out of a copy. A curve that refuses the cut is written whole.
    fn segmented(
        curve: &Handle<GeomBSplineCurve>,
        lower: f64,
        upper: f64,
    ) -> Result<(Handle<GeomBSplineCurve>, f64, f64), Failure> {
        let live = iges_live(curve, WHERE)?;
        let tolerance = precision::p_confusion();
        let (first, last) = (live.first_parameter(), live.last_parameter());
        let umin = if lower - first < tolerance {
            first
        } else {
            lower
        };
        let umax = if last - upper < tolerance {
            last
        } else {
            upper
        };
        if umin - first <= tolerance && last - umax <= tolerance {
            return Ok((curve.clone(), umin, umax));
        }
        let segment = Self::cut_copy(live, (umin, umax), tolerance);
        Ok((segment.unwrap_or_else(|| curve.clone()), umin, umax))
    }
    fn cut_copy(
        live: &GeomBSplineCurve,
        (umin, umax): (f64, f64),
        tolerance: f64,
    ) -> Option<Handle<GeomBSplineCurve>> {
        let copy = GeomBSplineCurve::copy_of(live).ok()?;
        if (umax - umin).abs() > tolerance {
            copy.segment(umin, umax, tolerance).ok()?;
        }
        Some(Handle::new(copy))
    }
    /// A copy without its periodicity when the curve is periodic, the curve itself otherwise.
    fn unperiodic(start: &Handle<GeomBSplineCurve>) -> Result<Handle<GeomBSplineCurve>, Failure> {
        let live = iges_live(start, WHERE)?;
        if !live.is_periodic() {
            return Ok(start.clone());
        }
        let copy = GeomBSplineCurve::copy_of(live)?;
        copy.set_not_periodic()?;
        Ok(Handle::new(copy))
    }
    /// The B-spline entity between two parameters; `tool.transfer_bspline_curve(&curve, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the reading of the curve.
    pub fn transfer_bspline_curve(
        &self,
        start: &Handle<GeomBSplineCurve>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        let Some(original) = start.get() else {
            return Ok(Handle::default());
        };
        let periodic = original.is_periodic();
        let (lower, upper) = super::bounded_range(first, last);
        let (cut, umin, umax) = Self::segmented(&Self::unperiodic(start)?, lower, upper)?;
        let curve = iges_live(&cut, WHERE)?;
        let index = curve.nb_poles() - 1;
        let degree = curve.degree();
        let knots = shared_array(curve.knot_sequence().as_slice(), -degree)?;
        let weight_values = (1..=curve.nb_poles())
            .map(|rank| curve.weight(rank))
            .collect::<Result<Vec<f64>, Failure>>()?;
        let weights = shared_array(&weight_values, 0)?;
        let poles = curve.poles_array(|poles| self.pole_array(poles))?;
        let (planar, normal) = planar_normal(&start.cast())?;
        let entity = IgesGeomBSplineCurve::new();
        entity.init(
            index,
            degree,
            planar,
            curve.is_closed()?,
            !curve.is_rational(),
            periodic,
            &knots,
            &weights,
            &poles,
            umin,
            umax,
            if normal.z() < 0.0 {
                normal.reversed()
            } else {
                normal
            },
        )?;
        Ok(Handle::new(entity).cast())
    }
    fn pole_array(&self, poles: &Array1<'_, Point3>) -> Result<Handle<HArray1<Xyz>>, Failure> {
        let scaled: Vec<Xyz> = poles
            .as_slice()
            .iter()
            .map(|pole| self.in_unit(pole.xyz()))
            .collect();
        shared_array(&scaled, 0)
    }
    /// The B-spline entity of a Bezier curve trimmed between two parameters;
    /// `tool.transfer_bezier_curve(&curve, 0.0, 1.0)?`.
    ///
    /// # Errors
    /// A failure of the trim or of the conversion.
    pub fn transfer_bezier_curve(
        &self,
        start: &Handle<GeomBezierCurve>,
        first: f64,
        last: f64,
    ) -> EntityResult {
        if start.is_null() {
            return Ok(Handle::default());
        }
        let basis: Handle<dyn GeomCurve> = start.cast();
        let trimmed = GeomTrimmedCurve::new(&basis, first, last, true, true)?;
        let converted = geom_convert::curve_to_bspline_curve(
            &Handle::new(trimmed).cast(),
            ParameterisationType::RationalC1,
        )?;
        let live = iges_live(&converted, WHERE)?;
        self.transfer_bspline_curve(&converted, live.first_parameter(), live.last_parameter())
    }
}
