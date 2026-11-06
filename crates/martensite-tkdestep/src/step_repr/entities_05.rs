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
        my_measure_with_unit: Handle<StepBasicMeasureWithUnit>,
    }
}
impl Default for StepReprReprItemAndMeasureWithUnit {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            let mut state = object.state.write();
            state.my_measure_with_unit = Handle::new(StepBasicMeasureWithUnit::new());
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
        set set_measure_with_unit = my_measure_with_unit: Handle<StepBasicMeasureWithUnit>;
        get get_measure_with_unit = my_measure_with_unit: Handle<StepBasicMeasureWithUnit>;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        a_mwu: &Handle<StepBasicMeasureWithUnit>,
        a_ri: &Handle<StepReprRepresentationItem>,
    ) -> Result<(), Failure> {
        self.state.write().my_measure_with_unit = a_mwu.clone();
        self.set_name(&step_live(a_ri, "StepReprReprItemAndMeasureWithUnit::Init")?.name());
        Ok(())
    }
    /// `get_representation_item`.
    pub fn get_representation_item(&self) -> Handle<StepReprRepresentationItem> {
        let ri = Handle::new(StepReprRepresentationItem::new());
        ri.init(&self.name());
        ri.clone()
    }
}

step_entity! {
    /// Base class for complex types (MEASURE_REPRESENTATION_ITEM, MEASURE_WITH_UNIT,
    /// QUALIFIED_REPRESENTATION_ITEM REPRESENTATION_ITEM,
    /// LENGTH_MEASURE_WITH_UNIT/PLANE_ANGLE_MEASURE_WITH_UNIT).
    custom_default StepReprReprItemAndMeasureWithUnitAndQRI, StepReprReprItemAndMeasureWithUnitAndQRIState, "StepReprReprItemAndMeasureWithUnitAndQRI", StepReprReprItemAndMeasureWithUnit, {
        my_qualified_representation_item: Handle<StepShapeQualifiedRepresentationItem>,
    }
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
        a_mwu: &Handle<StepBasicMeasureWithUnit>,
        a_ri: &Handle<StepReprRepresentationItem>,
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
    }
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
    }
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
        items: Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        context_of_items: Handle<StepReprRepresentationContext>,
    }
}
impl StepReprRepresentation {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_items = items: Handle<HArray1<Handle<StepReprRepresentationItem>>>;
        get items = items: Handle<HArray1<Handle<StepReprRepresentationItem>>>;
        item items_value = items: Handle<StepReprRepresentationItem>;
        len nb_items = items;
        set set_context_of_items = context_of_items: Handle<StepReprRepresentationContext>;
        get context_of_items = context_of_items: Handle<StepReprRepresentationContext>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        a_context_of_items: &Handle<StepReprRepresentationContext>,
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
    }
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
    }
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
        mapping_origin: Handle<StepReprRepresentationItem>,
        mapped_representation: Handle<StepReprRepresentation>,
    }
}
impl StepReprRepresentationMap {
    step_methods! {
        set set_mapping_origin = mapping_origin: Handle<StepReprRepresentationItem>;
        get mapping_origin = mapping_origin: Handle<StepReprRepresentationItem>;
        set set_mapped_representation = mapped_representation: Handle<StepReprRepresentation>;
        get mapped_representation = mapped_representation: Handle<StepReprRepresentation>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_mapping_origin: &Handle<StepReprRepresentationItem>,
        a_mapped_representation: &Handle<StepReprRepresentation>,
    ) {
        let mut state = self.state.write();
        state.mapping_origin = a_mapping_origin.clone();
        state.mapped_representation = a_mapped_representation.clone();
    }
}
