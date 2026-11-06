#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;

step_entity! {
    /// Surface curve.
    StepGeomSurfaceCurve, StepGeomSurfaceCurveState, "StepGeomSurfaceCurve", StepGeomCurve, {
        curve3d: Handle<StepGeomCurve>,
        associated_geometry: Handle<HArray1<StepGeomPcurveOrSurface>>,
        master_representation: StepGeomPreferredSurfaceCurveRepresentation,
    }
}
impl StepGeomSurfaceCurve {
    step_methods! {
        set set_curve3d = curve3d: Handle<StepGeomCurve>;
        get curve3d = curve3d: Handle<StepGeomCurve>;
        set set_associated_geometry = associated_geometry: Handle<HArray1<StepGeomPcurveOrSurface>>;
        get associated_geometry = associated_geometry: Handle<HArray1<StepGeomPcurveOrSurface>>;
        item associated_geometry_value = associated_geometry: StepGeomPcurveOrSurface;
        len_checked nb_associated_geometry = associated_geometry;
        set_copy set_master_representation = master_representation: StepGeomPreferredSurfaceCurveRepresentation;
        get master_representation = master_representation: StepGeomPreferredSurfaceCurveRepresentation;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_curve3d: &Handle<StepGeomCurve>,
        a_associated_geometry: &Handle<HArray1<StepGeomPcurveOrSurface>>,
        a_master_representation: StepGeomPreferredSurfaceCurveRepresentation,
    ) {
        let mut state = self.state.write();
        state.curve3d = a_curve3d.clone();
        state.associated_geometry = a_associated_geometry.clone();
        state.master_representation = a_master_representation;
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// complex type: bounded_curve + surface_curve
    /// needed for curve_bounded_surfaces (S4132)
    StepGeomSurfaceCurveAndBoundedCurve, StepGeomSurfaceCurveAndBoundedCurveState, "StepGeomSurfaceCurveAndBoundedCurve", StepGeomSurfaceCurve, {
        my_bounded_curve: Handle<StepGeomBoundedCurve>,
    }
}
impl StepGeomSurfaceCurveAndBoundedCurve {
    step_methods! {
        get bounded_curve = my_bounded_curve: Handle<StepGeomBoundedCurve>;
    }
}

step_entity! {
    /// Surface of linear extrusion.
    StepGeomSurfaceOfLinearExtrusion, StepGeomSurfaceOfLinearExtrusionState, "StepGeomSurfaceOfLinearExtrusion", StepGeomSweptSurface, {
        extrusion_axis: Handle<StepGeomVector>,
    }
}
impl StepGeomSurfaceOfLinearExtrusion {
    step_methods! {
        set set_extrusion_axis = extrusion_axis: Handle<StepGeomVector>;
        get extrusion_axis = extrusion_axis: Handle<StepGeomVector>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_swept_curve: &Handle<StepGeomCurve>,
        a_extrusion_axis: &Handle<StepGeomVector>,
    ) {
        self.state.write().extrusion_axis = a_extrusion_axis.clone();
        self.parent.init(a_name, a_swept_curve);
    }
}

step_entity! {
    /// Surface of revolution.
    StepGeomSurfaceOfRevolution, StepGeomSurfaceOfRevolutionState, "StepGeomSurfaceOfRevolution", StepGeomSweptSurface, {
        axis_position: Handle<StepGeomAxis1Placement>,
    }
}
impl StepGeomSurfaceOfRevolution {
    step_methods! {
        set set_axis_position = axis_position: Handle<StepGeomAxis1Placement>;
        get axis_position = axis_position: Handle<StepGeomAxis1Placement>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_swept_curve: &Handle<StepGeomCurve>,
        a_axis_position: &Handle<StepGeomAxis1Placement>,
    ) {
        self.state.write().axis_position = a_axis_position.clone();
        self.parent.init(a_name, a_swept_curve);
    }
}

step_entity! {
    /// Surface patch.
    StepGeomSurfacePatch, StepGeomSurfacePatchState, "StepGeomSurfacePatch", StandardTransient, {
        parent_surface: Handle<StepGeomBoundedSurface>,
        u_transition: StepGeomTransitionCode,
        v_transition: StepGeomTransitionCode,
        u_sense: bool,
        v_sense: bool,
    }
}
impl StepGeomSurfacePatch {
    step_methods! {
        set set_parent_surface = parent_surface: Handle<StepGeomBoundedSurface>;
        get parent_surface = parent_surface: Handle<StepGeomBoundedSurface>;
        set_copy set_u_transition = u_transition: StepGeomTransitionCode;
        get u_transition = u_transition: StepGeomTransitionCode;
        set_copy set_v_transition = v_transition: StepGeomTransitionCode;
        get v_transition = v_transition: StepGeomTransitionCode;
        set_copy set_u_sense = u_sense: bool;
        get u_sense = u_sense: bool;
        set_copy set_v_sense = v_sense: bool;
        get v_sense = v_sense: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_parent_surface: &Handle<StepGeomBoundedSurface>,
        a_u_transition: StepGeomTransitionCode,
        a_v_transition: StepGeomTransitionCode,
        a_u_sense: bool,
        a_v_sense: bool,
    ) {
        let mut state = self.state.write();
        state.parent_surface = a_parent_surface.clone();
        state.u_transition = a_u_transition;
        state.v_transition = a_v_transition;
        state.u_sense = a_u_sense;
        state.v_sense = a_v_sense;
    }
}

step_entity! {
    /// Surface replica.
    StepGeomSurfaceReplica, StepGeomSurfaceReplicaState, "StepGeomSurfaceReplica", StepGeomSurface, {
        parent_surface: Handle<StepGeomSurface>,
        transformation: Handle<StepGeomCartesianTransformationOperator3d>,
    }
}
impl StepGeomSurfaceReplica {
    step_methods! {
        set set_parent_surface = parent_surface: Handle<StepGeomSurface>;
        get parent_surface = parent_surface: Handle<StepGeomSurface>;
        set set_transformation = transformation: Handle<StepGeomCartesianTransformationOperator3d>;
        get transformation = transformation: Handle<StepGeomCartesianTransformationOperator3d>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_parent_surface: &Handle<StepGeomSurface>,
        a_transformation: &Handle<StepGeomCartesianTransformationOperator3d>,
    ) {
        let mut state = self.state.write();
        state.parent_surface = a_parent_surface.clone();
        state.transformation = a_transformation.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Swept surface.
    StepGeomSweptSurface, StepGeomSweptSurfaceState, "StepGeomSweptSurface", StepGeomSurface, {
        swept_curve: Handle<StepGeomCurve>,
    }
}
impl StepGeomSweptSurface {
    step_methods! {
        set set_swept_curve = swept_curve: Handle<StepGeomCurve>;
        get swept_curve = swept_curve: Handle<StepGeomCurve>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_swept_curve: &Handle<StepGeomCurve>) {
        self.state.write().swept_curve = a_swept_curve.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Toroidal surface.
    StepGeomToroidalSurface, StepGeomToroidalSurfaceState, "StepGeomToroidalSurface", StepGeomElementarySurface, {
        major_radius: f64,
        minor_radius: f64,
    }
}
impl StepGeomToroidalSurface {
    step_methods! {
        set_copy set_major_radius = major_radius: f64;
        get major_radius = major_radius: f64;
        set_copy set_minor_radius = minor_radius: f64;
        get minor_radius = minor_radius: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis2Placement3d>,
        a_major_radius: f64,
        a_minor_radius: f64,
    ) {
        let mut state = self.state.write();
        state.major_radius = a_major_radius;
        state.minor_radius = a_minor_radius;
        drop(state);
        self.parent.init(a_name, a_position);
    }
}

step_entity! {
    /// Trimmed curve.
    StepGeomTrimmedCurve, StepGeomTrimmedCurveState, "StepGeomTrimmedCurve", StepGeomBoundedCurve, {
        basis_curve: Handle<StepGeomCurve>,
        trim1: Handle<HArray1<StepGeomTrimmingSelect>>,
        trim2: Handle<HArray1<StepGeomTrimmingSelect>>,
        sense_agreement: bool,
        master_representation: StepGeomTrimmingPreference,
    }
}
impl StepGeomTrimmedCurve {
    step_methods! {
        set set_basis_curve = basis_curve: Handle<StepGeomCurve>;
        get basis_curve = basis_curve: Handle<StepGeomCurve>;
        set set_trim1 = trim1: Handle<HArray1<StepGeomTrimmingSelect>>;
        get trim1 = trim1: Handle<HArray1<StepGeomTrimmingSelect>>;
        item trim1_value = trim1: StepGeomTrimmingSelect;
        len_checked nb_trim1 = trim1;
        set set_trim2 = trim2: Handle<HArray1<StepGeomTrimmingSelect>>;
        get trim2 = trim2: Handle<HArray1<StepGeomTrimmingSelect>>;
        item trim2_value = trim2: StepGeomTrimmingSelect;
        len_checked nb_trim2 = trim2;
        set_copy set_sense_agreement = sense_agreement: bool;
        get sense_agreement = sense_agreement: bool;
        set_copy set_master_representation = master_representation: StepGeomTrimmingPreference;
        get master_representation = master_representation: StepGeomTrimmingPreference;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_curve: &Handle<StepGeomCurve>,
        a_trim1: &Handle<HArray1<StepGeomTrimmingSelect>>,
        a_trim2: &Handle<HArray1<StepGeomTrimmingSelect>>,
        a_sense_agreement: bool,
        a_master_representation: StepGeomTrimmingPreference,
    ) {
        let mut state = self.state.write();
        state.basis_curve = a_basis_curve.clone();
        state.trim1 = a_trim1.clone();
        state.trim2 = a_trim2.clone();
        state.sense_agreement = a_sense_agreement;
        state.master_representation = a_master_representation;
        drop(state);
        self.parent.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Uniform curve.
    StepGeomUniformCurve, StepGeomUniformCurveState, "StepGeomUniformCurve", StepGeomBSplineCurve, {
    }
}
