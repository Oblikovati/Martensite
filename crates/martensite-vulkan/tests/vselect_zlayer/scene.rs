//! The original's topology and pre-display attributes retain their real source order.
use crate::case_view::viewer_session::{ViewerSession, corner_persistence};
use crate::primitive_shapes::box_sized;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::{Failure, Handle},
};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{
    aspect::AspectTypeOfTriedronPosition,
    graphic3d::{
        GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dTransModeFlags, Graphic3dTransformPersSource,
    },
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
use martensite_tkv3d::{
    ais::{AisInteractiveObject, AisShape},
    prs_mgr::PrsMgrPresentableObjectMembers as _,
};

/// Construct the literal boxes and topological vertex; `source_inputs()?[2]` is vertex(100,-300,0).
pub(super) fn source_inputs() -> Result<[Shape; 3], Failure> {
    let first = box_sized([2., 3., 4.])?;
    let second = box_sized([1., 1., 2.])?;
    let vertex = BRepBuilderAPIMakeVertex::new(Point3::new(100., -300., 0.))?.vertex()?;
    Ok([first, second, vertex.into_shape()])
}

fn box_object(shape: &Shape, layer: Option<i32>) -> Handle<dyn AisInteractiveObject> {
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
    if let Some(layer) = layer {
        object.set_z_layer(layer);
    }
    object.set_display_mode(1);
    object.set_hilight_mode(1);
    object
}

/// Preserve both display and colour -noupdate commands; `source_boxes(session, &inputs, layer)?`.
pub(super) fn source_boxes(
    session: &mut ViewerSession,
    inputs: &[Shape; 3],
    layer: i32,
) -> Result<(), Failure> {
    session.without_updates(|session| {
        session.display_object("b1", box_object(&inputs[0], None))?;
        session.display_object("b2", box_object(&inputs[1], Some(layer)))?;
        session.set_color("b1", Color::named(NamedColor::RED))?;
        session.set_color("b2", Color::named(NamedColor::GREEN))
    })
}

/// The vertex stays a real AisShape with TopOSD/2D top-left persistence; `source_osd_vertex(session, &inputs[2])?`.
pub(super) fn source_osd_vertex(
    session: &mut ViewerSession,
    vertex: &Shape,
) -> Result<(), Failure> {
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(vertex).cast();
    object.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP_OSD);
    let persistence = source_corner_persistence()?;
    session
        .context()
        .set_transform_persistence(&object, &persistence)?;
    object.set_display_mode(0);
    session.display_object("v", object)
}

fn source_corner_persistence() -> Result<Handle<dyn Graphic3dTransformPersSource>, Failure> {
    corner_persistence(
        Graphic3dTransModeFlags::TMF_2D,
        AspectTypeOfTriedronPosition::LEFT_UPPER,
        [0, 0],
    )
}
