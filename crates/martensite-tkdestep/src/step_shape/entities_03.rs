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
    /// Edge curve.
    StepShapeEdgeCurve, StepShapeEdgeCurveState, "StepShapeEdgeCurve", StepShapeEdge, {
        edge_geometry: Handle<StepGeomCurve>,
        same_sense: bool,
    }
}
impl StepShapeEdgeCurve {
    step_methods! {
        set set_edge_geometry = edge_geometry: Handle<StepGeomCurve>;
        get edge_geometry = edge_geometry: Handle<StepGeomCurve>;
        set_copy set_same_sense = same_sense: bool;
        get same_sense = same_sense: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_edge_start: &Handle<StepShapeVertex>,
        a_edge_end: &Handle<StepShapeVertex>,
        a_edge_geometry: &Handle<StepGeomCurve>,
        a_same_sense: bool,
    ) {
        let mut state = self.state.write();
        state.edge_geometry = a_edge_geometry.clone();
        state.same_sense = a_same_sense;
        drop(state);
        self.parent.init(a_name, a_edge_start, a_edge_end);
    }
}

step_entity! {
    /// Edge loop.
    StepShapeEdgeLoop, StepShapeEdgeLoopState, "StepShapeEdgeLoop", StepShapeLoop, {
        edge_list: Handle<HArray1<Handle<StepShapeOrientedEdge>>>,
    }
}
impl StepShapeEdgeLoop {
    step_methods! {
        set set_edge_list = edge_list: Handle<HArray1<Handle<StepShapeOrientedEdge>>>;
        get edge_list = edge_list: Handle<HArray1<Handle<StepShapeOrientedEdge>>>;
        item edge_list_value = edge_list: Handle<StepShapeOrientedEdge>;
        len nb_edge_list = edge_list;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_edge_list: &Handle<HArray1<Handle<StepShapeOrientedEdge>>>,
    ) {
        self.parent.parent.parent.init(a_name);
        self.state.write().edge_list = a_edge_list.clone();
    }
}

step_entity! {
    /// Extruded area solid.
    StepShapeExtrudedAreaSolid, StepShapeExtrudedAreaSolidState, "StepShapeExtrudedAreaSolid", StepShapeSweptAreaSolid, {
        extruded_direction: Handle<StepGeomDirection>,
        depth: f64,
    }
}
impl StepShapeExtrudedAreaSolid {
    step_methods! {
        set set_extruded_direction = extruded_direction: Handle<StepGeomDirection>;
        get extruded_direction = extruded_direction: Handle<StepGeomDirection>;
        set_copy set_depth = depth: f64;
        get depth = depth: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_swept_area: &Handle<StepGeomCurveBoundedSurface>,
        a_extruded_direction: &Handle<StepGeomDirection>,
        a_depth: f64,
    ) {
        let mut state = self.state.write();
        state.extruded_direction = a_extruded_direction.clone();
        state.depth = a_depth;
        drop(state);
        self.parent.init(a_name, a_swept_area);
    }
}

step_entity! {
    /// Extruded face solid.
    StepShapeExtrudedFaceSolid, StepShapeExtrudedFaceSolidState, "StepShapeExtrudedFaceSolid", StepShapeSweptFaceSolid, {
        extruded_direction: Handle<StepGeomDirection>,
        depth: f64,
    }
}
impl StepShapeExtrudedFaceSolid {
    step_methods! {
        set set_extruded_direction = extruded_direction: Handle<StepGeomDirection>;
        get extruded_direction = extruded_direction: Handle<StepGeomDirection>;
        set_copy set_depth = depth: f64;
        get depth = depth: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_swept_area: &Handle<StepShapeFaceSurface>,
        a_extruded_direction: &Handle<StepGeomDirection>,
        a_depth: f64,
    ) {
        let mut state = self.state.write();
        state.extruded_direction = a_extruded_direction.clone();
        state.depth = a_depth;
        drop(state);
        self.parent.init(a_name, a_swept_area);
    }
}

step_entity! {
    /// Representation of STEP entity FaceBasedSurfaceModel
    StepShapeFaceBasedSurfaceModel, StepShapeFaceBasedSurfaceModelState, "StepShapeFaceBasedSurfaceModel", StepGeomGeometricRepresentationItem, {
        the_fbsm_faces: Handle<HArray1<Handle<StepShapeConnectedFaceSet>>>,
    }
}
impl StepShapeFaceBasedSurfaceModel {
    step_methods! {
        get fbsm_faces = the_fbsm_faces: Handle<HArray1<Handle<StepShapeConnectedFaceSet>>>;
        set set_fbsm_faces = the_fbsm_faces: Handle<HArray1<Handle<StepShapeConnectedFaceSet>>>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        a_representation_item_name: &Handle<HAsciiString>,
        a_fbsm_faces: &Handle<HArray1<Handle<StepShapeConnectedFaceSet>>>,
    ) {
        self.parent.init(a_representation_item_name);
        self.state.write().the_fbsm_faces = a_fbsm_faces.clone();
    }
}

step_entity! {
    /// Face bound.
    StepShapeFaceBound, StepShapeFaceBoundState, "StepShapeFaceBound", StepShapeTopologicalRepresentationItem, {
        bound: Handle<StepShapeLoop>,
        orientation: bool,
    }
}
impl StepShapeFaceBound {
    step_methods! {
        set set_bound = bound: Handle<StepShapeLoop>;
        get bound = bound: Handle<StepShapeLoop>;
        set_copy set_orientation = orientation: bool;
        get orientation = orientation: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_bound: &Handle<StepShapeLoop>,
        a_orientation: bool,
    ) {
        let mut state = self.state.write();
        state.bound = a_bound.clone();
        state.orientation = a_orientation;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Face outer bound.
    StepShapeFaceOuterBound, StepShapeFaceOuterBoundState, "StepShapeFaceOuterBound", StepShapeFaceBound, {
    }
}

step_entity! {
    /// Face surface.
    StepShapeFaceSurface, StepShapeFaceSurfaceState, "StepShapeFaceSurface", StepShapeFace, {
        face_geometry: Handle<StepGeomSurface>,
        same_sense: bool,
    }
}
impl StepShapeFaceSurface {
    step_methods! {
        set set_face_geometry = face_geometry: Handle<StepGeomSurface>;
        get face_geometry = face_geometry: Handle<StepGeomSurface>;
        set_copy set_same_sense = same_sense: bool;
        get same_sense = same_sense: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_bounds: &Handle<HArray1<Handle<StepShapeFaceBound>>>,
        a_face_geometry: &Handle<StepGeomSurface>,
        a_same_sense: bool,
    ) {
        let mut state = self.state.write();
        state.face_geometry = a_face_geometry.clone();
        state.same_sense = a_same_sense;
        drop(state);
        self.parent.init(a_name, a_bounds);
    }
}

step_entity! {
    /// Faceted brep.
    StepShapeFacetedBrep, StepShapeFacetedBrepState, "StepShapeFacetedBrep", StepShapeManifoldSolidBrep, {
    }
}

step_entity! {
    /// Faceted brep and brep with voids.
    StepShapeFacetedBrepAndBrepWithVoids, StepShapeFacetedBrepAndBrepWithVoidsState, "StepShapeFacetedBrepAndBrepWithVoids", StepShapeManifoldSolidBrep, {
        faceted_brep: Handle<StepShapeFacetedBrep>,
        brep_with_voids: Handle<StepShapeBrepWithVoids>,
    }
}
impl StepShapeFacetedBrepAndBrepWithVoids {
    step_methods! {
        set set_faceted_brep = faceted_brep: Handle<StepShapeFacetedBrep>;
        get faceted_brep = faceted_brep: Handle<StepShapeFacetedBrep>;
        set set_brep_with_voids = brep_with_voids: Handle<StepShapeBrepWithVoids>;
        get brep_with_voids = brep_with_voids: Handle<StepShapeBrepWithVoids>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_outer: &Handle<StepShapeClosedShell>,
        a_faceted_brep: &Handle<StepShapeFacetedBrep>,
        a_brep_with_voids: &Handle<StepShapeBrepWithVoids>,
    ) {
        let mut state = self.state.write();
        state.faceted_brep = a_faceted_brep.clone();
        state.brep_with_voids = a_brep_with_voids.clone();
        drop(state);
        self.parent.init(a_name, a_outer);
    }
    /// `init`.
    pub fn init_fields(
        &self,
        a_name: &Handle<HAsciiString>,
        a_outer: &Handle<StepShapeClosedShell>,
        a_voids: &Handle<HArray1<Handle<StepShapeOrientedClosedShell>>>,
    ) {
        self.parent.init(a_name, a_outer);
        self.state.write().brep_with_voids = Handle::new(StepShapeBrepWithVoids::new());
        let field_brep_with_voids = self.state.read().brep_with_voids.clone();
        field_brep_with_voids.init(a_name, a_outer, a_voids);
        self.state.write().faceted_brep = Handle::new(StepShapeFacetedBrep::new());
        let field_faceted_brep = self.state.read().faceted_brep.clone();
        field_faceted_brep.init(a_name, a_outer);
    }
    /// `set_voids`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_voids(
        &self,
        a_voids: &Handle<HArray1<Handle<StepShapeOrientedClosedShell>>>,
    ) -> Result<(), Failure> {
        let field_brep_with_voids = self.state.read().brep_with_voids.clone();
        step_live(
            &field_brep_with_voids,
            "StepShapeFacetedBrepAndBrepWithVoids::SetVoids",
        )?
        .set_voids(a_voids);
        Ok(())
    }
    /// `voids`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn voids(&self) -> Result<Handle<HArray1<Handle<StepShapeOrientedClosedShell>>>, Failure> {
        let field_brep_with_voids = self.state.read().brep_with_voids.clone();
        Ok(step_live(
            &field_brep_with_voids,
            "StepShapeFacetedBrepAndBrepWithVoids::Voids",
        )?
        .voids())
    }
    /// `voids_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn voids_value(&self, num: i32) -> Result<Handle<StepShapeOrientedClosedShell>, Failure> {
        let field_brep_with_voids = self.state.read().brep_with_voids.clone();
        step_live(
            &field_brep_with_voids,
            "StepShapeFacetedBrepAndBrepWithVoids::VoidsValue",
        )?
        .voids_value(num)
    }
    /// `nb_voids`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_voids(&self) -> Result<i32, Failure> {
        let field_brep_with_voids = self.state.read().brep_with_voids.clone();
        Ok(step_live(
            &field_brep_with_voids,
            "StepShapeFacetedBrepAndBrepWithVoids::NbVoids",
        )?
        .nb_voids())
    }
}

step_entity! {
    /// Faceted brep shape representation.
    StepShapeFacetedBrepShapeRepresentation, StepShapeFacetedBrepShapeRepresentationState, "StepShapeFacetedBrepShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Geometric curve set.
    StepShapeGeometricCurveSet, StepShapeGeometricCurveSetState, "StepShapeGeometricCurveSet", StepShapeGeometricSet, {
    }
}

step_entity! {
    /// Geometric set.
    StepShapeGeometricSet, StepShapeGeometricSetState, "StepShapeGeometricSet", StepGeomGeometricRepresentationItem, {
        elements: Handle<HArray1<StepShapeGeometricSetSelect>>,
    }
}
impl StepShapeGeometricSet {
    step_methods! {
        set set_elements = elements: Handle<HArray1<StepShapeGeometricSetSelect>>;
        get elements = elements: Handle<HArray1<StepShapeGeometricSetSelect>>;
        item elements_value = elements: StepShapeGeometricSetSelect;
        len nb_elements = elements;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_elements: &Handle<HArray1<StepShapeGeometricSetSelect>>,
    ) {
        self.state.write().elements = a_elements.clone();
        self.parent.parent.init(a_name);
    }
}
