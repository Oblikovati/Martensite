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
use crate::step_geom::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Curve style font pattern.
    StepVisualCurveStyleFontPattern, StepVisualCurveStyleFontPatternState, "StepVisualCurveStyleFontPattern", StandardTransient, {
        visible_segment_length: f64,
        invisible_segment_length: f64,
    }
}
impl StepVisualCurveStyleFontPattern {
    step_methods! {
        set_copy set_visible_segment_length = visible_segment_length: f64;
        get visible_segment_length = visible_segment_length: f64;
        set_copy set_invisible_segment_length = invisible_segment_length: f64;
        get invisible_segment_length = invisible_segment_length: f64;
    }
    /// `init`.
    pub fn init(&self, a_visible_segment_length: f64, a_invisible_segment_length: f64) {
        let mut state = self.state.write();
        state.visible_segment_length = a_visible_segment_length;
        state.invisible_segment_length = a_invisible_segment_length;
    }
}

step_entity! {
    /// Draughting annotation occurrence.
    StepVisualDraughtingAnnotationOccurrence, StepVisualDraughtingAnnotationOccurrenceState, "StepVisualDraughtingAnnotationOccurrence", StepVisualAnnotationOccurrence, {
    }
}

step_entity! {
    /// Draughting callout.
    StepVisualDraughtingCallout, StepVisualDraughtingCalloutState, "StepVisualDraughtingCallout", StepGeomGeometricRepresentationItem, {
        my_contents: Handle<HArray1<StepVisualDraughtingCalloutElement>>,
    }
}
impl StepVisualDraughtingCallout {
    step_methods! {
        get contents = my_contents: Handle<HArray1<StepVisualDraughtingCalloutElement>>;
        set set_contents = my_contents: Handle<HArray1<StepVisualDraughtingCalloutElement>>;
        len nb_contents = my_contents;
        item contents_value = my_contents: StepVisualDraughtingCalloutElement;
    }
    /// `init`.
    /// Init
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_contents: &Handle<HArray1<StepVisualDraughtingCalloutElement>>,
    ) {
        self.parent.init(the_name);
        self.state.write().my_contents = the_contents.clone();
    }
    /// `set_contents_value`.
    /// Sets Contents with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_contents_value(
        &self,
        the_num: i32,
        the_item: &StepVisualDraughtingCalloutElement,
    ) -> Result<(), Failure> {
        let field_my_contents = self.state.read().my_contents.clone();
        step_array1_set(
            &field_my_contents,
            the_num,
            the_item.clone(),
            "StepVisualDraughtingCallout::SetContentsValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Representation of STEP entity DraughtingModel
    StepVisualDraughtingModel, StepVisualDraughtingModelState, "StepVisualDraughtingModel", StepReprRepresentation, {
    }
}

step_entity! {
    /// Draughting pre defined colour.
    StepVisualDraughtingPreDefinedColour, StepVisualDraughtingPreDefinedColourState, "StepVisualDraughtingPreDefinedColour", StepVisualPreDefinedColour, {
    }
}

step_entity! {
    /// Draughting pre defined curve font.
    StepVisualDraughtingPreDefinedCurveFont, StepVisualDraughtingPreDefinedCurveFontState, "StepVisualDraughtingPreDefinedCurveFont", StepVisualPreDefinedCurveFont, {
    }
}

step_entity! {
    /// Representation of STEP entity ExternallyDefinedCurveFont
    StepVisualExternallyDefinedCurveFont, StepVisualExternallyDefinedCurveFontState, "StepVisualExternallyDefinedCurveFont", StepBasicExternallyDefinedItem, {
    }
}

step_entity! {
    /// Representation of STEP entity ExternallyDefinedTextFont
    StepVisualExternallyDefinedTextFont, StepVisualExternallyDefinedTextFontState, "StepVisualExternallyDefinedTextFont", StepBasicExternallyDefinedItem, {
    }
}

step_entity! {
    /// Fill area style.
    StepVisualFillAreaStyle, StepVisualFillAreaStyleState, "StepVisualFillAreaStyle", StandardTransient, {
        name: Handle<HAsciiString>,
        fill_styles: Handle<HArray1<StepVisualFillStyleSelect>>,
    }
}
impl StepVisualFillAreaStyle {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_fill_styles = fill_styles: Handle<HArray1<StepVisualFillStyleSelect>>;
        get fill_styles = fill_styles: Handle<HArray1<StepVisualFillStyleSelect>>;
        item fill_styles_value = fill_styles: StepVisualFillStyleSelect;
        len nb_fill_styles = fill_styles;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_fill_styles: &Handle<HArray1<StepVisualFillStyleSelect>>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.fill_styles = a_fill_styles.clone();
    }
}

step_entity! {
    /// Fill area style colour.
    StepVisualFillAreaStyleColour, StepVisualFillAreaStyleColourState, "StepVisualFillAreaStyleColour", StandardTransient, {
        name: Handle<HAsciiString>,
        fill_colour: Handle<StepVisualColour>,
    }
}
impl StepVisualFillAreaStyleColour {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_fill_colour = fill_colour: Handle<StepVisualColour>;
        get fill_colour = fill_colour: Handle<StepVisualColour>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_fill_colour: &Handle<StepVisualColour>) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.fill_colour = a_fill_colour.clone();
    }
}

step_entity! {
    /// Invisibility.
    StepVisualInvisibility, StepVisualInvisibilityState, "StepVisualInvisibility", StandardTransient, {
        invisible_items: Handle<HArray1<StepVisualInvisibleItem>>,
    }
}
impl StepVisualInvisibility {
    step_methods! {
        set init = invisible_items: Handle<HArray1<StepVisualInvisibleItem>>;
        set set_invisible_items = invisible_items: Handle<HArray1<StepVisualInvisibleItem>>;
        get invisible_items = invisible_items: Handle<HArray1<StepVisualInvisibleItem>>;
        item invisible_items_value = invisible_items: StepVisualInvisibleItem;
        len_checked nb_invisible_items = invisible_items;
    }
}

step_entity! {
    /// Mechanical design geometric presentation area.
    StepVisualMechanicalDesignGeometricPresentationArea, StepVisualMechanicalDesignGeometricPresentationAreaState, "StepVisualMechanicalDesignGeometricPresentationArea", StepVisualPresentationArea, {
    }
}

step_entity! {
    /// Mechanical design geometric presentation representation.
    StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualMechanicalDesignGeometricPresentationRepresentationState, "StepVisualMechanicalDesignGeometricPresentationRepresentation", StepVisualPresentationRepresentation, {
    }
}

step_entity! {
    /// Over riding styled item.
    StepVisualOverRidingStyledItem, StepVisualOverRidingStyledItemState, "StepVisualOverRidingStyledItem", StepVisualStyledItem, {
        over_ridden_style: Handle<StepVisualStyledItem>,
    }
}
impl StepVisualOverRidingStyledItem {
    step_methods! {
        set set_over_ridden_style = over_ridden_style: Handle<StepVisualStyledItem>;
        get over_ridden_style = over_ridden_style: Handle<StepVisualStyledItem>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_styles: &Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>,
        a_item: &Handle<StandardTransient>,
        a_over_ridden_style: &Handle<StepVisualStyledItem>,
    ) {
        self.state.write().over_ridden_style = a_over_ridden_style.clone();
        self.parent.init(a_name, a_styles, a_item);
    }
}

step_entity! {
    /// Planar box.
    StepVisualPlanarBox, StepVisualPlanarBoxState, "StepVisualPlanarBox", StepVisualPlanarExtent, {
        placement: StepGeomAxis2Placement,
    }
}
impl StepVisualPlanarBox {
    step_methods! {
        set set_placement = placement: StepGeomAxis2Placement;
        get placement = placement: StepGeomAxis2Placement;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_size_in_x: f64,
        a_size_in_y: f64,
        a_placement: &StepGeomAxis2Placement,
    ) {
        self.state.write().placement = a_placement.clone();
        self.parent.init(a_name, a_size_in_x, a_size_in_y);
    }
}

step_entity! {
    /// Planar extent.
    StepVisualPlanarExtent, StepVisualPlanarExtentState, "StepVisualPlanarExtent", StepGeomGeometricRepresentationItem, {
        size_in_x: f64,
        size_in_y: f64,
    }
}
impl StepVisualPlanarExtent {
    step_methods! {
        set_copy set_size_in_x = size_in_x: f64;
        get size_in_x = size_in_x: f64;
        set_copy set_size_in_y = size_in_y: f64;
        get size_in_y = size_in_y: f64;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_size_in_x: f64, a_size_in_y: f64) {
        let mut state = self.state.write();
        state.size_in_x = a_size_in_x;
        state.size_in_y = a_size_in_y;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Point style.
    StepVisualPointStyle, StepVisualPointStyleState, "StepVisualPointStyle", StandardTransient, {
        name: Handle<HAsciiString>,
        marker: StepVisualMarkerSelect,
        marker_size: StepBasicSizeSelect,
        marker_colour: Handle<StepVisualColour>,
    }
}
impl StepVisualPointStyle {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_marker = marker: StepVisualMarkerSelect;
        get marker = marker: StepVisualMarkerSelect;
        set set_marker_size = marker_size: StepBasicSizeSelect;
        get marker_size = marker_size: StepBasicSizeSelect;
        set set_marker_colour = marker_colour: Handle<StepVisualColour>;
        get marker_colour = marker_colour: Handle<StepVisualColour>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_marker: &StepVisualMarkerSelect,
        a_marker_size: &StepBasicSizeSelect,
        a_marker_colour: &Handle<StepVisualColour>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.marker = a_marker.clone();
        state.marker_size = a_marker_size.clone();
        state.marker_colour = a_marker_colour.clone();
    }
}
