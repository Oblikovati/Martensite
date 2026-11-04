//! Independent fixture and exact-source-predicate controls; none execute the original GPU case.
use super::{commands::selection_color_error, layer::source_layer, scene::*};
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkbrep::{brep::BRepTool, topods::Vertex};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkservice::{
    aspect::AspectTypeOfTriedronPosition,
    graphic3d::{
        GRAPHIC3D_ZLAYER_ID_DEFAULT, GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dTransModeFlags,
        Graphic3dTransformPersMembers as _,
    },
    null_driver::NullGraphicDriver,
};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkv3d::{
    ais::{AisShapeParameters as _, AisShapeSource},
    prs_mgr::PrsMgrPresentableObjectMembers as _,
};

fn cpu_session() -> ViewerSession {
    ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409)
        .expect("zlayer control; expected actual isolated409x409 null-driver context")
}

#[test]
fn exact_source_color_predicate_rejects_close_or_differently_cased_names() {
    assert_eq!(selection_color_error("GRAY76"), None);
    for actual in ["GRAY75", "GRAY77", "gray76", "GRAY76 ", "", "GREEN"] {
        assert_eq!(
            selection_color_error(actual),
            Some("Error: wrong object has been selected"),
            "actual named color {actual:?}"
        );
    }
}

#[test]
fn source_topology_has_literal_box_volumes_and_screen_vertex_coordinates() {
    let inputs = source_inputs().unwrap();
    for (shape, expected) in inputs[..2].iter().zip([24., 2.]) {
        let mut properties = GProps::new();
        BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
        assert!(
            (properties.mass() - expected).abs() < 1.0e-10,
            "actual volume{}; expected closed-form{expected}",
            properties.mass()
        );
    }
    let point = BRepTool::pnt(&Vertex::unchecked(inputs[2].clone())).unwrap();
    assert_eq!([point.x(), point.y(), point.z()], [100., -300., 0.]);
}

#[test]
fn actual_new_layer_keeps_the_explicit_depth_clear_setting() {
    let mut session = cpu_session();
    let layer = source_layer(&mut session).unwrap();
    assert_ne!(layer, GRAPHIC3D_ZLAYER_ID_DEFAULT);
    let viewer = session.view().view_base().viewer();
    assert!(viewer.get_all_z_layers().contains(&layer));
    assert!(viewer.z_layer_settings(layer).unwrap().to_clear_depth());
}

#[test]
fn deferred_display_keeps_actual_layer_and_own_modes_before_the_fit() {
    let mut session = cpu_session();
    let inputs = source_inputs().unwrap();
    let layer = source_layer(&mut session).unwrap();
    session.view().set_immediate_update(true);
    source_boxes(&mut session, &inputs, layer).unwrap();
    assert!(session.view().set_immediate_update(false));
    for (name, expected) in [("b1", GRAPHIC3D_ZLAYER_ID_DEFAULT), ("b2", layer)] {
        let object = session.object(name).unwrap();
        assert_eq!(session.context().get_z_layer(object), expected);
        assert_eq!(object.display_mode(), 1);
        assert_eq!(object.hilight_mode(), 1);
    }
}

#[test]
fn osd_vertex_preserves_shape_identity_and_top_left_coordinate_policy() {
    let mut session = cpu_session();
    let inputs = source_inputs().unwrap();
    source_osd_vertex(&mut session, &inputs[2]).unwrap();
    let object = session.object("v").unwrap();
    assert_osd_vertex_object(&session, object, &inputs[2]);
    assert_top_left_persistence(object);
}

fn assert_osd_vertex_object(
    session: &ViewerSession,
    object: &martensite_tkernel::standard::Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>,
    vertex: &martensite_tkbrep::topods::Shape,
) {
    assert!(object.cast::<dyn AisShapeSource>().shape().is_same(vertex));
    assert_eq!(
        session.context().get_z_layer(object),
        GRAPHIC3D_ZLAYER_ID_TOP_OSD
    );
    assert_eq!(object.display_mode(), 0);
}

fn assert_top_left_persistence(
    object: &martensite_tkernel::standard::Handle<dyn martensite_tkv3d::ais::AisInteractiveObject>,
) {
    let persistence = object.transform_persistence();
    assert_eq!(persistence.mode(), Graphic3dTransModeFlags::TMF_2D);
    assert_eq!(
        persistence.corner_2d().unwrap(),
        AspectTypeOfTriedronPosition::LEFT_UPPER
    );
    let offset = persistence.offset_2d().unwrap();
    assert_eq!([offset.x, offset.y], [0, 0]);
}

#[test]
fn reused_state_query_reports_actual_named_objects_without_selecting_them() {
    let mut session = cpu_session();
    let inputs = source_inputs().unwrap();
    let layer = source_layer(&mut session).unwrap();
    source_boxes(&mut session, &inputs, layer).unwrap();
    let text = crate::selection_queries::state_text(&session, &["b1", "b2"], false).unwrap();
    assert!(
        text.starts_with("Neutral-point state:\n"),
        "actual state{text:?}"
    );
    assert_eq!(text.matches(" Type:  AisShape (AisShape)").count(), 2);
    assert_eq!(session.selected_count(), 0);
}
