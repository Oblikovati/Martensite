//! Query the actual viewer settings after each explicit source command.
use crate::{
    case_layer_commands::{RemovalOutput, remove_first_layer_command},
    case_view::viewer_session::ViewerSession,
};
use martensite_tkernel::{
    message::{Gravity, Message},
    standard::Failure,
    tcollection::AsciiString,
};
use martensite_tkservice::{aspect::AspectPolygonOffsetMode, graphic3d::Graphic3dPolygonOffset};
struct LayerRemovalMessages;
impl RemovalOutput for LayerRemovalMessages {
    fn cannot_remove(&mut self) {
        Message::send_text(
            &AsciiString::from("Z layer can not be removed"),
            Gravity::Fail,
        );
    }
}
pub(super) fn source_layer(session: &mut ViewerSession) -> Result<i32, Failure> {
    let _caught = remove_first_layer_command(session, &mut LayerRemovalMessages);
    session.add_z_layer()
}
pub(super) fn depth_policies(session: &ViewerSession, layer: i32) -> Result<(), Failure> {
    session.update_view(|view| {
        let viewer = view.viewer();
        let mut settings = viewer.z_layer_settings(0)?;
        settings.set_enable_depth_test(false);
        viewer.set_z_layer_settings(0, &settings)
    })?;
    session.update_view(|view| {
        let viewer = view.viewer();
        let mut settings = viewer.z_layer_settings(layer)?;
        settings.set_clear_depth(false);
        viewer.set_z_layer_settings(layer, &settings)
    })?;
    query_settings(session, 0)?;
    query_settings(session, layer)
}
fn query_settings(session: &ViewerSession, layer: i32) -> Result<(), Failure> {
    let settings = session
        .view()
        .view_base()
        .viewer()
        .z_layer_settings(layer)?;
    print_optional_settings(&settings);
    let origin = settings.origin();
    println!("  Origin: {} {} {}", origin.x(), origin.y(), origin.z());
    println!("  Culling distance: {}", settings.culling_distance());
    println!("  Culling size: {}", settings.culling_size());
    print_depth_settings(&settings);
    Ok(())
}
fn print_optional_settings(settings: &martensite_tkservice::graphic3d::Graphic3dZLayerSettings) {
    if !settings.name().is_empty() {
        println!(
            "  Name: {}",
            String::from_utf8_lossy(settings.name().as_bytes())
        );
    }
    if settings.is_immediate() {
        println!("  Immediate: TRUE");
    }
}
fn print_depth_settings(settings: &martensite_tkservice::graphic3d::Graphic3dZLayerSettings) {
    let word = |enabled: bool| if enabled { "enabled" } else { "disabled" };
    println!("  Depth test:   {}", word(settings.to_enable_depth_test()));
    println!("  Depth write:  {}", word(settings.to_enable_depth_write()));
    println!(
        "  Depth buffer clearing: {}",
        word(settings.to_clear_depth())
    );
    let offset = settings.polygon_offset();
    if offset.mode != AspectPolygonOffsetMode::NONE {
        println!("  Depth offset: {} {}", offset.factor, offset.units);
    }
}
pub(super) fn offset(session: &ViewerSession) -> Result<(), Failure> {
    session.update_view(|view| {
        let viewer = view.viewer();
        let mut settings = viewer.z_layer_settings(1)?;
        settings.set_polygon_offset(Graphic3dPolygonOffset {
            mode: AspectPolygonOffsetMode::FILL,
            factor: 2.,
            units: 1.,
        });
        viewer.set_z_layer_settings(1, &settings)
    })
}
