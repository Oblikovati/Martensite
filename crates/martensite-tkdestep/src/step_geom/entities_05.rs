#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Bezier curve and rational B spline curve.
    StepGeomBezierCurveAndRationalBSplineCurve, StepGeomBezierCurveAndRationalBSplineCurveState, "StepGeomBezierCurveAndRationalBSplineCurve", StepGeomBSplineCurve, {
        bezier_curve: Handle<StepGeomBezierCurve>,
        rational_b_spline_curve: Handle<StepGeomRationalBSplineCurve>,
    }
}
impl StepGeomBezierCurveAndRationalBSplineCurve {
    step_methods! {
        set set_bezier_curve = bezier_curve: Handle<StepGeomBezierCurve>;
        get bezier_curve = bezier_curve: Handle<StepGeomBezierCurve>;
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
        a_bezier_curve: &Handle<StepGeomBezierCurve>,
        a_rational_b_spline_curve: &Handle<StepGeomRationalBSplineCurve>,
    ) {
        let mut state = self.state.write();
        state.bezier_curve = a_bezier_curve.clone();
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
        self.state.write().bezier_curve = Handle::new(StepGeomBezierCurve::new());
        let field_bezier_curve = self.state.read().bezier_curve.clone();
        field_bezier_curve.init(
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
    }
    /// `set_weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_weights_data(&self, a_weights_data: &Handle<HArray1<f64>>) -> Result<(), Failure> {
        let field_rational_b_spline_curve = self.state.read().rational_b_spline_curve.clone();
        step_live(
            &field_rational_b_spline_curve,
            "StepGeomBezierCurveAndRationalBSplineCurve::SetWeightsData",
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
            "StepGeomBezierCurveAndRationalBSplineCurve::WeightsData",
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
            "StepGeomBezierCurveAndRationalBSplineCurve::WeightsDataValue",
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
            "StepGeomBezierCurveAndRationalBSplineCurve::NbWeightsData",
        )?
        .nb_weights_data()
    }
}

step_entity! {
    /// Bezier surface.
    StepGeomBezierSurface, StepGeomBezierSurfaceState, "StepGeomBezierSurface", StepGeomBSplineSurface, {
    }
}

step_entity! {
    /// Bezier surface and rational B spline surface.
    StepGeomBezierSurfaceAndRationalBSplineSurface, StepGeomBezierSurfaceAndRationalBSplineSurfaceState, "StepGeomBezierSurfaceAndRationalBSplineSurface", StepGeomBSplineSurface, {
        bezier_surface: Handle<StepGeomBezierSurface>,
        rational_b_spline_surface: Handle<StepGeomRationalBSplineSurface>,
    }
}
impl StepGeomBezierSurfaceAndRationalBSplineSurface {
    step_methods! {
        set set_bezier_surface = bezier_surface: Handle<StepGeomBezierSurface>;
        get bezier_surface = bezier_surface: Handle<StepGeomBezierSurface>;
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
        a_bezier_surface: &Handle<StepGeomBezierSurface>,
        a_rational_b_spline_surface: &Handle<StepGeomRationalBSplineSurface>,
    ) {
        let mut state = self.state.write();
        state.bezier_surface = a_bezier_surface.clone();
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
        self.state.write().bezier_surface = Handle::new(StepGeomBezierSurface::new());
        let field_bezier_surface = self.state.read().bezier_surface.clone();
        field_bezier_surface.init(
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
    }
    /// `set_weights_data`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_weights_data(&self, a_weights_data: &Handle<HArray2<f64>>) -> Result<(), Failure> {
        let field_rational_b_spline_surface = self.state.read().rational_b_spline_surface.clone();
        step_live(
            &field_rational_b_spline_surface,
            "StepGeomBezierSurfaceAndRationalBSplineSurface::SetWeightsData",
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
            "StepGeomBezierSurfaceAndRationalBSplineSurface::WeightsData",
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
            "StepGeomBezierSurfaceAndRationalBSplineSurface::WeightsDataValue",
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
            "StepGeomBezierSurfaceAndRationalBSplineSurface::NbWeightsDataI",
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
            "StepGeomBezierSurfaceAndRationalBSplineSurface::NbWeightsDataJ",
        )?
        .nb_weights_data_j()
    }
}

step_entity! {
    /// Boundary curve.
    StepGeomBoundaryCurve, StepGeomBoundaryCurveState, "StepGeomBoundaryCurve", StepGeomCompositeCurveOnSurface, {
    }
}

step_entity! {
    /// Bounded curve.
    StepGeomBoundedCurve, StepGeomBoundedCurveState, "StepGeomBoundedCurve", StepGeomCurve, {
    }
}

step_entity! {
    /// Bounded surface.
    StepGeomBoundedSurface, StepGeomBoundedSurfaceState, "StepGeomBoundedSurface", StepGeomSurface, {
    }
}

step_entity! {
    /// Cartesian transformation operator.
    StepGeomCartesianTransformationOperator, StepGeomCartesianTransformationOperatorState, "StepGeomCartesianTransformationOperator", StepGeomGeometricRepresentationItem, {
        axis1: Handle<StepGeomDirection>,
        axis2: Handle<StepGeomDirection>,
        local_origin: Handle<StepGeomCartesianPoint>,
        scale: f64,
        has_axis1: bool,
        has_axis2: bool,
        has_scale: bool,
    }
}
impl StepGeomCartesianTransformationOperator {
    step_methods! {
        set set_axis1 = axis1 flag has_axis1: Handle<StepGeomDirection>;
        unset un_set_axis1 = axis1 flag has_axis1;
        get axis1 = axis1: Handle<StepGeomDirection>;
        get has_axis1 = has_axis1: bool;
        set set_axis2 = axis2 flag has_axis2: Handle<StepGeomDirection>;
        unset un_set_axis2 = axis2 flag has_axis2;
        get axis2 = axis2: Handle<StepGeomDirection>;
        get has_axis2 = has_axis2: bool;
        set set_local_origin = local_origin: Handle<StepGeomCartesianPoint>;
        get local_origin = local_origin: Handle<StepGeomCartesianPoint>;
        set_copy set_scale = scale flag has_scale: f64;
        clear un_set_scale = has_scale;
        get scale = scale: f64;
        get has_scale = has_scale: bool;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_aaxis1: bool,
        a_axis1: &Handle<StepGeomDirection>,
        has_aaxis2: bool,
        a_axis2: &Handle<StepGeomDirection>,
        a_local_origin: &Handle<StepGeomCartesianPoint>,
        has_ascale: bool,
        a_scale: f64,
    ) {
        let mut state = self.state.write();
        state.has_axis1 = has_aaxis1;
        state.axis1 = a_axis1.clone();
        state.has_axis2 = has_aaxis2;
        state.axis2 = a_axis2.clone();
        state.local_origin = a_local_origin.clone();
        state.has_scale = has_ascale;
        state.scale = a_scale;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Added from StepGeom Rev2 to Rev4
    StepGeomCartesianTransformationOperator2d, StepGeomCartesianTransformationOperator2dState, "StepGeomCartesianTransformationOperator2d", StepGeomCartesianTransformationOperator, {
    }
}

step_entity! {
    /// Cartesian transformation operator3d.
    StepGeomCartesianTransformationOperator3d, StepGeomCartesianTransformationOperator3dState, "StepGeomCartesianTransformationOperator3d", StepGeomCartesianTransformationOperator, {
        axis3: Handle<StepGeomDirection>,
        has_axis3: bool,
    }
}
impl StepGeomCartesianTransformationOperator3d {
    step_methods! {
        set set_axis3 = axis3 flag has_axis3: Handle<StepGeomDirection>;
        unset un_set_axis3 = axis3 flag has_axis3;
        get axis3 = axis3: Handle<StepGeomDirection>;
        get has_axis3 = has_axis3: bool;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        has_aaxis1: bool,
        a_axis1: &Handle<StepGeomDirection>,
        has_aaxis2: bool,
        a_axis2: &Handle<StepGeomDirection>,
        a_local_origin: &Handle<StepGeomCartesianPoint>,
        has_ascale: bool,
        a_scale: f64,
        has_aaxis3: bool,
        a_axis3: &Handle<StepGeomDirection>,
    ) {
        let mut state = self.state.write();
        state.has_axis3 = has_aaxis3;
        state.axis3 = a_axis3.clone();
        drop(state);
        self.parent.init(
            a_name,
            has_aaxis1,
            a_axis1,
            has_aaxis2,
            a_axis2,
            a_local_origin,
            has_ascale,
            a_scale,
        );
    }
}

step_entity! {
    /// Circle.
    StepGeomCircle, StepGeomCircleState, "StepGeomCircle", StepGeomConic, {
        radius: f64,
    }
}
impl StepGeomCircle {
    step_methods! {
        set_copy set_radius = radius: f64;
        get radius = radius: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &StepGeomAxis2Placement,
        a_radius: f64,
    ) {
        self.state.write().radius = a_radius;
        self.parent.init(a_name, a_position);
    }
}
