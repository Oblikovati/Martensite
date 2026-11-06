#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Composite curve.
    StepGeomCompositeCurve, StepGeomCompositeCurveState, "StepGeomCompositeCurve", StepGeomBoundedCurve, {
        segments: Handle<HArray1<Handle<StepGeomCompositeCurveSegment>>>,
        self_intersect: StepDataLogical,
    }
}
impl StepGeomCompositeCurve {
    step_methods! {
        set set_segments = segments: Handle<HArray1<Handle<StepGeomCompositeCurveSegment>>>;
        get segments = segments: Handle<HArray1<Handle<StepGeomCompositeCurveSegment>>>;
        item segments_value = segments: Handle<StepGeomCompositeCurveSegment>;
        len nb_segments = segments;
        set_copy set_self_intersect = self_intersect: StepDataLogical;
        get self_intersect = self_intersect: StepDataLogical;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_segments: &Handle<HArray1<Handle<StepGeomCompositeCurveSegment>>>,
        a_self_intersect: StepDataLogical,
    ) {
        let mut state = self.state.write();
        state.segments = a_segments.clone();
        state.self_intersect = a_self_intersect;
        drop(state);
        self.parent.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Composite curve on surface.
    StepGeomCompositeCurveOnSurface, StepGeomCompositeCurveOnSurfaceState, "StepGeomCompositeCurveOnSurface", StepGeomCompositeCurve, {
    }
}

step_entity! {
    /// Composite curve segment.
    StepGeomCompositeCurveSegment, StepGeomCompositeCurveSegmentState, "StepGeomCompositeCurveSegment", StandardTransient, {
        transition: StepGeomTransitionCode,
        same_sense: bool,
        parent_curve: Handle<StepGeomCurve>,
    }
}
impl StepGeomCompositeCurveSegment {
    step_methods! {
        set_copy set_transition = transition: StepGeomTransitionCode;
        get transition = transition: StepGeomTransitionCode;
        set_copy set_same_sense = same_sense: bool;
        get same_sense = same_sense: bool;
        set set_parent_curve = parent_curve: Handle<StepGeomCurve>;
        get parent_curve = parent_curve: Handle<StepGeomCurve>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_transition: StepGeomTransitionCode,
        a_same_sense: bool,
        a_parent_curve: &Handle<StepGeomCurve>,
    ) {
        let mut state = self.state.write();
        state.transition = a_transition;
        state.same_sense = a_same_sense;
        state.parent_curve = a_parent_curve.clone();
    }
}

step_entity! {
    /// Conic.
    StepGeomConic, StepGeomConicState, "StepGeomConic", StepGeomCurve, {
        position: StepGeomAxis2Placement,
    }
}
impl StepGeomConic {
    step_methods! {
        set set_position = position: StepGeomAxis2Placement;
        get position = position: StepGeomAxis2Placement;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_position: &StepGeomAxis2Placement) {
        self.state.write().position = a_position.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Conical surface.
    StepGeomConicalSurface, StepGeomConicalSurfaceState, "StepGeomConicalSurface", StepGeomElementarySurface, {
        radius: f64,
        semi_angle: f64,
    }
}
impl StepGeomConicalSurface {
    step_methods! {
        set_copy set_radius = radius: f64;
        get radius = radius: f64;
        set_copy set_semi_angle = semi_angle: f64;
        get semi_angle = semi_angle: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis2Placement3d>,
        a_radius: f64,
        a_semi_angle: f64,
    ) {
        let mut state = self.state.write();
        state.radius = a_radius;
        state.semi_angle = a_semi_angle;
        drop(state);
        self.parent.init(a_name, a_position);
    }
}

step_entity! {
    /// Curve.
    StepGeomCurve, StepGeomCurveState, "StepGeomCurve", StepGeomGeometricRepresentationItem, {
    }
}

step_entity! {
    /// Representation of STEP entity CurveBoundedSurface
    StepGeomCurveBoundedSurface, StepGeomCurveBoundedSurfaceState, "StepGeomCurveBoundedSurface", StepGeomBoundedSurface, {
        the_basis_surface: Handle<StepGeomSurface>,
        the_boundaries: Handle<HArray1<StepGeomSurfaceBoundary>>,
        the_implicit_outer: bool,
    }
}
impl StepGeomCurveBoundedSurface {
    step_methods! {
        get basis_surface = the_basis_surface: Handle<StepGeomSurface>;
        set set_basis_surface = the_basis_surface: Handle<StepGeomSurface>;
        get boundaries = the_boundaries: Handle<HArray1<StepGeomSurfaceBoundary>>;
        set set_boundaries = the_boundaries: Handle<HArray1<StepGeomSurfaceBoundary>>;
        get implicit_outer = the_implicit_outer: bool;
        set_copy set_implicit_outer = the_implicit_outer: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_basis_surface: &Handle<StepGeomSurface>,
        a_boundaries: &Handle<HArray1<StepGeomSurfaceBoundary>>,
        a_implicit_outer: bool,
    ) {
        self.parent.init(a_representation_item_name);
        let mut state = self.state.write();
        state.the_basis_surface = a_basis_surface.clone();
        state.the_boundaries = a_boundaries.clone();
        state.the_implicit_outer = a_implicit_outer;
    }
}

step_entity! {
    /// Curve replica.
    StepGeomCurveReplica, StepGeomCurveReplicaState, "StepGeomCurveReplica", StepGeomCurve, {
        parent_curve: Handle<StepGeomCurve>,
        transformation: Handle<StepGeomCartesianTransformationOperator>,
    }
}
impl StepGeomCurveReplica {
    step_methods! {
        set set_parent_curve = parent_curve: Handle<StepGeomCurve>;
        get parent_curve = parent_curve: Handle<StepGeomCurve>;
        set set_transformation = transformation: Handle<StepGeomCartesianTransformationOperator>;
        get transformation = transformation: Handle<StepGeomCartesianTransformationOperator>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_parent_curve: &Handle<StepGeomCurve>,
        a_transformation: &Handle<StepGeomCartesianTransformationOperator>,
    ) {
        let mut state = self.state.write();
        state.parent_curve = a_parent_curve.clone();
        state.transformation = a_transformation.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Cylindrical surface.
    StepGeomCylindricalSurface, StepGeomCylindricalSurfaceState, "StepGeomCylindricalSurface", StepGeomElementarySurface, {
        radius: f64,
    }
}
impl StepGeomCylindricalSurface {
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
    /// Degenerate pcurve.
    StepGeomDegeneratePcurve, StepGeomDegeneratePcurveState, "StepGeomDegeneratePcurve", StepGeomPoint, {
        basis_surface: Handle<StepGeomSurface>,
        reference_to_curve: Handle<StepReprDefinitionalRepresentation>,
    }
}
impl StepGeomDegeneratePcurve {
    step_methods! {
        set set_basis_surface = basis_surface: Handle<StepGeomSurface>;
        get basis_surface = basis_surface: Handle<StepGeomSurface>;
        set set_reference_to_curve = reference_to_curve: Handle<StepReprDefinitionalRepresentation>;
        get reference_to_curve = reference_to_curve: Handle<StepReprDefinitionalRepresentation>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_surface: &Handle<StepGeomSurface>,
        a_reference_to_curve: &Handle<StepReprDefinitionalRepresentation>,
    ) {
        let mut state = self.state.write();
        state.basis_surface = a_basis_surface.clone();
        state.reference_to_curve = a_reference_to_curve.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Degenerate toroidal surface.
    StepGeomDegenerateToroidalSurface, StepGeomDegenerateToroidalSurfaceState, "StepGeomDegenerateToroidalSurface", StepGeomToroidalSurface, {
        select_outer: bool,
    }
}
impl StepGeomDegenerateToroidalSurface {
    step_methods! {
        set_copy set_select_outer = select_outer: bool;
        get select_outer = select_outer: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis2Placement3d>,
        a_major_radius: f64,
        a_minor_radius: f64,
        a_select_outer: bool,
    ) {
        self.state.write().select_outer = a_select_outer;
        self.parent
            .init(a_name, a_position, a_major_radius, a_minor_radius);
    }
}

step_entity! {
    /// Elementary surface.
    StepGeomElementarySurface, StepGeomElementarySurfaceState, "StepGeomElementarySurface", StepGeomSurface, {
        position: Handle<StepGeomAxis2Placement3d>,
    }
}
impl StepGeomElementarySurface {
    step_methods! {
        set set_position = position: Handle<StepGeomAxis2Placement3d>;
        get position = position: Handle<StepGeomAxis2Placement3d>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis2Placement3d>,
    ) {
        self.state.write().position = a_position.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Ellipse.
    StepGeomEllipse, StepGeomEllipseState, "StepGeomEllipse", StepGeomConic, {
        semi_axis1: f64,
        semi_axis2: f64,
    }
}
impl StepGeomEllipse {
    step_methods! {
        set_copy set_semi_axis1 = semi_axis1: f64;
        get semi_axis1 = semi_axis1: f64;
        set_copy set_semi_axis2 = semi_axis2: f64;
        get semi_axis2 = semi_axis2: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &StepGeomAxis2Placement,
        a_semi_axis1: f64,
        a_semi_axis2: f64,
    ) {
        let mut state = self.state.write();
        state.semi_axis1 = a_semi_axis1;
        state.semi_axis2 = a_semi_axis2;
        drop(state);
        self.parent.init(a_name, a_position);
    }
}
