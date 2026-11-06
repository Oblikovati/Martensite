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

step_entity! {
    /// Representation of STEP entity NonManifoldSurfaceShapeRepresentation
    StepShapeNonManifoldSurfaceShapeRepresentation, StepShapeNonManifoldSurfaceShapeRepresentationState, "StepShapeNonManifoldSurfaceShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Open shell.
    StepShapeOpenShell, StepShapeOpenShellState, "StepShapeOpenShell", StepShapeConnectedFaceSet, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapePlusMinusTolerance, StepShapePlusMinusToleranceState, "StepShapePlusMinusTolerance", StandardTransient, {
        the_range: StepShapeToleranceMethodDefinition,
        the_toleranced_dimension: StepShapeDimensionalCharacteristic,
    }
}
impl StepShapePlusMinusTolerance {
    step_methods! {
        get range = the_range: StepShapeToleranceMethodDefinition;
        set set_range = the_range: StepShapeToleranceMethodDefinition;
        get toleranced_dimension = the_toleranced_dimension: StepShapeDimensionalCharacteristic;
        set set_toleranced_dimension = the_toleranced_dimension: StepShapeDimensionalCharacteristic;
    }
    /// `init`.
    pub fn init(
        &self,
        range: &StepShapeToleranceMethodDefinition,
        toleranced_dimension: &StepShapeDimensionalCharacteristic,
    ) {
        let mut state = self.state.write();
        state.the_range = range.clone();
        state.the_toleranced_dimension = toleranced_dimension.clone();
    }
}

step_entity! {
    /// Representation of STEP entity PointRepresentation
    StepShapePointRepresentation, StepShapePointRepresentationState, "StepShapePointRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Poly loop.
    StepShapePolyLoop, StepShapePolyLoopState, "StepShapePolyLoop", StepShapeLoop, {
        polygon: Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
    }
}
impl StepShapePolyLoop {
    step_methods! {
        set set_polygon = polygon: Handle<HArray1<Handle<StepGeomCartesianPoint>>>;
        get polygon = polygon: Handle<HArray1<Handle<StepGeomCartesianPoint>>>;
        item polygon_value = polygon: Handle<StepGeomCartesianPoint>;
        len nb_polygon = polygon;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_polygon: &Handle<HArray1<Handle<StepGeomCartesianPoint>>>,
    ) {
        self.state.write().polygon = a_polygon.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapePrecisionQualifier, StepShapePrecisionQualifierState, "StepShapePrecisionQualifier", StandardTransient, {
        the_precision_value: i32,
    }
}
impl StepShapePrecisionQualifier {
    step_methods! {
        set_copy init = the_precision_value: i32;
        get precision_value = the_precision_value: i32;
        set_copy set_precision_value = the_precision_value: i32;
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapeQualifiedRepresentationItem, StepShapeQualifiedRepresentationItemState, "StepShapeQualifiedRepresentationItem", StepReprRepresentationItem, {
        the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>,
    }
}
impl StepShapeQualifiedRepresentationItem {
    step_methods! {
        get qualifiers = the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>;
        len_checked nb_qualifiers = the_qualifiers;
        set set_qualifiers = the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>;
        item qualifiers_value = the_qualifiers: StepShapeValueQualifier;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        qualifiers: &Handle<HArray1<StepShapeValueQualifier>>,
    ) {
        self.parent.init(a_name);
        self.state.write().the_qualifiers = qualifiers.clone();
    }
    /// `set_qualifiers_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_qualifiers_value(
        &self,
        num: i32,
        aqualifier: &StepShapeValueQualifier,
    ) -> Result<(), Failure> {
        let field_the_qualifiers = self.state.read().the_qualifiers.clone();
        step_array1_set(
            &field_the_qualifiers,
            num,
            aqualifier.clone(),
            "StepShapeQualifiedRepresentationItem::SetQualifiersValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Revolved area solid.
    StepShapeRevolvedAreaSolid, StepShapeRevolvedAreaSolidState, "StepShapeRevolvedAreaSolid", StepShapeSweptAreaSolid, {
        axis: Handle<StepGeomAxis1Placement>,
        angle: f64,
    }
}
impl StepShapeRevolvedAreaSolid {
    step_methods! {
        set set_axis = axis: Handle<StepGeomAxis1Placement>;
        get axis = axis: Handle<StepGeomAxis1Placement>;
        set_copy set_angle = angle: f64;
        get angle = angle: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_swept_area: &Handle<StepGeomCurveBoundedSurface>,
        a_axis: &Handle<StepGeomAxis1Placement>,
        a_angle: f64,
    ) {
        let mut state = self.state.write();
        state.axis = a_axis.clone();
        state.angle = a_angle;
        drop(state);
        self.parent.init(a_name, a_swept_area);
    }
}

step_entity! {
    /// Revolved face solid.
    StepShapeRevolvedFaceSolid, StepShapeRevolvedFaceSolidState, "StepShapeRevolvedFaceSolid", StepShapeSweptFaceSolid, {
        axis: Handle<StepGeomAxis1Placement>,
        angle: f64,
    }
}
impl StepShapeRevolvedFaceSolid {
    step_methods! {
        set set_axis = axis: Handle<StepGeomAxis1Placement>;
        get axis = axis: Handle<StepGeomAxis1Placement>;
        set_copy set_angle = angle: f64;
        get angle = angle: f64;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_swept_area: &Handle<StepShapeFaceSurface>) {
        self.parent.init(a_name, a_swept_area);
    }
    /// `init`.
    pub fn init_axis1_placement(
        &self,
        a_name: &Handle<HAsciiString>,
        a_swept_area: &Handle<StepShapeFaceSurface>,
        a_axis: &Handle<StepGeomAxis1Placement>,
        a_angle: f64,
    ) {
        let mut state = self.state.write();
        state.axis = a_axis.clone();
        state.angle = a_angle;
        drop(state);
        self.parent.init(a_name, a_swept_area);
    }
}

step_entity! {
    /// Right angular wedge.
    StepShapeRightAngularWedge, StepShapeRightAngularWedgeState, "StepShapeRightAngularWedge", StepGeomGeometricRepresentationItem, {
        position: Handle<StepGeomAxis2Placement3d>,
        x: f64,
        y: f64,
        z: f64,
        ltx: f64,
    }
}
impl StepShapeRightAngularWedge {
    step_methods! {
        set set_position = position: Handle<StepGeomAxis2Placement3d>;
        get position = position: Handle<StepGeomAxis2Placement3d>;
        set_copy set_x = x: f64;
        get x = x: f64;
        set_copy set_y = y: f64;
        get y = y: f64;
        set_copy set_z = z: f64;
        get z = z: f64;
        set_copy set_ltx = ltx: f64;
        get ltx = ltx: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis2Placement3d>,
        a_x: f64,
        a_y: f64,
        a_z: f64,
        a_ltx: f64,
    ) {
        let mut state = self.state.write();
        state.position = a_position.clone();
        state.x = a_x;
        state.y = a_y;
        state.z = a_z;
        state.ltx = a_ltx;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Right circular cone.
    StepShapeRightCircularCone, StepShapeRightCircularConeState, "StepShapeRightCircularCone", StepGeomGeometricRepresentationItem, {
        position: Handle<StepGeomAxis1Placement>,
        height: f64,
        radius: f64,
        semi_angle: f64,
    }
}
impl StepShapeRightCircularCone {
    step_methods! {
        set set_position = position: Handle<StepGeomAxis1Placement>;
        get position = position: Handle<StepGeomAxis1Placement>;
        set_copy set_height = height: f64;
        get height = height: f64;
        set_copy set_radius = radius: f64;
        get radius = radius: f64;
        set_copy set_semi_angle = semi_angle: f64;
        get semi_angle = semi_angle: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis1Placement>,
        a_height: f64,
        a_radius: f64,
        a_semi_angle: f64,
    ) {
        let mut state = self.state.write();
        state.position = a_position.clone();
        state.height = a_height;
        state.radius = a_radius;
        state.semi_angle = a_semi_angle;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Right circular cylinder.
    StepShapeRightCircularCylinder, StepShapeRightCircularCylinderState, "StepShapeRightCircularCylinder", StepGeomGeometricRepresentationItem, {
        position: Handle<StepGeomAxis1Placement>,
        height: f64,
        radius: f64,
    }
}
impl StepShapeRightCircularCylinder {
    step_methods! {
        set set_position = position: Handle<StepGeomAxis1Placement>;
        get position = position: Handle<StepGeomAxis1Placement>;
        set_copy set_height = height: f64;
        get height = height: f64;
        set_copy set_radius = radius: f64;
        get radius = radius: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis1Placement>,
        a_height: f64,
        a_radius: f64,
    ) {
        let mut state = self.state.write();
        state.position = a_position.clone();
        state.height = a_height;
        state.radius = a_radius;
        drop(state);
        self.parent.parent.init(a_name);
    }
}
