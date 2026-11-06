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
    /// Torus.
    StepShapeTorus, StepShapeTorusState, "StepShapeTorus", StepGeomGeometricRepresentationItem, {
        position: Handle<StepGeomAxis1Placement>,
        major_radius: f64,
        minor_radius: f64,
    }
}
impl StepShapeTorus {
    step_methods! {
        set set_position = position: Handle<StepGeomAxis1Placement>;
        get position = position: Handle<StepGeomAxis1Placement>;
        set_copy set_major_radius = major_radius: f64;
        get major_radius = major_radius: f64;
        set_copy set_minor_radius = minor_radius: f64;
        get minor_radius = minor_radius: f64;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_position: &Handle<StepGeomAxis1Placement>,
        a_major_radius: f64,
        a_minor_radius: f64,
    ) {
        let mut state = self.state.write();
        state.position = a_position.clone();
        state.major_radius = a_major_radius;
        state.minor_radius = a_minor_radius;
        drop(state);
        self.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Transitional shape representation.
    StepShapeTransitionalShapeRepresentation, StepShapeTransitionalShapeRepresentationState, "StepShapeTransitionalShapeRepresentation", StepShapeShapeRepresentation, {
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapeTypeQualifier, StepShapeTypeQualifierState, "StepShapeTypeQualifier", StandardTransient, {
        the_name: Handle<HAsciiString>,
    }
}
impl StepShapeTypeQualifier {
    step_methods! {
        set init = the_name: Handle<HAsciiString>;
        get name = the_name: Handle<HAsciiString>;
        set set_name = the_name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepShapeValueFormatTypeQualifier, StepShapeValueFormatTypeQualifierState, "StepShapeValueFormatTypeQualifier", StandardTransient, {
        format_type: Handle<HAsciiString>,
    }
}
impl StepShapeValueFormatTypeQualifier {
    step_methods! {
        set init = format_type: Handle<HAsciiString>;
        get format_type = format_type: Handle<HAsciiString>;
        set set_format_type = format_type: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Vertex.
    StepShapeVertex, StepShapeVertexState, "StepShapeVertex", StepShapeTopologicalRepresentationItem, {
    }
}

step_entity! {
    /// Vertex loop.
    StepShapeVertexLoop, StepShapeVertexLoopState, "StepShapeVertexLoop", StepShapeLoop, {
        loop_vertex: Handle<StepShapeVertex>,
    }
}
impl StepShapeVertexLoop {
    step_methods! {
        set set_loop_vertex = loop_vertex: Handle<StepShapeVertex>;
        get loop_vertex = loop_vertex: Handle<StepShapeVertex>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_loop_vertex: &Handle<StepShapeVertex>) {
        self.state.write().loop_vertex = a_loop_vertex.clone();
        self.parent.parent.parent.init(a_name);
    }
}

step_entity! {
    /// Vertex point.
    StepShapeVertexPoint, StepShapeVertexPointState, "StepShapeVertexPoint", StepShapeVertex, {
        vertex_geometry: Handle<StepGeomPoint>,
    }
}
impl StepShapeVertexPoint {
    step_methods! {
        set set_vertex_geometry = vertex_geometry: Handle<StepGeomPoint>;
        get vertex_geometry = vertex_geometry: Handle<StepGeomPoint>;
    }
    /// `init`.
    pub fn init(&self, a_name: &Handle<HAsciiString>, a_vertex_geometry: &Handle<StepGeomPoint>) {
        self.state.write().vertex_geometry = a_vertex_geometry.clone();
        self.parent.parent.parent.init(a_name);
    }
}
