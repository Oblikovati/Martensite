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
    /// Geometrically bounded surface shape representation.
    StepShapeGeometricallyBoundedSurfaceShapeRepresentation, StepShapeGeometricallyBoundedSurfaceShapeRepresentationState, "StepShapeGeometricallyBoundedSurfaceShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Geometrically bounded wireframe shape representation.
    StepShapeGeometricallyBoundedWireframeShapeRepresentation, StepShapeGeometricallyBoundedWireframeShapeRepresentationState, "StepShapeGeometricallyBoundedWireframeShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Half space solid.
    StepShapeHalfSpaceSolid, StepShapeHalfSpaceSolidState, "StepShapeHalfSpaceSolid", StepGeomGeometricRepresentationItem, {
        base_surface: Handle<StepGeomSurface>,
        agreement_flag: bool,
    }
}
impl StepShapeHalfSpaceSolid {
    step_methods! {
        set set_base_surface = base_surface: Handle<StepGeomSurface>;
        get base_surface = base_surface: Handle<StepGeomSurface>;
        set_copy set_agreement_flag = agreement_flag: bool;
        get agreement_flag = agreement_flag: bool;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_base_surface: &Handle<StepGeomSurface>,
        a_agreement_flag: bool,
    ) {
        let mut state = self.state.write();
        state.base_surface = a_base_surface.clone();
        state.agreement_flag = a_agreement_flag;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapeLimitsAndFits, StepShapeLimitsAndFitsState, "StepShapeLimitsAndFits", StandardTransient, {
        the_form_variance: Handle<HAsciiString>,
        the_zone_variance: Handle<HAsciiString>,
        the_grade: Handle<HAsciiString>,
        the_source: Handle<HAsciiString>,
    }
}
impl StepShapeLimitsAndFits {
    step_methods! {
        get form_variance = the_form_variance: Handle<HAsciiString>;
        set set_form_variance = the_form_variance: Handle<HAsciiString>;
        get zone_variance = the_zone_variance: Handle<HAsciiString>;
        set set_zone_variance = the_zone_variance: Handle<HAsciiString>;
        get grade = the_grade: Handle<HAsciiString>;
        set set_grade = the_grade: Handle<HAsciiString>;
        get source = the_source: Handle<HAsciiString>;
        set set_source = the_source: Handle<HAsciiString>;
    }
    /// `init`.
    pub fn init(
        &self,
        form_variance: &Handle<HAsciiString>,
        zone_variance: &Handle<HAsciiString>,
        grade: &Handle<HAsciiString>,
        source: &Handle<HAsciiString>,
    ) {
        let mut state = self.state.write();
        state.the_form_variance = form_variance.clone();
        state.the_zone_variance = zone_variance.clone();
        state.the_grade = grade.clone();
        state.the_source = source.clone();
    }
}

step_entity! {
    /// Loop.
    StepShapeLoop, StepShapeLoopState, "StepShapeLoop", StepShapeTopologicalRepresentationItem, {
    }
}

step_entity! {
    /// Loop and path.
    StepShapeLoopAndPath, StepShapeLoopAndPathState, "StepShapeLoopAndPath", StepShapeTopologicalRepresentationItem, {
        loop_: Handle<StepShapeLoop>,
        path: Handle<StepShapePath>,
    }
}
impl StepShapeLoopAndPath {
    step_methods! {
        set set_loop = loop_: Handle<StepShapeLoop>;
        get loop_ = loop_: Handle<StepShapeLoop>;
        set set_path = path: Handle<StepShapePath>;
        get path = path: Handle<StepShapePath>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_loop: &Handle<StepShapeLoop>,
        a_path: &Handle<StepShapePath>,
    ) {
        let mut state = self.state.write();
        state.loop_ = a_loop.clone();
        state.path = a_path.clone();
        drop(state);
        self.parent.parent.init(a_name);
    }
    /// `init`.
    pub fn init_fields(
        &self,
        a_name: &Handle<HAsciiString>,
        a_edge_list: &Handle<HArray1<Handle<StepShapeOrientedEdge>>>,
    ) {
        self.parent.parent.init(a_name);
        self.state.write().loop_ = Handle::new(StepShapeLoop::new());
        let field_loop_ = self.state.read().loop_.clone();
        field_loop_.init(a_name);
        self.state.write().path = Handle::new(StepShapePath::new());
        let field_path = self.state.read().path.clone();
        field_path.init(a_name, a_edge_list);
    }
    /// `set_edge_list`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_edge_list(
        &self,
        a_edge_list: &Handle<HArray1<Handle<StepShapeOrientedEdge>>>,
    ) -> Result<(), Failure> {
        let field_path = self.state.read().path.clone();
        step_live(&field_path, "StepShapeLoopAndPath::SetEdgeList")?.set_edge_list(a_edge_list);
        Ok(())
    }
    /// `edge_list`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn edge_list(&self) -> Result<Handle<HArray1<Handle<StepShapeOrientedEdge>>>, Failure> {
        let field_path = self.state.read().path.clone();
        step_live(&field_path, "StepShapeLoopAndPath::EdgeList")?.edge_list()
    }
    /// `edge_list_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn edge_list_value(&self, num: i32) -> Result<Handle<StepShapeOrientedEdge>, Failure> {
        let field_path = self.state.read().path.clone();
        step_live(&field_path, "StepShapeLoopAndPath::EdgeListValue")?.edge_list_value(num)
    }
    /// `nb_edge_list`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_edge_list(&self) -> Result<i32, Failure> {
        let field_path = self.state.read().path.clone();
        step_live(&field_path, "StepShapeLoopAndPath::NbEdgeList")?.nb_edge_list()
    }
}

step_entity! {
    /// Manifold solid brep.
    StepShapeManifoldSolidBrep, StepShapeManifoldSolidBrepState, "StepShapeManifoldSolidBrep", StepShapeSolidModel, {
        outer: Handle<StepShapeConnectedFaceSet>,
    }
}
impl StepShapeManifoldSolidBrep {
    step_methods! {
        set set_outer = outer: Handle<StepShapeConnectedFaceSet>;
        get outer = outer: Handle<StepShapeConnectedFaceSet>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_outer: &Handle<StepShapeClosedShell>) {
        self.state.write().outer = a_outer.cast();
        self.parent.parent.parent.init(a_name);
    }
    /// `init`.
    pub fn init_connected_face_set(
        &self,
        a_name: &Handle<HAsciiString>,
        a_outer: &Handle<StepShapeConnectedFaceSet>,
    ) {
        self.state.write().outer = a_outer.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Manifold surface shape representation.
    StepShapeManifoldSurfaceShapeRepresentation, StepShapeManifoldSurfaceShapeRepresentationState, "StepShapeManifoldSurfaceShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapeMeasureQualification, StepShapeMeasureQualificationState, "StepShapeMeasureQualification", StandardTransient, {
        the_name: Handle<HAsciiString>,
        the_description: Handle<HAsciiString>,
        the_qualified_measure: Handle<StandardTransient>,
        the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>,
    }
}
impl StepShapeMeasureQualification {
    step_methods! {
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
        get description = the_description: Handle<HAsciiString>;
        set set_description = the_description: Handle<HAsciiString>;
        get qualified_measure = the_qualified_measure: Handle<StandardTransient>;
        set set_qualified_measure = the_qualified_measure: Handle<StandardTransient>;
        get qualifiers = the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>;
        len_checked nb_qualifiers = the_qualifiers;
        set set_qualifiers = the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>;
        item qualifiers_value = the_qualifiers: StepShapeValueQualifier;
    }
    /// `init`.
    pub fn init(
        &self,
        name: &Handle<HAsciiString>,
        description: &Handle<HAsciiString>,
        qualified_measure: &Handle<StandardTransient>,
        qualifiers: &Handle<HArray1<StepShapeValueQualifier>>,
    ) {
        let mut state = self.state.write();
        state.the_name = name.clone();
        state.the_description = description.clone();
        state.the_qualified_measure = qualified_measure.clone();
        state.the_qualifiers = qualifiers.clone();
    }
    /// `set_qualifiers_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_qualifiers_value(
        &self,
        num: i32,
        aqualifier: &StepShapeValueQualifier,
    ) -> Result<(), Failure> {
        let field_the_qualifiers = self.state.read().the_qualifiers.clone();
        step_array1_set(
            &field_the_qualifiers,
            num,
            aqualifier.clone(),
            "StepShapeMeasureQualification::SetQualifiersValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    /// Complex Type between MeasureRepresentationItem and
    /// QualifiedRepresentationItem
    custom_default StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem, StepShapeMeasureRepresentationItemAndQualifiedRepresentationItemState, "StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem", StepReprRepresentationItem, {
        my_measure: Handle<StepBasicMeasureWithUnit>,
        the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>,
    }
}
impl Default for StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem {
    fn default() -> Self {
        let object = Self {
            parent: Default::default(),
            state: Default::default(),
        };
        {
            object.state.write().my_measure = Handle::new(StepBasicMeasureWithUnit::new());
        }
        object
    }
}
impl StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem {
    /// `StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem()`, which creates the sub-entities of the class.
    pub fn new() -> Self {
        Self::default()
    }
}
impl StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem {
    step_methods! {
        set set_measure = my_measure: Handle<StepBasicMeasureWithUnit>;
        get measure = my_measure: Handle<StepBasicMeasureWithUnit>;
        get qualifiers = the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>;
        len_checked nb_qualifiers = the_qualifiers;
        set set_qualifiers = the_qualifiers: Handle<HArray1<StepShapeValueQualifier>>;
        item qualifiers_value = the_qualifiers: StepShapeValueQualifier;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_value_component: &Handle<StepBasicMeasureValueMember>,
        a_unit_component: &StepBasicUnit,
        qualifiers: &Handle<HArray1<StepShapeValueQualifier>>,
    ) -> Result<(), Failure> {
        self.parent.init(a_name);
        let field_my_measure = self.state.read().my_measure.clone();
        step_live(
            &field_my_measure,
            "StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem::Init",
        )?
        .init(a_value_component, a_unit_component);
        self.state.write().the_qualifiers = qualifiers.clone();
        Ok(())
    }
    /// `set_qualifiers_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_qualifiers_value(
        &self,
        num: i32,
        aqualifier: &StepShapeValueQualifier,
    ) -> Result<(), Failure> {
        let field_the_qualifiers = self.state.read().the_qualifiers.clone();
        step_array1_set(
            &field_the_qualifiers,
            num,
            aqualifier.clone(),
            "StepShapeMeasureRepresentationItemAndQualifiedRepresentationItem::SetQualifiersValue",
        )?;
        Ok(())
    }
}
