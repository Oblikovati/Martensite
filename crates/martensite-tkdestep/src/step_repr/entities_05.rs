#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_shape::*;

step_entity! {
    /// Base class for complex types (MEASURE_REPRESENTATION_ITEM, MEASURE_WITH_UNIT,
    /// REPRESENTATION_ITEM, LENGTH_MEASURE_WITH_UNIT/PLANE_ANGLE_MEASURE_WITH_UNIT).
    custom_default StepReprReprItemAndMeasureWithUnit, StepReprReprItemAndMeasureWithUnitState, "StepReprReprItemAndMeasureWithUnit", StepReprRepresentationItem, {
        my_measure_representation_item: Handle<StepReprMeasureRepresentationItem>,
        my_measure_with_unit: Handle<dyn crate::StepEntityBase<StepBasicMeasureWithUnit>>,
    }, bases: [crate::step_repr::StepReprRepresentationItem, StepReprReprItemAndMeasureWithUnit]
}
impl Default for StepReprReprItemAndMeasureWithUnit {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            let mut state = object.state.write();
            state.my_measure_with_unit = Handle::new(StepBasicMeasureWithUnit::new()).into_upcast();
            state.my_measure_representation_item =
                Handle::new(StepReprMeasureRepresentationItem::new());
        }
        object
    }
}
impl StepReprReprItemAndMeasureWithUnit {
    /// `StepReprReprItemAndMeasureWithUnit()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepReprReprItemAndMeasureWithUnit {
    step_methods! {
        get get_measure_representation_item = my_measure_representation_item: Handle<StepReprMeasureRepresentationItem>;
        set set_measure_with_unit = my_measure_with_unit: Handle<dyn crate::StepEntityBase<StepBasicMeasureWithUnit>>;
        get get_measure_with_unit = my_measure_with_unit: Handle<dyn crate::StepEntityBase<StepBasicMeasureWithUnit>>;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        a_mwu: &Handle<dyn crate::StepEntityBase<StepBasicMeasureWithUnit>>,
        a_ri: &Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>,
    ) -> Result<(), Failure> {
        self.state.write().my_measure_with_unit = a_mwu.clone();
        self.set_name(&step_live(a_ri, "StepReprReprItemAndMeasureWithUnit::Init")?.name());
        Ok(())
    }
    /// `get_representation_item`.
    pub fn get_representation_item(
        &self,
    ) -> Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>> {
        let ri = Handle::new(StepReprRepresentationItem::new());
        ri.init(&self.name());
        ri.into_upcast()
    }
}

step_entity! {
    /// Base class for complex types (MEASURE_REPRESENTATION_ITEM, MEASURE_WITH_UNIT,
    /// QUALIFIED_REPRESENTATION_ITEM REPRESENTATION_ITEM,
    /// LENGTH_MEASURE_WITH_UNIT/PLANE_ANGLE_MEASURE_WITH_UNIT).
    custom_default StepReprReprItemAndMeasureWithUnitAndQRI, StepReprReprItemAndMeasureWithUnitAndQRIState, "StepReprReprItemAndMeasureWithUnitAndQRI", StepReprReprItemAndMeasureWithUnit, {
        my_qualified_representation_item: Handle<StepShapeQualifiedRepresentationItem>,
    }, bases: [crate::step_repr::StepReprRepresentationItem, StepReprReprItemAndMeasureWithUnit, StepReprReprItemAndMeasureWithUnitAndQRI]
}
impl Default for StepReprReprItemAndMeasureWithUnitAndQRI {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_qualified_representation_item =
                Handle::new(StepShapeQualifiedRepresentationItem::new());
        }
        object
    }
}
impl StepReprReprItemAndMeasureWithUnitAndQRI {
    /// `StepReprReprItemAndMeasureWithUnitAndQRI()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepReprReprItemAndMeasureWithUnitAndQRI {
    step_methods! {
        set set_qualified_representation_item = my_qualified_representation_item: Handle<StepShapeQualifiedRepresentationItem>;
        get get_qualified_representation_item = my_qualified_representation_item: Handle<StepShapeQualifiedRepresentationItem>;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        a_mwu: &Handle<dyn crate::StepEntityBase<StepBasicMeasureWithUnit>>,
        a_ri: &Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>,
        a_qri: &Handle<StepShapeQualifiedRepresentationItem>,
    ) -> Result<(), Failure> {
        self.parent.init(a_mwu, a_ri)?;
        self.state.write().my_qualified_representation_item = a_qri.clone();
        Ok(())
    }
}

step_entity! {
    /// Repr item and plane angle measure with unit.
    custom_default StepReprReprItemAndPlaneAngleMeasureWithUnit, StepReprReprItemAndPlaneAngleMeasureWithUnitState, "StepReprReprItemAndPlaneAngleMeasureWithUnit", StepReprReprItemAndMeasureWithUnit, {
        my_plane_angle_measure_with_unit: Handle<StepBasicPlaneAngleMeasureWithUnit>,
    }, bases: [crate::step_repr::StepReprRepresentationItem, StepReprReprItemAndMeasureWithUnit]
}
impl Default for StepReprReprItemAndPlaneAngleMeasureWithUnit {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_plane_angle_measure_with_unit =
                Handle::new(StepBasicPlaneAngleMeasureWithUnit::new());
        }
        object
    }
}
impl StepReprReprItemAndPlaneAngleMeasureWithUnit {
    /// `StepReprReprItemAndPlaneAngleMeasureWithUnit()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepReprReprItemAndPlaneAngleMeasureWithUnit {
    step_methods! {
        set set_plane_angle_measure_with_unit = my_plane_angle_measure_with_unit: Handle<StepBasicPlaneAngleMeasureWithUnit>;
        get get_plane_angle_measure_with_unit = my_plane_angle_measure_with_unit: Handle<StepBasicPlaneAngleMeasureWithUnit>;
    }
}

step_entity! {
    /// Repr item and plane angle measure with unit and QRI.
    custom_default StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI, StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRIState, "StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI", StepReprReprItemAndMeasureWithUnitAndQRI, {
        my_plane_angle_measure_with_unit: Handle<StepBasicPlaneAngleMeasureWithUnit>,
    }, bases: [crate::step_repr::StepReprRepresentationItem, StepReprReprItemAndMeasureWithUnit, StepReprReprItemAndMeasureWithUnitAndQRI]
}
impl Default for StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_plane_angle_measure_with_unit =
                Handle::new(StepBasicPlaneAngleMeasureWithUnit::new());
        }
        object
    }
}
impl StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI {
    /// `StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI {
    step_methods! {
        set set_plane_angle_measure_with_unit = my_plane_angle_measure_with_unit: Handle<StepBasicPlaneAngleMeasureWithUnit>;
        get get_plane_angle_measure_with_unit = my_plane_angle_measure_with_unit: Handle<StepBasicPlaneAngleMeasureWithUnit>;
    }
}

step_entity! {
    /// Representation.
    StepReprRepresentation, StepReprRepresentationState, "StepReprRepresentation", StandardTransient, {
        name: Handle<HAsciiString>,
        items: Handle<HArray1<Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>>>,
        context_of_items: Handle<dyn crate::StepEntityBase<StepReprRepresentationContext>>,
    }, bases: [crate::step_repr::StepReprRepresentation]
}
impl StepReprRepresentation {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_items = items: Handle<HArray1<Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>>>;
        get items = items: Handle<HArray1<Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>>>;
        item items_value = items: Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>;
        len nb_items = items;
        set set_context_of_items = context_of_items: Handle<dyn crate::StepEntityBase<StepReprRepresentationContext>>;
        get context_of_items = context_of_items: Handle<dyn crate::StepEntityBase<StepReprRepresentationContext>>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_items: &Handle<HArray1<Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>>>,
        a_context_of_items: &Handle<dyn crate::StepEntityBase<StepReprRepresentationContext>>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.items = a_items.clone();
        state.context_of_items = a_context_of_items.clone();
    }
}

step_entity! {
    /// Representation context.
    StepReprRepresentationContext, StepReprRepresentationContextState, "StepReprRepresentationContext", StandardTransient, {
        context_identifier: Handle<HAsciiString>,
        context_type: Handle<HAsciiString>,
    }, bases: [crate::step_repr::StepReprRepresentationContext]
}
impl StepReprRepresentationContext {
    step_methods! {
        set set_context_identifier = context_identifier: Handle<HAsciiString>;
        get context_identifier = context_identifier: Handle<HAsciiString>;
        set set_context_type = context_type: Handle<HAsciiString>;
        get context_type = context_type: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.context_identifier = a_context_identifier.clone();
        state.context_type = a_context_type.clone();
    }
}

step_entity! {
    /// Representation of STEP entity RepresentationContextReference
    StepReprRepresentationContextReference, StepReprRepresentationContextReferenceState, "StepReprRepresentationContextReference", StandardTransient, {
        my_context_identifier: Handle<HAsciiString>,
    }
}
impl StepReprRepresentationContextReference {
    step_methods! {
        set init = my_context_identifier: Handle<HAsciiString>;
        get context_identifier = my_context_identifier: Handle<HAsciiString>;
        set set_context_identifier = my_context_identifier: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Representation item.
    StepReprRepresentationItem, StepReprRepresentationItemState, "StepReprRepresentationItem", StandardTransient, {
        name: Handle<HAsciiString>,
    }, bases: [crate::step_repr::StepReprRepresentationItem]
}
impl StepReprRepresentationItem {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Representation map.
    StepReprRepresentationMap, StepReprRepresentationMapState, "StepReprRepresentationMap", StandardTransient, {
        mapping_origin: Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>,
        mapped_representation: Handle<dyn crate::StepEntityBase<StepReprRepresentation>>,
    }
}
impl StepReprRepresentationMap {
    step_methods! {
        set set_mapping_origin = mapping_origin: Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>;
        get mapping_origin = mapping_origin: Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>;
        set set_mapped_representation = mapped_representation: Handle<dyn crate::StepEntityBase<StepReprRepresentation>>;
        get mapped_representation = mapped_representation: Handle<dyn crate::StepEntityBase<StepReprRepresentation>>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_mapping_origin: &Handle<dyn crate::StepEntityBase<StepReprRepresentationItem>>,
        a_mapped_representation: &Handle<dyn crate::StepEntityBase<StepReprRepresentation>>,
    ) {
        let mut state = self.state.write();
        state.mapping_origin = a_mapping_origin.clone();
        state.mapped_representation = a_mapped_representation.clone();
    }
}
