#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// B spline curve with knots and rational B spline curve.
    StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve, StepGeomBSplineCurveWithKnotsAndRationalBSplineCurveState, "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve", StepGeomBSplineCurve, {
        b_spline_curve_with_knots: Handle<StepGeomBSplineCurveWithKnots>,
        rational_b_spline_curve: Handle<StepGeomRationalBSplineCurve>,
    }
}
impl StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve {
    step_methods! {
        set set_b_spline_curve_with_knots = b_spline_curve_with_knots: Handle<StepGeomBSplineCurveWithKnots>;
        get b_spline_curve_with_knots = b_spline_curve_with_knots: Handle<StepGeomBSplineCurveWithKnots>;
        set set_rational_b_spline_curve = rational_b_spline_curve: Handle<StepGeomRationalBSplineCurve>;
        get rational_b_spline_curve = rational_b_spline_curve: Handle<StepGeomRationalBSplineCurve>;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_degree: i32,
        a_control_points_list: &Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
        a_curve_form: StepGeomBSplineCurveForm,
        a_closed_curve: StepDataLogical,
        a_self_intersect: StepDataLogical,
        a_b_spline_curve_with_knots: &Handle<StepGeomBSplineCurveWithKnots>,
        a_rational_b_spline_curve: &Handle<StepGeomRationalBSplineCurve>,
    ) {
        let mut state = self.state.write();
        state.b_spline_curve_with_knots = a_b_spline_curve_with_knots.clone();
        state.rational_b_spline_curve = a_rational_b_spline_curve.clone();
        drop(state);
        self.parent.init(
            a_name,
            a_degree,
            a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
        );
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init_fields(
        &self,
        a_name: &Handle<HAsciiString>,
        a_degree: i32,
        a_control_points_list: &Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
        a_curve_form: StepGeomBSplineCurveForm,
        a_closed_curve: StepDataLogical,
        a_self_intersect: StepDataLogical,
        a_knot_multiplicities: &Handle<HArray1<i32>>,
        a_knots: &Handle<HArray1<f64>>,
        a_knot_spec: StepGeomKnotType,
        a_weights_data: &Handle<HArray1<f64>>,
    ) {
        self.parent.init(
            a_name,
            a_degree,
            a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
        );
        self.state.write().b_spline_curve_with_knots =
            Handle::new(StepGeomBSplineCurveWithKnots::new());
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        field_b_spline_curve_with_knots.init(
            a_name,
            a_degree,
            a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
            a_knot_multiplicities,
            a_knots,
            a_knot_spec,
        );
        self.state.write().rational_b_spline_curve =
            Handle::new(StepGeomRationalBSplineCurve::new());
        let field_rational_b_spline_curve = self.state.read().rational_b_spline_curve.clone();
        field_rational_b_spline_curve.init(
            a_name,
            a_degree,
            a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
            a_weights_data,
        );
    }
    /// `set_knot_multiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_knot_multiplicities(
        &self,
        a_knot_multiplicities: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::SetKnotMultiplicities",
        )?
        .set_knot_multiplicities(a_knot_multiplicities);
        Ok(())
    }
    /// `knot_multiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn knot_multiplicities(&self) -> Result<Handle<HArray1<i32>>, Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        Ok(step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::KnotMultiplicities",
        )?
        .knot_multiplicities())
    }
    /// `knot_multiplicities_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn knot_multiplicities_value(&self, num: i32) -> Result<i32, Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::KnotMultiplicitiesValue",
        )?
        .knot_multiplicities_value(num)
    }
    /// `nb_knot_multiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_knot_multiplicities(&self) -> Result<i32, Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::NbKnotMultiplicities",
        )?
        .nb_knot_multiplicities()
    }
    /// `set_knots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_knots(&self, a_knots: &Handle<HArray1<f64>>) -> Result<(), Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::SetKnots",
        )?
        .set_knots(a_knots);
        Ok(())
    }
    /// `Knots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn knots(&self) -> Result<Handle<HArray1<f64>>, Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        Ok(step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::Knots",
        )?
        .knots())
    }
    /// `knots_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn knots_value(&self, num: i32) -> Result<f64, Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::KnotsValue",
        )?
        .knots_value(num)
    }
    /// `nb_knots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_knots(&self) -> Result<i32, Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::NbKnots",
        )?
        .nb_knots()
    }
    /// `set_knot_spec`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_knot_spec(&self, a_knot_spec: StepGeomKnotType) -> Result<(), Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::SetKnotSpec",
        )?
        .set_knot_spec(a_knot_spec);
        Ok(())
    }
    /// `knot_spec`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn knot_spec(&self) -> Result<StepGeomKnotType, Failure> {
        let field_b_spline_curve_with_knots = self.state.read().b_spline_curve_with_knots.clone();
        Ok(step_live(
            &field_b_spline_curve_with_knots,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::KnotSpec",
        )?
        .knot_spec())
    }
    /// `set_weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_weights_data(&self, a_weights_data: &Handle<HArray1<f64>>) -> Result<(), Failure> {
        let field_rational_b_spline_curve = self.state.read().rational_b_spline_curve.clone();
        step_live(
            &field_rational_b_spline_curve,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::SetWeightsData",
        )?
        .set_weights_data(a_weights_data);
        Ok(())
    }
    /// `weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn weights_data(&self) -> Result<Handle<HArray1<f64>>, Failure> {
        let field_rational_b_spline_curve = self.state.read().rational_b_spline_curve.clone();
        Ok(step_live(
            &field_rational_b_spline_curve,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::WeightsData",
        )?
        .weights_data())
    }
    /// `weights_data_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn weights_data_value(&self, num: i32) -> Result<f64, Failure> {
        let field_rational_b_spline_curve = self.state.read().rational_b_spline_curve.clone();
        step_live(
            &field_rational_b_spline_curve,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::WeightsDataValue",
        )?
        .weights_data_value(num)
    }
    /// `nb_weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_weights_data(&self) -> Result<i32, Failure> {
        let field_rational_b_spline_curve = self.state.read().rational_b_spline_curve.clone();
        step_live(
            &field_rational_b_spline_curve,
            "StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve::NbWeightsData",
        )?
        .nb_weights_data()
    }
}

step_entity! {
    /// B spline surface.
    StepGeomBSplineSurface, StepGeomBSplineSurfaceState, "StepGeomBSplineSurface", StepGeomBoundedSurface, {
        u_degree: i32,
        v_degree: i32,
        control_points_list: Handle<HArray2<Handle<StepGeomCartesianPoint>>>,
        surface_form: StepGeomBSplineSurfaceForm,
        u_closed: StepDataLogical,
        v_closed: StepDataLogical,
        self_intersect: StepDataLogical,
    }
}
impl StepGeomBSplineSurface {
    step_methods! {
        set_copy set_u_degree = u_degree: i32;
        get u_degree = u_degree: i32;
        set_copy set_v_degree = v_degree: i32;
        get v_degree = v_degree: i32;
        set set_control_points_list = control_points_list: Handle<HArray2<Handle<StepGeomCartesianPoint>>>;
        get control_points_list = control_points_list: Handle<HArray2<Handle<StepGeomCartesianPoint>>>;
        set_copy set_surface_form = surface_form: StepGeomBSplineSurfaceForm;
        get surface_form = surface_form: StepGeomBSplineSurfaceForm;
        set_copy set_u_closed = u_closed: StepDataLogical;
        get u_closed = u_closed: StepDataLogical;
        set_copy set_v_closed = v_closed: StepDataLogical;
        get v_closed = v_closed: StepDataLogical;
        set_copy set_self_intersect = self_intersect: StepDataLogical;
        get self_intersect = self_intersect: StepDataLogical;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_u_degree: i32,
        a_v_degree: i32,
        a_control_points_list: &Handle<HArray2<Handle<StepGeomCartesianPoint>>>,
        a_surface_form: StepGeomBSplineSurfaceForm,
        a_u_closed: StepDataLogical,
        a_v_closed: StepDataLogical,
        a_self_intersect: StepDataLogical,
    ) {
        let mut state = self.state.write();
        state.u_degree = a_u_degree;
        state.v_degree = a_v_degree;
        state.control_points_list = a_control_points_list.clone();
        state.surface_form = a_surface_form;
        state.u_closed = a_u_closed;
        state.v_closed = a_v_closed;
        state.self_intersect = a_self_intersect;
        drop(state);
        self.parent.parent.parent.parent.init(a_name);
    }
    /// `control_points_list_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn control_points_list_value(
        &self,
        num1: i32,
        num2: i32,
    ) -> Result<Handle<StepGeomCartesianPoint>, Failure> {
        let field_control_points_list = self.state.read().control_points_list.clone();
        step_array2_item(
            &field_control_points_list,
            num1,
            num2,
            "StepGeomBSplineSurface::ControlPointsListValue",
        )
    }
    /// `nb_control_points_list_i`.
    pub fn nb_control_points_list_i(&self) -> i32 {
        if self.state.read().control_points_list.is_null() {
            return 0;
        }
        let field_control_points_list = self.state.read().control_points_list.clone();
        field_control_points_list.array2().upper_row()
            - field_control_points_list.array2().lower_row()
            + 1
    }
    /// `nb_control_points_list_j`.
    pub fn nb_control_points_list_j(&self) -> i32 {
        if self.state.read().control_points_list.is_null() {
            return 0;
        }
        let field_control_points_list = self.state.read().control_points_list.clone();
        field_control_points_list.array2().upper_col()
            - field_control_points_list.array2().lower_col()
            + 1
    }
}
