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
    /// Styled item.
    StepVisualStyledItem, StepVisualStyledItemState, "StepVisualStyledItem", StepReprRepresentationItem, {
        my_styles: Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>,
        my_item: Handle<StandardTransient>,
        my_repr_item: Handle<StepReprRepresentationItem>,
    }
}
impl StepVisualStyledItem {
    step_methods! {
        set set_styles = my_styles: Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>;
        get styles = my_styles: Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>;
        item styles_value = my_styles: Handle<StepVisualPresentationStyleAssignment>;
        len nb_styles = my_styles;
        get item = my_repr_item: Handle<StepReprRepresentationItem>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_styles: &Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>,
        a_item: &Handle<StandardTransient>,
    ) {
        let mut state = self.state.write();
        state.my_styles = a_styles.clone();
        state.my_item = a_item.clone();
        state.my_repr_item = a_item.cast::<StepReprRepresentationItem>();
        drop(state);
        self.parent.init(a_name);
    }
}

step_entity! {
    /// Surface side style.
    StepVisualSurfaceSideStyle, StepVisualSurfaceSideStyleState, "StepVisualSurfaceSideStyle", StandardTransient, {
        name: Handle<HAsciiString>,
        styles: Handle<HArray1<StepVisualSurfaceStyleElementSelect>>,
    }
}
impl StepVisualSurfaceSideStyle {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_styles = styles: Handle<HArray1<StepVisualSurfaceStyleElementSelect>>;
        get styles = styles: Handle<HArray1<StepVisualSurfaceStyleElementSelect>>;
        item styles_value = styles: StepVisualSurfaceStyleElementSelect;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_styles: &Handle<HArray1<StepVisualSurfaceStyleElementSelect>>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.styles = a_styles.clone();
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
            step_live(&field_styles, "StepVisualSurfaceSideStyle::NbStyles")?
                .array1()
                .length()
        })
    }
}

step_entity! {
    /// Surface style boundary.
    StepVisualSurfaceStyleBoundary, StepVisualSurfaceStyleBoundaryState, "StepVisualSurfaceStyleBoundary", StandardTransient, {
        style_of_boundary: Handle<StepVisualCurveStyle>,
    }
}
impl StepVisualSurfaceStyleBoundary {
    step_methods! {
        set init = style_of_boundary: Handle<StepVisualCurveStyle>;
        set set_style_of_boundary = style_of_boundary: Handle<StepVisualCurveStyle>;
        get style_of_boundary = style_of_boundary: Handle<StepVisualCurveStyle>;
    }
}

step_entity! {
    /// Surface style control grid.
    StepVisualSurfaceStyleControlGrid, StepVisualSurfaceStyleControlGridState, "StepVisualSurfaceStyleControlGrid", StandardTransient, {
        style_of_control_grid: Handle<StepVisualCurveStyle>,
    }
}
impl StepVisualSurfaceStyleControlGrid {
    step_methods! {
        set init = style_of_control_grid: Handle<StepVisualCurveStyle>;
        set set_style_of_control_grid = style_of_control_grid: Handle<StepVisualCurveStyle>;
        get style_of_control_grid = style_of_control_grid: Handle<StepVisualCurveStyle>;
    }
}

step_entity! {
    /// Surface style fill area.
    StepVisualSurfaceStyleFillArea, StepVisualSurfaceStyleFillAreaState, "StepVisualSurfaceStyleFillArea", StandardTransient, {
        fill_area: Handle<StepVisualFillAreaStyle>,
    }
}
impl StepVisualSurfaceStyleFillArea {
    step_methods! {
        set init = fill_area: Handle<StepVisualFillAreaStyle>;
        set set_fill_area = fill_area: Handle<StepVisualFillAreaStyle>;
        get fill_area = fill_area: Handle<StepVisualFillAreaStyle>;
    }
}

step_entity! {
    /// Surface style parameter line.
    StepVisualSurfaceStyleParameterLine, StepVisualSurfaceStyleParameterLineState, "StepVisualSurfaceStyleParameterLine", StandardTransient, {
        style_of_parameter_lines: Handle<StepVisualCurveStyle>,
        direction_counts: Handle<HArray1<StepVisualDirectionCountSelect>>,
    }
}
impl StepVisualSurfaceStyleParameterLine {
    step_methods! {
        set set_style_of_parameter_lines = style_of_parameter_lines: Handle<StepVisualCurveStyle>;
        get style_of_parameter_lines = style_of_parameter_lines: Handle<StepVisualCurveStyle>;
        set set_direction_counts = direction_counts: Handle<HArray1<StepVisualDirectionCountSelect>>;
        get direction_counts = direction_counts: Handle<HArray1<StepVisualDirectionCountSelect>>;
        item direction_counts_value = direction_counts: StepVisualDirectionCountSelect;
        len_checked nb_direction_counts = direction_counts;
    }
    /// `init`.
    pub fn init(
        &self,
        a_style_of_parameter_lines: &Handle<StepVisualCurveStyle>,
        a_direction_counts: &Handle<HArray1<StepVisualDirectionCountSelect>>,
    ) {
        let mut state = self.state.write();
        state.style_of_parameter_lines = a_style_of_parameter_lines.clone();
        state.direction_counts = a_direction_counts.clone();
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceStyleReflectanceAmbient
    StepVisualSurfaceStyleReflectanceAmbient, StepVisualSurfaceStyleReflectanceAmbientState, "StepVisualSurfaceStyleReflectanceAmbient", StandardTransient, {
        my_ambient_reflectance: f64,
    }
}
impl StepVisualSurfaceStyleReflectanceAmbient {
    step_methods! {
        set_copy init = my_ambient_reflectance: f64;
        get ambient_reflectance = my_ambient_reflectance: f64;
        set_copy set_ambient_reflectance = my_ambient_reflectance: f64;
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceStyleReflectanceAmbientDiffuse
    StepVisualSurfaceStyleReflectanceAmbientDiffuse, StepVisualSurfaceStyleReflectanceAmbientDiffuseState, "StepVisualSurfaceStyleReflectanceAmbientDiffuse", StepVisualSurfaceStyleReflectanceAmbient, {
        my_diffuse_reflectance: f64,
    }
}
impl StepVisualSurfaceStyleReflectanceAmbientDiffuse {
    step_methods! {
        get diffuse_reflectance = my_diffuse_reflectance: f64;
        set_copy set_diffuse_reflectance = my_diffuse_reflectance: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(&self, the_ambient_reflectance: f64, the_diffuse_reflectance: f64) {
        self.parent.init(the_ambient_reflectance);
        self.state.write().my_diffuse_reflectance = the_diffuse_reflectance;
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceStyleReflectanceAmbientDiffuseSpecular
    StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular, StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecularState, "StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular", StepVisualSurfaceStyleReflectanceAmbientDiffuse, {
        my_specular_reflectance: f64,
        my_specular_exponent: f64,
        my_specular_colour: Handle<StepVisualColour>,
    }
}
impl StepVisualSurfaceStyleReflectanceAmbientDiffuseSpecular {
    step_methods! {
        get specular_reflectance = my_specular_reflectance: f64;
        set_copy set_specular_reflectance = my_specular_reflectance: f64;
        get specular_exponent = my_specular_exponent: f64;
        set_copy set_specular_exponent = my_specular_exponent: f64;
        get specular_colour = my_specular_colour: Handle<StepVisualColour>;
        set set_specular_colour = my_specular_colour: Handle<StepVisualColour>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_ambient_reflectance: f64,
        the_diffuse_reflectance: f64,
        the_specular_reflectance: f64,
        the_specular_exponent: f64,
        the_specular_colour: &Handle<StepVisualColour>,
    ) {
        self.parent
            .init(the_ambient_reflectance, the_diffuse_reflectance);
        let mut state = self.state.write();
        state.my_specular_reflectance = the_specular_reflectance;
        state.my_specular_exponent = the_specular_exponent;
        state.my_specular_colour = the_specular_colour.clone();
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceStyleRendering
    StepVisualSurfaceStyleRendering, StepVisualSurfaceStyleRenderingState, "StepVisualSurfaceStyleRendering", StandardTransient, {
        my_rendering_method: StepVisualShadingSurfaceMethod,
        my_surface_colour: Handle<StepVisualColour>,
    }
}
impl StepVisualSurfaceStyleRendering {
    step_methods! {
        get rendering_method = my_rendering_method: StepVisualShadingSurfaceMethod;
        set_copy set_rendering_method = my_rendering_method: StepVisualShadingSurfaceMethod;
        get surface_colour = my_surface_colour: Handle<StepVisualColour>;
        set set_surface_colour = my_surface_colour: Handle<StepVisualColour>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_rendering_method: StepVisualShadingSurfaceMethod,
        the_surface_colour: &Handle<StepVisualColour>,
    ) {
        let mut state = self.state.write();
        state.my_rendering_method = the_rendering_method;
        state.my_surface_colour = the_surface_colour.clone();
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceStyleRenderingWithProperties
    StepVisualSurfaceStyleRenderingWithProperties, StepVisualSurfaceStyleRenderingWithPropertiesState, "StepVisualSurfaceStyleRenderingWithProperties", StepVisualSurfaceStyleRendering, {
        my_properties: Handle<HArray1<StepVisualRenderingPropertiesSelect>>,
    }
}
impl StepVisualSurfaceStyleRenderingWithProperties {
    step_methods! {
        get properties = my_properties: Handle<HArray1<StepVisualRenderingPropertiesSelect>>;
        set set_properties = my_properties: Handle<HArray1<StepVisualRenderingPropertiesSelect>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_surface_style_rendering_rendering_method: StepVisualShadingSurfaceMethod,
        the_surface_style_rendering_surface_colour: &Handle<StepVisualColour>,
        the_properties: &Handle<HArray1<StepVisualRenderingPropertiesSelect>>,
    ) {
        self.parent.init(
            the_surface_style_rendering_rendering_method,
            the_surface_style_rendering_surface_colour,
        );
        self.state.write().my_properties = the_properties.clone();
    }
}

step_entity! {
    /// Surface style segmentation curve.
    StepVisualSurfaceStyleSegmentationCurve, StepVisualSurfaceStyleSegmentationCurveState, "StepVisualSurfaceStyleSegmentationCurve", StandardTransient, {
        style_of_segmentation_curve: Handle<StepVisualCurveStyle>,
    }
}
impl StepVisualSurfaceStyleSegmentationCurve {
    step_methods! {
        set init = style_of_segmentation_curve: Handle<StepVisualCurveStyle>;
        set set_style_of_segmentation_curve = style_of_segmentation_curve: Handle<StepVisualCurveStyle>;
        get style_of_segmentation_curve = style_of_segmentation_curve: Handle<StepVisualCurveStyle>;
    }
}

step_entity! {
    /// Surface style silhouette.
    StepVisualSurfaceStyleSilhouette, StepVisualSurfaceStyleSilhouetteState, "StepVisualSurfaceStyleSilhouette", StandardTransient, {
        style_of_silhouette: Handle<StepVisualCurveStyle>,
    }
}
impl StepVisualSurfaceStyleSilhouette {
    step_methods! {
        set init = style_of_silhouette: Handle<StepVisualCurveStyle>;
        set set_style_of_silhouette = style_of_silhouette: Handle<StepVisualCurveStyle>;
        get style_of_silhouette = style_of_silhouette: Handle<StepVisualCurveStyle>;
    }
}
