use super::super::{marker, occlusion};
use super::*;
use martensite_tkernel::ncollection::List;
use martensite_tkservice::{
    aspect::AspectTypeOfTriedronPosition,
    graphic3d::{
        GRAPHIC3D_ZLAYER_ID_TOP_OSD, Graphic3dTransModeFlags, Graphic3dTransformPersMembers as _,
    },
};
use martensite_tkv3d::ais::AisPoint;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

fn scene() -> ViewerSession {
    let mut session = cpu_session();
    let boxes = occlusion::boxes().unwrap();
    occlusion::scene(&mut session, &boxes).unwrap();
    session
}

#[test]
fn actual_three_projection_picks_keep_protruding_box_in_front_and_miss_distinct() {
    let mut session = scene();
    let front = hover_object(&session, "b1").unwrap();
    for stage in occlusion::STAGES {
        assert_front_then_miss(&mut session, stage, &front);
    }
}

#[test]
fn actual_point_is_displayed_before_deactivation_with_source_pixel_persistence() {
    let mut session = scene();
    super::super::reused_datums::point_2d(&mut session, "before", [325., 100.]).unwrap();
    let before = hover_object(&session, "before").unwrap();
    assert!(session.context().is_displayed(&before));
    assert!(!active_modes(&session, &before).is_empty());
    session.context().deactivate(&before).unwrap();
    assert!(active_modes(&session, &before).is_empty());
    marker::display_no_selection(&mut session, "p1", [325, 100]).unwrap();
    let actual = hover_object(&session, "p1").unwrap();
    assert_marker(&session, &actual);
    assert!(active_modes(&session, &actual).is_empty());
}

fn active_modes(session: &ViewerSession, object: &Handle<dyn AisInteractiveObject>) -> Vec<i32> {
    let mut modes = List::default();
    session.context().activated_modes(object, &mut modes);
    modes.iter().copied().collect()
}

fn assert_marker(session: &ViewerSession, object: &Handle<dyn AisInteractiveObject>) {
    let point = object.cast::<AisPoint>();
    let p = point.component().pnt();
    assert_eq!([p.x(), p.y(), p.z()], [325., -100., 0.]);
    let persistence = object.transform_persistence();
    assert_eq!(persistence.mode(), Graphic3dTransModeFlags::TMF_2D);
    assert_eq!(
        persistence.corner_2d().unwrap(),
        AspectTypeOfTriedronPosition::LEFT_UPPER
    );
    assert_eq!(
        session.context().get_z_layer(object),
        GRAPHIC3D_ZLAYER_ID_TOP_OSD
    );
    assert!(session.context().is_displayed(object));
}

#[test]
fn actual_removed_marker_snapshots_release_leases_without_changing_other_objects() {
    let mut session = scene();
    let b1 = hover_object(&session, "b1").unwrap();
    occlusion::camera(&session, occlusion::STAGES[0]).unwrap();
    session.move_cursor(325, 100).unwrap();
    marker::display_no_selection(&mut session, "p1", [325, 100]).unwrap();
    let point = hover_object(&session, "p1").unwrap();
    let weak = point.downgrade();
    let detected = detected_objects(&session);
    assert!(!detected.is_empty());
    session.remove(&["p1"]).unwrap();
    assert!(!session.context().is_displayed(&point));
    assert!(session.context().is_displayed(&b1));
    assert_missing_marker(&session);
    drop(detected);
    drop(point);
    assert!(weak.upgrade().is_null());
    assert!(active_modes(&session, &b1).contains(&0));
}

fn assert_front_then_miss(
    session: &mut ViewerSession,
    stage: occlusion::OcclusionStage,
    front: &Handle<dyn AisInteractiveObject>,
) {
    occlusion::camera(session, stage).unwrap();
    session
        .move_cursor(stage.cursor[0], stage.cursor[1])
        .unwrap();
    assert_eq!(&highlighted(session), front);
    let point = session.detected_point().unwrap();
    assert!(point.iter().all(|v| v.is_finite()));
    observe(session, stage.marker);
    session.move_cursor(0, 0).unwrap();
    assert!(!session.context().has_detected());
}

fn assert_missing_marker(session: &ViewerSession) {
    let error = hover_object(session, "p1").unwrap_err();
    assert!(error.message().contains("p1"));
    assert!(error.message().contains("actual displayed binding"));
}
