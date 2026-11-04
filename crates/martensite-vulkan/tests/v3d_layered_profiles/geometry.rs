//! Existing BRep text and polygon factories keep their original defaults and command order.
use super::specification::ScreenPair;
use crate::{
    brep_text_shape::{BRepText, text_shape},
    case_view::viewer_session::{ViewerSession, corner_persistence},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{
    aspect::AspectTypeOfTriedronPosition, graphic3d::Graphic3dTransModeFlags,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakePolygon;
use martensite_tkv3d::{ais::AisShape, prs_mgr::PrsMgrPresentableObject};
type Outcome = Result<(), Box<dyn std::error::Error>>;
pub(super) fn pair(session: &mut ViewerSession, spec: &ScreenPair, layer: i32) -> Outcome {
    display_text(session, spec, layer)?;
    display_wire(session, spec, layer)
}
fn display_text(session: &mut ViewerSession, spec: &ScreenPair, layer: i32) -> Outcome {
    let options = BRepText {
        font: "SansFont",
        height: 30.,
        position: spec.position,
        ..Default::default()
    };
    let text = text_shape(spec.text, &options)?;
    display(
        session,
        spec.text_name,
        &text,
        spec.corner,
        spec.text_offset,
        layer,
    )?;
    Ok(())
}
fn display_wire(session: &mut ViewerSession, spec: &ScreenPair, layer: i32) -> Outcome {
    let wire = polyline(spec.points)?;
    display(
        session,
        spec.line_name,
        &wire,
        spec.corner,
        spec.line_offset,
        layer,
    )?;
    Ok(())
}
pub(super) fn polyline(points: &[[f64; 3]]) -> Result<Shape, Failure> {
    let mut maker = BRepBuilderAPIMakePolygon::new();
    for &[x, y, z] in points {
        maker.add_point(Point3::new(x, y, z))?;
    }
    Ok(maker.wire()?.into_shape())
}
fn display(
    session: &mut ViewerSession,
    name: &str,
    shape: &Shape,
    corner: AspectTypeOfTriedronPosition,
    offset: [i32; 2],
    layer: i32,
) -> Result<(), Failure> {
    let object = AisShape::new(shape);
    let persistence = corner_persistence(Graphic3dTransModeFlags::TMF_2D, corner, offset)?;
    object.set_z_layer(layer);
    object.set_transform_persistence(&persistence);
    session.display_object(name, object.cast())
}
