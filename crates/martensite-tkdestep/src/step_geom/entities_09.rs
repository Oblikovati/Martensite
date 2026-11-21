#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Point on curve.
    StepGeomPointOnCurve, StepGeomPointOnCurveState, "StepGeomPointOnCurve", StepGeomPoint, {
        basis_curve: Handle<dyn crate::StepEntityBase<StepGeomCurve>>,
        point_parameter: f64,
    }, bases: [StepGeomPoint, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem]
}
impl StepGeomPointOnCurve {
    step_methods! {
        set set_basis_curve = basis_curve: Handle<dyn crate::StepEntityBase<StepGeomCurve>>;
        get basis_curve = basis_curve: Handle<dyn crate::StepEntityBase<StepGeomCurve>>;
        set_copy set_point_parameter = point_parameter: f64;
        get point_parameter = point_parameter: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_curve: &Handle<dyn crate::StepEntityBase<StepGeomCurve>>,
        a_point_parameter: f64,
    ) {
        let mut state = self.state.write();
        state.basis_curve = a_basis_curve.clone();
        state.point_parameter = a_point_parameter;
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Point on surface.
    StepGeomPointOnSurface, StepGeomPointOnSurfaceState, "StepGeomPointOnSurface", StepGeomPoint, {
        basis_surface: Handle<dyn crate::StepEntityBase<StepGeomSurface>>,
        point_parameter_u: f64,
        point_parameter_v: f64,
    }, bases: [StepGeomPoint, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem]
}
impl StepGeomPointOnSurface {
    step_methods! {
        set set_basis_surface = basis_surface: Handle<dyn crate::StepEntityBase<StepGeomSurface>>;
        get basis_surface = basis_surface: Handle<dyn crate::StepEntityBase<StepGeomSurface>>;
        set_copy set_point_parameter_u = point_parameter_u: f64;
        get point_parameter_u = point_parameter_u: f64;
        set_copy set_point_parameter_v = point_parameter_v: f64;
        get point_parameter_v = point_parameter_v: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_surface: &Handle<dyn crate::StepEntityBase<StepGeomSurface>>,
        a_point_parameter_u: f64,
        a_point_parameter_v: f64,
    ) {
        let mut state = self.state.write();
        state.basis_surface = a_basis_surface.clone();
        state.point_parameter_u = a_point_parameter_u;
        state.point_parameter_v = a_point_parameter_v;
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Point replica.
    StepGeomPointReplica, StepGeomPointReplicaState, "StepGeomPointReplica", StepGeomPoint, {
        parent_pt: Handle<dyn crate::StepEntityBase<StepGeomPoint>>,
        transformation: Handle<StepGeomCartesianTransformationOperator>,
    }, bases: [StepGeomPoint, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem]
}
impl StepGeomPointReplica {
    step_methods! {
        set set_parent_pt = parent_pt: Handle<dyn crate::StepEntityBase<StepGeomPoint>>;
        get parent_pt = parent_pt: Handle<dyn crate::StepEntityBase<StepGeomPoint>>;
        set set_transformation = transformation: Handle<StepGeomCartesianTransformationOperator>;
        get transformation = transformation: Handle<StepGeomCartesianTransformationOperator>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_parent_pt: &Handle<dyn crate::StepEntityBase<StepGeomPoint>>,
        a_transformation: &Handle<StepGeomCartesianTransformationOperator>,
    ) {
        let mut state = self.state.write();
        state.parent_pt = a_parent_pt.clone();
        state.transformation = a_transformation.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Polyline.
    StepGeomPolyline, StepGeomPolylineState, "StepGeomPolyline", StepGeomBoundedCurve, {
        points: Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
    }, bases: [crate::step_geom::StepGeomCurve, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem, crate::step_geom::StepGeomBoundedCurve]
}
impl StepGeomPolyline {
    step_methods! {
        set set_points = points: Handle<HArray1<Handle<StepGeomCartesianPoint>>>;
        get points = points: Handle<HArray1<Handle<StepGeomCartesianPoint>>>;
        item points_value = points: Handle<StepGeomCartesianPoint>;
        len nb_points = points;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_points: &Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
    ) {
        self.state.write().points = a_points.clone();
        self.parent.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Quasi uniform curve.
    StepGeomQuasiUniformCurve, StepGeomQuasiUniformCurveState, "StepGeomQuasiUniformCurve", StepGeomBSplineCurve, {
    }, bases: [crate::step_geom::StepGeomCurve, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem, crate::step_geom::StepGeomBSplineCurve, crate::step_geom::StepGeomBoundedCurve]
}

step_entity! {
    /// Quasi uniform curve and rational B spline curve.
    StepGeomQuasiUniformCurveAndRationalBSplineCurve, StepGeomQuasiUniformCurveAndRationalBSplineCurveState, "StepGeomQuasiUniformCurveAndRationalBSplineCurve", StepGeomBSplineCurve, {
        quasi_uniform_curve: Handle<StepGeomQuasiUniformCurve>,
        rational_b_spline_curve: Handle<StepGeomRationalBSplineCurve>,
    }, bases: [crate::step_geom::StepGeomCurve, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem, crate::step_geom::StepGeomBSplineCurve, crate::step_geom::StepGeomBoundedCurve]
}
impl StepGeomQuasiUniformCurveAndRationalBSplineCurve {
    step_methods! {
        set set_quasi_uniform_curve = quasi_uniform_curve: Handle<StepGeomQuasiUniformCurve>;
        get quasi_uniform_curve = quasi_uniform_curve: Handle<StepGeomQuasiUniformCurve>;
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
        a_quasi_uniform_curve: &Handle<StepGeomQuasiUniformCurve>,
        a_rational_b_spline_curve: &Handle<StepGeomRationalBSplineCurve>,
    ) {
        let mut state = self.state.write();
        state.quasi_uniform_curve = a_quasi_uniform_curve.clone();
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
        self.state.write().quasi_uniform_curve = Handle::new(StepGeomQuasiUniformCurve::new());
        let field_quasi_uniform_curve = self.state.read().quasi_uniform_curve.clone();
        field_quasi_uniform_curve.init(
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
            "StepGeomQuasiUniformCurveAndRationalBSplineCurve::SetWeightsData",
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
            "StepGeomQuasiUniformCurveAndRationalBSplineCurve::WeightsData",
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
            "StepGeomQuasiUniformCurveAndRationalBSplineCurve::WeightsDataValue",
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
            "StepGeomQuasiUniformCurveAndRationalBSplineCurve::NbWeightsData",
        )?
        .nb_weights_data()
    }
}

step_entity! {
    /// Quasi uniform surface.
    StepGeomQuasiUniformSurface, StepGeomQuasiUniformSurfaceState, "StepGeomQuasiUniformSurface", StepGeomBSplineSurface, {
    }, bases: [crate::step_geom::StepGeomSurface, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem, crate::step_geom::StepGeomBSplineSurface, crate::step_geom::StepGeomBoundedSurface]
}

step_entity! {
    /// Quasi uniform surface and rational B spline surface.
    StepGeomQuasiUniformSurfaceAndRationalBSplineSurface, StepGeomQuasiUniformSurfaceAndRationalBSplineSurfaceState, "StepGeomQuasiUniformSurfaceAndRationalBSplineSurface", StepGeomBSplineSurface, {
        quasi_uniform_surface: Handle<StepGeomQuasiUniformSurface>,
        rational_b_spline_surface: Handle<StepGeomRationalBSplineSurface>,
    }, bases: [crate::step_geom::StepGeomSurface, crate::step_geom::StepGeomGeometricRepresentationItem, crate::step_repr::StepReprRepresentationItem, crate::step_geom::StepGeomBSplineSurface, crate::step_geom::StepGeomBoundedSurface]
}
impl StepGeomQuasiUniformSurfaceAndRationalBSplineSurface {
    step_methods! {
        set set_quasi_uniform_surface = quasi_uniform_surface: Handle<StepGeomQuasiUniformSurface>;
        get quasi_uniform_surface = quasi_uniform_surface: Handle<StepGeomQuasiUniformSurface>;
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
        a_quasi_uniform_surface: &Handle<StepGeomQuasiUniformSurface>,
        a_rational_b_spline_surface: &Handle<StepGeomRationalBSplineSurface>,
    ) {
        let mut state = self.state.write();
        state.quasi_uniform_surface = a_quasi_uniform_surface.clone();
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
        self.state.write().quasi_uniform_surface = Handle::new(StepGeomQuasiUniformSurface::new());
        let field_quasi_uniform_surface = self.state.read().quasi_uniform_surface.clone();
        field_quasi_uniform_surface.init(
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
            "StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::SetWeightsData",
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
            "StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::WeightsData",
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
            "StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::WeightsDataValue",
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
            "StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::NbWeightsDataI",
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
            "StepGeomQuasiUniformSurfaceAndRationalBSplineSurface::NbWeightsDataJ",
        )?
        .nb_weights_data_j()
    }
}
