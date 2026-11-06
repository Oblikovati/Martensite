#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

/// B spline curve form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepGeomBSplineCurveForm {
    /// Polyline form.
    #[default]
    BscfPolylineForm,
    /// Circular arc.
    BscfCircularArc,
    /// Elliptic arc.
    BscfEllipticArc,
    /// Parabolic arc.
    BscfParabolicArc,
    /// Hyperbolic arc.
    BscfHyperbolicArc,
    /// Unspecified.
    BscfUnspecified,
}

/// B spline surface form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepGeomBSplineSurfaceForm {
    /// Plane surf.
    #[default]
    BssfPlaneSurf,
    /// Cylindrical surf.
    BssfCylindricalSurf,
    /// Conical surf.
    BssfConicalSurf,
    /// Spherical surf.
    BssfSphericalSurf,
    /// Toroidal surf.
    BssfToroidalSurf,
    /// Surf of revolution.
    BssfSurfOfRevolution,
    /// Ruled surf.
    BssfRuledSurf,
    /// Generalised cone.
    BssfGeneralisedCone,
    /// Quadric surf.
    BssfQuadricSurf,
    /// Surf of linear extrusion.
    BssfSurfOfLinearExtrusion,
    /// Unspecified.
    BssfUnspecified,
}

/// Knot type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepGeomKnotType {
    /// Uniform knots.
    #[default]
    KtUniformKnots,
    /// Unspecified.
    KtUnspecified,
    /// Quasi uniform knots.
    KtQuasiUniformKnots,
    /// Piecewise bezier knots.
    KtPiecewiseBezierKnots,
}

/// Preferred surface curve representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepGeomPreferredSurfaceCurveRepresentation {
    /// Curve3d.
    #[default]
    PscrCurve3d,
    /// Pcurve S 1.
    PscrPcurveS1,
    /// Pcurve S 2.
    PscrPcurveS2,
}

/// Transition code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepGeomTransitionCode {
    /// Discontinuous.
    #[default]
    TcDiscontinuous,
    /// Continuous.
    TcContinuous,
    /// Cont same gradient.
    TcContSameGradient,
    /// Cont same gradient same curvature.
    TcContSameGradientSameCurvature,
}

/// Trimming preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepGeomTrimmingPreference {
    /// Cartesian.
    #[default]
    TpCartesian,
    /// Parameter.
    TpParameter,
    /// Unspecified.
    TpUnspecified,
}

step_select! {
    /// Axis2 placement.
    StepGeomAxis2Placement, [StepGeomAxis2Placement2d, StepGeomAxis2Placement3d], {
        axis2_placement2d: StepGeomAxis2Placement2d,
        axis2_placement3d: StepGeomAxis2Placement3d,
    }
}

step_select! {
    /// Pcurve or surface.
    StepGeomPcurveOrSurface, [StepGeomPcurve, StepGeomSurface], {
        pcurve: StepGeomPcurve,
        surface: StepGeomSurface,
    }
}

step_select! {
    /// Representation of STEP SELECT type SurfaceBoundary
    StepGeomSurfaceBoundary, [StepGeomBoundaryCurve, StepGeomDegeneratePcurve], {
        boundary_curve: StepGeomBoundaryCurve,
        degenerate_pcurve: StepGeomDegeneratePcurve,
    }
}

step_entity! {
    /// Axis1 placement.
    StepGeomAxis1Placement, StepGeomAxis1PlacementState, "StepGeomAxis1Placement", StepGeomPlacement, {
        axis: Handle<StepGeomDirection>,
        has_axis: bool,
    }
}
impl StepGeomAxis1Placement {
    step_methods! {
        set set_axis = axis flag has_axis: Handle<StepGeomDirection>;
        unset un_set_axis = axis flag has_axis;
        get axis = axis: Handle<StepGeomDirection>;
        get has_axis = has_axis: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_location: &Handle<StepGeomCartesianPoint>,
        has_aaxis: bool,
        a_axis: &Handle<StepGeomDirection>,
    ) {
        let mut state = self.state.write();
        state.has_axis = has_aaxis;
        state.axis = a_axis.clone();
        drop(state);
        self.parent.init(a_name, a_location);
    }
}

step_entity! {
    /// Axis2 placement2d.
    StepGeomAxis2Placement2d, StepGeomAxis2Placement2dState, "StepGeomAxis2Placement2d", StepGeomPlacement, {
        ref_direction: Handle<StepGeomDirection>,
        has_ref_direction: bool,
    }
}
impl StepGeomAxis2Placement2d {
    step_methods! {
        set set_ref_direction = ref_direction flag has_ref_direction: Handle<StepGeomDirection>;
        unset un_set_ref_direction = ref_direction flag has_ref_direction;
        get ref_direction = ref_direction: Handle<StepGeomDirection>;
        get has_ref_direction = has_ref_direction: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_location: &Handle<StepGeomCartesianPoint>,
        has_aref_direction: bool,
        a_ref_direction: &Handle<StepGeomDirection>,
    ) {
        let mut state = self.state.write();
        state.has_ref_direction = has_aref_direction;
        state.ref_direction = a_ref_direction.clone();
        drop(state);
        self.parent.init(a_name, a_location);
    }
}

step_entity! {
    /// Axis2 placement3d.
    StepGeomAxis2Placement3d, StepGeomAxis2Placement3dState, "StepGeomAxis2Placement3d", StepGeomPlacement, {
        axis: Handle<StepGeomDirection>,
        ref_direction: Handle<StepGeomDirection>,
        has_axis: bool,
        has_ref_direction: bool,
    }
}
impl StepGeomAxis2Placement3d {
    step_methods! {
        set set_axis = axis flag has_axis: Handle<StepGeomDirection>;
        unset un_set_axis = axis flag has_axis;
        get axis = axis: Handle<StepGeomDirection>;
        get has_axis = has_axis: bool;
        set set_ref_direction = ref_direction flag has_ref_direction: Handle<StepGeomDirection>;
        unset un_set_ref_direction = ref_direction flag has_ref_direction;
        get ref_direction = ref_direction: Handle<StepGeomDirection>;
        get has_ref_direction = has_ref_direction: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_location: &Handle<StepGeomCartesianPoint>,
        has_aaxis: bool,
        a_axis: &Handle<StepGeomDirection>,
        has_aref_direction: bool,
        a_ref_direction: &Handle<StepGeomDirection>,
    ) {
        let mut state = self.state.write();
        state.has_axis = has_aaxis;
        state.axis = a_axis.clone();
        state.has_ref_direction = has_aref_direction;
        state.ref_direction = a_ref_direction.clone();
        drop(state);
        self.parent.init(a_name, a_location);
    }
}

step_entity! {
    /// B spline curve.
    StepGeomBSplineCurve, StepGeomBSplineCurveState, "StepGeomBSplineCurve", StepGeomBoundedCurve, {
        degree: i32,
        control_points_list: Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
        curve_form: StepGeomBSplineCurveForm,
        closed_curve: StepDataLogical,
        self_intersect: StepDataLogical,
    }
}
impl StepGeomBSplineCurve {
    step_methods! {
        set_copy set_degree = degree: i32;
        get degree = degree: i32;
        set set_control_points_list = control_points_list: Handle<HArray1<Handle<StepGeomCartesianPoint>>>;
        get control_points_list = control_points_list: Handle<HArray1<Handle<StepGeomCartesianPoint>>>;
        item control_points_list_value = control_points_list: Handle<StepGeomCartesianPoint>;
        len nb_control_points_list = control_points_list;
        set_copy set_curve_form = curve_form: StepGeomBSplineCurveForm;
        get curve_form = curve_form: StepGeomBSplineCurveForm;
        set_copy set_closed_curve = closed_curve: StepDataLogical;
        get closed_curve = closed_curve: StepDataLogical;
        set_copy set_self_intersect = self_intersect: StepDataLogical;
        get self_intersect = self_intersect: StepDataLogical;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_degree: i32,
        a_control_points_list: &Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
        a_curve_form: StepGeomBSplineCurveForm,
        a_closed_curve: StepDataLogical,
        a_self_intersect: StepDataLogical,
    ) {
        let mut state = self.state.write();
        state.degree = a_degree;
        state.control_points_list = a_control_points_list.clone();
        state.curve_form = a_curve_form;
        state.closed_curve = a_closed_curve;
        state.self_intersect = a_self_intersect;
        drop(state);
        self.parent.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// B spline curve with knots.
    StepGeomBSplineCurveWithKnots, StepGeomBSplineCurveWithKnotsState, "StepGeomBSplineCurveWithKnots", StepGeomBSplineCurve, {
        knot_multiplicities: Handle<HArray1<i32>>,
        knots: Handle<HArray1<f64>>,
        knot_spec: StepGeomKnotType,
    }
}
impl StepGeomBSplineCurveWithKnots {
    step_methods! {
        set set_knot_multiplicities = knot_multiplicities: Handle<HArray1<i32>>;
        get knot_multiplicities = knot_multiplicities: Handle<HArray1<i32>>;
        item knot_multiplicities_value = knot_multiplicities: i32;
        len_checked nb_knot_multiplicities = knot_multiplicities;
        set set_knots = knots: Handle<HArray1<f64>>;
        get knots = knots: Handle<HArray1<f64>>;
        item knots_value = knots: f64;
        len_checked nb_knots = knots;
        set_copy set_knot_spec = knot_spec: StepGeomKnotType;
        get knot_spec = knot_spec: StepGeomKnotType;
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
        a_knot_multiplicities: &Handle<HArray1<i32>>,
        a_knots: &Handle<HArray1<f64>>,
        a_knot_spec: StepGeomKnotType,
    ) {
        let mut state = self.state.write();
        state.knot_multiplicities = a_knot_multiplicities.clone();
        state.knots = a_knots.clone();
        state.knot_spec = a_knot_spec;
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
}
