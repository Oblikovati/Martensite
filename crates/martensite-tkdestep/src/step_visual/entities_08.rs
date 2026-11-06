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
    /// Representation of STEP entity TessellatedShapeRepresentationWithAccuracyParameters
    StepVisualTessellatedShapeRepresentationWithAccuracyParameters, StepVisualTessellatedShapeRepresentationWithAccuracyParametersState, "StepVisualTessellatedShapeRepresentationWithAccuracyParameters", StepVisualTessellatedShapeRepresentation, {
        my_tessellation_accuracy_parameters: Handle<HArray1<f64>>,
    }
}
impl StepVisualTessellatedShapeRepresentationWithAccuracyParameters {
    step_methods! {
        get tessellation_accuracy_parameters = my_tessellation_accuracy_parameters: Handle<HArray1<f64>>;
        set set_tessellation_accuracy_parameters = my_tessellation_accuracy_parameters: Handle<HArray1<f64>>;
        len nb_tessellation_accuracy_parameters = my_tessellation_accuracy_parameters;
        item tessellation_accuracy_parameters_value = my_tessellation_accuracy_parameters: f64;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_name: &Handle<HAsciiString>,
        the_representation_items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
        the_representation_context_of_items: &Handle<StepReprRepresentationContext>,
        the_tessellation_accuracy_parameters: &Handle<HArray1<f64>>,
    ) {
        self.parent.init(
            the_representation_name,
            the_representation_items,
            the_representation_context_of_items,
        );
        self.state.write().my_tessellation_accuracy_parameters =
            the_tessellation_accuracy_parameters.clone();
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedShell
    StepVisualTessellatedShell, StepVisualTessellatedShellState, "StepVisualTessellatedShell", StepVisualTessellatedItem, {
        my_items: Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>,
        my_topological_link: Handle<StepShapeConnectedFaceSet>,
        my_has_topological_link: bool,
    }
}
impl StepVisualTessellatedShell {
    step_methods! {
        get items = my_items: Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>;
        set set_items = my_items: Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>;
        len nb_items = my_items;
        item items_value = my_items: Handle<StepVisualTessellatedStructuredItem>;
        get topological_link = my_topological_link: Handle<StepShapeConnectedFaceSet>;
        set set_topological_link = my_topological_link: Handle<StepShapeConnectedFaceSet>;
        get has_topological_link = my_has_topological_link: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_items: &Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>,
        the_has_topological_link: bool,
        the_topological_link: &Handle<StepShapeConnectedFaceSet>,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_items = the_items.clone();
        state.my_has_topological_link = the_has_topological_link;
        if state.my_has_topological_link {
            state.my_topological_link = the_topological_link.clone();
        } else {
            state.my_topological_link = Default::default();
        }
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedSolid
    StepVisualTessellatedSolid, StepVisualTessellatedSolidState, "StepVisualTessellatedSolid", StepVisualTessellatedItem, {
        my_items: Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>,
        my_geometric_link: Handle<StepShapeManifoldSolidBrep>,
        my_has_geometric_link: bool,
    }
}
impl StepVisualTessellatedSolid {
    step_methods! {
        get items = my_items: Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>;
        set set_items = my_items: Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>;
        len nb_items = my_items;
        item items_value = my_items: Handle<StepVisualTessellatedStructuredItem>;
        get geometric_link = my_geometric_link: Handle<StepShapeManifoldSolidBrep>;
        set set_geometric_link = my_geometric_link: Handle<StepShapeManifoldSolidBrep>;
        get has_geometric_link = my_has_geometric_link: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_items: &Handle<HArray1<Handle<StepVisualTessellatedStructuredItem>>>,
        the_has_geometric_link: bool,
        the_geometric_link: &Handle<StepShapeManifoldSolidBrep>,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_items = the_items.clone();
        state.my_has_geometric_link = the_has_geometric_link;
        if state.my_has_geometric_link {
            state.my_geometric_link = the_geometric_link.clone();
        } else {
            state.my_geometric_link = Default::default();
        }
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedStructuredItem
    StepVisualTessellatedStructuredItem, StepVisualTessellatedStructuredItemState, "StepVisualTessellatedStructuredItem", StepVisualTessellatedItem, {
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedSurfaceSet
    StepVisualTessellatedSurfaceSet, StepVisualTessellatedSurfaceSetState, "StepVisualTessellatedSurfaceSet", StepVisualTessellatedItem, {
        my_coordinates: Handle<StepVisualCoordinatesList>,
        my_pnmax: i32,
        my_normals: Handle<HArray2<f64>>,
    }
}
impl StepVisualTessellatedSurfaceSet {
    step_methods! {
        get coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        set set_coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        get pnmax = my_pnmax: i32;
        set_copy set_pnmax = my_pnmax: i32;
        get normals = my_normals: Handle<HArray2<f64>>;
        set set_normals = my_normals: Handle<HArray2<f64>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_coordinates: &Handle<StepVisualCoordinatesList>,
        the_pnmax: i32,
        the_normals: &Handle<HArray2<f64>>,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_coordinates = the_coordinates.clone();
        state.my_pnmax = the_pnmax;
        state.my_normals = the_normals.clone();
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
    /// Representation of STEP entity TessellatedVertex
    StepVisualTessellatedVertex, StepVisualTessellatedVertexState, "StepVisualTessellatedVertex", StepVisualTessellatedStructuredItem, {
        my_coordinates: Handle<StepVisualCoordinatesList>,
        my_topological_link: Handle<StepShapeVertexPoint>,
        my_point_index: i32,
        my_has_topological_link: bool,
    }
}
impl StepVisualTessellatedVertex {
    step_methods! {
        get coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        set set_coordinates = my_coordinates: Handle<StepVisualCoordinatesList>;
        get topological_link = my_topological_link: Handle<StepShapeVertexPoint>;
        set set_topological_link = my_topological_link: Handle<StepShapeVertexPoint>;
        get has_topological_link = my_has_topological_link: bool;
        get point_index = my_point_index: i32;
        set_copy set_point_index = my_point_index: i32;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_coordinates: &Handle<StepVisualCoordinatesList>,
        the_has_topological_link: bool,
        the_topological_link: &Handle<StepShapeVertexPoint>,
        the_point_index: i32,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_coordinates = the_coordinates.clone();
        state.my_has_topological_link = the_has_topological_link;
        if state.my_has_topological_link {
            state.my_topological_link = the_topological_link.clone();
        } else {
            state.my_topological_link = Default::default();
        }
        state.my_point_index = the_point_index;
    }
}

step_entity! {
    /// Representation of STEP entity TessellatedWire
    StepVisualTessellatedWire, StepVisualTessellatedWireState, "StepVisualTessellatedWire", StepVisualTessellatedItem, {
        my_items: Handle<HArray1<StepVisualTessellatedEdgeOrVertex>>,
        my_geometric_model_link: StepVisualPathOrCompositeCurve,
        my_has_geometric_model_link: bool,
    }
}
impl StepVisualTessellatedWire {
    step_methods! {
        get items = my_items: Handle<HArray1<StepVisualTessellatedEdgeOrVertex>>;
        set set_items = my_items: Handle<HArray1<StepVisualTessellatedEdgeOrVertex>>;
        len nb_items = my_items;
        item items_value = my_items: StepVisualTessellatedEdgeOrVertex;
        get geometric_model_link = my_geometric_model_link: StepVisualPathOrCompositeCurve;
        set set_geometric_model_link = my_geometric_model_link: StepVisualPathOrCompositeCurve;
        get has_geometric_model_link = my_has_geometric_model_link: bool;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_representation_item_name: &Handle<HAsciiString>,
        the_items: &Handle<HArray1<StepVisualTessellatedEdgeOrVertex>>,
        the_has_geometric_model_link: bool,
        the_geometric_model_link: &StepVisualPathOrCompositeCurve,
    ) {
        self.parent.init(the_representation_item_name);
        let mut state = self.state.write();
        state.my_items = the_items.clone();
        state.my_has_geometric_model_link = the_has_geometric_model_link;
        if state.my_has_geometric_model_link {
            state.my_geometric_model_link = the_geometric_model_link.clone();
        } else {
            state.my_geometric_model_link = StepVisualPathOrCompositeCurve::default();
        }
    }
}

step_entity! {
    /// Text literal.
    StepVisualTextLiteral, StepVisualTextLiteralState, "StepVisualTextLiteral", StepGeomGeometricRepresentationItem, {
        literal: Handle<HAsciiString>,
        placement: StepGeomAxis2Placement,
        alignment: Handle<HAsciiString>,
        path: StepVisualTextPath,
        font: StepVisualFontSelect,
    }
}
impl StepVisualTextLiteral {
    step_methods! {
        set set_literal = literal: Handle<HAsciiString>;
        get literal = literal: Handle<HAsciiString>;
        set set_placement = placement: StepGeomAxis2Placement;
        get placement = placement: StepGeomAxis2Placement;
        set set_alignment = alignment: Handle<HAsciiString>;
        get alignment = alignment: Handle<HAsciiString>;
        set_copy set_path = path: StepVisualTextPath;
        get path = path: StepVisualTextPath;
        set set_font = font: StepVisualFontSelect;
        get font = font: StepVisualFontSelect;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_literal: &Handle<HAsciiString>,
        a_placement: &StepGeomAxis2Placement,
        a_alignment: &Handle<HAsciiString>,
        a_path: StepVisualTextPath,
        a_font: &StepVisualFontSelect,
    ) {
        let mut state = self.state.write();
        state.literal = a_literal.clone();
        state.placement = a_placement.clone();
        state.alignment = a_alignment.clone();
        state.path = a_path;
        state.font = a_font.clone();
        drop(state);
        self.parent.parent.init(a_name);
    }
}
