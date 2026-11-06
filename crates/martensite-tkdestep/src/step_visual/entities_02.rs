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
    /// Annotation fill area occurrence.
    StepVisualAnnotationFillAreaOccurrence, StepVisualAnnotationFillAreaOccurrenceState, "StepVisualAnnotationFillAreaOccurrence", StepVisualAnnotationOccurrence, {
        my_fill_style_target: Handle<StepGeomGeometricRepresentationItem>,
    }
}
impl StepVisualAnnotationFillAreaOccurrence {
    step_methods! {
        get fill_style_target = my_fill_style_target: Handle<StepGeomGeometricRepresentationItem>;
        set set_fill_style_target = my_fill_style_target: Handle<StepGeomGeometricRepresentationItem>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_styles: &Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>,
        the_item: &Handle<StandardTransient>,
        the_fill_style_target: &Handle<StepGeomGeometricRepresentationItem>,
    ) {
        self.parent.init(the_name, the_styles, the_item);
        self.state.write().my_fill_style_target = the_fill_style_target.clone();
    }
}

step_entity! {
    /// Annotation occurrence.
    StepVisualAnnotationOccurrence, StepVisualAnnotationOccurrenceState, "StepVisualAnnotationOccurrence", StepVisualStyledItem, {
    }
}

step_entity! {
    /// Annotation plane.
    StepVisualAnnotationPlane, StepVisualAnnotationPlaneState, "StepVisualAnnotationPlane", StepVisualAnnotationOccurrence, {
        my_elements: Handle<HArray1<StepVisualAnnotationPlaneElement>>,
    }
}
impl StepVisualAnnotationPlane {
    step_methods! {
        get elements = my_elements: Handle<HArray1<StepVisualAnnotationPlaneElement>>;
        set set_elements = my_elements: Handle<HArray1<StepVisualAnnotationPlaneElement>>;
        len nb_elements = my_elements;
        item elements_value = my_elements: StepVisualAnnotationPlaneElement;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_styles: &Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>,
        the_item: &Handle<StandardTransient>,
        the_elements: &Handle<HArray1<StepVisualAnnotationPlaneElement>>,
    ) {
        self.parent.init(the_name, the_styles, the_item);
        self.state.write().my_elements = the_elements.clone();
    }
    /// `set_elements_value`.
    /// Sets Elements with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_elements_value(
        &self,
        the_num: i32,
        the_item: &StepVisualAnnotationPlaneElement,
    ) -> Result<(), Failure> {
        let field_my_elements = self.state.read().my_elements.clone();
        step_array1_set(
            &field_my_elements,
            the_num,
            the_item.clone(),
            "StepVisualAnnotationPlane::SetElementsValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Annotation text.
    StepVisualAnnotationText, StepVisualAnnotationTextState, "StepVisualAnnotationText", StepReprMappedItem, {
    }
}

step_entity! {
    /// Annotation text occurrence.
    StepVisualAnnotationTextOccurrence, StepVisualAnnotationTextOccurrenceState, "StepVisualAnnotationTextOccurrence", StepVisualAnnotationOccurrence, {
    }
}

step_entity! {
    /// Area in set.
    StepVisualAreaInSet, StepVisualAreaInSetState, "StepVisualAreaInSet", StandardTransient, {
        area: Handle<StepVisualPresentationArea>,
        in_set: Handle<StepVisualPresentationSet>,
    }
}
impl StepVisualAreaInSet {
    step_methods! {
        set set_area = area: Handle<StepVisualPresentationArea>;
        get area = area: Handle<StepVisualPresentationArea>;
        set set_in_set = in_set: Handle<StepVisualPresentationSet>;
        get in_set = in_set: Handle<StepVisualPresentationSet>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_area: &Handle<StepVisualPresentationArea>,
        a_in_set: &Handle<StepVisualPresentationSet>,
    ) {
        let mut state = self.state.write();
        state.area = a_area.clone();
        state.in_set = a_in_set.clone();
    }
}

step_entity! {
    /// Background colour.
    StepVisualBackgroundColour, StepVisualBackgroundColourState, "StepVisualBackgroundColour", StepVisualColour, {
        presentation: StepVisualAreaOrView,
    }
}
impl StepVisualBackgroundColour {
    step_methods! {
        set init = presentation: StepVisualAreaOrView;
        set set_presentation = presentation: StepVisualAreaOrView;
        get presentation = presentation: StepVisualAreaOrView;
    }
}

step_entity! {
    /// Camera image.
    StepVisualCameraImage, StepVisualCameraImageState, "StepVisualCameraImage", StepReprMappedItem, {
    }
}

step_entity! {
    /// Camera image2d with scale.
    StepVisualCameraImage2dWithScale, StepVisualCameraImage2dWithScaleState, "StepVisualCameraImage2dWithScale", StepVisualCameraImage, {
    }
}

step_entity! {
    /// Camera image3d with scale.
    StepVisualCameraImage3dWithScale, StepVisualCameraImage3dWithScaleState, "StepVisualCameraImage3dWithScale", StepVisualCameraImage, {
    }
}

step_entity! {
    /// Camera model.
    StepVisualCameraModel, StepVisualCameraModelState, "StepVisualCameraModel", StepGeomGeometricRepresentationItem, {
    }
}

step_entity! {
    /// Camera model D 2.
    StepVisualCameraModelD2, StepVisualCameraModelD2State, "StepVisualCameraModelD2", StepVisualCameraModel, {
        view_window: Handle<StepVisualPlanarBox>,
        view_window_clipping: bool,
    }
}
impl StepVisualCameraModelD2 {
    step_methods! {
        set set_view_window = view_window: Handle<StepVisualPlanarBox>;
        get view_window = view_window: Handle<StepVisualPlanarBox>;
        set_copy set_view_window_clipping = view_window_clipping: bool;
        get view_window_clipping = view_window_clipping: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_view_window: &Handle<StepVisualPlanarBox>,
        a_view_window_clipping: bool,
    ) {
        let mut state = self.state.write();
        state.view_window = a_view_window.clone();
        state.view_window_clipping = a_view_window_clipping;
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Camera model D 3.
    StepVisualCameraModelD3, StepVisualCameraModelD3State, "StepVisualCameraModelD3", StepVisualCameraModel, {
        view_reference_system: Handle<StepGeomAxis2Placement3d>,
        perspective_of_volume: Handle<StepVisualViewVolume>,
    }
}
impl StepVisualCameraModelD3 {
    step_methods! {
        set set_view_reference_system = view_reference_system: Handle<StepGeomAxis2Placement3d>;
        get view_reference_system = view_reference_system: Handle<StepGeomAxis2Placement3d>;
        set set_perspective_of_volume = perspective_of_volume: Handle<StepVisualViewVolume>;
        get perspective_of_volume = perspective_of_volume: Handle<StepVisualViewVolume>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_view_reference_system: &Handle<StepGeomAxis2Placement3d>,
        a_perspective_of_volume: &Handle<StepVisualViewVolume>,
    ) {
        let mut state = self.state.write();
        state.view_reference_system = a_view_reference_system.clone();
        state.perspective_of_volume = a_perspective_of_volume.clone();
        drop(state);
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Camera model D 3 multi clipping.
    StepVisualCameraModelD3MultiClipping, StepVisualCameraModelD3MultiClippingState, "StepVisualCameraModelD3MultiClipping", StepVisualCameraModelD3, {
        my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>,
    }
}
impl StepVisualCameraModelD3MultiClipping {
    step_methods! {
        set set_shape_clipping = my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>;
        get shape_clipping = my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_view_reference_system: &Handle<StepGeomAxis2Placement3d>,
        the_perspective_of_volume: &Handle<StepVisualViewVolume>,
        the_shape_clipping: &Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>,
    ) {
        self.state.write().my_shape_clipping = the_shape_clipping.clone();
        self.parent.init(
            the_name,
            the_view_reference_system,
            the_perspective_of_volume,
        );
    }
}

step_entity! {
    /// Camera model D 3 multi clipping intersection.
    StepVisualCameraModelD3MultiClippingIntersection, StepVisualCameraModelD3MultiClippingIntersectionState, "StepVisualCameraModelD3MultiClippingIntersection", StepGeomGeometricRepresentationItem, {
        my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>,
    }
}
impl StepVisualCameraModelD3MultiClippingIntersection {
    step_methods! {
        set set_shape_clipping = my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>;
        get shape_clipping = my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_shape_clipping: &Handle<HArray1<StepVisualCameraModelD3MultiClippingInterectionSelect>>,
    ) {
        self.state.write().my_shape_clipping = the_shape_clipping.clone();
        self.parent.init(the_name);
    }
}

step_entity! {
    /// Camera model D 3 multi clipping union.
    StepVisualCameraModelD3MultiClippingUnion, StepVisualCameraModelD3MultiClippingUnionState, "StepVisualCameraModelD3MultiClippingUnion", StepGeomGeometricRepresentationItem, {
        my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingUnionSelect>>,
    }
}
impl StepVisualCameraModelD3MultiClippingUnion {
    step_methods! {
        set set_shape_clipping = my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingUnionSelect>>;
        get shape_clipping = my_shape_clipping: Handle<HArray1<StepVisualCameraModelD3MultiClippingUnionSelect>>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_shape_clipping: &Handle<HArray1<StepVisualCameraModelD3MultiClippingUnionSelect>>,
    ) {
        self.state.write().my_shape_clipping = the_shape_clipping.clone();
        self.parent.init(the_name);
    }
}

step_entity! {
    /// Camera usage.
    StepVisualCameraUsage, StepVisualCameraUsageState, "StepVisualCameraUsage", StepReprRepresentationMap, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    /// Complex STEP entity Characterized_Object & Characterized_Representation & Draughting_Model &
    /// Representation
    StepVisualCharacterizedObjAndRepresentationAndDraughtingModel, StepVisualCharacterizedObjAndRepresentationAndDraughtingModelState, "StepVisualCharacterizedObjAndRepresentationAndDraughtingModel", StepVisualDraughtingModel, {
    }
}

step_entity! {
    /// Colour.
    StepVisualColour, StepVisualColourState, "StepVisualColour", StandardTransient, {
    }
}

step_entity! {
    /// Colour rgb.
    StepVisualColourRgb, StepVisualColourRgbState, "StepVisualColourRgb", StepVisualColourSpecification, {
        red: f64,
        green: f64,
        blue: f64,
    }
}
impl StepVisualColourRgb {
    step_methods! {
        set_copy set_red = red: f64;
        get red = red: f64;
        set_copy set_green = green: f64;
        get green = green: f64;
        set_copy set_blue = blue: f64;
        get blue = blue: f64;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_red: f64, a_green: f64, a_blue: f64) {
        let mut state = self.state.write();
        state.red = a_red;
        state.green = a_green;
        state.blue = a_blue;
        drop(state);
        self.parent.init(a_name);
    }
}
