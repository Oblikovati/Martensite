#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_geom::*;
#[allow(unused_imports)]
use crate::step_repr::*;

/// Angle relator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepShapeAngleRelator {
    /// Equal.
    #[default]
    Equal,
    /// Large.
    Large,
    /// Small.
    Small,
}

/// Boolean operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepShapeBooleanOperator {
    /// Difference.
    #[default]
    BoDifference,
    /// Intersection.
    BoIntersection,
    /// Union.
    BoUnion,
}

step_select! {
    /// Csg primitive.
    StepShapeCsgPrimitive, [StepShapeSphere, StepShapeBlock, StepShapeRightAngularWedge, StepShapeTorus, StepShapeRightCircularCone, StepShapeRightCircularCylinder], {
        sphere: StepShapeSphere,
        block: StepShapeBlock,
        right_angular_wedge: StepShapeRightAngularWedge,
        torus: StepShapeTorus,
        right_circular_cone: StepShapeRightCircularCone,
        right_circular_cylinder: StepShapeRightCircularCylinder,
    }
}

step_select! {
    /// Representation of STEP SELECT type DimensionalCharacteristic
    StepShapeDimensionalCharacteristic, [StepShapeDimensionalLocation, StepShapeDimensionalSize], {
        dimensional_location: StepShapeDimensionalLocation,
        dimensional_size: StepShapeDimensionalSize,
    }
}

step_select! {
    /// Geometric set select.
    StepShapeGeometricSetSelect, [StepGeomPoint, StepGeomCurve, StepGeomSurface, StepGeomGeometricRepresentationItem], {
        point: StepGeomPoint,
        curve: StepGeomCurve,
        surface: StepGeomSurface,
    }
}

step_select! {
    /// Shape dimension representation item.
    StepShapeShapeDimensionRepresentationItem, [StepReprCompoundRepresentationItem, StepReprDescriptiveRepresentationItem, StepReprMeasureRepresentationItem, StepGeomPlacement], {
        compound_representation_item: StepReprCompoundRepresentationItem,
        descriptive_representation_item: StepReprDescriptiveRepresentationItem,
        measure_representation_item: StepReprMeasureRepresentationItem,
        placement: StepGeomPlacement,
    }
}

step_select! {
    /// Shell.
    StepShapeShell, [StepShapeOpenShell, StepShapeClosedShell], {
        open_shell: StepShapeOpenShell,
        closed_shell: StepShapeClosedShell,
    }
}

step_select! {
    /// Added for Dimensional Tolerances
    StepShapeToleranceMethodDefinition, [StepShapeToleranceValue, StepShapeLimitsAndFits], {
        tolerance_value: StepShapeToleranceValue,
        limits_and_fits: StepShapeLimitsAndFits,
    }
}

step_select! {
    /// Added for Dimensional Tolerances
    StepShapeValueQualifier, numbered [StepShapePrecisionQualifier = 1, StepShapeTypeQualifier = 2, StepShapeValueFormatTypeQualifier = 4], {
        precision_qualifier: StepShapePrecisionQualifier,
        type_qualifier: StepShapeTypeQualifier,
        value_format_type_qualifier: StepShapeValueFormatTypeQualifier,
    }
}

step_entity! {
    /// Advanced brep shape representation.
    StepShapeAdvancedBrepShapeRepresentation, StepShapeAdvancedBrepShapeRepresentationState, "StepShapeAdvancedBrepShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Advanced face.
    StepShapeAdvancedFace, StepShapeAdvancedFaceState, "StepShapeAdvancedFace", StepShapeFaceSurface, {
    }
}

step_entity! {
    /// Representation of STEP entity AngularLocation
    StepShapeAngularLocation, StepShapeAngularLocationState, "StepShapeAngularLocation", StepShapeDimensionalLocation, {
        the_angle_selection: StepShapeAngleRelator,
    }
}
impl StepShapeAngularLocation {
    step_methods! {
        get angle_selection = the_angle_selection: StepShapeAngleRelator;
        set_copy set_angle_selection = the_angle_selection: StepShapeAngleRelator;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_shape_aspect_relationship_name: &Handle<HAsciiString>,
        has_shape_aspect_relationship_description: bool,
        a_shape_aspect_relationship_description: &Handle<HAsciiString>,
        a_shape_aspect_relationship_relating_shape_aspect: &Handle<StepReprShapeAspect>,
        a_shape_aspect_relationship_related_shape_aspect: &Handle<StepReprShapeAspect>,
        a_angle_selection: StepShapeAngleRelator,
    ) {
        self.parent.init(
            a_shape_aspect_relationship_name,
            has_shape_aspect_relationship_description,
            a_shape_aspect_relationship_description,
            a_shape_aspect_relationship_relating_shape_aspect,
            a_shape_aspect_relationship_related_shape_aspect,
        );
        self.state.write().the_angle_selection = a_angle_selection;
    }
}

step_entity! {
    /// Representation of STEP entity AngularSize
    StepShapeAngularSize, StepShapeAngularSizeState, "StepShapeAngularSize", StepShapeDimensionalSize, {
        the_angle_selection: StepShapeAngleRelator,
    }
}
impl StepShapeAngularSize {
    step_methods! {
        get angle_selection = the_angle_selection: StepShapeAngleRelator;
        set_copy set_angle_selection = the_angle_selection: StepShapeAngleRelator;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_dimensional_size_applies_to: &Handle<StepReprShapeAspect>,
        a_dimensional_size_name: &Handle<HAsciiString>,
        a_angle_selection: StepShapeAngleRelator,
    ) {
        self.parent
            .init(a_dimensional_size_applies_to, a_dimensional_size_name);
        self.state.write().the_angle_selection = a_angle_selection;
    }
}

step_entity! {
    /// Block.
    StepShapeBlock, StepShapeBlockState, "StepShapeBlock", StepGeomGeometricRepresentationItem, {
        position: Handle<StepGeomAxis2Placement3d>,
        x: f64,
        y: f64,
        z: f64,
    }
}
impl StepShapeBlock {
    step_methods! {
        set set_position = position: Handle<StepGeomAxis2Placement3d>;
        get position = position: Handle<StepGeomAxis2Placement3d>;
        set_copy set_x = x: f64;
        get x = x: f64;
        set_copy set_y = y: f64;
        get y = y: f64;
        set_copy set_z = z: f64;
        get z = z: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis2Placement3d>,
        a_x: f64,
        a_y: f64,
        a_z: f64,
    ) {
        let mut state = self.state.write();
        state.position = a_position.clone();
        state.x = a_x;
        state.y = a_y;
        state.z = a_z;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Boolean result.
    StepShapeBooleanResult, StepShapeBooleanResultState, "StepShapeBooleanResult", StepGeomGeometricRepresentationItem, {
        an_operator: StepShapeBooleanOperator,
        first_operand: StepShapeBooleanOperand,
        second_operand: StepShapeBooleanOperand,
    }
}
impl StepShapeBooleanResult {
    step_methods! {
        set_copy set_operator = an_operator: StepShapeBooleanOperator;
        get operator = an_operator: StepShapeBooleanOperator;
        set set_first_operand = first_operand: StepShapeBooleanOperand;
        get first_operand = first_operand: StepShapeBooleanOperand;
        set set_second_operand = second_operand: StepShapeBooleanOperand;
        get second_operand = second_operand: StepShapeBooleanOperand;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_operator: StepShapeBooleanOperator,
        a_first_operand: &StepShapeBooleanOperand,
        a_second_operand: &StepShapeBooleanOperand,
    ) {
        let mut state = self.state.write();
        state.an_operator = a_operator;
        state.first_operand = a_first_operand.clone();
        state.second_operand = a_second_operand.clone();
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Box domain.
    StepShapeBoxDomain, StepShapeBoxDomainState, "StepShapeBoxDomain", StandardTransient, {
        corner: Handle<StepGeomCartesianPoint>,
        xlength: f64,
        ylength: f64,
        zlength: f64,
    }
}
impl StepShapeBoxDomain {
    step_methods! {
        set set_corner = corner: Handle<StepGeomCartesianPoint>;
        get corner = corner: Handle<StepGeomCartesianPoint>;
        set_copy set_xlength = xlength: f64;
        get xlength = xlength: f64;
        set_copy set_ylength = ylength: f64;
        get ylength = ylength: f64;
        set_copy set_zlength = zlength: f64;
        get zlength = zlength: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_corner: &Handle<StepGeomCartesianPoint>,
        a_xlength: f64,
        a_ylength: f64,
        a_zlength: f64,
    ) {
        let mut state = self.state.write();
        state.corner = a_corner.clone();
        state.xlength = a_xlength;
        state.ylength = a_ylength;
        state.zlength = a_zlength;
    }
}

step_entity! {
    /// Boxed half space.
    StepShapeBoxedHalfSpace, StepShapeBoxedHalfSpaceState, "StepShapeBoxedHalfSpace", StepShapeHalfSpaceSolid, {
        enclosure: Handle<StepShapeBoxDomain>,
    }
}
impl StepShapeBoxedHalfSpace {
    step_methods! {
        set set_enclosure = enclosure: Handle<StepShapeBoxDomain>;
        get enclosure = enclosure: Handle<StepShapeBoxDomain>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_base_surface: &Handle<StepGeomSurface>,
        a_agreement_flag: bool,
        a_enclosure: &Handle<StepShapeBoxDomain>,
    ) {
        self.state.write().enclosure = a_enclosure.clone();
        self.parent.init(a_name, a_base_surface, a_agreement_flag);
    }
}

step_entity! {
    /// Brep with voids.
    StepShapeBrepWithVoids, StepShapeBrepWithVoidsState, "StepShapeBrepWithVoids", StepShapeManifoldSolidBrep, {
        voids: Handle<HArray1<Handle<StepShapeOrientedClosedShell>>>,
    }
}
impl StepShapeBrepWithVoids {
    step_methods! {
        set set_voids = voids: Handle<HArray1<Handle<StepShapeOrientedClosedShell>>>;
        get voids = voids: Handle<HArray1<Handle<StepShapeOrientedClosedShell>>>;
        item voids_value = voids: Handle<StepShapeOrientedClosedShell>;
        len nb_voids = voids;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_outer: &Handle<StepShapeClosedShell>,
        a_voids: &Handle<HArray1<Handle<StepShapeOrientedClosedShell>>>,
    ) {
        self.state.write().voids = a_voids.clone();
        self.parent.init(a_name, a_outer);
    }
}
