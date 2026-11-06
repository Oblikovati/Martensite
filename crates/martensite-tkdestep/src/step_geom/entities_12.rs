#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Uniform curve and rational B spline curve.
    StepGeomUniformCurveAndRationalBSplineCurve, StepGeomUniformCurveAndRationalBSplineCurveState, "StepGeomUniformCurveAndRationalBSplineCurve", StepGeomBSplineCurve, {
        uniform_curve: Handle<StepGeomUniformCurve>,
        rational_b_spline_curve: Handle<StepGeomRationalBSplineCurve>,
    }
}
impl StepGeomUniformCurveAndRationalBSplineCurve {
    step_methods! {
        set set_uniform_curve = uniform_curve: Handle<StepGeomUniformCurve>;
        get uniform_curve = uniform_curve: Handle<StepGeomUniformCurve>;
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
        a_uniform_curve: &Handle<StepGeomUniformCurve>,
        a_rational_b_spline_curve: &Handle<StepGeomRationalBSplineCurve>,
    ) {
        let mut state = self.state.write();
        state.uniform_curve = a_uniform_curve.clone();
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
        self.state.write().uniform_curve = Handle::new(StepGeomUniformCurve::new());
        let field_uniform_curve = self.state.read().uniform_curve.clone();
        field_uniform_curve.init(
            a_name,
            a_degree,
            a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
        );
    }
    /// `set_weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_weights_data(&self, a_weights_data: &Handle<HArray1<f64>>) -> Result<(), Failure> {
        let field_rational_b_spline_curve = self.state.read().rational_b_spline_curve.clone();
        step_live(
            &field_rational_b_spline_curve,
            "StepGeomUniformCurveAndRationalBSplineCurve::SetWeightsData",
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
            "StepGeomUniformCurveAndRationalBSplineCurve::WeightsData",
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
            "StepGeomUniformCurveAndRationalBSplineCurve::WeightsDataValue",
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
            "StepGeomUniformCurveAndRationalBSplineCurve::NbWeightsData",
        )?
        .nb_weights_data()
    }
}

step_entity! {
    /// Uniform surface.
    StepGeomUniformSurface, StepGeomUniformSurfaceState, "StepGeomUniformSurface", StepGeomBSplineSurface, {
    }
}

step_entity! {
    /// Uniform surface and rational B spline surface.
    StepGeomUniformSurfaceAndRationalBSplineSurface, StepGeomUniformSurfaceAndRationalBSplineSurfaceState, "StepGeomUniformSurfaceAndRationalBSplineSurface", StepGeomBSplineSurface, {
        uniform_surface: Handle<StepGeomUniformSurface>,
        rational_b_spline_surface: Handle<StepGeomRationalBSplineSurface>,
    }
}
impl StepGeomUniformSurfaceAndRationalBSplineSurface {
    step_methods! {
        set set_uniform_surface = uniform_surface: Handle<StepGeomUniformSurface>;
        get uniform_surface = uniform_surface: Handle<StepGeomUniformSurface>;
        set set_rational_b_spline_surface = rational_b_spline_surface: Handle<StepGeomRationalBSplineSurface>;
        get rational_b_spline_surface = rational_b_spline_surface: Handle<StepGeomRationalBSplineSurface>;
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
        a_uniform_surface: &Handle<StepGeomUniformSurface>,
        a_rational_b_spline_surface: &Handle<StepGeomRationalBSplineSurface>,
    ) {
        let mut state = self.state.write();
        state.uniform_surface = a_uniform_surface.clone();
        state.rational_b_spline_surface = a_rational_b_spline_surface.clone();
        drop(state);
        self.parent.init(
            a_name,
            a_u_degree,
            a_v_degree,
            a_control_points_list,
            a_surface_form,
            a_u_closed,
            a_v_closed,
            a_self_intersect,
        );
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init_fields(
        &self,
        a_name: &Handle<HAsciiString>,
        a_u_degree: i32,
        a_v_degree: i32,
        a_control_points_list: &Handle<HArray2<Handle<StepGeomCartesianPoint>>>,
        a_surface_form: StepGeomBSplineSurfaceForm,
        a_u_closed: StepDataLogical,
        a_v_closed: StepDataLogical,
        a_self_intersect: StepDataLogical,
        a_weights_data: &Handle<HArray2<f64>>,
    ) {
        self.parent.init(
            a_name,
            a_u_degree,
            a_v_degree,
            a_control_points_list,
            a_surface_form,
            a_u_closed,
            a_v_closed,
            a_self_intersect,
        );
        self.state.write().rational_b_spline_surface =
            Handle::new(StepGeomRationalBSplineSurface::new());
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        field_rational_b_spline_surface.init(
            a_name,
            a_u_degree,
            a_v_degree,
            a_control_points_list,
            a_surface_form,
            a_u_closed,
            a_v_closed,
            a_self_intersect,
            a_weights_data,
        );
        self.state.write().uniform_surface = Handle::new(StepGeomUniformSurface::new());
        let field_uniform_surface = self.state.read().uniform_surface.clone();
        field_uniform_surface.init(
            a_name,
            a_u_degree,
            a_v_degree,
            a_control_points_list,
            a_surface_form,
            a_u_closed,
            a_v_closed,
            a_self_intersect,
        );
    }
    /// `set_weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_weights_data(&self, a_weights_data: &Handle<HArray2<f64>>) -> Result<(), Failure> {
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        step_live(
            &field_rational_b_spline_surface,
            "StepGeomUniformSurfaceAndRationalBSplineSurface::SetWeightsData",
        )?
        .set_weights_data(a_weights_data);
        Ok(())
    }
    /// `weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn weights_data(&self) -> Result<Handle<HArray2<f64>>, Failure> {
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        Ok(step_live(
            &field_rational_b_spline_surface,
            "StepGeomUniformSurfaceAndRationalBSplineSurface::WeightsData",
        )?
        .weights_data())
    }
    /// `weights_data_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn weights_data_value(&self, num1: i32, num2: i32) -> Result<f64, Failure> {
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        step_live(
            &field_rational_b_spline_surface,
            "StepGeomUniformSurfaceAndRationalBSplineSurface::WeightsDataValue",
        )?
        .weights_data_value(num1, num2)
    }
    /// `nb_weights_data_i`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_weights_data_i(&self) -> Result<i32, Failure> {
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        step_live(
            &field_rational_b_spline_surface,
            "StepGeomUniformSurfaceAndRationalBSplineSurface::NbWeightsDataI",
        )?
        .nb_weights_data_i()
    }
    /// `nb_weights_data_j`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_weights_data_j(&self) -> Result<i32, Failure> {
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        step_live(
            &field_rational_b_spline_surface,
            "StepGeomUniformSurfaceAndRationalBSplineSurface::NbWeightsDataJ",
        )?
        .nb_weights_data_j()
    }
}

step_entity! {
    /// Vector.
    StepGeomVector, StepGeomVectorState, "StepGeomVector", StepGeomGeometricRepresentationItem, {
        orientation: Handle<StepGeomDirection>,
        magnitude: f64,
    }
}
impl StepGeomVector {
    step_methods! {
        set set_orientation = orientation: Handle<StepGeomDirection>;
        get orientation = orientation: Handle<StepGeomDirection>;
        set_copy set_magnitude = magnitude: f64;
        get magnitude = magnitude: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_orientation: &Handle<StepGeomDirection>,
        a_magnitude: f64,
    ) {
        let mut state = self.state.write();
        state.orientation = a_orientation.clone();
        state.magnitude = a_magnitude;
        drop(state);
        self.parent.parent.init(a_name);
    }
}
