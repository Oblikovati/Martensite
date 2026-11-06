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

step_entity! {
    /// Colour specification.
    StepVisualColourSpecification, StepVisualColourSpecificationState, "StepVisualColourSpecification", StepVisualColour, {
        name: Handle<HAsciiString>,
    }
}
impl StepVisualColourSpecification {
    step_methods! {
        set init = name: Handle<HAsciiString>;
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Representation of STEP entity ComplexTriangulatedFace
    StepVisualComplexTriangulatedFace, StepVisualComplexTriangulatedFaceState, "StepVisualComplexTriangulatedFace", StepVisualTessellatedFace, {
        my_pnindex: Handle<HArray1<i32>>,
        my_triangle_strips: Handle<HArray1<Handle<StandardTransient>>>,
        my_triangle_fans: Handle<HArray1<Handle<StandardTransient>>>,
    }
}
impl StepVisualComplexTriangulatedFace {
    step_methods! {
        get pnindex = my_pnindex: Handle<HArray1<i32>>;
        set set_pnindex = my_pnindex: Handle<HArray1<i32>>;
        len nb_pnindex = my_pnindex;
        item pnindex_value = my_pnindex: i32;
        get triangle_strips = my_triangle_strips: Handle<HArray1<Handle<StandardTransient>>>;
        set set_triangle_strips = my_triangle_strips: Handle<HArray1<Handle<StandardTransient>>>;
        len nb_triangle_strips = my_triangle_strips;
        get triangle_fans = my_triangle_fans: Handle<HArray1<Handle<StandardTransient>>>;
        set set_triangle_fans = my_triangle_fans: Handle<HArray1<Handle<StandardTransient>>>;
        len nb_triangle_fans = my_triangle_fans;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_tessellated_face_coordinates: &Handle<StepVisualCoordinatesList>,
        the_tessellated_face_pnmax: i32,
        the_tessellated_face_normals: &Handle<HArray2<f64>>,
        the_has_tessellated_face_geometric_link: bool,
        the_tessellated_face_geometric_link: &StepVisualFaceOrSurface,
        the_pnindex: &Handle<HArray1<i32>>,
        the_triangle_strips: &Handle<HArray1<Handle<StandardTransient>>>,
        the_triangle_fans: &Handle<HArray1<Handle<StandardTransient>>>,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_tessellated_face_coordinates,
            the_tessellated_face_pnmax,
            the_tessellated_face_normals,
            the_has_tessellated_face_geometric_link,
            the_tessellated_face_geometric_link,
        );
        let mut state = self.state.write();
        state.my_pnindex = the_pnindex.clone();
        state.my_triangle_strips = the_triangle_strips.clone();
        state.my_triangle_fans = the_triangle_fans.clone();
    }
}

step_entity! {
    /// Representation of STEP entity ComplexTriangulatedSurfaceSet
    StepVisualComplexTriangulatedSurfaceSet, StepVisualComplexTriangulatedSurfaceSetState, "StepVisualComplexTriangulatedSurfaceSet", StepVisualTessellatedSurfaceSet, {
        my_pnindex: Handle<HArray1<i32>>,
        my_triangle_strips: Handle<HArray1<Handle<StandardTransient>>>,
        my_triangle_fans: Handle<HArray1<Handle<StandardTransient>>>,
    }
}
impl StepVisualComplexTriangulatedSurfaceSet {
    step_methods! {
        get pnindex = my_pnindex: Handle<HArray1<i32>>;
        set set_pnindex = my_pnindex: Handle<HArray1<i32>>;
        len nb_pnindex = my_pnindex;
        item pnindex_value = my_pnindex: i32;
        get triangle_strips = my_triangle_strips: Handle<HArray1<Handle<StandardTransient>>>;
        set set_triangle_strips = my_triangle_strips: Handle<HArray1<Handle<StandardTransient>>>;
        len nb_triangle_strips = my_triangle_strips;
        get triangle_fans = my_triangle_fans: Handle<HArray1<Handle<StandardTransient>>>;
        set set_triangle_fans = my_triangle_fans: Handle<HArray1<Handle<StandardTransient>>>;
        len nb_triangle_fans = my_triangle_fans;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_tessellated_surface_set_coordinates: &Handle<StepVisualCoordinatesList>,
        the_tessellated_surface_set_pnmax: i32,
        the_tessellated_surface_set_normals: &Handle<HArray2<f64>>,
        the_pnindex: &Handle<HArray1<i32>>,
        the_triangle_strips: &Handle<HArray1<Handle<StandardTransient>>>,
        the_triangle_fans: &Handle<HArray1<Handle<StandardTransient>>>,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_tessellated_surface_set_coordinates,
            the_tessellated_surface_set_pnmax,
            the_tessellated_surface_set_normals,
        );
        let mut state = self.state.write();
        state.my_pnindex = the_pnindex.clone();
        state.my_triangle_strips = the_triangle_strips.clone();
        state.my_triangle_fans = the_triangle_fans.clone();
    }
}

step_entity! {
    /// Composite text.
    StepVisualCompositeText, StepVisualCompositeTextState, "StepVisualCompositeText", StepGeomGeometricRepresentationItem, {
        collected_text: Handle<HArray1<StepVisualTextOrCharacter>>,
    }
}
impl StepVisualCompositeText {
    step_methods! {
        set set_collected_text = collected_text: Handle<HArray1<StepVisualTextOrCharacter>>;
        get collected_text = collected_text: Handle<HArray1<StepVisualTextOrCharacter>>;
        item collected_text_value = collected_text: StepVisualTextOrCharacter;
        len_checked nb_collected_text = collected_text;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_collected_text: &Handle<HArray1<StepVisualTextOrCharacter>>,
    ) {
        self.state.write().collected_text = a_collected_text.clone();
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Composite text with extent.
    StepVisualCompositeTextWithExtent, StepVisualCompositeTextWithExtentState, "StepVisualCompositeTextWithExtent", StepVisualCompositeText, {
        extent: Handle<StepVisualPlanarExtent>,
    }
}
impl StepVisualCompositeTextWithExtent {
    step_methods! {
        set set_extent = extent: Handle<StepVisualPlanarExtent>;
        get extent = extent: Handle<StepVisualPlanarExtent>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_collected_text: &Handle<HArray1<StepVisualTextOrCharacter>>,
        a_extent: &Handle<StepVisualPlanarExtent>,
    ) {
        self.state.write().extent = a_extent.clone();
        self.parent.init(a_name, a_collected_text);
    }
}

step_entity! {
    /// Context dependent invisibility.
    StepVisualContextDependentInvisibility, StepVisualContextDependentInvisibilityState, "StepVisualContextDependentInvisibility", StepVisualInvisibility, {
        presentation_context: StepVisualInvisibilityContext,
    }
}
impl StepVisualContextDependentInvisibility {
    step_methods! {
        set set_presentation_context = presentation_context: StepVisualInvisibilityContext;
        get presentation_context = presentation_context: StepVisualInvisibilityContext;
    }
    /// `init`.
    pub fn init(
        &self,
        a_invisible_items: &Handle<HArray1<StepVisualInvisibleItem>>,
        a_presentation_context: &StepVisualInvisibilityContext,
    ) {
        self.state.write().presentation_context = a_presentation_context.clone();
        self.parent.init(a_invisible_items);
    }
}

step_entity! {
    /// Context dependent over riding styled item.
    StepVisualContextDependentOverRidingStyledItem, StepVisualContextDependentOverRidingStyledItemState, "StepVisualContextDependentOverRidingStyledItem", StepVisualOverRidingStyledItem, {
        style_context: Handle<HArray1<StepVisualStyleContextSelect>>,
    }
}
impl StepVisualContextDependentOverRidingStyledItem {
    step_methods! {
        set set_style_context = style_context: Handle<HArray1<StepVisualStyleContextSelect>>;
        get style_context = style_context: Handle<HArray1<StepVisualStyleContextSelect>>;
        item style_context_value = style_context: StepVisualStyleContextSelect;
        len_checked nb_style_context = style_context;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_styles: &Handle<HArray1<Handle<StepVisualPresentationStyleAssignment>>>,
        a_item: &Handle<StandardTransient>,
        a_over_ridden_style: &Handle<StepVisualStyledItem>,
        a_style_context: &Handle<HArray1<StepVisualStyleContextSelect>>,
    ) {
        self.state.write().style_context = a_style_context.clone();
        self.parent
            .init(a_name, a_styles, a_item, a_over_ridden_style);
    }
}

step_entity! {
    /// Coordinates list.
    StepVisualCoordinatesList, StepVisualCoordinatesListState, "StepVisualCoordinatesList", StepVisualTessellatedItem, {
        my_points: Handle<HArray1<Xyz>>,
    }
}
impl StepVisualCoordinatesList {
    step_methods! {
        get points = my_points: Handle<HArray1<Xyz>>;
    }
    /// `init`.
    pub fn init(&self, the_name: &Handle<HAsciiString>, the_points: &Handle<HArray1<Xyz>>) {
        self.parent.parent.parent.init(the_name);
        self.state.write().my_points = the_points.clone();
    }
}

step_entity! {
    /// Representation of STEP entity CubicBezierTessellatedEdge
    StepVisualCubicBezierTessellatedEdge, StepVisualCubicBezierTessellatedEdgeState, "StepVisualCubicBezierTessellatedEdge", StepVisualTessellatedEdge, {
    }
}

step_entity! {
    /// Representation of STEP entity CubicBezierTriangulatedFace
    StepVisualCubicBezierTriangulatedFace, StepVisualCubicBezierTriangulatedFaceState, "StepVisualCubicBezierTriangulatedFace", StepVisualTessellatedFace, {
        my_ctriangles: Handle<HArray2<i32>>,
    }
}
impl StepVisualCubicBezierTriangulatedFace {
    step_methods! {
        get ctriangles = my_ctriangles: Handle<HArray2<i32>>;
        set set_ctriangles = my_ctriangles: Handle<HArray2<i32>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_tessellated_face_coordinates: &Handle<StepVisualCoordinatesList>,
        the_tessellated_face_pnmax: i32,
        the_tessellated_face_normals: &Handle<HArray2<f64>>,
        the_has_tessellated_face_geometric_link: bool,
        the_tessellated_face_geometric_link: &StepVisualFaceOrSurface,
        the_ctriangles: &Handle<HArray2<i32>>,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_tessellated_face_coordinates,
            the_tessellated_face_pnmax,
            the_tessellated_face_normals,
            the_has_tessellated_face_geometric_link,
            the_tessellated_face_geometric_link,
        );
        self.state.write().my_ctriangles = the_ctriangles.clone();
    }
    /// `nb_ctriangles`.
    /// Returns number of Ctriangles
    pub fn nb_ctriangles(&self) -> i32 {
        if self.state.read().my_ctriangles.is_null() {
            return 0;
        }
        let field_my_ctriangles = self.state.read().my_ctriangles.clone();
        field_my_ctriangles.array2().col_length()
    }
}

step_entity! {
    /// Curve style.
    StepVisualCurveStyle, StepVisualCurveStyleState, "StepVisualCurveStyle", StandardTransient, {
        name: Handle<HAsciiString>,
        curve_font: StepVisualCurveStyleFontSelect,
        curve_width: StepBasicSizeSelect,
        curve_colour: Handle<StepVisualColour>,
    }
}
impl StepVisualCurveStyle {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_curve_font = curve_font: StepVisualCurveStyleFontSelect;
        get curve_font = curve_font: StepVisualCurveStyleFontSelect;
        set set_curve_width = curve_width: StepBasicSizeSelect;
        get curve_width = curve_width: StepBasicSizeSelect;
        set set_curve_colour = curve_colour: Handle<StepVisualColour>;
        get curve_colour = curve_colour: Handle<StepVisualColour>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_curve_font: &StepVisualCurveStyleFontSelect,
        a_curve_width: &StepBasicSizeSelect,
        a_curve_colour: &Handle<StepVisualColour>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.curve_font = a_curve_font.clone();
        state.curve_width = a_curve_width.clone();
        state.curve_colour = a_curve_colour.clone();
    }
}

step_entity! {
    /// Curve style font.
    StepVisualCurveStyleFont, StepVisualCurveStyleFontState, "StepVisualCurveStyleFont", StandardTransient, {
        name: Handle<HAsciiString>,
        pattern_list: Handle<HArray1<Handle<StepVisualCurveStyleFontPattern>>>,
    }
}
impl StepVisualCurveStyleFont {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_pattern_list = pattern_list: Handle<HArray1<Handle<StepVisualCurveStyleFontPattern>>>;
        get pattern_list = pattern_list: Handle<HArray1<Handle<StepVisualCurveStyleFontPattern>>>;
        item pattern_list_value = pattern_list: Handle<StepVisualCurveStyleFontPattern>;
        len nb_pattern_list = pattern_list;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_pattern_list: &Handle<HArray1<Handle<StepVisualCurveStyleFontPattern>>>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.pattern_list = a_pattern_list.clone();
    }
}
