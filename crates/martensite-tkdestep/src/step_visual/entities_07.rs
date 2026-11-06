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
#[allow(unused_imports)]
use crate::step_shape::*;

step_entity! {
    /// Representation of STEP entity SurfaceStyleTransparent
    StepVisualSurfaceStyleTransparent, StepVisualSurfaceStyleTransparentState, "StepVisualSurfaceStyleTransparent", StandardTransient, {
        my_transparency: f64,
    }
}
impl StepVisualSurfaceStyleTransparent {
    step_methods! {
        set_copy init = my_transparency: f64;
        get transparency = my_transparency: f64;
        set_copy set_transparency = my_transparency: f64;
    }
}

step_entity! {
    /// Surface style usage.
    StepVisualSurfaceStyleUsage, StepVisualSurfaceStyleUsageState, "StepVisualSurfaceStyleUsage", StandardTransient, {
        side: StepVisualSurfaceSide,
        style: Handle<StepVisualSurfaceSideStyle>,
    }
}
impl StepVisualSurfaceStyleUsage {
    step_methods! {
        set_copy set_side = side: StepVisualSurfaceSide;
        get side = side: StepVisualSurfaceSide;
        set set_style = style: Handle<StepVisualSurfaceSideStyle>;
        get style = style: Handle<StepVisualSurfaceSideStyle>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_side: StepVisualSurfaceSide,
        a_style: &Handle<StepVisualSurfaceSideStyle>,
    ) {
        let mut state = self.state.write();
        state.side = a_side;
        state.style = a_style.clone();
    }
}

step_entity! {
    /// Template.
    StepVisualTemplate, StepVisualTemplateState, "StepVisualTemplate", StepReprRepresentation, {
    }
}

step_entity! {
    /// Template instance.
    StepVisualTemplateInstance, StepVisualTemplateInstanceState, "StepVisualTemplateInstance", StepReprMappedItem, {
    }
}

step_entity! {
    /// Tessellated annotation occurrence.
    StepVisualTessellatedAnnotationOccurrence, StepVisualTessellatedAnnotationOccurrenceState, "StepVisualTessellatedAnnotationOccurrence", StepVisualStyledItem, {
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedConnectingEdge
    StepVisualTessellatedConnectingEdge, StepVisualTessellatedConnectingEdgeState, "StepVisualTessellatedConnectingEdge", StepVisualTessellatedEdge, {
        my_smooth: StepDataLogical,
        my_face1: Handle<StepVisualTessellatedFace>,
        my_face2: Handle<StepVisualTessellatedFace>,
        my_line_strip_face1: Handle<HArray1<i32>>,
        my_line_strip_face2: Handle<HArray1<i32>>,
    }
}
impl StepVisualTessellatedConnectingEdge {
    step_methods! {
        get smooth = my_smooth: StepDataLogical;
        set_copy set_smooth = my_smooth: StepDataLogical;
        get face1 = my_face1: Handle<StepVisualTessellatedFace>;
        set set_face1 = my_face1: Handle<StepVisualTessellatedFace>;
        get face2 = my_face2: Handle<StepVisualTessellatedFace>;
        set set_face2 = my_face2: Handle<StepVisualTessellatedFace>;
        get line_strip_face1 = my_line_strip_face1: Handle<HArray1<i32>>;
        set set_line_strip_face1 = my_line_strip_face1: Handle<HArray1<i32>>;
        len nb_line_strip_face1 = my_line_strip_face1;
        item line_strip_face1_value = my_line_strip_face1: i32;
        get line_strip_face2 = my_line_strip_face2: Handle<HArray1<i32>>;
        set set_line_strip_face2 = my_line_strip_face2: Handle<HArray1<i32>>;
        len nb_line_strip_face2 = my_line_strip_face2;
        item line_strip_face2_value = my_line_strip_face2: i32;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_tessellated_edge_coordinates: &Handle<StepVisualCoordinatesList>,
        the_has_tessellated_edge_geometric_link: bool,
        the_tessellated_edge_geometric_link: &StepVisualEdgeOrCurve,
        the_tessellated_edge_line_strip: &Handle<HArray1<i32>>,
        the_smooth: StepDataLogical,
        the_face1: &Handle<StepVisualTessellatedFace>,
        the_face2: &Handle<StepVisualTessellatedFace>,
        the_line_strip_face1: &Handle<HArray1<i32>>,
        the_line_strip_face2: &Handle<HArray1<i32>>,
    ) {
        self.parent.init(
            the_representation_item_name,
            the_tessellated_edge_coordinates,
            the_has_tessellated_edge_geometric_link,
            the_tessellated_edge_geometric_link,
            the_tessellated_edge_line_strip,
        );
        let mut state = self.state.write();
        state.my_smooth = the_smooth;
        state.my_face1 = the_face1.clone();
        state.my_face2 = the_face2.clone();
        state.my_line_strip_face1 = the_line_strip_face1.clone();
        state.my_line_strip_face2 = the_line_strip_face2.clone();
    }
}

step_entity! {
    /// Tessellated curve set.
    StepVisualTessellatedCurveSet, StepVisualTessellatedCurveSetState, "StepVisualTessellatedCurveSet", StepVisualTessellatedItem, {
        my_coord_list: Handle<StepVisualCoordinatesList>,
        my_curves: ObjectHandle<DynamicArray<Handle<HSequence<i32>>>>,
    }
}
impl StepVisualTessellatedCurveSet {
    step_methods! {
        get coord_list = my_coord_list: Handle<StepVisualCoordinatesList>;
        get curves = my_curves: ObjectHandle<DynamicArray<Handle<HSequence<i32>>>>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_coord_list: &Handle<StepVisualCoordinatesList>,
        the_curves: &ObjectHandle<DynamicArray<Handle<HSequence<i32>>>>,
    ) {
        self.parent.parent.parent.init(the_name);
        let mut state = self.state.write();
        state.my_coord_list = the_coord_list.clone();
        state.my_curves = the_curves.clone();
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedEdge
    StepVisualTessellatedEdge, StepVisualTessellatedEdgeState, "StepVisualTessellatedEdge", StepVisualTessellatedStructuredItem, {
        my_coordinates: Handle<StepVisualCoordinatesList>,
        my_geometric_link: StepVisualEdgeOrCurve,
        my_line_strip: Handle<HArray1<i32>>,
        my_has_geometric_link: bool,
    }
}
impl StepVisualTessellatedEdge {
    step_methods! {
        get coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        set set_coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        get geometric_link = my_geometric_link: StepVisualEdgeOrCurve;
        set set_geometric_link = my_geometric_link: StepVisualEdgeOrCurve;
        get has_geometric_link = my_has_geometric_link: bool;
        get line_strip = my_line_strip: Handle<HArray1<i32>>;
        set set_line_strip = my_line_strip: Handle<HArray1<i32>>;
        len nb_line_strip = my_line_strip;
        item line_strip_value = my_line_strip: i32;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_coordinates: &Handle<StepVisualCoordinatesList>,
        the_has_geometric_link: bool,
        the_geometric_link: &StepVisualEdgeOrCurve,
        the_line_strip: &Handle<HArray1<i32>>,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_coordinates = the_coordinates.clone();
        state.my_has_geometric_link = the_has_geometric_link;
        if state.my_has_geometric_link {
            state.my_geometric_link = the_geometric_link.clone();
        } else {
            state.my_geometric_link = StepVisualEdgeOrCurve::default();
        }
        state.my_line_strip = the_line_strip.clone();
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedFace
    StepVisualTessellatedFace, StepVisualTessellatedFaceState, "StepVisualTessellatedFace", StepVisualTessellatedStructuredItem, {
        my_coordinates: Handle<StepVisualCoordinatesList>,
        my_pnmax: i32,
        my_normals: Handle<HArray2<f64>>,
        my_geometric_link: StepVisualFaceOrSurface,
        my_has_geometric_link: bool,
    }
}
impl StepVisualTessellatedFace {
    step_methods! {
        get coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        set set_coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        get pnmax = my_pnmax: i32;
        set_copy set_pnmax = my_pnmax: i32;
        get normals = my_normals: Handle<HArray2<f64>>;
        set set_normals = my_normals: Handle<HArray2<f64>>;
        get geometric_link = my_geometric_link: StepVisualFaceOrSurface;
        set set_geometric_link = my_geometric_link: StepVisualFaceOrSurface;
        get has_geometric_link = my_has_geometric_link: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_coordinates: &Handle<StepVisualCoordinatesList>,
        the_pnmax: i32,
        the_normals: &Handle<HArray2<f64>>,
        the_has_geometric_link: bool,
        the_geometric_link: &StepVisualFaceOrSurface,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_coordinates = the_coordinates.clone();
        state.my_pnmax = the_pnmax;
        state.my_normals = the_normals.clone();
        state.my_has_geometric_link = the_has_geometric_link;
        if state.my_has_geometric_link {
            state.my_geometric_link = the_geometric_link.clone();
        } else {
            state.my_geometric_link = StepVisualFaceOrSurface::default();
        }
    }
    /// `nb_normals`.
    /// Returns number of Normals
    pub fn nb_normals(&self) -> i32 {
        if self.state.read().my_normals.is_null() {
            return 0;
        }
        let field_my_normals = self.state.read().my_normals.clone();
        field_my_normals.array2().col_length()
    }
}

step_entity! {
    /// Tessellated geometric set.
    StepVisualTessellatedGeometricSet, StepVisualTessellatedGeometricSetState, "StepVisualTessellatedGeometricSet", StepVisualTessellatedItem, {
        my_items: ObjectHandle<Array1<'static, Handle<StepVisualTessellatedItem>>>,
    }
}
impl StepVisualTessellatedGeometricSet {
    step_methods! {
        get items = my_items: ObjectHandle<Array1<'static, Handle<StepVisualTessellatedItem>>>;
    }
    /// `init`.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_items: &ObjectHandle<Array1<'static, Handle<StepVisualTessellatedItem>>>,
    ) {
        self.parent.parent.parent.init(the_name);
        self.state.write().my_items = the_items.clone();
    }
}

step_entity! {
    /// Tessellated item.
    StepVisualTessellatedItem, StepVisualTessellatedItemState, "StepVisualTessellatedItem", StepGeomGeometricRepresentationItem, {
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedPointSet
    StepVisualTessellatedPointSet, StepVisualTessellatedPointSetState, "StepVisualTessellatedPointSet", StepVisualTessellatedItem, {
        my_coordinates: Handle<StepVisualCoordinatesList>,
        my_point_list: Handle<HArray1<i32>>,
    }
}
impl StepVisualTessellatedPointSet {
    step_methods! {
        get coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        set set_coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        get point_list = my_point_list: Handle<HArray1<i32>>;
        set set_point_list = my_point_list: Handle<HArray1<i32>>;
        len nb_point_list = my_point_list;
        item point_list_value = my_point_list: i32;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_coordinates: &Handle<StepVisualCoordinatesList>,
        the_point_list: &Handle<HArray1<i32>>,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_coordinates = the_coordinates.clone();
        state.my_point_list = the_point_list.clone();
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedShapeRepresentation
    StepVisualTessellatedShapeRepresentation, StepVisualTessellatedShapeRepresentationState, "StepVisualTessellatedShapeRepresentation", StepShapeShapeRepresentation, {
    }
}
