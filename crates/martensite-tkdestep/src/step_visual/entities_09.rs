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

step_entity! {
    /// Text style.
    StepVisualTextStyle, StepVisualTextStyleState, "StepVisualTextStyle", StandardTransient, {
        name: Handle<HAsciiString>,
        character_appearance: Handle<StepVisualTextStyleForDefinedFont>,
    }
}
impl StepVisualTextStyle {
    step_methods! {
        set set_name = name: Handle<HAsciiString>;
        get name = name: Handle<HAsciiString>;
        set set_character_appearance = character_appearance: Handle<StepVisualTextStyleForDefinedFont>;
        get character_appearance = character_appearance: Handle<StepVisualTextStyleForDefinedFont>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_character_appearance: &Handle<StepVisualTextStyleForDefinedFont>,
    ) {
        let mut state = self.state.write();
        state.name = a_name.clone();
        state.character_appearance = a_character_appearance.clone();
    }
}

step_entity! {
    /// Text style for defined font.
    StepVisualTextStyleForDefinedFont, StepVisualTextStyleForDefinedFontState, "StepVisualTextStyleForDefinedFont", StandardTransient, {
        text_colour: Handle<StepVisualColour>,
    }
}
impl StepVisualTextStyleForDefinedFont {
    step_methods! {
        set init = text_colour: Handle<StepVisualColour>;
        set set_text_colour = text_colour: Handle<StepVisualColour>;
        get text_colour = text_colour: Handle<StepVisualColour>;
    }
}

step_entity! {
    /// Text style with box characteristics.
    StepVisualTextStyleWithBoxCharacteristics, StepVisualTextStyleWithBoxCharacteristicsState, "StepVisualTextStyleWithBoxCharacteristics", StepVisualTextStyle, {
        characteristics: Handle<HArray1<StepVisualBoxCharacteristicSelect>>,
    }
}
impl StepVisualTextStyleWithBoxCharacteristics {
    step_methods! {
        set set_characteristics = characteristics: Handle<HArray1<StepVisualBoxCharacteristicSelect>>;
        get characteristics = characteristics: Handle<HArray1<StepVisualBoxCharacteristicSelect>>;
        item characteristics_value = characteristics: StepVisualBoxCharacteristicSelect;
        len_checked nb_characteristics = characteristics;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_character_appearance: &Handle<StepVisualTextStyleForDefinedFont>,
        a_characteristics: &Handle<HArray1<StepVisualBoxCharacteristicSelect>>,
    ) {
        self.state.write().characteristics = a_characteristics.clone();
        self.parent.init(a_name, a_character_appearance);
    }
}

step_entity! {
    /// Representation of STEP entity TriangulatedFace
    StepVisualTriangulatedFace, StepVisualTriangulatedFaceState, "StepVisualTriangulatedFace", StepVisualTessellatedFace, {
        my_pnindex: Handle<HArray1<i32>>,
        my_triangles: Handle<HArray2<i32>>,
    }
}
impl StepVisualTriangulatedFace {
    step_methods! {
        get pnindex = my_pnindex: Handle<HArray1<i32>>;
        set set_pnindex = my_pnindex: Handle<HArray1<i32>>;
        len nb_pnindex = my_pnindex;
        item pnindex_value = my_pnindex: i32;
        get triangles = my_triangles: Handle<HArray2<i32>>;
        set set_triangles = my_triangles: Handle<HArray2<i32>>;
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
        the_triangles: &Handle<HArray2<i32>>,
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
        state.my_triangles = the_triangles.clone();
    }
    /// `nb_triangles`.
    /// Returns number of Triangles
    pub fn nb_triangles(&self) -> i32 {
        if self.state.read().my_triangles.is_null() {
            return 0;
        }
        let field_my_triangles = self.state.read().my_triangles.clone();
        field_my_triangles.array2().col_length()
    }
}

step_entity! {
    /// Representation of STEP entity TriangulatedSurfaceSet
    StepVisualTriangulatedSurfaceSet, StepVisualTriangulatedSurfaceSetState, "StepVisualTriangulatedSurfaceSet", StepVisualTessellatedSurfaceSet, {
        my_pnindex: Handle<HArray1<i32>>,
        my_triangles: Handle<HArray2<i32>>,
    }
}
impl StepVisualTriangulatedSurfaceSet {
    step_methods! {
        get pnindex = my_pnindex: Handle<HArray1<i32>>;
        set set_pnindex = my_pnindex: Handle<HArray1<i32>>;
        len nb_pnindex = my_pnindex;
        item pnindex_value = my_pnindex: i32;
        get triangles = my_triangles: Handle<HArray2<i32>>;
        set set_triangles = my_triangles: Handle<HArray2<i32>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_tessellated_surface_set_coordinates: &Handle<StepVisualCoordinatesList>,
        the_tessellated_surface_set_pnmax: i32,
        the_tessellated_surface_set_normals: &Handle<HArray2<f64>>,
        the_pnindex: &Handle<HArray1<i32>>,
        the_triangles: &Handle<HArray2<i32>>,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_tessellated_surface_set_coordinates,
            the_tessellated_surface_set_pnmax,
            the_tessellated_surface_set_normals,
        );
        let mut state = self.state.write();
        state.my_pnindex = the_pnindex.clone();
        state.my_triangles = the_triangles.clone();
    }
    /// `nb_triangles`.
    /// Returns number of Triangles
    pub fn nb_triangles(&self) -> i32 {
        if self.state.read().my_triangles.is_null() {
            return 0;
        }
        let field_my_triangles = self.state.read().my_triangles.clone();
        field_my_triangles.array2().col_length()
    }
}

step_entity! {
    /// View volume.
    StepVisualViewVolume, StepVisualViewVolumeState, "StepVisualViewVolume", StandardTransient, {
        projection_type: StepVisualCentralOrParallel,
        projection_point: Handle<StepGeomCartesianPoint>,
        view_plane_distance: f64,
        front_plane_distance: f64,
        front_plane_clipping: bool,
        back_plane_distance: f64,
        back_plane_clipping: bool,
        view_volume_sides_clipping: bool,
        view_window: Handle<StepVisualPlanarBox>,
    }
}
impl StepVisualViewVolume {
    step_methods! {
        set_copy set_projection_type = projection_type: StepVisualCentralOrParallel;
        get projection_type = projection_type: StepVisualCentralOrParallel;
        set set_projection_point = projection_point: Handle<StepGeomCartesianPoint>;
        get projection_point = projection_point: Handle<StepGeomCartesianPoint>;
        set_copy set_view_plane_distance = view_plane_distance: f64;
        get view_plane_distance = view_plane_distance: f64;
        set_copy set_front_plane_distance = front_plane_distance: f64;
        get front_plane_distance = front_plane_distance: f64;
        set_copy set_front_plane_clipping = front_plane_clipping: bool;
        get front_plane_clipping = front_plane_clipping: bool;
        set_copy set_back_plane_distance = back_plane_distance: f64;
        get back_plane_distance = back_plane_distance: f64;
        set_copy set_back_plane_clipping = back_plane_clipping: bool;
        get back_plane_clipping = back_plane_clipping: bool;
        set_copy set_view_volume_sides_clipping = view_volume_sides_clipping: bool;
        get view_volume_sides_clipping = view_volume_sides_clipping: bool;
        set set_view_window = view_window: Handle<StepVisualPlanarBox>;
        get view_window = view_window: Handle<StepVisualPlanarBox>;
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        a_projection_type: StepVisualCentralOrParallel,
        a_projection_point: &Handle<StepGeomCartesianPoint>,
        a_view_plane_distance: f64,
        a_front_plane_distance: f64,
        a_front_plane_clipping: bool,
        a_back_plane_distance: f64,
        a_back_plane_clipping: bool,
        a_view_volume_sides_clipping: bool,
        a_view_window: &Handle<StepVisualPlanarBox>,
    ) {
        let mut state = self.state.write();
        state.projection_type = a_projection_type;
        state.projection_point = a_projection_point.clone();
        state.view_plane_distance = a_view_plane_distance;
        state.front_plane_distance = a_front_plane_distance;
        state.front_plane_clipping = a_front_plane_clipping;
        state.back_plane_distance = a_back_plane_distance;
        state.back_plane_clipping = a_back_plane_clipping;
        state.view_volume_sides_clipping = a_view_volume_sides_clipping;
        state.view_window = a_view_window.clone();
    }
}
