#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// B spline surface with knots and rational B spline surface.
    StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface, StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurfaceState, "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface", StepGeomBSplineSurface, {
        b_spline_surface_with_knots: Handle<StepGeomBSplineSurfaceWithKnots>,
        rational_b_spline_surface: Handle<StepGeomRationalBSplineSurface>,
    }
}
impl StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface {
    step_methods! {
        set set_b_spline_surface_with_knots = b_spline_surface_with_knots: Handle<StepGeomBSplineSurfaceWithKnots>;
        get b_spline_surface_with_knots = b_spline_surface_with_knots: Handle<StepGeomBSplineSurfaceWithKnots>;
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
        a_b_spline_surface_with_knots: &Handle<StepGeomBSplineSurfaceWithKnots>,
        a_rational_b_spline_surface: &Handle<StepGeomRationalBSplineSurface>,
    ) {
        let mut state = self.state.write();
        state.b_spline_surface_with_knots = a_b_spline_surface_with_knots.clone();
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
        a_u_multiplicities: &Handle<HArray1<i32>>,
        a_v_multiplicities: &Handle<HArray1<i32>>,
        a_u_knots: &Handle<HArray1<f64>>,
        a_v_knots: &Handle<HArray1<f64>>,
        a_knot_spec: StepGeomKnotType,
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
        self.state.write().b_spline_surface_with_knots =
            Handle::new(StepGeomBSplineSurfaceWithKnots::new());
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        field_b_spline_surface_with_knots.init(
            a_name,
            a_u_degree,
            a_v_degree,
            a_control_points_list,
            a_surface_form,
            a_u_closed,
            a_v_closed,
            a_self_intersect,
            a_u_multiplicities,
            a_v_multiplicities,
            a_u_knots,
            a_v_knots,
            a_knot_spec,
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
    }
    /// `set_u_multiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_u_multiplicities(
        &self,
        a_u_multiplicities: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::SetUMultiplicities",
        )?
        .set_u_multiplicities(a_u_multiplicities);
        Ok(())
    }
    /// `UMultiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn u_multiplicities(&self) -> Result<Handle<HArray1<i32>>, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        Ok(step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::UMultiplicities",
        )?
        .u_multiplicities())
    }
    /// `UMultiplicitiesValue`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn u_multiplicities_value(&self, num: i32) -> Result<i32, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::UMultiplicitiesValue",
        )?
        .u_multiplicities_value(num)
    }
    /// `nb_u_multiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_u_multiplicities(&self) -> Result<i32, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::NbUMultiplicities",
        )?
        .nb_u_multiplicities()
    }
    /// `set_v_multiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_v_multiplicities(
        &self,
        a_v_multiplicities: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::SetVMultiplicities",
        )?
        .set_v_multiplicities(a_v_multiplicities);
        Ok(())
    }
    /// `VMultiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn v_multiplicities(&self) -> Result<Handle<HArray1<i32>>, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        Ok(step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::VMultiplicities",
        )?
        .v_multiplicities())
    }
    /// `VMultiplicitiesValue`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn v_multiplicities_value(&self, num: i32) -> Result<i32, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::VMultiplicitiesValue",
        )?
        .v_multiplicities_value(num)
    }
    /// `nb_v_multiplicities`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_v_multiplicities(&self) -> Result<i32, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::NbVMultiplicities",
        )?
        .nb_v_multiplicities()
    }
    /// `set_u_knots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_u_knots(&self, a_u_knots: &Handle<HArray1<f64>>) -> Result<(), Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::SetUKnots",
        )?
        .set_u_knots(a_u_knots);
        Ok(())
    }
    /// `UKnots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn u_knots(&self) -> Result<Handle<HArray1<f64>>, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        Ok(step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::UKnots",
        )?
        .u_knots())
    }
    /// `UKnotsValue`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn u_knots_value(&self, num: i32) -> Result<f64, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::UKnotsValue",
        )?
        .u_knots_value(num)
    }
    /// `nb_u_knots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_u_knots(&self) -> Result<i32, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::NbUKnots",
        )?
        .nb_u_knots()
    }
    /// `set_v_knots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_v_knots(&self, a_v_knots: &Handle<HArray1<f64>>) -> Result<(), Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::SetVKnots",
        )?
        .set_v_knots(a_v_knots);
        Ok(())
    }
    /// `VKnots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn v_knots(&self) -> Result<Handle<HArray1<f64>>, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        Ok(step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::VKnots",
        )?
        .v_knots())
    }
    /// `VKnotsValue`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn v_knots_value(&self, num: i32) -> Result<f64, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::VKnotsValue",
        )?
        .v_knots_value(num)
    }
    /// `nb_v_knots`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_v_knots(&self) -> Result<i32, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::NbVKnots",
        )?
        .nb_v_knots()
    }
    /// `set_knot_spec`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_knot_spec(&self, a_knot_spec: StepGeomKnotType) -> Result<(), Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::SetKnotSpec",
        )?
        .set_knot_spec(a_knot_spec);
        Ok(())
    }
    /// `knot_spec`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn knot_spec(&self) -> Result<StepGeomKnotType, Failure> {
        let field_b_spline_surface_with_knots =
            self.state.read().b_spline_surface_with_knots.clone();
        Ok(step_live(
            &field_b_spline_surface_with_knots,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::KnotSpec",
        )?
        .knot_spec())
    }
    /// `set_weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_weights_data(&self, a_weights_data: &Handle<HArray2<f64>>) -> Result<(), Failure> {
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        step_live(
            &field_rational_b_spline_surface,
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::SetWeightsData",
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
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::WeightsData",
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
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::WeightsDataValue",
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
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::NbWeightsDataI",
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
            "StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface::NbWeightsDataJ",
        )?
        .nb_weights_data_j()
    }
}

step_entity! {
    /// Bezier curve.
    StepGeomBezierCurve, StepGeomBezierCurveState, "StepGeomBezierCurve", StepGeomBSplineCurve, {
    }
}
