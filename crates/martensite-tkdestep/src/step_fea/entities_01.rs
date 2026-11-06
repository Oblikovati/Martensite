#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_data::*;
#[allow(unused_imports)]
use crate::step_element::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

/// Coordinate system type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepFEACoordinateSystemType {
    /// Cartesian.
    #[default]
    Cartesian,
    /// Cylindrical.
    Cylindrical,
    /// Spherical.
    Spherical,
}

/// Enumerated degree of freedom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StepFEAEnumeratedDegreeOfFreedom {
    /// X translation.
    #[default]
    XTranslation,
    /// Y translation.
    YTranslation,
    /// Z translation.
    ZTranslation,
    /// X rotation.
    XRotation,
    /// Y rotation.
    YRotation,
    /// Z rotation.
    ZRotation,
    /// Warp.
    Warp,
}

step_select! {
    /// Representation of STEP SELECT type CurveElementEndCoordinateSystem
    StepFEACurveElementEndCoordinateSystem, [StepFEAFeaAxis2Placement3d, StepFEAAlignedCurve3dElementCoordinateSystem, StepFEAParametricCurve3dElementCoordinateSystem], {
        fea_axis2_placement3d: StepFEAFeaAxis2Placement3d,
        aligned_curve3d_element_coordinate_system: StepFEAAlignedCurve3dElementCoordinateSystem,
        parametric_curve3d_element_coordinate_system: StepFEAParametricCurve3dElementCoordinateSystem,
    }
}

step_select! {
    /// Representation of STEP SELECT type ElementOrElementGroup
    StepFEAElementOrElementGroup, [StepFEAElementRepresentation, StepFEAElementGroup], {
        element_representation: StepFEAElementRepresentation,
        element_group: StepFEAElementGroup,
    }
}

step_entity! {
    /// Representation of STEP entity AlignedCurve3dElementCoordinateSystem
    StepFEAAlignedCurve3dElementCoordinateSystem, StepFEAAlignedCurve3dElementCoordinateSystemState, "StepFEAAlignedCurve3dElementCoordinateSystem", StepFEAFeaRepresentationItem, {
        the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>,
    }
}
impl StepFEAAlignedCurve3dElementCoordinateSystem {
    step_methods! {
        get coordinate_system = the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>;
        set set_coordinate_system = the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_coordinate_system: &Handle<StepFEAFeaAxis2Placement3d>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_coordinate_system = a_coordinate_system.clone();
    }
}

step_entity! {
    /// Representation of STEP entity AlignedSurface3dElementCoordinateSystem
    StepFEAAlignedSurface3dElementCoordinateSystem, StepFEAAlignedSurface3dElementCoordinateSystemState, "StepFEAAlignedSurface3dElementCoordinateSystem", StepFEAFeaRepresentationItem, {
        the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>,
    }
}
impl StepFEAAlignedSurface3dElementCoordinateSystem {
    step_methods! {
        get coordinate_system = the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>;
        set set_coordinate_system = the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_coordinate_system: &Handle<StepFEAFeaAxis2Placement3d>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_coordinate_system = a_coordinate_system.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ArbitraryVolume3dElementCoordinateSystem
    StepFEAArbitraryVolume3dElementCoordinateSystem, StepFEAArbitraryVolume3dElementCoordinateSystemState, "StepFEAArbitraryVolume3dElementCoordinateSystem", StepFEAFeaRepresentationItem, {
        the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>,
    }
}
impl StepFEAArbitraryVolume3dElementCoordinateSystem {
    step_methods! {
        get coordinate_system = the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>;
        set set_coordinate_system = the_coordinate_system: Handle<StepFEAFeaAxis2Placement3d>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_coordinate_system: &Handle<StepFEAFeaAxis2Placement3d>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_coordinate_system = a_coordinate_system.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ConstantSurface3dElementCoordinateSystem
    StepFEAConstantSurface3dElementCoordinateSystem, StepFEAConstantSurface3dElementCoordinateSystemState, "StepFEAConstantSurface3dElementCoordinateSystem", StepFEAFeaRepresentationItem, {
        the_axis: i32,
        the_angle: f64,
    }
}
impl StepFEAConstantSurface3dElementCoordinateSystem {
    step_methods! {
        get axis = the_axis: i32;
        set_copy set_axis = the_axis: i32;
        get angle = the_angle: f64;
        set_copy set_angle = the_angle: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_axis: i32,
        a_angle: f64,
    ) {
        self.parent.init(a_representation_item_name);
        let mut state = self.state.write();
        state.the_axis = a_axis;
        state.the_angle = a_angle;
    }
}

step_entity! {
    /// Representation of STEP entity Curve3dElementProperty
    StepFEACurve3dElementProperty, StepFEACurve3dElementPropertyState, "StepFEACurve3dElementProperty", StandardTransient, {
        the_property_id: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_interval_definitions: Handle<HArray1<Handle<StepFEACurveElementInterval>>>,
        the_end_offsets: Handle<HArray1<Handle<StepFEACurveElementEndOffset>>>,
        the_end_releases: Handle<HArray1<Handle<StepFEACurveElementEndRelease>>>,
    }
}
impl StepFEACurve3dElementProperty {
    step_methods! {
        get property_id = the_property_id: Handle<HAsciiString>;
        set set_property_id = the_property_id: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get interval_definitions = the_interval_definitions: Handle<HArray1<Handle<StepFEACurveElementInterval>>>;
        set set_interval_definitions = the_interval_definitions: Handle<HArray1<Handle<StepFEACurveElementInterval>>>;
        get end_offsets = the_end_offsets: Handle<HArray1<Handle<StepFEACurveElementEndOffset>>>;
        set set_end_offsets = the_end_offsets: Handle<HArray1<Handle<StepFEACurveElementEndOffset>>>;
        get end_releases = the_end_releases: Handle<HArray1<Handle<StepFEACurveElementEndRelease>>>;
        set set_end_releases = the_end_releases: Handle<HArray1<Handle<StepFEACurveElementEndRelease>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_property_id: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_interval_definitions: &Handle<HArray1<Handle<StepFEACurveElementInterval>>>,
        a_end_offsets: &Handle<HArray1<Handle<StepFEACurveElementEndOffset>>>,
        a_end_releases: &Handle<HArray1<Handle<StepFEACurveElementEndRelease>>>,
    ) {
        let mut state = self.state.write();
        state.the_property_id = a_property_id.clone();
        state.the_description = a_description.clone();
        state.the_interval_definitions = a_interval_definitions.clone();
        state.the_end_offsets = a_end_offsets.clone();
        state.the_end_releases = a_end_releases.clone();
    }
}

step_entity! {
    /// Representation of STEP entity Curve3dElementRepresentation
    StepFEACurve3dElementRepresentation, StepFEACurve3dElementRepresentationState, "StepFEACurve3dElementRepresentation", StepFEAElementRepresentation, {
        the_model_ref: Handle<StepFEAFeaModel3d>,
        the_element_descriptor: Handle<StepElementCurve3dElementDescriptor>,
        the_property: Handle<StepFEACurve3dElementProperty>,
        the_material: Handle<StepElementElementMaterial>,
    }
}
impl StepFEACurve3dElementRepresentation {
    step_methods! {
        get model_ref = the_model_ref: Handle<StepFEAFeaModel3d>;
        set set_model_ref = the_model_ref: Handle<StepFEAFeaModel3d>;
        get element_descriptor = the_element_descriptor: Handle<StepElementCurve3dElementDescriptor>;
        set set_element_descriptor = the_element_descriptor: Handle<StepElementCurve3dElementDescriptor>;
        get property = the_property: Handle<StepFEACurve3dElementProperty>;
        set set_property = the_property: Handle<StepFEACurve3dElementProperty>;
        get material = the_material: Handle<StepElementElementMaterial>;
        set set_material = the_material: Handle<StepElementElementMaterial>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_representation_name: &Handle<HAsciiString>,
        a_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        a_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        a_element_representation_node_list: &Handle<HArray1<Handle<StepFEANodeRepresentation>>>,
        a_model_ref: &Handle<StepFEAFeaModel3d>,
        a_element_descriptor: &Handle<StepElementCurve3dElementDescriptor>,
        a_property: &Handle<StepFEACurve3dElementProperty>,
        a_material: &Handle<StepElementElementMaterial>,
    ) {
        self.parent.init(
            a_representation_name,
            a_representation_items,
            a_representation_context_of_items,
            a_element_representation_node_list,
        );
        let mut state = self.state.write();
        state.the_model_ref = a_model_ref.clone();
        state.the_element_descriptor = a_element_descriptor.clone();
        state.the_property = a_property.clone();
        state.the_material = a_material.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementEndOffset
    StepFEACurveElementEndOffset, StepFEACurveElementEndOffsetState, "StepFEACurveElementEndOffset", StandardTransient, {
        the_coordinate_system: StepFEACurveElementEndCoordinateSystem,
        the_offset_vector: Handle<HArray1<f64>>,
    }
}
impl StepFEACurveElementEndOffset {
    step_methods! {
        get coordinate_system = the_coordinate_system: StepFEACurveElementEndCoordinateSystem;
        set set_coordinate_system = the_coordinate_system: StepFEACurveElementEndCoordinateSystem;
        get offset_vector = the_offset_vector: Handle<HArray1<f64>>;
        set set_offset_vector = the_offset_vector: Handle<HArray1<f64>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_coordinate_system: &StepFEACurveElementEndCoordinateSystem,
        a_offset_vector: &Handle<HArray1<f64>>,
    ) {
        let mut state = self.state.write();
        state.the_coordinate_system = a_coordinate_system.clone();
        state.the_offset_vector = a_offset_vector.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CurveElementEndRelease
    StepFEACurveElementEndRelease, StepFEACurveElementEndReleaseState, "StepFEACurveElementEndRelease", StandardTransient, {
        the_coordinate_system: StepFEACurveElementEndCoordinateSystem,
        the_releases: Handle<HArray1<Handle<StepElementCurveElementEndReleasePacket>>>,
    }
}
impl StepFEACurveElementEndRelease {
    step_methods! {
        get coordinate_system = the_coordinate_system: StepFEACurveElementEndCoordinateSystem;
        set set_coordinate_system = the_coordinate_system: StepFEACurveElementEndCoordinateSystem;
        get releases = the_releases: Handle<HArray1<Handle<StepElementCurveElementEndReleasePacket>>>;
        set set_releases = the_releases: Handle<HArray1<Handle<StepElementCurveElementEndReleasePacket>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_coordinate_system: &StepFEACurveElementEndCoordinateSystem,
        a_releases: &Handle<HArray1<Handle<StepElementCurveElementEndReleasePacket>>>,
    ) {
        let mut state = self.state.write();
        state.the_coordinate_system = a_coordinate_system.clone();
        state.the_releases = a_releases.clone();
    }
}
