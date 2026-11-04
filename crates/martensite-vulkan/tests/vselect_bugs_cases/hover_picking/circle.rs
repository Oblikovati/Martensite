use super::*;
use martensite_tkbrep::topods::{Edge, Wire};
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeWire};

pub(super) struct CircleInputs {
    pub circle: Circle3,
    pub edge: Edge,
    pub wire: Wire,
}

pub(super) fn inputs() -> Result<CircleInputs, Failure> {
    let position = Axis2::new(Point3::new(150., 0., 200.), Direction3::X)?;
    let circle = Circle3::new(position, 100.)?;
    let edge = BRepBuilderAPIMakeEdge::from_circ(circle)?.edge()?;
    // The source builds w even though only e is displayed; retain its genuine topology owner.
    let wire = BRepBuilderAPIMakeWire::from_edge(&edge)?.wire()?;
    Ok(CircleInputs { circle, edge, wire })
}

pub(super) fn parameters() -> ViewParameters {
    ViewParameters {
        scale: Some(25.38),
        proj: Some([0.5, -0.5, 0.5]),
        up: Some([-0.4, 0.4, 0.8]),
        at: Some([125.128, 46.4744, 271.346]),
        ..Default::default()
    }
}

pub(super) fn scene(session: &mut ViewerSession, inputs: &CircleInputs) -> Result<(), Failure> {
    // Keep the constructed wire live through the source's scene, without displaying a substitute.
    let _source_owners = (&inputs.circle, &inputs.wire);
    session.display(
        "e",
        &inputs.edge.clone().into_shape(),
        &DisplayRequest::default(),
    )?;
    session.set_line_width("e", 2.)?;
    session.fit()?;
    session.set_view_parameters(&parameters())
}
