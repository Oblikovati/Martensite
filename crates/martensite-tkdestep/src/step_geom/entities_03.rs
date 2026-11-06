#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// B spline surface with knots.
    StepGeomBSplineSurfaceWithKnots, StepGeomBSplineSurfaceWithKnotsState, "StepGeomBSplineSurfaceWithKnots", StepGeomBSplineSurface, {
        u_multiplicities: Handle<HArray1<i32>>,
        v_multiplicities: Handle<HArray1<i32>>,
        u_knots: Handle<HArray1<f64>>,
        v_knots: Handle<HArray1<f64>>,
        knot_spec: StepGeomKnotType,
    }
}
impl StepGeomBSplineSurfaceWithKnots {
    step_methods! {
        set set_u_multiplicities = u_multiplicities: Handle<HArray1<i32>>;
        get u_multiplicities = u_multiplicities: Handle<HArray1<i32>>;
        item u_multiplicities_value = u_multiplicities: i32;
        len_checked nb_u_multiplicities = u_multiplicities;
        set set_v_multiplicities = v_multiplicities: Handle<HArray1<i32>>;
        get v_multiplicities = v_multiplicities: Handle<HArray1<i32>>;
        item v_multiplicities_value = v_multiplicities: i32;
        len_checked nb_v_multiplicities = v_multiplicities;
        set set_u_knots = u_knots: Handle<HArray1<f64>>;
        get u_knots = u_knots: Handle<HArray1<f64>>;
        item u_knots_value = u_knots: f64;
        len_checked nb_u_knots = u_knots;
        set set_v_knots = v_knots: Handle<HArray1<f64>>;
        get v_knots = v_knots: Handle<HArray1<f64>>;
        item v_knots_value = v_knots: f64;
        len_checked nb_v_knots = v_knots;
        set_copy set_knot_spec = knot_spec: StepGeomKnotType;
        get knot_spec = knot_spec: StepGeomKnotType;
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
        a_u_multiplicities: &Handle<HArray1<i32>>,
        a_v_multiplicities: &Handle<HArray1<i32>>,
        a_u_knots: &Handle<HArray1<f64>>,
        a_v_knots: &Handle<HArray1<f64>>,
        a_knot_spec: StepGeomKnotType,
    ) {
        let mut state = self.state.write();
        state.u_multiplicities = a_u_multiplicities.clone();
        state.v_multiplicities = a_v_multiplicities.clone();
        state.u_knots = a_u_knots.clone();
        state.v_knots = a_v_knots.clone();
        state.knot_spec = a_knot_spec;
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
}
