//! Trimmed curves: the parameters of their trimming selects.
use super::StepToGeom;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_geom::{
    StepGeomAxis2Placement3d, StepGeomCircle, StepGeomConic, StepGeomCurve, StepGeomEllipse,
    StepGeomLine, StepGeomTrimmedCurve, StepGeomTrimmingPreference, StepGeomTrimmingSelect,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomCurve, GeomPoint, GeomTrimmedCurve};
use martensite_tkgeombase::geom2d_convert;
use martensite_tkmath::convert::ParameterisationType;
use martensite_tkmath::elclib;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisCurve;
use std::f64::consts::{FRAC_PI_2, PI};

/// The master representation numbers: 1 cartesian, 2 parameter, 0 unspecified.
const MASTER_CARTESIAN: i32 = 1;
const MASTER_PARAMETER: i32 = 2;
const MASTER_UNSPECIFIED: i32 = 0;

/// The trimming selects of one end, read once.
struct TrimEnd {
    selects: Handle<HArray1<StepGeomTrimmingSelect>>,
    count: i32,
}

impl TrimEnd {
    fn select(&self, rank: i32) -> Result<StepGeomTrimmingSelect, Failure> {
        step_array1_item(&self.selects, rank, "StepToGeom trimming select")
    }
    fn has_point(&self) -> Result<bool, Failure> {
        for rank in 1..=self.count {
            if self.select(rank)?.cartesian_point().is_some() {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// The scale and the shift from a STEP parameter to a Geom one.
#[derive(Clone, Copy)]
struct ParameterScale {
    factor: f64,
    shift: f64,
}

impl ParameterScale {
    fn value(self, parameter: f64) -> f64 {
        self.shift + self.factor * parameter
    }
}

/// The parameter of a point: S4136: uses the advanced algorithm of the analysis tool.
fn project_select(
    curve: &Handle<dyn GeomCurve>,
    select: &StepGeomTrimmingSelect,
    factors: &StepDataFactors,
) -> Result<f64, Failure> {
    let point = StepToGeom::make_cartesian_point(&select.cartesian_point(), factors)?;
    let live = step_live(&point, "StepToGeom ExtractParameter")?;
    let range = (live_first(curve)?, live_last(curve)?);
    let projected =
        ShapeAnalysisCurve.project(curve, live.pnt(), precision::confusion(), range, true)?;
    let (_distance, _projection, parameter) = projected;
    Ok(parameter)
}

fn live_first(curve: &Handle<dyn GeomCurve>) -> Result<f64, Failure> {
    Ok(step_live(curve, "StepToGeom ExtractParameter")?.first_parameter())
}

fn live_last(curve: &Handle<dyn GeomCurve>) -> Result<f64, Failure> {
    Ok(step_live(curve, "StepToGeom ExtractParameter")?.last_parameter())
}

/// `extract_parameter`: the master representation first; if it is unspecified a parameter value
/// is preferred and otherwise one is computed from the point.
fn extract_parameter(
    curve: &Handle<dyn GeomCurve>,
    end: &TrimEnd,
    master: i32,
    scale: ParameterScale,
    factors: &StepDataFactors,
) -> Result<Option<f64>, Failure> {
    for rank in 1..=end.count {
        let select = end.select(rank)?;
        if master == MASTER_PARAMETER && select.case_member() > 0 {
            return Ok(Some(scale.value(select.parameter_value())));
        }
        if master == MASTER_CARTESIAN && select.case_number() > 0 {
            return Ok(Some(project_select(curve, &select, factors)?));
        }
    }
    for rank in 1..=end.count {
        let select = end.select(rank)?;
        if select.case_member() > 0 {
            return Ok(Some(scale.value(select.parameter_value())));
        }
    }
    for rank in 1..=end.count {
        let select = end.select(rank)?;
        if select.case_number() > 0 {
            return Ok(Some(project_select(curve, &select, factors)?));
        }
    }
    Ok(None)
}

/// Is the placement of the conic a 3D one without X direction? Then the parameters mean nothing:
/// skl 04.02.2002 for bug 133: no trimmed curve can be made from them.
fn conic_has_no_x_direction(curve: &Handle<StepGeomCurve>) -> bool {
    let conic = curve.cast::<StepGeomConic>();
    let Some(live) = conic.get() else {
        return false;
    };
    let select = live.position();
    if select.case_num(&select.value()) != 2 {
        return false;
    }
    let placement: Handle<StepGeomAxis2Placement3d> = select.value().cast();
    placement
        .get()
        .is_some_and(|axis| !axis.has_ref_direction())
}

/// The shift of an ellipse with its axes out of order: :p3 abv 23 Feb 99.
fn ellipse_shift(curve: &Handle<StepGeomCurve>) -> f64 {
    let ellipse = curve.cast::<StepGeomEllipse>();
    let reversed = ellipse
        .get()
        .is_some_and(|live| live.semi_axis1() - live.semi_axis2() < 0.0);
    if reversed { FRAC_PI_2 } else { 0.0 }
}

/// Clamps the parameters of a non periodic curve: :abv 09.04.99: S4136: bm2_ug_t4-B.stp #70610.
fn clamp_to_range(parameter: f64, first: f64, last: f64) -> f64 {
    if parameter < first {
        return first;
    }
    if parameter > last { last } else { parameter }
}

/// Parameters that coincide: spread them over a closed curve; `None` on another curve.
fn separate_equal_parameters(
    curve: &dyn GeomCurve,
    trim1: f64,
    trim2: f64,
) -> Result<Option<(f64, f64)>, Failure> {
    let (first, last) = (curve.first_parameter(), curve.last_parameter());
    if curve.is_periodic() {
        return Ok(Some(elclib::adjust_periodic(
            first,
            last,
            precision::p_confusion(),
            trim1,
            trim2,
        )));
    }
    if !curve.is_closed()? {
        return Ok(None);
    }
    if (trim1 - first).abs() < precision::p_confusion() {
        return Ok(Some((trim1, trim2 + last)));
    }
    Ok(Some((trim1 - last, trim2)))
}

/// The trimmed curve of the extracted parameters, in the direction of the sense agreement.
fn trimmed_from_parameters(
    curve: &Handle<dyn GeomCurve>,
    mut trim1: f64,
    mut trim2: f64,
    sense: bool,
) -> Result<Handle<GeomTrimmedCurve>, Failure> {
    let live = step_live(curve, "StepToGeom::MakeTrimmedCurve")?;
    if !live.is_periodic() {
        trim1 = clamp_to_range(trim1, live.first_parameter(), live.last_parameter());
        trim2 = clamp_to_range(trim2, live.first_parameter(), live.last_parameter());
    }
    if (trim1 - trim2).abs() < precision::p_confusion() {
        let Some(separated) = separate_equal_parameters(live, trim1, trim2)? else {
            return Ok(Handle::default());
        };
        (trim1, trim2) = separated;
    }
    // abv 29.09.00 PRO20362: reverse parameters in case of reversed curve
    let (u1, u2) = if sense {
        (trim1, trim2)
    } else {
        (trim2, trim1)
    };
    Ok(Handle::new(GeomTrimmedCurve::new(
        curve, u1, u2, sense, true,
    )?))
}

/// The master representation of an entity, with CARTESIAN_POINT selects under an unspecified or
/// parameter master recognized: gka 18.02.04.
fn master_of(sc: &StepGeomTrimmedCurve) -> i32 {
    match sc.master_representation() {
        StepGeomTrimmingPreference::TpCartesian => MASTER_CARTESIAN,
        StepGeomTrimmingPreference::TpParameter => MASTER_PARAMETER,
        _ => MASTER_UNSPECIFIED,
    }
}

impl StepToGeom {
    /// `make_trimmed_curve`: null when the basis or a parameter cannot be made.
    pub fn make_trimmed_curve(
        sc: &Handle<StepGeomTrimmedCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomTrimmedCurve>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeTrimmedCurve")?;
        let basis = live.basis_curve();
        let curve = Self::make_curve(&basis, factors)?;
        if curve.is_null() {
            return Ok(Handle::default());
        }
        let end1 = TrimEnd {
            selects: live.trim1(),
            count: live.nb_trim1()?,
        };
        let end2 = TrimEnd {
            selects: live.trim2(),
            count: live.nb_trim2()?,
        };
        let mut master = master_of(live);
        let inspect = master == MASTER_UNSPECIFIED
            || (master == MASTER_PARAMETER && end1.count > 1 && end2.count > 1);
        let is_point = inspect && end1.has_point()? && end2.has_point()?;
        let mut scale = ParameterScale {
            factor: 1.0,
            shift: 0.0,
        };
        if let Some(line) = basis.cast::<StepGeomLine>().get() {
            let direction = line.dir();
            scale.factor = step_live(&direction, "StepToGeom::MakeTrimmedCurve")?.magnitude()
                * factors.length_factor();
        } else if basis.cast::<StepGeomCircle>().is_some()
            || basis.cast::<StepGeomEllipse>().is_some()
        {
            scale.factor = factors.plane_angle_factor();
            scale.shift = ellipse_shift(&basis);
            // CKY 6-FEB-2004 for Airbus-MedialAxis: this restriction does not apply for trimming by POINTS
            if master != MASTER_CARTESIAN && conic_has_no_x_direction(&basis) {
                if !is_point {
                    let (u1, u2) = if live.sense_agreement() {
                        (0.0, 2.0 * PI)
                    } else {
                        (2.0 * PI, 0.0)
                    };
                    let trimmed =
                        GeomTrimmedCurve::new(&curve, u1, u2, live.sense_agreement(), true)?;
                    return Ok(Handle::new(trimmed));
                }
                master = MASTER_CARTESIAN;
            }
        }
        let trim1 = extract_parameter(&curve, &end1, master, scale, factors)?;
        let trim2 = extract_parameter(&curve, &end2, master, scale, factors)?;
        match (trim1, trim2) {
            (Some(first), Some(second)) => {
                trimmed_from_parameters(&curve, first, second, live.sense_agreement())
            }
            _ => Ok(Handle::default()),
        }
    }
    /// The factor and the shift of a 2D basis: 2D lengths are not scaled.
    fn scale_2d(
        basis: &Handle<StepGeomCurve>,
        factors: &StepDataFactors,
    ) -> Result<ParameterScale, Failure> {
        if let Some(line) = basis.cast::<StepGeomLine>().get() {
            let direction = line.dir();
            let factor = step_live(&direction, "StepToGeom::MakeTrimmedCurve2d")?.magnitude();
            return Ok(ParameterScale { factor, shift: 0.0 });
        }
        if basis.cast::<StepGeomCircle>().is_some() || basis.cast::<StepGeomEllipse>().is_some() {
            return Ok(ParameterScale {
                factor: factors.plane_angle_factor(),
                shift: ellipse_shift(basis),
            });
        }
        // Parabola and hyperbola: "LATER !!!", the parameters stay as they are.
        Ok(ParameterScale {
            factor: 1.0,
            shift: 0.0,
        })
    }
    /// `MakeTrimmedCurve2d`: only parameter trimming; a spline basis is returned untrimmed.
    pub fn make_trimmed_curve_2d(
        sc: &Handle<StepGeomTrimmedCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeTrimmedCurve2d")?;
        let basis = live.basis_curve();
        let curve: Handle<dyn Geom2dCurve> = Self::make_curve_2d(&basis, factors)?;
        if curve.is_null() {
            return Ok(Handle::default());
        }
        let spline = curve.cast::<Geom2dBSplineCurve>();
        if spline.is_some() {
            return Ok(spline);
        }
        let end1 = TrimEnd {
            selects: live.trim1(),
            count: live.nb_trim1()?,
        };
        let end2 = TrimEnd {
            selects: live.trim2(),
            count: live.nb_trim2()?,
        };
        if end1.count != 1 || end2.count != 1 {
            return Ok(Handle::default());
        }
        let (select1, select2) = (end1.select(1)?, end2.select(1)?);
        if select1.case_member() <= 0 || select2.case_member() <= 0 {
            return Ok(Handle::default());
        }
        let scale = Self::scale_2d(&basis, factors)?;
        let u1 = scale.value(select1.parameter_value());
        let u2 = scale.value(select2.parameter_value());
        let trimmed: Handle<dyn Geom2dCurve> = Handle::new(Geom2dTrimmedCurve::new(
            &curve,
            u1,
            u2,
            live.sense_agreement(),
            true,
        )?)
        .cast();
        geom2d_convert::curve_to_bspline_curve(&trimmed, ParameterisationType::TgtThetaOver2)
    }
}
