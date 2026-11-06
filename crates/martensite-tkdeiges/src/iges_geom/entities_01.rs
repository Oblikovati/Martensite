#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use crate::iges_basic::IgesBasicHArray1OfHArray1OfIgesEntity;
use crate::iges_basic::entity_shape::{
    iges_array1_length, iges_array1_lower, iges_array1_upper, iges_array2_col_length,
    iges_array2_lower_col, iges_array2_lower_row, iges_array2_row_length, iges_array2_upper_col,
    iges_array2_upper_row, iges_placement_methods,
};
use crate::iges_data::IgesDataIgesEntity;
use crate::iges_data::{iges_array1_item, iges_array2_item, iges_live};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{Point3, Xyz};

iges_entity! {
/// defines IGESBSplineCurve, Type `126` Form `0-5`
/// A parametric equation obtained by dividing two summations
/// involving weights (which are real numbers), the control
/// points, and B-Spline basis functions
IgesGeomBSplineCurve, IgesGeomBSplineCurveState, "IgesGeomBSplineCurve", parent IgesDataIgesEntity, { index: i32, degree: i32, is_planar: bool, is_closed: bool, is_polynomial: bool, is_periodic: bool, knots: Handle<HArray1<f64>>, weights: Handle<HArray1<f64>>, poles: Handle<HArray1<Xyz>>, umin: f64, umax: f64, norm: Xyz }
}
impl IgesGeomBSplineCurve {
    iges_methods! {
    get upper_index = index: i32;
    get degree = degree: i32;
    get is_planar = is_planar: bool;
    get is_closed = is_closed: bool;
    get is_periodic = is_periodic: bool;
    len nb_knots = knots;
    item knot = knots: f64;
    len nb_poles = poles;
    item weight = weights: f64;
    get u_min = umin: f64;
    get u_max = umax: f64;
    get normal = norm: Xyz;
    }
    /// This method is used to set the fields of the class
    /// BSplineCurve. Beware about indexation of arrays
    /// - anIndex      : Upper index of the sum
    /// - aDegree      : Degree of basis functions
    /// - aPlanar      : 0 = nonplanar curve, 1 = planar curve
    /// - aClosed      : 0 = open curve, 1 = closed curve
    /// - aPolynom     : 0 = rational, 1 = polynomial
    /// - aPeriodic    : 0 = nonperiodic, 1 = periodic
    /// - allKnots     : Knot sequence values [-Degree,Index+1]
    /// - allWeights   : Array of weights     [0,Index]
    /// - allPoles     : X, Y, Z coordinates of all control points
    ///   [0,Index]
    /// - aUmin, aUmax : Starting and ending parameter values
    /// - aNorm        : Unit normal (if the curve is planar)
    ///   raises exception if allWeights & allPoles are not of same size.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        an_index: i32,
        a_degree: i32,
        a_planar: bool,
        a_closed: bool,
        a_polynom: bool,
        a_periodic: bool,
        all_knots: &Handle<HArray1<f64>>,
        all_weights: &Handle<HArray1<f64>>,
        all_poles: &Handle<HArray1<Xyz>>,
        a_umin: f64,
        a_umax: f64,
        a_norm: Xyz,
    ) -> Result<(), Failure> {
        if !all_poles.is_null() {
            if iges_array1_lower(all_knots, "IgesGeomBSplineCurve::init")? != -a_degree
                || iges_array1_upper(all_knots, "IgesGeomBSplineCurve::init")? != an_index + 1
                || iges_array1_lower(all_poles, "IgesGeomBSplineCurve::init")? != 0
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesGeomBSplineCurve : Init",
                ));
            }
            if !all_weights.is_null() {
                if iges_array1_length(all_poles, "IgesGeomBSplineCurve::init")?
                    != iges_array1_length(all_weights, "IgesGeomBSplineCurve::init")?
                    || iges_array1_upper(all_weights, "IgesGeomBSplineCurve::init")? != an_index
                    || iges_array1_lower(all_weights, "IgesGeomBSplineCurve::init")? != 0
                {
                    return Err(exceptions::dimension_mismatch(
                        "IgesGeomBSplineCurve : Init",
                    ));
                }
            }
        }
        {
            let mut state = self.state.write();
            state.index = an_index;
            state.degree = a_degree;
            state.is_planar = a_planar;
            state.is_closed = a_closed;
            state.is_polynomial = a_polynom;
            state.is_periodic = a_periodic;
            state.knots = all_knots.clone();
            state.weights = all_weights.clone();
            state.poles = all_poles.clone();
            state.umin = a_umin;
            state.umax = a_umax;
            state.norm = a_norm;
        }
        self.init_type_and_form(126, self.form_number());
        Ok(())
    }
    /// Changes FormNumber (indicates the Shape of the Curve)
    /// Error if not in range [0-5]
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if form < 0 || form > 5 {
            return Err(exceptions::out_of_range(
                "IgesGeomBSplineCurve : SetFormNumber",
            ));
        }
        self.init_type_and_form(126, form);
        Ok(())
    }
    /// returns True if the curve is polynomial, False if rational
    /// `flag` False (D) : computed from the list of weights
    /// (all must be equal)
    /// `flag` True : as recorded
    pub fn is_polynomial(&self, flag: bool) -> Result<bool, Failure> {
        let the_weights = self.state.read().weights.clone();
        let is_polynomial = self.state.read().is_polynomial;
        if flag || the_weights.is_null() {
            return Ok(is_polynomial);
        }
        let i1: i32 = iges_array1_lower(&the_weights, "IgesGeomBSplineCurve::is_polynomial")?;
        let i2: i32 = iges_array1_upper(&the_weights, "IgesGeomBSplineCurve::is_polynomial")?;
        let w0: f64 = iges_array1_item(&the_weights, i1, "IgesGeomBSplineCurve::is_polynomial")?;
        for i in (i1 + 1)..=i2 {
            if (iges_array1_item(&the_weights, i, "IgesGeomBSplineCurve::is_polynomial")? - w0)
                .abs()
                > 1.0e-10
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
    /// returns the pole referred to by anIndex, in [0,UpperIndex]
    /// raises exception if anIndex ` 0 or anIndex ` UpperIndex()
    pub fn pole(&self, an_index: i32) -> Result<Point3, Failure> {
        let the_poles = self.state.read().poles.clone();
        let temp_xyz: Xyz = iges_array1_item(&the_poles, an_index, "IgesGeomBSplineCurve::pole")?;
        Ok(Point3::from(temp_xyz))
    }
    /// returns the anIndex'th pole after applying Transf. Matrix
    /// raises exception if an Index ` 0 or an Index ` UpperIndex()
    pub fn transformed_pole(&self, an_index: i32) -> Result<Point3, Failure> {
        let the_poles = self.state.read().poles.clone();
        let mut temp_xyz: Xyz = iges_array1_item(
            &the_poles,
            an_index,
            "IgesGeomBSplineCurve::transformed_pole",
        )?;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(Point3::from(temp_xyz))
    }
}
iges_entity! {
/// defines IGESBSplineSurface, Type `128` Form `0-9`
/// A parametric equation obtained by dividing two summations
/// involving weights (which are real numbers), the control
/// points, and B-Spline basis functions
IgesGeomBSplineSurface, IgesGeomBSplineSurfaceState, "IgesGeomBSplineSurface", parent IgesDataIgesEntity, { index_u: i32, index_v: i32, degree_u: i32, degree_v: i32, is_closed_u: bool, is_closed_v: bool, is_polynomial: bool, is_periodic_u: bool, is_periodic_v: bool, knots_u: Handle<HArray1<f64>>, knots_v: Handle<HArray1<f64>>, weights: Handle<HArray2<f64>>, poles: Handle<HArray2<Xyz>>, umin: f64, umax: f64, vmin: f64, vmax: f64 }
}
impl IgesGeomBSplineSurface {
    iges_methods! {
    get upper_index_u = index_u: i32;
    get upper_index_v = index_v: i32;
    get degree_u = degree_u: i32;
    get degree_v = degree_v: i32;
    get is_closed_u = is_closed_u: bool;
    get is_closed_v = is_closed_v: bool;
    get is_periodic_u = is_periodic_u: bool;
    get is_periodic_v = is_periodic_v: bool;
    len_checked nb_knots_u = knots_u;
    len_checked nb_knots_v = knots_v;
    item knot_u = knots_u: f64;
    item knot_v = knots_v: f64;
    get u_min = umin: f64;
    get u_max = umax: f64;
    get v_min = vmin: f64;
    get v_max = vmax: f64;
    }
    iges_placement_methods! {
    item2 weight = weights: f64;
    }
    /// This method is used to set the fields of the class
    /// BSplineSurface
    /// - anIndexU             : Upper index of first sum
    /// - anIndexV             : Upper index of second sum
    /// - aDegU, aDegV         : Degrees of first and second sets
    ///   of basis functions
    /// - aCloseU, aCloseV     : 1 = Closed in U, V directions
    ///   0 = open in U, V directions
    /// - aPolynom             : 0 = Rational, 1 = polynomial
    /// - aPeriodU, aPeriodV   : 0 = Non periodic in U or V direction
    ///   1 = Periodic in U or V direction
    /// - allKnotsU, allKnotsV : Knots in U and V directions
    /// - allWeights           : Array of weights
    /// - allPoles             : XYZ coordinates of all control points
    /// - aUmin                : Starting value of U direction
    /// - aUmax                : Ending value of U direction
    /// - aVmin                : Starting value of V direction
    /// - aVmax                : Ending value of V direction
    ///   raises exception if allWeights & allPoles are not of same size.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        an_index_u: i32,
        an_index_v: i32,
        a_deg_u: i32,
        a_deg_v: i32,
        a_close_u: bool,
        a_close_v: bool,
        a_polynom: bool,
        a_period_u: bool,
        a_period_v: bool,
        all_knots_u: &Handle<HArray1<f64>>,
        all_knots_v: &Handle<HArray1<f64>>,
        all_weights: &Handle<HArray2<f64>>,
        all_poles: &Handle<HArray2<Xyz>>,
        a_umin: f64,
        a_umax: f64,
        a_vmin: f64,
        a_vmax: f64,
    ) -> Result<(), Failure> {
        if iges_array2_row_length(all_weights, "IgesGeomBSplineSurface::init")?
            != iges_array2_row_length(all_poles, "IgesGeomBSplineSurface::init")?
            || iges_array2_col_length(all_weights, "IgesGeomBSplineSurface::init")?
                != iges_array2_col_length(all_poles, "IgesGeomBSplineSurface::init")?
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomBSplineSurface : Init",
            ));
        }
        if iges_array1_lower(all_knots_u, "IgesGeomBSplineSurface::init")? != -a_deg_u
            || iges_array1_lower(all_knots_v, "IgesGeomBSplineSurface::init")? != -a_deg_v
            || iges_array1_upper(all_knots_u, "IgesGeomBSplineSurface::init")? != an_index_u + 1
            || iges_array1_upper(all_knots_v, "IgesGeomBSplineSurface::init")? != an_index_v + 1
            || iges_array2_lower_row(all_weights, "IgesGeomBSplineSurface::init")? != 0
            || iges_array2_lower_col(all_weights, "IgesGeomBSplineSurface::init")? != 0
            || iges_array2_lower_row(all_poles, "IgesGeomBSplineSurface::init")? != 0
            || iges_array2_lower_col(all_poles, "IgesGeomBSplineSurface::init")? != 0
            || iges_array2_upper_row(all_poles, "IgesGeomBSplineSurface::init")? != an_index_u
            || iges_array2_upper_col(all_poles, "IgesGeomBSplineSurface::init")? != an_index_v
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomBSplineSurface : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.index_u = an_index_u;
            state.index_v = an_index_v;
            state.degree_u = a_deg_u;
            state.degree_v = a_deg_v;
            state.is_closed_u = a_close_u;
            state.is_closed_v = a_close_v;
            state.is_polynomial = a_polynom;
            state.is_periodic_u = a_period_u;
            state.is_periodic_v = a_period_v;
            state.knots_u = all_knots_u.clone();
            state.knots_v = all_knots_v.clone();
            state.weights = all_weights.clone();
            state.poles = all_poles.clone();
            state.umin = a_umin;
            state.umax = a_umax;
            state.vmin = a_vmin;
            state.vmax = a_vmax;
        }
        self.init_type_and_form(128, self.form_number());
        Ok(())
    }
    /// Changes FormNumber (indicates the Shape of the Surface)
    /// Error if not in range [0-9]
    pub fn set_form_number(&self, form: i32) -> Result<(), Failure> {
        if form < 0 || form > 9 {
            return Err(exceptions::out_of_range(
                "IgesGeomBSplineSurface : SetFormNumber",
            ));
        }
        self.init_type_and_form(128, form);
        Ok(())
    }
    /// True if polynomial, False if rational
    /// `flag` False (D) : computed from Weights
    /// `flag` True : recorded
    pub fn is_polynomial(&self, flag: bool) -> Result<bool, Failure> {
        let is_polynomial = self.state.read().is_polynomial;
        let the_weights = self.state.read().weights.clone();
        let the_index_v = self.state.read().index_v;
        let the_index_u = self.state.read().index_u;
        if flag {
            return Ok(is_polynomial);
        }
        let w0: f64 =
            iges_array2_item(&the_weights, 0, 0, "IgesGeomBSplineSurface::is_polynomial")?;
        for j in 0..the_index_v + 1 {
            for i in 0..the_index_u + 1 {
                if (iges_array2_item(&the_weights, i, j, "IgesGeomBSplineSurface::is_polynomial")?
                    - w0)
                    .abs()
                    > 1.0e-10
                {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
    /// returns number of poles in U direction
    pub fn nb_poles_u(&self) -> i32 {
        let the_index_u = self.state.read().index_u;
        the_index_u + 1
    }
    /// returns number of poles in V direction
    pub fn nb_poles_v(&self) -> i32 {
        let the_index_v = self.state.read().index_v;
        the_index_v + 1
    }
    /// returns the control point referenced by anIndex1, anIndex2
    /// raises exception if anIndex1 `= 0 or anIndex1 ` NbPolesU()
    /// or if anIndex2 `= 0 or anIndex2 ` NbPolesV()
    pub fn pole(&self, an_index1: i32, an_index2: i32) -> Result<Point3, Failure> {
        let the_poles = self.state.read().poles.clone();
        let temp_xyz: Xyz = iges_array2_item(
            &the_poles,
            an_index1,
            an_index2,
            "IgesGeomBSplineSurface::pole",
        )?;
        Ok(Point3::from(temp_xyz))
    }
    /// returns the control point referenced by anIndex1, anIndex2
    /// after applying the Transf.Matrix
    /// raises exception if anIndex1 `= 0 or anIndex1 ` NbPolesU()
    /// or if anIndex2 `= 0 or anIndex2 ` NbPolesV()
    pub fn transformed_pole(&self, an_index1: i32, an_index2: i32) -> Result<Point3, Failure> {
        let the_poles = self.state.read().poles.clone();
        let mut temp_xyz: Xyz = iges_array2_item(
            &the_poles,
            an_index1,
            an_index2,
            "IgesGeomBSplineSurface::transformed_pole",
        )?;
        if self.has_transf() {
            self.location()?.transforms(&mut temp_xyz);
        }
        Ok(Point3::from(temp_xyz))
    }
}
iges_entity! {
/// defines IGESBoundary, Type `141` Form `0`
/// A boundary entity identifies a surface boundary consisting
/// of a set of curves lying on the surface
IgesGeomBoundary, IgesGeomBoundaryState, "IgesGeomBoundary", parent IgesDataIgesEntity, { type_: i32, preference: i32, surface: Handle<IgesDataIgesEntity>, model_curves: Handle<HArray1<Handle<IgesDataIgesEntity>>>, senses: Handle<HArray1<i32>>, parameter_curves: Handle<IgesBasicHArray1OfHArray1OfIgesEntity> }
}
impl IgesGeomBoundary {
    iges_methods! {
    get boundary_type = type_: i32;
    get preference_type = preference: i32;
    get surface = surface: Handle<IgesDataIgesEntity>;
    len_checked nb_model_space_curves = model_curves;
    item model_space_curve = model_curves: Handle<IgesDataIgesEntity>;
    item sense = senses: i32;
    }
    /// This method is used to set the fields of the class
    /// Boundary
    /// - aType              : Type of bounded surface representation
    /// - aPreference        : Preferred representation of
    ///   Trimming Curve
    /// - aSurface           : Untrimmed surface to be bounded
    /// - allModelCurves     : Model Space Curves
    /// - allSenses          : Orientation flags of all Model Space
    ///   Curves
    /// - allParameterCurves : Parameter Space Curves
    ///   raises exception if allSenses, allModelCurves and
    ///   allParameterCurves do not have same dimensions
    pub fn init(
        &self,
        a_type: i32,
        a_preference: i32,
        a_surface: &Handle<IgesDataIgesEntity>,
        all_model_curves: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
        all_senses: &Handle<HArray1<i32>>,
        all_parameter_curves: &Handle<IgesBasicHArray1OfHArray1OfIgesEntity>,
    ) -> Result<(), Failure> {
        let all_parameter_curves_live = iges_live(all_parameter_curves, "IgesGeomBoundary::init")?;
        let num1: i32 = iges_array1_length(all_senses, "IgesGeomBoundary::init")?;
        if iges_array1_lower(all_senses, "IgesGeomBoundary::init")? != 1
            || (iges_array1_lower(all_model_curves, "IgesGeomBoundary::init")? != 1
                || iges_array1_length(all_model_curves, "IgesGeomBoundary::init")? != num1)
            || (all_parameter_curves_live.lower() != 1
                || all_parameter_curves_live.length() != num1)
        {
            return Err(exceptions::dimension_mismatch("IgesGeomBoundary: Init"));
        }
        {
            let mut state = self.state.write();
            state.type_ = a_type;
            state.preference = a_preference;
            state.surface = a_surface.clone();
            state.model_curves = all_model_curves.clone();
            state.senses = all_senses.clone();
            state.parameter_curves = all_parameter_curves.clone();
        }
        self.init_type_and_form(141, 0);
        Ok(())
    }
    /// returns the number of parameter curves associated with one
    /// model space curve referred to by Index
    /// raises exception if Index `= 0 or Index ` NbModelSpaceCurves()
    pub fn nb_parameter_curves(&self, index: i32) -> Result<i32, Failure> {
        let the_parameter_curves = self.state.read().parameter_curves.clone();
        if iges_live(
            &the_parameter_curves,
            "IgesGeomBoundary::nb_parameter_curves",
        )?
        .value(index)?
        .is_null()
        {
            return Ok(0);
        }
        iges_array1_length(
            &iges_live(
                &the_parameter_curves,
                "IgesGeomBoundary::nb_parameter_curves",
            )?
            .value(index)?,
            "IgesGeomBoundary::nb_parameter_curves",
        )
    }
    /// returns an array of parameter space curves associated with
    /// a model space curve referred to by the Index
    /// raises exception if Index `= 0 or Index ` NbModelSpaceCurves()
    pub fn parameter_curves(
        &self,
        index: i32,
    ) -> Result<Handle<HArray1<Handle<IgesDataIgesEntity>>>, Failure> {
        let the_parameter_curves = self.state.read().parameter_curves.clone();
        iges_live(&the_parameter_curves, "IgesGeomBoundary::parameter_curves")?.value(index)
    }
    /// returns an individual parameter curve
    /// raises exception if Index or Num is out of range
    pub fn parameter_curve(
        &self,
        index: i32,
        num: i32,
    ) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        let the_parameter_curves = self.state.read().parameter_curves.clone();
        iges_array1_item(
            &iges_live(&the_parameter_curves, "IgesGeomBoundary::parameter_curve")?.value(index)?,
            num,
            "IgesGeomBoundary::parameter_curve",
        )
    }
}
