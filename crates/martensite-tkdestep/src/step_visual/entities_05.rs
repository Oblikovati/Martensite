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
    /// Pre defined colour.
    custom_default StepVisualPreDefinedColour, StepVisualPreDefinedColourState, "StepVisualPreDefinedColour", StepVisualColour, {
        my_item: Handle<StepVisualPreDefinedItem>,
    }
}
impl Default for StepVisualPreDefinedColour {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_item = Handle::new(StepVisualPreDefinedItem::new());
        }
        object
    }
}
impl StepVisualPreDefinedColour {
    /// `StepVisualPreDefinedColour()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepVisualPreDefinedColour {
    step_methods! {
        set set_pre_defined_item = my_item: Handle<StepVisualPreDefinedItem>;
        get get_pre_defined_item = my_item: Handle<StepVisualPreDefinedItem>;
    }
}

step_entity! {
    /// Pre defined curve font.
    StepVisualPreDefinedCurveFont, StepVisualPreDefinedCurveFontState, "StepVisualPreDefinedCurveFont", StepVisualPreDefinedItem, {
    }
}

step_entity! {
    /// Pre defined item.
    StepVisualPreDefinedItem, StepVisualPreDefinedItemState, "StepVisualPreDefinedItem", StandardTransient, {
        name: Handle<HAsciiString>,
    }
}
impl StepVisualPreDefinedItem {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Pre defined text font.
    StepVisualPreDefinedTextFont, StepVisualPreDefinedTextFontState, "StepVisualPreDefinedTextFont", StepVisualPreDefinedItem, {
    }
}

step_entity! {
    /// Presentation area.
    StepVisualPresentationArea, StepVisualPresentationAreaState, "StepVisualPresentationArea", StepVisualPresentationRepresentation, {
    }
}

step_entity! {
    /// Presentation layer assignment.
    StepVisualPresentationLayerAssignment, StepVisualPresentationLayerAssignmentState, "StepVisualPresentationLayerAssignment", StandardTransient, {
        name: Handle<HAsciiString>,
        description: Handle<HAsciiString>,
        assigned_items: Handle<HArray1<StepVisualLayeredItem>>,
    }
}
impl StepVisualPresentationLayerAssignment {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_description = description: Handle<HAsciiString>;
        get description = description: Handle<HAsciiString>;
        set set_assigned_items = assigned_items: Handle<HArray1<StepVisualLayeredItem>>;
        get assigned_items = assigned_items: Handle<HArray1<StepVisualLayeredItem>>;
        item assigned_items_value = assigned_items: StepVisualLayeredItem;
        len nb_assigned_items = assigned_items;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_assigned_items: &Handle<HArray1<StepVisualLayeredItem>>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.description = a_description.clone();
        state.assigned_items = a_assigned_items.clone();
    }
}

step_entity! {
    /// Added from StepVisual Rev2 to Rev4
    StepVisualPresentationLayerUsage, StepVisualPresentationLayerUsageState, "StepVisualPresentationLayerUsage", StandardTransient, {
        the_assignment: Handle<StepVisualPresentationLayerAssignment>,
        the_presentation: Handle<StepVisualPresentationRepresentation>,
    }
}
impl StepVisualPresentationLayerUsage {
    step_methods! {
        set set_assignment = the_assignment: Handle<StepVisualPresentationLayerAssignment>;
        get assignment = the_assignment: Handle<StepVisualPresentationLayerAssignment>;
        set set_presentation = the_presentation: Handle<StepVisualPresentationRepresentation>;
        get presentation = the_presentation: Handle<StepVisualPresentationRepresentation>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_assignment: &Handle<StepVisualPresentationLayerAssignment>,
        a_presentation: &Handle<StepVisualPresentationRepresentation>,
    ) {
        let mut state = self.state.write();
        state.the_assignment = a_assignment.clone();
        state.the_presentation = a_presentation.clone();
    }
}

step_entity! {
    /// Presentation representation.
    StepVisualPresentationRepresentation, StepVisualPresentationRepresentationState, "StepVisualPresentationRepresentation", StepReprRepresentation, {
    }
}

step_entity! {
    /// Presentation set.
    StepVisualPresentationSet, StepVisualPresentationSetState, "StepVisualPresentationSet", StandardTransient, {
    }
}

step_entity! {
    /// Presentation size.
    StepVisualPresentationSize, StepVisualPresentationSizeState, "StepVisualPresentationSize", StandardTransient, {
        unit: StepVisualPresentationSizeAssignmentSelect,
        size: Handle<StepVisualPlanarBox>,
    }
}
impl StepVisualPresentationSize {
    step_methods! {
        set set_unit = unit: StepVisualPresentationSizeAssignmentSelect;
        get unit = unit: StepVisualPresentationSizeAssignmentSelect;
        set set_size = size: Handle<StepVisualPlanarBox>;
        get size = size: Handle<StepVisualPlanarBox>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_unit: &StepVisualPresentationSizeAssignmentSelect,
        a_size: &Handle<StepVisualPlanarBox>,
    ) {
        let mut state = self.state.write();
        state.unit = a_unit.clone();
        state.size = a_size.clone();
    }
}

step_entity! {
    /// Presentation style assignment.
    StepVisualPresentationStyleAssignment, StepVisualPresentationStyleAssignmentState, "StepVisualPresentationStyleAssignment", StandardTransient, {
        styles: Handle<HArray1<StepVisualPresentationStyleSelect>>,
    }
}
impl StepVisualPresentationStyleAssignment {
    step_methods! {
        set init = styles: Handle<HArray1<StepVisualPresentationStyleSelect>>;
        set set_styles = styles: Handle<HArray1<StepVisualPresentationStyleSelect>>;
        get styles = styles: Handle<HArray1<StepVisualPresentationStyleSelect>>;
        item styles_value = styles: StepVisualPresentationStyleSelect;
    }
    /// `nb_styles`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_styles(&self) -> Result<i32, Failure> {
        let field_styles = self.state.read().styles.clone();
        Ok(if self.state.read().styles.is_null() {
            0
        } else {
            step_live(
                &field_styles,
                "StepVisualPresentationStyleAssignment::NbStyles",
            )?
            .array1()
            .length()
        })
    }
}

step_entity! {
    /// Presentation style by context.
    StepVisualPresentationStyleByContext, StepVisualPresentationStyleByContextState, "StepVisualPresentationStyleByContext", StepVisualPresentationStyleAssignment, {
        style_context: StepVisualStyleContextSelect,
    }
}
impl StepVisualPresentationStyleByContext {
    step_methods! {
        set set_style_context = style_context: StepVisualStyleContextSelect;
        get style_context = style_context: StepVisualStyleContextSelect;
    }
    /// `init`.
    pub fn init(
        &self,
        a_styles: &Handle<HArray1<StepVisualPresentationStyleSelect>>,
        a_style_context: &StepVisualStyleContextSelect,
    ) {
        self.state.write().style_context = a_style_context.clone();
        self.parent.init(a_styles);
    }
}

step_entity! {
    /// Presentation view.
    StepVisualPresentationView, StepVisualPresentationViewState, "StepVisualPresentationView", StepVisualPresentationRepresentation, {
    }
}

step_entity! {
    /// Presented item.
    StepVisualPresentedItem, StepVisualPresentedItemState, "StepVisualPresentedItem", StandardTransient, {
    }
}

step_entity! {
    /// Added from StepVisual Rev2 to Rev4
    StepVisualPresentedItemRepresentation, StepVisualPresentedItemRepresentationState, "StepVisualPresentedItemRepresentation", StandardTransient, {
        the_presentation: StepVisualPresentationRepresentationSelect,
        the_item: Handle<StepVisualPresentedItem>,
    }
}
impl StepVisualPresentedItemRepresentation {
    step_methods! {
        set set_presentation = the_presentation: StepVisualPresentationRepresentationSelect;
        get presentation = the_presentation: StepVisualPresentationRepresentationSelect;
        set set_item = the_item: Handle<StepVisualPresentedItem>;
        get item = the_item: Handle<StepVisualPresentedItem>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_presentation: &StepVisualPresentationRepresentationSelect,
        a_item: &Handle<StepVisualPresentedItem>,
    ) {
        let mut state = self.state.write();
        state.the_presentation = a_presentation.clone();
        state.the_item = a_item.clone();
    }
}

step_entity! {
    /// Representation of complex STEP entity RepositionedTessellatedGeometricSet
    StepVisualRepositionedTessellatedGeometricSet, StepVisualRepositionedTessellatedGeometricSetState, "StepVisualRepositionedTessellatedGeometricSet", StepVisualTessellatedGeometricSet, {
        my_location: Handle<StepGeomAxis2Placement3d>,
    }
}
impl StepVisualRepositionedTessellatedGeometricSet {
    step_methods! {
        get location = my_location: Handle<StepGeomAxis2Placement3d>;
        set set_location = my_location: Handle<StepGeomAxis2Placement3d>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_items: &ObjectHandle<Array1<'static, Handle<StepVisualTessellatedItem>>>,
        the_location: &Handle<StepGeomAxis2Placement3d>,
    ) {
        self.parent.init(the_name, the_items);
        self.state.write().my_location = the_location.clone();
    }
}

step_entity! {
    /// Representation of STEP entity RepositionedTessellatedItem
    StepVisualRepositionedTessellatedItem, StepVisualRepositionedTessellatedItemState, "StepVisualRepositionedTessellatedItem", StepVisualTessellatedItem, {
        my_location: Handle<StepGeomAxis2Placement3d>,
    }
}
impl StepVisualRepositionedTessellatedItem {
    step_methods! {
        get location = my_location: Handle<StepGeomAxis2Placement3d>;
        set set_location = my_location: Handle<StepGeomAxis2Placement3d>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_location: &Handle<StepGeomAxis2Placement3d>,
    ) {
        self.parent.init(the_name);
        self.state.write().my_location = the_location.clone();
    }
}
