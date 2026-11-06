#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Rational B spline curve.
    StepGeomRationalBSplineCurve, StepGeomRationalBSplineCurveState, "StepGeomRationalBSplineCurve", StepGeomBSplineCurve, {
        weights_data: Handle<HArray1<f64>>,
    }
}
impl StepGeomRationalBSplineCurve {
    step_methods! {
        set set_weights_data = weights_data: Handle<HArray1<f64>>;
        get weights_data = weights_data: Handle<HArray1<f64>>;
        item weights_data_value = weights_data: f64;
        len_checked nb_weights_data = weights_data;
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
        a_weights_data: &Handle<HArray1<f64>>,
    ) {
        self.state.write().weights_data = a_weights_data.clone();
        self.parent.init(
            a_name,
            a_degree,
            a_control_points_list,
            a_curve_form,
            a_closed_curve,
            a_self_intersect,
        );
    }
}

step_entity! {
    /// Rational B spline surface.
    StepGeomRationalBSplineSurface, StepGeomRationalBSplineSurfaceState, "StepGeomRationalBSplineSurface", StepGeomBSplineSurface, {
        weights_data: Handle<HArray2<f64>>,
    }
}
impl StepGeomRationalBSplineSurface {
    step_methods! {
        set set_weights_data = weights_data: Handle<HArray2<f64>>;
        get weights_data = weights_data: Handle<HArray2<f64>>;
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
        a_weights_data: &Handle<HArray2<f64>>,
    ) {
        self.state.write().weights_data = a_weights_data.clone();
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
    /// `weights_data_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn weights_data_value(&self, num1: i32, num2: i32) -> Result<f64, Failure> {
        let field_weights_data = self.state.read().weights_data.clone();
        step_array2_item(
            &field_weights_data,
            num1,
            num2,
            "StepGeomRationalBSplineSurface::WeightsDataValue",
        )
    }
    /// `nb_weights_data_i`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_weights_data_i(&self) -> Result<i32, Failure> {
        let field_weights_data = self.state.read().weights_data.clone();
        Ok(step_live(
            &field_weights_data,
            "StepGeomRationalBSplineSurface::NbWeightsDataI",
        )?
        .array2()
        .upper_row()
            - step_live(
                &field_weights_data,
                "StepGeomRationalBSplineSurface::NbWeightsDataI",
            )?
            .array2()
            .lower_row()
            + 1)
    }
    /// `nb_weights_data_j`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_weights_data_j(&self) -> Result<i32, Failure> {
        let field_weights_data = self.state.read().weights_data.clone();
        Ok(step_live(
            &field_weights_data,
            "StepGeomRationalBSplineSurface::NbWeightsDataJ",
        )?
        .array2()
        .upper_col()
            - step_live(
                &field_weights_data,
                "StepGeomRationalBSplineSurface::NbWeightsDataJ",
            )?
            .array2()
            .lower_col()
            + 1)
    }
}

step_entity! {
    /// Rectangular composite surface.
    StepGeomRectangularCompositeSurface, StepGeomRectangularCompositeSurfaceState, "StepGeomRectangularCompositeSurface", StepGeomBoundedSurface, {
        segments: Handle<HArray2<Handle<StepGeomSurfacePatch>>>,
    }
}
impl StepGeomRectangularCompositeSurface {
    step_methods! {
        set set_segments = segments: Handle<HArray2<Handle<StepGeomSurfacePatch>>>;
        get segments = segments: Handle<HArray2<Handle<StepGeomSurfacePatch>>>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_segments: &Handle<HArray2<Handle<StepGeomSurfacePatch>>>,
    ) {
        self.state.write().segments = a_segments.clone();
        self.parent.parent.parent.parent.init(a_name);
    }
    /// `segments_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn segments_value(
        &self,
        num1: i32,
        num2: i32,
    ) -> Result<Handle<StepGeomSurfacePatch>, Failure> {
        let field_segments = self.state.read().segments.clone();
        step_array2_item(
            &field_segments,
            num1,
            num2,
            "StepGeomRectangularCompositeSurface::SegmentsValue",
        )
    }
    /// `nb_segments_i`.
    pub fn nb_segments_i(&self) -> i32 {
        if self.state.read().segments.is_null() {
            return 0;
        }
        let field_segments = self.state.read().segments.clone();
        field_segments.array2().upper_row() - field_segments.array2().lower_row() + 1
    }
    /// `nb_segments_j`.
    pub fn nb_segments_j(&self) -> i32 {
        if self.state.read().segments.is_null() {
            return 0;
        }
        let field_segments = self.state.read().segments.clone();
        field_segments.array2().upper_col() - field_segments.array2().lower_col() + 1
    }
}

step_entity! {
    /// Rectangular trimmed surface.
    StepGeomRectangularTrimmedSurface, StepGeomRectangularTrimmedSurfaceState, "StepGeomRectangularTrimmedSurface", StepGeomBoundedSurface, {
        basis_surface: Handle<StepGeomSurface>,
        u1: f64,
        u2: f64,
        v1: f64,
        v2: f64,
        usense: bool,
        vsense: bool,
    }
}
impl StepGeomRectangularTrimmedSurface {
    step_methods! {
        set set_basis_surface = basis_surface: Handle<StepGeomSurface>;
        get basis_surface = basis_surface: Handle<StepGeomSurface>;
        set_copy set_u1 = u1: f64;
        get u1 = u1: f64;
        set_copy set_u2 = u2: f64;
        get u2 = u2: f64;
        set_copy set_v1 = v1: f64;
        get v1 = v1: f64;
        set_copy set_v2 = v2: f64;
        get v2 = v2: f64;
        set_copy set_usense = usense: bool;
        get usense = usense: bool;
        set_copy set_vsense = vsense: bool;
        get vsense = vsense: bool;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_surface: &Handle<StepGeomSurface>,
        a_u1: f64,
        a_u2: f64,
        a_v1: f64,
        a_v2: f64,
        a_usense: bool,
        a_vsense: bool,
    ) {
        let mut state = self.state.write();
        state.basis_surface = a_basis_surface.clone();
        state.u1 = a_u1;
        state.u2 = a_u2;
        state.v1 = a_v1;
        state.v2 = a_v2;
        state.usense = a_usense;
        state.vsense = a_vsense;
        drop(state);
        self.parent.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Reparametrised composite curve segment.
    StepGeomReparametrisedCompositeCurveSegment, StepGeomReparametrisedCompositeCurveSegmentState, "StepGeomReparametrisedCompositeCurveSegment", StepGeomCompositeCurveSegment, {
        param_length: f64,
    }
}
impl StepGeomReparametrisedCompositeCurveSegment {
    step_methods! {
        set_copy set_param_length = param_length: f64;
        get param_length = param_length: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_transition: StepGeomTransitionCode,
        a_same_sense: bool,
        a_parent_curve: &Handle<StepGeomCurve>,
        a_param_length: f64,
    ) {
        self.state.write().param_length = a_param_length;
        self.parent.init(a_transition, a_same_sense, a_parent_curve);
    }
}

step_entity! {
    /// Seam curve.
    StepGeomSeamCurve, StepGeomSeamCurveState, "StepGeomSeamCurve", StepGeomSurfaceCurve, {
    }
}

step_entity! {
    /// Spherical surface.
    StepGeomSphericalSurface, StepGeomSphericalSurfaceState, "StepGeomSphericalSurface", StepGeomElementarySurface, {
        radius: f64,
    }
}
impl StepGeomSphericalSurface {
    step_methods! {
        set_copy set_radius = radius: f64;
        get radius = radius: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis2Placement3d>,
        a_radius: f64,
    ) {
        self.state.write().radius = a_radius;
        self.parent.init(a_name, a_position);
    }
}

step_entity! {
    /// Representation of STEP entity SuParameters
    StepGeomSuParameters, StepGeomSuParametersState, "StepGeomSuParameters", StepGeomGeometricRepresentationItem, {
        my_a: f64,
        my_alpha: f64,
        my_b: f64,
        my_beta: f64,
        my_c: f64,
        my_gamma: f64,
    }
}
impl StepGeomSuParameters {
    step_methods! {
        get a = my_a: f64;
        set_copy set_a = my_a: f64;
        get alpha = my_alpha: f64;
        set_copy set_alpha = my_alpha: f64;
        get b = my_b: f64;
        set_copy set_b = my_b: f64;
        get beta = my_beta: f64;
        set_copy set_beta = my_beta: f64;
        get c = my_c: f64;
        set_copy set_c = my_c: f64;
        get gamma = my_gamma: f64;
        set_copy set_gamma = my_gamma: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_a: f64,
        the_alpha: f64,
        the_b: f64,
        the_beta: f64,
        the_c: f64,
        the_gamma: f64,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_a = the_a;
        state.my_alpha = the_alpha;
        state.my_b = the_b;
        state.my_beta = the_beta;
        state.my_c = the_c;
        state.my_gamma = the_gamma;
    }
}

step_entity! {
    /// Surface.
    StepGeomSurface, StepGeomSurfaceState, "StepGeomSurface", StepGeomGeometricRepresentationItem, {
    }
}
