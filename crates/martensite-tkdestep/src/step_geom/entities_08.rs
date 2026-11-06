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
    /// Geometric representation context and parametric representation context.
    StepGeomGeometricRepresentationContextAndParametricRepresentationContext, StepGeomGeometricRepresentationContextAndParametricRepresentationContextState, "StepGeomGeometricRepresentationContextAndParametricRepresentationContext", StepReprRepresentationContext, {
        geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>,
        parametric_representation_context: Handle<StepReprParametricRepresentationContext>,
    }
}
impl StepGeomGeometricRepresentationContextAndParametricRepresentationContext {
    step_methods! {
        set set_geometric_representation_context = geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>;
        get geometric_representation_context = geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>;
        set set_parametric_representation_context = parametric_representation_context: Handle<StepReprParametricRepresentationContext>;
        get parametric_representation_context = parametric_representation_context: Handle<StepReprParametricRepresentationContext>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_geometric_representation_context: &Handle<StepGeomGeometricRepresentationContext>,
        a_parametric_representation_context: &Handle<StepReprParametricRepresentationContext>,
    ) {
        let mut state = self.state.write();
        state.geometric_representation_context = a_geometric_representation_context.clone();
        state.parametric_representation_context = a_parametric_representation_context.clone();
        drop(state);
        self.parent.init(a_context_identifier, a_context_type);
    }
    /// `init`.
    pub fn init_int(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_coordinate_space_dimension: i32,
    ) {
        self.parent.init(a_context_identifier, a_context_type);
        self.state.write().geometric_representation_context =
            Handle::new(StepGeomGeometricRepresentationContext::new());
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        field_geometric_representation_context.init(
            a_context_identifier,
            a_context_type,
            a_coordinate_space_dimension,
        );
        self.state.write().parametric_representation_context =
            Handle::new(StepReprParametricRepresentationContext::new());
        let field_parametric_representation_context =
            self.state.read().parametric_representation_context.clone();
        field_parametric_representation_context.init(a_context_identifier, a_context_type);
    }
    /// `set_coordinate_space_dimension`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_coordinate_space_dimension(
        &self,
        a_coordinate_space_dimension: i32,
    ) -> Result<(), Failure> {
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        step_live(&field_geometric_representation_context, "StepGeomGeometricRepresentationContextAndParametricRepresentationContext::SetCoordinateSpaceDimension")?.set_coordinate_space_dimension(a_coordinate_space_dimension);
        Ok(())
    }
    /// `coordinate_space_dimension`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn coordinate_space_dimension(&self) -> Result<i32, Failure> {
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        Ok(step_live(&field_geometric_representation_context, "StepGeomGeometricRepresentationContextAndParametricRepresentationContext::CoordinateSpaceDimension")?.coordinate_space_dimension())
    }
}

step_entity! {
    /// Geometric representation item.
    StepGeomGeometricRepresentationItem, StepGeomGeometricRepresentationItemState, "StepGeomGeometricRepresentationItem", StepReprRepresentationItem, {
    }
}

step_entity! {
    /// Hyperbola.
    StepGeomHyperbola, StepGeomHyperbolaState, "StepGeomHyperbola", StepGeomConic, {
        semi_axis: f64,
        semi_imag_axis: f64,
    }
}
impl StepGeomHyperbola {
    step_methods! {
        set_copy set_semi_axis = semi_axis: f64;
        get semi_axis = semi_axis: f64;
        set_copy set_semi_imag_axis = semi_imag_axis: f64;
        get semi_imag_axis = semi_imag_axis: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &StepGeomAxis2Placement,
        a_semi_axis: f64,
        a_semi_imag_axis: f64,
    ) {
        let mut state = self.state.write();
        state.semi_axis = a_semi_axis;
        state.semi_imag_axis = a_semi_imag_axis;
        drop(state);
        self.parent.init(a_name, a_position);
    }
}

step_entity! {
    /// Intersection curve.
    StepGeomIntersectionCurve, StepGeomIntersectionCurveState, "StepGeomIntersectionCurve", StepGeomSurfaceCurve, {
    }
}

step_entity! {
    /// Line.
    StepGeomLine, StepGeomLineState, "StepGeomLine", StepGeomCurve, {
        pnt: Handle<StepGeomCartesianPoint>,
        dir: Handle<StepGeomVector>,
    }
}
impl StepGeomLine {
    step_methods! {
        set set_pnt = pnt: Handle<StepGeomCartesianPoint>;
        get pnt = pnt: Handle<StepGeomCartesianPoint>;
        set set_dir = dir: Handle<StepGeomVector>;
        get dir = dir: Handle<StepGeomVector>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_pnt: &Handle<StepGeomCartesianPoint>,
        a_dir: &Handle<StepGeomVector>,
    ) {
        let mut state = self.state.write();
        state.pnt = a_pnt.clone();
        state.dir = a_dir.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Offset curve3d.
    StepGeomOffsetCurve3d, StepGeomOffsetCurve3dState, "StepGeomOffsetCurve3d", StepGeomCurve, {
        basis_curve: Handle<StepGeomCurve>,
        distance: f64,
        self_intersect: StepDataLogical,
        ref_direction: Handle<StepGeomDirection>,
    }
}
impl StepGeomOffsetCurve3d {
    step_methods! {
        set set_basis_curve = basis_curve: Handle<StepGeomCurve>;
        get basis_curve = basis_curve: Handle<StepGeomCurve>;
        set_copy set_distance = distance: f64;
        get distance = distance: f64;
        set_copy set_self_intersect = self_intersect: StepDataLogical;
        get self_intersect = self_intersect: StepDataLogical;
        set set_ref_direction = ref_direction: Handle<StepGeomDirection>;
        get ref_direction = ref_direction: Handle<StepGeomDirection>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_curve: &Handle<StepGeomCurve>,
        a_distance: f64,
        a_self_intersect: StepDataLogical,
        a_ref_direction: &Handle<StepGeomDirection>,
    ) {
        let mut state = self.state.write();
        state.basis_curve = a_basis_curve.clone();
        state.distance = a_distance;
        state.self_intersect = a_self_intersect;
        state.ref_direction = a_ref_direction.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Offset surface.
    StepGeomOffsetSurface, StepGeomOffsetSurfaceState, "StepGeomOffsetSurface", StepGeomSurface, {
        basis_surface: Handle<StepGeomSurface>,
        distance: f64,
        self_intersect: StepDataLogical,
    }
}
impl StepGeomOffsetSurface {
    step_methods! {
        set set_basis_surface = basis_surface: Handle<StepGeomSurface>;
        get basis_surface = basis_surface: Handle<StepGeomSurface>;
        set_copy set_distance = distance: f64;
        get distance = distance: f64;
        set_copy set_self_intersect = self_intersect: StepDataLogical;
        get self_intersect = self_intersect: StepDataLogical;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_surface: &Handle<StepGeomSurface>,
        a_distance: f64,
        a_self_intersect: StepDataLogical,
    ) {
        let mut state = self.state.write();
        state.basis_surface = a_basis_surface.clone();
        state.distance = a_distance;
        state.self_intersect = a_self_intersect;
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Representation of STEP entity OrientedSurface
    StepGeomOrientedSurface, StepGeomOrientedSurfaceState, "StepGeomOrientedSurface", StepGeomSurface, {
        the_orientation: bool,
    }
}
impl StepGeomOrientedSurface {
    step_methods! {
        get orientation = the_orientation: bool;
        set_copy set_orientation = the_orientation: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(&self, a_representation_item_name: &Handle<HAsciiString>, a_orientation: bool) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_orientation = a_orientation;
    }
}

step_entity! {
    /// Outer boundary curve.
    StepGeomOuterBoundaryCurve, StepGeomOuterBoundaryCurveState, "StepGeomOuterBoundaryCurve", StepGeomBoundaryCurve, {
    }
}

step_entity! {
    /// Parabola.
    StepGeomParabola, StepGeomParabolaState, "StepGeomParabola", StepGeomConic, {
        focal_dist: f64,
    }
}
impl StepGeomParabola {
    step_methods! {
        set_copy set_focal_dist = focal_dist: f64;
        get focal_dist = focal_dist: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &StepGeomAxis2Placement,
        a_focal_dist: f64,
    ) {
        self.state.write().focal_dist = a_focal_dist;
        self.parent.init(a_name, a_position);
    }
}

step_entity! {
    /// Pcurve.
    StepGeomPcurve, StepGeomPcurveState, "StepGeomPcurve", StepGeomCurve, {
        basis_surface: Handle<StepGeomSurface>,
        reference_to_curve: Handle<StepReprDefinitionalRepresentation>,
    }
}
impl StepGeomPcurve {
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
    /// Placement.
    StepGeomPlacement, StepGeomPlacementState, "StepGeomPlacement", StepGeomGeometricRepresentationItem, {
        location: Handle<StepGeomCartesianPoint>,
    }
}
impl StepGeomPlacement {
    step_methods! {
        set set_location = location: Handle<StepGeomCartesianPoint>;
        get location = location: Handle<StepGeomCartesianPoint>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_location: &Handle<StepGeomCartesianPoint>) {
        self.state.write().location = a_location.clone();
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Plane.
    StepGeomPlane, StepGeomPlaneState, "StepGeomPlane", StepGeomElementarySurface, {
    }
}

step_entity! {
    /// Point.
    StepGeomPoint, StepGeomPointState, "StepGeomPoint", StepGeomGeometricRepresentationItem, {
    }
}
